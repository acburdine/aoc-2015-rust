use std::collections::HashSet;
use std::fs;

fn min_size(data: &Vec<usize>, target: usize) -> usize {
    let mut packages = data.clone();
    packages.sort();
    packages.reverse();

    let mut sum = 0;
    packages
        .into_iter()
        .take_while(|p| {
            sum += p;
            sum < target
        })
        .count()
        + 1
}

fn combinations(remaining: Vec<usize>, current: Vec<usize>, n: usize) -> Vec<Vec<usize>> {
    if current.len() == n {
        return vec![current];
    }

    remaining
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, p)| {
            let dup = remaining.clone().into_iter().skip(i + 1).collect();
            let mut new = current.clone();
            new.push(p);

            combinations(dup, new, n)
        })
        .flatten()
        .collect()
}

#[test]
fn test_combinations() {
    let data = vec![1, 2, 3, 4];
    assert_eq!(
        vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4],
        ],
        combinations(data, vec![], 2)
    );
}

fn entanglement(packages: &Vec<usize>, groups: usize) -> usize {
    let split_count: usize = packages.iter().sum::<usize>() / groups;

    let mut set: HashSet<(usize, usize)> = HashSet::new();
    for n in min_size(&packages, split_count)..(packages.len() / groups) {
        for combo in combinations(packages.clone(), vec![], n) {
            let mut sum: usize = 0;
            let mut entanglement: usize = 1;

            for (i, package) in combo.into_iter().enumerate() {
                sum += package;
                entanglement *= package;

                if sum == split_count {
                    set.insert((i + 1, entanglement));
                } else if sum > split_count {
                    break;
                }
            }
        }

        // we've already found a match, exit
        if set.len() > 0 {
            break;
        }
    }

    let min_size = set.iter().map(|(i, _)| i).min().unwrap();
    *set.iter()
        .filter(|(i, _)| i == min_size)
        .map(|(_, e)| e)
        .min()
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("inputs/day24.txt").unwrap();
    let packages: Vec<usize> = input.trim().lines().map(|p| p.parse().unwrap()).collect();

    println!(
        "min entanglement (3 groups): {}",
        entanglement(&packages, 3)
    );

    println!(
        "min entanglement (4 groups): {}",
        entanglement(&packages, 4)
    );
}
