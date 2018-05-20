use std::borrow::BorrowMut;
use std::fs;
use std::str::FromStr;

fn main() {
    let instructions =
    fs::read_to_string("C:\\Users\\jankes\\Documents\\AdventOfCode\\2017\\5\\instructions.txt")
    .expect("couldn't read instructions file")
    .lines()
    .map(|instruction_str| i16::from_str(instruction_str).expect("instructions must be valid 16 bit integers"))
    .collect::<Vec<i16>>();

    if instructions.len() > i16::max_value() as usize {
        println!("too many instructions");
        return;
    }

    let count = run_part_1(instructions.clone().borrow_mut());
    println!("part 1: program finishes after {} instructions", count);

    let count = run_part_2(instructions.clone().borrow_mut());
    println!("part 2: program finishes after {} instructions", count);
}

fn run_part_1(instructions: &mut [i16]) -> u32 {
    run(instructions, |previous, instructions| instructions[previous as usize] += 1)
}

fn run_part_2(instructions: &mut [i16]) -> u32 {
    run(instructions, |previous, instructions| {
        if instructions[previous as usize] >= 3 {
            instructions[previous as usize] -= 1;
        } else {
            instructions[previous as usize] += 1;
        }        
    })
}

fn run<F: FnMut(i16, &mut [i16])>(instructions: &mut [i16], mut update: F) -> u32 {
    let mut count = 0u32;
    let mut current = 0i16;
    while 0 <= current && current < instructions.len() as i16 {
        let previous = current;
        current += instructions[current as usize];
        count += 1;
        update(previous, instructions);
    }
    return count;
}