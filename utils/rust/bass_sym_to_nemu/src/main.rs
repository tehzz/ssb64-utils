#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!( TestApp =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: "Converts the symbol output text file from bass into an organized Nemu bookmark file")
        (@arg INPUT: +required "Sets the input file to use")
        (@arg indent: -i --indent +takes_value "Set the indentation level.
    Any symbols with more than 'i' leves of scope have the higher levels put into folders
and removed the bookmark's name ")
    ).get_matches();


    if let Some(input) = matches.value_of("INPUT") {
        println!("Value for INPUT: {}", input);
    }

    if let Some(indent) = matches.value_of("Indent"){
        println!("Value for indent: {}", indent);
    }
}
