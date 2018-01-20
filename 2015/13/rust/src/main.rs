use std::error::Error;
use std::fmt::{self, Debug, Formatter};
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

fn main() {
    if let Ok(input) = read_input("C:\\Users\\jankes\\Documents\\AdventOfCode\\2015\\13\\input.txt") {
        let size = 8;

        let table = HappyTable::parse(size, &input);
        println!("{:?}", table);

        let (optimal_happy, optimal_order) = find_optimal_seating(&table);
        print_optimal_results(optimal_happy, &optimal_order);

        println!("\r\n-- -- --\r\n");

        let table_with_me = table.add_myself();
        println!("{:?}", table_with_me);

        let (optimal_happy, optimal_order) = find_optimal_seating(&table_with_me);
        print_optimal_results(optimal_happy, &optimal_order);
    }
}

fn print_optimal_results(optimal_happy: i16, optimal_order: &[u8]) {
    println!("optimal order:");
    for d in optimal_order {
        println!("{} ", d);
    }
    println!("change in happiness = {}", optimal_happy);
}

fn find_optimal_seating(table: &HappyTable) -> (i16, Vec<u8>) {
    let mut start_diners: Vec<u8> = (0..table.size).collect();
    let mut optimal_happy = 0i16;
    let mut optimal_order = start_diners.clone();
    enumerate_permutations(&mut start_diners, &mut |diners| {
        let mut happiness = 0i16;
        for i in 0..diners.len() - 1 {
            happiness += table.get(diners[i], diners[i + 1]);
            happiness += table.get(diners[i + 1], diners[i]);
        }
        happiness += table.get(diners[diners.len() - 1], diners[0]);
        happiness += table.get(diners[0], diners[diners.len() - 1]);

        if happiness > optimal_happy {
            optimal_happy = happiness;
            optimal_order = diners.to_vec();
        }
    });
    (optimal_happy, optimal_order)
}

fn read_input<P: AsRef<Path>>(path: P) -> Result<String, Box<Error>> {
    let mut file = OpenOptions::new()
	               .read(true)
				   .open(path)?;

    let mut input = String::new();
    file.read_to_string(&mut input)?;
    Ok(input)
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

struct HappyTable {
    size: u8,
    values: Vec<i16>
}

impl HappyTable {
    fn parse(size: u8, table: &str) -> HappyTable {
        let mut parsed = Self::new(size);
        let mut lines = table.lines();
        let mut skip = 0;
        for p1 in 0..size {
            for p2 in 0..size {
                if p2 == skip {
                    continue;
                }
                let line = lines.next().expect("expect a line");
                parsed.set(p1, p2, Self::parse_line(line));
            }
            skip += 1;
        }
        parsed
    }

    fn parse_line(line: &str) -> i16 {
        let mut parts = line.split(' ');
        parts.next().expect("expect token for first <name>");
        parts.next().expect("expect token for would \"would\"");

        let gain = match parts.next().expect("expect token for \"gain\" or \"lose\"") {
            "gain" => true,
            "lose" => false,
            _      => panic!("expect \"gain\" or \"lose\"")
        };

        let units_str = parts.next().expect("expect token for happiness units");
        let units = i16::from_str(units_str).expect("expect base 10 integer for happiness units");

        parts.next().expect("expect literal token \"happiness\"");
        parts.next().expect("expect literal token \"units\"");
        parts.next().expect("expect literal token \"by\"");
        parts.next().expect("expect literal token \"sitting\"");
        parts.next().expect("expect literal token \"next\"");
        parts.next().expect("expect literal token \"to\"");
        parts.next().expect("expect literal token second <name>");

        if gain {
            units
        } else {
            -units
        }
    }

    fn new(size: u8) -> HappyTable {
        HappyTable {
            size: size,
            values: zeros(size as usize * size as usize)
        }
    }

    fn add_myself(self) -> HappyTable {
        let new_size = self.size + 1;
        let mut new_table_values = Vec::<i16>::with_capacity(new_size as usize);
        let mut i = 0usize;
        for _ in 0..self.size {
            for _ in 0..self.size {
                new_table_values.push(self.values[i]);
                i += 1;
            }
            new_table_values.push(0);
        }
        for _ in 0..new_size {
            new_table_values.push(0);
        }
        HappyTable {
            size: new_size,
            values: new_table_values
        }
    }

    fn set(&mut self, p1: u8, p2: u8, happy: i16) {
        let index = (p1 as usize) * (self.size as usize) + (p2 as usize);
        self.values[index] = happy;
    }

    fn get(&self, p1: u8, p2: u8) -> i16 {
        self.values[(p1 as usize) * (self.size as usize) + (p2 as usize)]
    }
}

impl Debug for HappyTable {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for p1 in 0..self.size {
            for p2 in 0..self.size {
                write!(f, "{:?} ", self.get(p1, p2))?
            }
            write!(f, "\r\n")?;
        }
        Ok(())
    }
}

fn zeros(count: usize) -> Vec<i16> {
    let mut v = Vec::<i16>::with_capacity(count);
    v.resize(count, 0);
    v
}
