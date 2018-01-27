use std::{fmt, mem};

pub fn run(grid_string: &str) {
    println!("--------------------------");
    println!("with function pointers ...");
    println!("--------------------------");
    example("EXAMPLE part 1:", part1_get);
    example("EXAMPLE part 2:", part2_get);
    do_steps("PART 1", grid_string, part1_get);
    do_steps("PART 2", grid_string, part2_get);
}

fn do_steps(message: &str, grid_string: &str, getter: GetFn) {
    let mut grid = Grid::parse(getter, grid_string);
    let mut temp = Grid::new(getter, 100);

    let (mut current, mut next) = (&mut grid, &mut temp);
    for _ in 0..100 {
        Grid::update(current, next);
        mem::swap(&mut current, &mut next);
    }
    println!("{}", message);
    println!("Ater 100 steps, there are {} lights on", current.count_total_on());
}

fn example(message: &str, getter: GetFn) {
    let mut test = String::with_capacity(36);
    test += ".#.#.#\r\n";
    test += "...##.\r\n";
    test += "#....#\r\n";
    test += "..#...\r\n";
    test += "#.#..#\r\n";
    test += "####..\r\n";

    let mut temp = Grid::new(getter, 6);
    let mut grid = Grid::parse(getter, &test);

    set_to_self(&mut grid, 1, 1);
    set_to_self(&mut grid, 1, 6);
    set_to_self(&mut grid, 6, 1);
    set_to_self(&mut grid, 6, 6);

    println!("{}", message);
    println!("initial\r\n{}", grid);

    let (mut current, mut next) = (&mut grid, &mut temp);
    for i in 0..4 {
        Grid::update(current, next);
        println!("{}.\r\n{}", i + 1, next);
        mem::swap(&mut current, &mut next);
    }
    println!("after 4 steps, grid is:");
    println!("{}", current);
    println!("lights on = {}", current.count_total_on());
}

fn set_to_self(grid: &mut Grid, row: usize, col: usize) {
    let state = grid.get(row, col);
    grid.set(row, col, state);
}

type GetFn = fn(&Grid, usize, usize) -> bool; 

struct Grid {
    getter: GetFn,
    size: usize,
    cells: Vec<bool>
}

impl Grid {
    fn new(getter: GetFn, size: usize) -> Grid {
        Grid {
            getter: getter,
            size: size,
            cells: Grid::filled_vec(size * size, false)
        }
    }

    fn filled_vec(size: usize, value: bool) -> Vec<bool> {
        let mut v = Vec::with_capacity(size);
        v.resize(size, value);
        v
    }

    fn parse(getter: GetFn, cells: &str) -> Grid {
        let bytes = cells.as_bytes();
        Grid {
            getter: getter,
            size: Grid::find_size(bytes),
            cells: bytes.iter()
                        .filter(|&&b| b != b'\r')
                        .filter(|&&b| b != b'\n')
                        .map(|&b| if b == b'#' { true } else { false })
                        .collect::<Vec<bool>>()
        }
    }

    fn find_size(bytes: &[u8]) -> usize {
        for (i, &b) in bytes.iter().enumerate() {
            if b == b'\r' || b == b'\n' {
                return i;
            }
        }
        panic!("grid rows must be separated by CR or CRLF");
    }

    fn count_total_on(&self) -> i32 {
        let mut count = 0;
        for row in 1..self.size + 1 {
            for col in 1..self.size + 1 {
                if self.get(row, col) {
                    count += 1;
                }
            }
        }
        count
    }

    fn update(current: &Grid, next: &mut Grid) {
        for row in 1..current.size + 1 {
            for col in 1..current.size + 1 {
                let neighboors_on_count = current.count_neighboors_on(row, col);
                let next_state = if current.get(row, col) {
                                     neighboors_on_count == 2 || neighboors_on_count == 3
                                 } else {
                                     neighboors_on_count == 3
                                 };
                next.set(row, col, next_state);
            }
        }
    }

    fn count_neighboors_on(&self, row: usize, col: usize) -> i32 {
        let mut neighboors_on_count = 0;
        if self.get(row - 1, col - 1) {
            neighboors_on_count += 1;
        }
        if self.get(row - 1, col) {
            neighboors_on_count += 1;
        }
        if self.get(row - 1, col + 1) {
            neighboors_on_count += 1;
        }
        if self.get(row, col - 1) {
            neighboors_on_count += 1;
        }
        if self.get(row, col + 1) {
            neighboors_on_count += 1;
        }
        if self.get(row + 1, col - 1) {
            neighboors_on_count += 1;
        }
        if self.get(row + 1, col) {
            neighboors_on_count += 1;
        }
        if self.get(row + 1, col + 1) {
            neighboors_on_count += 1;
        }
        neighboors_on_count
    }

    fn get(&self, row: usize, col: usize) -> bool {
        (self.getter)(&self, row, col)
    }

    fn set(&mut self, row: usize, col: usize, state: bool) {
        self.cells[(self.size * (row - 1)) + (col - 1)] = state;
    }
}

fn part1_get(grid: &Grid, row: usize, col: usize) -> bool {
    if is_out_of_bounds(grid.size, row, col) {
        false
    } else {
        grid.cells[(grid.size * (row - 1)) + (col - 1)]
    }
}

fn part2_get(grid: &Grid, row: usize, col: usize) -> bool {
    if is_out_of_bounds(grid.size, row, col) {
        false
    } else if is_corner(grid.size, row, col) {
        true
    } else {
        grid.cells[(grid.size * (row - 1)) + (col - 1)]
    }
}

fn is_out_of_bounds(size: usize, row: usize, col: usize) -> bool {
    row == 0 || col == 0 || row == size + 1 || col == size + 1
}

fn is_corner(size: usize, row: usize, col: usize) -> bool {
    (row == 1 && col == 1) ||
    (row == 1 && col == size) ||
    (row == size && col == 1) ||
    (row == size && col == size)
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 1..self.size + 1 {
            for col in 1..self.size + 1 {
                let chr = if self.get(row, col) {
                              "#"
                          } else {
                              "."
                          };
                write!(f, "{} ", chr)?;
            }
            write!(f, "\r\n")?;
        }
        Ok(())
    }
}