use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

fn main() {
    let spec = fs::read_to_string("C:\\Users\\jankes\\Documents\\AdventOfCode\\2017\\12\\pipes.txt")
               .expect("should be able to read connection specification to a String");

    let mut programs = parse_connections(&spec);

    // part 1
    let root = programs.find_compress(0);    
    println!("program 0 group size = {}", programs.get_size(root));

    // part 2
    let mut roots = HashSet::new();
    for i in 0..programs.get_count() {
        let root = programs.find_compress(i);
        roots.insert(root);
    }
    println!("there are {} groups", roots.len());
}

fn parse_connections(connections: &str) -> Programs {
    let mut programs = Programs::with_count(count_lines(connections));

    for line in connections.lines() {
        let mut spec = line.split(" <-> ");

        let program_id_str = spec.next().expect("expect program id");
        let program_id = u16::from_str(program_id_str).expect("program id should be able to parse to Rust u16");

        let connections_str = spec.next().expect("expect list of connected programs");
        let mut connected_to = connections_str.split(", ");
        while let Some(connected_id_str) = connected_to.next() {
            let connected_id = u16::from_str(connected_id_str).expect("connected program id should be able to parse to Rust u16");
            programs.union(program_id, connected_id);
        }
    }
    programs
}

fn count_lines(s: &str) -> u16 {
    s.lines().map(|_| 1u16).sum::<u16>()
}

struct Node {
    parent: u16,
    size: u16
}

struct Programs {
    nodes: Vec<Node>
}

impl Programs {
    fn with_count(count: u16) -> Programs {
        let mut nodes = Vec::<Node>::with_capacity(count as usize);
        for i in 0..count {
            nodes.push(Node {
                parent: i,
                size: 1
            });
        }
        Programs {
            nodes: nodes
        }
    }

    fn union(&mut self, a: u16, b: u16) {
        let a_root = self.find_compress(a);
        let b_root = self.find_compress(b);
        if a_root != b_root {
            let (a_root, b_root) = self.order_by_size(a_root, b_root);
            self.set_parent(b_root, a_root);
            let new_size = self.get_size(a_root) + self.get_size(b_root);
            self.set_size(a_root, new_size);
        }
    }

    fn find_compress(&mut self, id: u16) -> u16 {
        let parent = self.get_parent(id);
        if parent == id {
            parent
        } else {
            let root = self.find_compress(parent);
            self.set_parent(id, root);
            root
        }
    }

    fn order_by_size(&self, a: u16, b: u16) -> (u16, u16) {
        if self.get_size(a) < self.get_size(b) {
            (b, a)
        } else {
            (a, b)
        }
    }

    fn get_parent(&self, id: u16) -> u16 {
        self.nodes[id as usize].parent
    }

    fn get_size(&self, id: u16) -> u16 {
        self.nodes[id as usize].size
    }

    fn set_parent(&mut self, id: u16, parent: u16) {
        self.nodes[id as usize].parent = parent;
    }

    fn set_size(&mut self, id: u16, size: u16) {
        self.nodes[id as usize].size = size;
    }

    fn get_count(&self) -> u16 {
        self.nodes.len() as u16
    }
}
