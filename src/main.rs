extern crate clap;

use clap::{App,Arg}; 
use std::env;

// TODO this doesn't really work
static DEFAULT_PROMPT: &'static str = "${c::0xde,bold}c${c::0xdd}o${c::0xdc}l${c::0xdb}e${c::0xda}m${c::0xd9}a${c::0xd8}n${c::0xd7}:${c::0xd6}$basename(${SWD})${c::0x05} # ${c::reset}";
 
fn main() { 
    let matches = App::new("candypaint")
       .version("0.1.0")
       .about("candy coated ion shell prompts")
       .author("Coleman Emery McFarland")
       .arg(Arg::with_name("theme").help("theme name").required(false))
       .get_matches(); 

    let prompt = match matches.value_of("theme") {
        Some(theme) => {
            match theme {
                "chad" => chad(),
                _ => chad(),
            }
        },
        _ => Some(String::from(DEFAULT_PROMPT)), // this shouldn't happen, but we have a fallback anyway
    };

    println!("{}", prompt.unwrap_or(String::from("# ${c::reset}")));
}

fn chad() -> Option<String> {

    let mut ret = String::new();

    let range: Vec<i32> = (0xd6..0xde).rev().collect();

    // get user
    if let Ok(user) = env::var("USER") {
        for (i, c) in user.chars().enumerate() {
            if let Some(num) = range.get(i) {
                ret.push_str("${c::0x");
                let s = format!("{:X},bold}}{}", num, c);
                ret.push_str(&s);
            } else { 
                break 
            }
        }
        ret.push_str("${c::0xd7}:")
    }

    if let Ok(path) = env::current_dir() {
        if let Some(pwd) = path.file_stem() {
            ret.push_str("${c::0xd6}");
            let s = format!("{}", pwd.to_str().unwrap_or(""));
            ret.push_str(&s)
        }
    }

    // expand user
    ret.push_str("${c::0x05} # ${c::reset}");

    Some(ret)
}

