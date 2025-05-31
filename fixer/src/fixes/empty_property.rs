use regex::Regex;

use super::task::Task;

pub struct EmptyProperty {
}

// Replace my-property="" with nothing
impl Task for EmptyProperty {
  fn fix(&self, svg: &String) -> String {
    let re = Regex::new(r#"[\w-]+="""#).unwrap();
    
    let cow = re.replace_all(&svg, "");
    return cow.into_owned();
  }
}