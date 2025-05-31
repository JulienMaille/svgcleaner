use regex::Regex;

use super::task::Task;

pub struct WidthAndHeight {
}

/*
// TODO : Remove Width/Height? Leave to 100%
*/
impl Task for WidthAndHeight {
  fn fix(&self, svg: &String) -> String {

    return svg;
  }
}