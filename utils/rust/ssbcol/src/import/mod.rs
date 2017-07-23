use configs::{ImportConfig};
use errors::*;

use std::io::{Read, Write, Seek};
use std::fmt::Debug;

pub fn import_collision<O>(config: ImportConfig<O>) -> Result<String>
    where O: Read + Write + Seek + Debug
{

    println!("{:?}", config);

    Ok(format!("Import not fully implemented yet"))
}
