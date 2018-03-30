use std::fmt;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;
use std::str;
use std::str::FromStr;

#[derive(Copy, Clone)]
enum Register {
    A, B
}

enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i16),
    Jie(Register, i16),
    Jio(Register, i16)
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Register::A => write!(f, "a"),
            &Register::B => write!(f, "b")
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn offset_prefix(offset: i16) -> &'static str {
            if offset > 0 {"+"} else {""}
        }

        match self {
            &Instruction::Hlf(register)         => write!(f, "hlf {}", register),
            &Instruction::Tpl(register)         => write!(f, "tpl {}", register),
            &Instruction::Inc(register)         => write!(f, "inc {}", register),
            &Instruction::Jmp(offset)           => write!(f, "jmp {}{}", offset_prefix(offset), offset),
            &Instruction::Jie(register, offset) => write!(f, "jie {}, {}{}", register, offset_prefix(offset), offset),
            &Instruction::Jio(register, offset) => write!(f, "jio {}, {}{}", register, offset_prefix(offset), offset)
        }
    }
}

fn main() {
    let input = read_input("C:\\Users\\jankes\\Documents\\AdventOfCode\\2015\\23\\program.txt");
    let program = parse_program(&input);
    for instruction in program.iter() {
        println!("{}", instruction)
    }

    // part 1
    // let mut a = 0i64;
    
    // part 2
    let mut a = 1i64;
    
    let mut b = 0i64;
    let mut pc = 0i64;
    while 0 <= pc && (pc as usize) < program.len() {
        let instruction = &program[pc as usize];
        match instruction {
            &Instruction::Hlf(Register::A)         => a /= 2,
            &Instruction::Hlf(Register::B)         => b /= 2,
            &Instruction::Tpl(Register::A)         => a *= 3,
            &Instruction::Tpl(Register::B)         => b *= 3,
            &Instruction::Inc(Register::A)         => a += 1,
            &Instruction::Inc(Register::B)         => b += 1,
            &Instruction::Jmp(offset)              => pc += (offset as i64) - 1,
            &Instruction::Jie(Register::A, offset) => if a % 2 == 0 { pc += (offset as i64) - 1; },
            &Instruction::Jie(Register::B, offset) => if b % 2 == 0 { pc += (offset as i64) - 1; },
            &Instruction::Jio(Register::A, offset) => if a == 1 { pc += (offset as i64) - 1; },
            &Instruction::Jio(Register::B, offset) => if b == 1 { pc += (offset as i64) - 1; },
        };
        pc += 1;
    }

    println!("a = {}", a);
    println!("b = {}", b);
}

fn parse_program(bytes: &[u8]) -> Vec<Instruction> {
    bytes.split(|&b| b == b'\n')
         .map(|line| parse_instruction(line))
         .collect::<Vec<Instruction>>()
}

fn parse_instruction(line: &[u8]) -> Instruction {
    const HLF: [u8; 3] = [b'h', b'l', b'f'];
    const TPL: [u8; 3] = [b't', b'p', b'l'];
    const INC: [u8; 3] = [b'i', b'n', b'c'];
    const JMP: [u8; 3] = [b'j', b'm', b'p'];
    const JIE: [u8; 3] = [b'j', b'i', b'e'];
    const JIO: [u8; 3] = [b'j', b'i', b'o'];

    let mut iter = line.iter().filter(|&&c| c != b'\r');
    let instruction = [*iter.next().unwrap(), *iter.next().unwrap(), *iter.next().unwrap()];
    let _space = iter.next().unwrap();
    match instruction {
        HLF | TPL | INC => {
            let register = parse_register(&mut iter);
            match instruction {
                HLF => Instruction::Hlf(register),
                TPL => Instruction::Tpl(register),
                INC => Instruction::Inc(register),
                _   => panic!("unknown instruction")
            }
        },
        JMP => {
            Instruction::Jmp(parse_offset_at_end(&mut iter))
        },
        JIE | JIO => {
            let register = parse_register(&mut iter);
            let _comma = iter.next().unwrap();
            let _space = iter.next().unwrap();
            let offset = parse_offset_at_end(&mut iter);
            match instruction {
                JIE => Instruction::Jie(register, offset),
                JIO => Instruction::Jio(register, offset),
                _   => panic!("unknown instruction")
            }
        },
        _ => panic!("unknown instruction")
    }
}

fn parse_register<'a, T>(iter: &mut T) -> Register where
    T: std::iter::Iterator<Item=&'a u8>
{
    match *iter.next().unwrap() {
        b'a' => Register::A,
        b'b' => Register::B,
        _    => panic!("unknown register")
    }
}

fn parse_offset_at_end<'a, T>(iter: &mut T) -> i16 where
    T: std::iter::Iterator<Item=&'a u8>
{
    let direction = parse_offset_direction(iter);
    direction * parse_number_at_end(iter)
}

fn parse_offset_direction<'a, T>(iter: &mut T) -> i16 where
    T: std::iter::Iterator<Item=&'a u8>
{
    match *iter.next().unwrap() {
        b'+' => 1i16,
        b'-' => -1i16,
        _    => panic!("unknown increment direction")
    }
}

fn parse_number_at_end<'a, T>(iter: &mut T) -> i16 where
    T: std::iter::Iterator<Item=&'a u8>
{
    let mut number_bytes = [0u8; 4];
    let mut count = 0usize;
    iter.for_each(|&b| {
        if count == number_bytes.len() {
            panic!("too many digits in offset");
        }
        number_bytes[count] = b;
        count += 1;
    });
    let number_str = str::from_utf8(&number_bytes[0..count]).unwrap();
    i16::from_str(number_str).unwrap()
}

fn read_input<P: AsRef<Path>>(path: P) -> Vec<u8> {
	let mut file = OpenOptions::new()
	               .read(true)
				   .open(path)
				   .expect("failed to open input file");

	let mut input = Vec::<u8>::new();
	file.read_to_end(&mut input).expect("failed to read input file");
	input
}
