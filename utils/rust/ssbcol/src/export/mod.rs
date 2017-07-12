use configs::{ExportConfig};
//use std::io::{BufReader};
use errors::*;
use byteorder::{BE, ReadBytesExt};
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::io::Result as IoResult;
use std::fmt;


pub fn export_collision(config: ExportConfig) -> Result<String> {
    let mut f = config.input;
    let ptr = config.col_ptr;
    // array for collision pointers data : [u8; 0xC0]
    let mut col_ptrs = [0u8; 0x1C];

    f.seek(SeekFrom::Start(ptr as u64))
        .chain_err(||
            format!("Error seeking in file to collision pointers at 0x{:08X}", ptr)
        )?;
    f.read_exact(&mut col_ptrs)
        .chain_err(||"Error reading 0x1C bytes for collision pointers")?;

    //let unk1: u16, col_points: u32, col_connect: u32, col_planes: u32,
    let ptrs = get_collisions_ptrs(&col_ptrs)
        .chain_err(|| "Error getting collision pointers")?;

    println!("Testing reading:\n{}", ptrs);

    Ok(format!("Not Implemented, but\n col-ptr: {:08X}", config.col_ptr))
}

fn check_res_ptr(input: u32) -> IoResult<u32> {
    // two MSB == 0x80, probably pointer from a RAM dump
    if (input >> 24) == 0x80 {
        Ok(input)
    } else {
        // assume resource file; take lower half and convert from words to bytes
        Ok((input & 0xFFFF) << 2)
    }
}
#[derive(Debug)]
struct ColPtrs {
    unk1: u16,
    points: u32,
    connections: u32,
    planes: u32,
    surface: u32,
    spawn_count: u16,
    spawns: u32
}

impl fmt::Display for ColPtrs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self;

        write!(f, "ColPtrs {{
    unk1:        {:#06X},
    points:      {:#010X},
    connections: {:#010X},
    planes:      {:#010X},
    surface:     {:#010X},
    spawn_count: {:#06X},
    spawns:      {:#010X}
}}", s.unk1, s.points, s.connections, s.planes, s.surface, s.spawn_count, s.spawns)
    }
}

fn get_collisions_ptrs(arr: &[u8]) -> IoResult<ColPtrs> {
    let mut c = Cursor::new(arr);
    // read some values
    let unk1 = c.read_u16::<BE>()?;
    //skip two byte pad
    c.seek(SeekFrom::Current(2))?;
    // read some pointer
    let points_ptr = c.read_u32::<BE>().and_then(check_res_ptr)?;
    let connect_ptr = c.read_u32::<BE>().and_then(check_res_ptr)?;
    let planes_ptr = c.read_u32::<BE>().and_then(check_res_ptr)?;
    let surface_ptr = c.read_u32::<BE>().and_then(check_res_ptr)?;
    let spawn_count = c.read_u16::<BE>()?;
    c.seek(SeekFrom::Current(2))?;
    let spawns_ptr = c.read_u32::<BE>().and_then(check_res_ptr)?;

    Ok(ColPtrs {
        unk1,
        points: points_ptr,
        connections: connect_ptr,
        planes: planes_ptr,
        surface: surface_ptr,
        spawn_count,
        spawns: spawns_ptr
    })
}
