extern crate toml;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use toml::Value;

pub fn get_crates_and_versions() -> HashMap<String, String> {
    let path_to_read = Path::new("Cargo-stdx.toml");

    let mut file = match File::open(&path_to_read) {
        Err(why) => panic!("couldn't open due to: {}", why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read due to: {}", why.description()),
        Ok(_) => print!("Reading successful"),
    }
    
    let mut crate_info = HashMap::new();
    
    let doc = s.parse::<Value>().unwrap();
    let abc = doc["dependencies"].as_table().expect("dependency table");

    for (key, value) in abc {
         if let Some(x) = value.as_table() {
             for (k, v) in x {
                 if k == "version" {
                     if let Some(y) = v.as_str() {
                         crate_info.insert(key.to_string(), y.to_string());
                     }
                 }
             }    
        }

        if let Some(x) = value.as_str() {
             crate_info.insert(key.to_string(), x.to_string());
        }
    }
    crate_info
}




