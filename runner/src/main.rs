use clap::Parser;
use svgcleaner::{CleaningOptions, ParseOptions, WriteOptions};
use svgcompare::comparison::compare_svgs;
use core::panic;

use std::{io};
use std::fs::{self, DirEntry};
use std::path::Path;

use svgfixer::{fixlib::fix_svg};
use svgcleaner::{cleaner::parse_data, cleaner::clean_doc, cleaner::write_buffer};
use svgcompare::{comparison::ComparisonOptions};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Args {

    #[arg()]
    directory: String,

    // #[arg()]
    // recursive: bool
}

fn main() {

    let args = Args::parse();

    let directory = match std::path::absolute(args.directory) {
        Ok(a) => a,
        Err(_) => panic!("Directory is invalid"),
    };

    let func: &dyn Fn(&DirEntry) = &|entry| {
        if !std::path::Path::extension(&entry.path()).is_some_and(|ex| ex.eq("svg")) { return; }

        // TODO : Try/catch the function

        let svg_data = match fs::read_to_string(entry.path()) {
            Ok(a) => a,
            Err(_) => {
                panic!("Could not read SVG data");
            }
        };

        // Fix the Svg (to a new path)
        let fixed_svg_data = fix_svg(svg_data.clone());

        // Clean the svg (at the new path)
        let parse_options = ParseOptions::default();
        let mut svg_doc = match parse_data(&fixed_svg_data, &parse_options) {
            Ok(a) => a,
            Err(e) => panic!("{}", e),
        };

        let cleaning_options = CleaningOptions::default();
        let write_options = WriteOptions::default();
        let _ = match clean_doc(&mut svg_doc, &cleaning_options, &write_options) {
            Ok(a) => a,
            Err(e) => panic!("{}", e),
        };

        let mut buf = Vec::new();
        write_buffer(&svg_doc, &write_options, &mut buf);

        let cleaned_svg_data = str::from_utf8(&buf).unwrap().to_string();

        let svg_data_count = svg_data.len() as f64;
        let cleaned_svg_data_count = cleaned_svg_data.len() as f64;

        let comparison_options = ComparisonOptions
        {
            path:String::from("~/example_path.svg"),
            left_svg_data:cleaned_svg_data.clone(),
            right_svg_data:svg_data,
            pixel_tolerance:2u8,
            image_fuziness:8u8,
            image_size:64,
            save_error_image:false
        };

        // Compare the two svgs (at the old and new path)
        let _ = match compare_svgs(comparison_options) {
            Ok(_) => {
                let _ = fs::write(entry.path(), cleaned_svg_data);
                
                let ratio = 100.0 - (svg_data_count / cleaned_svg_data_count * 100.0);
                let message = format!("File is {:.2}% smaller now : {:?}", ratio, entry.file_name());
                println!("{}", message);
            },
            Err(_) => {
                let message = format!("File wasn't cleaned successfully : {:?}", entry.file_name());
                println!("{}", message);
            },
        };

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
