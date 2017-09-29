use byteorder::{BE, ByteOrder};

/// Interal SSB64 "resource files" have dynamic offsets chains that are used to create
/// pointers when moved from ROM to RAM. The building block of the chain is a packed (u16, u16)
/// value, where the first u16 is the word offset to the next link, and the second u16 is the
/// word offset from the beginning of file for the pointer. The chain is over when the next 
/// value is equal to 0xFFFF. 
pub struct ResPtr {
    next:   Option<u32>,
    offset: u32,
}

impl ResPtr {
    pub fn from_u32(val: u32) -> Self {
        const END_CHAIN: u32 = 0xFFFF;

        let next   = (val & 0xFFFF0000) >> 16;
        let offset = (val & 0xFFFF) << 2;
        // check next value to see if it has the "end chain" value, and parse accordingly
        let next = if next == END_CHAIN { None } else { Some(next << 2) };

        ResPtr{next, offset}
    }

    pub fn to_u32(&self) -> u32 {
        let &ResPtr{ next, offset } = self;
        let next = if let Some(ref val) = next { *val >> 2 } else { 0xFFFF };
        
        (next << 16) | (offset >> 2)
    }
}

// convert to reader/Result type...
// maybe use a HashMap instead...? Or a sorted btree?
pub fn read_ResPtr_chain(start: usize, buffer: &[u8]) -> Vec<(usize, ResPtr)> {
    let mut output = Vec::new();
    let mut offset = start;
    loop {
        let raw_value: u32 = BE::read_u32(&buffer[offset..offset+4]);
        let res_ptr = ResPtr::from_u32(raw_value);
        let next    = res_ptr.next;

        output.push((offset, res_ptr));

        if let Some(ref next_offset) = next {
            offset = *next_offset as usize;
        } else {
            break;
        }
    }

    output
}