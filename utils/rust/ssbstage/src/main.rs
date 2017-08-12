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
mod parse;
mod stage;

use getopts::Options;
use std::env;
use std::fs::{File};
use std::path::{PathBuf};



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

    match config {
        Config::Parse{ref input, ref output, kind, verbose} => {
            let input_file = File::open(&input)
                .chain_err(||format!("opening file <{:?}> for reading", &input))?;

            let stage_struct = parse::stage_binary(input_file, kind, verbose)
                .chain_err(||format!("parsing <{:?}> to stage main JSON file <{:?}>", &input, &output))?;
            println!("{}", serde_json::to_string_pretty(&stage_struct)?);

             let output_file = File::create(&output)
                .chain_err(||format!("creating file <{:?}> for writing output", &output))?;

            serde_json::to_writer_pretty(output_file, &stage_struct)
                .chain_err(||format!("writing JSON to ouptut file <{:?}>", &output))?;
        },
        Config::Build{..} => println!("Not implemented"),
        Config::Help => (),
    }
    println!("{:#?}", config);

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StageFileKind {
    NoItem = 0x00,
    Item = 0x14
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

    opts
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage:
    {p} parse FILE [options]
    {p} build FILE [options]", p=program);
    print!("{}", opts.usage(&brief));
}
