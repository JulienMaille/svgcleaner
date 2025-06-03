use regex::Regex;

use super::task::Task;

pub struct EmptyProperty;

impl Task for EmptyProperty {
  fn fix(&self, svg: &String) -> String {
    let re = Regex::new(r#"[:\w-]+="" *"#).unwrap();
    let cow = re.replace_all(&svg, "");
    return cow.into_owned();
  }
}

#[cfg(test)]
mod tests {
  use super::{EmptyProperty, Task};
  use pretty_assertions::{assert_eq};

  #[test]
  fn test_remove_no_end_space() {
    let svg = String::from(r#"<svg my-property="" property=""/>"#);
    
    let data = EmptyProperty {};
    let svg_out = data.fix(&svg);

    let correct = String::from(r#"<svg />"#);
    
    assert_eq!(correct, svg_out);
  }

  #[test]
  fn test_remove_colon_property() {
    let svg = String::from(r#"<rdf:RDF>
  <cc:Work
      rdf:about="">
    <dc:format>image/svg+xml</dc:format>
    <dc:type
        rdf:resource="http://purl.org/dc/dcmitype/StillImage" />
  </cc:Work>
</rdf:RDF>"#);
    
    let data = EmptyProperty {};
    let svg_out = data.fix(&svg);

    let correct = String::from(r#"<rdf:RDF>
  <cc:Work
      >
    <dc:format>image/svg+xml</dc:format>
    <dc:type
        rdf:resource="http://purl.org/dc/dcmitype/StillImage" />
  </cc:Work>
</rdf:RDF>"#);
    
    assert_eq!(correct, svg_out);
  }

}