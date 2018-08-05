use std::fs;
use std::str;
use std::str::FromStr;

fn main() {
    let dance = fs::read_to_string("C:\\Users\\jankes\\Documents\\AdventOfCode\\2017\\16\\dance.txt")
                .expect("should be able to read dance input as String");
    let moves = parse_dance(&dance);

    let mut programs = Programs::new();

    programs.dance(&moves);
    println!("part 1: order = {}", programs.get_names());

    // The dancing programs end up in their initial ordering (a, b, c, ...) after 42 rounds of dancing!
    // So we can skip [higest multiple of 42 less than 1 billion] dances, and just do the remainder
    // Note they've already done one dance
    let dances_remaining = (1000000000 % 42) - 1;
    for _ in 0..dances_remaining {
        programs.dance(&moves);
    }
    println!("part 2: order = {}", programs.get_names());
}

fn parse_dance(dance: &str) -> Vec<Move> {
    dance
    .split(",")
    .map(|mv| {
        let (mv_type, mv_params) = mv.split_at(1);
        match mv_type {
            "s" => {
                let size = u8::from_str(mv_params).expect("bad spin size (should be parsable to u8)");
                Move::Spin(size)
            },
            "x" => {
                let mut params = mv_params.split("/");
                let index_a = params.next().expect("need index of first in exchange");
                let index_b = params.next().expect("need index of second in exchange");
                Move::Exchange(u8::from_str(index_a).expect("bad first exchange index (should be parsable to u8)"),
                               u8::from_str(index_b).expect("bad second exchange index (should be parsable to u8)"))
            },
            "p" => {
                let mut params = mv_params.split("/");
                let name_a = params.next().expect("need first name in partner");
                let name_b = params.next().expect("need second name in partner");
                Move::Partner(name_a.as_bytes()[0], name_b.as_bytes()[0])
            },
            _ => panic!("unknown dance move type")
        }
    })
    .collect::<Vec<Move>>()
}

#[derive(Copy, Clone, Debug)]
enum Move {
    Spin(u8),
    Exchange(u8, u8),
    Partner(u8, u8)
}

struct Programs {
    names: [u8; 16],
    scratch: Vec<u8>
}

impl Programs {
    fn new() -> Programs {
        Programs {
            names: [b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h',
                    b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p'],
            scratch: Vec::<u8>::with_capacity(16)
        }
    }

    fn get_names(&self) -> &str {
        str::from_utf8(&self.names).unwrap()
    }

    fn dance(&mut self, moves: &[Move]) {
        for &mv in moves {
            match mv {
                Move::Spin(count) => self.spin(count),
                Move::Exchange(index_a, index_b) => self.exchange(index_a, index_b),
                Move::Partner(name_a, name_b) => self.partner(name_a, name_b)
            };
        }
    }

    fn spin(&mut self, x: u8) {
        let x = x as usize;
        self.scratch.clear();
        self.scratch.extend_from_slice(&self.names[(16 - x)..]);
        self.scratch.extend_from_slice(&self.names[0..(16 - x)]);
        self.names.copy_from_slice(self.scratch.as_slice());
    }

    fn exchange(&mut self, index_a: u8, index_b: u8) {
        let index_a = index_a as usize;
        let index_b = index_b as usize;
        self.names.swap(index_a, index_b);
    }

    fn partner(&mut self, name_a: u8, name_b: u8) {
        let i = self.index_of(name_a);
        let j = self.index_of(name_b);
        self.names.swap(i, j);
    }

    fn index_of(&self, name: u8) -> usize {
        let mut i = 0;
        while self.names[i] != name {
            i += 1;
        }
        i
    }
}