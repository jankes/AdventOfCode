
fn main() {
    // my input
    let mut a = 618u64;
    let mut b = 814u64;

    // example
    // let mut a = 65u64;
    // let mut b = 8921u64;

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
    println!("final count = {}", match_count);
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