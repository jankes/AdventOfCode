use std::fs::File;
use std::io::Read;
use std::path::Path;

// uses axial coordinate system shown at https://www.redblobgames.com/grids/hexagons/ 

fn main() {
    let directions = read_to_vec("C:\\Users\\jankes\\Documents\\AdventOfCode\\2017\\11\\steps.txt");
    let steps = compute_fewest_steps(&directions);

    println!("fewest number of steps needed: {}", steps);
}

fn compute_fewest_steps(raw_directions: &[u8]) -> i32 {
    let (q, r) = raw_directions
        .split(|&c| c == b',')
        .map(direction_to_axial_coord)
        .fold((0i32, 0i32), sum_axial_coords);

    distance_from_origin(q, r)
}

fn distance_from_origin(q: i32, r: i32) -> i32 {
    (q.wrapping_abs() + r.wrapping_abs() + i32::wrapping_abs(-q - r)) / 2
}

fn sum_axial_coords(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    let (a_q, a_r) = a;
    let (b_q, b_r) = b;
    (a_q + b_q, a_r + b_r)
}

fn direction_to_axial_coord(direction: &[u8]) -> (i32, i32) {
    match direction {
        b"n"  => (0, -1),
        b"ne" => (1, -1),
        b"se" => (1, 0),
        b"s"  => (0, 1),
        b"sw" => (-1, 1),
        b"nw" => (-1, 0),
        _ => panic!("unexpected direction")
    }
}

fn read_to_vec<P: AsRef<Path>>(file: P) -> Vec<u8> {
    let mut raw_data = Vec::<u8>::with_capacity(22000);
    File::open(file).unwrap()
    .read_to_end(&mut raw_data).expect("should be able to read file to memory");
    raw_data
}

#[cfg(test)]
mod tests {
    use super::compute_steps;

    #[test]
    fn example_1_n() {
        assert_eq!(3, compute_steps(b"n,n,n"));
    }

    #[test]
    fn example_1_ne() {
        assert_eq!(3, compute_steps(b"ne,ne,ne"));
    }

    #[test]
    fn example_1_se() {
        assert_eq!(3, compute_steps(b"se,se,se"));
    }

    #[test]
    fn example_1_s() {
        assert_eq!(3, compute_steps(b"s,s,s"));
    }

    #[test]
    fn example_1_sw() {
        assert_eq!(3, compute_steps(b"sw,sw,sw"));
    }

    #[test]
    fn example_1_nw() {
        assert_eq!(3, compute_steps(b"nw,nw,nw"));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, compute_steps(b"ne,ne,sw,sw"));
    }

    #[test]
    fn example_3() {
        assert_eq!(2, compute_steps(b"ne,ne,s,s"));
    }

    #[test]
    fn example_4() {
        assert_eq!(3, compute_steps(b"se,sw,se,sw,sw"));
    }
}