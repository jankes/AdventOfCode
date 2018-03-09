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
        let gate = parse_gate(line, &arena, &mut wires);
        connect_wires(gate);
        update_value(gate);

        //println!("{} -> {}", gate.signal, gate.wire);
    }
    print_values(&wires);
}

fn print_values<'a>(wires: &HashMap<[u8; 2], &'a Wire<'a>>) {
    let mut entries = wires.iter().collect::<Vec<(&[u8; 2], &&Wire)>>();
    entries.sort_by_key(|&(key, _)| *key);

    for &(_, wire) in entries.iter() {
        match wire.value.get() {
            Some(value) => println!("{} = {}", wire, value),
            None        => println!("{} = <not set>", wire)
        };
    }
}

fn update_value<'a>(gate: &'a Gate) {
    if gate.wire.value.get().is_some() {
        return;
    }
    match &gate.signal {
        &Signal::And(ref left, ref right) => {
            if let (Some(left), Some(right)) = (left.value(), right.value()) {
                gate.wire.value.set(Some(left & right));
            }
        },
        &Signal::Or(ref left, ref right) => {
            if let (Some(left), Some(right)) = (left.value(), right.value()) {
                gate.wire.value.set(Some(left | right));
            }
        },
        &Signal::LShift(ref left, ref right) => {
            if let (Some(wire), count) = (left.value.get(), right) {
                gate.wire.value.set(Some(wire << count));
            }
        },
        &Signal::RShift(ref left, ref right) => {
            if let (Some(wire), count) = (left.value.get(), right) {
                gate.wire.value.set(Some(wire >> count));
            }
        },
        &Signal::Not(ref wire) => {
            if let Some(value) = wire.value.get() {
                gate.wire.value.set(Some(!value));
            }
        },
        &Signal::FromWire(ref wire) => {
            if let Some(value) = wire.value.get() {
                gate.wire.value.set(Some(value));
            }
        }
        &Signal::Constant(value) => {
            gate.wire.value.set(Some(value));
        }
    };
    if gate.wire.value.get().is_some() {
        for gate in gate.wire.outputs.borrow().iter() {
            update_value(gate);
        }
    }
}

fn connect_wires<'a>(gate: &'a Gate<'a>) {
    gate.wire.input.set(Some(gate));

    match &gate.signal {
        &Signal::And(ref left, ref right) |
        &Signal::Or(ref left, ref right)  => {
            if let &Operand::FromWire(ref left) = left {
                left.outputs.borrow_mut().push(gate);
            }
            if let &Operand::FromWire(ref right) = right {
                right.outputs.borrow_mut().push(gate);
            }
        },
        &Signal::LShift(ref wire, _) |
        &Signal::RShift(ref wire, _) |
        &Signal::Not(ref wire) |
        &Signal::FromWire(ref wire) => wire.outputs.borrow_mut().push(gate),
        &Signal::Constant(_) => ()
    };
}

fn parse_gate<'a>(line: &str,
              arena: &'a ComponentArena<'a>,
              wires: &mut HashMap<[u8; 2], &'a Wire<'a>>) -> &'a Gate<'a> {
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
                      wires: &mut HashMap<[u8; 2], &'a Wire<'a>>) -> &'a Gate<'a> {
    let (left, right, output) = parse_boolean_gate(line, arena, wires);
    arena.alloc_gate(Signal::And(left, right), output)
}

fn parse_or_gate<'a>(line: &str,
                     arena: &'a ComponentArena<'a>,
                     wires: &mut HashMap<[u8; 2], &'a Wire<'a>>) -> &'a Gate<'a> {
    let (left, right, output) = parse_boolean_gate(line, arena, wires);
    arena.alloc_gate(Signal::Or(left, right), output)
}

fn parse_lshift_gate<'a>(line: &str,
                         arena: &'a ComponentArena<'a>,
                         wires: &mut HashMap<[u8; 2], &'a Wire<'a>>) -> &'a Gate<'a> {
    let (left, right, output) = parse_shift_gate(line, arena, wires);
    arena.alloc_gate(Signal::LShift(left, right), output)
}

fn parse_rshift_gate<'a>(line: &str,
                         arena: &'a ComponentArena<'a>,
                         wires: &mut HashMap<[u8; 2], &'a Wire<'a>>) -> &'a Gate<'a> {
    let (left, right, output) = parse_shift_gate(line, arena, wires);
    arena.alloc_gate(Signal::RShift(left, right), output)
}

fn parse_not_gate<'a>(line: &str,
                      arena: &'a ComponentArena<'a>,
                      wires: &mut HashMap<[u8; 2], &'a Wire<'a>>) -> &'a Gate<'a> {
    let mut parts = line.split(' ');
    let _not = parts.next().unwrap();
    let input = parts.next().unwrap();
    let _arrow = parts.next().unwrap();
    let output = parts.next().unwrap();
    arena.alloc_gate(Signal::Not(parse_wire(input, arena, wires)),
                     parse_wire(output, arena, wires))
}

fn parse_direct_set<'a>(line: &str,
                    arena: &'a ComponentArena<'a>,
                    wires: &mut HashMap<[u8; 2], &'a Wire<'a>>) ->  &'a Gate<'a> {
    let mut parts = line.split(' ');
    let input = parts.next().unwrap();
    let _arrow = parts.next().unwrap();
    let output = parts.next().unwrap();

    let output_wire = parse_wire(output, arena, wires);
    match input.as_bytes()[0] {
        b'0'...b'9' => arena.alloc_gate(Signal::Constant(parse_constant(input)), output_wire),
        _           => arena.alloc_gate(Signal::FromWire(parse_wire(input, arena, wires)), output_wire)
    }
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
            _           => Operand::FromWire(parse_wire(s, arena, wires))
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
    gates: Arena<Gate<'a>>,
    wires: Arena<Wire<'a>>
}

impl<'a> ComponentArena<'a> {
    fn new() -> ComponentArena<'a> {
        ComponentArena {
            gates: Arena::<Gate<'a>>::new(),
            wires: Arena::<Wire<'a>>::new()
        }
    }

    fn alloc_gate(&self, signal: Signal<'a>, wire: &'a Wire<'a>) -> &mut Gate<'a> {
        self.gates.alloc( Gate { signal: signal, wire: wire })
    }

    fn alloc_wire(&self, wire: Wire<'a>) -> &mut Wire<'a> {
        self.wires.alloc(wire)
    }
}

struct Gate<'a> {
    signal: Signal<'a>,
    wire: &'a Wire<'a>
}

enum Signal<'a> {
    And(Operand<'a>, Operand<'a>),
    Or(Operand<'a>, Operand<'a>),
    LShift(&'a Wire<'a>, u16),
    RShift(&'a Wire<'a>, u16),
    Not(&'a Wire<'a>),
    FromWire(&'a Wire<'a>),
    Constant(u16)
}

impl<'a> fmt::Display for Signal<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Signal::And(ref left, ref right)    => write!(f, "{} AND {}", left, right),
            &Signal::Or(ref left, ref right)     => write!(f, "{} OR {}", left, right),
            &Signal::LShift(ref wire, ref count) => write!(f, "{} LSHIFT {}", wire, count),
            &Signal::RShift(ref wire, ref count) => write!(f, "{} RSHIFT {}", wire, count),
            &Signal::Not(ref wire)               => write!(f, "NOT {}", wire),
            &Signal::FromWire(ref wire)          => write!(f, "{}", wire),
            &Signal::Constant(ref c)             => write!(f, "{}", c)
        }
    }
}

enum Operand<'a> {
    Constant(u16),
    FromWire(&'a Wire<'a>)
}

impl<'a> Operand<'a> {
    fn value(&self) -> Option<u16> {
        match *self {
            Operand::Constant(value) => Some(value),
            Operand::FromWire(wire)  => wire.value.get()
        }
    }
}

impl<'a> fmt::Display for Operand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Operand::Constant(ref c)    => write!(f, "{}", c),
            &Operand::FromWire(ref wire) => write!(f, "{}", wire)
        }
    }
}

struct Wire<'a> {
    name: [u8; 2],
    value: Cell<Option<u16>>,
    input: Cell<Option<&'a Gate<'a>>>,
    outputs: RefCell<Vec<&'a Gate<'a>>>
}

impl<'a> Wire<'a> {
    fn new(name: [u8; 2]) -> Wire<'a> {
        Wire {
            name: name,
            value: Cell::new(None),
            input: Cell::new(None),
            outputs: RefCell::new(Vec::<&'a Gate<'a>>::new())
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
