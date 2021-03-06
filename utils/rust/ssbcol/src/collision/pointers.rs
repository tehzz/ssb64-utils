use std::fmt;
use errors::*;
use byteorder::{BE, WriteBytesExt};
use std::io::{Cursor, Seek, SeekFrom};

#[derive(Debug)]
pub struct ColPtrs {
    pub col_count: u16,
    pub points: u32,
    pub connections: u32,
    pub planes: u32,
    pub col_direct: u32,
    pub spawn_count: u16,
    pub spawns: u32
}
impl ColPtrs {
    pub fn from_raw(i: &[u32; 7]) -> Result<Self> {
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
    pub fn new_null() -> Self {
        ColPtrs {
            col_count: 0,
            points: 0,
            connections: 0,
            planes: 0,
            col_direct: 0,
            spawn_count: 0,
            spawns: 0,
        }
    }
    pub fn to_bytes(&self) -> Result<[u8; 28]> {
        let mut output = [0u8; 28];
        {
            let mut csr = Cursor::new(output.as_mut());
            csr.write_u16::<BE>(self.col_count)?;
            csr.seek(SeekFrom::Current(2))?;
            csr.write_u32::<BE>(self.points)?;
            csr.write_u32::<BE>(self.connections)?;
            csr.write_u32::<BE>(self.planes)?;
            csr.write_u32::<BE>(self.col_direct)?;
            csr.write_u16::<BE>(self.spawn_count)?;
            csr.seek(SeekFrom::Current(2))?;
            csr.write_u32::<BE>(self.spawns)?;
        }
        Ok(output)
    }
    /// Convert the interal pointers to reference each other,
    /// and link to the next pointer (if necessary)
    pub fn to_resource_pointers(&mut self, offset: u32, next_chain: Option<u32> ) {
        // adjust initial offset to point to self.connections
        let mut next_ptr = offset + 0x8;
        self.points = make_res_ptr(Some(next_ptr), self.points);
        // point to self.planes
        next_ptr += 0x4;
        self.connections = make_res_ptr(Some(next_ptr), self.connections);
        // point to self.col_direct
        next_ptr += 0x4;
        self.planes = make_res_ptr(Some(next_ptr), self.planes);
        // point to self.spawns
        next_ptr += 0x8;
        self.col_direct = make_res_ptr(Some(next_ptr), self.col_direct);
        // encode final pointer to the next chain (if there a next chain)
        self.spawns = make_res_ptr(next_chain, self.spawns);
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

fn cnvrt_res_ptr(input: u32) -> u32 {
    // two MSB == 0x80, probably pointer from a RAM dump
    if (input >> 24) == 0x80 {
        input
    } else {
        // assume resource file; take lower half and convert from words to bytes
        (input & 0xFFFF) << 2
    }
}

fn make_res_ptr(next: Option<u32>, ptr: u32) -> u32 {
    // resource pointers are word offsets: u16 next u16 pointer
    let u16_word_ptr = (ptr >> 2) & 0xFFFF;
    match next {
        Some(next_ptr) => (((next_ptr >> 2) & 0xFFFF) << 16) | u16_word_ptr,
        None           => (0xFFFF << 16) | u16_word_ptr,
    }
}
