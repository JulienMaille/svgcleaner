use regex::Regex;

use super::task::Task;

pub struct ViewBoxPt {
}

// Replace viewBox="0pt 0pt 0pt 0pt" with viewBox="0 0 0 0"
impl Task for ViewBoxPt {
  fn fix(&self, svg: &String) -> String {
    let re = Regex::new(r#"viewBox="([-\d.]+)[\w]* *([-\d.]+)[\w]* *([-\d.]+)[\w]* *([-\d.]+)[\w]* *""#).unwrap();
    let cow = re.replace_all(&svg, r#"viewBox="$1 $2 $3 $4""#);
    return cow.into_owned();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_remove_no_end_space() {
    let svg = String::from(r#"<svg viewBox="0pt -0pt 0px 0pt"/>"#);
    
    let data = ViewBoxPt {};
    let svg_out = data.fix(&svg);

    let svg_fixed = String::from(r#"<svg viewBox="0 -0 0 0"/>"#);
    
    assert_eq!(svg_fixed, svg_out);
  }

}