// Puzzle input
const TARGET_ADDRESS: u32 = 265149;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum State {
    Right, Up, Left, Down
}

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let mut current_address = 1u32;
    let (mut x, mut y) = (0i32, 0i32);
    for state in spiral() {
        if current_address == TARGET_ADDRESS {
            println!("part 1: takes {} steps", i32::abs(x) + i32::abs(y));
            break;
        }
        current_address += 1;
        match state {
            State::Right => x += 1,
            State::Up    => y += 1,
            State::Left  => x -= 1,
            State::Down  => y -= 1
        };
    }
}

fn part_2() {
    // let mut count = 0;

    let mut mem = Memory::new(5);
    mem.write(0, 0, 1);

    let (mut x, mut y) = (0i32, 0i32);
    for state in spiral() {
        match state {
            State::Right => x += 1,
            State::Up    => y += 1,
            State::Left  => x -= 1,
            State::Down  => y -= 1
        };
        let value =
        mem.read(x - 1, y + 1) + mem.read(x,     y + 1) + mem.read(x + 1, y + 1) +
        mem.read(x - 1, y    ) + mem.read(x + 1, y    ) +
        mem.read(x - 1, y - 1) + mem.read(x,     y - 1) + mem.read(x + 1, y - 1);
        if value > TARGET_ADDRESS {
            println!("part 2: first value greater than {} is {}", TARGET_ADDRESS, value);
            break;
        }
        mem.write(x, y, value);

        // count += 1;
        // if count == 24 {
        //     let mut y2 = 2;
        //     while y2 >= -2 {
        //         for x2 in -2..=2 {
        //             print!("{} ", mem.read(x2, y2));
        //         }
        //         println!();
        //         y2 -= 1;
        //     }
            
        //     for y2 in 2..=-2 {
        //         for x2 in -2..=2 {
        //             print!("{} ", mem.read(x2, y2));
        //         }
        //         println!();
        //     }
        //     break;
        // }
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Memory {
    size: i32,
    squares: Vec<u32>
}

impl Memory {
    fn new(size: i32) -> Memory {
        if size < 0 {
            panic!("can't create negative sized memory!");
        }
        let mut squares = Vec::with_capacity(((2*size + 1) * (2*size + 1)) as usize);
        for _ in 0..squares.capacity() {
            squares.push(0);
        }
        Memory {
            size: size,
            squares: squares
        }
    }

    fn read(&self, x: i32, y: i32) -> u32 {
        if i32::abs(x) > self.size || i32::abs(y) > self.size {
            return 0;
        }
        self.squares[self.address_to_index(x, y)]
    }

    fn write(&mut self, x: i32, y: i32, value: u32) {
        self.grow_if_necessary(x, y);
        let index = self.address_to_index(x, y);
        self.squares[index] = value;
    }

    fn address_to_index(&self, x: i32, y: i32) -> usize {
        let index = (2*self.size + 1)*(y + self.size) + (x + self.size);
        index as usize
    }

    fn grow_if_necessary(&mut self, x: i32, y: i32) {
        if i32::abs(x) > self.size || i32::abs(y) > self.size {
            let new_size = 3 * i32::max(i32::abs(x), i32::abs(y)) / 2;
            let mut new_memory = Memory::new(new_size);
            for y in -self.size..=self.size {
                for x in -self.size..=self.size {
                    new_memory.write(x, y, self.read(x, y));
                }
            }
            std::mem::swap(&mut *self, &mut new_memory);
        }
    }
}

fn spiral() -> SpiralIter {
    SpiralIter::new()
}

struct SpiralIter {
    step_count: u32,
    right: u32,
    up: u32,
    left: u32,
    down: u32,
    state: State
}

impl SpiralIter {
    fn new() -> SpiralIter {
        SpiralIter {
            step_count: 0,
            right: 1,
            up: 1,
            left: 2,
            down: 2,
            state: State::Right
        }
    }
}

impl Iterator for SpiralIter {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        let current_state = self.state;
        self.step_count += 1;
        match current_state {
            State::Right => {
                if self.step_count == self.right {
                    self.step_count = 0;
                    self.right += 2;
                    self.state = State::Up;
                }
            },
            State::Up => {
                if self.step_count == self.up {
                    self.step_count = 0;
                    self.up += 2;
                    self.state = State::Left;
                }
            },
            State::Left => {
                if self.step_count == self.left {
                    self.step_count = 0;
                    self.left += 2;
                    self.state = State::Down;
                }
            },
            State::Down => {
                if self.step_count == self.down {
                    self.step_count = 0;
                    self.down += 2;
                    self.state = State::Right;
                }
            }
        };
        Some(current_state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral() {
        let actual = spiral().take(17).collect::<Vec<State>>();
        let expected = vec!(State::Right,
                            State::Up,
                            State::Left, State::Left,
                            State::Down, State::Down,
                            State::Right, State::Right, State::Right,
                            State::Up, State::Up, State::Up,
                            State::Left, State::Left, State::Left, State::Left,
                            State::Down);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_memory_grow() {
        let mut mem = Memory::new(1);
        let mut i = 0;
        for y in -1..=1 {
            for x in -1..=1 {
                mem.write(x, y, i);
                i += 1;
            }
        }

        let original_mem = mem.clone();
        mem.write(-2, 0, 100);
        assert_eq!(mem.size, 3);
        for y in -3..=3 {
            for x in -3..=3 {
                if (x, y) == (-2, 0) {
                    assert_eq!(mem.read(x, y), 100);
                } else if -1 <= x && x <= 1 && -1 <= y && y <= 1 {
                    assert_eq!(mem.read(x, y), original_mem.read(x, y));
                } else {
                    assert_eq!(mem.read(x, y), 0);
                }
            }
        }
    }
}