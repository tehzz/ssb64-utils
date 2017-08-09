/// This `struct` represents the "main stage file" in ssb64. This is the file that points to
/// all other components of the stage (geometry, collision, background, etc.).
struct StageMain {
    ItemBytes: Option<[u8; 14]>,
    ItemBytesPtr: Option<u32>,
    Geometries: [StageGeo; 4],
    CollisionPtr: u32,
    Unknown0x44: u32,
    BkgdPtr: u32,
    MagnifierColor: Color,
    PlayerLogoColor: [Color; 4],
    Lighting1: f32,
    Lighting2: f32,
    CameraXTilt: f32,
    CameraBoxes: CameraBox,
    BlastZones: BlastZones,
    BkgdMusic: u32,
    Pad0x80: u32,
    FallingWhistle: i16,
    ExtraInfo: Option<Vec<u8>>,
}

/// A 32bit color wrapper -> (R, G, B, A)
struct Color(u8, u8, u8, u8)

/// A struct wrapper to represent the +Y, -Y, +X, -X
struct Coords(i16, i16, i16, i16)

/// Holds the Coords for both the normal and 1P CPU Blastzones
struct BlastZones {
    Normal: Coords,
    Cpu1P: Coords,
}

/// Holds the camera frame "bounding box" for the both VS and 1P Mode
struct CameraBox {
    VsMode: Coords,
    OnePlayer: Coords,
}
