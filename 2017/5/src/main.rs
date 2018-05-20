use std::fs;
use std::str::FromStr;

fn main() {
    let mut instructions =
    fs::read_to_string("C:\\Users\\jankes\\Documents\\AdventOfCode\\2017\\5\\instructions.txt")
    .expect("couldn't read instructions file")
    .lines()
    .map(|instruction_str| i16::from_str(instruction_str).expect("instructions must be valid 16 bit integers"))
    .collect::<Vec<i16>>();

    if instructions.len() > i16::max_value() as usize {
        println!("too many instructions");
        return;
    }

    let mut count = 0;
    let mut current = 0i16;
    while 0 <= current && current < instructions.len() as i16 {
        let previous = current;
        current += instructions[current as usize];
        count += 1;
        instructions[previous as usize] += 1;
    }
    println!("program finishes after {} instructions", count);
}
