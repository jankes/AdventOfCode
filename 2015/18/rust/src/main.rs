use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

mod with_traits;

fn main() {
    let grid_string = read_input("C:\\Users\\jankes\\Documents\\AdventOfCode\\2015\\18\\input.txt");

    with_traits::run(&grid_string);
}

fn read_input<P: AsRef<Path>>(path: P) -> String {
    let mut file = OpenOptions::new()
	               .read(true)
				   .open(path)
                   .expect("expect to be able to open input file for reading");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("should be able to read input file");
    s
}