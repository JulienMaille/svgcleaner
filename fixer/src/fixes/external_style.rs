use regex::Regex;

use super::task::Task;

pub struct ExternalStyle {
}

// Delete <style type="text/css">@import url('https://themes.googleusercontent.com/fonts/css?family=Open Sans:400,600');</style>
impl Task for ExternalStyle {
  fn fix(&self, svg: &String) -> String {
    let re = Regex::new(r#"[\n ]*<style [\S ]+@import url[\S ]+/[style]*>"#).unwrap();
    let cow = re.replace_all(&svg, "");
    return cow.into_owned();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_remove_no_end_space() {
    let svg = String::from(r#"<svg>
  <defs>
    <style type="text/css">@import url('https://themes.googleusercontent.com/fonts/css?family=Open Sans:400,600');</style>
  </defs>
</svg>"#);
    
    let data = ExternalStyle {};
    let svg_out = data.fix(&svg);

    let svg_fixed = String::from(r#"<svg>
  <defs>
  </defs>
</svg>"#);
    
    assert_eq!(svg_fixed, svg_out);
  }

}