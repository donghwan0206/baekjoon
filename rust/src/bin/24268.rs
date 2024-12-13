use std::io;
fn main() {
  let mut i = String::new();
  io::stdin()
    .read_line(&mut i)
    .unwrap();
  let i: usize = i.trim().parse().expect("error");
  println!("{}\n3",i*i*i);
}