use std::fmt;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

fn main() {
    let input = read_input("C:\\Users\\jankes\\Documents\\AdventOfCode\\2015\\16\\input.txt");
    let aunts = parse_input(&preparse_input(input));

    part1(&aunts);
    part2(&aunts);
}

fn part1(aunts: &[Aunt]) {
    let aunts_filtered = aunts.iter()
         .filter(|a| match a.children {
             None => true,
             Some(children) => children == 3,
         })
         .filter(|a| match a.cats {
             None => true,
             Some(cats) => cats == 7
         })
         .filter(|a| match a.samoyeds {
             None => true,
             Some(samoyeds) => samoyeds == 2
         })
         .filter(|a| match a.pomeranians {
             None => true,
             Some(pomeranians) => pomeranians == 3
         })
         .filter(|a| match a.akitas {
             None => true,
             Some(akitas) => akitas == 0
         })
         .filter(|a| match a.vizslas {
             None => true,
             Some(vizslas) => vizslas == 0
         })
         .filter(|a| match a.goldfish {
             None => true,
             Some(goldfish) => goldfish == 5
         })
         .filter(|a| match a.trees {
             None => true,
             Some(trees) => trees == 3
         })
         .filter(|a| match a.cars {
             None => true,
             Some(cars) => cars == 2
         })
         .filter(|a| match a.perfumes {
             None => true,
             Some(perfumes) => perfumes == 1
         })
         .collect::<Vec<&Aunt>>();
    
    if aunts_filtered.len() != 1 {
        println!("Search didn't work!");
    } else {
        println!("Aunt {} gave the present", aunts_filtered[0].id);
    }
}

fn part2(aunts: &[Aunt]) {
    let aunts_filtered = aunts.iter()
         .filter(|a| match a.children {
             None => true,
             Some(children) => children == 3,
         })
         .filter(|a| match a.cats {
             None => true,
             Some(cats) => cats > 7
         })
         .filter(|a| match a.samoyeds {
             None => true,
             Some(samoyeds) => samoyeds == 2
         })
         .filter(|a| match a.pomeranians {
             None => true,
             Some(pomeranians) => pomeranians < 3
         })
         .filter(|a| match a.akitas {
             None => true,
             Some(akitas) => akitas == 0
         })
         .filter(|a| match a.vizslas {
             None => true,
             Some(vizslas) => vizslas == 0
         })
         .filter(|a| match a.goldfish {
             None => true,
             Some(goldfish) => goldfish < 5
         })
         .filter(|a| match a.trees {
             None => true,
             Some(trees) => trees > 3
         })
         .filter(|a| match a.cars {
             None => true,
             Some(cars) => cars == 2
         })
         .filter(|a| match a.perfumes {
             None => true,
             Some(perfumes) => perfumes == 1
         })
         .collect::<Vec<&Aunt>>();
    
    if aunts_filtered.len() != 1 {
        println!("Search didn't work!");
    } else {
        println!("Aunt {} really gave the present", aunts_filtered[0].id);
    }
}

struct Aunt {
    id: u16,
    children: Option<u8>,
    cats: Option<u8>,
    samoyeds: Option<u8>,
    pomeranians: Option<u8>,
    akitas: Option<u8>,
    vizslas: Option<u8>,
    goldfish: Option<u8>,
    trees: Option<u8>,
    cars: Option<u8>,
    perfumes: Option<u8>
}

impl Aunt {
    fn new(id: u16) -> Aunt {
        Aunt {
            id: id,
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None
        }
    }
}

impl fmt::Display for Aunt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sue {}: ", self.id)?;
        if let Some(children) = self.children {
            write!(f, "children: {} ", children)?;
        }
        if let Some(cats) = self.cats {
            write!(f, "cats: {} ", cats)?;
        }
        if let Some(samoyeds) = self.samoyeds {
            write!(f, "samoyeds: {} ", samoyeds)?;
        }
        if let Some(pomeranians) = self.pomeranians {
            write!(f, "pomeranians: {} ", pomeranians)?;
        }
        if let Some(akitas) = self.akitas {
            write!(f, "akitas: {} ", akitas)?;
        }
        if let Some(vizslas) = self.vizslas {
            write!(f, "vizslas: {} ", vizslas)?;
        }
        if let Some(goldfish) = self.goldfish {
            write!(f, "goldfish: {} ", goldfish)?;
        }
        if let Some(trees) = self.trees {
            write!(f, "trees: {} ", trees)?;
        }
        if let Some(cars) = self.cars {
            write!(f, "cars: {} ", cars)?;
        }
        if let Some(perfumes) = self.perfumes {
            write!(f, "perfumes: {} ", perfumes)?;
        }
        Ok(())
    }
}

fn read_input<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = OpenOptions::new()
	               .read(true)
				   .open(path)
                   .expect("expect to be able to open input file for reading");
    let mut vec = Vec::<u8>::new();
    file.read_to_end(&mut vec).expect("should be able to read input file");
    vec
}

fn preparse_input(input: Vec<u8>) -> String {
    let no_colons_or_commas: Vec<u8> =
        input.iter()
             .filter_map(|&b| match b {
                 b':' | b',' => None,
                 b           => Some(b)
             })
             .collect();
    String::from_utf8(no_colons_or_commas).expect("input should be utf8 encoded")
}

fn parse_input(input: &str) -> Vec<Aunt> {
    let mut aunts = Vec::<Aunt>::with_capacity(500);
    for (i, line) in input.lines().enumerate() {
        let mut aunt = Aunt::new((i as u16) + 1);

        let mut tokens = line.split(' ');
        tokens.next().expect("Sue");
        tokens.next().expect("<number>");

        while let Some(key) = tokens.next() {
            let value_str = tokens.next().expect("value");
            let value = Some(u8::from_str(value_str).expect("value should be parsable to unsigned byte"));
            match key {
                "children"    => aunt.children = value,
                "cats"        => aunt.cats = value,
                "samoyeds"    => aunt.samoyeds = value,
                "pomeranians" => aunt.pomeranians = value,
                "akitas"      => aunt.akitas = value,
                "vizslas"     => aunt.vizslas = value,
                "goldfish"    => aunt.goldfish = value,
                "trees"       => aunt.trees = value,
                "cars"        => aunt.cars = value,
                "perfumes"    => aunt.perfumes = value,
                _             => panic!("unexpected thing")
            };
        }
        aunts.push(aunt);
    }
    aunts
}