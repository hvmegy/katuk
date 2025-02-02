use crate::bookmark::Bookmark;
use crate::display;
use std::path::Path;
use std::{
    collections::BTreeMap,
    env,
    error::Error,
    fs::{self, File},
    io::prelude::*,
};

type Map<K, V> = BTreeMap<K, V>;

pub struct Bookmarker {
    map: Map<String, String>,
}

impl Bookmarker {
    pub fn new() -> Self {
        let mut path = match env::var("HOME") {
            Ok(home_dir) => String::from(home_dir),
            Err(_) => display::print_err("Cant find home directory".into()),
        };

        path.push_str("/.cache/katuk/data");

        {
            let path = Path::new(&path);
            if let Some(parent) = path.parent() {
                if parent.exists() == false {
                    let _ = fs::create_dir_all(&parent);
                }
            }

            if path.exists() == false {
                let _ = File::create(path);
            }
        }

        let data = match fs::read_to_string(path) {
            Ok(data) => data,
            Err(_err) => {
                // display::print_err(Box::new(err));
                display::print_err("Cannot read data".into())
            }
        };

        let mut map: Map<String, String> = Map::new();
        let data: Vec<&str> = data.lines().collect();
        for line in data {
            let word: Vec<&str> = line.split_whitespace().collect();
            map.insert(word[0].to_string(), word[1].to_string());
        }
        Self { map }
    }

    pub fn get_path(&self, name: &str) -> Result<String, Box<dyn Error>> {
        match self.map.get(name) {
            Some(path) => return Ok(path.to_string()),
            None => return Err(format!("The bookmark does not exists").into()),
        };
    }

    pub fn add_bookmark(&mut self, name: &str, path: &str) -> Result<(), Box<dyn Error>> {
        match self.map.get(name) {
            Some(path) => {
                Err(format!("Exist a bookmarks with a same name\n{} -> {}", name, path).into())
            }
            None => {
                self.map.insert(name.to_string(), path.to_string());
                Ok(())
            }
        }
    }

    pub fn edit_bookmark(&mut self, name: &str, path: &str) -> Result<(), Box<dyn Error>> {
        match self.map.get(name) {
            Some(..) => {
                self.map.insert(name.to_string(), path.to_string());
                Ok(())
            }
            None => Err(format!("The bookmark does not exists").into()),
        }
    }

    pub fn remove_bookmark(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        match self.map.get(name) {
            Some(..) => {
                self.map.remove(name);
                Ok(())
            }
            None => Err(format!("The bookmark does not exists").into()),
        }
    }

    pub fn list_bookmark(&self) -> Result<(), Box<dyn Error>> {
        for (name, path) in &self.map {
            let bookmark = Bookmark::new(name.clone(), path.clone());
            display::print_bookmark(bookmark);
        }
        Ok(())
    }

    pub fn write(&self) -> () {
        let mut path = match env::var("HOME") {
            Ok(home_dir) => String::from(home_dir),
            Err(_) => display::print_err("Cannot find home directory".into()),
        };

        path.push_str("/.cache/katuk/data");

        {
            let path = Path::new(&path);
            if let Some(parent) = path.parent() {
                if parent.exists() == false {
                    let _ = fs::create_dir_all(&parent);
                }
            }

            if path.exists() == false {
                let _ = File::create(path);
            }
        }

        let mut file = File::create(path).expect("Cannot create file");
        for (key, value) in &self.map {
            let _ = writeln!(file, "{} {}", key, value);
        }
    }
}
