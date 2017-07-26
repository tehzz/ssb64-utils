use configs::{ImportConfig};
use errors::*;
use collision::{CollisionPoint, Spawn, PlaneInfo, ColDetection};

use std::io::{Read, Write, Seek};
use std::fmt::Debug;

pub fn import_collision<O>(config: ImportConfig<O>) -> Result<String>
    where O: Read + Write + Seek + Debug
{
    let verbose = config.verbose;
    // Break the deserialized input FormattedCollision back into the individual component structures
    let (col_points, spawn_points, plane_info, point_connections, col_directions) = config.input.to_parts();
    //(Vec<&CollisionPoint>, &[Spawn], Vec<PlaneInfo>, Vec<u16>, Vec<ColDetection>)
    // (col_points, spawn_points, plane_info, point_connections, col_dects)

    if verbose {
        println!("{:?}", col_points);
        println!("{:?}", spawn_points);
        println!("{:#?}", plane_info);
    }
    println!("{:#?}", col_directions);
    println!("{:?}", point_connections);

    // Make these generic with a trait.... fn size; fn to_bytes; fn bytes_iter
    //transform CollisionPoint vec into u8 byte vec
    let points_size = col_points.len() * CollisionPoint::sizeof_struct();
    let points_bytes = col_points
        .iter()
        .map(|p| p.to_bytes())
        .fold(Vec::with_capacity(points_size),
        |a, v| fold_bytes(a, v.as_ref()));
    println!("{:?}", &points_bytes);

    //transform Spawn vec into u8 byte vec
    let spawn_size = spawn_points.len() * Spawn::sizeof_struct();
    let spawn_bytes = spawn_points
        .iter()
        .map(|s| s.to_bytes())
        .fold(Vec::with_capacity(spawn_size),
        |a, v| fold_bytes(a, v.as_ref()));
    println!("{:?}", spawn_bytes);

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

    Ok(format!("Import not fully implemented yet"))
}

fn fold_bytes(mut acc: Vec<u8>, bytes: &[u8]) -> Vec<u8> {
    acc.extend(bytes.iter());

    acc
}
