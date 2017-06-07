use std::io::{BufRead};
use std::collections::BTreeMap;

use nemu_mem::{MemType, SymbolInfo};

#[derive(Debug)]
struct Container {
    symbols: Vec<SymbolInfo>,
    branches: BTreeMap<String, Container>
}

impl Container {
    fn new() -> Self {
        Container {
            symbols: Vec::new(),
            branches: BTreeMap::new()
        }
    }

    fn add_symbol(&mut self, sym: SymbolInfo) {
        self.symbols.push(sym);
    }

    fn get_branch_mut(&mut self, branch: &str) -> Option<&mut Container> {
        self.branches.get_mut(branch)
    }

    fn add_branch(&mut self, branch: &str) {
        self.branches.insert(branch.to_string(), Container::new());
    }

    fn has_branch(&self, branch: &str) -> bool {
        self.branches.contains_key(branch)
    }

    fn print(&self, indent_level: u32) -> String {
        let mut output = String::new();

        // calculate indent string
        let indent = (0..indent_level).fold(String::new(),
            |mut inter, _| {
                inter.push('\t');

                inter
            });

        for (scope, subcontain) in self.branches.iter() {
            let subcon_print = subcontain.print(indent_level + 1);

            // add indent
            output.push_str(&indent);
            // add sub branch name
            output.push_str(scope);
            // new line
            output.push('\n');
            // add output from sub-container
            output.push_str(&subcon_print);
        }

        // then, add all SymbolInfo(s) at this indent level
        let mut syms = self.symbols.iter()
            .map( |sym| {
                let mut base = sym.print();

                base.insert_str(0, &indent);
                base
            })
            .collect::<Vec<_>>();

        syms.sort();

        let mut syms_string = syms.join("\n");

        // If there are symbols, add a newline to the end
        if !syms_string.is_empty() {
            syms_string.push('\n');

            // add string of newline separated symbols (if present)
            output.push_str(&syms_string);
        }

        output
    }
}

/* Want to have two attributes for setting the nesting level:
    1. "Nest" : Option<usize>   - Maximum Possible Nesting Level. Everything after {N} nest(s) is
                                concatinated into one string for the memory address' name
    2. "Scope" : Option<usize>  - Scope Values into Name String. Put {M} scopes (input.split('.'))
                                into the final name string for a memory address
*/
pub fn nester(br: Box<BufRead>, scope: usize, nest: Option<usize>, data_filter: &str) -> String {
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
            scope_and_nest_line(substrs, scope, nest, &data_filter)
        }).collect::<Vec<_>>();

    // Combine the component lines into the Container Struct
    let output_container = to_container(parsed_lines);

    // Return string from Container.print() fn
    output_container.print(0)
}

fn scope_and_nest_line(pair: Vec<String>, scope: usize, nest: Option<usize>, data_filter: &str ) -> Option<(MemType, u32, Vec<String>)> {
    let mut iter = pair.iter();
    let addr = iter.next().unwrap();
    let name = iter.next().unwrap();

    let mem = if name.split('.').any( |substr| substr == data_filter )
            { MemType::RAM } else { MemType::CPU };

    let hex_addr = u32::from_str_radix(&addr, 16);

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
        // fold into a final vector
        .fold(Vec::new(), | mut acc, (i, name) | {
            match nest {
                Some(n) => {
                    if i <= n {
                        acc.push(name.to_string());
                    } else {
                        let l: usize = acc.len();
                        let ref mut nested_str: String = acc[l-1];
                        nested_str.push('.');
                        nested_str.push_str(name);
                    }
                },
                None => {acc.push(name.to_string());}
            };

            acc
        });

    // Return only if there is a valid hex address
    match hex_addr {
        Ok(hex) => Some((mem, hex, nest_name)),
        Err(_)  => None
    }
}

fn to_container( addrs: Vec<(MemType, u32, Vec<String>)> ) -> Container {
    let mut output = Container::new();
    // step through every line in vector and fold into the base container
    for &(mem_type, addr, ref split_name) in addrs.iter() {
        let (name, scopes) = split_name.split_last().unwrap(); // TODO: remove unwrap
        let sym = SymbolInfo::new(addr, &name, mem_type);

        // Now, step through the remaining parts of the split name string to
        // find the nested container that should hold the symbol info
        let mut home: &mut Container = scopes.iter()
            .fold( &mut output, | nest_ref, substr | {

                if nest_ref.has_branch(substr) {
                    nest_ref.get_branch_mut(substr).unwrap()
                } else {
                    nest_ref.add_branch(substr);
                    nest_ref.get_branch_mut(substr).unwrap()
                }
        });

        // Add the SymbolInfo to the found Contianer
        home.add_symbol(sym);
    }

    // return filled container
    output
}



//---Test Input and Output-------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader};

//-------------------------------------------------------------------------------------------------
    const INPUT: &'static str = "80400000 boot.data.hitboxFlags
80400004 boot.render
80400004 boot.render.model
80171f4c boot.draw.Bobomb.prologue";
    const ZERO_SCOPE_NONE_NEST: &'static str = "boot
\tdata
\t\tMEM 0x80400000: hitboxFlags
\tdraw
\t\tBobomb
\t\t\tCPU 0x80171F4C: prologue
\trender
\t\tCPU 0x80400004: model
\tCPU 0x80400004: render";

    const ONE_SCOPE_NONE_NEST: &'static str = "boot
\tdraw
\t\tCPU 0x80171F4C: Bobomb.prologue
\tCPU 0x80400004: render.model
\tMEM 0x80400000: data.hitboxFlags
CPU 0x80400004: boot.render";
    const TWO_SCOPE_NONE_NEST: &'static str = "boot
\tCPU 0x80171F4C: draw.Bobomb.prologue
CPU 0x80400004: boot.render
CPU 0x80400004: boot.render.model
MEM 0x80400000: boot.data.hitboxFlags";
    const THREE_SCOPE_NONE_NEST: &'static str = "CPU 0x80171F4C: boot.draw.Bobomb.prologue
CPU 0x80400004: boot.render
CPU 0x80400004: boot.render.model
MEM 0x80400000: boot.data.hitboxFlags";

    const ZERO_SCOPE_ZERO_NEST: &'static str = "CPU 0x80171F4C: boot.draw.Bobomb.prologue
CPU 0x80400004: boot.render
CPU 0x80400004: boot.render.model
MEM 0x80400000: boot.data.hitboxFlags";
    const ZERO_SCOPE_ONE_NEST: &'static str = "boot
\tCPU 0x80171F4C: draw.Bobomb.prologue
\tCPU 0x80400004: render
\tCPU 0x80400004: render.model
\tMEM 0x80400000: data.hitboxFlags";
    const ZERO_SCOPE_TWO_NEST: &'static str = "boot
\tdata
\t\tMEM 0x80400000: hitboxFlags
\tdraw
\t\tCPU 0x80171F4C: Bobomb.prologue
\trender
\t\tCPU 0x80400004: model
\tCPU 0x80400004: render";
//-------------------------------------------------------------------------------------------------

    fn str_to_buf<'a>(input: &'static str) -> Box<BufRead> {
        Box::new(BufReader::new(input.as_bytes()))
    }

    #[test]
    fn substrs_tests(){
        let strs = vec!["80400000".to_string(), "boot.data.1.2.3.4.hitboxFlag".to_string()];
        let expected = (MemType::RAM, 0x80400000, vec!["boot", "data", "1", "2.3.4.hitboxFlag"]);

        let output = scope_and_nest_line(strs, 1, Some(3), "data").unwrap();

        assert_eq!(output.0, expected.0);
        assert_eq!(output.1, expected.1);
        assert_eq!(output.2, expected.2);
    }

    #[test]
    fn single_nest_input(){
        assert_eq!(nester(str_to_buf("80400000 boot.data.hitboxFlags"), 0, Some(0), "data").trim(),
                    "MEM 0x80400000: boot.data.hitboxFlags");

        assert_eq!(nester(str_to_buf("80400000 boot.data.hitboxFlags"), 0, Some(1), "data").trim(),
                    "boot
\tMEM 0x80400000: data.hitboxFlags");
        assert_eq!(nester(str_to_buf("80400000 boot.data.hitboxFlags"), 0, Some(2), "data").trim(),
                    "boot
\tdata
\t\tMEM 0x80400000: hitboxFlags");
        assert_eq!(nester(str_to_buf("80400000 boot.data.hitboxFlags"), 0, None, "data").trim(),
                    "boot
\tdata
\t\tMEM 0x80400000: hitboxFlags");
    }

    #[test]
    fn data_filter_strings() {
        assert_eq!(nester(str_to_buf("80400000 boot.data.hitboxFlags"), 0, Some(0), "data").trim(),
                    "MEM 0x80400000: boot.data.hitboxFlags");
        assert_eq!(nester(str_to_buf("80400000 boot.data.hitboxFlags"), 0, Some(0), "").trim(),
                    "CPU 0x80400000: boot.data.hitboxFlags");
        assert_eq!(nester(str_to_buf("80400000 boot.data.hitboxFlags"), 0, Some(0), "boot").trim(),
                    "MEM 0x80400000: boot.data.hitboxFlags");
        assert_eq!(nester(str_to_buf("80400000 boot.data.hitboxFlags"), 0, Some(0), "hitbox").trim(),
                    "CPU 0x80400000: boot.data.hitboxFlags");
        assert_eq!(nester(str_to_buf("80400000 boot.data.hitboxFlags"), 0, Some(0), "hitboxFlags").trim(),
                    "MEM 0x80400000: boot.data.hitboxFlags");
        assert_eq!(nester(str_to_buf("80123456 1.2.3.4.symbol"), 0, Some(0), "1").trim(),
                    "MEM 0x80123456: 1.2.3.4.symbol");
    }

    #[test]
    fn full_nesting_scopes() {
        assert_eq!(nester(str_to_buf(INPUT), 0, None, "data").trim(), ZERO_SCOPE_NONE_NEST);
        assert_eq!(nester(str_to_buf(INPUT), 1, None, "data").trim(), ONE_SCOPE_NONE_NEST);
        assert_eq!(nester(str_to_buf(INPUT), 2, None, "data").trim(), TWO_SCOPE_NONE_NEST);
        assert_eq!(nester(str_to_buf(INPUT), 3, None, "data").trim(), THREE_SCOPE_NONE_NEST);
        assert_eq!(nester(str_to_buf(INPUT), 10, None, "data").trim(), THREE_SCOPE_NONE_NEST);
    }

    #[test]
    fn full_nesting_nests() {
        assert_eq!(nester(str_to_buf(INPUT), 0, None, "data").trim(), ZERO_SCOPE_NONE_NEST);
        assert_eq!(nester(str_to_buf(INPUT), 0, Some(0), "data").trim(), ZERO_SCOPE_ZERO_NEST);
        assert_eq!(nester(str_to_buf(INPUT), 0, Some(1), "data").trim(), ZERO_SCOPE_ONE_NEST);
        assert_eq!(nester(str_to_buf(INPUT), 0, Some(2), "data").trim(), ZERO_SCOPE_TWO_NEST);
        assert_eq!(nester(str_to_buf(INPUT), 0, Some(3), "data").trim(), ZERO_SCOPE_NONE_NEST);
        assert_eq!(nester(str_to_buf(INPUT), 0, Some(10), "data").trim(), ZERO_SCOPE_NONE_NEST);
    }

}
