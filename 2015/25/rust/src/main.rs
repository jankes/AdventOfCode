fn main() {
    let mut row = 1u16;
    let mut col = 1u16;
    let mut next_row = 2;
    let mut value = 20151125u64;

    loop {
        if row == 2981 && col == 3075 {
            println!("value = {}", value);
            break;
        }

        value = (value * 252533) % 33554393;

        row -= 1;
        if row == 0 {
            row = next_row;
            col = 1;
            next_row += 1;
        } else {
            col += 1;
        }
    }
}
