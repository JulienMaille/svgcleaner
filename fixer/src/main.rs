use clap::Parser;
use core::panic;
use std::{fs};
use svgcleaner::fixlib::fix_svg;

// https://docs.rs/clap/latest/clap/

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
  
  #[arg()]
  svg_in: String,

  #[arg()]
  svg_out: String,
}

fn main() {
    let args = Args::parse();

    let path_in = match std::path::absolute(args.svg_in)
    {
        Ok(a) => a,
        Err(_) => {
            panic!("SVG IN path is invalid!");
        }
    };

    let path_out = match std::path::absolute(args.svg_out)
    {
        Ok(a) => a,
        Err(_) => {
            panic!("SVG OUT path is invalid!");
        }
    };
    
    let data = match fs::read_to_string(path_in)
    {
        Ok(a) => a,
        Err(_) => {
            panic!("SVG IN does not exist!");
        }
    };

    let result = fix_svg(data);

    let _ = fs::write(path_out, result);
}
