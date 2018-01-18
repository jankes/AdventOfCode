use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() {
	let floor = determine_which_floor().unwrap();
	println!("part 1: Santa goes to floor {}", floor);

	let position = determine_when_basement_imperative().unwrap();
	println!("part 2 imperative: Gets to basement at position {}", position);

	let (floor, position) = determine_when_basement_functional().unwrap();
	println!("part 2 functional: Final Floor = {}, Gets to basement at position {}", floor, position);
}

fn determine_which_floor() -> Result<i32, Box<Error>> {
	let input = File::open("C:\\Users\\jankes\\Documents\\AdventOfCode\\2015\\1\\input.txt")?;
	let floor = 
		input.bytes()
		     .map(|b| match b {
			     Ok(b'(') => 1,
			     Ok(b')') => -1,
			     _    => 0
		     })
		     .fold(0, |floor, instruction| floor + instruction);
	Ok(floor)
}

fn determine_when_basement_functional() -> Result<(i32, usize), Box<Error>> {
	let (floor, position, _) =
	File::open("C:\\Users\\jankes\\Documents\\AdventOfCode\\2015\\1\\input1.txt")?
	.bytes()
	.map(|b| match b {
			     Ok(b'(') => 1,
			     Ok(b')') => -1,
			     _    => 0
		     })
    .enumerate()
	.fold((0, 1, false), |(floor, position, found_answer), (index, instruction)| {
		if !found_answer {
			if floor == -1 {
				(floor + instruction, index, true)
			} else {
				(floor + instruction, index + 1, false)
			}
		} else {
			(floor + instruction, position, true)
		}
	});

	Ok((floor, position))
}

fn determine_when_basement_imperative() -> Result<usize, Box<Error>> {
	let mut floor = 0;
	for (index, instruction) in File::open("C:\\Users\\jankes\\Documents\\AdventOfCode\\2015\\1\\input.txt")?
	         .bytes().enumerate() {
		match instruction {
			Ok(b'(') => floor += 1,
			Ok(b')') => floor -= 1,
			_        => ()
		};
		if floor == -1 {
			return Ok(index + 1);
		}
	}
	Ok(0)
}
