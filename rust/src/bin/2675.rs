use std::io;

fn main() {
  let mut i = String::new();
  io::stdin()
    .read_line(&mut i)
    .expect("input error");
  
  let mut i: usize = i.trim().parse().expect("NaN");

  let mut line = String::new();
  let mut split: Vec<String> = Vec::new();

  while i > 0 {
    line.clear();
    io::stdin()
      .read_line(&mut line)
      .expect("read error");
    split.push(line.trim().to_string());
    i-=1;
  }
  
  for l in split {
    let parts: Vec<&str> = l.split_whitespace().collect();
    let n: usize = parts[0].parse().expect("NaN");
    let t = parts[1];
    for c in t.chars() {
      print!("{}", c.to_string().repeat(n));
    }
    println!("")
  }  
}