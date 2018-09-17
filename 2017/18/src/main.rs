use std::fs;
use std::str::FromStr;

fn main() {
    let program_str = fs::read_to_string("C:\\Users\\jankes\\Documents\\AdventOfCode\\2017\\18\\duet.txt")
        .expect("should be able to read dance");

    let program = program_str
        .lines()
        .map(|line| {
            let mut tokens = line.split_whitespace();
            parse_instruction(&mut tokens)
        })
        .collect::<Vec<Instruction>>();

    let mut program_counter = 0i32;
    let mut registers = Registers::new();
    let mut last_sound = 0i32;

    while 0 <= program_counter && (program_counter as usize) < program.len() {
        match program[program_counter as usize] {
            Instruction::Snd(register)         => last_sound = registers.get(register),
            Instruction::Set(register_dst, op) => {
                match op {
                    Op::Val(val)          => registers.set(register_dst, val),
                    Op::Reg(register_src) => {
                        let val = registers.get(register_src);
                        registers.set(register_dst, val);
                    }
                }
            },
            Instruction::Add(register_dst, op) => {
                let val_original = registers.get(register_dst);
                match op {
                    Op::Val(val_to_add)   => registers.set(register_dst, val_original + val_to_add),
                    Op::Reg(register_src) => {
                        let val_to_add = registers.get(register_src);
                        registers.set(register_dst, val_original + val_to_add);
                    }
                }
            },
            Instruction::Mul(register_dst, op) => {
                let val_original = registers.get(register_dst);
                match op {
                    Op::Val(val_to_mul)   => registers.set(register_dst, val_original * val_to_mul),
                    Op::Reg(register_src) => {
                        let val_to_mul = registers.get(register_src);
                        registers.set(register_dst, val_original * val_to_mul);
                    }
                }
            }
            Instruction::Mod(register_dst, op) => {
                let val_original = registers.get(register_dst);
                match op {
                    Op::Val(val_to_mod)   => registers.set(register_dst, val_original % val_to_mod),
                    Op::Reg(register_src) => {
                        let val_to_mod = registers.get(register_src);
                        registers.set(register_dst, val_original % val_to_mod);
                    }
                }
            },
            Instruction::Rcv(register_test) => {
                if registers.get(register_test) != 0 {
                    println!("frequency of the last played sound was {}", last_sound);
                    return;
                }
            },
            Instruction::Jgz(op_test, op_offset) => {
                let test = match op_test {
                    Op::Val(test_val)      => test_val,
                    Op::Reg(test_register) => registers.get(test_register)
                };
                if test > 0 {
                    match op_offset {
                        Op::Val(offset)          => program_counter += offset,
                        Op::Reg(register_offset) => program_counter += registers.get(register_offset)
                    }
                    continue;
                }
            }
        };
        program_counter += 1;
    }
}

struct Registers {
    regs: [i32; 5]
}

impl Registers {
    fn new() -> Registers {
        Registers {
            regs: [0; 5]
        }
    }
    
    fn set(&mut self, dst: Register, val: i32) {
        self.regs[dst as usize] = val;
    }

    fn get(&self, src: Register) -> i32 {
        self.regs[src as usize]
    }
}

fn parse_instruction<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Instruction {
    match tokens.next() {
        None => panic!("expect instructions on every line"),
        Some(instruction_str) => {
            if instruction_str == "jgz" {
                Instruction::Jgz(parse_op(tokens), parse_op(tokens))
            } else {
                let register = parse_register(tokens);
                match instruction_str {
                    "snd" => Instruction::Snd(register),
                    "set" => Instruction::Set(register, parse_op(tokens)),
                    "add" => Instruction::Add(register, parse_op(tokens)),
                    "mul" => Instruction::Mul(register, parse_op(tokens)),
                    "mod" => Instruction::Mod(register, parse_op(tokens)),
                    "rcv" => Instruction::Rcv(register),
                    _     => panic!("unknown instruction")
                }
            }
        }
    }
}

fn parse_register<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Register {
    match tokens.next() {
        Some("a") => Register::A,
        Some("b") => Register::B,
        Some("f") => Register::F,
        Some("i") => Register::I,
        Some("p") => Register::P,
        _   => panic!("unknown register")
    }
}

fn parse_op<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Op {
    match tokens.next() {
        Some("a") => Op::Reg(Register::A),
        Some("b") => Op::Reg(Register::B),
        Some("f") => Op::Reg(Register::F),
        Some("i") => Op::Reg(Register::I),
        Some("p") => Op::Reg(Register::P),
        Some(val) => Op::Val(i32::from_str(val).expect("unknown register or invalid value")),
        _         => panic!("expect operand")
    }
}

#[derive(Copy, Clone)]
enum Register {
    A = 0,
    B = 1,
    F = 2,
    I = 3,
    P = 4
}

#[derive(Copy, Clone)]
enum Op {
    Reg(Register), Val(i32)
}

enum Instruction {
    Snd(Register),
    Set(Register, Op),
    Add(Register, Op),
    Mul(Register, Op),
    Mod(Register, Op),
    Rcv(Register),
    Jgz(Op, Op)
}