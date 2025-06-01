use regex::Regex;

use super::task::Task;

pub struct FontData {
}

// Delete <style type="text/css">@import url('https://themes.googleusercontent.com/fonts/css?family=Open Sans:400,600');</style>
impl Task for FontData {
  fn fix(&self, svg: &String) -> String {
    let re = Regex::new(r#"style="font-variant-ligatures:none" *"#).unwrap();
    let cow = re.replace_all(&svg, "");
    return cow.into_owned();
  }
} 

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_remove_no_end_space() {
    let svg = String::from(r#"<svg style="font-variant-ligatures:none"/>"#);
    let data = FontData {};

    let svg_out = data.fix(&svg);
    
    assert_eq!("<svg />", svg_out);
  }

  #[test]
  fn test_remove_end_space() {
    let svg = String::from(r#"<svg style="font-variant-ligatures:none" />"#);
    let data = FontData {};

    let svg_out = data.fix(&svg);
    
    assert_eq!("<svg />", svg_out);
  }

}