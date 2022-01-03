fn main() {
    println!("Hello, world!");
    println!("{:?}", sum(4.0, 5.0))
}

fn sum(x: f32, y: f32) -> f32 {
    return x + y;
}
