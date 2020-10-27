use std::fs;

struct Dimension {
    length: u32,
    width: u32,
    height: u32,
}

impl Dimension {
    fn from(s: &str) -> Option<Dimension> {
        let split: Vec<&str> = s.splitn(3, 'x').collect();
        if split.len() != 3 {
            return None;
        }

        return Some(Dimension {
            length: split[0].parse().unwrap(),
            width: split[1].parse().unwrap(),
            height: split[2].parse().unwrap(),
        });
    }

    fn area_smallest_side(&self) -> u32 {
        let sides: [u32; 3] = [
            self.width * self.length,
            self.length * self.height,
            self.height * self.width,
        ];

        return *sides.iter().min().unwrap();
    }

    fn paper_sqft(&self) -> u32 {
        (2 * self.length * self.width)
            + (2 * self.width * self.height)
            + (2 * self.height * self.length)
            + self.area_smallest_side()
    }

    fn vol(&self) -> u32 {
        self.length * self.height * self.width
    }

    fn ribbon_ft(&self) -> u32 {
        let sides: [u32; 3] = [
            (2 * self.width) + (2 * self.length),
            (2 * self.length) + (2 * self.height),
            (2 * self.height) + (2 * self.width),
        ];

        return self.vol() + *sides.iter().min().unwrap();
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day2.txt").unwrap();
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let mut paper_amt = 0;
    let mut ribbon_amt = 0;

    for l in lines {
        let dim = Dimension::from(l).unwrap();

        paper_amt += dim.paper_sqft();
        ribbon_amt += dim.ribbon_ft();
    }

    println!("total sq ft of paper: {}", paper_amt);
    println!("total ft of ribbon: {}", ribbon_amt);
}
