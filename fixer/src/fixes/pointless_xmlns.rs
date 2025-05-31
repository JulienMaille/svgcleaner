use regex::Regex;

use super::task::Task;

pub struct PointlessXmlns {
}

/*
xmlns="http://www.w3.org/2000/svg" <-- Leaves this alone
xmlns:xlink="http://www.w3.org/1999/xlink" 
xmlns:dc="http://purl.org/dc/elements/1.1/"
xmlns:cc="http://creativecommons.org/ns#"
xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
xmlns:svg="http://www.w3.org/2000/svg"
xmlns:sodipodi="http://sodipodi.sourceforge.net/DTD/sodipodi-0.dtd"
xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape"
sodipodi:docname="rectangle-figure-form-geometry-graphic-line-svgrepo-com.svg"
inkscape:version="1.0.2-2 (e86c870879, 2021-01-15)"
*/
impl Task for PointlessXmlns {
  fn fix(&self, svg: &String) -> String {
    
    let xmlns_re = Regex::new(r#"xmlns:[\w]+="[\w:/./#-]+""#).unwrap();
    let xmlns_cow = xmlns_re.replace_all(&svg, "");
    let xmlns_pass = xmlns_cow.into_owned();

    let sodipodi_re = Regex::new(r#"sodipodi:[\w]+="[\w:/./#-]+""#).unwrap();
    let sodipodi_cow = sodipodi_re.replace_all(&xmlns_pass, "");
    let sodipodi_pass = sodipodi_cow.into_owned();

    let inkscape_re = Regex::new(r#"inkscape:version="[\w.-]+[ \(\w,.\)\-]+""#).unwrap();
    let inkscape_cow = inkscape_re.replace_all(&sodipodi_pass, "");
    let inkscape_pass = inkscape_cow.into_owned();

    return inkscape_pass;
  }
}