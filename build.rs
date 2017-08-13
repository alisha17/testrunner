#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate flate2;
extern crate tar;
extern crate toml;
extern crate walkdir;
extern crate tempdir;

use std::fs;
use std::env;
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
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::fs::create_dir_all;

mod errors {
    error_chain!{
        foreign_links {
            IoError(::std::io::Error);
            VarError(::std::env::VarError);
            ReqwestError(::reqwest::Error);
        }
    }
}

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

pub fn download_tarballs(path: &Path, crate_version: &HashMap<String, String>) -> Result<()>{
    println!("{:?}",path);
   for (crate_name, version) in crate_version {
        //let mut path_to_crate = path.join(ver);
            //if Path::new(&path_to_crate).exists() == false {
                let url = format!("https://crates-io.s3-us-west-1.amazonaws.com/crates/{0}/{0}-{1}.crate", crate_name, version);
                let client = reqwest::Client::new().expect("could not setup https client");
                let mut bin: reqwest::Response = client.get(&url).send()?;
                let dl_crate = format!("{}-{}.crate",crate_name, version);
                //let new_path = path.join(dl_crate);
                //println!("Here: {:?}",new_path);
                let mut new_path = Path::new(path).join(dl_crate);
                let mut file = OpenOptions::new().write(true).create(true).open(new_path).unwrap();
                println!("bdbdbdb");
                io::copy(& mut bin, & mut file);  
                println!("hheee");
        }     
        Ok(())
}

fn main() {
    let c_dir = env::var("OUT_DIR").unwrap();
    let a_dir = PathBuf::from(&c_dir).as_path().join("tarball-cache");
    let tem = create_dir_all(a_dir).unwrap();
    let tempnew = format!("{}/tarball-cache/", &c_dir);
    let cache_dir = Path::new(&tempnew);
    println!("{:?}",cache_dir);
    let mut crate_version = get_crates_and_versions();
    println!("{:?}", crate_version);

    download_tarballs(&cache_dir, &crate_version);

    generate_tests(&cache_dir, &crate_version);
}

fn generate_tests(cache_dir: &Path, crates: &HashMap<String, String>) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let destpath = Path::new(&out_dir).join("sdx-tests.rs");
    let mut dest_path = File::create(&destpath).unwrap();
    // For each crate  
    for (name, version) in crates {
        println!("inside for");
        generate_single_test(&mut dest_path, &cache_dir, &name, &version);
   }
}

fn generate_single_test(dest_path: &mut File, cache_dir: &Path, name: &String, version: &String) {

    let expected = [
        "#[test]\n",
        "fn test_",name,"(){\n",
        "let outdir = TempDir::new('temptest').unwrap();\n",
        "let new_path = cache_dir.join('",name,"-",version,".crate');\n",
        "let mut tar = Archive::new(GzDecoder::new(new_path)?)\n",
        "let archive = &mut tar;\n",
        "archive.unpack(outdir);\n",
        "let mut path_to_test : &Path = outdir.clone();\n",
        "let mut path_to_test = path_to_test.join('",name,"-",version,"');\n",
        "let cmd = Command::new('cargo').arg('test').current_dir(path_to_test).output()?;\n",
        "if cmd.status.success() {\n",
        "    println!('It was a success!');\n",
        "    Ok(())\n",
        "} else {\n",
        "    Err(panic('command failed'))\n",
        "}\n",
      ].concat();

      dest_path.write(expected.as_bytes());
}    



