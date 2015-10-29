
#![feature(libc)]


extern crate regex;
extern crate libc;

use std::process;

pub mod match_entry;
pub mod parser;

pub fn complete_suggestion(pos: (i32,i32), fln: &str) -> Vec<match_entry::MatchEntry> {
    // let mut child = process::Command::new("racer")
    //                                 .arg("complete")
    //                                 .arg(&format!("{} {}", pos.0,pos.1))
    //                                 .arg(fln)
    //                                 .spawn().unwrap();
    let out = process::Command::new("racer").arg("complete")
                                                  .arg(format!("{}",pos.0))
                                                  .arg(format!("{}",pos.1))
                                                  .arg(format!("{}",fln)).output().unwrap();
    //println!("{:?} {}", out.stdout, format!("{} {}", pos.0, pos.1));
    let out = (*String::from_utf8_lossy(&out.stdout)).to_string();
    //println!("{} {}", out, format!("{} {}", pos.0, pos.1));
    parser::parse(out)
}

#[no_mangle]
pub extern "C" fn get_entry() -> match_entry::c_mentry {
    match_entry::MatchEntry{name:"houssem".to_string(),
                            kind:"file".to_string(),
                            file:".".to_string(),
                            pos: (34,6)}.into()
}

#[no_mangle]
pub extern "C" fn get_entries() -> match_entry::c_mentries {
    let out = process::Command::new("racer").arg("complete")
                                                  .arg("std::fs::")
                                                  .output().unwrap();
    //println!("{:?} {}", out.stdout, format!("{} {}", pos.0, pos.1));
    let out = (*String::from_utf8_lossy(&out.stdout)).to_string();
    //println!("{} {}", out, format!("{} {}", pos.0, pos.1));
    let es = parser::parse(out);
    let p = match_entry::c_mentries::from(es);
    p
}
