use clap::Parser;
use fixes::pointless_xmlns::PointlessXmlns;
use core::panic;
use std::{fs};

use crate::fixes::{
    task::Task,
    pointless_transform::PointlessTransform,
    empty_property::EmptyProperty,
    view_box_pt::ViewBoxPt,
    external_style::ExternalStyle,
    font_data::FontData,
    add_view_box::AddViewBox,
};

pub mod fixes;

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
    
    let mut data = match fs::read_to_string(path_in)
    {
        Ok(a) => a,
        Err(_) => {
            panic!("SVG IN does not exist!");
        }
    };

    let mut stack: Vec<Box<dyn Task>> = Vec::new();
    stack.push(Box::new(PointlessXmlns { }));
    stack.push(Box::new(EmptyProperty { }));
    stack.push(Box::new(PointlessTransform { }));
    stack.push(Box::new(AddViewBox { })); // <-- Best before ViewBoxPt
    stack.push(Box::new(ViewBoxPt { }));
    stack.push(Box::new(ExternalStyle { }));
    stack.push(Box::new(FontData { }));

    while let Some(top) = stack.pop() {
        data = top.fix(&data);
    };

    let _ = fs::write(path_out, data);
}
