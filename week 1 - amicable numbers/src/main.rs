// https://projecteuler.net/problem=21

fn main() {
    println!("Hello, world!");

    let mut i: i32 = 0;
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    let mut sum: u32 = 0;

    findDivisors(220);
    findDivisors(284);

    // for i in 2..10000 {
    //     (x, y) = getAmicablePair(i);
    //     if x == 0 {
    //         break;
    //     }
    //     // TODO store x,y as amicables in hash map (to skip second check)
    //     sum += x + y;
    //     println!("Amicable: {} and {}", x, y);
    // }

    println!("{:?}", sum)
}

fn findDivisors(x: u32) -> std::vec::Vec<u32> {
    let mut divisors: Vec<u32> = vec![1];
    let max = ((x as f32).sqrt().ceil() as u32) + 1;
    for i in 2..max {
        if x % i == 0 {
            divisors.push(i);
            // Only take square root of x as factor once
            if i * i != x {
                divisors.push(x / i)
            }
        }
    }

    divisors.sort();

    println!("{:?}", divisors);

    return divisors;
}

// If x is not amicable number, return (0,0)
fn getAmicablePair(x: u32) -> (u32, u32) {
    let x_factors: Vec<u32> = findDivisors(x);

    // TODO sum factors to get y
    let y: u32 = 0; // TODO change to sum

    let y_factors: Vec<u32> = findDivisors(y);

    let y_factor_sum = 0; // TODO sum

    if (y_factor_sum != x) {
        return (u32::MIN, u32::MIN);
    }

    return (x, y);
}
