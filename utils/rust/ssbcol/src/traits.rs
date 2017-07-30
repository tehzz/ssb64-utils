use std::convert::AsRef;

pub trait N64Bytes {
    type Output: AsRef<[u8]>;

    fn size() -> usize;
    fn to_bytes(&self) -> Self::Output;
    //fn to_byte_iter<'a>(&self) -> Iter<'a, u8>;
}
