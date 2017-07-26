use byteorder::{WriteBytesExt, BE};
use std::io::{Cursor};

/// PlaneInfo defines an offset and length into the plane array (which is an array of offsets into the
// collision points array) to define a collision plane
#[derive(Debug)]
pub struct PlaneInfo {
    pub start: u16,
    pub length: u16
}
impl PlaneInfo {
    pub fn sizeof_struct() -> usize {4}

    pub fn new(s: u16, l: u16) -> Self {
        PlaneInfo{ start: s, length: l}
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        let mut output = [0u8;4];
        {
            let mut csr = Cursor::new(output.as_mut());
            csr.write_u16::<BE>(self.start).unwrap();
            csr.write_u16::<BE>(self.length).unwrap();
        }

        output
    }
}
