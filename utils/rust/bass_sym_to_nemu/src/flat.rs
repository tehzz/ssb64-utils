use std::io::{BufRead};
use std::fmt;

#[derive(Debug)]
enum MemType {
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
struct SymbolInfo {
    addr: u32,
    name: String,
    mem_type: MemType
}

impl SymbolInfo {
    fn new( addr: u32, name: &str, mem: MemType ) -> Self {
        SymbolInfo {
            addr     : addr,
            name     : name.to_string(),
            mem_type : mem
        }
    }

    fn print(&self) -> String {
        format!("{MemType} {addr:#010X}: {name}",
            MemType = self.mem_type,
            addr    = self.addr,
            name    = self.name
        )
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


pub fn flatten(br: Box<BufRead>) -> String {
    /* read each line
    / check if each line conforms to "{addr} {name}"
    / -> Toss bad lines?
    / convert line to SymbolInfo
    / convert vector of SymbolInfos into a String
    / return String
    */
    br.lines()
        .filter_map( | line | line.ok() )
        .map( | line |
            line.split_whitespace()
                .map(|str| str.to_string())
                .collect::<Vec<_>>()
        )
        .filter_map( |substrs|
            match substrs.len() {
                2 => Some(substrs),
                _ => None
            }
        )
        .filter_map( |substrs| {
            let mem = if substrs[1].contains(".data.")
                    { MemType::RAM } else { MemType::CPU };

            match u32::from_str_radix(substrs[0].as_str(),16) {
                Ok(addr) => Some( SymbolInfo::new(
                                    addr,
                                    &substrs[1],
                                    mem
                            )),
                Err(_)   => None
            }
        })
        .map( | info | info.print() )
        .collect::<Vec<_>>()
        .join("\n")
}
