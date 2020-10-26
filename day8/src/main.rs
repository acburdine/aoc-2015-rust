use std::fs;

// part 1 soln inspired by https://github.com/jasonbrackman/advent_of_code_2015/blob/master/src/day_08.rs
fn main() {
    let input = fs::read_to_string("inputs/day8.txt").unwrap();

    let mut code_chars = 0;
    let mut new_chars = 0;
    let mut mem_chars = 0;

    for l in input.lines() {
        code_chars += l.trim().chars().count();
        new_chars += l.trim().escape_default().count() + 2; // account for start + end '"'

        let mut skip = 0;
        for c in l.trim().chars() {
            if c == ' ' {
                continue;
            }

            if skip > 0 {
                if skip == 1 {
                    if c == 'x' {
                        skip += 2;
                        mem_chars += 1;
                    } else if c == '"' || c == '\\' {
                        mem_chars += 1;
                    }
                }

                skip -= 1;
            } else if c == '\\' {
                skip = 1;
            } else if c != '"' {
                mem_chars += 1;
            }
        }
    }

    println!("part 1 result: {}", code_chars - mem_chars);
    println!("part 2 result: {}", new_chars - code_chars);
}
