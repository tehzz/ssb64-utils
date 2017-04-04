#[macro_use]
extern crate clap;
use clap::{App, Arg};

//http://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

fn main() {
    let matches = App::new("Bass Symbol to Nemu Bookmark Converter")
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
                    ).get_matches();



    if let Some(input) = matches.value_of("INPUT") {
        println!("Value for INPUT: {}", input);
    }

    if let Some(indent) = matches.value_of("indent"){
        println!("Value for indent: {}", indent);
    }

    let path = Path::new(matches.value_of("INPUT").unwrap());

    let f = File::open(path).expect("Unable to read file");
    let br = BufReader::new(f);

    for line in br.lines(){
        for substr in line.unwrap().split_whitespace(){
            println!("Substr: {}", substr );
        }
        //println!("{}", line.unwrap());
    }
}
