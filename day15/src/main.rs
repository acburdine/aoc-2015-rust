extern crate regex;

const PATTERN: &str = r"^(?:[A-Za-z]+): capacity (?P<cap>[0-9-]+), durability (?P<dur>[0-9-]+), flavor (?P<flav>[0-9-]+), texture (?P<text>[0-9-]+), calories (?P<cal>[0-9-]+)$";

use regex::Regex;
use std::fs;

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn new(line: &str) -> Ingredient {
        let caps = Regex::new(PATTERN).unwrap().captures(line).unwrap();

        Ingredient {
            capacity: caps.name("cap").unwrap().as_str().parse().unwrap(),
            durability: caps.name("dur").unwrap().as_str().parse().unwrap(),
            flavor: caps.name("flav").unwrap().as_str().parse().unwrap(),
            texture: caps.name("text").unwrap().as_str().parse().unwrap(),
            calories: caps.name("cal").unwrap().as_str().parse().unwrap(),
        }
    }
}

fn score(ingredients: &Vec<Ingredient>, amounts: &Vec<i32>, cals: i32) -> i32 {
    let mut capacity_sum: i32 = ingredients
        .iter()
        .zip(amounts)
        .map(|(i, a)| i.capacity * a)
        .sum();
    if capacity_sum < 0 {
        capacity_sum = 0;
    }

    let mut durability_sum: i32 = ingredients
        .iter()
        .zip(amounts)
        .map(|(i, a)| i.durability * a)
        .sum();
    if durability_sum < 0 {
        durability_sum = 0;
    }

    let mut flavor_sum: i32 = ingredients
        .iter()
        .zip(amounts)
        .map(|(i, a)| i.flavor * a)
        .sum();
    if flavor_sum < 0 {
        flavor_sum = 0;
    }

    let mut texture_sum: i32 = ingredients
        .iter()
        .zip(amounts)
        .map(|(i, a)| i.texture * a)
        .sum();
    if texture_sum < 0 {
        texture_sum = 0;
    }

    let cal_sum: i32 = ingredients
        .iter()
        .zip(amounts)
        .map(|(i, a)| i.calories * a)
        .sum();
    if cals > 0 && cals != cal_sum {
        return 0;
    }

    capacity_sum * durability_sum * flavor_sum * texture_sum
}

const MAX_INGREDIENTS: i32 = 100;
const MAX_CALS: i32 = 500;

fn main() {
    let input = fs::read_to_string("inputs/day15.txt").unwrap();
    let ingredients: Vec<Ingredient> = input.lines().map(|l| Ingredient::new(l)).collect();

    let mut amounts: Vec<Vec<i32>> = Vec::with_capacity(100 ^ 4);

    // TODO: I would absolutely love to come up with a way to generate dynamic nested for loops but
    // for simplicity I'm gonna just hardcode nested loops
    let mut i: i32 = 0;
    while i < MAX_INGREDIENTS {
        let mut j: i32 = 0;

        while j < MAX_INGREDIENTS - i {
            let mut k: i32 = 0;

            while k < (MAX_INGREDIENTS - i - j) {
                amounts.push(vec![i, j, k, (MAX_INGREDIENTS - i - j - k)]);
                k += 1;
            }

            j += 1;
        }

        i += 1;
    }

    let max_score = amounts
        .iter()
        .map(|amt| score(&ingredients, amt, 0))
        .max()
        .unwrap();

    println!("max score: {}", max_score);

    let max_score_500_cals = amounts
        .iter()
        .map(|amt| score(&ingredients, amt, MAX_CALS))
        .max()
        .unwrap();

    println!("max score @ 500 cals: {}", max_score_500_cals);
}
