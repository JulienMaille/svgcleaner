use regex::Regex;

use super::task::Task;

pub struct ExternalStyle {
}

// Delete <style type="text/css">@import url('https://themes.googleusercontent.com/fonts/css?family=Open Sans:400,600');</style>
impl Task for ExternalStyle {
  fn fix(&self, svg: &String) -> String {
    let re = Regex::new(r#"[\n ]*<style [\S ]+@import url[\S ]+/[style]*>"#).unwrap();
    let cow = re.replace_all(&svg, "");
    return cow.into_owned();
  }
}