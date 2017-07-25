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
