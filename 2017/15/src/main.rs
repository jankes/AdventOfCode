// Example
// const A_INIT: u64 = 65;
// const B_INIT: u64 = 8921;

// My Input
const A_INIT: u64 = 618;
const B_INIT: u64 = 814;

fn main() {
    part_1();
    part_2();
}

fn part_2() {
    let mut a = A_INIT;
    let mut b = B_INIT;
    let mut match_count = 0;
    for _ in 0..5000000 {
        a = next_a(a);
        b = next_b(b);
        if (a & 0xFFFF) == (b & 0xFFFF) {
            match_count += 1;
        }
    }
    println!("part 2: match_count = {}", match_count);

    fn next_a(current: u64) -> u64 {
        next(current, 16807, 0b11)
    }

    fn next_b(current: u64) -> u64 {
        next(current, 48271, 0b111)
    }

    fn next(current: u64, factor: u64, criteria: u64) -> u64 {
        let mut value = mul(current, factor);
        while (value & criteria) != 0 {
            value = mul(value, factor);
        }
        value
    }
}

fn part_1() {
    let mut a = A_INIT;
    let mut b = B_INIT;
    let mut match_count = 0u32;
    for _ in 0..40000000 {
        // a *= 16807;
        // a = a % 2147483647;
        a = mul(a, 16807);

        // b *= 48271;
        // b = b % 2147483647;
        b = mul(b, 48271);

        if (a & 0xFFFF) == (b & 0xFFFF) {
            match_count += 1;
        }
    }
    println!("part 1: final count = {}", match_count);
}

// https://en.wikipedia.org/wiki/Modular_arithmetic
// ported from listed C function example implementation of modular multiplication
fn mul(mut a: u64, mut b: u64) -> u64 {
    let m = 2147483647u64;
    if a >= m {
        a %= m;
    }
    if b >= m {
        b %= m;
    }
    let x = a as f64;
    let c = (x * (b as f64) / (m as f64)) as u64;
    let r = ((a * b - c * m) as i64) % (m as i64);
    if r < 0 {
        (r + (m as i64)) as u64
    } else {
        r as u64
    }
}