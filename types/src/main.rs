fn main() {
    let x = std::i64::MAX;
    let y = x.wrapping_add(1);
    println!("Hello {} {}", (-4i32).count_ones(), y);
}
