extern crate toml;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use toml::Value;

pub fn extract_crate_info() -> HashMap<String, String> {
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

//use if let
    for (key, value) in abc {
            match value.as_table() {
                Some(x) => {
                    for (k, v) in x {
                        if k == "version" {
                            match v.as_str() {
                                Some(y) => {
                                crate_info.insert(key.to_string(), y.to_string());
                                } ,
                                None    => {},
                            } 
                        }
                    }
                },
                None    => {},
            }

            match value.as_str() {
                Some(x) => {
                    crate_info.insert(key.to_string(), x.to_string());
                } ,
                None    => {},
        }       
    }
    crate_info
}




