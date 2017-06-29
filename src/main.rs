#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate flate2;
extern crate tar;

use std::io::Read;
use std::path::Path;
use flate2::read::GzDecoder;
use tar::Archive;
use std::env;
use std::process::{Command, Output};

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
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

    let r = test_crate(&env::current_dir().unwrap(),"cargo", &"test");
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
  let cwd = env::current_dir().unwrap();
  let path = Path::new(&cwd);
  let url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/zopfli/zopfli-0.3.3.crate";
  let bin = download(&url).chain_err(
        || format!("unable to download {}", url),
  )?;
  let mut tar = Archive::new(GzDecoder::new(bin)?);
  let r = unpack_to_folder(&mut tar, path).chain_err(|| "unable to unpack crate tarball");
  r
}

fn test_crate(cd: &Path, name: &str, args: &str) -> Result<()> {
    let mut cmd = Command::new("cargo").arg("test").output()?;
    let cmdstr = format!{"{:?}", cmd};

    if cmd.status.success() {
        Ok(())
    } else {
        Err(format!("command `{}` failed", cmdstr).into())
    }
}

 /* .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
    });

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("rustc failed and stderr was:\n{}", s);
    }
}
*/