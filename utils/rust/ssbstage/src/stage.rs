use errors::*;
use byteorder::{BE, ReadBytesExt};
use std::io::{Cursor, Seek, SeekFrom};


/// This `struct` represents the "main stage file" in ssb64. This is the file that points to
/// all other components of the stage (geometry, collision, background, etc.). It also contains
/// some "general" information about the stage.
#[derive(Debug, Serialize)]
pub struct StageMain {
    item_bytes: Option<[u8; 0x14]>,
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
    background_music: BGM,
    pad_0x80: u32,
    falling_whistle: i16,
    unknown_0x9c: u32,
    unknown_0xa0: u32,
    end_of_file_0xa4: u32,
    extra_info: Option<Vec<u8>>,
}

impl StageMain {
    /// This assumes that the underlying bytes are Big-Endian (n64)
    pub fn from_bytes(main: &[u8], item_bytes: Option<&[u8]>, extra_info: Option<&[u8]>)
    -> Result<Self>
    {
        // sanity checks to ensure minimum sized slices
        if main.len() < 0xa8 {
            bail!("Main stage data was less than 0xa8 bytes")
        } else if let Some(ref b) = item_bytes {
            if b.len() < 0x14 {
                bail!("Item data was less than 0x14 bytes")
            }
        }

        // start parsing the main stage data
        let mut csr = Cursor::new(main);

        // Four StageGeo structs in a row
        let mut geometries: [StageGeo; 4] = [StageGeo::default(); 4];
        for geo in geometries.iter_mut() {
            let mut bytes = [0u32; 4];
            csr.read_u32_into::<BE>(&mut bytes)?;
            *geo = StageGeo::from_bytes(&bytes);
        }
        // Various Pointers
        let collision_ptr  = csr.read_u32::<BE>()?;
        let unknown_0x44   = csr.read_u32::<BE>()?;
        let background_ptr = csr.read_u32::<BE>()?;
        // 5 color structs in a row!
        let magnifier_color = {
            let color = csr.read_u32::<BE>()?;
            Color::from_u32(color)
        };
        let mut player_logo_color = [Color::default(); 4];
        for player in player_logo_color.iter_mut() {
            let color = csr.read_u32::<BE>()?;
            *player = Color::from_u32(color);
        }
        // 3 float32 values for lighting and camera
        let lighting1   = csr.read_f32::<BE>()?;
        let lighting2   = csr.read_f32::<BE>()?;
        let camera_tilt = csr.read_f32::<BE>()?;

        // get camera boundries
        let camera_bounds = {
            let mut vals = [0i16; 4];
            csr.read_i16_into::<BE>(&mut vals)?;
            let vs = Bounds::from_i16(&vals);
            csr.seek(SeekFrom::Start(0x8a))?;
            csr.read_i16_into::<BE>(&mut vals)?;
            let single = Bounds::from_i16(&vals);

            CameraBox::from_bounds(vs, single)
        };
        // get blastzones
        let blastzones = {
            let mut vals = [0i16; 4];
            //cursor already at 1p mode blastzones
            csr.read_i16_into::<BE>(&mut vals)?;
            let single = Bounds::from_i16(&vals);
            // move cursor back to regular blastzones + back "in order"
            csr.seek(SeekFrom::Start(0x74))?;
            csr.read_i16_into::<BE>(&mut vals)?;
            let regular = Bounds::from_i16(&vals);

            BlastZones::from_bounds(regular, single)
        };
        // back to reading sequentially at 0x7c
        let background_music = BGM::from_bits(csr.read_u32::<BE>()?)?;
        let pad_0x80         = csr.read_u32::<BE>()?;
        let item_bytes_ptr = if item_bytes.is_some() {
            Some(csr.read_u32::<BE>()?)
        } else {
            csr.seek(SeekFrom::Current(4))?;
            None
        };
        let falling_whistle  = csr.read_i16::<BE>()?;

        // as yet unknown values?
        csr.seek(SeekFrom::Start(0x9c))?;
        let unknown_0x9c = csr.read_u32::<BE>()?;
        let unknown_0xa0 = csr.read_u32::<BE>()?;
        let end_of_file_0xa4 = csr.read_u32::<BE>()?;

        // Convert the optional parts of the file, if present
        let item_bytes = item_bytes.map(|vals| {
            let mut arr = [0u8;0x14];
            for i in 0..0x14 {
                arr[i] = vals[i];
            }

            arr
        });

        let extra_info = extra_info.map(|slice| slice.to_vec() );

        // finally, return the very large mess
        Ok(StageMain {
            item_bytes,
            item_bytes_ptr,
            geometries,
            collision_ptr,
            unknown_0x44,
            background_ptr,
            magnifier_color,
            player_logo_color,
            lighting1,
            lighting2,
            camera_tilt,
            camera_bounds,
            blastzones,
            background_music,
            pad_0x80,
            falling_whistle,
            unknown_0x9c,
            unknown_0xa0,
            end_of_file_0xa4,
            extra_info,
        })
    }
}

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, Default)]
struct StageGeo {
    geometry_ptr: u32,
    move_script_ptr: u32,
    colored_ptr: u32,
    colored_script_ptr: u32,
}

impl StageGeo {
    fn from_bytes(input: &[u32; 4]) -> Self {
        StageGeo {
            geometry_ptr:       input[0],
            move_script_ptr:    input[1],
            colored_ptr:        input[2],
            colored_script_ptr: input[3],
        }
    }
}

/// A 32bit color wrapper. Will accept/return bytes in RGBA8888
#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, Default)]
struct Color{
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
impl Color {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 4 {
            bail!("Need four bytes to generate a Color; got {:?}", &bytes)
        };
        let r = bytes[0];
        let g = bytes[1];
        let b = bytes[2];
        let a = bytes[3];

        Ok(Color{ r, g, b, a })
    }
    fn as_bytes(&self) -> [u8; 4] {
        let s = self;
        [s.r, s.g, s.b, s.a]
    }
    fn from_u32(val: u32) -> Self {
        let r = (val >> 24) as u8;
        let g = ((val & 0x00FF0000) >> 16) as u8;
        let b = ((val & 0x0000FF00) >>  8) as u8;
        let a = (val & 0x000000FF) as u8;

        Color{ r, g, b, a }
    }
}

/// A struct wrapper to represent the +Y, -Y, +X, -X "limits" of a box
#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, Default)]
struct Bounds {
    top: i16,
    bottom: i16,
    right: i16,
    left: i16,
}

impl Bounds {
    fn from_i16(input: &[i16; 4]) -> Self {
        Bounds {
            top: input[0],
            bottom: input[1],
            right: input[2],
            left: input[3],
        }
    }
}

/// Holds the Coords for both the normal and 1P CPU Blastzones. The normal
/// blastzones are used at all times for human controlled characters. The 1P Mode
/// CPU blastzones are used only for the computer controlled characters in the
/// 1P mode (surprise!).
#[derive(Serialize, Debug, Copy, Clone)]
struct BlastZones {
    regular: Bounds,
    cpu_1p: Bounds,
}

impl BlastZones {
    fn from_bounds(regular: Bounds, cpu_1p: Bounds) -> Self {
        BlastZones{regular, cpu_1p}
    }
}

/// Holds the camera frame "bounding box" for the both VS and 1P Game Mode. The set
/// "Bounds" struct defines how far the camera will zoom out (relative to the distance
/// between multiple characters). If a character goes outside of this bound, they will
/// be in a maginfying glass
#[derive(Serialize, Debug, Copy, Clone)]
struct CameraBox {
    versus: Bounds,
    one_player: Bounds,
}

impl CameraBox {
    fn from_bounds(versus: Bounds, one_player: Bounds) -> Self {
        CameraBox{versus, one_player}
    }
}

/// This is the list of background music (BGM) tracks in ssb64

enum_bits!{
    #[derive(Serialize)]
    enum BGM: u32 {
        Dreamland       = 0x00,
        PlanetZebes     = 0x01,
        MushroomKingdom = 0x02,
        MushroomKingdomFast = 0x03,
        SectorZ         = 0x04,
        CongoJungle     = 0x05,
        PeachsCastle    = 0x06,
        SaffronCity     = 0x07,
        YoshisIsland    = 0x08,
        HyruleCastle    = 0x09,
        CharacterSelect = 0x0a,
        BetaFanfare     = 0x0b,
        MarioWins       = 0x0c,
        SamusWins       = 0x0d,
        DKWins          = 0x0e,
        KirbyWins       = 0x0f,
        FoxWins         = 0x10,
        NessWins        = 0x11,
        YoshiWins       = 0x12,
        CaptainFalconWins = 0x13,
        PokemonWins     = 0x14,
        LinkWins        = 0x15,
        ResultsScreen   = 0x16,
        PreMasterHand1  = 0x17,
        PreMasterHand2  = 0x18,
        FinalDestination = 0x19,
        BonusStage      = 0x1a,
        StageClear      = 0x1b,
        BonusStageClear = 0x1c,
        MasterHandClear = 0x1d,
        BonusStageFailure = 0x1e,
        Continue        = 0x1f,
        GameOver        = 0x20,
        Intro           = 0x21,
        HowtoPlay       = 0x22,
        Pre1PBattle     = 0x23,
        Battlefield     = 0x24,
        MetalCavern     = 0x25,
        GameComplete    = 0x26,
        Credits         = 0x27,
        Secret          = 0x28,
        HiddenCharacter = 0x29,
        TrainingMode    = 0x2a,
        VSRecord        = 0x2b,
        MainMenu        = 0x2c,
        Hammer          = 0x2d,
        Invincibility   = 0x2E,
    }
}
