use configs::{ImportConfig};
use errors::*;
use collision::{FormattedCollision, CollisionPoint, Spawn, PlaneInfo, ColDetection, ColPtrs};

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

    /*if verbose {
        println!("{:#?}", &col_points);
        println!("{:?}",  &point_connections);
        println!("{:#?}", &plane_info);
        println!("{:#?}", &col_directions);
        println!("{:#?}", &spawn_points);
    }*/

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

fn fold_bytes(mut acc: Vec<u8>, bytes: &[u8]) -> Vec<u8> {
    acc.extend(bytes.iter());

    acc
}

/// Move the position in the Cursor forward until its aligned with align
/// Return the new position of the cursor
fn align_cursor<T>(csr: &mut Cursor<T>, align: u64) -> u64 {
    let pos = csr.position();
    let aligned = pos + (pos % align);
    csr.set_position(aligned);

    aligned
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

    // Make these generic with a trait.... fn size; fn to_bytes; fn bytes_iter
    //transform Spawn vec into u8 byte vec
    let spawn_size = spawn_points.len() * Spawn::sizeof_struct();
    let spawn_bytes = spawn_points
        .iter()
        .map(|s| s.to_bytes())
        .fold(Vec::with_capacity(spawn_size),
        |a, v| fold_bytes(a, v.as_ref()));

    //transform CollisionPoint vec into u8 byte vec
    let points_size = col_points.len() * CollisionPoint::sizeof_struct();
    let points_bytes = col_points
        .iter()
        .map(|p| p.to_bytes())
        .fold(Vec::with_capacity(points_size),
        |a, v| fold_bytes(a, v.as_ref()));

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
    let pi_size  = plane_info.len() * PlaneInfo::sizeof_struct();
    let pi_bytes = plane_info
        .iter()
        .map(|s| s.to_bytes())
        .fold(Vec::with_capacity(pi_size),
        |a, v| fold_bytes(a, v.as_ref()));

    //transform ColDetection vec into u8 byte vec
    let detect_size  = col_directions.len() * ColDetection::sizeof_struct();
    let detect_bytes = col_directions
        .iter()
        .map(|s| s.to_bytes())
        .fold(Vec::with_capacity(detect_size),
        |a, v| fold_bytes(a, v.as_ref()));

    // Combine component vec<u8>s into one buffer with proper component alignment
    let buffer: Vec<u8> = Vec::new();
    let mut cbuf = Cursor::new(buffer);

    // get and configure new set of collision pointer struct
    let mut ptrs     = ColPtrs::new_null();
    ptrs.col_count   = col_directions.len() as u16;
    ptrs.spawn_count = spawn_points.len() as u16;

    // output buffer obviously aligned. write in all of collision points vec
    ptrs.points = (offset + 0) as u32;
    cbuf.write_all(&points_bytes).chain_err(||"writing collision points array to buffer")?;
    // word align cursor of output buffer, add to file end, and write all of plane array to buffer
    ptrs.connections = (offset + align_cursor(&mut cbuf, 4)) as u32;
    cbuf.write_all(&connect_bytes).chain_err(||"writing point connection (plane points) array to buffer")?;
    // word align cursor and write the plane info bytes
    ptrs.planes = (offset + align_cursor(&mut cbuf, 4)) as u32;
    cbuf.write_all(&pi_bytes).chain_err(||"writing plane info bytes to buffer")?;
    // word align cursor and write collision detection
    ptrs.col_direct = (offset + align_cursor(&mut cbuf, 4)) as u32;
    cbuf.write_all(&detect_bytes).chain_err(||"writing collision detection bytes to buffer")?;
    // word align cursor and write spawn points
    ptrs.spawns = (offset + align_cursor(&mut cbuf, 4)) as u32;
    cbuf.write_all(&spawn_bytes).chain_err(||"writing spawn points bytes to buffer")?;
    // word align cursor and write pointer struct
    let colptrs_ptr = (offset + align_cursor(&mut cbuf, 4)) as u32;
    cbuf.write_all(ptrs.to_bytes()?.as_ref()).chain_err(||"writing pointer struct to buffer")?;

    Ok((cbuf.into_inner(), ptrs, colptrs_ptr))
}
