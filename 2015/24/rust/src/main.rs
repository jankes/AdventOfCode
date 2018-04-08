use std::str;

fn get_letter<'a>(ascii_slice: &'a [u8], index: usize) -> &'a str {
    str::from_utf8(&ascii_slice[index..index+1]).unwrap()
}

fn main() {
    let test = b"abcdef";

    choose(3, test.len(), |indexes| {
        println!("{} {} {}", get_letter(test, indexes[0]), get_letter(test, indexes[1]), get_letter(test, indexes[2]));
    });
}

fn choose<F: FnMut(&[usize])>(choose_count: usize, total_item_count: usize, mut indexes_callback: F) {
    let mut stack = Vec::<usize>::with_capacity(choose_count);
    let mut indexes = (0..choose_count).collect::<Vec<_>>();
    loop {
        indexes_callback(&indexes);

        for z in 0..choose_count {
            indexes[choose_count - 1 - z] += 1;
            if indexes[choose_count - 1 - z] != total_item_count - z {
                break;
            }
            if z == choose_count - 1 {
                return;
            }
            stack.push(choose_count - 1 - z);
        }

        while let Some(x) = stack.pop() {
            indexes[x] = indexes[x - 1] + 1;
        }
    }
}

////////////////////
////////////////////
////////////////////

use std::{
    fs::OpenOptions,
    io::Read,
    path::Path,
    str::FromStr
};

fn main() {
    let input = read_input("C:\\Users\\jankes\\Documents\\AdventOfCode\\2015\\24\\pkgs.txt");
    let total = input
                .lines()
                .map(|line| i16::from_str(line).unwrap())
                .sum::<i16>();

    println!("total = {}", total);
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

