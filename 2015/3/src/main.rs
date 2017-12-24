use std::collections::HashSet;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

fn main() {
	let path = "C:\\Users\\jankes\\Documents\\AdventOfCode\\2015\\3\\input.txt";
	
	let directions;
	match get_directions(path) {
		Err(e) => {
			println!("failed to get directions:\r\n{}", e);
			return;
		},
		Ok(d) => {
			directions = d;
		}
	};
	
	let mut santa = Santa::new(0, 0);
	let mut robo = Santa::new(0, 0);
	let mut visits = HashSet::with_capacity(4096);
	visits.insert((0i16, 0i16));
	let mut robos_turn = false;
	for d in directions.iter() {
		let s = if robos_turn {
			&mut robo
		} else {
			&mut santa
		};
		match *d {
			b'^' => s.move_up(&mut visits),
			b'v' => s.move_down(&mut visits),
			b'>' => s.move_right(&mut visits),
			b'<' => s.move_left(&mut visits),
			_ => println!("unknown direction")
		};		
		robos_turn = !robos_turn;
	}
	println!("{}", visits.len());
}

struct Santa {
	x: i16,
	y: i16
}

impl Santa {
	fn new(x: i16, y: i16) -> Santa {
		Santa {
			x: x,
			y: y
		}
	}
	
	fn move_up(&mut self, visits: &mut HashSet<(i16, i16)>) {
		self.y += 1;
		visits.insert((self.x, self.y));
	}
	
	fn move_down(&mut self, visits: &mut HashSet<(i16, i16)>) {
		self.y -= 1;
		visits.insert((self.x, self.y));
	}
	
	fn move_right(&mut self, visits: &mut HashSet<(i16, i16)>) {
		self.x += 1;
		visits.insert((self.x, self.y));
	}
	
	fn move_left(&mut self, visits: &mut HashSet<(i16, i16)>) {
		self.x -= 1;
		visits.insert((self.x, self.y));
	}
}

fn get_directions<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Box<Error>> {
	let mut file = OpenOptions::new().read(true).open(path)?;
	let mut directions = Vec::with_capacity(8192);
	file.read_to_end(&mut directions)?;
	Ok(directions)
}

/*
fn part_1() {
	let path = "C:\\Users\\jankes\\Documents\\AdventOfCode\\2015\\3\\input.txt";
	
	let mut file;
	match OpenOptions::new().read(true).open(path) {
		Err(e) => {
			println!("{}", e);
			return;
		}
		Ok(f) => {
			file = f;
		}
	};
	
	let mut directions = Vec::with_capacity(8192);
	if let Err(e) = file.read_to_end(&mut directions) {
		println!("{}", e);
		return;
	}
	
	let mut presents = HashMap::with_capacity(4096);
	presents.insert((0i16, 0i16), 1u16);
	
	let mut x = 0i16;
	let mut y = 0i16;
	for d in directions.iter() {
		match *d {
			b'^' => y += 1,
			b'v' => y -= 1,
			b'>' => x += 1,
			b'<' => x -= 1,
			_ => println!("unknown direction")
		};
		let count = presents.entry((x, y)).or_insert(0);
		*count += 1;
	}
	
	println!("{}", presents.len());
}
*/