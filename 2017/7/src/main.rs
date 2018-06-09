extern crate typed_arena;

use typed_arena::Arena;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::iter::Peekable;
use std::path::Path;

fn main() {
    let mut raw_data = Vec::<u8>::with_capacity(35500);

    File::open("C:\\Users\\jankes\\Documents\\AdventOfCode\\2017\\7\\example.txt").unwrap()
    .read_to_end(&mut raw_data).expect("should be able to read file to memory");

    let arena = Arena::<Node>::new();
    let mut nodes = HashMap::<[u8; 8], &Node>::new();

    for line in raw_data.split(|&c| c == b'\n') {
        let mut chars = line.iter().peekable();

        let name = parse_name(&mut chars, b' ');
        let weight = parse_weight(&mut chars);
        skip_optional_cr(&mut chars);

        match chars.next() {
            None => {
                nodes.entry(name)
                     .and_modify(|node| node.weight.set(weight))
                     .or_insert(arena.alloc(Node::new_with_weight(name, weight)));
            },
            Some(&b' ') => {
                skip_arrow_to_children(&mut chars);

                let children = parse_children(&mut chars, |child_name|
                    nodes.entry(child_name)
                         .or_insert(arena.alloc(Node::new(child_name)))
                );

                let node = nodes.entry(name).or_insert(arena.alloc(Node::new(name)));
                node.weight.set(weight);
                node.children.replace(children);
                for child in node.children.borrow().iter() {
                    child.parent.set(Some(node));
                }
            },
            _ => panic!("unexpected char at end of line"),
        };
    }

    let mut root = *nodes.values().next().expect("must have at least one node");
    while let Some(parent) = root.parent.get() {
        root = parent;
    }
    println!("root is {}", std::str::from_utf8(&root.name).unwrap());

    print_tree(root, 0);
}

fn print_tree<'a>(node: &'a Node<'a>, level: u16) {
    for _ in 0..level {
        print!("  ");
    }
    println!("{} ({})", std::str::from_utf8(&node.name).unwrap(), node.weight.get());
    
    for &child in node.children.borrow().iter() {
        print_tree(child, level + 1);
    }
}

fn parse_tree<'a, P: AsRef<Path>>(arena: &'a Arena<Node<'a>>, file: P) -> &'a Node<'a> {
    let mut raw_data = Vec::<u8>::with_capacity(35500);

    File::open(file).unwrap()
    .read_to_end(&mut raw_data).expect("should be able to read file to memory");

    let mut nodes = HashMap::<[u8; 8], &Node>::new();

    for line in raw_data.split(|&c| c == b'\n') {
        let mut chars = line.iter().peekable();

        let name = parse_name(&mut chars, b' ');
        let weight = parse_weight(&mut chars);
        skip_optional_cr(&mut chars);

        match chars.next() {
            None => {
                nodes.entry(name)
                     .and_modify(|node| node.weight.set(weight))
                     .or_insert(arena.alloc(Node::new_with_weight(name, weight)));
            },
            Some(&b' ') => {
                skip_arrow_to_children(&mut chars);

                // let mut children = Vec::new();
                // let child_name = parse_name(&mut chars, b',');
                // children.push(*nodes.entry(child_name)
                //                     .or_insert(arena.alloc(Node::new(child_name))));
                // while let Some(&&c) = chars.peek() {
                //     if c == b' ' {
                //         chars.next();
                //     }
                //     let child_name = parse_name(&mut chars, b',');
                //     children.push(*nodes.entry(child_name)
                //                         .or_insert(arena.alloc(Node::new(child_name))));
                // }

                // // let children = parse_children(&mut chars, |child_name|
                // //     nodes.entry(child_name)
                // //          .or_insert(arena.alloc(Node::new(child_name)))
                // // );

                // let node = nodes.entry(name).or_insert(arena.alloc(Node::new(name)));
                // node.weight.set(weight);
                // node.children.replace(children);
                // for child in node.children.borrow().iter() {
                //     child.parent.set(Some(node));
                // }

                let children = parse_children_2(&mut chars, arena, &mut nodes);

                let node = nodes.entry(name).or_insert(arena.alloc(Node::new(name)));
                node.weight.set(weight);
                node.children.replace(children);
                for child in node.children.borrow().iter() {
                    child.parent.set(Some(node));
                }

            },
            _ => panic!("unexpected char at end of line"),
        };
    }

    let mut root = *nodes.values().next().expect("must have at least one node");
    while let Some(parent) = root.parent.get() {
        root = parent;
    }

    root
    // let node_1 = arena.alloc(Node::new(*b"one     "));
    // let node_2 = arena.alloc(Node::new(*b"two     "));
    // let node_3 = arena.alloc(Node::new(*b"three   "));

    // node_1.children.borrow_mut().push(node_2);
    // node_2.children.borrow_mut().push(node_3);

    // node_3.parent.set(Some(node_2));
    // node_2.parent.set(Some(node_1));

    //node_1

    // Tree {
    //     nodes: arena,
    //     root: None
    // }
}

fn parse_children_2<'c, 'a, I>(chars: &mut Peekable<I>,
                               arena: &'a Arena<Node<'a>>,
                               nodes: &mut HashMap<[u8; 8], &'a Node<'a>>) -> Vec<&'a Node<'a>>
    where I: Iterator<Item = &'c u8>
{
    let mut children = Vec::new();

    let child_name = parse_name(chars, b',');
    children.push(*nodes.entry(child_name)
                        .or_insert(arena.alloc(Node::new(child_name))));

    while let Some(&&c) = chars.peek() {
        if c == b' ' {
            chars.next();
        }
        let child_name = parse_name(chars, b',');
        children.push(*nodes.entry(child_name)
                            .or_insert(arena.alloc(Node::new(child_name))));
    }

    children
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
        if c == end_char || c == b'\r' {
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

struct Tree<'a> {
    nodes: Arena<Node<'a>>,
    root: Option<&'a Node<'a>>
}

struct Node<'a> {
    name: [u8; 8],
    weight: Cell<u32>,
    parent: Cell<Option<&'a Node<'a>>>,
    children: RefCell<Vec<&'a Node<'a>>>
}

impl<'a> Node<'a> {
    fn new(name: [u8; 8]) -> Node<'a> {
        Node {
            name: name,
            weight: Cell::new(0),
            parent: Cell::new(None),
            children: RefCell::new(Vec::new())
        }
    }

    fn new_with_weight(name: [u8; 8], weight: u32) -> Node<'a> {
        Node {
            name: name,
            weight: Cell::new(weight),
            parent: Cell::new(None),
            children: RefCell::new(Vec::new())
        }
    }
}
