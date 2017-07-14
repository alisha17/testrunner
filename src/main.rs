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
mod runtest;

mod errors {
    error_chain!{
        foreign_links {
            IoError(::std::io::Error);
            ReqwestError(::reqwest::Error);
        }
    }
}

use errors::*;

fn run() -> Result<()> {
  let cwd = "/home/alisha/temp_crate";
  let path = PathBuf::from(&cwd);
  let mut crate_version = crates::get_crates_and_versions();
  crates::download_tarballs(&path, crate_version);
 // let cache_dir: PathBuf = env::var("OUT_DIR").join("tarball-cache")
 // println!("{:?}", crate_version);
  //runtest::generate_tests(&path, crate_version);
  Ok(())
}  

quick_main!(run);


