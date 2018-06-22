use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str;
use std::str::FromStr;

fn main() {
    let program_data = read_to_vec("C:\\\\Users\\jankes\\Documents\\AdventOfCode\\2017\\8\\program.txt");
    let (program, register_count) = parse_program(&program_data);

    for instruction in program.iter() {
        println!("{}", instruction);
    }

    let mut registers = (0..register_count).map(|_| 0).collect::<Vec<i16>>();
    let mut max = i16::min_value();
    for instruction in program.iter() {
        if eval_cmp(&instruction.cmp, &registers) {
            let value = eval_math(&instruction.math, &mut registers);
            if value > max {
                max = value;
            }
        }
    }

    println!("largest value in any register after program completion is {}", registers.iter().max().unwrap());
    println!("largest value ever held in any register throughout program run is {}", max);
}

fn eval_math(math: &MathOp, registers: &mut [i16]) -> i16 {
    match math {
        &MathOp::Inc(register_id, arg) => registers[register_id as usize] += arg,
        &MathOp::Dec(register_id, arg) => registers[register_id as usize] -= arg
    }
    match math {
        &MathOp::Inc(register_id, _) |
        &MathOp::Dec(register_id, _) => registers[register_id as usize]
    }
}

fn eval_cmp(cmp: &CmpOp, registers: &[i16]) -> bool {
    match cmp {
        &CmpOp::Equal(register_id, arg) => registers[register_id as usize] == arg,
        &CmpOp::NotEqual(register_id, arg) => registers[register_id as usize] != arg,
        &CmpOp::LessThan(register_id, arg) => registers[register_id as usize] < arg,
        &CmpOp::GreaterThan(register_id, arg) => registers[register_id as usize] > arg,
        &CmpOp::LessThanOrEqual(register_id, arg) => registers[register_id as usize] <= arg,
        &CmpOp::GreaterThanOrEqual(register_id, arg) => registers[register_id as usize] >= arg
    }
}

fn parse_program(program_data: &[u8]) -> (Vec<Instruction>, usize) {
    let mut register_ids = HashMap::<&[u8],u16>::new();
    let mut get_next_id = next_id_fn();

    let instructions = program_data
        .split(|&c| c == b'\n')
        .map(|line| {
            let mut tokens = line.split(|&c| c == b' ');
            let math_op = parse_math_op(&mut tokens, &mut register_ids, &mut *get_next_id);
            let cmp_op = parse_cmp_op(&mut tokens, &mut register_ids, &mut *get_next_id);
            Instruction {
                cmp: cmp_op,
                math: math_op
            }
        })
        .collect::<Vec<Instruction>>();

    (instructions, register_ids.len())
}

fn parse_math_op<'a, I, F>(tokens: &mut I, register_ids: &mut HashMap<&'a [u8], u16>, next_id: F) -> MathOp
        where I: Iterator<Item = &'a [u8]>,
              F: FnOnce() -> u16 {
    let math_register_data = tokens.next().expect("expect register to operate on");
    let math_register_id = *register_ids.entry(math_register_data).or_insert_with(next_id);

    let math_op_data = tokens.next().expect("expect inc/dec operation");

    let math_arg_data = tokens.next().expect("expect argument to math operation");
    let math_arg_str = str::from_utf8(math_arg_data).expect("argument to math operation should be ascii string");
    let math_arg = i16::from_str(math_arg_str).expect("expect ascii string for integer math argument");

    match math_op_data {
        b"inc" => MathOp::Inc(math_register_id, math_arg),
        b"dec" => MathOp::Dec(math_register_id, math_arg),
        _      => panic!("unexpected math operation (should be inc or dec)")
    }
}

fn parse_cmp_op<'a, I, F>(tokens: &mut I, register_ids: &mut HashMap<&'a [u8], u16>, next_id: F) -> CmpOp
        where I: Iterator<Item = &'a [u8]>,
              F: FnOnce() -> u16 {
    let _if = tokens.next().expect("expect \"if\"");

    let cmp_register_data = tokens.next().expect("expect register to compare with");
    let cmp_register_id = *register_ids.entry(cmp_register_data).or_insert_with(next_id);

    let cmp_op_data = tokens.next().expect("expect comparison operation");

    let cmp_arg_data = tokens.next().expect("expect argument to compare operation")
                                .split(|&c| c == b'\r').next().expect("unexpected line ending");
    let cmp_arg_str = str::from_utf8(cmp_arg_data).expect("expect argument to compare operation");
    let cmp_arg = i16::from_str(cmp_arg_str).expect("expect ascii string for integer compare argument");

    match cmp_op_data {
        b"==" => CmpOp::Equal(cmp_register_id, cmp_arg),
        b"!=" => CmpOp::NotEqual(cmp_register_id, cmp_arg),
        b"<"  => CmpOp::LessThan(cmp_register_id, cmp_arg),
        b">"  => CmpOp::GreaterThan(cmp_register_id, cmp_arg),
        b"<=" => CmpOp::LessThanOrEqual(cmp_register_id, cmp_arg),
        b">=" => CmpOp::GreaterThanOrEqual(cmp_register_id, cmp_arg),
        _     => panic!("unexpected compare operation")
    }
}

fn next_id_fn() -> Box<FnMut() -> u16> {
    let mut next_id = 0u16;
    Box::new(move || {
        let id = next_id;
        next_id += 1;
        id
    })
}

enum MathOp {
    Inc(u16, i16),
    Dec(u16, i16)
}

enum CmpOp {
    Equal(u16, i16),
    NotEqual(u16, i16),
    LessThan(u16, i16),
    GreaterThan(u16, i16),
    LessThanOrEqual(u16, i16),
    GreaterThanOrEqual(u16, i16)
}

struct Instruction {
    cmp: CmpOp,
    math: MathOp
}

impl fmt::Display for MathOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            MathOp::Inc(_, _) => "inc",
            MathOp::Dec(_, _) => "dec"
        };
        let (register_id, arg) = match self {
            MathOp::Inc(id, a) |
            MathOp::Dec(id, a) => (id, a)
        };
        write!(f, "{} {} {}", register_id, symbol, arg)
    }
}

impl fmt::Display for CmpOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            CmpOp::Equal(_, _) => "==",
            CmpOp::NotEqual(_, _) => "!=",
            CmpOp::LessThan(_, _) => "<",
            CmpOp::GreaterThan(_, _) => ">",
            CmpOp::LessThanOrEqual(_, _) => "<=",
            CmpOp::GreaterThanOrEqual(_, _) => ">="
        };
        let (register_id, arg) = match self {
            CmpOp::Equal(id, a) |
            CmpOp::NotEqual(id, a) |
            CmpOp::LessThan(id, a) |
            CmpOp::GreaterThan(id, a) |
            CmpOp::LessThanOrEqual(id, a) |
            CmpOp::GreaterThanOrEqual(id, a) => (*id, *a)
        };
        write!(f, "if {} {} {}", register_id, symbol, arg)
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.math, self.cmp)
    }
}

fn read_to_vec<P: AsRef<Path>>(file: P) -> Vec<u8> {
    let mut raw_data = Vec::<u8>::with_capacity(35500);
    File::open(file).unwrap()
    .read_to_end(&mut raw_data).expect("should be able to read file to memory");
    raw_data
}