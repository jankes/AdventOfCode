use std::error::Error;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

fn main() {
	let input;
	match get_input("C:\\Users\\jankes\\Documents\\AdventOfCode\\2015\\5\\input.txt") {
		Err(e) => {
			println!("{}", e);
			return;
		}
		Ok(i) => {
			input = i;
		}
	}
	
	let mut nice_count = 0;
	let mut i = 0;
	while i+15 < input.len() {
		let s = &input[i..(i+16)];
		
		//if has_special_string(s) {
		//	println!("{:?}", std::str::from_utf8(s).expect("should have valid string data"));
		//}
		
		if is_nice(s) {
			nice_count += 1;
		}
		i += 18;
	}
	
	println!("nice = {}", nice_count);
	
	/*
	let test1 = b"ugknbfddgicrmopn";
	let test2 = b"jchzalrnumimnmhp";
	let test3 = b"haegwjzuvuyypxyu";
	let test4 = b"dvszwmarrgswjxmb";
	
	println!("{}", is_nice(test1));
	println!("{}", is_nice(test2));
	println!("{}", is_nice(test3));
	println!("{}", is_nice(test4));
	*/
}

fn is_nice(s: &[u8]) -> bool {
	has_at_least_three_vowels(s) &&
	has_repeated_letter(s) &&
	!has_special_string(s)
}

fn has_at_least_three_vowels(s: &[u8]) -> bool {
	let mut count = 0;
	for c in s.iter() {
		if *c == b'a' || *c == b'e' || *c == b'i' || *c == b'o' || *c == b'u' {
			count += 1;
		}
	}
	count == 3
}

fn has_repeated_letter(s: &[u8]) -> bool {
	let mut prev_letter = 0u8;
	let mut first_loop = true;
	for c in s.iter() {
		if first_loop {
			first_loop = false;
		} else {
			if *c == prev_letter {
				return true; 
			}
		}
		prev_letter = *c;
	}
	return false;
}

fn has_special_string(s: &[u8]) -> bool {
	let mut prev_letter = 0u8;
	let mut first_loop = true;
	for c in s.iter() {
		if first_loop {
			first_loop = false;
		} else {
			let pair = [prev_letter, *c];
			if pair == [b'a', b'b'] ||
			   pair == [b'c', b'd'] ||
			   pair == [b'p', b'q'] ||
			   pair == [b'x', b'y'] {
				return true;
			}
		}
		prev_letter = *c;
	}
	return false;
}

fn get_input<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Box<Error>> {
	let mut file = OpenOptions::new().read(true).open(path)?;
	let mut input = Vec::with_capacity(18000);
	file.read_to_end(&mut input)?;
	Ok(input)
}
