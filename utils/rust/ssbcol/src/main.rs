#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
extern crate byteorder;
#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate clap;
use clap::{App, Arg, SubCommand};

use std::fs::File;
use std::path::{Path, PathBuf};

#[macro_use]
mod macros;
mod configs;
use configs::{ExportConfig};
mod export;
use export::export_collision;
mod collision;      // structs to represent collision data

mod errors {
    error_chain!{
        types {}
        foreign_links {
            ParseInt(::std::num::ParseIntError);
            Io(::std::io::Error);
            SerdeJSON(::serde_json::Error);
        }
    }
}
use errors::*;

#[derive(Debug, PartialEq, Eq)]
enum Mode {
    Import,
    Export
}

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let matches = cli().get_matches();
    // check if verbose
    let verbose = matches.is_present("verbose");

    // get the proper mode
    let mode = match matches.subcommand_name() {
        Some("import") => Ok(Mode::Import),
        Some("export") => Ok(Mode::Export),
        _ => Err("Mode not selected. Run with subcommand \"import\" or \"export\"")
    }?;

    // get proper output?
    match mode {
        Mode::Import => println!("Import not implemented yet T-T"),
        Mode::Export => {
            let submatch = matches.subcommand_matches("export").unwrap();
            let path  = Path::new(submatch.value_of("input").unwrap());
            let f     = File::open(path)
                         .chain_err(|| "Unable to open input file for export")?;
            let in_ptr =  submatch.value_of("col-ptr").unwrap();
            let ptr  = parse_str_to_u32(in_ptr)
                .chain_err(|| format!("\"--collision\" flag called with \"{}\". Call with an integer (\"0x\" for hex)", in_ptr))?;

            let config = ExportConfig::new(f, ptr, verbose);

            let parsed_col = export_collision(config)
                .chain_err(|| format!("couldn't parse collision from input file <{:?}>", path))?;
            // get the output file
            let o_path = if let Some(named) = submatch.value_of("output") {
                PathBuf::from(named)
            } else {
                let mut p = path.to_path_buf();
                p.set_extension("json");
                p
            };
            let o = File::create(&o_path)
                .chain_err(|| format!("creating or reading output file <{:?}>", &o_path))?;

            serde_json::to_writer_pretty(o, &parsed_col)
                .chain_err(|| "writing serialized json to output")?;
        }
    }

    Ok(())
}

fn parse_str_to_u32(input: &str) -> ::std::result::Result<u32, ::std::num::ParseIntError> {
    if input.starts_with("0x") || input.starts_with("0X") {
        u32::from_str_radix(&input[2..], 16)
    } else {
        input.parse::<u32>()
    }
}

fn cli<'a,'b>() -> App<'a,'b> {
    // make subcommands slice?

    App::new("SSB64 Collision Data Utility")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Import or export collision data from a stage main geometry resource file")
        .arg_from_usage("--verbose 'Enable verbose mode'")
    .subcommand(SubCommand::with_name("export")
        .about("Export collision information into a JSON file")
        .arg(Arg::with_name("input")
            .help("Input resource file to extract collision data from")
            .required(true)
            .index(1)
        )
        .arg(Arg::with_name("output")
            .help("An optional name for the output JSON file.\n \
            By default, the output file name is \"<input>.json\"")
            .required(false)
            .index(2)
        )
        .arg(Arg::with_name("col-ptr")
            .help("Offset to the collision pointer area of the file.\n \
            This is the same offset from 0x40/0x5C in base stage file.")
            .takes_value(true)
            .short("c")
            .long("collision")
            .required(true)
            .multiple(false)
        )
    )
}
