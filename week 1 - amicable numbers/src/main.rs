// https://projecteuler.net/problem=21

use std::collections::HashMap;

fn main() {
    // TODO change to 10000
    const MAX_NUM: u32 = 300;

    let mut numbersAdded = HashMap::new();
    numbersAdded.insert(0, false);

    let mut i: i32 = 0;
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    let mut sum: u32 = 0;
    let mut res: (u32, u32);

    // res = getAmicablePair(220);
    // println!("{:?}", res);
    // println!("{:?}", res.0 + res.1);

    for i in 2..MAX_NUM + 1 {
        // if (numbersAdded.get(i)) {
        //     continue;
        // }
        res = getAmicablePair(i);
        x = res.0;
        if x == 0 {
            continue;
        }
        // TODO store x,y as amicables in hash map (to skip second check)
        // sum += x + y; //TODO use this when hashmaps working
        sum += x;
        // println!("Amicable: {} and {}", x, y);
        println!("Amicable: {}", x);
    }

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
    divisors.dedup();

    return divisors;
}

// If x is not amicable number, return (0,0)
fn getAmicablePair(x: u32) -> (u32, u32) {
    let x_factors: Vec<u32> = findDivisors(x);
    if x_factors.len() == 1 {
        return (0, 0);
    }

    let y: u32 = x_factors.iter().sum();
    let y_factors: Vec<u32> = findDivisors(y);
    let y_factor_sum: u32 = y_factors.iter().sum();

    if (y_factor_sum != x || x == y) {
        return (0, 0);
    }

    return (x, y);
}
