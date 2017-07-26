use byteorder::{BE, WriteBytesExt};
use std::io::Cursor;
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
    pub fn sizeof_struct() -> usize {18}

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

    pub fn to_bytes(&self) -> [u8; 18] {
        let mut output = [0u8; 18];
        {
            let mut csr = Cursor::new(output.as_mut());
            csr.write_u16::<BE>(self.id).unwrap();
            csr.write_u16::<BE>(self.top_start).unwrap();
            csr.write_u16::<BE>(self.top_size).unwrap();
            csr.write_u16::<BE>(self.bottom_start).unwrap();
            csr.write_u16::<BE>(self.bottom_size).unwrap();
            csr.write_u16::<BE>(self.right_start).unwrap();
            csr.write_u16::<BE>(self.right_size).unwrap();
            csr.write_u16::<BE>(self.left_start).unwrap();
            csr.write_u16::<BE>(self.left_size).unwrap();
        }
        output
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
