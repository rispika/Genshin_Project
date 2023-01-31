extern crate fs2;
use std::{path::Path, fs::File};
use fs2::FileExt;

pub fn check_lock() {
    if !Path::exists(Path::new("process.lock")) {
        File::create("process.lock").unwrap();
    }
    File::open("process.lock").unwrap();
    
}