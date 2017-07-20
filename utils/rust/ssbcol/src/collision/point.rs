use std::mem;
use errors::*;
use serde::ser::{Serialize, Serializer};

/// An (x,y) point for a collision plane
#[derive(Debug, Serialize)]
pub struct CollisionPoint {
    x: i16,
    y: i16,
    #[serde(rename = "property")]
    prop_flag: ColProp,
    #[serde(rename = "floor")]
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

        let prop_flag = ColProp::from_bits(flag)
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
#[serde(rename_all = "camelCase")]
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
    #[derive(Default)]
    struct ColProp: u8 {
        const LEDGE_GRAB = 0b10000000;
        const FALL_THRU  = 0b01000000;
    }
}

impl Serialize for ColProp {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let name = format!("{:?}",self).to_lowercase();
        match self.bits {
            0x40 | 0x80 | 0xC0 => serializer.serialize_str(&name),
            _    => serializer.serialize_str("normal"),
        }
    }
}
