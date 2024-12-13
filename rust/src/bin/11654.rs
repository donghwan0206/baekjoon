use std::io;

fn main() {
  let mut c = String::new();
  io::stdin().read_line(&mut c).expect("input error");
  println!("{}", c.bytes().nth(0).unwrap_or_default());
}