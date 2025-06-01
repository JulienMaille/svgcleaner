use regex::Regex;

use super::task::Task;

pub struct PointlessXmlns {
}

impl Task for PointlessXmlns {
  fn fix(&self, svg: &String) -> String {
    
    let xmlns_re = Regex::new(r#"xmlns:[\w]+="[\w:/./#-]+" *\s*"#).unwrap();
    let xmlns_cow = xmlns_re.replace_all(&svg, "");
    let xmlns_pass = xmlns_cow.into_owned();

    let sodipodi_re = Regex::new(r#"sodipodi:[\w]+="[\w:/./#-]+" *\s*"#).unwrap();
    let sodipodi_cow = sodipodi_re.replace_all(&xmlns_pass, "");
    let sodipodi_pass = sodipodi_cow.into_owned();

    let inkscape_re = Regex::new(r#"inkscape:version="[\w.-]+[ \(\w,.\)\-]+" *\s*"#).unwrap();
    let inkscape_cow = inkscape_re.replace_all(&sodipodi_pass, "");
    let inkscape_pass = inkscape_cow.into_owned();

    return inkscape_pass;
  }
}

#[cfg(test)]
mod tests {

  use crate::PointlessXmlns;
  use crate::Task;
  use pretty_assertions::{assert_eq};

  #[test]
  fn test_remove_no_end_space() {
    let svg = String::from(r#"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:cc="http://creativecommons.org/ns#" xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#" xmlns:svg="http://www.w3.org/2000/svg" xmlns:sodipodi="http://sodipodi.sourceforge.net/DTD/sodipodi-0.dtd" xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape" sodipodi:docname="rectangle-figure-form-geometry-graphic-line-svgrepo-com.svg" inkscape:version="1.0.2-2 (e86c870879, 2021-01-15)"/>"#);
    
    let data = PointlessXmlns {};
    let svg_out = data.fix(&svg);

    let correct = String::from(r#"<svg xmlns="http://www.w3.org/2000/svg" />"#);
    
    assert_eq!(correct, svg_out);
  }

  #[test]
  fn full_svg() {
    let svg = include_str!("../../data/ellipse_old.svg").to_string();
    
    let data = PointlessXmlns {};
    let svg_out = data.fix(&svg);

    let correct = include_str!("../../data/ellipse_new.svg");
    
    assert_eq!(correct, svg_out);
  }

}