use std::collections::HashMap;

// solution taken from https://github.com/petertseng/adventofcode-rb-2015/commit/65a3aac32bb699da4a6ecd0b3839edb8c8522fae#diff-563e8198dfa6b02d0d88c668b64cafa5a21834488d3e22e863ba03d42d6a7cc7

const INPUT: usize = 33100000;

fn gifts(mult: usize, limit: Option<usize>) -> usize {
    let num_elves = INPUT / mult;
    let mut best_case = num_elves;
    let mut houses: HashMap<usize, usize> = HashMap::new();

    for elf in 1..num_elves {
        let gift_nums: Vec<usize>;
        if let Some(l) = limit {
            gift_nums = (elf..num_elves).step_by(elf).take(l).collect();
        } else {
            gift_nums = (elf..num_elves).step_by(elf).collect();
        }

        for (i, n) in gift_nums.into_iter().enumerate() {
            let house = houses.entry(n).and_modify(|h| *h += elf).or_insert(elf);
            if *house >= num_elves {
                best_case = vec![best_case, n].into_iter().min().unwrap();
                if i == 0 {
                    return best_case;
                }
            }
        }
    }

    best_case
}

fn main() {
    println!("min house: {}", gifts(10, None));
    println!("min house pt 2: {}", gifts(11, Some(50)));
}
