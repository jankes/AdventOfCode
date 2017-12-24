use std::cmp;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

fn main() {	
	main1();
	main2();
}


fn main1() {
	match get_input("C:\\Users\\jankes\\Documents\\AdventOfCode\\2015\\2\\input.txt")
	      .and_then(calculate_total_wrapping_and_ribbon) {
		Ok((wrapping, ribbon)) => println!("main1 wrapping = {}, ribbon = {}", wrapping, ribbon),
		Err(err)   => println!("{}", err)
	};
}

fn main2() {
	
	let mut total_wrapping_paper = 0;
	let mut total_ribbon = 0;

	let _ = get_input("C:\\Users\\jankes\\Documents\\AdventOfCode\\2015\\2\\input.txt")
	.map(|i|
		i.split("\r\n")
	     .map(get_components)
		 .for_each(|c| {
			if let Ok((l, w, h)) = c {
				let min = min3(l, w, h);
				let mid = mid3(l, w, h);
				total_wrapping_paper += 2*l*w + 2*l*h + 2*w*h + min*mid;
				total_ribbon += 2*min + 2*mid + l*w*h;
			}
		 })
	);
	println!("main2 wrapping  = {}, ribbon = {}", total_wrapping_paper, total_ribbon);

	/*
			  .map(|comps| {
			      let comps = comps?;
				  let result : Result<(i32, i32, i32), Box<Error>> = Ok(comps);
				  result
			  }));
    */
			  
			  //.map(|c| {
			//	let a : u8 = c;
				//c
			  //}));
			  
			  
			  //.for_each());
			  
			  //.fold(0, |_, a| {
				//let z : u8 = a;
				//0
			  //}));
	
	
	//input.and_then(|aa| Ok(aa.split("\r\n")));
	
	/*
	match input {
		Ok(input) => {
		
		},
		Err(..) => {}
	};
	*/
	//and_then(|_| Ok(input.split("\r\n")));
}
/*
fn calculate_total_wrapping(input: String) -> Result<i32, Box<Error>> {
	input.split("\r\n")
	     .map(|dimensions_str| {
			let mut components_iter = dimensions_str.split("x");
			let l_str = components_iter.next().ok_or("no length")?;
			let w_str = components_iter.next().ok_or("no width")?;
			let h_str = components_iter.next().ok_or("no height")?;
			let l = i32::from_str(l_str)?;
			let w = i32::from_str(w_str)?;
			let h = i32::from_str(h_str)?;
			let dimensions : Result<(i32, i32, i32), Box<Error>> = Ok((l, w, h));
			return dimensions;
		 })
		 .fold(Ok(0), |total, dimensions| {
			let total = total?;
			let (l, w, h) = dimensions?;
			Ok(total + 2*l*w + 2*l*h + 2*w*h + min3(l, w, h) * mid3(l, w, h))
		 })
}
*/

fn calculate_total_wrapping_and_ribbon(input: String) -> Result<(i32, i32), Box<Error>> {
	input.split("\r\n")
	     .map(|dimensions_str| {
			let mut components_iter = dimensions_str.split("x");
			let l_str = components_iter.next().ok_or("no length")?;
			let w_str = components_iter.next().ok_or("no width")?;
			let h_str = components_iter.next().ok_or("no height")?;
			let l = i32::from_str(l_str)?;
			let w = i32::from_str(w_str)?;
			let h = i32::from_str(h_str)?;
			let dimensions : Result<(i32, i32, i32), Box<Error>> = Ok((l, w, h));
			return dimensions;
		 })
		 .fold(Ok((0, 0)), |totals, dimensions| {
			let (wrapping, ribbon) = totals?;
			let (l, w, h) = dimensions?;
			let min = min3(l, w, h);
			let mid = mid3(l, w, h);
			Ok((wrapping + 2*l*w + 2*l*h + 2*w*h + min*mid, ribbon + 2*min + 2*mid + l*w*h))
		 })
}

//std::iter::Map<std::str::Split<'_, &str>

//Result<(i32, i32, i32), Box<Error>>


/*
fn get_dimensions(input: &String) -> std::iter::Map<std::str::Split<'_, &str>, > {
	input.split("\r\n")
	     .map(get_components)
}
*/

fn get_components(dimensions_str: &str) -> Result<(i32, i32, i32), Box<Error>> {
	let mut components_iter = dimensions_str.split("x");
	let l_str = components_iter.next().ok_or("no length")?;
	let w_str = components_iter.next().ok_or("no width")?;
	let h_str = components_iter.next().ok_or("no height")?;
	let l = i32::from_str(l_str)?;
	let w = i32::from_str(w_str)?;
	let h = i32::from_str(h_str)?;
	Ok((l, w, h))
}

/*
fn dummy() {
	let test = "aBa".split("B").map(|stuff| {0i32});
	let z : i32 = test;
}
*/
fn min3(x: i32, y: i32, z: i32) -> i32 {
	cmp::min(x, cmp::min(y, z))
}

fn mid3(x: i32, y: i32, z: i32) -> i32 {
	if (x <= y && y <= z) || (z <= y && y <= x) {
		y
	} else if (y <= x && x <= z) || (z <= x && x <= y) {
		x
	} else {
		z
	}
}

fn get_input<P: AsRef<Path>>(path: P) -> Result<String, Box<Error>> {
	let mut file = OpenOptions::new().read(true).open(path)?;
	let mut input = String::new();
	file.read_to_string(&mut input)?;
	return Ok(input);
}


/*
fn my_swap(a: &mut [i32; 3], i: usize, j: usize) {
	let temp = a[i];
	a[i] = a[j];
	a[j] = temp;
}

let mut test = [l, w, h];
if test[0] > test[1] {
	my_swap(&mut test, 0, 1);
}
if test[0] > test[2] {
	my_swap(&mut test, 0, 2);
}
if test[1] > test[2] {
	my_swap(&mut test, 1, 2);
}
if test[0] > test[1] {
	panic!("bug 1");
}
if test[1] > test[2] {
	panic!("bug 2");
}
if test[2] < test[0] {
	panic!("bug 3");
}
if mid != test[1] {
	println!("mid = {}; ({}, {}, {})",mid, l, w, h);
	panic!("bug 4");
}
*/
//if mid < l || mid > h {
//	println!("mid = {}; ({}, {}, {})",mid, l, w, h);
	//panic!("BUG!");
//}