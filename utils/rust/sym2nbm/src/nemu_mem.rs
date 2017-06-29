use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemType {
    CPU,
    RAM
}

impl fmt::Display for MemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MemType::CPU => write!(f, "CPU"),
            MemType::RAM => write!(f, "MEM")
        }
    }
}


#[derive(Debug)]
pub struct SymbolInfo {
    addr: u32,
    name: String,
    mem_type: MemType
}

impl SymbolInfo {
    pub fn new( addr: u32, name: &str, mem: MemType ) -> Self {
        SymbolInfo {
            addr     : addr,
            name     : name.to_string(),
            mem_type : mem
        }
    }

    pub fn print(&self) -> String {
        format!("{}", self)
    }

    pub fn get_values(&self) -> (u32, &str, MemType) {
        (self.addr,
         &self.name,
         self.mem_type)
    }
}

impl fmt::Display for SymbolInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{MemType} {addr:#010X}: {name}",
            MemType = self.mem_type,
            addr    = self.addr,
            name    = self.name
        )
    }
}
