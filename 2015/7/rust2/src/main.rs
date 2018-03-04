extern crate typed_arena;

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;
use typed_arena::Arena;
use std::str::FromStr;

fn main() {
    let arena = ComponentArena::new();

    // let wire1 = arena.alloc_wire(Wire::new([b'a', b'b']));
    // let wire2 = arena.alloc_wire(Wire::new([b'a', b'c']));

    // let signal1 = arena.alloc_signal(Signal::AndGate(
    //     Operand::FromWire(Cell::new(wire1)),
    //     Operand::FromWire(Cell::new(wire1))));


    let mut wires = HashMap::<[u8; 2], &Wire>::new();

    let input = read_input("C:\\Users\\sjank\\Documents\\Projects\\AdventOfCode\\2015\\7\\input1.txt");
    for line in input.lines() {
        if line.contains("AND") {
            /*
            let mut parts = line.split(' ');
            let left = parts.next().unwrap();
            let _and = parts.next().unwrap();
            let right = parts.next().unwrap();
            let _arrow = parts.next().unwrap();
            let output = parts.next().unwrap();

            //let op = parse_operand(left, &arena, &mut wires);

            let signal = arena.alloc_signal(Signal::AndGate(
                parse_operand(left, &arena, &mut wires),
                parse_operand(right, &arena, &mut wires)));

            let name_bytes = output.as_bytes();
            let wire = arena.alloc_wire(Wire::new([name_bytes[0], name_bytes[1]]));
            //wire.input = Cell::new(Some(&signal));
            */
            //parse_and_gate(line, arena, wires)
        }

        let (signal, output_wire) = {
            if line.contains("AND") {
                parse_and_gate(line, &arena, &mut wires)
            } else {
                panic!("unexpected gate type!");
            }
        };

        output_wire.input.set(Some(signal));

        match signal {
            &Signal::AndGate(ref left, ref right) |
            &Signal::OrGate(ref left, ref right) => {
                if let &Operand::FromWire(ref left) = left {
                    left.get().outputs.borrow_mut().push(signal);
                }
                if let &Operand::FromWire(ref right) = right {
                    right.get().outputs.borrow_mut().push(signal);
                }
            }
            _ => ()
        };

    }
}

fn parse_and_gate<'a>(line: &str,
                      arena: &'a ComponentArena<'a>,
                      wires: &mut HashMap<[u8; 2], &'a Wire<'a>>) -> (&'a Signal<'a>, &'a Wire<'a>) {
    let (left, right, output) = parse_two_input_gate(line, arena, wires);
    let signal = arena.alloc_signal(Signal::AndGate(left, right));
    (signal, output)
    /*
    output.input.set(Some(signal));

    match &*signal {
        &Signal::AndGate(ref left, ref right) |
        &Signal::OrGate(ref left, ref right) => {
            if let &Operand::FromWire(ref left) = left {
                left.get().outputs.borrow_mut().push(&*signal);
            }
            if let &Operand::FromWire(ref right) = right {
                right.get().outputs.borrow_mut().push(&*signal);
            }
        }
        _ => ()
    };
    */
}

fn parse_two_input_gate<'a>(line: &str,
                            arena: &'a ComponentArena<'a>,
                            wires: &mut HashMap<[u8; 2], &'a Wire<'a>>) -> (Operand<'a>, Operand<'a>, &'a Wire<'a>) {
    let mut parts = line.split(' ');
    let left = parts.next().unwrap();
    let _and = parts.next().unwrap();
    let right = parts.next().unwrap();
    let _arrow = parts.next().unwrap();
    let output = parts.next().unwrap().as_bytes();

    let output_wire = {
        let output_name = [output[0], output[1]];
        *wires.entry(output_name).or_insert_with(|| arena.alloc_wire(Wire::new(output_name)))
    };

    (parse_operand(left, arena, wires),
     parse_operand(right, arena, wires),
     output_wire)
}

fn parse_operand<'a>(s: &str, arena: &'a ComponentArena<'a>, wires: &mut HashMap<[u8; 2], &'a Wire<'a>>) -> Operand<'a> {
    let bytes = s.as_bytes();
	match bytes[0] {
		b'0'...b'9' => Operand::Constant(u16::from_str(s).unwrap()),
		_           => {
            let name = [bytes[0], bytes[1]];
            let wire = wires.entry(name).or_insert_with(|| arena.alloc_wire(Wire::new(name)));
            Operand::FromWire(Cell::new(*wire))
        }
	}
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

enum Signal<'a> {
    Constant(u16),
    FromWire(Cell<&'a Wire<'a>>),
    AndGate((Operand<'a>), (Operand<'a>)),
    OrGate((Operand<'a>), (Operand<'a>)),
    NotGate(Cell<&'a Wire<'a>>),
    LeftShift((Cell<&'a Wire<'a>>, u16)),
    RightShift((Cell<&'a Wire<'a>>, u16))
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

fn read_input<P: AsRef<Path>>(path: P) -> String {
	let mut file = OpenOptions::new()
	               .read(true)
				   .open(path)
				   .expect("failed to open input file");
	
	let mut input = String::new();
	file.read_to_string(&mut input).expect("failed to read input file");
	input
}

/*
enum Signal<'a> {
    Constant(u16),
    FromWire(Cell<Option<&'a Wire>>),
    AndGate((Operand<'a>), (Operand<'a>)),
    OrGate((Operand<'a>), (Operand<'a>)),
    NotGate(Cell<Option<&'a Wire>>),
    LeftShift((Cell<Option<&'a Wire>>, u16)),
    RightShift((Cell<Option<&'a Wire>>, u16))
}
*/