use std::fmt;
use std::mem;
use errors::*;

/// This structure represents which planes have collision from which direction
/// There can be any number (?) of sets to define different collisions
#[derive(Debug)]
pub struct ColDetection {
    id : u16,
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
    pub fn from_raw(raw: &[u16; 9]) -> Self {
        ColDetection {
            id: raw[0],
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
    pub fn calc_total_planes(&self) -> u16 {
        let s = self;

        *[s.top_start + s.top_size,
         s.bottom_start + s.bottom_size,
         s.right_start + s.right_size,
         s.left_start + s.left_size
        ].iter().max().unwrap()
    }
    fn get_directon(&self, direction:Side) -> (u16, u16) {
        let s = self;

        match direction {
            Side::Top => (s.top_start, s.top_size),
            Side::Bottom => (s.bottom_start, s.bottom_size),
            Side::Right => (s.right_start, s.right_size),
            Side::Left => (s.left_start, s.left_size),
        }
    }
    pub fn get_top(&self) -> (u16, u16){ self.get_directon(Side::Top)}
    pub fn get_bottom(&self) -> (u16, u16){ self.get_directon(Side::Bottom)}
    pub fn get_right(&self) -> (u16, u16){ self.get_directon(Side::Right)}
    pub fn get_left(&self) -> (u16, u16){ self.get_directon(Side::Left)}
    pub fn get_id(&self) -> u16 { self.id }
}

enum Side {
    Top,
    Bottom,
    Right,
    Left
}

/// This struct represents a spawn point in ssb64
#[derive(Debug, Serialize)]
pub struct Spawn {
    stype: u16,
    x: i16,
    y: i16
}

impl fmt::Display for Spawn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self;

        write!(f, "Spawn {{
    stype: {:#06X},
    x:     {},
    y:     {},\n}}", s.stype, s.x, s.y)
    }
}

impl Spawn {
    pub fn from_raw(points: &[u16]) -> Result<Self> {
        if points.len() < 3 {
            return Err(format!("input slice {:?} to small for Spawn::from_raw",points).into())
        }
        Ok(Spawn{
            stype: points[0],
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
#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
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
    Unk0xC       = 0x0C,
    Unk0xD       = 0x0D,
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
    #[derive(Default, Serialize)]
    struct ColProperty: u8 {
        const LEDGE_GRAB = 0b10000000;
        const FALL_THRU  = 0b01000000;
        const NORMAL     = 0b00000000;
    }
}
