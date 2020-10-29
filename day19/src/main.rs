use std::collections::{HashMap, HashSet};
use std::fs;

fn split_replacement(line: &str, reverse: bool) -> (&str, &str) {
    let parts: Vec<&str> = line.splitn(2, " => ").collect();
    if reverse {
        return (parts[1], parts[0]);
    }

    (parts[0], parts[1])
}

fn replace(s: &str, i: usize, from: &str, to: &str) -> String {
    let before: String = s.chars().take(i).collect();
    let after: String = s.chars().skip(i + from.len()).collect();
    before + to + &after
}

fn distinct_molecules(replacements: &str, molecule: &str) -> usize {
    let set: HashSet<String> = replacements
        .trim()
        .lines()
        .map(|l| split_replacement(l, false))
        .map(|(from, to)| {
            molecule
                .match_indices(from)
                .map(move |(i, _)| replace(molecule, i, from, to))
        })
        .flatten()
        .collect();

    set.len()
}

#[test]
fn test_distinct_molecules() {
    let replacements = r"H => HO
H => OH
O => HH";

    assert_eq!(4, distinct_molecules(replacements, "HOH"));
    assert_eq!(7, distinct_molecules(replacements, "HOHOHO"));
}

fn decompose(molecule: &str, replacements: &HashMap<&str, &str>) -> Option<usize> {
    let mut steps: usize = 0;
    let mut decomposed = String::from(molecule);

    while decomposed.ne("e") {
        if !replacements.iter().any(|(k, _)| decomposed.contains(k)) {
            return None;
        }

        for (k, v) in replacements {
            if !decomposed.contains(k) {
                continue;
            }

            steps += 1;
            decomposed = decomposed.replacen(k, v, 1);

            if decomposed.eq("e") {
                break;
            }
        }
    }

    Some(steps)
}

fn build_molecule(replacements: &str, molecule: &str) -> usize {
    let replace_map = replacements
        .trim()
        .lines()
        .map(|l| split_replacement(l, true))
        .collect::<HashMap<&str, &str>>();

    loop {
        match decompose(molecule, &replace_map) {
            Some(s) => {
                return s;
            }
            None => (),
        }
    }
}

#[test]
fn test_build_molecule() {
    let replacements = r"e => H
e => O
H => HO
H => OH
O => HH";

    assert_eq!(3, build_molecule(replacements, "HOH"));
    assert_eq!(6, build_molecule(replacements, "HOHOHO"));
}

fn main() {
    let input = fs::read_to_string("inputs/day19.txt").unwrap();
    let input_parts: Vec<&str> = input.splitn(2, "\n\n").collect();

    println!(
        "distinct molecules: {}",
        distinct_molecules(input_parts[0], input_parts[1])
    );

    println!(
        "min steps for molecule: {}",
        build_molecule(input_parts[0], input_parts[1])
    );
}
