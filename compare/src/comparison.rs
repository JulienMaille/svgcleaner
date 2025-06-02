use resvg::{
        tiny_skia::{
            Pixmap, PixmapMut, PremultipliedColorU8
        },
        usvg::{
        Transform, Tree
    }
};

pub fn compare_svgs(path: &String, left_svg_data: &String, right_svg_data: &String) {

    let mut left_pixmap = Pixmap::new(64,64).unwrap();
    let mut right_pixmap = Pixmap::new(64,64).unwrap();

    get_pixmap(&left_svg_data, &mut left_pixmap.as_mut());
    get_pixmap(&right_svg_data, &mut right_pixmap.as_mut());

    let left_pixels = left_pixmap.pixels_mut();
    let right_pixels = right_pixmap.pixels_mut();

    let mut a = 0;
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;

    let mut comparison_pixmap = Pixmap::new(64,64).unwrap();
    let comp_pixels = comparison_pixmap.pixels_mut();

    let tol = 2u8;
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

    if a > 0 || r > 0 || g > 0 || b > 0 {

        let new_path = path.replace(".svg", "_error.png");
        let _ = comparison_pixmap.save_png(new_path.clone());

        panic!("Svgs are NOT equal ~ a:{}/4096, r:{}/4096, g:{}/4096, b:{}/4096 -> {}", a,r,g,b, new_path);
    }

    std::process::exit(0);
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
  use crate::compare_svgs;

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
  fn compare_simple_svg_asdsd(a: &str, b: &str) {
    compare_svgs(&a.to_string(), &b.to_string());
  }

}