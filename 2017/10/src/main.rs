
fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let lengths = [102u8, 255, 99, 252, 200, 24, 219, 57, 103, 2, 226, 254, 1, 0, 69, 216];
    let mut list = [0u8; 256];
    knot_hash(&mut list, &lengths, 1);
    println!("part 1: first two numbers in list: {} {}", list[0], list[1]);
}

fn part_2() {
    let lengths = b"102,255,99,252,200,24,219,57,103,2,226,254,1,0,69,216\x11\x1F\x49\x2F\x17";
    let mut list = [0u8; 256];
    knot_hash(&mut list, lengths, 64);
    let dense = [
        xor(&list[0..16]),
        xor(&list[16..32]),
        xor(&list[32..48]),
        xor(&list[48..64]),
        xor(&list[64..80]),
        xor(&list[80..96]),
        xor(&list[96..112]),
        xor(&list[112..128]),
        xor(&list[128..144]),
        xor(&list[144..160]),
        xor(&list[160..176]),
        xor(&list[176..192]),
        xor(&list[192..208]),
        xor(&list[208..224]),
        xor(&list[224..240]),
        xor(&list[240..256]),
    ];
    println!("part 2: {:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
             dense[0], dense[1], dense[2], dense[3], dense[4], dense[5], dense[6], dense[7],
             dense[8], dense[9], dense[10], dense[11], dense[12], dense[13], dense[14], dense[15]);
}

fn xor(list: &[u8]) -> u8 {
    let mut iter = list.iter();
    let first = *iter.next().unwrap();
    iter.fold(first, |result, &next| result ^ next)
}

fn knot_hash(list: &mut [u8; 256], lengths: &[u8], rounds: u8) {
    for i in 0u8..=255 {
        list[i as usize] = i;
    }
    let mut current_position = 0u8;
    let mut skip_size = 0u8;
    for _ in 0..rounds {
        for length in lengths {        
            reverse(&mut *list, current_position, *length);
            current_position = current_position.wrapping_add(*length);
            current_position = current_position.wrapping_add(skip_size);
            skip_size = skip_size.wrapping_add(1);
        }
    }
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