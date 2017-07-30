use std::fmt;
use errors::*;
use byteorder::{BE, WriteBytesExt};
use std::io::Cursor;
use traits::N64Bytes;

/// This struct represents a spawn point in ssb64
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Spawn {
    #[serde(rename = "type")]
    stype: SpawnType,
    x: i16,
    y: i16
}

impl fmt::Display for Spawn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self;

        write!(f, "Spawn {{
    stype: {:?},
    x:     {},
    y:     {},\n}}", s.stype, s.x, s.y)
    }
}

impl N64Bytes for Spawn {
    type Output = [u8; 6];

    fn size() -> usize {6}

    fn to_bytes(&self) -> [u8; 6] {
        let mut output = [0u8; 6];
        {
            let mut csr = Cursor::new(output.as_mut());
            csr.write_u16::<BE>(self.stype as u16).unwrap();
            csr.write_i16::<BE>(self.x).unwrap();
            csr.write_i16::<BE>(self.y).unwrap();
        }
        output
    }
}

impl Spawn {
    pub fn from_raw(points: &[u16]) -> Result<Self> {
        if points.len() < 3 {
            return Err(format!("input slice {:?} to small for Spawn::from_raw",points).into())
        }
        let stype = SpawnType::from_bits(points[0])?;

        Ok(Spawn{
            stype,
            x: points[1] as i16,
            y: points[2] as i16,
        })
    }
}

enum_bits! {
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    enum SpawnType: u16 {
        P1SpawnVs = 0x00,
        P2SpawnVs = 0x01,
        P3SpawnVs = 0x02,
        P4SpawnVs = 0x03,
        ItemSpawn = 0x04,
        TornadoSpawn = 0x0D,
        BumperSpawn  = 0x13,
        Unk0x15      = 0x15,
        Unk0x18      = 0x18,
        Unk0x19      = 0x19,
        Unk0x1a      = 0x1A,
        Unk0x1b      = 0x1B,
        Unk0x1c      = 0x1C,
        Unk0x1d      = 0x1D,
        Unk0x1e      = 0x1E,
        Unk0x1f      = 0x1F,
        VsRespawn    = 0x20,
        PlayerSpawn1p = 0x21,
        Ally1Spawn1p  = 0x22,
        Ally2Spawn1p  = 0x23,
        Ally3Spawn1p  = 0x24,
        Cpu1Spawn1p   = 0x25,
        Cpu2Spawn1p   = 0x26,
        Cpu3Spawn1p   = 0x27,
        CpuRespawn1p  = 0x2B,
        Unk0x23      = 0x2C,
        Unk0x2d      = 0x2D,
    }
}
