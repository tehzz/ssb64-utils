// http://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

extern crate clap;
use clap::{App, Arg};

mod nemu_mem;
mod nest;
use nest::nester;

mod flat;
use flat::flatten;

fn main() {
    let matches = cli().get_matches();

    //-- Get CLI settings
    let scope = match matches.value_of("scope") {
        Some(s) => match s.parse::<usize>() {
            Ok(val) => val,
            Err(_)  => 0
        },
        None => 0
    };

    let nest: Option<usize> = match matches.value_of("nests") {
        Some(s) => match s.parse::<usize>() {
            Ok(val) => Some(val),
            Err(_)  => None,
        },
        None => None
    };

    let data_mask = match matches.value_of("data mask") {
        Some(mask) => mask,
        None       => "data"
    };

    // Debug printing
    println!("Debug cli arguments:");
    println!("Scope: {} \nNest: {:?}", scope, nest);
    println!("Data Mask: {}", data_mask);

    // Get BufReader of file from INPUT from clap
    let path = Path::new(matches.value_of("INPUT").unwrap());
    let f    = File::open(path).expect("Unable to read input file\n\n");
    let br   = Box::new(BufReader::new(f));

    // re-format the file!
    let output =
        if matches.is_present("flatten") {
            flatten(br, &data_mask)
        } else {
            let test = nester(br, scope, nest, &data_mask);
            println!("Scope Test: {}", test);
            panic!("Only \"flatten\" is implemented so far :(");
        };

    // write the reformated string out to a file!
    let output_path =
        if let Some(custom_output) = matches.value_of("output") {
            let mut temp = PathBuf::from(custom_output);
            match temp.extension() {
                Some(_) => (),
                None    => {
                    temp.set_extension("nbm");
                    ()
                }
            };
            temp
        } else {
            let mut temp = PathBuf::from(path);
            temp.set_extension("nbm");
            temp
        };


    let o = File::create(output_path)
            .expect("Unable to create output file :(\n\n");

    let mut bw = BufWriter::new(o);
    bw.write_all(output.as_bytes()).expect("Unable to write to output file");
}

fn cli<'a,'b>() -> App<'a,'b> {
    App::new("Bass Symbol to Nemu Bookmark Converter")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Converts the symbol output text file from bass into an organized Nemu bookmark file")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input bass symbol file to convert")
            .required(true)
            .index(1))
        .arg(Arg::with_name("scope")
            .short("s")
            .long("scope")
            .takes_value(true)
            .help("Set the \"scope\" level of the output nemu bookmark file.
Any symbols whose name contains less than or equal to 's' scopes in bass will have those scopes added to the name of the symbol.
The default value is 0.

Example:
    \"fn.main_menu.init.loadAssets\"

    '-s 1': fn -> main_menu -> init.loadAssets
    '-s 3': fn.main_menu.init.loadAssets

")
        )
        .arg(Arg::with_name("nests")
            .short("n")
            .long("nest")
            .takes_value(true)
            .help("Set the \"nest\" level of the output nemu bookmark file.
This option is similar to scope, but it works from the top of the scope tree.
Setting a nest value limits how many nemu folders can be created from a symbol.
The default value is \"none\", which is unlimited nesting. A '0' value is the same as setting the 'flatten' flag

Example:
    \"fn.main_menu.init.loadAssets\"

    '    ': fn -> main_menu -> init -> loadAssets
    '-n 0': fn.main_menu.init.loadAssets
    '-n 1': fn -> main_menu.init.loadAssets
    '-n 3': fn -> main_menu -> init -> loadAssets

")
        )
        .arg(Arg::with_name("data mask")
            .short("d")
            .long("data-mask")
            .takes_value(true)
            .help("A string that can switch the memory type of a symbol from its default of 'CPU' to 'RAM'
This string needs to be its own scope within bass.

Default value is \"data\".

")
        )
        .arg(Arg::with_name("flatten")
            .short("f")
            .long("flatten")
            .multiple(false)
            .help("Make a quick one-to-one mapping between bass' output symbol file and Nemu's bookmarks.
Every bass symbol is converted 1-to-1 into a nemu bookmark entry.

")
        )
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .takes_value(true)
            .multiple(false)
            .help("Explicitly set the name of the output file. Will automatically add \".nbm\" if no extension is specified
By default, the output file is \"<INPUT>.nbm\"

")
        )
}
