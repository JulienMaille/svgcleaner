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
        panic!("Svgs are NOT equal ~ {} : {} != {}", count, left_svg_data, right_svg_data);
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

  use crate::compare_svgs;

  #[test]
  fn compare_simple_svg() {

    let original = include_str!("../data/original_add.svg").to_string();
    let cleaned = include_str!("../data/cleaned_add.svg").to_string();

    compare_svgs(&original, &cleaned);
  }

}