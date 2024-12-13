use std::io;

fn main() {
  let mut i = String::new();
  let s = String::from("abcdefghijklmnopqrstuvwxyz");

  io::stdin()
    .read_line(&mut i)
    .expect("input error");
  

  let positions: Vec<i32> = s.chars()
                .map(
                  |x| {
                    i.chars()
                    .enumerate()
                    .fold( -1, |acc, (index, c)| {
                        if acc == -1 && c == x {
                          index as i32
                        } else {
                          acc
                        }
                      }
                    )  
                  }
                ).collect();
  
  print!("{}", positions.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));

}