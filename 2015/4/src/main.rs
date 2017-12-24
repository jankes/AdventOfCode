extern crate md5;

/*
fn main() {
	let hash = md5::compute(b"abcdef609043");
	println!("{:x}", hash);
}
*/
/*
fn main() {
	let secret_key = String::from("abcdef");
	let count = 609043.to_string();
	
	let to_hash = secret_key.clone() + &count;
	println!("{}", to_hash);
	
	let hash = md5::compute(to_hash);
	println!("{:x}", hash);
	
	let hash2 = md5::compute(String::from("abcdef609043"));
	println!("{:x}", hash2);
}
*/

use std::string::ToString;

fn main() {
	let secret_key = String::from("ckczppom");
	//let secret_key = String::from("abcdef");
	
	for i in 1..10000000 {
		let count = i.to_string();
		let to_hash = secret_key.clone() + &count;
		let digest = md5::compute(to_hash);
		let hash = format!("{:x}", digest);
		
		let mut j = 0;
		for b in hash.as_bytes().iter() {
			if *b != b'0' {
				break;
			}
			j += 1;
			if j == 6 {
				println!("magic number = {}", i);
				return;
			}
		}
	}
}
