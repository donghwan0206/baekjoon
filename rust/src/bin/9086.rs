use std::io;

fn main() {
  let mut i = String::new();
  
  io::stdin()
    .read_line(&mut i)
    .expect("error");

  let mut i: usize = i.trim().parse().expect("Nan");

  let mut s = String::new();
  while i > 0 {
    s.clear();
    io::stdin()
      .read_line(&mut s)
      .expect("error");
    
    println!("{}{}",s.trim().chars().nth(0).unwrap_or_default(),s.trim().chars().last().unwrap_or_default() );
    i -=1;
  }
}
