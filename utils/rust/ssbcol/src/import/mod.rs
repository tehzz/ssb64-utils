mod align;
use self::align::{align_seek, align_cursor};

use configs::{ImportConfig};
use errors::*;
use collision::{FormattedCollision, ColPtrs};
use traits::N64Bytes;

use std::io::{Read, Write, Seek, SeekFrom, Cursor};
use std::fmt::Debug;
use byteorder::{BE, ByteOrder, WriteBytesExt, ReadBytesExt};

pub fn import_collision<O>(config: ImportConfig<O>) -> Result<String>
    where O: Read + Write + Seek + Debug
{
    let ImportConfig{
        input: collision,
        mut output,
        verbose,
        res_ptr,
        req_start
    } = config;

    //find size of output file and align length to 8 byte boundry?
    let output_start = {
        output.seek(SeekFrom::End(0))?;
        align_seek(&mut output, 8)?
    };

    let (buffer, mut ptrs, ..) = generate_buffer(&collision, output_start)
        .chain_err(||"generating collision output buffer")?;

    // align buffer to store ColPtrs struct
    let mut bufcsr = Cursor::new(buffer);
    let ptrs_ptr = {
        bufcsr.seek(SeekFrom::End(0))?;
        align_cursor(&mut bufcsr, 4) + output_start
    } as u32;

    // if there is a resource pointer chain, include collision pointers in chain
    if let Some(chain) = res_ptr {
        // convert pointers to resource file list pointer
        ptrs.to_resource_pointers(ptrs_ptr, None);

        // add collision pointers to the file's resource pointer list
        let mut chain = (chain >> 2) as u16;
        let mut ptr = 0u64;

        while chain != 0xFFFF {
            ptr = (chain << 2) as u64;
            output.seek(SeekFrom::Start(ptr))
                .chain_err(||format!("seeking to node pointer at {:#010X}", ptr))?;
            chain = output.read_u16::<BE>()
                .chain_err(||format!("reading u16 at {:#010X}", ptr))?;
        }

        // replace the end chain (0xFFFFu16) with the u16 word offset
        // to the first pointer in the ColPtrs struct
        let ptrs_word_offset = (((ptrs_ptr + 4) >> 2) & 0xFFFF) as u16;
        output.seek(SeekFrom::Start(ptr))
            .chain_err(||format!("seeking to {:#010X} to update original resource file pointer list end", ptr))?;
        output.write_u16::<BE>(ptrs_word_offset)?;
    }

    //write ColPtrs struct into buffer as big endian bytes
    let test = ptrs.to_bytes()?;
    bufcsr.write_all(&test)
        .chain_err(||"writing ColPtrs struct to buffer vector of collision data")?;

    if verbose {
        println!("Output buffer:\n{:?}", &bufcsr.get_ref());
        println!("Collision Pointers Struct:\n{}\n{:?}", ptrs, &test);
        println!("Offset of Pointers struct in file:\n{:#010X}", ptrs_ptr);
    }

    // write to file? align to output start
    output.seek(SeekFrom::Start(output_start))
        .chain_err(||format!("seeking to {:#010X} in order to write buffer into output", output_start))?;

    output.write_all(bufcsr.get_ref())
        .chain_err(||"writing full buffer to output file")?;
    // fill output file to 4 word boundry ?


    Ok(format!("Import not fully implemented yet"))
}

fn flatten_collision_slice<T>(input: &[T]) -> Vec<u8>
    where T: N64Bytes
{
    let byte_size = input.len() * T::size();
    let output: Vec<u8> = input
        .iter()
        .fold(Vec::with_capacity(byte_size),
        |mut acc, t| {
            let bytes = t.to_bytes();
            acc.extend(bytes.as_ref().iter());
            acc
        });

    output
}

/// Take the input FormattedCollision struct and u32 pointer offset
/// and return a tupple containing: a vector that has the complete
/// u8 buffer of collisions, spawns, and pointers;
/// a ColPtrs struct with pointers into the vector + the offset argument
/// a u64 of the end of the vector + the offset argument
fn generate_buffer(collision: &FormattedCollision, offset: u64)
-> Result<(Vec<u8>, ColPtrs, u64)>
{
    // Break the deserialized input FormattedCollision back into the individual component structures
    let (col_points, spawn_points, plane_info, point_connections, col_directions) = collision.to_parts();


    //transform CollisionPoint vec into u8 byte vec
    let points_bytes = flatten_collision_slice(&col_points);
    //transform the points connections/plane array into BE u8
    let connect_length = point_connections.len() * 2;
    let connect_bytes: Vec<u8> = point_connections
        .iter()
        .fold(Vec::with_capacity(connect_length),
        |mut acc, val| {
            let mut bytes = [0u8; 2];
            BE::write_u16(&mut bytes, *val);
            acc.extend(bytes.iter());

            acc
        });
    //transform PlaneInfo vec into u8 byte vec
    let pi_bytes = flatten_collision_slice(&plane_info);
    //transform ColDetection vec into u8 byte vec
    let detect_bytes = flatten_collision_slice(&col_directions);
    // &[Spawn] -> Vec<u8>
    let spawn_bytes = flatten_collision_slice(&spawn_points);
    // Combine component vec<u8>s into one buffer with proper component alignment
    let buffer: Vec<u8> = Vec::new();
    let mut cbuf = Cursor::new(buffer);

    // get and configure new set of collision pointer struct
    let mut ptrs     = ColPtrs::new_null();
    ptrs.col_count   = col_directions.len() as u16;
    ptrs.spawn_count = spawn_points.len() as u16;

    // output buffer is at 0, and thus aligned. write in all of collision points vec
    ptrs.points = (offset + 0) as u32;
    cbuf.write_all(&points_bytes)
        .chain_err(||"writing collision points array to buffer")?;
    // word align cursor of output buffer, add to file end, and write all of plane array to buffer
    ptrs.connections = (offset + align_cursor(&mut cbuf, 4)) as u32;
    cbuf.write_all(&connect_bytes)
        .chain_err(||"writing point connection (plane points) array to buffer")?;
    // word align cursor and write the plane info bytes
    ptrs.planes = (offset + align_cursor(&mut cbuf, 4)) as u32;
    cbuf.write_all(&pi_bytes)
        .chain_err(||"writing plane info bytes to buffer")?;
    // word align cursor and write collision detection
    ptrs.col_direct = (offset + align_cursor(&mut cbuf, 4)) as u32;
    cbuf.write_all(&detect_bytes)
        .chain_err(||"writing collision detection bytes to buffer")?;
    // word align cursor and write spawn points
    ptrs.spawns = (offset + align_cursor(&mut cbuf, 4)) as u32;
    cbuf.write_all(&spawn_bytes)
        .chain_err(||"writing spawn points bytes to buffer")?;

    let final_position = offset + cbuf.position();
    Ok((cbuf.into_inner(), ptrs, final_position))
}
