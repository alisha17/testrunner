#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate flate2;
extern crate tar;
extern crate toml;
extern crate walkdir;

use std::error::Error;
use std::fs::File;
use walkdir::WalkDir;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;

use std::collections::HashMap;
use toml::Value;
use flate2::read::GzDecoder;
use tar::Archive;
use std::process::Command;
mod crates;

mod errors {
    error_chain!{
        foreign_links {
            IoError(::std::io::Error);
            ReqwestError(::reqwest::Error);
        }
    }
}

use errors::*;

fn download_tarballs(path: &PathBuf, crate_version: HashMap<String, String>) -> Result<()>{
   for (crate_name, version) in crate_version {
        let ver = format!("/{}-{}", &crate_name, version);
        let mut path_to_crate = path.join(ver);
        println!("{:?}",path_to_crate);
            //if Path::new(&path_to_crate).exists() == false {
                let url = format!("https://crates-io.s3-us-west-1.amazonaws.com/crates/{0}/{0}-{1}.crate", crate_name, version);
                let client = reqwest::Client::new().expect("could not setup https client");
                let bin: reqwest::Response = client.get(&url).send()?;
                let mut tar = Archive::new(GzDecoder::new(bin)?);
                println!("before unpacking to folder");
                let archive = &mut tar;
                archive.unpack(&path).chain_err(|| "unable to unpack crate tarball")?;
           // }
            //test_crate(Path::new(&path_to_crate));
        }     
        Ok(())
}

// cargo test using std::process::Command

fn test_crate(cd: &Path) -> Result<()> {
    let cmd = Command::new("cargo").arg("test").current_dir(cd).output()?;
    let cmdstr = format!{"{:?}", cmd};
    println!("{:?}", cmdstr);

    if cmd.status.success() {
        println!("It was a success!");
        Ok(())
    } else {
        Err(format!("command `{}` failed", cmdstr).into())
    }
}

fn run() -> Result<()> {
  let cwd = "/home/alisha/temp_crate";
  let path = PathBuf::from(&cwd);
  let mut crate_version = crates::get_crates_and_versions();
  download_tarballs(&path, crate_version);
 // let cache_dir: PathBuf = env::var("OUT_DIR").join("tarball-cache")
 // println!("{:?}", crate_version);
  Ok(())
}  

quick_main!(run);


