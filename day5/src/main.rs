use std::collections::HashSet;
use std::fs;

const VOWELS: &str = "aeiou";
const NOT_ALLOWED: [&str; 4] = ["ab", "cd", "pq", "xy"];

fn is_nice(s: &str) -> bool {
    let mut vowel_ct = 0;
    let mut dbl_ltr = false;

    for pt in NOT_ALLOWED.iter() {
        if s.contains(pt) {
            return false;
        }
    }

    let mut chars = s.chars();

    // remove first character from iterator
    chars.next();

    for c in s.chars() {
        if VOWELS.contains(c) {
            vowel_ct += 1;
        }

        // this *should* always be the character immediately after
        // c, since we've removed the first character prior to beginning the loop
        match chars.next() {
            Some(next) => {
                if next == c {
                    dbl_ltr = true;
                }
            }
            None => break,
        }
    }

    return vowel_ct >= 3 && dbl_ltr;
}

fn is_nice_v2(s: &str) -> bool {
    let mut pair_rpt = false;
    let mut skip_rpt = false;

    let mut set: HashSet<[char; 2]> = HashSet::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        let next1 = match chars.get(i + 1) {
            Some(c1) => *c1,
            None => break,
        };
        let next2 = chars.get(i + 2);

        let key = [c, next1];
        {
            if set.contains(&key) && i > 0 && chars[i - 1] != c {
                pair_rpt = true;
            }
        }
        set.insert(key);

        match next2 {
            Some(c2) => {
                if next1 != c && *c2 == c {
                    skip_rpt = true;
                }
            }
            None => (),
        }

        if pair_rpt && skip_rpt {
            return true;
        }

        i += 1;
    }

    return false;
}

fn main() {
    let input = fs::read_to_string("inputs/day5.txt").unwrap();
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let mut nice_count = 0;
    let mut nice_v2_count = 0;

    for l in lines {
        if is_nice(l) {
            nice_count += 1;
        }

        if is_nice_v2(l) {
            nice_v2_count += 1;
        }
    }

    println!("number of nice words: {}", nice_count);
    println!("number of v2 nice words: {}", nice_v2_count);
}
