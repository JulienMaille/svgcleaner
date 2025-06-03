use std::io::{Error, ErrorKind};

use resvg::{
        tiny_skia::{
            Pixmap, PixmapMut, PremultipliedColorU8
        },
        usvg::{
        Transform, Tree
    }
};

pub struct ComparisonOptions {
  // Path for Debug Files
  pub path: String,

  // Left Data
  pub left_svg_data:String,
  
  // Right Data
  pub right_svg_data:String,
  
  // 0 would require exact pixels, 1 would mean 254 and 255 are equal
  pub pixel_tolerance:u8,

  // As a percent, 0 - 100
  pub image_fuziness:u8,

  // Width / Height, smaller is faster
  pub image_size:u8,

  // Will save an image showing errors with _error.svg as the suffix
  pub save_error_image:bool
}

pub fn compare_svgs(options: ComparisonOptions) -> Result<u8, Error> {

    let size = options.image_size as u32;
    let pixel_count = size * size;

    let mut left_pixmap = Pixmap::new(size,size).unwrap();
    let mut right_pixmap = Pixmap::new(size,size).unwrap();

    get_pixmap(&options.left_svg_data, &mut left_pixmap.as_mut());
    get_pixmap(&options.right_svg_data, &mut right_pixmap.as_mut());

    let left_pixels = left_pixmap.pixels_mut();
    let right_pixels = right_pixmap.pixels_mut();

    let mut a = 0;
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;

    let mut comparison_pixmap = Pixmap::new(size,size).unwrap();
    let comp_pixels = comparison_pixmap.pixels_mut();

    let tol = options.pixel_tolerance;
    for n in 0..left_pixels.len() {
        let left_pixel = left_pixels[n];
        let right_pixel = right_pixels[n];

        // RGB comparison unecessary if transparent(ish)
        if left_pixel.alpha() < tol && right_pixel.alpha() < tol {
          continue;
        }

        let a_diff = left_pixel.alpha().abs_diff(right_pixel.alpha());
        let r_diff = left_pixel.red().abs_diff(right_pixel.red());
        let g_diff = left_pixel.green().abs_diff(right_pixel.green());
        let ab_diff = left_pixel.blue().abs_diff(right_pixel.blue());

        let mut different = false;

        // RGB comparison unecessary if transparent(ish)
        if a_diff > tol {
          a +=1;
          different = true;
        }

        if r_diff > tol {
          r +=1;
          different = true;
        }

        if g_diff > tol {
          g +=1;
          different = true;
        }

        if ab_diff > tol {
          b +=1;
          different = true;
        }

        if different {
          comp_pixels[n] = PremultipliedColorU8::from_rgba(255, 0, 0, 255).unwrap();
        }
        else
        {
          comp_pixels[n] = left_pixel;
        }
        
    }

    let diff = (a + r + g + b) as f32;
    let diff_percent = diff/(pixel_count as f32) * 100f32;
    if diff >= 1f32 {
      
      let new_path = options.path.replace(".svg", "_error.png");

      if options.save_error_image {
        let _ = comparison_pixmap.save_png(new_path.clone());
      }
      
      if diff_percent > (options.image_fuziness as f32)
      {
        let error = format!("Svgs are NOT equal ~ a:{}/{5}, r:{}/{5}, g:{}/{5}, b:{}/{5} -> {}", a,r,g,b, new_path, pixel_count);
        return Result::Err(Error::new(ErrorKind::InvalidData, error));
      }
    }

    return Result::Ok(0);

}

fn get_pixmap(svg_data:&String, pixmap:&mut PixmapMut) {

    let options = resvg::usvg::Options::default();
    let tree = match Tree::from_str(&svg_data, &options) {
      Ok(a) => a,
      Err(e) => {
        panic!("{}", e);
      } 
    };
    
    let transform = Transform::from_scale(1f32, 1f32);

    resvg::render(&tree, transform, pixmap);
}

#[cfg(test)]
mod tests {
  use test_case::test_case;
  use super::{compare_svgs,ComparisonOptions};

  #[test_case(include_str!("../data/back_new.svg"), include_str!("../data/back_old.svg"))]
  #[test_case(include_str!("../data/KeyboardShortcuts_darkmode_new.svg"), include_str!("../data/KeyboardShortcuts_darkmode_old.svg"))]
  #[test_case(include_str!("../data/KeyboardShortcuts_new.svg"), include_str!("../data/KeyboardShortcuts_old.svg"))]
  #[test_case(include_str!("../data/LayoutMenu_new.svg"), include_str!("../data/LayoutMenu_old.svg"))]
  #[test_case(include_str!("../data/link_darkmode_new.svg"), include_str!("../data/link_darkmode_old.svg"))]
  #[test_case(include_str!("../data/link_new.svg"), include_str!("../data/link_old.svg"))]
  #[test_case(include_str!("../data/MacStatusBarHistory_new.svg"), include_str!("../data/MacStatusBarHistory_old.svg"))]
  #[test_case(include_str!("../data/NewDetailOff_darkmode_new.svg"), include_str!("../data/NewDetailOff_darkmode_old.svg"))]
  #[test_case(include_str!("../data/NewDetailOff_new.svg"), include_str!("../data/NewDetailOff_old.svg"))]
  #[test_case(include_str!("../data/NewDetailOn_darkmode_new.svg"), include_str!("../data/NewDetailOn_darkmode_old.svg"))]
  #[test_case(include_str!("../data/NewDetailOn_new.svg"), include_str!("../data/NewDetailOn_old.svg"))]
  #[test_case(include_str!("../data/original_add_new.svg"), include_str!("../data/original_add_old.svg"))]
  #[test_case(include_str!("../data/Properties_new.svg"), include_str!("../data/Properties_old.svg"))]
  #[test_case(include_str!("../data/Save_darkmode_new.svg"), include_str!("../data/Save_darkmode_old.svg"))]
  #[test_case(include_str!("../data/SVG_Editor_EyeDropper_darkmode_new.svg"), include_str!("../data/SVG_Editor_EyeDropper_darkmode_old.svg"))]
  #[test_case(include_str!("../data/SVG_Editor_Pencil_darkmode_new.svg"), include_str!("../data/SVG_Editor_Pencil_darkmode_old.svg"))]
  #[test_case(include_str!("../data/SVG_Editor_Pencil_new.svg"), include_str!("../data/SVG_Editor_Pencil_old.svg"))]
  fn compare_simple_svg(a: &str, b: &str) {
    let example_path = String::from("~/example/path.svg");

    let options = ComparisonOptions
    {
      path:example_path,
      left_svg_data:a.to_string(),
      right_svg_data:b.to_string(),
      pixel_tolerance:2u8,
      image_fuziness:8u8,
      image_size:64,
      save_error_image:false
    };

    let _ = match compare_svgs(options) {
      Ok(_) => assert!(true),
      Err(e) => panic!("{}", e),
    };
  }

  #[test_case(include_str!("../data/SVG_Editor_Pencil_darkmode_new.svg"), include_str!("../data/SVG_Editor_Pencil_old.svg"))]
  #[test_case(include_str!("../data/SVG_Editor_Pencil_new.svg"), include_str!("../data/SVG_Editor_Pencil_darkmode_old.svg"))]
  fn compare_failing_svg(a: &str, b: &str) {
    let example_path = String::from("~/example/path.svg");

    let options = ComparisonOptions
    {
      path:example_path,
      left_svg_data:a.to_string(),
      right_svg_data:b.to_string(),
      pixel_tolerance:10u8,
      image_fuziness:8u8,
      image_size:64,
      save_error_image:false
    };

    let _ = match compare_svgs(options) {
      Err(_) => assert!(true),
      Ok(__) => panic!("Two different svgs were seen as equal, test has failed!"),
    };
  }

}