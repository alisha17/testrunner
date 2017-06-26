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

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
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
  let path = Path::new("/home/alisha/temp_crate");
  let url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/zopfli/zopfli-0.3.3.crate";
  let bin = download(&url).chain_err(
        || format!("unable to download {}", url),
  )?;
  let mut tar = Archive::new(GzDecoder::new(bin)?);
  let r = unpack_to_folder(&mut tar, path).chain_err(|| "unable to unpack crate tarball");
  r
}