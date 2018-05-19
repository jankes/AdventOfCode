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
    for (x, y) in spiral() {
        if current_address == TARGET_ADDRESS {
            println!("part 1: takes {} steps", i16::abs(x) + i16::abs(y));
            break;
        }
        current_address += 1;
    }
}

fn part_2() {
    let mut mem = Memory::new(2);
    mem.write(0, 0, 1);

    for (x, y) in spiral() {
        let value =
        mem.read(x - 1, y + 1) + mem.read(x,     y + 1) + mem.read(x + 1, y + 1) +
        mem.read(x - 1, y    ) + mem.read(x + 1, y    ) +
        mem.read(x - 1, y - 1) + mem.read(x,     y - 1) + mem.read(x + 1, y - 1);
        if value > TARGET_ADDRESS {
            println!("part 2: first value greater than {} is {}", TARGET_ADDRESS, value);
            break;
        }
        mem.write(x, y, value);
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Memory {
    size: i16,
    squares: Vec<u32>
}

impl Memory {
    fn new(size: i16) -> Memory {
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

    fn read(&self, x: i16, y: i16) -> u32 {
        if self.is_address_out_of_bounds(x, y) {
            return 0;
        }
        self.squares[self.address_to_index(x, y)]
    }

    fn write(&mut self, x: i16, y: i16, value: u32) {
        self.grow_if_necessary(x, y);
        let index = self.address_to_index(x, y);
        self.squares[index] = value;
    }

    fn address_to_index(&self, x: i16, y: i16) -> usize {
        let size = self.size as i32;
        let index = (2*size + 1)*(y as i32 + size) + (x as i32 + size);
        index as usize
    }

    fn grow_if_necessary(&mut self, x: i16, y: i16) {
        if self.is_address_out_of_bounds(x, y) {
            let new_size = Memory::get_new_size(x, y);
            let mut new_memory = Memory::new(new_size);
            self.copy_to(&mut new_memory);
            std::mem::swap(&mut *self, &mut new_memory);
        }
    }

    fn copy_to(&self, other: &mut Memory) {
        for y in -self.size..=self.size {
            for x in -self.size..=self.size {
                other.write(x, y, self.read(x, y));
            }
        }
    }

    fn get_new_size(x: i16, y: i16) -> i16 {
        let bound = i16::max(i16::abs(x), i16::abs(y)) as i32;
        (3 * bound / 2) as i16
    }

    fn is_address_out_of_bounds(&self, x: i16, y: i16) -> bool {
        i16::abs(x) > self.size || i16::abs(y) > self.size
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
    state: State,
    x: i16,
    y: i16
}

impl SpiralIter {
    fn new() -> SpiralIter {
        SpiralIter {
            step_count: 0,
            right: 1,
            up: 1,
            left: 2,
            down: 2,
            state: State::Right,
            x: 0,
            y: 0
        }
    }
}

impl Iterator for SpiralIter {
    type Item = (i16, i16);

    fn next(&mut self) -> Option<Self::Item> {
        let current_state = self.state;
        self.step_count += 1;
        match current_state {
            State::Right => {
                self.x += 1;
                if self.step_count == self.right {
                    self.step_count = 0;
                    self.right += 2;
                    self.state = State::Up;
                }
            },
            State::Up => {
                self.y += 1;
                if self.step_count == self.up {
                    self.step_count = 0;
                    self.up += 2;
                    self.state = State::Left;
                }
            },
            State::Left => {
                self.x -= 1;
                if self.step_count == self.left {
                    self.step_count = 0;
                    self.left += 2;
                    self.state = State::Down;
                }
            },
            State::Down => {
                self.y -= 1;
                if self.step_count == self.down {
                    self.step_count = 0;
                    self.down += 2;
                    self.state = State::Right;
                }
            }
        };
        Some((self.x, self.y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral() {
        let actual = spiral().take(17).collect::<Vec<(i16, i16)>>();
        let expected = vec!((1, 0),
                            (1, 1),
                            (0, 1), (-1, 1),
                            (-1, 0), (-1, -1),
                            (0, -1), (1, -1), (2, -1),
                            (2, 0), (2, 1), (2, 2),
                            (1, 2), (0, 2), (-1, 2), (-2, 2),
                            (-2, 1));
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