use regex::Regex;

use super::task::Task;

pub struct EmptyProperty {
}

impl Task for EmptyProperty {
  fn fix(&self, svg: &String) -> String {
    let re = Regex::new(r#"[\w-]+="" *"#).unwrap();
    let cow = re.replace_all(&svg, "");
    return cow.into_owned();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_remove_no_end_space() {
    let svg = String::from(r#"<svg my-property="" property=""/>"#);
    
    let data = EmptyProperty {};
    let svg_out = data.fix(&svg);

    let correct = String::from(r#"<svg />"#);
    
    assert_eq!(correct, svg_out);
  }

}