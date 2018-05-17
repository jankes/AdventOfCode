// Puzzle input
const TARGET_ADDRESS: u32 = 265149;

enum State {
    Right, Up, Left, Down
}

fn main() {
    let (x, y) = read_address(TARGET_ADDRESS);
    let total_steps = i32::abs(x) + i32::abs(y);
    println!("{} steps are needed", total_steps);
}

fn read_address(address: u32) -> (i32, i32) {
    let mut x = 0i32;
    let mut y = 0i32;

    let mut current_direction_step_count = 0u32;
    let mut right = 1u32;
    let mut up = 1u32;
    let mut left  = 2u32;
    let mut down = 2u32;

    let mut current_address = 1u32;
    let mut state = State::Right;

    while current_address != address {
        current_address += 1;
        current_direction_step_count += 1;
        match state {
            State::Right => {
                x += 1;
                if current_direction_step_count == right {
                    current_direction_step_count = 0;
                    right += 2;
                    state = State::Up;
                }
            },
            State::Up => {
                y += 1;
                if current_direction_step_count == up {
                    current_direction_step_count = 0;
                    up += 2;
                    state = State::Left;
                }
            },
            State::Left => {
                x -= 1;
                if current_direction_step_count == left {
                    current_direction_step_count = 0;
                    left += 2;
                    state = State::Down;
                }
            },
            State::Down => {
                y -= 1;
                if current_direction_step_count == down {
                    current_direction_step_count = 0;
                    down += 2;
                    state = State::Right;
                }
            }
        };
    }
    (x, y)
}

#[cfg(test)]
mod tests {
    use super::read_address;

    #[test]
    fn test_read() {
        let cases = [(1,  ( 0,  0)),
                     (2,  ( 1,  0)),
                     (3,  ( 1,  1)),
                     (4,  ( 0,  1)),
                     (5,  (-1,  1)),
                     (6,  (-1,  0)),
                     (7,  (-1, -1)),
                     (8,  ( 0, -1)),
                     (9,  ( 1, -1)),
                     (10, ( 2, -1)),
                     (23, ( 0, -2))];
        
        for case in cases.iter() {
            let &(address, (expected_x, expected_y)) = case;
            let (actual_x, actual_y) = read_address(address);
            if (actual_x, actual_y) != (expected_x, expected_y) {
                panic!("read address {} failed. Expected ({}, {}) but got ({}, {})", address, expected_x, expected_y, actual_x, actual_y);
            }
        }
    }

    #[test]
    fn test_far_read() {
        let (x, y) = read_address(1024);
        assert_eq!(i32::abs(x) + i32::abs(y), 31);
    }
}