use std::io::{Seek, SeekFrom, Cursor, Write};
use errors::*;
use byteorder::{WriteBytesExt};

/// Move the position in the Cursor forward until its aligned with align
/// Return the new position of the cursor
pub fn align_cursor<T>(csr: &mut Cursor<T>, align: u64) -> u64 {
    let pos = csr.position();
    let r = pos % align;
    let aligned = if r == 0 { pos } else { pos + (align - r) };
    csr.set_position(aligned);

    aligned
}

/// Move the position in the type implementing Seek forward until its aligned with align
/// Return the new position of the type
pub fn align_seek<T: Seek>(input: &mut T, align: u64) -> Result<u64> {
    let pos = input.seek(SeekFrom::Current(0))
        .chain_err(||"finding position of cursor for T:Seek (fn align_seek)")?;

    let r = pos % align;
    let aligned = if r == 0 { pos } else { pos + (align - r)};
    input.seek(SeekFrom::Start(aligned))
        .chain_err(||format!("aligning type T:Seek to {:#010X}", aligned))?;

    Ok(aligned)
}

/// Fill the `csr` with `fill` until `align`
pub fn fill_cursor<T>(csr: &mut Cursor<T>, align: u64, fill: u8) -> Result<u64>
    where Cursor<T>: Write
{
    while csr.position() % align != 0 {
        csr.write_u8(fill)?
    };

    Ok(csr.position())
}
