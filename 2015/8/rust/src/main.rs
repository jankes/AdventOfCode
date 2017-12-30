use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

fn main() {
    let input = read_input("C:\\Users\\sjank\\Documents\\Projects\\AdventOfCode\\2015\\8\\input.txt");

    let total_chars = input.lines()
                                .fold(0i32, |total, line| total + line.as_bytes().len() as i32);

    let in_memory_chars = input.lines()
                               .map(count_in_memory_chars)
                               .fold(0i32, |total, count| total + count);
    
    println!("total chars = {}", total_chars);
    println!("in memory chars = {}", in_memory_chars);
    println!("difference = {}", total_chars - in_memory_chars);
}

enum State {
    MemoryChar, Esc, Hex1, Hex2
}

fn count_in_memory_chars(s: &str) -> i32 {
    let mut count = 0i32;
    let mut state = State::MemoryChar;
    let bytes = s.as_bytes();
    for b in bytes[1..s.len() - 1].iter() {
        match state {
            State::MemoryChar => {
                count += 1;
                if *b == b'\\' {
                    state = State::Esc;
                }
            },
            State::Esc => {
                match *b {
                    b'\\' | b'\"' => state = State::MemoryChar,
                    b'x' => state = State::Hex1,
                    _ => panic!("unknown escape")
                }
            },
            State::Hex1 => {
                match *b {
                    b'0'...b'9' | b'a'...b'z' => state = State::Hex2,
                    _ => panic!("unknown hex 1")
                }
            },
            State::Hex2 => {
                match *b {
                    b'0'...b'9' | b'a'...b'z' => state = State::MemoryChar,
                    _ => panic!("unknown hex 1")
                }
            }
        }
    }
    count
}

fn read_input<P: AsRef<Path>>(path: P) -> String {
	let mut file = OpenOptions::new()
	               .read(true)
				   .open(path)
				   .expect("failed to open input file");
	
	let mut input = String::new();
	file.read_to_string(&mut input).expect("failed to read input file");
	input
}