// https://projecteuler.net/problem=21

fn main() {
    println!("Hello, world!");
    println!("{:?}", sum(4.0, 5.0))
}

fn sum(x: f32, y: f32) -> f32 {
    return x + y;
}

fn findDivisors(x: u32) -> std::vec::Vec<u32> {
    let mut i = 0;
    while i < (x.sqrt() + 1) {}
}
