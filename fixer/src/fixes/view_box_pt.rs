use regex::Regex;

use super::task::Task;

pub struct ViewBoxPt {
}

// Replace my-property="" with nothing
impl Task for ViewBoxPt {
  fn fix(&self, svg: &String) -> String {
    let re = Regex::new(r#"viewBox="([\d.]+)[\w]* *([\d.]+)[\w]* *([\d.]+)[\w]* *([\d.]+)[\w]* *""#).unwrap();
    let cow = re.replace_all(&svg, r#"viewBox="$1 $2 $3 $4""#);
    return cow.into_owned();
  }
}