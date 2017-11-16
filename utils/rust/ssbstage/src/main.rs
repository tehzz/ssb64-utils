#![recursion_limit = "32"]
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate getopts;
extern crate byteorder;
#[macro_use]
mod macros;
mod ssbpointers;
mod stage;
mod parser;
mod builder;

use getopts::Options;
use std::env;
use std::fs::{File, self};
use std::path::{PathBuf, Path};
use std::io::{Write};

/// error_chain errors mod
mod errors {
    error_chain!{
        foreign_links {
            GetOpts(::getopts::Fail);
            Io(::std::io::Error);
            Json(::serde_json::Error);
        }
    }
}

use errors::*;
quick_main!(run);

fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = parse_args(&args)?;

    if config.is_verbose() {
        println!("settings:\n{:#?}", &config);
    };

    match config {
        Config::Parse{ref input, ref output, kind, verbose} => {
            let input_file = File::open(&input)
                .chain_err(||format!("opening file <{:?}> for reading", &input))?;

            let stage_struct = parser::parse_stage_binary(input_file, kind, verbose)
                .chain_err(||format!("parsing <{:?}> to stage main JSON file <{:?}>", &input, &output))?;

            // create any directories if needed for output
            create_dir(output.as_path())
                .chain_err(||format!("creating output directory for <{:?}>", &output))?;

             let output_file = File::create(&output)
                .chain_err(||format!("creating file <{:?}> for writing output", &output))?;

            serde_json::to_writer_pretty(output_file, &stage_struct)
                .chain_err(||format!("writing JSON to ouptut file <{:?}>", &output))?;
        },
        Config::Build{ref input, ref output, verbose} =>
        {
            let input_json       = File::open(&input)
                .chain_err(||format!("opening json file <{:?}>", &input))?;
            let input_stage_data = serde_json::from_reader(input_json)
                .chain_err(||format!("deserializing JSON file <{:?}>", &input))?;
            let stage_binary     = builder::build_binary(&input_stage_data, verbose)?;

            // create any directories if needed for output
            create_dir(output.as_path())
                .chain_err(||format!("creating output directory for <{:?}>", &output))?;

            let mut output_file = File::create(&output)
                .chain_err(||format!("creating file <{:?}> for writing output", &output))?;

            output_file.write_all(&stage_binary)
                .chain_err(||format!("writing binary stage data to <{:?}>", &output))?;

        },
        Config::Help => (),
    }


    Ok(())
}
#[derive(Debug)]
enum Config {
    Parse {
        input: PathBuf,
        output: PathBuf,
        kind: Option<StageFileKind>,
        verbose: bool,
    },
    Build {
        input: PathBuf,
        output: PathBuf,
        verbose: bool,
    },
    Help
}

impl Config {
    fn is_verbose(&self) -> bool {
        use Config::{Parse, Build};

        match self {
            &Parse{verbose: true, ..} => true,
            &Build{verbose: true, ..} => true,
            _ => false
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StageFileKind {
    NoItem = 0x00,
    Item = 0x14
}
/// create directories for the Path "full_path" to a file
/// return `Result<bool>` to indicate if a directory was made
fn create_dir(full_path: &Path) -> Result<bool> {
    let dir_only = full_path.parent();

    match dir_only {
        Some(path) => {
            fs::create_dir_all(path)?;
            Ok(true)
        },
        None => Ok(false)
    }
}

fn parse_args(args: &[String]) -> Result<Config>
{
    use Config::*;

    let (program, args) = match args.split_first() {
        Some(split) => split,
        None => bail!("no args received from command line")
    };

    let opts = cli_options();

    let matches = opts.parse(args)
        .chain_err(||format!("parsing cli arguments:\n{:?}", &args))?;

    if matches.opt_present("h") {
        print_usage(program, opts);
        return Ok(Help);
    }

    if matches.opt_present("V") {
        print_version();
        return Ok(Help);
    }

    if matches.free.len() < 2 {
        let free = matches.free.len();
        if free == 0 {
            eprintln!("No subcommand argument found!");
        }
        eprintln!("No <FILE> argument found!\n");
        print_usage(program, opts);
        return Ok(Help);
    }

    let subcmd = &matches.free[0];
    let input = &matches.free[1];
    let output = matches.opt_str("o");
    let verbose = matches.opt_present("verbose");

    let config = match subcmd.as_str() {
        "parse" => {
            let input = PathBuf::from(&input);
            let output = match output {
                Some(ref path) => PathBuf::from(&path),
                None => {
                    let mut o = input.clone();
                    o.set_extension("json");
                    o
                }
            };
            let kind = match matches.opt_str("t") {
                Some(val) => {
                    match val.as_str() {
                        "no-item" => Some(StageFileKind::NoItem),
                        "item" => Some(StageFileKind::Item),
                        _ => None,
                    }
                },
                None => None
            };

            Parse{input, output, kind, verbose}
        },
        "build" => {
            let input = PathBuf::from(&input);
            let output = match output {
                Some(ref path) => PathBuf::from(&path),
                None => {
                    let mut o = input.clone();
                    o.set_extension("bin");
                    o
                }
            };
            Build{input, output, verbose}
        },
        _ => {
            eprintln!("Unknown subcommand <{}> entered.", subcmd);
            print_usage(program, opts);
            Help
        }
    };

    Ok(config)
}

fn cli_options() -> Options {
    let mut opts = Options::new();
    opts.optopt("o", "output", "set name of output file", "NAME");
    opts.optopt("t", "type", "manually set the type of stage file when parsing\n\
        Either \"no-item\" or \"item\"", "TYPE");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("", "verbose", "print additional information to the console");
    opts.optflag("V", "version", "print version information");

    opts
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage:
    {p} parse FILE [options]
    {p} build FILE [options]", p=program);
    print!("{}", opts.usage(&brief));
}

fn print_version() {
    println!("{} {}\n{}",
    env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));
}
