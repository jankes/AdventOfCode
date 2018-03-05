extern crate typed_arena;

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fmt;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;
use typed_arena::Arena;
use std::str;
use std::str::FromStr;

fn main() {
    let arena = ComponentArena::new();
    let mut wires = HashMap::<[u8; 2], &Wire>::new();
    let input = read_input("C:\\Users\\jankes\\Documents\\AdventOfCode\\2015\\7\\input.txt");
    for line in input.lines() {
        let (signal, output_wire) = parse_gate(line, &arena, &mut wires);
        connect_wires(signal, output_wire);

        println!("{} -> {}", signal, output_wire);
    }
}

fn connect_wires<'a>(signal: &'a Signal<'a>, output_wire: &'a Wire<'a>) {
    output_wire.input.set(Some(signal));

    match signal {
        &Signal::And(ref left, ref right) |
        &Signal::Or(ref left, ref right)  => {
            if let &Operand::FromWire(ref left) = left {
                left.get().outputs.borrow_mut().push(signal);
            }
            if let &Operand::FromWire(ref right) = right {
                right.get().outputs.borrow_mut().push(signal);
            }
        },
        &Signal::LShift(ref wire, _) |
        &Signal::RShift(ref wire, _) |
        &Signal::Not(ref wire) |
        &Signal::FromWire(ref wire) => wire.get().outputs.borrow_mut().push(signal),
        &Signal::Constant(_) => ()
    };
}

fn parse_gate<'a>(line: &str,
              arena: &'a ComponentArena<'a>,
              wires: &mut HashMap<[u8; 2], &'a Wire<'a>>) -> (&'a Signal<'a>, &'a Wire<'a>) {
    if line.contains("AND") {
        parse_and_gate(line, arena, wires)
    } else if line.contains("OR") {
        parse_or_gate(line, arena, wires)
    } else if line.contains("LSHIFT") {
        parse_lshift_gate(line, arena, wires)
    } else if line.contains("RSHIFT") {
        parse_rshift_gate(line, arena, wires)
    } else if line.contains("NOT") {
        parse_not_gate(line, arena, wires)
    } else {
        parse_direct_set(line, &arena, wires)
    }
}

fn parse_and_gate<'a>(line: &str,
                      arena: &'a ComponentArena<'a>,
                      wires: &mut HashMap<[u8; 2], &'a Wire<'a>>) -> (&'a Signal<'a>, &'a Wire<'a>) {
    let (left, right, output) = parse_boolean_gate(line, arena, wires);
    let signal = arena.alloc_signal(Signal::And(left, right));
    (signal, output)
}

fn parse_or_gate<'a>(line: &str,
                     arena: &'a ComponentArena<'a>,
                     wires: &mut HashMap<[u8; 2], &'a Wire<'a>>) -> (&'a Signal<'a>, &'a Wire<'a>) {
    let (left, right, output) = parse_boolean_gate(line, arena, wires);
    let signal = arena.alloc_signal(Signal::Or(left, right));
    (signal, output)
}

fn parse_lshift_gate<'a>(line: &str,
                         arena: &'a ComponentArena<'a>,
                         wires: &mut HashMap<[u8; 2], &'a Wire<'a>>) -> (&'a Signal<'a>, &'a Wire<'a>) {
    let (left, right, output) = parse_shift_gate(line, arena, wires);
    let signal = arena.alloc_signal(Signal::LShift(Cell::new(left), right));
    (signal, output)
}

fn parse_rshift_gate<'a>(line: &str,
                         arena: &'a ComponentArena<'a>,
                         wires: &mut HashMap<[u8; 2], &'a Wire<'a>>) -> (&'a Signal<'a>, &'a Wire<'a>) {
    let (left, right, output) = parse_shift_gate(line, arena, wires);
    let signal = arena.alloc_signal(Signal::RShift(Cell::new(left), right));
    (signal, output)
}

fn parse_not_gate<'a>(line: &str,
                      arena: &'a ComponentArena<'a>,
                      wires: &mut HashMap<[u8; 2], &'a Wire<'a>>) -> (&'a Signal<'a>, &'a Wire<'a>) {
    let mut parts = line.split(' ');
    let _not = parts.next().unwrap();
    let input = parts.next().unwrap();
    let _arrow = parts.next().unwrap();
    let output = parts.next().unwrap();

    (arena.alloc_signal(Signal::Not(Cell::new(parse_wire(input, arena, wires)))),
     parse_wire(output, arena, wires))
}

fn parse_direct_set<'a>(line: &str,
                    arena: &'a ComponentArena<'a>,
                    wires: &mut HashMap<[u8; 2], &'a Wire<'a>>) -> (&'a Signal<'a>, &'a Wire<'a>) {
    let mut parts = line.split(' ');
    let input = parts.next().unwrap();
    let _arrow = parts.next().unwrap();
    let output = parts.next().unwrap();

    let input_bytes = input.as_bytes();
    let signal = match input_bytes[0] {
        b'0'...b'9' => arena.alloc_signal(Signal::Constant(parse_constant(input))),
        _           => arena.alloc_signal(Signal::FromWire(Cell::new(parse_wire(input, arena, wires))))
    };
    (signal, parse_wire(output, arena, wires))
}

fn parse_boolean_gate<'a>(line: &str,
                          arena: &'a ComponentArena<'a>,
                          wires: &mut HashMap<[u8; 2], &'a Wire<'a>>) -> (Operand<'a>, Operand<'a>, &'a Wire<'a>) {
    let (left, right, output_wire) = parse_two_operand_gate(line, arena, wires);
    return (parse_operand(left, arena, wires),
            parse_operand(right, arena, wires),
            output_wire);

    fn parse_operand<'a>(s: &str,
                        arena: &'a ComponentArena<'a>,
                        wires: &mut HashMap<[u8; 2], &'a Wire<'a>>) -> Operand<'a> {
        match s.as_bytes()[0] {
            b'0'...b'9' => Operand::Constant(parse_constant(s)),
            _           => Operand::FromWire(Cell::new(parse_wire(s, arena, wires)))
        }
    }
}

fn parse_shift_gate<'a>(line: &str,
                        arena: &'a ComponentArena<'a>,
                        wires: &mut HashMap<[u8; 2], &'a Wire<'a>>) -> (&'a Wire<'a>, u16, &'a Wire<'a>) {
    let (left, right, output_wire) = parse_two_operand_gate(line, arena, wires);
    (parse_wire(left, arena, wires),
     parse_constant(right),
     output_wire)
}

fn parse_two_operand_gate<'a, 's>(line: &'s str,
                                  arena: &'a ComponentArena<'a>,
                                  wires: &mut HashMap<[u8; 2], &'a Wire<'a>>) -> (&'s str, &'s str, &'a Wire<'a>) {
    let mut parts = line.split(' ');
    let left = parts.next().unwrap();
    let _op = parts.next().unwrap();
    let right = parts.next().unwrap();
    let _arrow = parts.next().unwrap();
    let output = parts.next().unwrap();

    let output_wire = parse_wire(output, arena, wires);
    (left, right, output_wire)
}

fn parse_wire<'a>(s: &str,
                  arena: &'a ComponentArena<'a>,
                  wires: &mut HashMap<[u8; 2], &'a Wire<'a>>) -> &'a Wire<'a> {
    let name = parse_wire_name(s);
    *wires.entry(name).or_insert_with(|| arena.alloc_wire(Wire::new(name)))
}

fn parse_wire_name(s: &str) -> [u8; 2] {
    let bytes = s.as_bytes();
    if bytes.len() >= 2 {
        [bytes[0], bytes[1]]
    } else if bytes.len() == 1 {
        [bytes[0], b' ']
    } else {
        panic!("empty wire name")
    }
}

fn parse_constant(s: &str) -> u16 {
    u16::from_str(s).unwrap()
}

struct ComponentArena<'a> {
    signals: Arena<Signal<'a>>,
    wires: Arena<Wire<'a>>
}

impl<'a> ComponentArena<'a> {
    fn new() -> ComponentArena<'a> {
        ComponentArena {
            signals: Arena::<Signal<'a>>::new(),
            wires: Arena::<Wire<'a>>::new()
        }
    }

    fn alloc_signal(&self, signal: Signal<'a>) -> &mut Signal<'a> {
        self.signals.alloc(signal)
    }

    fn alloc_wire(&self, wire: Wire<'a>) -> &mut Wire<'a> {
        self.wires.alloc(wire)
    }
}

enum Operand<'a> {
    Constant(u16),
    FromWire(Cell<&'a Wire<'a>>)
}

impl<'a> fmt::Display for Operand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Operand::Constant(ref c)    => write!(f, "{}", c),
            &Operand::FromWire(ref wire) => write!(f, "{}", wire.get())
        }
    }
}

enum Signal<'a> {
    And(Operand<'a>, Operand<'a>),
    Or(Operand<'a>, Operand<'a>),
    LShift(Cell<&'a Wire<'a>>, u16),
    RShift(Cell<&'a Wire<'a>>, u16),
    Not(Cell<&'a Wire<'a>>),
    FromWire(Cell<&'a Wire<'a>>),
    Constant(u16)
}

impl<'a> fmt::Display for Signal<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Signal::And(ref left, ref right)    => write!(f, "{} AND {}", left, right),
            &Signal::Or(ref left, ref right)     => write!(f, "{} OR {}", left, right),
            &Signal::LShift(ref wire, ref count) => write!(f, "{} LSHIFT {}", wire.get(), count),
            &Signal::RShift(ref wire, ref count) => write!(f, "{} RSHIFT {}", wire.get(), count),
            &Signal::Not(ref wire)               => write!(f, "NOT {}", wire.get()),
            &Signal::FromWire(ref wire)          => write!(f, "{}", wire.get()),
            &Signal::Constant(ref c)             => write!(f, "{}", c)
        }
    }
}

struct Wire<'a> {
    name: [u8; 2],
    value: Cell<Option<u16>>,
    input: Cell<Option<&'a Signal<'a>>>,
    outputs: RefCell<Vec<&'a Signal<'a>>>
}

impl<'a> Wire<'a> {
    fn new(name: [u8; 2]) -> Wire<'a> {
        Wire {
            name: name,
            value: Cell::new(None),
            input: Cell::new(None),
            outputs: RefCell::new(Vec::<&'a Signal<'a>>::new())
        }
    }
}

impl<'a> fmt::Display for Wire<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.name[1] == b' ' {
            write!(f, "{}", str::from_utf8(&self.name[0..1]).unwrap())
        } else {
            write!(f, "{}", str::from_utf8(&self.name).unwrap())
        }
    }
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
