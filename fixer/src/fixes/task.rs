pub trait Task {
  fn fix(&self, svg: &String) -> String;
}