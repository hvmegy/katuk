pub mod bookmark;
pub mod bookmarker;
pub mod display;

use bookmarker::Bookmarker;
use std::path::Path;
use std::{env, error::Error};

pub fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(format!("Invalid arguments").into());
    } else {
        let mut bookmarker = Bookmarker::new();
        match args[1].as_str() {
            "-h" => {}
            "-a" => {
                let name = match args.get(2) {
                    Some(name) => name,
                    None => return Err(format!("Invalid arguments").into()),
                };

                let current_dir = env::current_dir().unwrap().to_str().unwrap().to_string();

                let path = match args.get(3) {
                    Some(path) => {
                        if Path::new(&path).exists() {
                            path
                        } else {
                            return Err(format!("Path does not exist").into());
                        }
                    }
                    None => &current_dir,
                };

                match bookmarker.add_bookmark(name, path) {
                    Ok(()) => {
                        println!("add");
                        display::print_ok(&format!("added bookmark {} -> {}", name, path));
                        bookmarker.write();
                    }
                    Err(err) => {
                        display::print_err(err);
                    }
                }
            }
            "-p" => {
                // get path
                let name = match args.get(2) {
                    Some(name) => name,
                    None => return Err(format!("Invalid arguments").into()),
                };

                match bookmarker.get_path(&name) {
                    Ok(path) => {
                        println!("cd");
                        println!("{}", path);
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            "-l" => {
                println!("ls");
                let _ = bookmarker.list_bookmark();
            }
            "-e" => {
                let name = match args.get(2) {
                    Some(name) => name,
                    None => return Err(format!("Invalid arguments").into()),
                };

                let current_dir = env::current_dir().unwrap().to_str().unwrap().to_string();

                let path = match args.get(3) {
                    Some(path) => {
                        if Path::new(&path).exists() {
                            path
                        } else {
                            return Err(format!("Path does not exist").into());
                        }
                    }
                    None => &current_dir,
                };

                match bookmarker.edit_bookmark(name, path) {
                    Ok(()) => {
                        println!("edit");
                        display::print_ok(&format!("edited bookmark {} -> {}", name, path));
                        bookmarker.write();
                    }
                    Err(err) => {
                        display::print_err(err);
                    }
                }
            }
            "-d" => {
                let name = match args.get(2) {
                    Some(name) => name,
                    None => return Err(format!("Invalid arguments").into()),
                };
                match bookmarker.remove_bookmark(name) {
                    Ok(()) => {
                        println!("remove");
                        display::print_ok(&format!("removed bookmark {}", name));
                        bookmarker.write();
                    }
                    Err(err) => {
                        display::print_err(err);
                    }
                }
            }
            _ => {
                return Err(format!("Invalid arguments!").into());
            }
        }
    }
    Ok(())
}
