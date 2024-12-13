use std::io;

fn main() {
  let mut i = String::new();
  let mut s = String::new();
  io::stdin()
    .read_line(&mut i)
    .expect("input error");
  
  io::stdin()
    .read_line(&mut s)
    .expect("input error");
  
  let i: usize = i.trim().parse().expect("type cast error");

  
  print!("{}", 
              s.trim()
                .chars()
                .take(i)
                .fold(0, |acc, x| acc + x.to_digit(10).unwrap_or_default())
  );

}