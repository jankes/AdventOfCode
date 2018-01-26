use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;
use std::str::{self, FromStr};

fn main() {
    let input = read_input("C:\\Users\\jankes\\Documents\\AdventOfCode\\2015\\17\\input.txt");
    let sizes = parse_input(&input);

    let (min_count, max_count) = calculate_limits(sizes.clone());    
    println!("Number of containers is greater than or equal to {}", min_count);
    println!("Number of containers is less than {}", max_count);

    /*
    let test = vec!(b'A', b'B', b'C', b'D', b'E');
    enum_combinations(&test, 4, |combination| {
        println!("{}", str::from_utf8(combination).unwrap());
    })
    */
    part_1(&sizes, min_count, max_count);
    part_2(&sizes, min_count, max_count);
}

fn part_2(sizes: &[u16], min_container_count: usize, max_container_count: usize) {
    let mut found_min_count = false;
    for n in min_container_count..max_container_count {
        let mut target_count_for_n = 0usize;
        enum_combinations(sizes, n, |sizes_chosen| {
            let total = sizes_chosen.iter().sum::<u16>();
            if total == 150 {
                target_count_for_n += 1;
                found_min_count = true;
            }
        });
        if found_min_count {
            println!("Minimum number of containers needed to get 150 liters is {}", n);
            println!("There are {} different ways to fill {} containers to get 150 liters", target_count_for_n, n);    
            return;
        }
    }
}

fn part_1(sizes: &[u16], min_container_count: usize, max_container_count: usize) {
    let mut target_count = 0usize;
    for container_count in min_container_count..max_container_count {
        enum_combinations(sizes, container_count, |sizes_chosen| {
            let total = sizes_chosen.iter().sum::<u16>();
            if total == 150 {
                target_count += 1;
            }
        });
    }
    println!("there are {} combinations of containers that exactly fill 150 liters", target_count);
}

fn enum_combinations<T, F>(list: &[T], n: usize, mut f: F)
    where T:Copy, T:Clone, F:FnMut(&[T]) {
    let mut current = Vec::with_capacity(n);
    enum_combinations_helper(list, n, 0, &mut current, &mut f);
}

fn enum_combinations_helper<T, F>(list: &[T], n: usize, left: usize, current: &mut Vec<T>, f: &mut F)
    where T:Copy, T:Clone, F:FnMut(&[T]) {
    if n == 2 {
        let mut left = left;
        while left < list.len() - 1 {
            current.push(list[left]);
            let mut right = left + 1;
            while right < list.len() {
                current.push(list[right]);
                f(current);
                current.pop();
                right += 1;
            }
            left += 1;
            current.pop();
        }
    } else {
        let mut left = left;
        while left + n - 1 < list.len() {
            current.push(list[left]);
            enum_combinations_helper(list, n - 1, left + 1, current, f);
            current.pop();
            left += 1;
        }
    }
}

// (20 choose 4) + (20 choose 5) + (20 choose 6) + (20 choose 7) + (20 choose 8) + (20 choose 9) + (20 choose 10) + (20 choose 11) =
// 783275

fn calculate_limits(mut sizes: Vec<u16>) -> (usize, usize) {
    let mut sizes_desc = { sizes.sort_unstable_by(|a, b| b.cmp(a)); sizes };

    let min_count = container_count_to_fill_target(&sizes_desc, 150);

    let sizes_asc = { sizes_desc.reverse(); sizes_desc };

    let max_count = container_count_to_fill_target(&sizes_asc, 150);

    (min_count, max_count)
}

fn container_count_to_fill_target(sizes_ordered: &[u16], target: u16) -> usize {
    let mut total = 0u16;
    let mut index = 0usize;
    while let Some(size) = sizes_ordered.get(index) {
        total += size;
        if total >= target {
            break;
        }
        index += 1;
    }
    index + 1
}

fn parse_input(input: &str) -> Vec<u16> {
    let mut sizes = Vec::<u16>::with_capacity(20); 
    for line in input.lines() {
        let size = u16::from_str(line).expect("should have integer that fits in u16 per line");
        sizes.push(size);
    }
    sizes
}

fn read_input<P: AsRef<Path>>(path: P) -> String {
    let mut file = OpenOptions::new()
	               .read(true)
				   .open(path)
                   .expect("expect to be able to open input file for reading");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("should be able to read input file");
    s
}
