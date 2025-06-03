use super::task::Task;

pub struct NanTransform;

impl Task for NanTransform {
  fn fix(&self, svg: &String) -> String {
    return svg.replace("scale(-nan(ind) 0)", "");
  }
} 

#[cfg(test)]
mod tests {
  use super::{NanTransform, Task};

  #[test]
  fn test_remove_no_end_space() {
    let svg = String::from(r#"<radialGradient id="RadialGradient" gradientUnits="userSpaceOnUse" cx="0" cy="0" r="0.605" gradientTransform="translate(17.189 20.246) rotate(180) skewX(90) scale(-nan(ind) 0)">"#);
    let data = NanTransform {};

    let svg_out = data.fix(&svg);

    let correct: String = String::from(r#"<radialGradient id="RadialGradient" gradientUnits="userSpaceOnUse" cx="0" cy="0" r="0.605" gradientTransform="translate(17.189 20.246) rotate(180) skewX(90) ">"#);
    
    assert_eq!(correct, svg_out);
  }

}