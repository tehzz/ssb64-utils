use byteorder::{BE, ByteOrder};

/// Interal SSB64 "resource files" have dynamic offsets chains that are used to create
/// pointers when moved from ROM to RAM. The building block of the chain is a packed (u16, u16)
/// value, where the first u16 is the word offset to the next link, and the second u16 is the
/// word offset from the beginning of file for the pointer. The chain is over when the next 
/// value is equal to 0xFFFF. 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    pub fn as_u32(&self) -> u32 {
        let &ResPtr{ next, offset } = self;
        let next = if let Some(ref val) = next { *val >> 2 } else { 0xFFFF };
        
        (next << 16) | (offset >> 2)
    }
}

/*
// convert to reader/Result type...
// maybe use a HashMap instead...? Or a sorted btree?
pub fn read_res_chain(start: usize, buffer: &[u8]) -> Vec<(usize, ResPtr)> {
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
}*/

// make possible enum to encode null value?
pub fn check_res_ptr(value: u32) -> Option<ResPtr> {
    let possible = ResPtr::from_u32(value);
    let offset = possible.offset;

    if let Some(ref next) = possible.next {
        // probably a null value (0x00000000) where a pointer could be
        if *next == offset { return None }
        // an offset of '0' for 'next' probably won't happen with Nintendo, but it is a valid case...
        if *next == 0 { return None }
    } 

    Some(possible)
}

/// Encode either a "fixed," standard u32 pointer or a ResPtr
enum SsbPtr {
    fixed(u32),
    resource(ResPtr),
}

impl SsbPtr {
    fn from_u32(input: u32) -> Self {
        match check_res_ptr(input) {
            Some(ptr) => SsbPtr::resource(ptr),
            None      => SsbPtr::fixed(input),
        }
    }
    fn to_u32(self) -> u32 {
        match self {
            SsbPtr::fixed(value)  => value,
            SsbPtr::resource(ptr) => ptr.as_u32(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: u32         = 0x001A00FF;
    const VALID_END: u32     = 0xFFFF0001;
    const INVALID_NULL: u32  = 0x00000000; 
    const INVALID_EMPTY: u32 = 0x00004530;

    const VALID_PARSED: ResPtr = ResPtr {
        next: Some((VALID & 0xFFFF0000) >> 14),
        offset: (VALID & 0xFFFF) << 2,
    };
    const VALID_END_PARSED: ResPtr = ResPtr {
        next: None,
        offset: (VALID_END & 0xFFFF) << 2,
    };

    const VALID_BUFFER: Vec<u8> = vec![
        0x00,0x01,0x00,0x04,
        0x00,0x02,0x00,0xA4,
        0x00,0x04,0x00,0x04,
        0xAB,0xCD,0xEF,0x01,
        0xFF,0xFF,0x12,0x34,
    ];

    #[test]
    fn parse_valid_res_ptrs() {
        let test_valid = ResPtr::from_u32(VALID);
        let test_valid_end = ResPtr::from_u32(VALID_END);

        assert_eq!(test_valid, VALID_PARSED, "Error directly parsing \"VALID\"");
        assert_eq!(test_valid_end, VALID_END_PARSED, "Error directly parsing \"VALID_END\"");
    }

    #[test]
    fn check_valid_res_ptrs() {
        let check_valid = check_res_ptr(VALID);
        let check_valid_end = check_res_ptr(VALID_END);

        assert_eq!(check_valid, Some(VALID_PARSED), "Error checking for valid res_ptr \"VALID\"");
        assert_eq!(check_valid_end, Some(VALID_END_PARSED), "Error checking for valid res_ptr \"VALID_END\"");
    }

    #[test]
    fn check_invalid_res_ptrs() {
        let check_invalid_null = check_res_ptr(INVALID_NULL);
        let check_invalid_empty = check_res_ptr(INVALID_EMPTY);

        assert_eq!(check_invalid_null, None, "Error checking for invalid res_ptr \"INVALID_NULL\"");
        assert_eq!(check_invalid_empty, None, "Error checking for invalid res_ptr \"INVALID_EMPTY\"");
    }

}