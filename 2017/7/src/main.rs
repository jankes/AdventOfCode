extern crate typed_arena;

use typed_arena::Arena;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::iter::Peekable;

fn main() {
    let mut raw_data = Vec::<u8>::with_capacity(35500);

    File::open("C:\\Users\\jankes\\Documents\\AdventOfCode\\2017\\7\\tower.txt").unwrap()
    .read_to_end(&mut raw_data).expect("should be able to read file to memory");

    let arena = Arena::<Node>::new();
    let mut nodes = HashMap::<[u8; 8], &Node>::new();

    for line in raw_data.split(|&c| c == b'\n') {
        let mut chars = line.iter().peekable();

        let name = parse_name(&mut chars, b' ');
        let weight = parse_weight(&mut chars);
        skip_optional_cr(&mut chars);

        // println!("{}", std::str::from_utf8(&name).unwrap());
        // println!("{}", weight);

        match chars.next() {
            None => {
                nodes.entry(name)
                     .and_modify(|node| node.weight.set(weight))
                     .or_insert(arena.alloc(Node::new_with_weight(name, weight)));
            },
            Some(&b' ') => {
                skip_arrow_to_children(&mut chars);

                // let mut children = Vec::<&Node>::new();
                // parse_children(&mut chars, |child_name| {
                //     //println!("   {}", std::str::from_utf8(child_name).unwrap()));
                //     let child_node = nodes.entry(child_name).or_insert(arena.alloc(Node::new(child_name)));
                //     children.push(child_node);
                // });

                let children = parse_children(&mut chars, |child_name|
                    nodes.entry(child_name)
                         .or_insert(arena.alloc(Node::new(child_name)))
                );

                let node = nodes.entry(name).or_insert(arena.alloc(Node::new(name)));
                node.weight.set(weight);
                node.children.replace(children);
            },
            _ => panic!("unexpected char at end of line"),
        };
    }

}

fn parse_children<'c, 'a, I, F>(chars: &mut Peekable<I>, mut alloc_node: F) -> Vec<&'a Node<'a>>
    where I: Iterator<Item = &'c u8>,
          F: FnMut([u8; 8]) -> &'a Node<'a>
{
    let mut children = Vec::new();
    let child_name = parse_name(chars, b',');
    children.push(alloc_node(child_name));
    while let Some(&&c) = chars.peek() {
        if c == b' ' {
            chars.next();
        }
        let child_name = parse_name(chars, b',');
        children.push(alloc_node(child_name));
    }
    children
}

// Alternate implemenation where the caller creates the list and adds to it
// fn parse_children<'c, 'a, I, F>(chars: &mut Peekable<I>, mut handle_name: F)
//     where I: Iterator<Item = &'c u8>,
//           F: FnMut([u8; 8])
// {
//     let child_name = parse_name(chars, b',');
//     handle_name(child_name);
//     while let Some(&&c) = chars.peek() {
//         if c == b' ' {
//             chars.next();
//         }
//         let child_name = parse_name(chars, b',');
//         handle_name(child_name);
//     }
// }

fn skip_arrow_to_children<'c, I: Iterator<Item = &'c u8>>(chars: &mut Peekable<I>) {
    if *chars.next().expect("should get -") != b'-' {
        panic!("expected -");
    }
    if *chars.next().expect("should get >") != b'>' {
        panic!("expected >");
    }
    if *chars.next().expect("should get <space>") != b' ' {
        panic!("expected <space>");
    }
}

fn skip_optional_cr<'c, I: Iterator<Item = &'c u8>>(chars: &mut Peekable<I>) {
    if let Some(&&b'\r') = chars.peek() {
        chars.next();
    }
}

fn parse_name<'c, I: Iterator<Item = &'c u8>>(chars: &mut Peekable<I>, end_char: u8) -> [u8; 8] {
    let mut name = [0u8; 8];
    let mut i = 0;
    while let Some(&c) = chars.next() {
        if c == end_char {
            break;
        }
        name[i] = c;
        i += 1;
    }
    name
}

fn parse_weight<'c, I: Iterator<Item = &'c u8>>(chars: &mut Peekable<I>) -> u32 {
    let (weight_chars, count) = parse_weight_chars(chars);
    weight_chars_to_u32(&weight_chars, count)
}

fn parse_weight_chars<'c, I: Iterator<Item = &'c u8>>(chars: &mut Peekable<I>) -> ([u8; 8], usize) {
    if *chars.next().expect("should get (") != b'(' {
        panic!("expected (");
    }
    let mut weight_chars = [0u8; 8];
    let mut i = 0;
    while let Some(&c) = chars.next() {
        if c == b')' {
            break;
        }
        weight_chars[i] = c;
        i += 1;
    }
    (weight_chars, i)
}

fn weight_chars_to_u32(weight_chars: &[u8; 8], count: usize) -> u32 {
    let mut weight = 0u32;
    let mut i = 0;
    while i < count {
        weight += ascii_digit_to_number(weight_chars[i]) * power_10(count - i - 1);
        i += 1;
    }
    weight
}

fn ascii_digit_to_number(digit: u8) -> u32 {
    (digit - 48) as u32
}

fn power_10(p: usize) -> u32 {
    let mut p = p as u32;
    let mut result = 1u32;
    while p > 0 {
        result *= 10;
        p -= 1;
    }
    result
}

// Do this?
// #[derive(Clone, Copy, Eq, PartialEq)]
// struct Name([u8; 8]);

struct Node<'a> {
    name: [u8; 8],
    weight: Cell<u32>,
    children: RefCell<Vec<&'a Node<'a>>>
}

impl<'a> Node<'a> {
    fn new(name: [u8; 8]) -> Node<'a> {
        Node {
            name: name,
            weight: Cell::new(0),
            children: RefCell::new(Vec::new())
        }
    }

    fn new_with_weight(name: [u8; 8], weight: u32) -> Node<'a> {
        Node {
            name: name,
            weight: Cell::new(weight),
            children: RefCell::new(Vec::new())
        }
    }

    // fn new_with_weight_and_children(name: [u8; 8], weight: u32, children: Vec<&'a Node<'a>>) -> Node<'a> {
    //     Node {
    //         name: name,
    //         weight: Cell::new(weight),
    //         children: RefCell::new(children)
    //     }
    // }
}

//use std::str::FromStr;
/*
fn test_stuff() {
    //let full_description = fs::read_to_string("C:\\Users\\jankes\\Documents\\AdventOfCode\\2017\\7\\tower.txt").unwrap();

    // see all the names
    {
        let mut names = full_description
        .lines()
        .map(|line| {
            line.split_whitespace().next().unwrap()
        })
        .map(|name| name.len())
        .collect::<Vec<_>>();
        
        names.sort();

        for &name_len in names.iter() {
            println!("{}", name_len);
        }
    }

    // see all the weights
    {
        let mut weights = full_description

        .lines()

        .map(|line| line.split_whitespace().skip(1).next().unwrap())

        .map(|weight| weight.trim_left_matches('(').trim_right_matches(')'))

        .map(|weight| u32::from_str(weight).unwrap())

        .collect::<Vec<_>>();

        weights.sort();

        for &w in weights.iter() {
            println!("{}", w);
        }
    }
}
*/