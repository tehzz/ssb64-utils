use std::io::{BufRead};
use std::fmt;
use std::collections::HashMap;

use nemu_mem::{MemType, SymbolInfo};

#[derive(Debug)]
struct Container {
    mut symbols: Vec<MemType>,
    mut branches: HashMap<String, Container>
}

impl Container {
    fn new() -> Self {
        Container {
            symbols: Vec::new(),
            branches: HashMap::new()
        }
    }
}

/* Want to have two attributes for setting the nesting level:
    1. "Nest" : Option<usize>   - Maximum Possible Nesting Level. Everything after {N} nest(s) is
                                concatinated into one string for the memory address' name
    2. "Scope" : Option<usize>  - Scope Values into Name String. Put {M} scopes (input.split('.'))
                                into the final name string for a memory address
*/

pub fn nest(br: Box<BufRead>, nest: usize, scope: usize, data_str: &str) -> String {
    // First, get only the lines that are legal bass symbol file lines (addr + ' ' + constant_name)
    let data_filter = String::from(".") + data_str + ".";

    // Parse each line into it's components
    let parsed_lines: Vec<(MemType, u32, Vec<String>)> = br.lines()
        .filter_map( | line | line.ok() )
        .filter_map( | line | {
            let pair = line.split_whitespace()
                .map(|str| str.to_string())
                .collect::<Vec<_>>();

            match pair.len() {
                2 => Some(pair),
                _ => None
            }
        }).filter_map( | substrs | {
            let [addr, name] = substrs;

            let mem = if name.contains(data_filter)
                    { MemType::RAM } else { MemType::CPU };

            let hex_addr: u32 = u32::from_str_radix(&addr, 16);

            // split up the name substr (substr[1]) by '.'
            let nest_name: Vec<String> = name.split('.').rev().enumerate()
                // fold into a new iter() to limit scope
                .fold(Vec::new(), | mut acc: Vec<String>, (i, name) | {
                    if i <= scope && i != 0 {
                        let rescope: String = name.to_string() + "." + &acc[0];

                        acc[0] = rescope;
                    } else {
                        acc.push(name.to_string());
                    }

                    acc
                })
                // iterator over scope vector to limit for nesting
                .iter().rev().enumerate()
                //.inspect( | val | println!("inspect: {:?}", val))
                // fold into a final vector
                .fold(Vec::new(), | mut acc, (i, name) | {
                    if i <= nest && i != 0 {
                        let ref mut nested_str: String = acc[0];
                        nested_str.push('.');
                        nested_str.push_str(name)
                    } else {
                        acc.push(name.to_string());
                    }

                    acc
                });

            match hex_addr {
                Ok(hex) => Some(mem, hex, nest_name),
                Err(_)  => None
            }
        }).collect::<Vec<_>>()

    // Combine the component lines into the Container Struct

    // Return string from Container.print() fr
}

/*fn to_Container( substrs: Vec<String> ) -> Container {

}*/


//---Test Input and Output-------------------------------------------------------------------------
/*
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader};

    const INPUT: &'static str = "80400000 boot.data.hitboxFlags
80400004 boot.render
80400004 boot.render.model
80171f4c boot.draw.Bobomb.prologue"

    fn str_to_buf<'a>(input: &'static str) -> Box<BufRead> {
        Box::new(BufReader::new(input.as_bytes()))
    }

    #[test]
    fn single_nest_input(){
        assert_eq!(nest(str_to_buf("80400000 boot.data.hitboxFlags"), 0, 0, "data"),
                    "MEM 0x80400000: boot.data.hitboxFlags");

        assert_eq!(nest(str_to_buf("80400000 boot.data.hitboxFlags"), 1, 0, "data"),
                    "boot
\tMEM 0x80400000: data.hitboxFlags");
        assert_eq!(nest(str_to_buf("80400000 boot.data.hitboxFlags"), 2, 0, "data"),
                    "boot
\tdata
\t\tMEM 0x80400000: hitboxFlags");
    }
}
