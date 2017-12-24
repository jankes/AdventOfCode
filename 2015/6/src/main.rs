use std::error::Error;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;


fn main() {
	let input;
	match get_input("C:\\Users\\jankes\\Documents\\AdventOfCode\\2015\\6\\input.txt") {
		Err(e) => {
			println!("{}", e);
			return;
		}
		Ok(i) => {
			input = i;
		}
	}
	
	part1::print_answer(&input);
	part2::print_answer(&input);
}

fn parse_cmd(cmd_str: &str) -> Cmd {
	let (typ, cmd_str) = parse_cmd_type(cmd_str);
	
	let mut left = 0u16;
	let mut bottom = 0u16;
	let mut right = 0u16;
	let mut top = 0u16;
	cmd_str.split(" through ")
		   .flat_map(|s| {s.split(",")})
		   .enumerate()
		   .for_each(|(i, s)| {
			   match i {
			       0 => left = u16::from_str(s).expect("expect coordinate number"),
				   1 => bottom = u16::from_str(s).expect("expect coordinate number"),
				   2 => right = u16::from_str(s).expect("expect coordinate number"),
				   3 => top = u16::from_str(s).expect("expect coordinate number"),
				   _ => ()
			   };
		   });
	Cmd {typ: typ, coords: Rectangle {left: left, bottom: bottom, right: right, top: top}}
}

fn parse_cmd_type(cmd_str: &str) -> (CmdType, &str) {
	let typ;
	let cmd_str = if cmd_str.starts_with("turn on") {
		typ = CmdType::On;
		cmd_str.trim_left_matches("turn on ")
	} else if cmd_str.starts_with("turn off") {
		typ = CmdType::Off;
		cmd_str.trim_left_matches("turn off ")
	} else if cmd_str.starts_with("toggle") {
		typ = CmdType::Toggle;
		cmd_str.trim_left_matches("toggle ")
	} else {
		panic!("unknown command")
	};
	(typ, cmd_str)
}

/*
fn show_cmds(input: &Vec<Cmd>) {
	for cmd in input.iter() {
		match cmd.typ {
			CmdType::On => print!("On"),
			CmdType::Off => print!("Off"),
			CmdType::Toggle => print!("toggle")
		};
		print!(" {} {} {} {}", cmd.coords.left, cmd.coords.bottom, cmd.coords.right, cmd.coords.top);
		println!();
	}
}
*/

pub struct Rectangle {
	left: u16,
	bottom: u16,
	right: u16,
	top: u16
}

#[derive(Clone, Copy)]
pub enum CmdType {
	On,
	Off,
	Toggle
}

pub struct Cmd {
	typ: CmdType,
	coords: Rectangle
}

mod part1 {
	use super::CmdType;
	use super::Cmd;
	
	pub fn print_answer(input: &Vec<Cmd>) {
		let mut grid = Grid::new(1000);
		for cmd in input.iter() {
			grid.do_cmd(cmd);
		}
		println!("part 1 lights on = {}", grid.calculate_on_count());
	}
	
	struct Grid {
		size: usize,
		lights: Vec<bool>
	}

	impl Grid {
		fn new(size: usize) -> Grid {
			Grid {
				size: size,
				lights: (0..size*size).map(|_| false).collect()
			}
		}
		
		fn calculate_on_count(&self) -> i32 {
			self.lights.iter().fold(0, |count, cur| {
				if *cur {
					count + 1
				} else {
					count
				}
			})
		}
		
		fn do_cmd(&mut self, cmd: &Cmd) {
			for y in cmd.coords.bottom..(cmd.coords.top + 1) {
				for x in cmd.coords.left..(cmd.coords.right + 1) {
					self.switch(cmd.typ, x as usize, y as usize);
				}
			}
		}
		
		fn switch(&mut self, typ: CmdType, x: usize, y: usize) {
			let index = y*self.size + x;
			match typ {
				CmdType::On => self.lights[index] = true,
				CmdType::Off => self.lights[index] = false,
				CmdType::Toggle => self.lights[index] = !self.lights[index]
			};
		}
	}
}

mod part2 {
	use super::CmdType;
	use super::Cmd;
	
	pub fn print_answer(input: &[Cmd]) {
		let mut grid = Grid::new(1000);
		for cmd in input.iter() {
			grid.do_cmd(cmd);
		}
		println!("part 2 brightness = {}", grid.calculate_brightness());
	}
	
	struct Grid {
		size: usize,
		lights: Vec<u16>
	}
	
	impl Grid {
		fn new(size: usize) -> Grid {
			Grid {
				size: size,
				lights: (0..size*size).map(|_| 0u16).collect()
			}
		}
		
		fn calculate_brightness(&self) -> i32 {
			self.lights.iter().fold(0, |brightness, cur| {
				brightness + (*cur as i32)
			})
		}
		
		fn do_cmd(&mut self, cmd: &Cmd) {
			for y in cmd.coords.bottom..(cmd.coords.top + 1) {
				for x in cmd.coords.left..(cmd.coords.right + 1) {
					self.switch(cmd.typ, x as usize, y as usize);
				}
			}
		}
		
		fn switch(&mut self, typ: CmdType, x: usize, y: usize) {
			let index = y*self.size + x;
			match typ {
				CmdType::On => self.lights[index] += 1,
				CmdType::Off => if self.lights[index] > 0 { self.lights[index] -= 1 },
				CmdType::Toggle => self.lights[index] += 2
			};
		}
	}
}

fn get_input<P: AsRef<Path>>(path: P) -> Result<Vec<Cmd>, Box<Error>> {
	let mut file = OpenOptions::new().read(true).open(path)?;
	let mut input = String::new();
	file.read_to_string(&mut input)?;
	let cmds = input.split("\r\n")
	     .map(parse_cmd)
		 .collect();
	Ok(cmds)
}
