use errors::*;
use byteorder::{BE, ReadBytesExt, ByteOrder, WriteBytesExt};
use std::io::{Write, Cursor, Seek, SeekFrom};
use ssbpointers::SSBPtr;

/// This `struct` represents the "main stage file" in ssb64. This is the file that points to
/// all other components of the stage (geometry, collision, background, etc.). It also contains
/// some "general" information about the stage.
#[derive(Debug, Serialize, Deserialize)]
pub struct StageMain {
    item_bytes: Option<[u8; 0x14]>,
    item_bytes_ptr: SSBPtr,
    geometries: [StageGeo; 4],
    collision_ptr: SSBPtr,
    unknown_0x44: u32,
    background_ptr: SSBPtr,
    magnifier_color: Color,
    player_logo_color: [Color; 4],
    lighting1: f32,
    lighting2: f32,
    camera_tilt: f32,
    camera_bounds: CameraBox,
    blastzones: BlastZones,
    background_music: BGM,
    other_movescript_ptr: SSBPtr,
    falling_whistle: i16,
    bonus_pause_camera: BonusPauseCamera,
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
            if b.len() != 0x14 {
                bail!("Item data was not 0x14 bytes")
            }
        }

        // start parsing the main stage data; it should have a fixed size of 0xa8 bytes (or 0xa6 aligned)
        let mut csr = Cursor::new(main);

        // Four StageGeo structs in a row [0x00..0x40]
        let mut geometries: [StageGeo; 4] = [StageGeo::default(); 4];
        for geo in geometries.iter_mut() {
            let mut bytes = [0u32; 4];
            csr.read_u32_into::<BE>(&mut bytes)?;
            *geo = StageGeo::from_u32s(&bytes);
        }
        // Read two pointers sandwiching a unknown word (...byte maybe?) [0x40..0x4c]
        let collision_ptr  = SSBPtr::from_u32(csr.read_u32::<BE>()?);
        let unknown_0x44   = csr.read_u32::<BE>()?;
        let background_ptr = SSBPtr::from_u32(csr.read_u32::<BE>()?);
        // 5 color structs in a row! [0x4c..0x60]
        let magnifier_color = {
            let color = csr.read_u32::<BE>()?;
            Color::from_u32(color)
        };
        let mut player_logo_color = [Color::default(); 4];
        for player in player_logo_color.iter_mut() {
            let color = csr.read_u32::<BE>()?;
            *player = Color::from_u32(color);
        }
        // 3 float32 values for lighting and camera [0x60..0x6c]
        let lighting1   = csr.read_f32::<BE>()?;
        let lighting2   = csr.read_f32::<BE>()?;
        let camera_tilt = csr.read_f32::<BE>()?;

        // get camera boundries [0x6c..0x74] and [0x8a..0x92]
        let camera_bounds = {
            let mut vals = [0i16; 4];
            csr.read_i16_into::<BE>(&mut vals)?;
            let vs = Bounds::from_i16_arr(&vals);
            csr.seek(SeekFrom::Start(0x8a))?;
            csr.read_i16_into::<BE>(&mut vals)?;
            let single = Bounds::from_i16_arr(&vals);

            CameraBox::from_bounds(vs, single)
        };
        // get blastzones [0x74..0x7c] [0x92..0x9a]
        let blastzones = {
            let mut vals = [0i16; 4];
            //cursor already at 1p mode blastzones
            csr.read_i16_into::<BE>(&mut vals)?;
            let single = Bounds::from_i16_arr(&vals);
            // move cursor back to regular blastzones + back "in order"
            csr.seek(SeekFrom::Start(0x74))?;
            csr.read_i16_into::<BE>(&mut vals)?;
            let regular = Bounds::from_i16_arr(&vals);

            BlastZones::from_bounds(regular, single)
        };
        // cursor has been moved to 0x7c while reading blastzones 
        // read 3 u32 values + 1 i16 value [0x7c..0x8a]
        let background_music     = BGM::from_bits(csr.read_u32::<BE>()?)?;
        let other_movescript_ptr = SSBPtr::from_u32(csr.read_u32::<BE>()?);
        let item_bytes_ptr       = SSBPtr::from_u32(csr.read_u32::<BE>()?);
        let falling_whistle      = csr.read_i16::<BE>()?;

        // since the 1P mode camera and blastzones were already read, seek to 0x9a and
        // read the set of 6 i16 values used to set-up the 1P bonus game pause camera [0x9a..0xa6]
        csr.seek(SeekFrom::Start(0x9a))?;
        let bonus_pause_camera = {
            let mut arr = [0i16; 6];
            csr.read_i16_into::<BE>(&mut arr)?;

            BonusPauseCamera::from_i16_arr(&arr)
        };

        // Convert the types of the optional parts of the file, if present
        let item_bytes = item_bytes.map(|vals| {
            let mut arr = [0u8; 0x14];
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
            other_movescript_ptr,
            falling_whistle,
            bonus_pause_camera,
            extra_info,
        })
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        //calculate output size to allocate vec?
        let sizeof_item_bytes  = if self.item_bytes.is_some()
            { 0x14 } else { 0 };
        let sizeof_extra_bytes = if let Some(ref ex) = self.extra_info
            { ex.len() } else { 0 };
        // the stage information common to all files is fixed at 0xa8 bytes
        let unaligned = sizeof_item_bytes + sizeof_extra_bytes + 0xa8;
        let size      = align_usize(unaligned, 16);
        // allocate vec, and fill with bytes
        let mut output = Vec::with_capacity(size);
        let main_info  = self.base_stage_bytes()
            .chain_err(||"generating bytes from StageMain struct")?;

        if let Some(ref bytes) = self.item_bytes {
            output.write_all(bytes)?;
        }
        output.write_all(main_info.as_ref())?;
        if let Some(ref ex_bytes) = self.extra_info {
            output.write_all(ex_bytes)?;
        }
        // fill vec with 0u8 until capacity / alignment?
        let fill = size - output.len();
        for _ in 0..fill { output.push(0u8) };

        Ok(output)
    }

    fn base_stage_bytes(&self) -> Result<[u8; 0xa8]> {
        let mut output = [0u8; 0xa8];
        {
            let mut csr = Cursor::new(output.as_mut());
            // write the four sets of geometry pointers [0..0x40]
            for geo in self.geometries.iter(){
                csr.write_all(geo.as_bytes().as_ref())?;
            }
            // write some individual u32 values [0x40..0x4c]
            csr.write_u32::<BE>(self.collision_ptr.as_u32())?;
            csr.write_u32::<BE>(self.unknown_0x44)?;
            csr.write_u32::<BE>(self.background_ptr.as_u32())?;
            // write the five color rgba32 structs [0x4c..0x60]
            csr.write_all(self.magnifier_color.as_bytes().as_ref())?;
            for color in self.player_logo_color.iter() {
                csr.write_all(color.as_bytes().as_ref())?;
            }
            // write the 3 camera float values [0x60..0x6c]
            csr.write_f32::<BE>(self.lighting1)?;
            csr.write_f32::<BE>(self.lighting2)?;
            csr.write_f32::<BE>(self.camera_tilt)?;
            // save the buffers from the camera and blastzones structs, since these aren't linear
            let (cam_vs, cam_1p)     = self.camera_bounds.as_bytes();
            let (bz_norm, bz_1p_cpu) = self.blastzones.as_bytes();
            // write VS mode camera and normal blastzones [0x6c..0x7c]
            csr.write_all(&cam_vs)?;
            csr.write_all(&bz_norm)?;
            // write music and the "pad?" u32 [0x7c..0x84]
            csr.write_u32::<BE>(self.background_music as u32)?;
            csr.write_u32::<BE>(self.other_movescript_ptr.as_u32())?;
            // write item_bytes pointer [0x84..0x88]
            csr.write_u32::<BE>(self.item_bytes_ptr.as_u32())?;
            // write the falling whistle threshold i16; this breaks word alignment [0x88..0x8a]
            csr.write_i16::<BE>(self.falling_whistle)?;
            // write 1P mode camera and 1P CPU blastzones [0x8a..0x9a]
            csr.write_all(&cam_1p)?;
            csr.write_all(&bz_1p_cpu)?;
            // write the pause camera settings for the bonus 1P games (BtT, BtP, Race) [0x9a..0xa6]
            csr.write_all(self.bonus_pause_camera.as_bytes().as_ref())?;
            // final two bytes are 0u8? [0xa6..0xa8]
        } // end cursor lifetime + end ref mut to array

        Ok(output)
    }
}
fn align_usize(val: usize, align: usize) -> usize {
    let off = val % align;
    if off != 0 { val + (align - off) } else { val }
}

/// Hold the settings for the "zoomed-out" camera for the 1P bonus games (BtT, BtP, Race to the Finish)
/// Technically, the PanX and PanY are two separate values, that can sort of "rotate" the camera.
/// If they are the same, set Pan{Axis}2 to None.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
struct BonusPauseCamera {
    x_pan1: i16,
    x_pan2: Option<i16>,
    y_pan1: i16,
    y_pan2: Option<i16>,
    /// I don't really know what this value is or does...
    bg_zoom: i16,
    zoom: i16,
}

impl BonusPauseCamera {
    /// from processed i16s in same order as file binary
    fn from_i16_arr(input: &[i16; 6]) -> Self {
        let x_pan1 = input[0];
        let x_pan2 = input[3];
        let x_pan2 = if x_pan1 == x_pan2 { None } else { Some(x_pan2) };

        let y_pan1 = input[1];
        let y_pan2 = input[4];
        let y_pan2 = if y_pan1 == y_pan2 { None } else { Some(y_pan2) };

        BonusPauseCamera {
            x_pan1,
            y_pan1,
            bg_zoom: input[2],
            x_pan2,
            y_pan2,
            zoom:    input[5],
        }
    }
    fn as_bytes(&self) -> [u8; 12] {
        let mut o = [0u8; 12];
        let x_pan2 = if let Some(ref pan) = self.x_pan2 {
            *pan } else {self.x_pan1};
        let y_pan2 = if let Some(ref pan) = self.y_pan2 {
            *pan } else {self.y_pan1};

        BE::write_i16(&mut o[0..2],   self.x_pan1);
        BE::write_i16(&mut o[2..4],   self.y_pan1);
        BE::write_i16(&mut o[4..6],   self.bg_zoom);
        BE::write_i16(&mut o[6..8],   x_pan2);
        BE::write_i16(&mut o[8..10],  y_pan2);
        BE::write_i16(&mut o[10..12], self.zoom);

        o
    }

}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Default)]
struct StageGeo {
    geometry_ptr: SSBPtr,
    move_script_ptr: SSBPtr,
    colored_ptr: SSBPtr,
    colored_script_ptr: SSBPtr,
}

impl StageGeo {
    fn from_u32s(input: &[u32; 4]) -> Self {
        StageGeo {
            geometry_ptr:       SSBPtr::from_u32(input[0]),
            move_script_ptr:    SSBPtr::from_u32(input[1]),
            colored_ptr:        SSBPtr::from_u32(input[2]),
            colored_script_ptr: SSBPtr::from_u32(input[3]),
        }
    }
    fn as_bytes(&self) -> [u8; 16] {
        let mut o = [0u8; 16];
        BE::write_u32(&mut o[0..4], self.geometry_ptr.as_u32());
        BE::write_u32(&mut o[4..8], self.move_script_ptr.as_u32());
        BE::write_u32(&mut o[8..12], self.colored_ptr.as_u32());
        BE::write_u32(&mut o[12..16], self.colored_script_ptr.as_u32());

        o
    }
}

/// A 32bit color wrapper. Will accept/return bytes in RGBA8888
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Default)]
struct Color{
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
impl Color {
    /*
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 4 {
            bail!("Need four bytes to generate a Color; got {:?}", &bytes)
        };
        let r = bytes[0];
        let g = bytes[1];
        let b = bytes[2];
        let a = bytes[3];

        Ok(Color{ r, g, b, a })
    } */
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
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Default)]
struct Bounds {
    top: i16,
    bottom: i16,
    right: i16,
    left: i16,
}

impl Bounds {
    fn from_i16_arr(input: &[i16; 4]) -> Self {
        Bounds {
            top: input[0],
            bottom: input[1],
            right: input[2],
            left: input[3],
        }
    }
    fn as_bytes(&self) -> [u8; 8] {
        let mut o = [0u8; 8];
        BE::write_i16(&mut o[0..2], self.top);
        BE::write_i16(&mut o[2..4], self.bottom);
        BE::write_i16(&mut o[4..6], self.right);
        BE::write_i16(&mut o[6..8], self.left);

        o
    }
}

/// Holds the Coords for both the normal and 1P CPU Blastzones. The normal
/// blastzones are used at all times for human controlled characters. The 1P Mode
/// CPU blastzones are used only for the computer controlled characters in the
/// 1P mode (surprise!).
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct BlastZones {
    regular: Bounds,
    cpu_1p: Bounds,
}

impl BlastZones {
    fn from_bounds(regular: Bounds, cpu_1p: Bounds) -> Self {
        BlastZones{regular, cpu_1p}
    }
    fn as_bytes(&self) -> ([u8; 8], [u8; 8]) {
        (self.regular.as_bytes(), self.cpu_1p.as_bytes())
    }
}

/// Holds the camera frame "bounding box" for the both VS and 1P Game Mode. The set
/// "Bounds" struct defines how far the camera will zoom out (relative to the distance
/// between multiple characters). If a character goes outside of this bound, they will
/// be in a maginfying glass
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct CameraBox {
    versus: Bounds,
    one_player: Bounds,
}

impl CameraBox {
    fn from_bounds(versus: Bounds, one_player: Bounds) -> Self {
        CameraBox{versus, one_player}
    }
    fn as_bytes(&self) -> ([u8; 8], [u8; 8]) {
        (self.versus.as_bytes(), self.one_player.as_bytes())
    }
}

/// This is the list of background music (BGM) tracks in ssb64

enum_bits!{
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    enum BGM: u32 {
        Dreamland           = 0x00,
        PlanetZebes         = 0x01,
        MushroomKingdom     = 0x02,
        MushroomKingdomFast = 0x03,
        SectorZ             = 0x04,
        CongoJungle         = 0x05,
        PeachsCastle        = 0x06,
        SaffronCity         = 0x07,
        YoshisIsland        = 0x08,
        HyruleCastle        = 0x09,
        CharacterSelect     = 0x0a,
        BetaFanfare         = 0x0b,
        MarioWins           = 0x0c,
        SamusWins           = 0x0d,
        DKWins              = 0x0e,
        KirbyWins           = 0x0f,
        FoxWins             = 0x10,
        NessWins            = 0x11,
        YoshiWins           = 0x12,
        CFalconWins         = 0x13,
        PokemonWins         = 0x14,
        LinkWins            = 0x15,
        ResultsScreen       = 0x16,
        PreMasterHand1      = 0x17,
        PreMasterHand2      = 0x18,
        FinalDestination    = 0x19,
        BonusStage          = 0x1a,
        StageClear          = 0x1b,
        BonusStageClear     = 0x1c,
        MasterHandClear     = 0x1d,
        BonusStageFailure   = 0x1e,
        Continue            = 0x1f,
        GameOver            = 0x20,
        Intro               = 0x21,
        HowtoPlay           = 0x22,
        Pre1PBattle         = 0x23,
        Battlefield         = 0x24,
        MetalCavern         = 0x25,
        GameComplete        = 0x26,
        Credits             = 0x27,
        Secret              = 0x28,
        HiddenCharacter     = 0x29,
        TrainingMode        = 0x2a,
        VSRecord            = 0x2b,
        MainMenu            = 0x2c,
        Hammer              = 0x2d,
        Invincibility       = 0x2E,
    }
}
