extern crate reqwest;
extern crate flate2;
extern crate tar;

use std::io::Read;
use std::fs;
use std::path::Path;
use flate2::read::GzDecoder;
use tar::Archive;

//fn get_contents(url: &str) -> reqwest::Result<reqwest::Response> {

fn download(url: &str) -> Result<reqwest::Response> {
    let mut client = reqwest::Client::new();
    client.get(url).send()
}

fn unpack_to_folder<R: Read>(archive: &mut Archive<R>, path: &Path) -> Result<()> {
   archive.unpack(&path)?;
   Ok(())
}

fn run() ->  Result<()> {
  let path = Path::new("/home/alisha/temp_crate");
  let bin = download("https://crates-io.s3-us-west-1.amazonaws.com/crates/zopfli/zopfli-0.3.3.crate");
  let archive = GzDecoder::new(bin)?;
  let mut archive = Archive::new(archive);
  let r = unpack_to_folder(&mut archive, path);
  r
}

fn main(){
     println!("{:?}",run());
}