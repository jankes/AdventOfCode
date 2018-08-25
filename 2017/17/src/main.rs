// my input
const STEP: u32 = 386;

fn main() {
    part_1();
    part_2();
}

fn part_2() {    
    let mut pos = 1u32;
    let mut first_val = 0u32;
    for i in 1..=50_000_002u32 {

        if pos == 1 {
            first_val = i;
        }

        // works 
        pos = (pos + STEP + 1) % (i + 1);
        if pos == 0 {
            pos = i + 1;
        }

        // also works
        // pos += STEP + 1;
        // if pos > i + 1 {
        //     pos = pos % (i + 1);
        // }

        if 50_000_000 - 2 <= i {
            println!("part 2: iteration = {}, first_value = {}", i, first_val);
        }
    }
}

fn part_1() {
    let mut buffer = Vec::<u16>::with_capacity(2018);
    buffer.push(0);

    let mut pos = 0u16;
    for i in 1..=2017u16 {
        for _ in 0..STEP {
            pos = move_next(pos, &buffer);
        }
        buffer.insert((pos + 1) as usize, i);
        pos += 1;
    }

    let value_after_last_insert = buffer[move_next(pos, &buffer) as usize];
    println!("part 1: value after last insert = {}", value_after_last_insert);

    fn move_next(current: u16, buffer: &Vec<u16>) -> u16 {
        let next = current + 1;
        if (next as usize) < buffer.len() {
            next
        } else {
            0
        }
    }
}

