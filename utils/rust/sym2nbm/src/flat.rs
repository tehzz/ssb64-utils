use std::io::{BufRead};

use nemu_mem::{MemType, SymbolInfo};

pub fn flatten(br: Box<BufRead>, data_filter: &str) -> String {
    /* read each line
    / check if each line conforms to "{addr} {name}"
    / -> Toss bad lines?
    / convert line to SymbolInfo
    / convert vector of SymbolInfos into a String
    / return String
    */
    br.lines()
        .filter_map( | line | line.ok() )
        .filter_map( | line | {
            let pair = line.split_whitespace()
                .map(|str| str.to_string())
                .collect::<Vec<_>>();

            match pair.len() {
                2 => Some(pair),
                _ => None
            }
        })
        .filter_map( |substrs| {
            let mem = if substrs[1].split('.').any(|substr| substr == data_filter)
                    { MemType::RAM } else { MemType::CPU };

            match u32::from_str_radix(&substrs[0],16) {
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


//---Test Input and Output-------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    use std::io::{BufReader};

    const VALIDINPUT: &'static str = "80400000 boot.data.hitboxFlags
800f2c00 boot.hook.beginning
800f3650 boot.hook.end
80400004 boot.render
80400004 boot.render.model
80171f4c boot.drawBobomb.prologue
80171f78 boot.drawBobomb.get_hitbox_flag_state
80171f80 boot.drawBobomb.model_check
80171f8c boot.drawBobomb.draw_model
80171f94 boot.drawBobomb.hitbox_checks
80171fc4 boot.drawBobomb.draw_hitbox
80171fcc boot.drawBobomb.epilogue
80032604 loader.bootDMA";

    const VALIDOUTPUT: &'static str = "MEM 0x80400000: boot.data.hitboxFlags
CPU 0x800F2C00: boot.hook.beginning
CPU 0x800F3650: boot.hook.end
CPU 0x80400004: boot.render
CPU 0x80400004: boot.render.model
CPU 0x80171F4C: boot.drawBobomb.prologue
CPU 0x80171F78: boot.drawBobomb.get_hitbox_flag_state
CPU 0x80171F80: boot.drawBobomb.model_check
CPU 0x80171F8C: boot.drawBobomb.draw_model
CPU 0x80171F94: boot.drawBobomb.hitbox_checks
CPU 0x80171FC4: boot.drawBobomb.draw_hitbox
CPU 0x80171FCC: boot.drawBobomb.epilogue
CPU 0x80032604: loader.bootDMA";

    const DATATEST: &'static str = "804000A0 test.data.globalVar";
    const CORRECT_DATATEST: &'static str = "MEM 0x804000A0: test.data.globalVar";

    const NOTDATATEST: &'static str = "804000A0 test.dataGlobalVar";
    const CORRECT_NOTDATATEST: &'static str = "CPU 0x804000A0: test.dataGlobalVar";

    fn str_to_buf<'a>(input: &'static str) -> Box<BufRead> {
        Box::new(BufReader::new(input.as_bytes()))
    }

    #[test]
    fn correct_flatten() {
        assert_eq!(flatten(str_to_buf(VALIDINPUT), "data"), VALIDOUTPUT);

        assert_eq!(flatten(str_to_buf(DATATEST), "data"), CORRECT_DATATEST);

        assert_eq!(flatten(str_to_buf(NOTDATATEST), "data"), CORRECT_NOTDATATEST);
    }
}
