use std::{
    fs::OpenOptions,
    io::Read,
    path::Path,
    str::FromStr
};

fn main() {
    let input = read_input("C:\\Users\\jankes\\Documents\\AdventOfCode\\2015\\24\\pkgs.txt");
    let weights = input
                  .lines()
                  .map(|line| i16::from_str(line).unwrap())
                  .collect::<Vec<i16>>();

    let total = weights.iter().sum::<i16>();
    println!("total = {}", total); // total = 1548; (total / 3) = 516; (total / 4) = 387

    println!("part 1:");
    find_best_qe(&weights, 6, 516); // no choices of 5 weights adds up to 516 

    println!("part 2:");
    find_best_qe(&weights, 5, 387); // no choices of 4 weights adds up to 387
}

fn find_best_qe(weights: &[i16], weight_count: usize, necessary_weight_sum: i16) {
    let mut best_qe = i64::max_value();

    choose(weight_count, weights.len(), |indexes| {
        let (sum, product) = calculate_statistics(weights, indexes);
        if sum == necessary_weight_sum && product <= best_qe {
            best_qe = product;
            print_choice(weights, indexes);
            println!("qe = {}", best_qe);
        }
    });    
}

fn print_choice(weights: &[i16], indexes: &[usize]) {
    for weight_index in indexes {
        print!("{} ", weights[*weight_index]);
    }
}

fn calculate_statistics(weights: &[i16], indexes: &[usize]) -> (i16, i64) {
    let mut sum = 0i16;
    let mut product = 1i64;
    for i in indexes {
        let w = weights[*i];
        sum += w;
        product *= w as i64;
    }
    (sum, product)
}

fn choose<F: FnMut(&[usize])>(choose_count: usize, total_item_count: usize, mut indexes_callback: F) {
    let mut stack = Vec::<usize>::with_capacity(choose_count);
    let mut indexes = (0..choose_count).collect::<Vec<_>>();
    loop {
        indexes_callback(&indexes);

        for z in 0..choose_count {
            indexes[choose_count - 1 - z] += 1;
            if indexes[choose_count - 1 - z] != total_item_count - z {
                break;
            }
            if z == choose_count - 1 {
                return;
            }
            stack.push(choose_count - 1 - z);
        }

        while let Some(x) = stack.pop() {
            indexes[x] = indexes[x - 1] + 1;
        }
    }
}

fn read_input<P: AsRef<Path>>(path: P) -> String {
	let mut file = OpenOptions::new()
	               .read(true)
				   .open(path)
				   .expect("failed to open input file");

	let mut input = String::new();

	file.read_to_string(&mut input).expect("failed to read input file");
	input
}