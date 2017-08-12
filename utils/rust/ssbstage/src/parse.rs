use std::io::{Read};
use byteorder::{BE, ByteOrder};
use errors::*;
use StageFileKind;
use stage::StageMain;

pub fn stage_binary<I>(mut input: I, kind: Option<StageFileKind>, verbose: bool) -> Result<StageMain>
    where I: Read
{
    // Read binary into a vec, and capture the filesize
    let mut stage_bin = Vec::new();
    let input_size = input.read_to_end(&mut stage_bin)
        .chain_err(||"reading input file into memory")?;

    // either use the user supplied stage-main file type,
    // or try to determine the type by checking the buffer size, and reading various pointers
    let kind = kind.map_or_else(|| -> Result<StageFileKind> {
        let item_ptr_84 = BE::read_u32(&stage_bin[0x84..0x88]);
        let item_ptr_9c = BE::read_u32(&stage_bin[0x98..0x9C]);

        if verbose {
            println!("No file type for stage main file was supplied.\n\
            Attemping to determine from file...\n\
            size: {:#x}\n0x84 ptr: {:#010x}\n0x9C ptr: {:#010x}",
            input_size, item_ptr_84, item_ptr_9c);
        }

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

    let (items, main, extra) = match kind {
        StageFileKind::Item => {
            let i = Some(&stage_bin[0..0x14]);
            let m = &stage_bin[0x14..0xBC];
            let x = &stage_bin[0xBC..];
            let e = check_padding(x);

            (i, m, e)
        },
        StageFileKind::NoItem => {
            let (m, x) = stage_bin.split_at(0xa8);
            let e = check_padding(x);

            (None, m, e)
        }
    };

    let output = StageMain::from_bytes(&main, items, extra)
        .chain_err(||"parsing binary")?;

    if verbose {
        println!("Sum of item bytes: {}", items.unwrap_or(&[0]).iter().map(|v| *v as u64).sum::<u64>());
    }

    Ok(output)
}

fn check_padding(input: &[u8]) -> Option<&[u8]> {
    //println!("Debug check_padding:\n{:?}", input);
    if input.is_empty() {
        None
    } else if input.iter().all(|b| *b == 0) {
        None
    } else {
        Some(input)
    }
}
