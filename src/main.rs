extern crate clap;

use clap::{App,Arg}; 

static DEFAULT_PROMPT: &'static str = "${c::0xde,bold}c${c::0xdd}o${c::0xdc}l${c::0xdb}e${c::0xda}m${c::0xd9}a${c::0xd8}n${c::0xd7}:${c::0xd6}$basename(${SWD})${c::0x05} # ${c::reset}";
 
fn main() { 
    let matches = App::new("candypaint")
       .version("0.1.0")
       .about("Make the prompt drip stains")
       .author("Coleman Emery McFarland")
       .arg(Arg::with_name("theme").help("theme name").required(false))
       .get_matches(); 

    if let Some(theme_key) = matches.value_of("theme") {
    } else {
        println!("{}", DEFAULT_PROMPT);
    };
}

fn default() -> String {
    let mut ret = String::new();

    let range: Vec<i32> = (0xd6..0xde).rev().collect();

    ret
}

