extern crate clap;

use clap::{App,Arg}; 
use std::env;
use std::path::Path;

 
fn main() { 
    let matches = App::new("candypaint")
       .version("0.1.0")
       .about("candy coated prompts for the ion shell")
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
        _ => None, 
    };

    println!("{}", prompt.unwrap_or(String::from("# ${c::reset}")));
}

/// chad is our default theme.
fn chad() -> Option<String> {

    let mut ret = String::new();

    let range: Vec<i32> = (0xd6..0xde).rev().collect();

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

    if Path::new(".git").exists() {
        ret.push_str(" (${git::branch}) ${c::0x05}# ${c::reset}");
    } else {
        ret.push_str(" ${c::0x05} # ${c::reset}");
    }

    Some(ret)
}

