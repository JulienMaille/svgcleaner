use regex::Regex;

use super::task::Task;

pub struct ExternalStyle;

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

    let correct = String::from(r#"<svg>
  <defs>
  </defs>
</svg>"#);
    
    assert_eq!(correct, svg_out);
  }

  #[test]
  fn full_svg() {
    let svg = String::from(r#"<svg fill="none" fill-rule="evenodd" stroke="black" stroke-linejoin="bevel" stroke-miterlimit="10" font-family="Times New Roman" font-size="16" style="font-variant-ligatures:none" xmlns:xlink="http://www.w3.org/1999/xlink" xmlns="http://www.w3.org/2000/svg" version="1.1" overflow="visible" width="36pt" height="36pt" viewBox="0 -36 36 36">
 <defs>
  <linearGradient id="LinearGradient" gradientUnits="userSpaceOnUse" x1="0" y1="0" x2="21.978" y2="0" gradientTransform="translate(28.498 4.296) rotate(111.454) skewX(-1.23916)">
   <stop offset="0" stop-color="/#f0f0f0"/>
   <stop offset="1" stop-color="/#ffffff"/>
  </linearGradient>
  <style type="text/css">@import url('https://themes.googleusercontent.com/fonts/css?family=Open Sans:400,600');</style>
 </defs>
 <g id="Layer 1" transform="scale(1 -1)" stroke-width="0.501">"#);
    
    let data = ExternalStyle {};
    let svg_out = data.fix(&svg);

    let correct = String::from(r#"<svg fill="none" fill-rule="evenodd" stroke="black" stroke-linejoin="bevel" stroke-miterlimit="10" font-family="Times New Roman" font-size="16" style="font-variant-ligatures:none" xmlns:xlink="http://www.w3.org/1999/xlink" xmlns="http://www.w3.org/2000/svg" version="1.1" overflow="visible" width="36pt" height="36pt" viewBox="0 -36 36 36">
 <defs>
  <linearGradient id="LinearGradient" gradientUnits="userSpaceOnUse" x1="0" y1="0" x2="21.978" y2="0" gradientTransform="translate(28.498 4.296) rotate(111.454) skewX(-1.23916)">
   <stop offset="0" stop-color="/#f0f0f0"/>
   <stop offset="1" stop-color="/#ffffff"/>
  </linearGradient>
 </defs>
 <g id="Layer 1" transform="scale(1 -1)" stroke-width="0.501">"#);
    
    assert_eq!(correct, svg_out);
  }

}