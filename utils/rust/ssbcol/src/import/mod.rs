use configs::{ImportConfig};
use errors::*;
use collision::{CollisionPoint, Spawn, PlaneInfo, ColDetection, ColPtrs};

use std::io::{Read, Write, Seek, SeekFrom, Cursor};
use std::fmt::Debug;
use byteorder::{BE, ByteOrder};

pub fn import_collision<O>(mut config: ImportConfig<O>) -> Result<String>
    where O: Read + Write + Seek + Debug
{
    let verbose = config.verbose;
    // Break the deserialized input FormattedCollision back into the individual component structures
    let (col_points, spawn_points, plane_info, point_connections, col_directions) = config.input.to_parts();
    //(Vec<&CollisionPoint>, &[Spawn], Vec<PlaneInfo>, Vec<u16>, Vec<ColDetection>)
    // (col_points, spawn_points, plane_info, point_connections, col_dects)

    if verbose {
        println!("{:#?}", &col_points);
        println!("{:?}",  &point_connections);
        println!("{:#?}", &plane_info);
        println!("{:#?}", &col_directions);
        println!("{:#?}", &spawn_points);
    }

    // Make these generic with a trait.... fn size; fn to_bytes; fn bytes_iter
    //transform Spawn vec into u8 byte vec
    let spawn_size = spawn_points.len() * Spawn::sizeof_struct();
    let spawn_bytes = spawn_points
    .iter()
    .map(|s| s.to_bytes())
    .fold(Vec::with_capacity(spawn_size),
    |a, v| fold_bytes(a, v.as_ref()));
    println!("{:?}", spawn_bytes);

    //transform CollisionPoint vec into u8 byte vec
    let points_size = col_points.len() * CollisionPoint::sizeof_struct();
    let points_bytes = col_points
        .iter()
        .map(|p| p.to_bytes())
        .fold(Vec::with_capacity(points_size),
        |a, v| fold_bytes(a, v.as_ref()));
    println!("{:?}", &points_bytes);

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
    println!("{:?}", pi_bytes);

    //transform ColDetection vec into u8 byte vec
    let detect_size  = col_directions.len() * ColDetection::sizeof_struct();
    let detect_bytes = col_directions
        .iter()
        .map(|s| s.to_bytes())
        .fold(Vec::with_capacity(detect_size),
        |a, v| fold_bytes(a, v.as_ref()));
    println!("{:?}", detect_bytes);

    // Combine component vec<u8>s into one buffer with proper component alignment
    let buffer: Vec<u8> = Vec::new();
    let mut cbuf = Cursor::new(buffer);

    //find size of output file and align length to 8 byte boundry?
    let output_start = {
        let end = config.output.seek(SeekFrom::End(0))?;
        let align = end + (end & 8);

        config.output.seek(SeekFrom::Start(align))?
    };
    // get and configure new set of collision pointer struct
    let mut ptrs     = ColPtrs::new_null();
    ptrs.col_count   = col_directions.len() as u16;
    ptrs.spawn_count = spawn_points.len() as u16;

    // output buffer obviously aligned. write in all of collision points vec
    ptrs.points = (output_start + 0) as u32;
    cbuf.write_all(&points_bytes).chain_err(||"writing collision points array to buffer")?;
    // word align cursor of output buffer, add to file end, and write all of plane array to buffer
    ptrs.connections = (output_start + align_cursor(&mut cbuf, 4)) as u32;
    cbuf.write_all(&connect_bytes).chain_err(||"writing point connection (plane points) array to buffer")?;
    // word align cursor and write the plane info bytes
    ptrs.planes = (output_start + align_cursor(&mut cbuf, 4)) as u32;
    cbuf.write_all(&pi_bytes).chain_err(||"writing plane info bytes to buffer")?;
    // word align cursor and write collision detection
    ptrs.col_direct = (output_start + align_cursor(&mut cbuf, 4)) as u32;
    cbuf.write_all(&detect_bytes).chain_err(||"writing collision detection bytes to buffer")?;
    // word align cursor and write spawn points
    ptrs.spawns = (output_start + align_cursor(&mut cbuf, 4)) as u32;
    cbuf.write_all(&spawn_bytes).chain_err(||"write spawn points bytes to buffer")?;

    println!("{:?}", &cbuf);
    println!("{}", ptrs);

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
