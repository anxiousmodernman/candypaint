extern crate clap;

use clap::{App,Arg}; 
use std::env;

 
fn main() { 
    let matches = App::new("candypaint")
       .version("0.2.0")
       .about("candy coated prompts for the ion shell")
       .author("Coleman Emery McFarland")
       .arg(Arg::with_name("theme").help("theme name").required(false))
       .get_matches(); 

    let prompt = match matches.value_of("theme") {
        Some(theme) => {
            match theme {
                "chad" => chad(),
                "darkside" => darkside(),
                _ => chad(),
            }
        },
        _ => None, 
    };

    println!("export CANDY = \"{}\"", prompt.unwrap_or(String::from("export CANDY = \"# ${c::reset}\"")));
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
        if let Some(pwd) = path.file_name() {
            ret.push_str("${c::0xd6}");
            let s = format!("{}", pwd.to_str().unwrap_or(""));
            ret.push_str(&s)
        }
    }

    if let Some(git_info) = git_info() {
        ret.push_str(&format!(" (${{c::0xb8}}{}${{c::0xd6}}) ${{c::0x05}}# ${{c::reset}}", &git_info.branch.trim()));
    } else {
        ret.push_str(" ${c::0x05}# ${c::reset}");
    }

    Some(ret)
}

/// darkside is scary.
fn darkside() -> Option<String> {

    let mut path = String::new();
    if let Ok(cwd) = env::current_dir() {
        if let Some(val) = cwd.as_path().to_str() {
            path.push_str(val);
        }
    }

    // black -> light grey
    let range: Vec<i32> = (0xe8..0xfe).collect();

    let mut temp = String::new();
    let length = path.len();

    let mut idx = 0;
    for (i, c) in path.chars().enumerate() {
        let mut inc: bool;
        if length > range.len() {
            if i < (length - range.len()) {
                continue
            }
            inc = true;
        } else {
            inc = true;
        }
        if let Some(num) = range.get(idx) {
            temp.push_str("${c::0x");
            let s = format!("{:X},bold}}{}", num, c);
            temp.push_str(&s);
        } else { 
            break 
        }
        if inc {
            idx += 1; 
        }
    }
    if let Some(git_info) = git_info() {
        let s = format!(" ${{c::0x7c}}<<{}${{c::0x7c}}>> ${{c::reset}}", &git_info.branch.trim());
        temp.push_str(&s);
    } else {
        temp.push_str(" ${c::0x7c}>> ${c::reset}");
    }
    Some(temp)
}

fn git_info() -> Option<GitInfo> {
    use std::process::Command;
    let mut cmd = Command::new("git");
    cmd.args(vec!["rev-parse", "--symbolic-full-name", "--abbrev-ref", "HEAD"]);
    let output = cmd.output().ok()?;
    if !output.status.success() {
        return None
    }
    let branch = String::from_utf8(output.stdout).ok()?;
    Some(GitInfo{ branch: branch })
}

/// GitInfo holds state related to the current git repo, if present.
#[derive(Debug)]
struct GitInfo {
    branch: String,
}


