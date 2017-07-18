use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;


fn main() {
    let c_dir = env::var("OUT_DIR").unwrap();
    let cache_dir = Path::new(&c_dir).join("tarball-cache");
    //let mut f = File::open(&dest_path);

    let mut crate_version = crates::get_crates_and_versions();

    crates::download_tarballs(&cache_dir, crate_version);
    
    generate_tests(&cache_dir, crate_version);
    
}

fn generate_tests(cache_dir, crates) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("stdx-tests.rs");
    //let mut f = File::open(&dest_path);

    // For each crate
    for (name, version) in crates {
        generate_single_test(dest_path, cache_dir, name, version);
    }
}

fn generate_single test(path: &Path, cache_dir: &Path, name, version) {
    write!(cache, "
    
#[test]
fn test_rand() {
    // create a temporary directory (TempDir type from the tempdir crate)
    
    // unpack rand tarball into the temporary directory
    
    // run cargo test on the Cargo.toml in that temporary directory
}
", cache_dir, name, vers);

}

