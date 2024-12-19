use std::io;

fn main() {
  let mut i = String::new();
  let mut arr: Vec<usize> = Vec::new();

  
  
  for _ in 0..5 {
    io::stdin()
      .read_line(&mut i)
      .expect("input error");

    let k: usize = i.trim().parse().expect("NaN");
    arr.push(k);
    i.clear();
  }  
  arr.sort();
  let sum = arr.iter().fold(0, |acc, x| acc + x);
  let avg = sum / arr.len();
  println!("{}\n{}", avg,arr[arr.len()/2]);

}