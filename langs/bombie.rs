fn bombie(b: &mut String) -> &mut String {
  b.push('.');
  b
}

fn main() {
  let mut _bombie = "bombie".to_string();
  println!("{}", bombie(&mut _bombie));
}
