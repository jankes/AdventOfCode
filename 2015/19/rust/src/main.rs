use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

//mod part1;
mod part2;

fn main() {	
	part2::part2();
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
