use std::ops::Index;

use regex::Regex;

use super::task::Task;

pub struct AddViewBox {
}

impl Task for AddViewBox {
  fn fix(&self, svg: &String) -> String {

    let str = svg.to_string();
    let parts = str.split(">");

    for part in parts {
      if !part.starts_with("<svg") {
        continue;
      }

      if part.contains(r#"viewBox=""#) {
        return str.to_string();
      }

      let mut svg_part: String = part.to_string();

      let re_width = Regex::new(r#"width="([\w\-. ]+)""#);
      let re_height = Regex::new(r#"height="([\w\-. ]+)""#);

      let match_width = match re_width {
        Ok(a) => a,
        Err(_) => {
          return str;
        }
      };

      let match_height = match re_height {
        Ok(a) => a,
        Err(_) => {
          return str;
        }
      };
      
      // Extract Values
      let capture_widths: Vec<String> = match_width.captures_iter(&svg_part).map(|caps| {
        let s = match caps.get(1) {
          None => {
            return str.clone();
          },
          Some(s) => s.as_str().to_string(),
        };

        return s;
      }).collect();
      let width = capture_widths.first().unwrap();

      let capture_heights: Vec<String> = match_height.captures_iter(&svg_part).map(|caps| {
        let s = match caps.get(1) {
          None => {
            return str.clone();
          },
          Some(s) => s.as_str().to_string(),
        };

        return s;
      }).collect();
      let  height = capture_heights.first().unwrap();

      // Remove width/height
      let cow_width = match_width.replace_all(&svg_part, "");
      let cow_width_str = cow_width.into_owned().to_string();
      let cow_height = match_height.replace_all(&cow_width_str, "");
      svg_part = cow_height.into_owned();

      // Add viewBox
      let index = svg_part.find("<svg ").unwrap();
      let view_box_str = format!(r#"viewBox="0 0 {} {}""#, width, height);
      svg_part.insert_str(index + 5, &view_box_str);

      let split = str.split_at(index);
      let mid_and_end = split.1;
      
      let start = split.0;
      let end = mid_and_end.split_at(part.len()).1.to_string();

      let mid = svg_part;

      let new_svg = format!("{}{}{}", start, mid, end);
      // panic!("{}{}{}", start, mid, end);
      return new_svg;
    }
    
    return svg.to_string();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add_view_box() {
    let svg = String::from(r#"<svg width="100px" height="200px"/>"#);
    
    let data = AddViewBox {};
    let svg_out = data.fix(&svg);

    let svg_fixed = String::from(r#"<svg viewBox="0 0 100px 200px" />"#);
    
    assert_eq!(svg_fixed, svg_out);
  }

}