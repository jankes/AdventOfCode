fn main() {
    // my input: 386

    part_1();
    part_2();

    // let mut buffer = Vec::<u32>::with_capacity(50000001);
    // buffer.push(0);

    // let mut pos = 0u32;
    // for i in 1..=30u32 {
    //     for _ in 0..3 {
    //         pos = move_next(pos, &buffer);
    //     }
    //     buffer.insert((pos + 1) as usize, i);
    //     pos += 1;

    //     println!("{} {}", i, pos);
    //     println!("{:?}", buffer);
        
    //     //println!("{}", buffer[1]);
        
    //     // if i % 131072 == 0 {
    //     //     println!("--");
    //     // }
    // }

    // // let value_after_last_insert = buffer[move_next(pos, &buffer) as usize];
    // // println!("part 1: value after last insert = {}", value_after_last_insert);

    // fn move_next(current: u32, buffer: &Vec<u32>) -> u32 {
    //     let next = current + 1;
    //     if (next as usize) < buffer.len() {
    //         next
    //     } else {
    //         0
    //     }
    // }
}

fn part_2() {    
    let step = 386u32;
    let mut pos = 1u32;
    let mut first_val = 0u32;
    for i in 1..=50_000_002u32 {

        if pos == 1 {
            first_val = i;
        }

        // works 
        pos = (pos + step + 1) % (i + 1);
        if pos == 0 {
            pos = i + 1;
        }

        // also works
        // pos += step + 1;
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
        for _ in 0..386 {
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

