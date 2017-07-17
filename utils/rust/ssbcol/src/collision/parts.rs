use std::fmt;
use std::mem;
use errors::*;

/// This struct represents a spawn point in ssb64
#[derive(Debug)]
pub struct Spawn {
    id: u16,
    x: i16,
    y: i16
}

impl fmt::Display for Spawn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self;

        write!(f, "Spawn {{
    id: {:#06X},
    x:  {},
    y:  {},\n}}", s.id, s.x, s.y)
    }
}

impl Spawn {
    pub fn from_raw(points: &[u16]) -> Result<Self> {
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

/// PlaneInfo defines an offset and length into the plane array (which is an array of offsets into the
// collision points array) to define a collision plane
#[derive(Debug)]
pub struct PlaneInfo {
    pub start: u16,
    pub length: u16
}
impl PlaneInfo {
    pub fn new(s: u16, l: u16) -> Self {
        PlaneInfo{ start: s, length: l}
    }
}

/// An (x,y) point for a collision plane
#[derive(Debug)]
pub struct CollisionPoint {
    x: i16,
    y: i16,
    prop_flag: ColProperty,
    floor_type: Floor
}

impl CollisionPoint {
    pub fn from_raw(i: &[u16]) -> Result<Self> {
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
