use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;
use std::str;
use std::str::FromStr;

fn main() {
	let input = read_input("C:\\Users\\sjank\\Documents\\Projects\\AdventOfCode\\2015\\7\\input2.txt");
	let mut sim = Simulation::new();
	sim.run(&input);
	
	println!("Part 1:");
	print_values(&sim.wire_values);
}

fn print_values(wire_values: &HashMap<WireRef, Option<u16>>) {
	let mut entries = wire_values.iter()
								.map(|(k, v)| (std::convert::Into::<&str>::into(k), v))
								.collect::<Vec<_>>();
	
	entries.sort_unstable();
	for &(k, v) in entries.iter() {
		println!("{} -> {}", k, v.map_or(String::from(""), |v| v.to_string()));
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct WireRef([u8; 2]);

impl<'a> From<&'a WireRef> for &'a str {
	fn from(w: &'a WireRef) -> &'a str {
		str::from_utf8(&w.0).expect("wire name should be valid utf8 string")
	}
}

#[derive(Clone, Copy)]
enum Operand {
	Literal(u16),
	WireRef(WireRef)
}

impl Operand {
	fn value(&self, wire_values: &HashMap<WireRef, Option<u16>>) -> Option<u16> {
		match self {
			&Operand::Literal(val) => Some(val),
			&Operand::WireRef(r) => wire_values[&r]
		}
	}
}

struct Infix {
	op_left: Operand,
	op_right: Operand,
	result_ref: WireRef
}

impl Infix {
	fn new(op_left: Operand, op_right: Operand, result_ref: WireRef) -> Infix {
		Infix {
			op_left: op_left,
			op_right: op_right,
			result_ref: result_ref
		}
	}
}

struct Prefix {
	op: Operand,
	result_ref: WireRef
}

impl Prefix {
	fn new(op: Operand, result_ref: WireRef) -> Prefix {
		Prefix {
			op: op, result_ref: result_ref
		}
	}
}

#[derive(Clone, Copy)]
struct GateRef(usize);

enum Gate {
	And(Infix),
	Or(Infix),
	LShift(Infix),
	RShift(Infix),
	Not(Prefix)
}

impl Gate {
	fn wires(&self) -> (Operand, Option<Operand>, WireRef) {
		match self {
			&Gate::And(Infix{op_left, op_right, result_ref})    |
			&Gate::Or(Infix{op_left, op_right, result_ref})     |
			&Gate::LShift(Infix{op_left, op_right, result_ref}) |
			&Gate::RShift(Infix{op_left, op_right, result_ref}) => (op_left, Some(op_right), result_ref),
			&Gate::Not(Prefix{op, result_ref}) => (op, None, result_ref)
		}
	}
}

struct Gates(Vec<Gate>);

impl Gates {
	fn new() -> Gates {
		Gates(Vec::new())
	}
	
	fn add(&mut self, gate: Gate) -> GateRef {
		self.0.push(gate);
		GateRef(self.0.len() - 1)
	}
	
	fn get(&self, gref: GateRef) -> &Gate {
		&self.0[gref.0]
	}
}

struct Simulation {
	gates: Gates,
	wire_values: HashMap<WireRef, Option<u16>>,
	wire_gates: HashMap<WireRef, Vec<GateRef>>
}

impl Simulation {
	fn new() -> Simulation {
		Simulation {
			wire_values: HashMap::new(),
			wire_gates: HashMap::new(),
			gates: Gates::new()
		}
	}

	fn run(&mut self, input: &str) {
		let gates = &mut self.gates;
		let mut wire_values = &mut self.wire_values;
		let wire_gates = &mut self.wire_gates;
		for s in input.split("\r\n") {
			let gate = parse_gate(s);
			let (left, right, result_ref) = gate.wires();
			let gate_ref = gates.add(gate);
			
			if let Operand::WireRef(wr) = left {
				wire_values.entry(wr).or_insert(None);
				let gate_refs = wire_gates.entry(wr).or_insert(Vec::new());
				gate_refs.push(gate_ref);
			}
			if let Some(Operand::WireRef(wr)) = right {
				wire_values.entry(wr).or_insert(None);
				let gate_refs = wire_gates.entry(wr).or_insert(Vec::new());
				gate_refs.push(gate_ref);
			}
			wire_values.entry(result_ref).or_insert(None);
			wire_gates.entry(result_ref).or_insert(Vec::new());
			
			if let Some(update_ref) = Simulation::update_result(gates.get(gate_ref), &mut wire_values) {
				Simulation::update_dependent_gates(update_ref, gates, wire_gates, wire_values);
			}
		}
	}
	
	fn update_dependent_gates(wire_ref: WireRef, gates: &Gates, wire_gates: &HashMap<WireRef, Vec<GateRef>>, wire_values: &mut HashMap<WireRef, Option<u16>>) {
		//println!("{}", std::convert::Into::<&str>::into(&wire_ref));
		for gate_ref in wire_gates[&wire_ref].iter() {
			if let Some(update_ref) = Simulation::update_result(gates.get(*gate_ref), wire_values) {
				Simulation::update_dependent_gates(update_ref, gates, wire_gates, wire_values);
			}
		}
	}

	fn update_result(gate: &Gate, wire_values: &mut HashMap<WireRef, Option<u16>>) -> Option<WireRef> {
		match gate {
			&Gate::And(Infix {op_left, op_right, result_ref}) => {
				match (op_left.value(wire_values), op_right.value(wire_values), wire_values[&result_ref]) {
					(Some(left_val), Some(right_val), None) => {
						let result = wire_values.get_mut(&result_ref).unwrap();
						*result = Some(left_val & right_val);
						Some(result_ref)
					}
					_ => None
				}
			},
			&Gate::Or(Infix {op_left, op_right, result_ref}) => {
				match (op_left.value(wire_values), op_right.value(wire_values), wire_values[&result_ref]) {
					(Some(left_val), Some(right_val), None) => {
						let result = wire_values.get_mut(&result_ref).unwrap();
						*result = Some(left_val | right_val);
						Some(result_ref)
					}
					_ => None
				}
			},
			&Gate::LShift(Infix {op_left, op_right, result_ref}) => {
				match (op_left.value(wire_values), op_right.value(wire_values), wire_values[&result_ref]) {
					(Some(left_val), Some(right_val), None) => {
						let result = wire_values.get_mut(&result_ref).unwrap();
						*result = Some(left_val << right_val);
						Some(result_ref)
					}
					_ => None
				}
			},
			&Gate::RShift(Infix {op_left, op_right, result_ref}) => {
				match (op_left.value(wire_values), op_right.value(wire_values), wire_values[&result_ref]) {
					(Some(left_val), Some(right_val), None) => {
						let result = wire_values.get_mut(&result_ref).unwrap();
						*result = Some(left_val >> right_val);
						Some(result_ref)
					}
					_ => None
				}
			}
			&Gate::Not(Prefix {op, result_ref}) => {
				match (op.value(wire_values), wire_values[&result_ref]) {
					(Some(val), None) => {
						let result = wire_values.get_mut(&result_ref).unwrap();
						*result = Some(!val);
						Some(result_ref)
					}
					_ => None
				}
			}
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

fn parse_gate(s: &str) -> Gate {
	let gate;
	if s.contains("OR") {
		let (left, right, result_ref) = parse_infix(s);
		gate = Gate::Or(Infix::new(left, right, result_ref))
	} else if s.contains("AND") {
		let (left, right, result_ref) = parse_infix(s);
		gate = Gate::And(Infix::new(left, right, result_ref))
	} else if s.contains("LSHIFT") {
		let (left, right, result_ref) = parse_infix(s);
		gate = Gate::LShift(Infix::new(left, right, result_ref))
	} else if s.contains("RSHIFT") {
		let (left, right, result_ref) = parse_infix(s);
		gate = Gate::RShift(Infix::new(left, right, result_ref))
	} else if s.contains("NOT") {
		let (op, result_ref) = parse_prefix(s);
		gate = Gate::Not(Prefix::new(op, result_ref));
	} else {
		let (left, right, result_ref) = parse_implicit_infix_with_right_implicitly_zero(s);
		gate = Gate::Or(Infix::new(left, right, result_ref));
	}
	gate
}

fn parse_infix(s: &str) -> (Operand, Operand, WireRef) {
	let mut parts = s.split(" ");
	let left = parse_operand(parts.next().expect("expect operand"));
	let _ = parts.next();
	let right = parse_operand(parts.next().expect("expect operand"));
	let _ = parts.next();
	let result_ref = parse_wire_ref(parts.next().expect("expect wire reference"));
	(left, right, result_ref)
}

fn parse_prefix(s: &str) -> (Operand, WireRef) {
	let mut parts = s.split(" ");
	let _ = parts.next();
	let op = parse_operand(parts.next().expect("expect operand"));
	let _ = parts.next();
	let result_ref = parse_wire_ref(parts.next().expect("expect wire reference"));
	(op, result_ref)
}

fn parse_implicit_infix_with_right_implicitly_zero(s: &str) -> (Operand, Operand, WireRef) {
	let mut parts = s.split(" ");
	let left = parse_operand(parts.next().expect("expect operand"));
	let _ = parts.next();
	let result_ref = parse_wire_ref(parts.next().expect("expect wire reference"));
	(left, Operand::Literal(0), result_ref)
}

fn parse_operand(s: &str) -> Operand {
	let bytes = s.as_bytes();
	match bytes[0] {
		b'0'...b'9' => Operand::Literal(u16::from_str(s).expect("failed to parse u16 operand")),
		_           => Operand::WireRef(parse_wire_ref(s))
	}
}

fn parse_wire_ref(s: &str) -> WireRef {
	let bytes = s.as_bytes();
	if bytes.len() == 1 {
		WireRef([bytes[0], b' '])
	} else {
		WireRef([bytes[0], bytes[1]])
	}
}

/*
let mut updated = HashSet::new();
let b_ref = WireRef([b'b', b' ']);

sim.wire_values.insert(b_ref, Some(46065));
updated.insert(b_ref);
Simulation::force_update_dependent_gates(b_ref, &sim.gates, &sim.wire_gates, &mut sim.wire_values, &mut updated);

println!("\r\nPart 2:");
print_values(&sim.wire_values);
*/

/*
impl Simulation {
	fn force_update_dependent_gates(wire_ref: WireRef,
									gates: &Gates,
									wire_gates: &HashMap<WireRef, Vec<GateRef>>,
									wire_values: &mut HashMap<WireRef, Option<u16>>,
									updated: &mut HashSet<WireRef>)
	{
		for gate_ref in wire_gates[&wire_ref].iter() {
			if let Some(update_ref) = Simulation::force_update_result(gates.get(*gate_ref), wire_values, updated) {
				Simulation::force_update_dependent_gates(update_ref, gates, wire_gates, wire_values, updated);
			}
		}
	}

	fn force_update_result(gate: &Gate, wire_values: &mut HashMap<WireRef, Option<u16>>, updated: &mut HashSet<WireRef>) -> Option<WireRef> {
		match gate {
			&Gate::And(Infix {op_left, op_right, result_ref}) => {
				match (op_left.value(wire_values), op_right.value(wire_values), updated.contains(&result_ref)) {
					(Some(left_val), Some(right_val), false) => {
						let result = wire_values.get_mut(&result_ref).unwrap();
						*result = Some(left_val & right_val);
						updated.insert(result_ref);
						Some(result_ref)
					}
					_ => None
				}
			},
			&Gate::Or(Infix {op_left, op_right, result_ref}) => {
				match (op_left.value(wire_values), op_right.value(wire_values), updated.contains(&result_ref)) {
					(Some(left_val), Some(right_val), false) => {
						let result = wire_values.get_mut(&result_ref).unwrap();
						*result = Some(left_val | right_val);
						updated.insert(result_ref);
						Some(result_ref)
					}
					_ => None
				}
			},
			&Gate::LShift(Infix {op_left, op_right, result_ref}) => {
				match (op_left.value(wire_values), op_right.value(wire_values), updated.contains(&result_ref)) {
					(Some(left_val), Some(right_val), false) => {
						let result = wire_values.get_mut(&result_ref).unwrap();
						*result = Some(left_val << right_val);
						updated.insert(result_ref);
						Some(result_ref)
					}
					_ => None
				}
			},
			&Gate::RShift(Infix {op_left, op_right, result_ref}) => {
				match (op_left.value(wire_values), op_right.value(wire_values), updated.contains(&result_ref)) {
					(Some(left_val), Some(right_val), false) => {
						let result = wire_values.get_mut(&result_ref).unwrap();
						*result = Some(left_val >> right_val);
						updated.insert(result_ref);
						Some(result_ref)
					}
					_ => None
				}
			}
			&Gate::Not(Prefix {op, result_ref}) => {
				match (op.value(wire_values), updated.contains(&result_ref)) {
					(Some(val), false) => {
						let result = wire_values.get_mut(&result_ref).unwrap();
						*result = Some(!val);
						updated.insert(result_ref);
						Some(result_ref)
					}
					_ => None
				}
			}
		}
	}
}
*/

/*
impl<'a> From<&'a WireRef> for String {
	fn from(w: &'a WireRef) -> String {
		let s: &str = w.into();
		String::from(w)
	}
}
*/

/*
fn simulate(input: &str) -> HashMap<WireRef, Option<u16>> {
	let mut wire_values = HashMap::<WireRef, Option<u16>>::new();
	let mut wire_gates = HashMap::<WireRef, Vec<GateRef>>::new();
	let mut gates = Gates::new();
	input.split("\r\n")
	     .for_each(|s| {
			let gate = parse_gate(s);
			let (left, right, result_ref) = gate.wires();
			let gate_ref = gates.add(gate);
			
			if let Operand::WireRef(wr) = left {
				wire_values.entry(wr).or_insert(None);
				let gate_refs = wire_gates.entry(wr).or_insert(Vec::new());
				gate_refs.push(gate_ref);
			}
			if let Some(Operand::WireRef(wr)) = right {
				wire_values.entry(wr).or_insert(None);
				let gate_refs = wire_gates.entry(wr).or_insert(Vec::new());
				gate_refs.push(gate_ref);
			}
			wire_values.entry(result_ref).or_insert(None);
			wire_gates.entry(result_ref).or_insert(Vec::new());
			
			if let Some(update_ref) = gates.update_result(gate_ref, &mut wire_values) {
				gates.update_dependent_gates(update_ref, &wire_gates, &mut wire_values);
			}
		 });
	wire_values
}
*/

/*
if let (Some(left_val), Some(right_val)) = (op_left.value(wire_values), op_right.value(wire_values)) {
	let value = wire_values.get_mut(&result_ref).unwrap();
	if value.is_none() {
		*value = Some(left_val & right_val);
		Some(result_ref)
	} else {
		None
	}
} else {
	None
}
*/

/*
impl Infix {
	fn left(&self) -> Operand {
		self.op_left
	}
	
	fn right(&self) -> Operand {
		self.op_right
	}
	
	fn result_ref(&self) -> WireRef {
		self.result_ref
	}
}

impl Gate {
	fn evaluate(&self, wires: &mut HashMap<WireRef, Wire>, gates: &mut Gates) {
		for gate_ref in wire.gates.iter() {
			gates.get(gate_ref).evaluate(wires, gates);
		}
		
		match self {
			&Gate::And(Infix {op_left, op_right, result_ref})    |
			&Gate::Or(Infix {op_left, op_right, result_ref})     |
			&Gate::LShift(Infix {op_left, op_right, result_ref}) |
			&Gate::RShift(Infix {op_left, op_right, result_ref}) => {
				if let (Some(left_val), Some(right_val)) = (op_left.value(wires), op_right.value(wires)) {
					
				}
			},
			&Gate::Not(Prefix {op, result_ref}) => {
				
			}
		};
		
	}
	
	fn update_result(&self, wires: &mut HashMap<WireRef, Wire>) -> Option<WireRef> {
		None
	}
}
*/

/*
	fn update_result(&self, gate_ref: GateRef, wire_values: &mut HashMap<WireRef, Option<u16>>) -> Option<WireRef> {
		let gate = self.get(gate_ref);
		match gate {
			&Gate::And(Infix {op_left, op_right, result_ref}) => {
				match (op_left.value(wire_values), op_right.value(wire_values), wire_values[&result_ref]) {
					(Some(left_val), Some(right_val), None) => {
						let result = wire_values.get_mut(&result_ref).unwrap();
						*result = Some(left_val & right_val);
						Some(result_ref)
					}
					_ => None
				}
			},
			&Gate::Or(Infix {op_left, op_right, result_ref}) => {
				match (op_left.value(wire_values), op_right.value(wire_values), wire_values[&result_ref]) {
					(Some(left_val), Some(right_val), None) => {
						let result = wire_values.get_mut(&result_ref).unwrap();
						*result = Some(left_val | right_val);
						Some(result_ref)
					}
					_ => None
				}
			},
			&Gate::LShift(Infix {op_left, op_right, result_ref}) => {
				match (op_left.value(wire_values), op_right.value(wire_values), wire_values[&result_ref]) {
					(Some(left_val), Some(right_val), None) => {
						let result = wire_values.get_mut(&result_ref).unwrap();
						*result = Some(left_val << right_val);
						Some(result_ref)
					}
					_ => None
				}
			},
			&Gate::RShift(Infix {op_left, op_right, result_ref}) => {
				match (op_left.value(wire_values), op_right.value(wire_values), wire_values[&result_ref]) {
					(Some(left_val), Some(right_val), None) => {
						let result = wire_values.get_mut(&result_ref).unwrap();
						*result = Some(left_val >> right_val);
						Some(result_ref)
					}
					_ => None
				}
			}
			&Gate::Not(Prefix {op, result_ref}) => {
				match (op.value(wire_values), wire_values[&result_ref]) {
					(Some(val), None) => {
						let result = wire_values.get_mut(&result_ref).unwrap();
						*result = Some(!val);
						Some(result_ref)
					}
					_ => None
				}
			}
		}
	}
	
	fn update_dependent_gates(&self, wire_ref: WireRef, wire_gates: &HashMap<WireRef, Vec<GateRef>>, wire_values: &mut HashMap<WireRef, Option<u16>>) {
		//println!("{}", std::convert::Into::<&str>::into(&wire_ref));
		for gate_ref in wire_gates[&wire_ref].iter() {
			if let Some(update_ref) = self.update_result(*gate_ref, wire_values) {
				self.update_dependent_gates(update_ref, wire_gates, wire_values);
			}
		}
	}
	*/