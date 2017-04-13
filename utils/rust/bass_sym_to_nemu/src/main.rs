// http://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

extern crate clap;
use clap::{App, Arg};

mod flat;
use flat::flatten;

fn main() {
    let matches = cli().get_matches();

    //--Start debugging stuff... remove eventually...
    if let Some(input) = matches.value_of("INPUT") {
        println!("Value for INPUT: {}", input);
    }

    if let Some(indent) = matches.value_of("indent"){
        println!("Value for indent: {}", indent);
    }
    //--End Debugging BS

    // Get BufReader of file from INPUT from clap
    let path = Path::new(matches.value_of("INPUT").unwrap());
    let f    = File::open(path).expect("Unable to read input file");
    let br   = BufReader::new(f);

    // re-format the file!
    let output =
        if matches.is_present("flatten") {
            flatten(Box::new(br))
        } else {
            println!("Only \"flatten\" is implemented so far :(");
            flatten(Box::new(br))
        };

    println!("Test of FP:\n{}", output);

    // write the reformated string out to a file!
    let mut output_path = PathBuf::from(path);
    output_path.set_extension("nbm");

    let o = File::create(output_path)
            .expect("Unable to create output file :(");

    let mut bw = BufWriter::new(o);
    bw.write_all(output.as_bytes()).expect("Unable to write output file");
    /*
    let test_addr = SymbolInfo {
        addr: 0xA1234,
        name: "Test Function".to_string(),
        mem_type: MemType::CPU
    };

    println!("Test of SymbolInfo Print: {}", test_addr.print());
    */


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
        .arg(Arg::with_name("indent")
            .short("i")
            .long("indent")
            .takes_value(true)
            .help("Set the indentation level.
Any symbols with more than 'i' leves of scope have the higher levels put into folders
and removed the bookmark's name")
        )
        .arg(Arg::with_name("flatten")
            .short("f")
            .long("flatten")
            .multiple(false)
            .help("Make a quick one-to-one mapping between bass' output symbol file and Nemu's bookmarks")
        )
}
