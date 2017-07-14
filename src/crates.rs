extern crate reqwest;
extern crate flate2;
extern crate tar;
extern crate toml;
extern crate walkdir;

use std::error::Error;
use std::fs::File;
use walkdir::WalkDir;
use std::io::prelude::*;
use std::path::{Path,PathBuf};
use flate2::read::GzDecoder;
use tar::Archive;
use std::process::Command;
use std::collections::HashMap;
use toml::Value;

use errors::*;

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

pub fn download_tarballs(path: &PathBuf, crate_version: HashMap<String, String>) -> Result<()>{
   for (crate_name, version) in crate_version {
        let ver = format!("/{}-{}", &crate_name, version);
        //let mut path_to_crate = path.join(ver);
            //if Path::new(&path_to_crate).exists() == false {
                let url = format!("https://crates-io.s3-us-west-1.amazonaws.com/crates/{0}/{0}-{1}.crate", crate_name, version);
                let client = reqwest::Client::new().expect("could not setup https client");
                let bin: reqwest::Response = client.get(&url).send()?;
                let mut tar = Archive::new(GzDecoder::new(bin)?);
                //println!("before unpacking to gitfolder");
                let archive = &mut tar;
                archive.unpack(&path).chain_err(|| "unable to unpack crate tarball")?;
           // }
            //test_crate(Path::new(&path_to_crate));
        }     
        Ok(())
}

// cargo test using std::process::Command

pub fn test_crate(cache_dir: &PathBuf) -> Result<()> {
    let cmd = Command::new("cargo").arg("test").current_dir(cache_dir).output()?;
    let cmdstr = format!{"{:?}", cmd};
    println!("{:?}", cmdstr);

    if cmd.status.success() {
        println!("It was a success!");
        Ok(())
    } else {
        Err(format!("command `{}` failed", cmdstr).into())
    }
}




