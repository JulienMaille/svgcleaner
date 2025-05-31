use regex::Regex;

use super::task::Task;

pub struct ViewBoxPt {
}

// Replace viewBox="0pt 0pt 0pt 0pt" with viewBox="0 0 0 0"
impl Task for ViewBoxPt {
  fn fix(&self, svg: &String) -> String {
    let re = Regex::new(r#"viewBox="([\d.]+)[\w]* *([\d.]+)[\w]* *([\d.]+)[\w]* *([\d.]+)[\w]* *""#).unwrap();
    let cow = re.replace_all(&svg, r#"viewBox="$1 $2 $3 $4""#);
    return cow.into_owned();
  }
}