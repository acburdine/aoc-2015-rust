use std::collections::HashMap;
use std::fs;

enum MatchVal {
    Greater(u32),
    Equal(u32),
    Less(u32),
}

impl MatchVal {
    fn matches(&self, v: u32, always_equal: bool) -> bool {
        if always_equal {
            return match self {
                MatchVal::Greater(u) => v == *u,
                MatchVal::Equal(u) => v == *u,
                MatchVal::Less(u) => v == *u,
            };
        }

        match self {
            MatchVal::Greater(u) => v > *u,
            MatchVal::Equal(u) => v == *u,
            MatchVal::Less(u) => v < *u,
        }
    }
}

fn matching_aunt(
    info: &HashMap<&str, MatchVal>,
    aunts: &Vec<Vec<(&str, u32)>>,
    always_eq: bool,
) -> usize {
    let (i, _) = aunts
        .iter()
        .enumerate()
        .find(|(_, a)| {
            a.iter()
                .all(|(k, v)| info.get(k).unwrap().matches(*v, always_eq))
        })
        .unwrap();
    i + 1
}

fn main() {
    let mut given_info: HashMap<&str, MatchVal> = HashMap::new();
    given_info.insert("children", MatchVal::Equal(3));
    given_info.insert("cats", MatchVal::Greater(7));
    given_info.insert("samoyeds", MatchVal::Equal(2));
    given_info.insert("pomeranians", MatchVal::Less(3));
    given_info.insert("akitas", MatchVal::Equal(0));
    given_info.insert("vizslas", MatchVal::Equal(0));
    given_info.insert("goldfish", MatchVal::Less(5));
    given_info.insert("trees", MatchVal::Greater(3));
    given_info.insert("cars", MatchVal::Equal(2));
    given_info.insert("perfumes", MatchVal::Equal(1));

    let input = fs::read_to_string("inputs/day16.txt").unwrap();
    let aunts: Vec<Vec<(&str, u32)>> = input
        .lines()
        .map(|l| {
            l.trim()
                .splitn(2, ": ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|prop| {
                    let kv: Vec<&str> = prop.splitn(2, ": ").collect();
                    (kv[0], kv[1].parse::<u32>().unwrap())
                })
                .collect()
        })
        .collect();

    println!(
        "aunt 1st part: {}",
        matching_aunt(&given_info, &aunts, true)
    );
    println!(
        "aunt 2nd part: {}",
        matching_aunt(&given_info, &aunts, false)
    );
}
