use configs::{ImportConfig};
use errors::*;

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
    }
    println!("{:#?}", col_directions);
    println!("{:#?}", plane_info);
    println!("{:?}", point_connections);

    Ok(format!("Import not fully implemented yet"))
}
