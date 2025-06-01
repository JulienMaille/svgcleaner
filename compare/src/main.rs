use clap::Parser;
use std::{fs};

pub mod comparison;

use comparison::compare_svgs;

// https://docs.rs/clap/latest/clap/

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
  
  #[arg()]
  left_svg: String,

  #[arg()]
  right_svg: String,
}

fn main() {
    let args = Args::parse();

    let path_left = match std::path::absolute(args.left_svg.clone())
    {
        Ok(a) => a,
        Err(_) => {
            panic!("Left Svg path is invalid!");
        }
    };

    let path_right= match std::path::absolute(args.right_svg.clone())
    {
        Ok(a) => a,
        Err(_) => {
            panic!("Right Svg path is invalid!");
        }
    };
    
    let left_svg_data = match fs::read_to_string(path_left)
    {
        Ok(a) => a,
        Err(_) => {
            panic!("Left Svg data is invalid!");
        }
    };
    
    let right_svg_data = match fs::read_to_string(path_right)
    {
        Ok(a) => a,
        Err(_) => {
            panic!("Right Svg data is invalid!");
        }
    };

    compare_svgs(&args.left_svg.clone(), &left_svg_data, &right_svg_data);
}
