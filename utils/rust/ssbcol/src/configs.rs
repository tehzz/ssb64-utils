use std::fs::File;
use std::io::{Read, Write, Seek};
use collision::FormattedCollision;

pub struct ExportConfig {
    pub input: File,
    pub col_ptr: u32,
    pub verbose: bool,
}

impl ExportConfig {
    pub fn new(i: File, ptr: u32, verbose: bool) -> Self {
        ExportConfig {
            input: i,
            col_ptr: ptr,
            verbose
        }
    }
}

pub struct ImportConfig<F> {
    pub input: FormattedCollision,
    pub output: F,
    pub verbose: bool,
    pub res_ptr: Option<u32>,
    pub req_start: Option<u32>,
}

impl<F: Read + Write + Seek> ImportConfig<F> {
    pub fn new(i: FormattedCollision, o: F, v: bool, res: Option<u32>, req: Option<u32>) -> Self {
        ImportConfig {
            input: i,
            output: o,
            verbose: v,
            res_ptr: res,
            req_start: req
        }
    }
}
