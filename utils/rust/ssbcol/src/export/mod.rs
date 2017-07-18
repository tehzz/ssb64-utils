use configs::{ExportConfig};
use errors::*;
use byteorder::{BE, ReadBytesExt};
use std::io::{Seek, SeekFrom};
use std::fmt;
use collision::{Spawn, PlaneInfo, CollisionPoint, ColDetection};

pub fn export_collision(config: ExportConfig) -> Result<String> {
    let mut f = config.input;
    let ptr = config.col_ptr;
    // array for collision pointers data : [u8; 0x1C]
    let mut col_ptrs_raw = [0u32; 7];

    f.seek(SeekFrom::Start(ptr as u64))
        .chain_err(||
            format!("seeking to collision pointers at 0x{:08X}", ptr)
        )?;
    f.read_u32_into::<BE>(&mut col_ptrs_raw)
        .chain_err(||"reading 0x1C bytes for collision pointers")?;

    let main_ptrs = ColPtrs::from_raw(&col_ptrs_raw)
        .chain_err(|| "formatting collision pointers")?;

    println!("Testing reading of pointer struct:\n{}", main_ptrs);

    // get all collision detection structs
    f.seek(SeekFrom::Start(main_ptrs.col_direct as u64))
        .chain_err(||
            format!("seeking to collision detection struct at {:#010X}", main_ptrs.col_direct)
        )?;
    let mut col_detect: Vec<ColDetection> = Vec::with_capacity(main_ptrs.col_count as usize);

    for _ in 0..main_ptrs.col_count {
        let mut col_detect_raw = [0u16; 0x9];
        f.read_u16_into::<BE>(&mut col_detect_raw)
        .chain_err(|| "reading 0x12 bytes for collision detection struct")?;

        col_detect.push(ColDetection::from_raw(&col_detect_raw));
    }

    println!("Debug collision detections struct: \n{:#?}", &col_detect);
    let total_planes = col_detect.iter().map(|dect| dect.calc_total_planes()).max().unwrap() as usize;
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
    //println!("Debug spawn points:\n{:?}", spawns_raw);

    let spawn_res: Result<Vec<_>> = spawns_raw.chunks(3).map(Spawn::from_raw).collect();
    let spawns = spawn_res.chain_err(||"converting raw u16 slice into Spawn vec")?;
    for s in spawns.iter() {
        println!("{}", s);
    };


    Ok(format!("Not Fully Implemented"))
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

fn to_plane_info(i: &u32) -> PlaneInfo {
    let s = ((*i & 0xFFFF0000) >> 16) as u16;
    let l = (*i & 0x0000FFFF) as u16;

    PlaneInfo::new(s, l)
}

fn cnvrt_res_ptr(input: u32) -> u32 {
    // two MSB == 0x80, probably pointer from a RAM dump
    if (input >> 24) == 0x80 {
        input
    } else {
        // assume resource file; take lower half and convert from words to bytes
        (input & 0xFFFF) << 2
    }
}

#[derive(Debug)]
struct ColPtrs {
    col_count: u16,
    points: u32,
    connections: u32,
    planes: u32,
    col_direct: u32,
    spawn_count: u16,
    spawns: u32
}
impl ColPtrs {
    fn from_raw(i: &[u32; 7]) -> Result<Self> {
        let col_count = ((i[0] & 0xFFFF0000)>> 16) as u16;
        if col_count == 0 {
            return Err("Read initial collision count u16 as 0; \
            Pointer to collision pointer struct maybe incorrect".into())
        }

        let points      = cnvrt_res_ptr(i[1]);
        let connections = cnvrt_res_ptr(i[2]);
        let planes      = cnvrt_res_ptr(i[3]);
        let col_direct  = cnvrt_res_ptr(i[4]);
        let spawn_count = ((i[5] & 0xFFFF0000)>> 16) as u16;
        let spawns      = cnvrt_res_ptr(i[6]);

        Ok(ColPtrs{col_count, points, connections, planes,
                    col_direct, spawn_count, spawns})
    }
}
impl fmt::Display for ColPtrs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self;

        write!(f, "ColPtrs {{
    col_count:   {:#06X},
    points:      {:#010X},
    connections: {:#010X},
    planes:      {:#010X},
    col_direct:  {:#010X},
    spawn_count: {:#06X},
    spawns:      {:#010X}\n}}",
        s.col_count, s.points, s.connections, s.planes,
        s.col_direct, s.spawn_count, s.spawns)
    }
}
