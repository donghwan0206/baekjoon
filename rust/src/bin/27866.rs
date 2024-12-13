use std::io;

fn main() {
    let mut text = String::new();
    let mut i = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("stdin error");
    io::stdin()
        .read_line(&mut i)
        .expect("stdin error");
    
    let ix: usize = i.trim().parse().expect("Nan");

    match text.chars().nth(ix-1) {
        Some(ch) => println!("{ch}"),
        None => println!("index over")
    }
}