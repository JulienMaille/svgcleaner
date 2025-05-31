use regex::Regex;

use super::task::Task;

pub struct AddViewBox {
}

impl Task for AddViewBox {
  fn fix(&self, svg: &String) -> String {

    // Split by >
    // Check the first line that starts with <svg
    // Does viewBox exist?
    // Yes -> Return
    
    // No ->
      // Does width + height exist?
      // Extract values
      // Remove width/height
      // Add viewBox

    return svg.to_string();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_remove_no_end_space() {
    let svg = String::from(r#"<svg width="100px" height="200px"/>"#);
    
    let data = AddViewBox {};
    let svg_out = data.fix(&svg);

    let svg_fixed = String::from(r#"<svg viewBox="0 0 100 200"/>"#);
    
    assert_eq!(svg_fixed, svg_out);
  }

}