use configs::{ImportConfig};
use errors::*;
use collision::{FormattedCollision, ColPtrs};
use traits::N64Bytes;

use std::io::{Read, Write, Seek, SeekFrom, Cursor};
use std::fmt::Debug;
use byteorder::{BE, ByteOrder};

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
        let end = output.seek(SeekFrom::End(0))?;
        let align = end + (end & 8);

        output.seek(SeekFrom::Start(align))?
    };

    let (buffer, ptrs, ptrs_offset) = generate_buffer(&collision, output_start)
        .chain_err(||"generating collision output buffer")?;

    if verbose {
        println!("Output buffer:\n{:?}", &buffer);
        println!("Collision Pointers Struct:\n{}", ptrs);
        println!("Offset of Pointers struct in file:\n{:#010X}", ptrs_offset);
    }

    // write to file? should be aligned from the output_start variable?
    output.write_all(&buffer)
        .chain_err(||"writing full buffer to output file")?;
    // fill output file to 4 word boundry ?


    Ok(format!("Import not fully implemented yet"))
}

/// Move the position in the Cursor forward until its aligned with align
/// Return the new position of the cursor
fn align_cursor<T>(csr: &mut Cursor<T>, align: u64) -> u64 {
    let pos = csr.position();
    let aligned = pos + (pos % align);
    csr.set_position(aligned);

    aligned
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
/// and return a tupple (Vec<u8>, ColPtrs, Offset).  The vector has the complete
/// u8 buffer of collisions, spawns, and pointers. The ColPtrs struct has the offsets
/// within the buffer for the various pointers. Finally, the offset u64 is the offset
/// within the buffer for the ColPtrs struct.
fn generate_buffer(collision: &FormattedCollision, offset: u64)
-> Result<(Vec<u8>, ColPtrs, u32)>
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
    println!("test spawn_bytes:\n{:#?}\n{:?}", &spawn_points, &spawn_bytes);

    // Combine component vec<u8>s into one buffer with proper component alignment
    let buffer: Vec<u8> = Vec::new();
    let mut cbuf = Cursor::new(buffer);

    // get and configure new set of collision pointer struct
    let mut ptrs     = ColPtrs::new_null();
    ptrs.col_count   = col_directions.len() as u16;
    ptrs.spawn_count = spawn_points.len() as u16;

    // output buffer obviously aligned. write in all of collision points vec
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
    // word align cursor and write pointer struct
    let colptrs_ptr = (offset + align_cursor(&mut cbuf, 4)) as u32;
    cbuf.write_all(ptrs.to_bytes()?.as_ref())
        .chain_err(||"writing pointer struct to buffer")?;

    Ok((cbuf.into_inner(), ptrs, colptrs_ptr))
}
