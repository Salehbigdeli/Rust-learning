extern crate num;
use num::complex;

struct Complex<T>{
    /// Real portion of complex number
    re: T,
    /// Imaginary portion of the complex number
    im: T
}

#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>){
    let mut z = Complex{re: 0.0, im: 0.0};
    loop {
        z = z * z + c;
    }
}
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
