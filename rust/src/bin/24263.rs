use std::io;
fn main() {
  let mut i = String::new();
  io::stdin()
    .read_line(&mut i)
    .unwrap();
  println!("{}\n1",i.trim());
}