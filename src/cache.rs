extern crate crypto;

use std::fs::{File, create_dir_all};

use std::string::String;
use std::io::{Read,Write};
use std::error::Error;
use std::path::{Path, PathBuf};
use self::crypto::sha1::Sha1;
use self::crypto::digest::Digest;
    
pub trait Cache {
    fn get(&self, k: &str) -> String;
}

pub struct DiskCache<'a> {
    pub directory: String,
    pub compute: &'a Fn(&str) -> String
}

fn generate_path(directory: &str, url: &str) -> PathBuf {
    let mut hasher = Sha1::new();
    hasher.input_str(url);
    let hex = hasher.result_str();

    let mut path = PathBuf::from(directory);
    path.push(hex[..1].to_string());
    path.push(hex[..2].to_string());
    path.push(hex);
    return path
}

fn load_file(path: PathBuf) -> String {
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                           Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                           Error::description(&why)),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
    return s;
}

fn save_file(path: PathBuf, s: &String) {
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           Error::description(&why)),
        Ok(file) => file,
    };
    
    match file.write_all(s.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                   Error::description(&why))
        },
        Ok(_) => return
    }
}

impl<'a> Cache for DiskCache<'a> {
    
    fn get(&self, key: &str) -> String {
        let path = generate_path(&*self.directory, key); // TODO I had to return PathBuf for reasons I don't understand yet
        if path.exists() {
            return load_file(path)
        } else {
            let result = (self.compute)(key);
//            try!( // TODO re-enable
            create_dir_all(path.parent().unwrap())//)
                ;
            save_file(path, &result);
            return result
        }
    }
    
}
