fn square_loop(mut x: f64){
    loop {
        x = x * x;
    }
}

fn square_add_loop(c: f64){
    let mut x: f64 = 0.;
    loop {
        x = x * x + c;
    }
}

fn main() {
    println!("Hello, world!");
}
