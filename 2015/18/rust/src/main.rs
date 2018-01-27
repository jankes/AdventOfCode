use std::fmt;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

fn main() {
    example();
/*
    let grid_string = read_input("C:\\Users\\jankes\\Documents\\AdventOfCode\\2015\\18\\input.txt");
    let mut grid = Grid::parse(&grid_string);
    let mut temp = Grid::new(100);

    let (mut current, mut next) = (&mut grid, &mut temp);
    for _ in 0..100 {
        Grid::update(current, next);
        std::mem::swap(&mut current, &mut next);
    }
    println!("after 100 steps, there are {} lights on", current.count_total_on());
*/
}

fn example() {
    let mut test = String::with_capacity(36);
    test += ".#.#.#\r\n";
    test += "...##.\r\n";
    test += "#....#\r\n";
    test += "..#...\r\n";
    test += "#.#..#\r\n";
    test += "####..\r\n";
    let mut grid = Grid::<Part1>::parse(&test);
/*
    let mut temp = Grid::new(6);

    println!("initial\r\n{}", grid);

    let (mut current, mut next) = (&mut grid, &mut temp);
    for i in 0..4 {
        Grid::update(current, next);
        println!("{}.\r\n{}", i + 1, next);
        std::mem::swap(&mut current, &mut next);
    }
    println!("after 4 steps, grid is:");
    println!("{}", current);
*/
}

trait Get {
    //fn get<G: Get>(grid: &Grid<G>, row: usize, col: usize) -> bool;
    fn get<G: Get>(grid: &Grid, row: usize, col: usize) -> bool;
}

struct Grid /*<G: Get>*/ {
    //getter: G,
    size: usize,
    cells: Vec<bool>
}

impl<G: Get> Grid /*<G>*/ {
    fn new(/*getter: G,*/ size: usize) -> Grid /*<G>*/ {
        Grid {
            /*getter: getter,*/
            size: size,
            cells: Grid::<G>::filled_vec(size * size, false)
        }
    }

    fn filled_vec(size: usize, value: bool) -> Vec<bool> {
        let mut v = Vec::with_capacity(size);
        v.resize(size, value);
        v
    }

    fn parse(/*getter: G,*/ cells: &str) -> Grid /*<G>*/ {
        let bytes = cells.as_bytes();
        Grid {
            /*getter: getter,*/
            size: Grid::<G>::find_size(bytes),
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
        self.cells.iter().map(|&c| if c { 1 } else { 0 })
                  .sum()
    }

    fn update(current: &Grid /*<G>*/, next: &mut Grid /*<G>*/) {
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
        /*
        if row == 0 || col == 0 || row == self.size + 1 || col == self.size + 1 {
            false
        } else {
            self.cells[(self.size * (row - 1)) + (col - 1)]
        }
        */
        G::get(&self, row, col)
    }

    fn set(&mut self, row: usize, col: usize, state: bool) {
        self.cells[(self.size * (row - 1)) + (col - 1)] = state;
    }
}

struct Part1();

impl Get for Part1 {
    fn get<G: Get>(grid: &Grid /*<G>*/, row: usize, col: usize) -> bool {
        if row == 0 || col == 0 || row == grid.size + 1 || col == grid.size + 1 {
            false
        } else {
            grid.cells[(grid.size * (row - 1)) + (col - 1)]
        }
    }
}

impl /*<G: Get>*/ fmt::Display for Grid /*<G>*/ {
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

fn read_input<P: AsRef<Path>>(path: P) -> String {
    let mut file = OpenOptions::new()
	               .read(true)
				   .open(path)
                   .expect("expect to be able to open input file for reading");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("should be able to read input file");
    s
}