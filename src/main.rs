/*#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate flate2;
extern crate tar;

use std::io::Read;
use std::path::Path;
use flate2::read::GzDecoder;
use tar::Archive;
use std::process::Command;

mod errors {
    error_chain!{
        foreign_links {
            IoError(::std::io::Error);
            ReqwestError(::reqwest::Error);
        }
    }
}

use errors::*;

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }

    let cwd1 = "/home/alisha/temp_crate/zopfli-0.3.3";
    let path1 = Path::new(&cwd1);
    test_crate(path1);
}

fn download(url: &str) -> Result<reqwest::Response> {
     let client = reqwest::Client::new().expect("could not setup https client");
     client.get(url).send().map_err(|e| e.into())
}

fn unpack_to_folder<R: Read>(archive: &mut Archive<R>, path: &Path) -> Result<()> {
   archive.unpack(&path)?;
   Ok(())
}

fn run() -> Result<()> {
  let cwd = "/home/alisha/temp_crate";
  let path = Path::new(&cwd);
  let url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/zopfli/zopfli-0.3.3.crate";
  let bin = download(&url).chain_err(
        || format!("unable to download {}", url),
  )?;
  let mut tar = Archive::new(GzDecoder::new(bin)?);
  let r = unpack_to_folder(&mut tar, path).chain_err(|| "unable to unpack crate tarball");
  r
}

// cargo test using std::process::Command

fn test_crate(cd: &Path) -> Result<()> {
    let cmd = Command::new("cargo").arg("test").current_dir(cd).output()?;
    let cmdstr = format!{"{:?}", cmd};

    if cmd.status.success() {
        Ok(())
    } else {
        Err(format!("command `{}` failed", cmdstr).into())
    }
}

*/

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


