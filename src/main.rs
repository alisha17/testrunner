#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate flate2;
extern crate tar;
extern crate toml;
extern crate walkdir;
extern crate tempdir;

use std::env;
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
            VarError(::std::env::VarError);
            ReqwestError(::reqwest::Error);
        }
    }
}

use errors::*;

fn run() -> Result<()>{
    Ok(())
}  

quick_main!(run);


