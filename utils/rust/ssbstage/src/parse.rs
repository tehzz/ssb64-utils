use std::io::{Read, Write, Seek};
use byteorder::{BE, ByteOrder};
use errors::*;
use StageFileKind;

pub fn stage_to_json<I,O>(mut input: I, kind: Option<StageFileKind>) -> Result<String>
    where I: Read, O: Write + Seek
{
    // Read binary into a vec, and capture the filesize
    let mut stage_bin = Vec::new();
    let input_size = input.read_to_end(&mut stage_bin)
        .chain_err(||"reading input file into memory")?;

    // either use the user supplied stage-main file type,
    // or try to determine the type by checking the buffer size, and reading various pointers
    if kind.is_none() {
        println!("No file type for stage main file was supplied.\n\
         Attemping to determine from file...");
    }

    let kind = kind.map_or_else(|| -> Result<StageFileKind> {
        let item_ptr_84 = BE::read_u32(&stage_bin[0x84..0x88]);
        let item_ptr_9c = BE::read_u32(&stage_bin[0x98..0x9C]);

        println!("Testing auto stage file determination:\n\
        size: {:#x}\n0x84 ptr: {:#010x}\n0x9C ptr: {:#010x}",
        input_size, item_ptr_84, item_ptr_9c);

        if input_size < 0xBC && item_ptr_84 == 0 {
            return Ok(StageFileKind::NoItem);
        } else if item_ptr_9c & 0xFFFF == 0{
            return Ok(StageFileKind::Item);
        };

        Err(format!("indeterminable combination of possible hints\n\
         file size: {:#x}\n0x84 ptr: {:#010x}\n0x9C ptr: {:#010x}",
         input_size, item_ptr_84, item_ptr_9c).into())

    },|val| { Ok(val) } )
        .chain_err(|| "attempting to automatically determine the type of the stage main file")?;

    println!("Kind result? {:?}", &kind);

    Ok(format!("Parsing stage main file to JSON not yet implemented T=T"))
}
