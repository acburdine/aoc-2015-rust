extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

fn main() {
    let input = fs::read_to_string("inputs/day6.txt").unwrap();
    let lines: Vec<&str> = input.trim().split('\n').collect();

    let re = Regex::new(r"^(?P<instr>turn on|turn off|toggle) (?P<x1>\d{1,3}),(?P<y1>\d{1,3}) through (?P<x2>\d{1,3}),(?P<y2>\d{1,3})$").unwrap();
    let mut lights: HashMap<Point, bool> = HashMap::with_capacity(1000000); // 1000 x 1000 grid
    let mut lights2: HashMap<Point, u32> = HashMap::with_capacity(1000000);

    for l in lines {
        let caps = re.captures(l.trim()).unwrap();

        let instr = caps.name("instr").unwrap().as_str();
        let x1: u32 = caps.name("x1").unwrap().as_str().parse().unwrap();
        let y1: u32 = caps.name("y1").unwrap().as_str().parse().unwrap();
        let x2: u32 = caps.name("x2").unwrap().as_str().parse().unwrap();
        let y2: u32 = caps.name("y2").unwrap().as_str().parse().unwrap();

        for x in x1..x2 + 1 {
            for y in y1..y2 + 1 {
                let pt = Point { x, y };
                let mut val2: u32 = 0;
                {
                    if let Some(v) = lights2.get(&pt) {
                        val2 = *v;
                    }
                }

                match instr {
                    "turn on" => {
                        lights.insert(pt, true);
                        lights2.insert(pt, val2 + 1);
                    }
                    "turn off" => {
                        lights.insert(pt, false);
                        lights2.insert(
                            pt,
                            match val2.checked_sub(1) {
                                Some(v) => v,
                                None => 0,
                            },
                        );
                    }
                    "toggle" => {
                        let mut val = false;
                        {
                            if let Some(true) = lights.get(&pt) {
                                val = true;
                            }
                        }
                        lights.insert(pt, !val);
                        lights2.insert(pt, val2 + 2);
                    }
                    _ => (),
                }
            }
        }
    }

    let mut lit_count = 0;
    for (_, lit) in lights.iter() {
        if *lit {
            lit_count += 1;
        }
    }
    println!("lit count: {}", lit_count);

    let mut brightness = 0;
    for (_, bright) in lights2.iter() {
        brightness += bright;
    }
    println!("brightness: {}", brightness);
}
