use errors::*;



/// This `struct` represents the "main stage file" in ssb64. This is the file that points to
/// all other components of the stage (geometry, collision, background, etc.).
struct StageMain {
    item_bytes: Option<[u8; 14]>,
    item_bytes_ptr: Option<u32>,
    geometries: [StageGeo; 4],
    collision_ptr: u32,
    unknown_0x44: u32,
    background_ptr: u32,
    magnifier_color: Color,
    player_logo_color: [Color; 4],
    lighting1: f32,
    lighting2: f32,
    camera_tilt: f32,
    camera_bounds: CameraBox,
    blastzones: BlastZones,
    background_music: u32,
    pad_0x80: u32,
    falling_whistle: i16,
    extra_info: Option<Vec<u8>>,
}

impl StageMain {
    from_bytes(main: &[u8], item_bytes: Option<&[u8]>, extra_info: Option<&[u8]>)
    -> Result<Self>
    {
        // main.len < 0xa8 ; item_bytes < 0x14
    }
}

/// A 32bit color wrapper -> (R, G, B, A)
#[#[derive(Debug, Copy, Clone, PartialEq, Eq)]]
struct Color{
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
impl Color {
    from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 4 {
            bail!("Need four bytes to generate a Color; got {:?}", &bytes)
        };
        let r = bytes[0];
        let g = bytes[1];
        let b = bytes[2];
        let a = bytes[3];

        Ok(Color{ r, g, b, a })
    }
    as_bytes(&self) -> [u8; 4] {
        let s = self;
        [s.r, s.g, s.b, s.a]
    }
}

/// A struct wrapper to represent the +Y, -Y, +X, -X
struct Bounds {
    top: i16,
    bottom: i16,
    right: i16,
    left: i16,
}

impl Bounds {
    from_i16(&[i16]) -> Self {

    }
    from_bytes(&[u8], le: bool) -> Self {
        
    }
}

/// Holds the Coords for both the normal and 1P CPU Blastzones
struct BlastZones {
    Normal: Bounds,
    Cpu1P: Bounds,
}

/// Holds the camera frame "bounding box" for the both VS and 1P Mode
struct CameraBox {
    VsMode: Bounds,
    OnePlayer: Bounds,
}
