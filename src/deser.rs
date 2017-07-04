extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;
#[macro_use]
extern crate semver;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use semver::Version;

#[derive(Debug, Deserialize)]
struct Manifest {
    package: Option<Box<Package>>,  
    dependencies: Option<HashMap<String, Dependency>>
}

#[derive(Debug, Deserialize)]
 struct Package { 
    name: String,
    version: Option<String>,
    authors: Option<Vec<String>>,
    description: Option<String>,
    license: Option<String>
 }

#[derive(Debug, Deserialize)]
 struct Dependency {
     version: semver::Version
 }

fn main() {
    let path_to_read = Path::new("cargo-stdx.toml");

    let mut file = match File::open(&path_to_read) {
        Err(why) => panic!("couldn't open due to: {}", why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read due to: {}", why.description()),
        Ok(_) => print!("contains: {}\n", s),
    }

    let decoded: Manifest = toml::from_str(&s).unwrap();
    println!("{:?}", decoded);
}
