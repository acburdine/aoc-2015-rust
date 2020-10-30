const FIRST_NUMBER: usize = 20151125;
const MULTIPLIER: usize = 252533;
const MODULO: usize = 33554393;

fn next_number(n: usize) -> usize {
    (n * MULTIPLIER) % MODULO
}

fn value_at(row: usize, column: usize) -> usize {
    let mut n = FIRST_NUMBER;
    let mut max_row = 1;
    let mut current_row = 0;
    let mut current_column = 0;

    loop {
        while current_row > 0 {
            n = next_number(n);

            if current_row == row && current_column == column {
                return n;
            }

            current_row -= 1;
            current_column += 1;
        }

        max_row += 1;
        current_row = max_row;
        current_column = 1;
    }
}

fn main() {
    println!("value at {}, {}: {}", 2981, 3075, value_at(2981, 3075));
}
