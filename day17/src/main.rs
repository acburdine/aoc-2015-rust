use std::fs;

fn combinations(
    containers: &Vec<u32>,
    max: u32,
    initial: u32,
    count: usize,
    skip: usize,
    groups: &mut Vec<usize>,
) {
    if initial == max {
        groups.push(count);
        return;
    }

    if initial > max || skip >= containers.len() {
        return;
    }

    containers
        .iter()
        .enumerate()
        .skip(skip)
        .for_each(|(i, c)| combinations(containers, max, c + initial, count + 1, i + 1, groups));
}

#[test]
fn test_combinations() {
    let testc = vec![20, 15, 10, 5, 5];
    let mut result: Vec<usize> = Vec::new();
    combinations(&testc, 25, 0, 0, 0, &mut result);

    assert_eq!(4, result.len());

    let min = result.iter().min().unwrap();
    assert_eq!(2, *min);
    assert_eq!(3, result.iter().filter(|c| *c == min).count());
}

const MAX: u32 = 150;

fn main() {
    let input = fs::read_to_string("inputs/day17.txt").unwrap();
    let containers: Vec<u32> = input
        .lines()
        .map(|l| l.trim().parse::<u32>().unwrap())
        .collect();

    let mut result: Vec<usize> = Vec::new();
    combinations(&containers, MAX, 0, 0, 0, &mut result);

    println!("number of combinations: {}", result.len());

    let min = result.iter().min().unwrap();
    println!(
        "number of min combinations: {}",
        result.iter().filter(|c| *c == min).count()
    );
}
