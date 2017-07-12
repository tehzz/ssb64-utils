use std::fs::File;

pub struct ExportConfig {
    pub input: File,
    pub col_ptr: u32
}

impl ExportConfig {
    pub fn new(i: File, ptr: u32) -> Self {
        ExportConfig {
            input: i,
            col_ptr: ptr
        }
    }
}
