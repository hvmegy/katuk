use crate::bookmark::Bookmark;
use std::{error::Error, process};

pub fn print_err(err: Box<dyn Error>) -> ! {
    println!("err");
    println!("Failed: {}", err);
    process::exit(1);
}

pub fn print_ok(ok: &String) -> () {
    println!("Success: {}", ok);
}

pub fn print_bookmark(bookmark: Bookmark) {
    println!("{} -> {}", bookmark.name(), bookmark.path());
}
