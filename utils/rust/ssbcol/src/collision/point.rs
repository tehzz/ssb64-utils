use std::fmt;
use serde::ser::{Serialize, Serializer};
use serde::de::{self, Visitor, Deserialize, Deserializer};
use errors::*;
use byteorder::{BE, WriteBytesExt};
use std::io::Cursor;
use traits::N64Bytes;

/// An (x,y) point for a collision plane
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CollisionPoint {
    x: i16,
    y: i16,
    #[serde(rename = "property")]
    prop_flag: ColProp,
    #[serde(rename = "floor")]
    floor_type: Floor
}

impl N64Bytes for CollisionPoint {
    type Output = [u8; 6];

    fn size() -> usize {6}
    
    fn to_bytes(&self) -> [u8; 6] {
        let mut output = [0u8; 6];
        {
            let mut csr = Cursor::new(output.as_mut());
            csr.write_i16::<BE>(self.x).unwrap();
            csr.write_i16::<BE>(self.y).unwrap();
            csr.write_u8(self.prop_flag.bits()).unwrap();
            csr.write_u8(self.floor_type as u8).unwrap();
        }
        output
    }
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

        let floor_type = Floor::from_bits(floor)?;

        Ok(CollisionPoint{x, y, prop_flag, floor_type})
    }
}

enum_bits! {
    #[derive(Serialize, Deserialize)]
    #[allow(non_camel_case_types)]
    #[serde(rename_all = "kebab-case")]
    enum Floor: u8 {
        Normal       = 0x00,
        Friction0x1  = 0x01,
        Friction0x2  = 0x02,
        Friction0x3  = 0x03,
        Friction0x4  = 0x04,
        Friction0x5  = 0x05,
        Friction0x6  = 0x06,
        LavaSideways = 0x07,
        Acid         = 0x08,
        LavaUp10     = 0x09,
        Spikes       = 0x0A,
        LavaUp1_0xb  = 0x0B,
        Unk0x0c      = 0x0C,
        Unk0x0d      = 0x0D,
        BtPPlatform  = 0x0E,
        LavaUp1_0xf  = 0x0F,
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

// Serde manual deserialization
struct ColPropVisitor;

impl<'de> Visitor<'de> for ColPropVisitor {
    type Value = ColProp;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a string with the values \"ledge_grab\", \"fall_thru\", or \"normal\". \
        Can be combined with \" | \"")
    }

    fn visit_str<E>(self, v: &str) -> ::std::result::Result<Self::Value, E>
        where E: de::Error
    {
        let ledge = v.contains("ledge_grab");
        let fall  = v.contains("fall_thru");
        let norm  = v.contains("normal");

        if ledge && fall {
            Ok(LEDGE_GRAB | FALL_THRU)
        } else if ledge && !fall {
            Ok(LEDGE_GRAB)
        } else if fall && !ledge {
            Ok(FALL_THRU)
        } else if norm {
            Ok(Default::default())
        } else {
            Err(E::custom(format!("Improper \"property\" (ColProp) flag string <{}>", v)))
        }
    }
}

impl<'de> Deserialize<'de> for ColProp {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<ColProp, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_str(ColPropVisitor)
    }
}
