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

// If x is not amicable number, return (0,0)
fn getAmicablePair(x: u32) -> (u32, u32) {
    let x_factors: Vec<u32> = findDivisors(x);

    // TODO sum factors to get y
    let y: u32 = 0; // TODO change to sum

    let y_factors: Vec<u32> = findDivisors(y);

    let y_factor_sum = 0; // TODO sum

    if (y_factor_sum != x) {
        return (0, 0);
    }

    return (x, y);
}
