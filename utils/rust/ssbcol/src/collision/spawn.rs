use std::fmt;
use errors::*;

/// This struct represents a spawn point in ssb64
#[derive(Debug, Serialize)]
pub struct Spawn {
    #[serde(rename = "type")]
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
