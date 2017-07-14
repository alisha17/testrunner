use std::env;
use std::fs::File;
use std::collections::HashMap;
use std::path::{Path,PathBuf};
use std::io::Write;

use errors::*;

/*pub fn generate_tests(cache_dir: &PathBuf, crates: HashMap<String,String>) {
     let test_name = env::var("OUT_DIR").join("tests.rs");
     let test_file = File::open(test_name);

    for (name, version) in crates {
        generate_single_test(test_file, cache_dir, name, version);
    } 
}*/

/*pub fn generate_single_test() {
    
}*/

pub fn generate_file() -> Result<()>{
   println!("hbbhbn");
    let out_dir = env!("OUT_DIR");
    //let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut out_file = PathBuf::from(out_dir.clone());
    out_file.push("my-test-file.rs");
    println!("{:?}",out_file);

    let mut test_file = File::create(out_file).unwrap();

    test_file.write_all("Welcome".as_bytes()).unwrap();
    Ok(())
}
