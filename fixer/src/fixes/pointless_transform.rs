use regex::Regex;

use super::task::Task;

pub struct PointlessTransform;

// TODO : Seems to cause a break!
impl Task for PointlessTransform {
  fn fix(&self, svg: &String) -> String {

    // REGEX-FIND : <svg [\w" -/#=:]+ viewBox="0 (-[\d]+)

    let viewbox_re = Regex::new(r#"<svg [\w" -/#=:]+ viewBox="0 (-[\d]+)"#).unwrap();

    let capture_offset: Vec<String> = viewbox_re.captures_iter(&svg).map(|caps| {
      let s = match caps.get(1) {
        None => {
          return svg.clone();
        },
        Some(s) => s.as_str().to_string(),
      };

      return s;
    }).collect();

    let num_capture = match capture_offset.first() {
      Some(a) => a,
      None => {
        return svg.clone();
      }
    };
    
    // REGEX-FIND : (<g [\w" -/#=:]+) transform="scale\(1 -1\)"
    
    let transform_re = Regex::new(r#"<g [\w" -/#=:]+ (transform="scale\(1 -1\)")"#).unwrap();

    let capture_transform: Vec<String> = transform_re.captures_iter(&svg).map(|caps| {
      let s = match caps.get(1) {
        None => {
          return svg.clone();
        },
        Some(s) => s.as_str().to_string(),
      };

      return s;
    }).collect();

    let transform_attribute = capture_transform.first().unwrap();

    let view_index = svg.find(num_capture).unwrap();
    let transform_index = svg.find(transform_attribute).unwrap();

    let svg_header_start = &svg[0..view_index];
    let svg_header_end = &svg[view_index + num_capture.len()..transform_index];
    let svg_transform_end = &svg[transform_index + transform_attribute.len()..];
    
    return format!("{}0{}{}", svg_header_start, svg_header_end, svg_transform_end);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_remove() {
    let svg = String::from(r#"<svg height="36pt" stroke-miterlimit="10" viewBox="0 -36 36 36" width="36pt">
	<g stroke-width=".501" transform="scale(1 -1)"></g>
</svg>"#);
    
    let data = PointlessTransform {};
    let svg_out = data.fix(&svg);

    let correct = String::from(r#"<svg height="36pt" stroke-miterlimit="10" viewBox="0 0 36 36" width="36pt">
	<g stroke-width=".501" ></g>
</svg>"#);
    
    assert_eq!(correct, svg_out);
  }

  #[test]
  fn no_view_box_data() {
    let svg = String::from(r#"<svg height="36pt" stroke-miterlimit="10" width="36pt">
	<g stroke-width=".501" transform="scale(1 -1)"></g>
</svg>"#);
    
    let data = PointlessTransform {};
    let svg_out = data.fix(&svg);

    let correct = String::from(r#"<svg height="36pt" stroke-miterlimit="10" width="36pt">
	<g stroke-width=".501" transform="scale(1 -1)"></g>
</svg>"#);
    
    assert_eq!(correct, svg_out);
  }

  #[test]
  fn view_box_zero() {
    let svg = String::from(r#"<svg height="36pt" stroke-miterlimit="10" viewBox="0 0 36 36" width="36pt">
	<g stroke-width=".501" transform="scale(1 -1)"></g>
</svg>"#);
    
    let data = PointlessTransform {};
    let svg_out = data.fix(&svg);

    let correct = String::from(r#"<svg height="36pt" stroke-miterlimit="10" viewBox="0 0 36 36" width="36pt">
	<g stroke-width=".501" transform="scale(1 -1)"></g>
</svg>"#);
    
    assert_eq!(correct, svg_out);
  }

}