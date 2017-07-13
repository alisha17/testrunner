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


// replace with quick_main()
quick_main!(run);

fn download(url: &str) -> Result<reqwest::Response> {
     let client = reqwest::Client::new().expect("could not setup https client");
     client.get(url).send().map_err(|e| e.into())
}

fn unpack_to_folder<R: Read>(archive: &mut Archive<R>, path: &Path) -> Result<()> {
   archive.unpack(&path)?;
   Ok(())
}

//download

fn run() -> Result<()> {
  let cwd = "/home/alisha/temp_crate";
  let path = Path::new(&cwd);
  let mut crate_version = crates::get_crates_and_versions();
  println!("{:?}", crate_version);
  for (crate_name, version) in crate_version {
      println!("inside for");
        let mut path_to_crate = format!("{}/{}", &cwd, format!("{}-{}", &crate_name, version));
            if Path::new(&path_to_crate).exists() == false {
                let url = format!("https://crates-io.s3-us-west-1.amazonaws.com/crates/{0}/{0}-{1}.crate", crate_name, version);
                let bin = download(&url).chain_err(
                    || format!("unable to download from {}", url),
                )?;
                let mut tar = Archive::new(GzDecoder::new(bin)?);
                println!("before unpacking to folder");
                let r = unpack_to_folder(&mut tar, path).chain_err(|| "unable to unpack crate tarball");  
            }
            test_crate(Path::new(&path_to_crate));

        }     

  Ok(())
}  

// run and test crate

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


