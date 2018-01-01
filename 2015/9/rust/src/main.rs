use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;
use std::str;
use std::str::FromStr;

fn main() {
	/*
	let d1 = Distances::new();
	let d2 = read_distances();
	
	println!("{}", d1.0 == d2.0);
	*/

	let mut test = [0u8, 1, 2, 3, ];
	print_permutations(&mut test, 0);

	/*
	let min = find_shortest_route();
	println!("shortest distance is: {}", min);

	let distances = Distances::new();
	let route = [0u8, 1, 2, 3, 4, 5, 6, 7];
	let test = route.windows(2)
                    .map(|r| distances.get(r[0] as usize, r[1] as usize))
	                .fold(0, |total_distance, from_to_distance| total_distance + from_to_distance);
	println!("test = {}", test);
	*/
}

struct Distances(Vec<u16>);

impl Distances {
	fn new() -> Distances {
		Distances(
			vec!(0u16,  65, 129, 144,  71, 137,   3, 149,
	               65,   0,  63,   4, 105, 125,  55,  14,
			      129,  63,   0,  68,  52,  65,  22, 143,
			      144,   4,  68,   0,   8,  23, 136, 115,
			       71, 105,  52,   8,   0, 101,  84,  96,
			      137, 125,  65,  23, 101,   0, 107,  14,
			        3,  55,  22, 136,  84, 107,   0,  46,
			      149,  14, 143, 115,  96,  14,  46,   0)
		)
	}

	fn get(&self, from: usize, to: usize) -> u16 {
		self.0[8 * from + to]
	}

	fn set(&mut self, from: usize, to: usize, value: u16) {
		self.0[8 * from + to] = value;
		self.0[8 * to + from] = value;
	}
}

fn find_shortest_route() -> u16 {
	let mut min = u16::max_value();
	let distances = Distances::new();
	let mut route = [0u8, 1, 2, 3, 4, 5, 6, 7];
	find_shortest_route_helper(&distances, &mut route, 0, 7, &mut min);
	min
}

fn find_shortest_route_helper(distances: &Distances, route: &mut [u8], left: u8, right: u8, min: &mut u16) {
	if right - left == 1 {
		update_min_route_distance(distances, route, min);
		route.swap(left as usize, right as usize);
		update_min_route_distance(distances, route, min);
	} else {
		find_shortest_route_helper(distances, route, left + 1, right, min);
		for _ in 0..(right - left) {
			route.swap(left as usize, right as usize);
			find_shortest_route_helper(distances, route, left + 1, right, min);
		}
	}	
}

fn update_min_route_distance(distances: &Distances, route: &[u8], min: &mut u16) {
	let distance = route.windows(2)
                        .map(|r| distances.get(r[0] as usize, r[1] as usize))
	                    .fold(0, |total_distance, from_to_distance| total_distance + from_to_distance);
	if distance < *min {
		*min = distance;
	}
}

/*
fn test_stuff() {
	//let mut test = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7'];
	let mut test = [0, 1, 2, 3];
	let right = test.len() - 1;
	print_permutations(&mut test, 0, right);
}
*/

fn print_permutations(s: &mut [u8], left: usize) {
	if s.len() - left == 2 {
		print_slice(s);
		rotate(s, left);
		print_slice(s);
		rotate(s, left);
	} else {
		//print_permutations(s, left + 1);
		for _ in 0..s.len() - left {
			print_permutations(s, left + 1);
			rotate(s, left);
		}
	}
}

fn rotate(s: &mut [u8], start: usize) {
	for i in start..s.len() - 1 {
		s.swap(i, i + 1);
	}
}

fn print_slice(s: &[u8]) {
	for c in s {
		print!("{} ", *c);
	}
	println!();
}

fn read_distances() -> Distances {
	
	fn get_city_number(city_name: &str) -> usize {
		match city_name {
			"Faerun"        => 0,
			"Tristram"      => 1,
			"Tambi"         => 2,
			"Norrath"       => 3,
			"Snowdin"       => 4,
			"Straylight"    => 5,
			"AlphaCentauri" => 6,
			"Arbre"         => 7,
			_ => panic!("unknown city")
		}
	}
	
	let mut distances = Distances((0u16..64).map(|_| 0).collect::<Vec<u16>>());

	let input = read_input("C:\\Users\\sjank\\Documents\\Projects\\AdventOfCode\\2015\\9\\input.txt");
	for line in input.lines() {
		let parts = line.split(' ').collect::<Vec<&str>>();
		let from = get_city_number(parts[0]);
		let to = get_city_number(parts[2]);
		let value = u16::from_str(parts[4]).unwrap();
		distances.set(to, from, value);
	}

	distances
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