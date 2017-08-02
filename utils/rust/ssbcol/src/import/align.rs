use std::io::{Seek, SeekFrom, Cursor};
use errors::*;

/// Move the position in the Cursor forward until its aligned with align
/// Return the new position of the cursor
pub fn align_cursor<T>(csr: &mut Cursor<T>, align: u64) -> u64 {
    let pos = csr.position();
    let aligned = pos + (pos % align);
    csr.set_position(aligned);

    aligned
}

/// Move the position in the type implementing Seek forward until its aligned with align
/// Return the new position of the type
pub fn align_seek<T: Seek>(input: &mut T, align: u64) -> Result<u64> {
    let pos = input.seek(SeekFrom::Current(0))
        .chain_err(||"finding position of cursor for T:Seek (fn align_seek)")?;
    let aligned = pos + (pos % align);
    input.seek(SeekFrom::Start(aligned))
        .chain_err(||format!("aligning type T:Seek to {:#010X}", aligned))?;

    Ok(aligned)
}
