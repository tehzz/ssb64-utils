use std::io::{BufRead};
use std::fmt;
use std::collections::HashMap;

use nemu_mem::{MemType, SymbolInfo};

#[derive(Debug)]
struct Container {
    symbols: Vec<SymbolInfo>,
    branches: HashMap<String, Container>
}

impl Container {
    fn new() -> Self {
        Container {
            symbols: Vec::new(),
            branches: HashMap::new()
        }
    }

    fn add_symbol(&mut self, sym: SymbolInfo) {
        self.symbols.push(sym);
    }

    fn get_branch(&self, branch: &str) -> Option<&Container> {
        self.branches.get(branch)
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

    fn print(&self) -> String {
        String::from("unimplemented! :(")
    }
}

/* Want to have two attributes for setting the nesting level:
    1. "Nest" : Option<usize>   - Maximum Possible Nesting Level. Everything after {N} nest(s) is
                                concatinated into one string for the memory address' name
    2. "Scope" : Option<usize>  - Scope Values into Name String. Put {M} scopes (input.split('.'))
                                into the final name string for a memory address
*/

pub fn nester(br: Box<BufRead>, scope: usize, nest: Option<usize>, data_str: &str) -> String {
    // TODO: fix for first or last value... check in iterator?
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
            scope_and_nest_line(substrs, scope, nest, &data_filter)
        }).collect::<Vec<_>>();

    println!("parsed_lines: \n{:?}\n\n", &parsed_lines);

    // Combine the component lines into the Container Struct
    let output_container = to_container(parsed_lines);

    println!("output_container: \n{:?}\n\n", output_container);

    // Return string from Container.print() fn
    // output_container.print()
    String::from("Not finished!")
}


fn scope_and_nest_line(pair: Vec<String>, scope: usize, nest: Option<usize>, data_filter: &str ) -> Option<(MemType, u32, Vec<String>)> {
    let mut iter = pair.iter();
    let addr = iter.next().unwrap();
    let name = iter.next().unwrap();

    let mem = if name.contains(data_filter)
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
        //.inspect( | val | println!("inspect: {:?}", val))
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

    match hex_addr {
        Ok(hex) => Some((mem, hex, nest_name)),
        Err(_)  => None
    }
}

fn to_container ( addrs: Vec<(MemType, u32, Vec<String>)> ) -> Container {
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

    /*
    addrs.iter()
        .fold( &mut output,
        | mut base_container, &(mem_type, addr, ref split_name) | {
            let (name, scopes) = split_name.split_last().unwrap(); // TODO: remove unwrap
            let sym = SymbolInfo::new(addr, &name, mem_type);

            // Now, step through the remaining parts of the split name string to
            // find the nested container that should hold the symbol info
            let mut home: &mut Container = scopes.iter()
                .fold( &mut base_container, | nest_ref, substr | {

                    if nest_ref.has_branch(substr) {
                        nest_ref.get_branch_mut(substr).unwrap()
                    } else {
                        nest_ref.add_branch(substr);
                        nest_ref.get_branch_mut(substr).unwrap()
                    }
            });

            // Add the SymbolInfo to the found Contianer
            home.addSymbol(sym);

            // Return the base container
            base_container
    });*/

    // return filled container
    output
}


//---Test Input and Output-------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader};

/*
    const INPUT: &'static str = "80400000 boot.data.hitboxFlags
80400004 boot.render
80400004 boot.render.model
80171f4c boot.draw.Bobomb.prologue";
*/

/*
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
    } */


    #[test]
    fn substrs_tests(){
        let strs = vec!["80400000".to_string(), "boot.data.1.2.3.4.hitboxFlag".to_string()];
        let expected = (MemType::RAM, 0x80400000, vec!["boot", "data", "1", "2.3.4.hitboxFlag"]);

        let output = scope_and_nest_line(strs, 1, 3, ".data.").unwrap();
        println!("{:?}", output);

        //assert_eq!(output.0, expected.0);
        assert_eq!(output.1, expected.1);
        assert_eq!(output.2, expected.2)
    }

    /*
    #[test]
    fn to_container_test(){
        let tups = (MemType::RAM, 0x80400000, vec!["boot", "data", "hitboxFlag"]);
        let sym = SymbolInfo::new(tups.1, tups.2[2], tups.0);
        let mut expect = Container::new();

        expect.add_branch(tups.2[0]);
        expect.get_branch_mut(tups.2[0]).unwrap().add_branch(tups.2[1]);

        let mut home = expect.get_branch(tups.2[0]).unwrap().get_branch_mut(tups.2[1]).unwrap();

        home.add_symbol(sym);

        assert_eq!(to_container(vec![tups]), expect);
    }*/
}
