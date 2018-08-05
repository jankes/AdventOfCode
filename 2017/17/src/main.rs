fn main() {
    // my input: 386

    let mut buffer = Vec::<u16>::with_capacity(2018);
    buffer.push(0);

    let mut pos = 0u16;
    for i in 1..=2017u16 {
        for _ in 0..386 {
            pos = move_next(pos, &buffer);
        }
        buffer.insert((pos + 1) as usize, i);
        pos += 1;
    }

    let value_after_last_insert = buffer[move_next(pos, &buffer) as usize];
    println!("part 1: value after last insert = {}", value_after_last_insert);
}

fn move_next(current: u16, buffer: &Vec<u16>) -> u16 {
    let next = current + 1;
    if (next as usize) < buffer.len() {
        next
    } else {
        0
    }
}