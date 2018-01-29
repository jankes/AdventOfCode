use std::collections::HashSet;
use std::fmt;
use std::str;
use read_input;

pub fn part1() {
    let replacement_input = read_input("C:\\Users\\jankes\\Documents\\AdventOfCode\\2015\\19\\replacements.txt");
	let replacements = parse_replacements(&replacement_input);
	let molecule = read_input("C:\\Users\\jankes\\Documents\\AdventOfCode\\2015\\19\\molecule.txt");

    let mut replaced = HashSet::<Vec<u8>>::new();
	for r in replacements.iter() {
		if r.from_len() == 1 {
			do_replacement_one_letter_target(r, molecule.as_bytes(), &mut replaced);
		} else {
			do_replacement_two_letter_target(r, molecule.as_bytes(), &mut replaced);
		}
	}
	println!("found {} distinct molecules after one replacement", replaced.len());
}

/*
fn test_stuff(replacements: &[Replacement]) {
	for r in replacements.iter() {
		println!("{}", r);
	}

	let molecule = "aaHTi";
	println!("molecule: {}", molecule);

	let r1 = &replacements[15];
	println!("from: {} to: {}", str::from_utf8(&r1.from[0..]).unwrap(), str::from_utf8(&r1.to[0..]).unwrap());

	let mut test = HashSet::<Vec<u8>>::new();

	do_replacement_one_letter_target(r1, molecule.as_bytes(), &mut test);

	for m in test.iter() {
		println!("{}", str::from_utf8(&m).unwrap());
	}
}
*/

fn do_replacement_one_letter_target(replacement: &Replacement, molecule: &[u8], generated: &mut HashSet<Vec<u8>>) {
	let mut start = 0usize;
	while let Some(i) = find_next_one_letter(replacement.from[0], &molecule[start..]) {
		start += i;

		let r = generate_replacement(replacement, molecule, start);
		generated.insert(r);

		start += 1;
		if start >= molecule.len() {
			break;
		}
	}
}

fn find_next_one_letter(target: u8, molecule: &[u8]) -> Option<usize> {
	let mut m = molecule.iter().enumerate();
	while let Some((i, &letter)) = m.next() {
		if target == letter {
			return Some(i);
		}
	}
	None
}

fn do_replacement_two_letter_target(replacement: &Replacement, molecule: &[u8], generated: &mut HashSet<Vec<u8>>) {
	let mut start = 0usize;
	while let Some(i) = find_next_two_letter(replacement.from, &molecule[start..]) {
		start += i;

		let r = generate_replacement(replacement, molecule, start);
		generated.insert(r);

		start += 2;
		if start >= molecule.len() {
			break;
		}
	}
}

fn find_next_two_letter(target: [u8; 2], molecule: &[u8]) -> Option<usize> {
	let mut found_first_letter = false;
	let mut m = molecule.iter().enumerate();
	while let Some((i, &letter)) = m.next() {
		if found_first_letter {
			if target[1] == letter {
				return Some(i - 1);
			}
			found_first_letter = false;
		} else {
			if target[0] == letter {
				found_first_letter = true;
			}
		}
	}
	None
}

fn generate_replacement(replacement: &Replacement, molecule: &[u8], index: usize) -> Vec<u8> {
	let mut generated = Vec::<u8>::with_capacity(molecule.len() + replacement.to_len() - replacement.from_len());
	push_up_to_index(&mut generated, molecule, index);
	push_replacement(&mut generated, replacement);
	push_starting_at_index(&mut generated, molecule, index + replacement.from_len());
	generated
}

fn push_up_to_index(vec: &mut Vec<u8>, letters: &[u8], index: usize) {
	for &letter in letters[0..index].iter() {
		vec.push(letter);
	}
}

fn push_replacement(vec: &mut Vec<u8>, replacement: &Replacement) {
	let mut replacement_letters = replacement.to.iter();
	while let Some(&replace) = replacement_letters.next() {
		if replace != 0u8 {
			vec.push(replace);
		} else {
			break;
		}
	}
}

fn push_starting_at_index(vec: &mut Vec<u8>, letters: &[u8], index: usize) {
	if index < letters.len() {
		for &letter in letters[index..].iter() {
			vec.push(letter);
		}
	}
}

struct Replacement {
	from: [u8; 2],
	to: [u8; 10]
}

impl Replacement {
	fn new() -> Replacement {
		Replacement {
			from: [0u8; 2],
			to: [0u8; 10]
		}
	}

	fn from_len(&self) -> usize {
		Replacement::len(&self.from[0..])		
	}

	fn to_len(&self) -> usize {
		Replacement::len(&self.to[0..])
	}

	fn len(s: &[u8]) -> usize {
		let mut count = 0usize;
		let mut letters = s.iter();
		while let Some(&letter) = letters.next() {
			if letter != 0u8 {
				count += 1;
			} else {
				break;
			}
		}
		count
	}
}

impl fmt::Display for Replacement {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} => ", str::from_utf8(&self.from).unwrap())?;
		write!(f, "{}", str::from_utf8(&self.to).unwrap())?;
		Ok(()) 
	}
}

fn parse_replacements(input: &str) -> Vec<Replacement> {
	input.lines()
	     .map(|line| {
			 let mut r = Replacement::new();
			 let mut parts = line.split(" ");

	         let mut first_word = parts.next().expect("expect target word").as_bytes().iter();
			 r.from[0] = *first_word.next().expect("target word must have at least one letter");
			 if let Some(&letter) = first_word.next() {
				 r.from[1] = letter;
			 }

			 parts.next().expect("expect \"=>\" after target word");

			 let mut second_word = parts.next().expect("expect maps-to word").as_bytes().iter().enumerate();
			 while let Some((i, &letter)) = second_word.next() {
				 r.to[i] = letter;
			 }
			 r
		 }).collect::<Vec<Replacement>>()
}
