use super::task::Task;

pub struct WidthAndHeight;

/*
// TODO : Replace Width/Height with 100%?
*/
impl Task for WidthAndHeight {
  fn fix(&self, svg: &String) -> String {

    return svg.to_string();
  }
}