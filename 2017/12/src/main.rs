extern crate fixedbitset;

use fixedbitset::FixedBitSet;
use std::boxed::Box;
use std::fs;
use std::str::FromStr;

fn main() {
    //let mut programs = Programs::new();

    // programs.add_neighbor(0, 2);
    // programs.add_neighbor(1, 1);
    // programs.add_neighbor(2, 0);
    // programs.add_neighbor(2, 3);
    // programs.add_neighbor(2, 4);
    // programs.add_neighbor(3, 2);
    // programs.add_neighbor(3, 4);
    // programs.add_neighbor(5, 6);
    // programs.add_neighbor(6, 4);
    // programs.add_neighbor(6, 5);
    
    // for a in 0..=6 {
    //     for b in 0..=6 {
    //         println!("{} and {} --> {}", a, b, if programs.are_neighbors(a, b) { "yes" } else { "no" });
    //     }
    // }

    let spec = fs::read_to_string("C:\\Users\\jankes\\Documents\\AdventOfCode\\2017\\12\\pipes.txt")
               .expect("should be able to read neighbor specification to a String");

    let programs = parse_programs(&spec);

    println!("program 0 group size = {} ", programs.group_size(0));


    // for program_id in 0u16..100 {
    //     print!("{} <-> ", program_id);
    //     for neighbor_id in programs.neighbors(program_id) {
    //         print!("{} ", neighbor_id);
    //     }
    //     println!();
    // }


}

fn parse_programs(neighbor_spec: &str) -> Programs {
    let mut programs = Programs::new();
    for line in neighbor_spec.lines() {
        let mut spec = line.split(" <-> ");

        let program_id_str = spec.next().expect("expect program id");
        let program_id = u16::from_str(program_id_str).expect("program id should be able to parse to Rust u16");

        let neighbors_str = spec.next().expect("expect list of neighbors");
        let mut neighbors = neighbors_str.split(", ");
        while let Some(neighbor_str) = neighbors.next() {
            let neighbor_id = u16::from_str(neighbor_str).expect("neighbor id should be able to parse to Rust u16");
            programs.add_neighbor(program_id, neighbor_id);
        }
    }
    programs
}

const NUM_PROGRAMS: usize = 2000;
const MAX_NEIGHBOORS: usize = 6;

struct Programs {
    neighbors: Box<[u16; MAX_NEIGHBOORS * NUM_PROGRAMS as usize]>,
    counts: Box<[u8; NUM_PROGRAMS as usize]>
}

impl Programs {
    fn new() -> Programs {
        Programs {
            neighbors: Box::new([0u16; MAX_NEIGHBOORS * NUM_PROGRAMS]),
            counts: Box::new([0u8; NUM_PROGRAMS as usize])
        }
    }

    fn group_size(&self, id: u16) -> usize {
        let mut found = FixedBitSet::with_capacity(NUM_PROGRAMS);
        self.group_size_helper(id, &mut found);
        found.count_ones(..)
    }

    fn group_size_helper(&self, id: u16, found: &mut FixedBitSet) {
        if !found[id as usize] {
            found.insert(id as usize);
            for neighbor_id in self.neighbors(id) {
                self.group_size_helper(neighbor_id, found);
            }
        }
    }

    fn add_neighbor(&mut self, a: u16, b: u16) {
        if self.are_neighbors(a, b) {
            return;
        }
        let a_count = self.counts[a as usize] as usize;
        let b_count = self.counts[b as usize] as usize;
        if a_count == MAX_NEIGHBOORS || b_count as usize == MAX_NEIGHBOORS {
            panic!("too many neighbors: ({}, {})", a, b);
        }
        self.neighbors[a as usize * MAX_NEIGHBOORS + a_count] = b;
        self.neighbors[b as usize * MAX_NEIGHBOORS + b_count] = a;
        self.counts[a as usize] += 1;
        self.counts[b as usize] += 1;
    }

    fn are_neighbors(&self, a: u16, b: u16) -> bool {
        if a == b {
            return true;
        }
        for program_id in self.neighbors(a) {
            if program_id == b {
                return true;
            }
        }
        false
    }

    fn neighbors(&self, id: u16) -> ProgramsIter {
        ProgramsIter::new(self, id)
    }
}

struct ProgramsIter<'a> {
    programs: &'a Programs,
    index: usize,
    end: usize
}

impl<'a> ProgramsIter<'a> {
    fn new(programs: &'a Programs, program_id: u16) -> ProgramsIter<'a> {
        let index = program_id as usize * MAX_NEIGHBOORS;
        ProgramsIter {
            programs: programs,
            index: index,
            end: index + programs.counts[program_id as usize] as usize
        }
    }
}

impl<'a> Iterator for ProgramsIter<'a> {
    type Item = u16;

    fn next(&mut self) -> Option<u16> {
        if self.index < self.end {
            let next = Some(self.programs.neighbors[self.index]);
            self.index  += 1;
            next
        } else {
            None
        }
    }
}