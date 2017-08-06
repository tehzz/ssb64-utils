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

#[macro_use]
mod macros;
mod traits;
mod configs;
mod export;
mod import;
mod collision;

use std::fs::{self, File, OpenOptions};
use std::path::{Path, PathBuf};
use std::ffi::{OsStr, OsString};
use clap::{App, Arg, SubCommand};

use configs::{ExportConfig, ImportConfig};
use export::export_collision;
use import::import_collision;
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
            let o_path = if submatch.is_present("copy") {
                copy_file(&o_name, submatch.value_of("copy"))
                    .chain_err(||format!("producing copy of output file <{:?}>",&o_name))?
            } else {
                PathBuf::from(&o_name)
            };

            println!("Import: Output file: <{:?}>", &o_path);

            let o_f = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&o_path)
                .chain_err(||format!("opening output file <{:?}>", &o_path))?;

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

            let collision_ptrs_ptr = if let Some(ptr) = submatch.value_of("ptr-replace") {
                let ok = parse_str_to_u32(ptr)
                    .chain_err(||format!("parsing pointer <{}> to collision pointers struct for in-place import", ptr))?;
                Some(ok)
            } else { None };

            let config = ImportConfig::new(collision, o_f, verbose,
                resource_pointer, req_list_start, collision_ptrs_ptr);

            let output = import_collision(config).chain_err(||"importing collision")?;
            println!("Offset to Collision Pointers:\n{:#x}", output);
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

fn copy_file(original: &str, copy: Option<&str>) -> Result<PathBuf> {
    // get path to original <output> file for copying
    let original = Path::new(&original);
    let copy = if let Some(user_path) = copy {
        // use the user provided a path to copy the original to, use that.
        PathBuf::from(user_path)
    } else {
        // else, use "<original>-copy.<original-ext>" as copied output file
        let ext = match original.extension() {
            Some(ext) => ext,
            None => OsStr::new("bin"),
        };

        let mut start = match original.file_stem() {
            Some(stem) => {
                let mut name = stem.to_os_string();
                name.push("-copy");
                name
            },
            None => OsString::from("copied-output"),
        };

        start.push(".");
        start.push(ext);

        let mut copy_path = PathBuf::from(original);
        copy_path.set_file_name(start);
        copy_path
    };

    //mkdir for copy if it doesn't exist
    match copy.parent() {
        Some(path) => {
            fs::create_dir_all(&path)
                .chain_err(||format!("making directories <{}>", path.display()))?
        },
        None => ()
    };

    fs::copy(&original, &copy)
        .chain_err(||format!("making copy of <{:?}> to <{:?}>", &original, &copy))?;

    Ok(copy)
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
            .help("Output file to write collision binary to. This will write-over the original file, \
            unless the \"copy\" flag is set")
            .required(true)
            .index(2)
        )
        .arg(Arg::with_name("res-ptr")
            .help("Optional pointer to the start of the resource file's internal pointer chain.\n\
            If provided, the output collision binary will have proper pointers.")
            .takes_value(true)
            .short("r")
            .long("resource")
        )
        .arg(Arg::with_name("ptr-replace")
            .help("Replace current collision pointer information in <output> starting at this location")
            .takes_value(true)
            .multiple(false)
            .conflicts_with("res-ptr")
            .short("p")
            .long("pointers")
        )
        .arg(Arg::with_name("req-start")
            .help("Beginning of required file list. Assumed to go from value to end of file")
            .takes_value(true)
            .short("q")
            .long("reqstart")
        )
        .arg(Arg::with_name("copy")
            .help("Make a copy of the output file. If no file name is given, default to \"<output>-copy.bin\"")
            .takes_value(true)
            .multiple(false)
            .short("c")
            .long("copy")
        );

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
