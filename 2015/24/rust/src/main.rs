use std::str;

fn get_letter<'a>(ascii_slice: &'a [u8], index: usize) -> &'a str {
    str::from_utf8(&ascii_slice[index..index+1]).unwrap()
}

fn main() {
    let test = b"abcdef";

    // choose 3 out of 6
    /*
    let mut i = 0usize;
    while i < test.len() - 2 {
        let mut j = i + 1;
        while j < test.len() - 1 {
            let mut k = j + 1;
            while k < test.len() {
                println!("{} {} {}", get_letter(test, i), get_letter(test, j), get_letter(test, k));                
                //println!("{} {} {}", i, j, k);
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    */

    // choose 3 out of 6 (also works)
    /*
    let mut i = 0usize;
    let mut j = i + 1;
    let mut k = j + 1;
    loop {
        println!("{} {} {}", get_letter(test, i), get_letter(test, j), get_letter(test, k));
        k += 1;
        if k == test.len() {
            j += 1;
            if j == test.len() - 1 {
                i += 1;
                if i == test.len() - 2 {
                    break;
                }
                j = i + 1;
            }
            k = j + 1;
        }
    }
    */

    // using a loop ...
    /*
    let mut stack = Vec::<usize>::with_capacity(3);
    let mut indexes = vec!(0usize, 1, 2);
    let size = indexes.len();
    loop {
        println!("{} {} {}", get_letter(test, indexes[0]), get_letter(test, indexes[1]), get_letter(test, indexes[2]));

        // for z in 0..size {
        //     indexes[size - 1 - z] += 1;
        //     if indexes[size - 1 - z] == test.len() - z {
        //         if z == size - 1 {
        //             return;
        //         }
        //         stack.push(size - 1 - z);
        //         continue;
        //     } else {
        //         break;
        //     }
        // }

        for z in 0..size {
            indexes[size - 1 - z] += 1;
            if indexes[size - 1 - z] != test.len() - z {
                break;
            }
            if z == size - 1 {
                return;
            }
            stack.push(size - 1 - z);
        }

        while let Some(x) = stack.pop() {
            indexes[x] = indexes[x - 1] + 1;
        }
    }
    */

    /*
    for choices in choose(3) {
        for c in choices {
            print!("{} ", test[*c]);
        }
        println!();
    }
    */

    let mut indexes = Vec::<usize>::with_capacity(3);
    let mut it = choose(&mut indexes);
    {
        it.next();
    }
    {
        it.next();
    }


    /*
    for _ in choose(&mut indexes) {
        // for i in indexes.iter() {
        //     println!("{}", i);
        // }
    }
    */
}

trait MyIterator<'b> {
    type Item;
    fn next(&'b mut self) -> Option<Self::Item>;
}

struct ChoicesIter<'a> {
    is_first: bool,
    stack: Vec<usize>,
    indexes: &'a mut Vec<usize>,
    size: usize,
}

impl<'a, 'b> MyIterator<'b> for ChoicesIter<'a> where 'b: 'a {
    type Item = &'a Vec<usize>;

    fn next(&'b mut self) -> Option<&'b Vec<usize>> {
        if self.is_first {
            self.indexes.iter_mut().enumerate().map(|(index, element)| *element = index);
            self.is_first = false;
            //return Some(std::convert::AsRef<usize>::as_ref(*self.indexes));
            return Some(self.indexes);
        }

        None
    }
}

fn choose<'a>(indexes: &'a mut Vec<usize>) -> ChoicesIter<'a> {
    let size = indexes.len();
    ChoicesIter {
        is_first: true,
        stack: Vec::with_capacity(indexes.len()),
        indexes: indexes,
        size: size
    }
}

/*
struct ChoicesIter<'a> {
    is_first: bool,
    stack: Vec<usize>,
    indexes: Vec<usize>,
    size: usize,
    _phantom: std::marker::PhantomData<&'a ()>
}

impl<'a> ChoicesIter<'a> {
    fn new(n: usize) -> ChoicesIter<'a> {
        ChoicesIter {
            is_first: true,
            stack: Vec::with_capacity(n),
            indexes: (0..n).collect::<Vec<_>>(),
            size: n,
            _phantom: std::marker::PhantomData
        }
    }
}

impl<'a> Iterator for ChoicesIter<'a> {
    type Item = &'a [usize];

    fn next(&mut self) -> Option<&'a [usize]> {
        if self.is_first {
            return Some(&self.indexes)
        }
        None
    }
}
*/

/*
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

    //let aa: u32 = total;

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
*/
