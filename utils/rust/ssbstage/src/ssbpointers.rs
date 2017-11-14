//use byteorder::{BE, ByteOrder};

/// Interal SSB64 "resource files" have dynamic offsets chains that are used to create
/// pointers when moved from ROM to RAM. The building block of the chain is a packed (u16, u16)
/// value, where the first u16 is the word offset to the next link, and the second u16 is the
/// word offset from the beginning of file for the pointer. The chain is over when the next 
/// value is equal to 0xFFFF. 
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

    /// This function returns the 32 bit pack version of the resource pointer
    pub fn as_packed_u32(&self) -> u32 {
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


/// Encode either a "fixed," standard u32 pointer, a ResPtr, or the null pointer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SSBPtr {
    Fixed(u32),
    Resource(ResPtr),
    Null,
}

impl SSBPtr {
    pub fn from_u32(input: u32) -> Self {
        // First, check for null pointer
        if input == 0x00000000 { return SSBPtr::Null }
        
        // Parse the u32 bit value as a packed pointer to check the offset and next values
        let possible = ResPtr::from_u32(input);

        // an offset of '0' for 'next' probably won't happen with Nintendo, but it is a valid case...
        if let Some(ref next) = possible.next {
            if *next == 0 { return SSBPtr::Fixed(input) }

            // Check if the input value has an obvious pointer with the top bit set (0x8-------),
            // as the u32 didn't contain the "end" magic number (0xFFFF----)
            if input & 0x80000000 != 0 { return SSBPtr::Fixed(input) }
        }
        // Nothing to really check for if the "end" magic number was found... 
        
        SSBPtr::Resource(possible)
    }

    pub fn as_u32(&self) -> u32 {
        match self {
            &SSBPtr::Fixed(value)      => value,
            &SSBPtr::Resource(ref ptr) => ptr.as_packed_u32(),
            &SSBPtr::Null              => 0x00000000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: u32          = 0x001A00FF;
    const VALID_END: u32      = 0xFFFF0001;
    const VALID_NULL: u32     = 0x00000000; 
    const INVALID_OFFSET: u32 = 0x00004530;
    const REAL_PTR: u32       = 0x800A4D08;

    const VALID_PARSED: ResPtr = ResPtr {
        next: Some((VALID & 0xFFFF0000) >> 14),
        offset: (VALID & 0xFFFF) << 2,
    };
    const VALID_END_PARSED: ResPtr = ResPtr {
        next: None,
        offset: (VALID_END & 0xFFFF) << 2,
    };

    const VALID_BUFFER: [u8; 20] = [
        0x00,0x01,0x00,0x04,
        0x00,0x02,0x00,0xA4,
        0x00,0x04,0x00,0x04,
        0xAB,0xCD,0xEF,0x01,
        0xFF,0xFF,0x12,0x34,
    ];

    #[test]
    fn parse_valid_res_ptrs() {
        let test_valid     = ResPtr::from_u32(VALID);
        let test_valid_end = ResPtr::from_u32(VALID_END);

        assert_eq!(test_valid, 
            VALID_PARSED, 
            "Error directly parsing \"VALID\"");
        assert_eq!(test_valid_end, 
            VALID_END_PARSED, 
            "Error directly parsing \"VALID_END\"");
        // check returning the parsed pointer to their packed u32 representation
        let test_valid_u32     = test_valid.as_packed_u32();
        let test_valid_end_u32 = test_valid_end.as_packed_u32();

        assert_eq!(test_valid_u32, VALID,
            "Error returning parsed \"VALID\" to u32");
        assert_eq!(test_valid_end_u32, VALID_END,
            "Error returning parsed \"VALID_END\" to u32");
    }

    #[test] 
    fn check_ssbptr_as_u32s() {
        let null_ssbptr     = SSBPtr::Null;
        let fixed_ssbptr    = SSBPtr::Fixed(REAL_PTR);
        let resource_ssbptr = SSBPtr::Resource(VALID_END_PARSED);

        assert_eq!(null_ssbptr.as_u32(), 0x0, 
            "Error casting null ssbptr as u32");
        assert_eq!(fixed_ssbptr.as_u32(), REAL_PTR, 
            "Error casting fixed pointer back to u32");
        assert_eq!(resource_ssbptr.as_u32(), VALID_END_PARSED.as_packed_u32(), 
            "Error casting resource ssbptr to u32");
    }
    #[test]
    fn check_ssbptr_parsing_valid() {
        let parsed_valid = SSBPtr::from_u32(VALID);
        let parsed_end   = SSBPtr::from_u32(VALID_END);

        assert_eq!(parsed_valid, SSBPtr::Resource(VALID_PARSED), 
            "Error when creating SSBPtr enum for valid u32 \"VALID\"");

        assert_eq!(parsed_end, SSBPtr::Resource(VALID_END_PARSED), 
            "Error when creating SSBPtr enum for valid u32 \"VALID_END\"");
    }

    #[test]
    fn check_ssbptr_parsing_null() {
        let parsed_null = SSBPtr::from_u32(VALID_NULL);

        assert_eq!(parsed_null, SSBPtr::Null, 
            "Error when creating SSBPtr enum for null u32 \"VALID_NULL\"");
    }

    #[test]
    fn check_ssbptr_fixed_parsing() {
        let parsed_ptr       = SSBPtr::from_u32(REAL_PTR);
        let parsed_no_offset = SSBPtr::from_u32(INVALID_OFFSET);

        assert_eq!(parsed_ptr, SSBPtr::Fixed(REAL_PTR), 
            "Error when parsing u32 to create fixed ssbptr");
        assert_eq!(parsed_no_offset, SSBPtr::Fixed(INVALID_OFFSET), 
            "Error when parsing u32 with empty offset only to create fixed ssbptr");
    }
}