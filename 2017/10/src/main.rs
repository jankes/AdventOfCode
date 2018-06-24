
fn main() {
    part_1();
}

fn part_1() {
    let mut list = [0u8; 256];
    for i in 0u8..=255 {
        list[i as usize] = i;
    }

    let lengths = [102u8, 255, 99, 252, 200, 24, 219, 57, 103, 2, 226, 254, 1, 0, 69, 216];

    let mut current_position = 0u8;
    let mut skip_size = 0u8;
    for length in &lengths {        
        reverse(&mut list, current_position, *length);
        current_position = current_position.wrapping_add(*length);
        current_position = current_position.wrapping_add(skip_size);
        skip_size += 1;
    }

    println!("first two numbers in list: {} {}", list[0], list[1]);
}

fn reverse(list: &mut [u8], mut start_index: u8, length: u8) {
    let count = length / 2;
    let mut end_index = start_index.wrapping_add(length).wrapping_sub(1);
    for _ in 0..count {
        list.swap(start_index as usize, end_index as usize);
        start_index = start_index.wrapping_add(1);
        end_index = end_index.wrapping_sub(1);
    }
}

// fn example() {
//     let mut list = [0u8, 1, 2, 3, 4];
//     let lengths = [3u8, 4, 1, 5];

//     let mut current_position = 0u8;
//     let mut skip_size = 0u8;

//     println!("initial:");
//     print_stats(&list, current_position);

//     for length in &lengths {
//         reverse_mod5(&mut list, current_position, *length);
//         current_position = wrapping_add(current_position, *length);
//         current_position = wrapping_add(current_position, skip_size);
//         skip_size += 1;

//         println!("{}:", skip_size);
//         print_stats(&list, current_position);
//     }
// }

// fn print_stats(list: &[u8], current_position: u8) {
//     println!("current position = {}", current_position);
//     print!("list = ");
//     for number in list {
//         print!("{} ", number);
//     }
//     println!();
// }

// fn reverse_mod5(list: &mut [u8], mut start_index: u8, length: u8) {
//     let count = length / 2;
//     let mut end_index = wrapping_sub(wrapping_add(start_index, length), 1);
//     for _ in 0..count {
//         list.swap(start_index as usize, end_index as usize);
//         start_index = wrapping_add(start_index, 1);
//         end_index = wrapping_sub(end_index, 1);
//     }
// }

// fn wrapping_add(a: u8, b: u8) -> u8 {
//     (((a as u16) + (b as u16)) % 5u16) as u8
// }

// fn wrapping_sub(a: u8, b: u8) -> u8 {
//     if a >= b {
//         a - b
//     } else {
//         5 - b + a
//     }
// }
