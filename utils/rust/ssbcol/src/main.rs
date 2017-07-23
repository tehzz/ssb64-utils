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
use configs::{ExportConfig, ImportConfig};
mod export;
use export::export_collision;
mod import;
use import::import_collision;
mod collision;      // structs to represent collision data
use collision::FormattedCollision;

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

#[derive(Debug, PartialEq, Eq)]
enum Mode {
    Import,
    Export
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
        Mode::Import => {
            let submatch = matches.subcommand_matches("import").unwrap();
            let i_path   = Path::new(submatch.value_of("input").unwrap());
            let i_f      = File::open(i_path)
                .chain_err(|| format!("Unable to open JSON file <{:?}>for import", i_path))?;
            let collision: FormattedCollision = serde_json::from_reader(&i_f)
                .chain_err(|| format!("Unable to parse JSON file <{:?}>", i_path))?;

            let o_name = submatch.value_of("output").unwrap();
            let o_path = if matches.is_present("copy") {
                // change to return tuple of path and file?
                get_unique_file_name(&o_name)
            } else {
                PathBuf::from(&o_name)
            };

            let o_f = File::create(&o_path)
                .chain_err(||format!("reading output file <{:?}>", &o_path))?;

            let resource_pointer = if let Some(val) = submatch.value_of("res-ptr"){
                let ok = parse_str_to_u32(val)
                    .chain_err(||format!("parsing resource node pointer at <{:?}>", val))?;

                Some(ok)
            } else { None };

            let req_list_start = if let Some(val) = submatch.value_of("req-start"){
                let ok = parse_str_to_u32(val)
                    .chain_err(||format!("parsing address of the start of the required file list at <{:?}>", val))?;

                Some(ok)
            } else { None };


            let config = ImportConfig::new(collision, o_f, verbose, resource_pointer, req_list_start);

            let output = import_collision(config).chain_err(||"importing collision")?;
            println!("Import Output:\n {:?}", output);
            println!("Import not implemented yet T-T");
        },
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

fn get_unique_file_name(name: &str) -> PathBuf {
    // for now, just return the input as a PathBuf
    PathBuf::from(name)
}

fn cli<'a,'b>() -> App<'a,'b> {
    let import = SubCommand::with_name("import")
        .about("Import collision information from a JSON file into a stage resource file")
        .arg(Arg::with_name("input")
            .help("Input JSON file containing collision information")
            .required(true)
            .index(1)
        )
        .arg(Arg::with_name("output")
            .help("Output file to write collision binary to")
            .required(true)
            .index(2)
        )
        .arg(Arg::with_name("res-ptr")
            .help("Optional pointer to the start of the node resource pointer chain.\n\
            If provided, the output collision binary will have proper pointers.")
            .takes_value(true)
            .short("r")
            .long("resource")
        )
        .arg(Arg::with_name("req-start")
            .help("Beginning of required file list. Assumed to go from value to end of file")
            .takes_value(true)
            .short("q")
            .long("reqstart")
        )
        .arg_from_usage("--copy 'Make a copy of the output file'");

    let export = SubCommand::with_name("export")
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
        );

    App::new("SSB64 Collision Data Utility")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Import or export collision data from a stage main geometry resource file")
        .arg_from_usage("--verbose 'Enable verbose mode'")
    .subcommand(export)
    .subcommand(import)
}
