use std::collections::VecDeque;
use std::fs;
use std::mem;
use std::str::FromStr;

fn main() {
    let program_str = fs::read_to_string("C:\\Users\\jankes\\Documents\\AdventOfCode\\2017\\18\\duet.txt")
        .expect("should be able to read dance");

    println!("size_of(Op) = {}", std::mem::size_of::<Op>());
    println!("size_of(Instruction) = {}", std::mem::size_of::<Instruction>());

    let program = program_str
        .lines()
        .map(|line| {
            let mut tokens = line.split_whitespace();
            parse_instruction(&mut tokens)
        })
        .collect::<Vec<Instruction>>();

    part_1(&program);
    part_2(&program);
}

fn part_2(instructions: &[Instruction]) {
    let mut program_0 = Program::new_with_id(0);
    let mut program_1 = Program::new_with_id(1);
    let mut running = &mut program_0;
    let mut paused = &mut program_1;
    let mut send_count = 0u32;

    while can_continue(running.counter, instructions.len()) ||
          can_continue(paused.counter, instructions.len()) {
        while can_continue(running.counter, instructions.len()) {
            let instruction = &instructions[running.counter as usize];
            if !handle_basic_instruction(&mut running.registers, &mut running.counter, instruction) {
                match instruction {
                    &Instruction::Snd(register) => {
                        if running.id == 1 {
                            send_count += 1;
                        }
                        paused.msg_queue.push_back(running.registers.get(register));
                        paused.is_waiting = false;
                    },
                    &Instruction::Rcv(register) => {
                        if let Some(value) = running.msg_queue.pop_front() {
                            running.registers.set(register, value);
                        } else {
                            if paused.is_waiting || !can_continue(paused.counter, instructions.len()) {
                                println!("deadlock!");
                                running.counter = -1;
                                paused.counter = -1;
                            } else {
                                running.is_waiting = true;
                                running.counter -= 1;
                            }
                            break;
                        }
                    },
                    _ => panic!("unknown instruction")
                }
            }
        }
        mem::swap(&mut running, &mut paused);
    }
    println!("part 2: program 1 sends {} values", send_count);
}

fn part_1(instructions: &[Instruction]) {
    let mut registers = Registers::new();
    let mut program_counter = 0i64;
    let mut last_sound = 0i64;

    while can_continue(program_counter, instructions.len()) {
        let instruction = &instructions[program_counter as usize];
        if !handle_basic_instruction(&mut registers, &mut program_counter, instruction) {
            match instruction {
                &Instruction::Snd(register) => {
                    last_sound = registers.get(register)
                },
                &Instruction::Rcv(register_test) => {
                    if registers.get(register_test) != 0 {
                        println!("part 1: frequency of the last played sound was {}", last_sound);
                        return;
                    }
                },
                _ => panic!("unknown instruction")
            }
        }
    }
}

fn can_continue(program_counter: i64, instructions_len: usize) -> bool {
    0 <= program_counter && (program_counter as usize) < instructions_len
}

fn handle_basic_instruction(registers: &mut Registers, program_counter: &mut i64, instruction: &Instruction) -> bool {
    match instruction {
        &Instruction::Set(register_dst, op) => {
            match op {
                Op::Val(val)          => registers.set(register_dst, val),
                Op::Reg(register_src) => {
                    let val = registers.get(register_src);
                    registers.set(register_dst, val);
                }
            }
            *program_counter += 1;
            return true;
        },
        &Instruction::Add(register_dst, op) |
        &Instruction::Mul(register_dst, op) |
        &Instruction::Mod(register_dst, op) => {
            let val_original = registers.get(register_dst);
            let second_operand = match op {
                Op::Val(val) => val,
                Op::Reg(reg) => registers.get(reg)
            };
            let val_updated = match instruction {
                Instruction::Add(_, _) => val_original + second_operand,
                Instruction::Mul(_, _) => val_original * second_operand,
                Instruction::Mod(_, _) => val_original % second_operand,
                _                      => unreachable!()
            };
            registers.set(register_dst, val_updated);
            *program_counter += 1;
            return true;
        },
        &Instruction::Jgz(op_test, op_offset) => {
            let test = match op_test {
                Op::Val(test_val)      => test_val,
                Op::Reg(test_register) => registers.get(test_register)
            };
            if test > 0 {
                match op_offset {
                    Op::Val(offset)          => *program_counter += offset,
                    Op::Reg(register_offset) => *program_counter += registers.get(register_offset)
                }
            } else {
                *program_counter += 1;
            }
            return true;
        },
        _ => {
            *program_counter += 1;
            return false;
        }
    };
}

struct Program {
    id: i64,
    counter: i64,
    registers: Registers,
    msg_queue: VecDeque<i64>,
    is_waiting: bool
}

impl Program {
    fn new_with_id(id: i64) -> Program {
        let mut p = Program {
            id: id,
            counter: 0i64,
            registers: Registers::new(),
            msg_queue: VecDeque::<i64>::with_capacity(32),
            is_waiting: false
        };
        p.registers.set(Register::P, id);
        p
    }
}

struct Registers {
    regs: [i64; 5]
}

impl Registers {
    fn new() -> Registers {
        Registers {
            regs: [0; 5]
        }
    }
    
    fn set(&mut self, dst: Register, val: i64) {
        self.regs[dst as usize] = val;
    }

    fn get(&self, src: Register) -> i64 {
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
        Some(val) => Op::Val(i64::from_str(val).expect("unknown register or invalid value")),
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
    Reg(Register), Val(i64)
}

#[derive(Copy, Clone)]
enum Instruction {
    Snd(Register),
    Set(Register, Op),
    Add(Register, Op),
    Mul(Register, Op),
    Mod(Register, Op),
    Rcv(Register),
    Jgz(Op, Op)
}