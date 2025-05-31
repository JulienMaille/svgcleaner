use regex::Regex;

use super::task::Task;

pub struct PointlessTransform {
}

/*
// TODO : Remove transforms like this
viewBox="0 -36 36 36"
transform="scale(1 -1)"
*/
impl Task for PointlessTransform {
  fn fix(&self, svg: &String) -> String {
    
    // REGEX-FIND : <svg [\w" -/#=:]+ viewBox="0 (-[\d]+)
    // REGEX-FIND : (<g [\w" -/#=:]+) transform="scale\(1 -1\)"

    return svg;
  }
}