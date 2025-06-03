use clap::Parser;
use core::panic;

use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Args {

    #[arg()]
    directory: String,

    #[arg()]
    recursive: bool

    // TODO: Add more args related to compare

}

fn main() {

    let args = Args::parse();

    let directory = match std::path::absolute(args.directory) {
        Ok(a) => a,
        Err(_) => panic!("Directory is invalid"),
    };

    let func: &dyn Fn(&DirEntry) = &|entry| {
        if !std::path::Path::extension(&entry.path()).is_some_and(|ex| ex.eq(".svg")) { return; }
        
        // let svg_data = fs::read_to_string(entry.path());
        // Fix the Svg (to a new path)
        // Clean the svg (at the new path)
        // Compare the two svgs (at the old and new path)
        // result is equal -> override the old path with the new path
        // result is not equal -> delete the new path

    };

    let _ = visit_dirs(directory.as_path(), func);
    
}

// let clos: fn(usize) -> usize = |x| x + 5;
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
