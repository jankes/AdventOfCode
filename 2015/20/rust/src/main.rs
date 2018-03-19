// 2259441 is too high

fn main() {
    let target = 34000000u32;

    // part 1
    // let mut houses = (0..1000000u32).map(|_| 0).collect::<Vec<u32>>();
    // deliver_presents_part_1(&mut houses);
    // print_lowest_house_num_with_n_presents(&houses, target);

    // part 2
    let mut houses = (0..1000000u32).map(|_| 0).collect::<Vec<u32>>();
    deliver_presents_part_2(&mut houses);
    print_lowest_house_num_with_n_presents(&houses, target);
}

fn print_lowest_house_num_with_n_presents(houses: &[u32], n: u32) {
    /*
    for (house_num, present_count) in houses.iter().enumerate() {
        if *present_count >= target {
            println!("{}", house_num);
            break;
        }
    }
    */
    match houses.iter().enumerate()
                .filter(|&(_, present_count)| *present_count >= n)
                .take(1)
                .next() {
        Some((house_num, _)) => println!("house number {} is first to get at least {} presents", house_num, n),
        None                 => println!("no house gets {} presents", n)
    };
}

fn deliver_presents_part_1(houses: &mut [u32]) {
    for elf in 1..houses.len() {
        let mut house = elf;
        while house < houses.len() {
            houses[house] += 10u32 * (elf as u32);
            house += elf;
        }
    }
}

fn deliver_presents_part_2(houses: &mut [u32]) {
    for elf in 1..houses.len() {
        let mut house = elf;
        let mut count = 0;
        while count <= 50 && house < houses.len() {
            houses[house] += 11u32 * (elf as u32);
            house += elf;
            count += 1;
        }
    }
}