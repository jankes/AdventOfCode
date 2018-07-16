fn main() {
    // let example = [knot_hash(b"flqrgnkx-0")[0],
    //                knot_hash(b"flqrgnkx-1")[0],
    //                knot_hash(b"flqrgnkx-2")[0],
    //                knot_hash(b"flqrgnkx-3")[0],
    //                knot_hash(b"flqrgnkx-4")[0],
    //                knot_hash(b"flqrgnkx-5")[0],
    //                knot_hash(b"flqrgnkx-6")[0],
    //                knot_hash(b"flqrgnkx-7")[0]];
    // for &t in test.iter() {
    //     println!("{:02x}", t);
    // }

    let squares_used = 
    (0..128u16)
    .map(|i| get_input_row(i))
    .map(|row| knot_hash(&row))
    .map(|hash| pop_count(&hash))
    .sum::<u16>();

    println!("{} squares are used", squares_used);
}

fn get_input_row(i: u16) -> Vec<u8> {
    let input = b"nbysizxe-";

    let mut row = Vec::<u8>::with_capacity(input.len() + 3);
    row.extend_from_slice(input);
    row.extend_from_slice(i.to_string().as_bytes());
    row
}

fn pop_count(input: &[u8; 16]) -> u16 {
    input.iter()
    .map(|&b| b as u16)
    .map(|b| ((b & 0b10000000) >> 7) +
             ((b & 0b01000000) >> 6) +
             ((b & 0b00100000) >> 5) +
             ((b & 0b00010000) >> 4) +
             ((b & 0b00001000) >> 3) +
             ((b & 0b00000100) >> 2) +
             ((b & 0b00000010) >> 1) +
              (b & 0b00000001)
    ).sum::<u16>()
}

fn knot_hash(input: &[u8]) -> [u8; 16] {
    let input_with_suffix = append_suffix(input);

    let mut sparse = [0u8; 256];
    sparse_hash(&mut sparse, &input_with_suffix, 64);
    let dense = [
        xor(&sparse[0..16]),
        xor(&sparse[16..32]),
        xor(&sparse[32..48]),
        xor(&sparse[48..64]),
        xor(&sparse[64..80]),
        xor(&sparse[80..96]),
        xor(&sparse[96..112]),
        xor(&sparse[112..128]),
        xor(&sparse[128..144]),
        xor(&sparse[144..160]),
        xor(&sparse[160..176]),
        xor(&sparse[176..192]),
        xor(&sparse[192..208]),
        xor(&sparse[208..224]),
        xor(&sparse[224..240]),
        xor(&sparse[240..256]),
    ];
    dense
}

fn append_suffix(input: &[u8]) -> Vec<u8> {
    let suffix = b"\x11\x1F\x49\x2F\x17";
    let mut with_suffix = Vec::with_capacity(input.len() + suffix.len());
    with_suffix.extend_from_slice(input);
    with_suffix.extend_from_slice(suffix);
    with_suffix
}

fn xor(input: &[u8]) -> u8 {
    let mut iter = input.iter();
    let first = *iter.next().unwrap();
    iter.fold(first, |result, &next| result ^ next)
}

fn sparse_hash(output: &mut [u8; 256], lengths: &[u8], rounds: u8) {
    for i in 0u8..=255 {
        output[i as usize] = i;
    }
    let mut current_position = 0u8;
    let mut skip_size = 0u8;
    for _ in 0..rounds {
        for length in lengths {        
            reverse(&mut *output, current_position, *length);
            current_position = current_position.wrapping_add(*length);
            current_position = current_position.wrapping_add(skip_size);
            skip_size = skip_size.wrapping_add(1);
        }
    }
}

fn reverse(output: &mut [u8], mut start_index: u8, length: u8) {
    let count = length / 2;
    let mut end_index = start_index.wrapping_add(length).wrapping_sub(1);
    for _ in 0..count {
        output.swap(start_index as usize, end_index as usize);
        start_index = start_index.wrapping_add(1);
        end_index = end_index.wrapping_sub(1);
    }
}