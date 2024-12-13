use std::{collections::HashSet, io};

fn main() {
  let mut i = String::new();
  io::stdin()
    .read_line(&mut i)
    .expect("input error");
  
  let nm: Vec<usize> = i.trim().split_whitespace()
                        .map (|x| x.parse::<usize>().expect("NaN"))
                        .collect();
  if nm.len() != 2 {
    panic!("Expected 2 numbers")
  }
  let mut n = nm[0];
  let mut m = nm[1];

  // print!("n: {n}, m: {m}");

  let mut db: HashSet<String> = HashSet::new();
  let mut bb: HashSet<String> = HashSet::new();
  while n > 0 {
    i.clear();
    io::stdin()
      .read_line(&mut i)
      .expect("read error");
    db.insert(i.clone());
    n-=1;
  }
  while m > 0 {
    i.clear();
    io::stdin()
      .read_line(&mut i)
      .expect("read error");
    bb.insert(i.clone());
    m-=1;
  }

  let mut inter: Vec<String> = db.intersection(&bb).cloned().collect();
  inter.sort();
  print!("{}\n", inter.len());
  print!("{}", inter.join(""));

}