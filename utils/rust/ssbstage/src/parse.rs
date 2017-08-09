use std::io::{Read, Write, Seek};
use StageFileKind;

pub fn stage_to_json<I,O>(input: I, output: O, kind: Option<StageFileKind>) -> String
    where I: Read + Seek, O: Write + Seek
{
    format!("Parsing stage main file to JSON not yet implemented T=T")
}
