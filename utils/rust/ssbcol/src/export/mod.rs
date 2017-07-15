use configs::{ExportConfig};
use errors::*;
use byteorder::{BE, ReadBytesExt};
use std::io::{Cursor, Read, Seek, SeekFrom, Result as IoResult};
use std::fmt;

pub fn export_collision(config: ExportConfig) -> Result<String> {
    let mut f = config.input;
    let ptr = config.col_ptr;
    // array for collision pointers data : [u8; 0x1C]
    let mut col_ptrs_raw = [0u8; 0x1C];

    f.seek(SeekFrom::Start(ptr as u64))
        .chain_err(||
            format!("seeking to collision pointers at 0x{:08X}", ptr)
        )?;
    f.read_exact(&mut col_ptrs_raw)
        .chain_err(||"reading 0x1C bytes for collision pointers")?;

    let main_ptrs = get_collisions_ptrs(&col_ptrs_raw)
        .chain_err(|| "formatting collision pointers")?;

    println!("Testing reading of pointer struct:\n{}", main_ptrs);

    // get collision detection struct
    f.seek(SeekFrom::Start(main_ptrs.col_direct as u64))
        .chain_err(||
            format!("seeking to collision detection struct at {:#010X}", main_ptrs.col_direct)
        )?;
    let mut col_detect_raw = [0u16; 0x9];
    f.read_u16_into::<BE>(&mut col_detect_raw)
        .chain_err(|| "reading 0x12 bytes for collision detection struct")?;

    let col_detect = ColDetection::new(&col_detect_raw);

    println!("Debug collision detections struct: \n{:#?}", &col_detect);
    let total_planes = col_detect.calc_total_planes() as usize;
    println!("Testing size of planes array: {:#X}", total_planes);

    // get plane descriptions (start + len into connections)
    f.seek(SeekFrom::Start(main_ptrs.planes as u64))
        .chain_err(||
            format!("seeking to beginning of planes array at {:#010X}", main_ptrs.planes)
        )?;
    let mut planes_raw = vec![0u32; total_planes];
    f.read_u32_into::<BE>(&mut planes_raw)
        .chain_err(||format!("reading {} words into planes vec", total_planes))?;
    //println!("Debug planes vec:\n{:#?}", planes_raw);
    let plane_info: Vec<PlaneInfo> = planes_raw.iter().map(to_plane_info).collect();
    println!("Debug planes info vec:\n{:#?}", plane_info);

    f.seek(SeekFrom::Start(main_ptrs.connections as u64))
        .chain_err(||
            format!("seeking to beginning of connections array at {:#010X}", main_ptrs.connections)
        )?;
    let connections_length = plane_info.iter().map( |&PlaneInfo{start, length}|
        start + length ).max().unwrap() as usize; //TODO remove unwrap
    println!("Testing length of connections array: {:?}", connections_length);
    let mut connect_raw = vec![0u16; connections_length];
    f.read_u16_into::<BE>(&mut connect_raw)
        .chain_err(||format!("reading {} words into connections vec", connections_length))?;
    println!("Debug connections raw:\n{:?}", connect_raw);

    // calculate size of 16 bit member collision points array
    let points_length = (*connect_raw.iter().max().unwrap() as usize + 1) * 3; //TODO remove unwrap
    println!("Calculated length of collision points array: {:#X}", points_length);
    let mut points_raw = vec![0u16; points_length];

    f.seek(SeekFrom::Start(main_ptrs.points as u64))
        .chain_err(|| "Error seeking to beginning of collision points array")?;

    f.read_u16_into::<BE>(&mut points_raw)
        .chain_err(|| "reading BE collision pointers into array")?;
    //println!("Debug points raw:\n{:?}", points_raw);

    let points: Result<Vec<_>> = points_raw.chunks(3).map(|parts|
            CollisionPoint::from_raw(parts)
        ).collect();

    let test = points.chain_err(||"converting raw u16 slice into CollisionPoint vec")?;
    println!("Debug processed points:\n{:#?}", test);

    //----Read Spawn length and spawn points-------
    f.seek(SeekFrom::Start(main_ptrs.spawns as u64))
        .chain_err(||format!("seeking to spawn array at {:#010X}", main_ptrs.spawns))?;
    let total_spawns = main_ptrs.spawn_count;   // number of 6 byte (u16, i16, i16) spawn structures
    let mut spawns_raw = vec![0u16; (total_spawns * 3) as usize];
    f.read_u16_into::<BE>(&mut spawns_raw)
        .chain_err(||"reading spawn points area as u16 BE")?;
    println!("Debug spawn points:\n{:?}", spawns_raw);

    let spawn_res: Result<Vec<_>> = spawns_raw.chunks(3).map(Spawn::from_raw).collect();
    let spawns = spawn_res.chain_err(||"converting raw u16 slice into Spawn vec")?;
    println!("Spawn points:\n{:#?}", spawns);


    Ok(format!("Not Fully Implemented"))
}

#[derive(Debug)]
struct Spawn {
    id: u16,
    x: i16,
    y: i16
}

impl Spawn {
    fn from_raw(points: &[u16]) -> Result<Self> {
        if points.len() < 3 {
            return Err(format!("input slice {:?} to small for Spawn::from_raw",points).into())
        }
        Ok(Spawn{
            id: points[0],
            x: points[1] as i16,
            y: points[2] as i16
        })
    }
}

#[derive(Debug)]
struct PlaneInfo {
    start: u16,
    length: u16
}
impl PlaneInfo {
    fn new(s: u16, l: u16) -> Self {
        PlaneInfo{ start: s, length: l}
    }
}
fn to_plane_info(i: &u32) -> PlaneInfo {
    let s = ((*i & 0xFFFF0000) >> 16) as u16;
    let l = (*i & 0x0000FFFF) as u16;

    PlaneInfo::new(s, l)
}

#[derive(Debug)]
struct ColDetection {
    unk1 : u16,
    top_start: u16,
    top_size: u16,
    bottom_start: u16,
    bottom_size: u16,
    right_start: u16,
    right_size: u16,
    left_start: u16,
    left_size: u16
}

impl ColDetection {
    fn new(raw: &[u16; 9]) -> Self {
        ColDetection {
            unk1: raw[0],
            top_start: raw[1],
            top_size: raw[2],
            bottom_start: raw[3],
            bottom_size: raw[4],
            right_start: raw[5],
            right_size: raw[6],
            left_start: raw[7],
            left_size: raw[8]
        }
    }
    fn calc_total_planes(&self) -> u16 {
        let s = self;

        *[s.top_start + s.top_size,
         s.bottom_start + s.bottom_size,
         s.right_start + s.right_size,
         s.left_start + s.left_size
        ].iter().max().unwrap()
    }

}

// Possible safe strat:
// Read collision direction struct since it's a fixed size of 0x14 bytes
//   map( starting point + number of surfaces ) -> max() = highest used plane
//   read highest_used_plane * 4 + 4 to get size in bytes of plane array
// Read plane array
//   map( starting point + len) -> max() * 4 = length in bytes of connections array
// Read connections array
//   (max() + 1) * 6 -> length in bytes of points array
// Read Points array


#[derive(Debug)]
struct CollisionPoint {
    x: i16,
    y: i16,
    prop_flag: ColProperty,
    floor_type: Floor
}

impl CollisionPoint {
    fn from_raw(i: &[u16]) -> Result<Self> {
        if i.len() < 3 {
            return Err("input slice to CollisionPoint::from_raw too small".into())
        }

        let x = i[0] as i16;
        let y = i[1] as i16;
        let flag = ((i[2] & 0xFF00) >> 8) as u8;
        let floor = (i[2] & 0xFF) as u8;

        let prop_flag = ColProperty::from_bits(flag)
            .ok_or(format!("Unknown collision property {:#X}", flag))?;
        let floor_type = Floor::from_bits(floor)
            .ok_or(
                format!("Unable to convert \"{:#X}\" to a floor type. Values should range 0 to 0xF", floor)
            )?;
        println!("Debug flag: {:X} == {:X} == {:?}", i[2], flag, prop_flag);

        Ok(CollisionPoint{x, y, prop_flag, floor_type})
    }
}

#[derive(Debug)]
#[allow(dead_code, non_camel_case_types)]
enum Floor {
    Normal       = 0x00,
    Fric1        = 0x01,
    Fric2        = 0x02,
    Fric3        = 0x03,
    Fric4        = 0x04,
    Fric5        = 0x05,
    Fric6        = 0x06,
    LavaSideways = 0x07,
    Acid         = 0x08,
    LavaUp10     = 0x09,
    Spikes       = 0x0A,
    LavaUp1_B    = 0x0B,
    Unk1         = 0x0C,
    Unk2         = 0x0D,
    BtPPlatform  = 0x0E,
    LavaUp1_F    = 0x0F
}


use std::mem;
impl Floor {
    fn from_bits(bits: u8) -> Option<Floor> {
        match bits {
            b @ 0...0x0F => unsafe {
                Some(mem::transmute::<u8, Floor>(b))
            },
            _ => None
        }
    }
}

bitflags! {
    #[derive(Default)]
    struct ColProperty: u8 {
        const FALL_THRU  = 0b10000000;
        const LEDGE_GRAB = 0b01000000;
        const NORMAL     = 0b00000000;
    }
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
    col_direct: u32,
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
    col_direct:  {:#010X},
    spawn_count: {:#06X},
    spawns:      {:#010X}
}}", s.unk1, s.points, s.connections, s.planes, s.col_direct, s.spawn_count, s.spawns)
    }
}

fn get_collisions_ptrs(arr: &[u8]) -> IoResult<ColPtrs> {
    let mut c = Cursor::new(arr);
    // read some values
    let unk1 = c.read_u16::<BE>()?;
    //skip two byte pad
    c.seek(SeekFrom::Current(2))?;
    // read some pointer
    let points = c.read_u32::<BE>().and_then(check_res_ptr)?;
    let connections = c.read_u32::<BE>().and_then(check_res_ptr)?;
    let planes = c.read_u32::<BE>().and_then(check_res_ptr)?;
    let col_direct = c.read_u32::<BE>().and_then(check_res_ptr)?;
    let spawn_count = c.read_u16::<BE>()?;
    c.seek(SeekFrom::Current(2))?;
    let spawns = c.read_u32::<BE>().and_then(check_res_ptr)?;

    Ok(ColPtrs {
        unk1, points, connections, planes,
        col_direct, spawn_count, spawns
    })
}
