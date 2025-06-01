use resvg::{
        tiny_skia::{
            Pixmap, PixmapMut
        },
        usvg::{
        Transform, Tree
    }
};

pub fn compare_svgs(left_svg_data: &String, right_svg_data: &String) {

    let mut left_pixmap = Pixmap::new(64,64).unwrap();
    let mut right_pixmap = Pixmap::new(64,64).unwrap();

    get_pixmap(&left_svg_data, &mut left_pixmap.as_mut());
    get_pixmap(&right_svg_data, &mut right_pixmap.as_mut());

    let left_pixels = left_pixmap.pixels_mut();
    let right_pixels = right_pixmap.pixels_mut();

    let mut count = 0;
    for n in 0..left_pixels.len() {
        let left_pixel = left_pixels[n];
        let right_pixel = right_pixels[n];

        if !left_pixel.eq(&right_pixel) {
            count += 1;
        }
    }

    if count > 0 {
        panic!("Svgs are NOT equal ~ {}/4096", count);
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