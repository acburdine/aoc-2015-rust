use std::collections::HashMap;
use std::fs;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn next(&self, inst: char) -> Point {
        match inst {
            '^' => Point {
                x: self.x,
                y: self.y + 1,
            },
            'v' => Point {
                x: self.x,
                y: self.y - 1,
            },
            '>' => Point {
                x: self.x + 1,
                y: self.y,
            },
            '<' => Point {
                x: self.x - 1,
                y: self.y,
            },
            _ => panic!("unrecongized instruction: {}", inst),
        }
    }
}

fn iteration(map: &mut HashMap<Point, u32>, mut pt: Point, inst: char) -> Point {
    pt = pt.next(inst);

    let mut v: u32 = 1;
    {
        match map.get(&pt) {
            Some(val) => v = *val + 1,
            None => (),
        }
    }
    map.insert(pt, v);

    return pt;
}

fn main() {
    let input = fs::read_to_string("inputs/day3.txt").unwrap();

    let mut year1: HashMap<Point, u32> = HashMap::new();
    let mut pt = Point { x: 0, y: 0 };

    year1.insert(pt, 1);

    for c in input.trim().chars() {
        pt = iteration(&mut year1, pt, c);
    }

    println!("year 1: {}", year1.len());

    let mut year2: HashMap<Point, u32> = HashMap::new();
    let mut pt2a = Point { x: 0, y: 0 };
    let mut pt2b = Point { x: 0, y: 0 };

    year2.insert(pt2a, 2);

    for (i, c) in input.trim().char_indices() {
        if i % 2 == 0 {
            pt2a = iteration(&mut year2, pt2a, c);
        } else {
            pt2b = iteration(&mut year2, pt2b, c);
        }
    }

    println!("year 2: {}", year2.len());
}
