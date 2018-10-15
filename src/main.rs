use std::io;
fn main() {
    println!("Guess Number!");

    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("fail to read");
    print!("your guess is {}", x)
}
