use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;
use std::str;
use std::str::FromStr;

fn main() {
	let distances = read_distances();


	let min = find_shortest_route(&distances);
	println!("shortest distance is: {}", min);

	/*
	let route = [0u8, 1, 2, 3, 4, 5, 6, 7];
	let test = route.windows(2)
                    .map(|r| distances.get(r[0] as usize, r[1] as usize))
	                .fold(0, |total_distance, from_to_distance| total_distance + from_to_distance);
	println!("test = {}", test);
	*/

	let (min, max) = find_min_max_route_distances(&distances);
	println!("min = {}, max = {}", min, max);
}

struct Distances(Vec<u16>);

impl Distances {
	/*
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
	*/

	fn get(&self, from: usize, to: usize) -> u16 {
		self.0[8 * from + to]
	}

	fn set(&mut self, from: usize, to: usize, value: u16) {
		self.0[8 * from + to] = value;
		self.0[8 * to + from] = value;
	}
}

fn find_shortest_route(distances: &Distances) -> u16 {
	let mut min = u16::max_value();
	let mut route = [0u8, 1, 2, 3, 4, 5, 6, 7];
	find_shortest_route_helper(&distances, &mut route, 0, &mut min);
	min
}

fn find_shortest_route_helper(distances: &Distances, route: &mut [u8], left: usize, min: &mut u16) {	
	if route.len() - left == 2 {
		update_min_route_distance(distances, route, min);
		rotate(route, left);
		update_min_route_distance(distances, route, min);
		rotate(route, left);
	} else {
		for _ in 0..route.len() - left {
			find_shortest_route_helper(distances, route, left + 1, min);
			rotate(route, left);
		}
	}
}

fn find_min_max_route_distances(distances: &Distances) -> (u16, u16) {
	let mut route = [0u8, 1, 2, 3, 4, 5, 6, 7];
	let mut min = u16::max_value();
	let mut max = 0; 
	{
		let mut update = |route: &[u8]| {
			let distance = route.windows(2)
								.map(|r| distances.get(r[0] as usize, r[1] as usize))
								.fold(0, |total_distance, from_to_distance| total_distance + from_to_distance);
			if distance < min {
				min = distance;
			}
			if distance > max {
				max = distance;
			}
		};
		enumerate_permutations(&mut route, &mut update);	
	}
	(min, max)
}

fn update_min_route_distance(distances: &Distances, route: &[u8], min: &mut u16) {
	let distance = route.windows(2)
                        .map(|r| distances.get(r[0] as usize, r[1] as usize))
	                    .fold(0, |total_distance, from_to_distance| total_distance + from_to_distance);
	if distance < *min {
		*min = distance;
	}
}

fn enumerate_permutations<F: FnMut(&[u8])>(s: &mut [u8], callback: &mut F) {
	enumerate_permutations_helper(s, 0, callback);
}

fn enumerate_permutations_helper<F: FnMut(&[u8])>(s: &mut [u8], left: usize, callback: &mut F) {
	if s.len() - left == 2 {
		callback(s);
		rotate(s, left);
		callback(s);
		rotate(s, left);
	} else {
		for _ in 0..s.len() - left {
			enumerate_permutations_helper(s, left + 1, callback);
			rotate(s, left);
		}
	}
}

fn rotate(s: &mut [u8], start: usize) {
	for i in start..s.len() - 1 {
		s.swap(i, i + 1);
	}
}

/*
fn test_stuff() {
	let mut test = [0u8, 1, 2, 3, ];
	print_permutations(&mut test, 0);
}
*/

/*
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

fn print_slice(s: &[u8]) {
	for c in s {
		print!("{} ", *c);
	}
	println!();
}
*/

fn read_distances() -> Distances {
	let mut distances = Distances((0u16..64).map(|_| 0).collect::<Vec<u16>>());

	let input = read_input("C:\\Users\\sjank\\Documents\\Projects\\AdventOfCode\\2015\\9\\input.txt");
	for line in input.lines() {
		let parts = line.split(' ').collect::<Vec<&str>>();
		let from = get_city_number(parts[0]);
		let to = get_city_number(parts[2]);
		let value = u16::from_str(parts[4]).unwrap();
		distances.set(to, from, value);
	}

	return distances;

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