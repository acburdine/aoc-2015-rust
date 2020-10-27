extern crate regex;

use regex::{Captures, Regex};
use std::fs;

enum State {
    Resting,
    Flying,
}

struct Deer {
    speed: u32,
    fly_time: u32,
    rest_time: u32,

    state: State,
    remaining: u32,
    distance: u32,
    points: u32,
}

impl Deer {
    fn new(caps: Captures) -> Deer {
        let fly_time: u32 = caps.name("fly").unwrap().as_str().parse().unwrap();

        Deer {
            speed: caps.name("speed").unwrap().as_str().parse().unwrap(),
            fly_time,
            rest_time: caps.name("rest").unwrap().as_str().parse().unwrap(),

            state: State::Flying,
            remaining: fly_time,
            distance: 0,
            points: 0,
        }
    }

    fn tick(&mut self) {
        if let State::Flying = self.state {
            self.distance += self.speed;
        }

        self.remaining -= 1;
        if self.remaining == 0 {
            match self.state {
                State::Flying => {
                    self.state = State::Resting;
                    self.remaining = self.rest_time;
                }
                State::Resting => {
                    self.state = State::Flying;
                    self.remaining = self.fly_time;
                }
            }
        }
    }

    fn give_point(&mut self) {
        self.points += 1;
    }
}

const RACE_TIME: u32 = 2503;

fn main() {
    let input = fs::read_to_string("inputs/day14.txt").unwrap();
    let re = Regex::new(r"^(?:[A-Za-z]+) can fly (?P<speed>\d+) km/s for (?P<fly>\d+) seconds, but then must rest for (?P<rest>\d+) seconds\.$").unwrap();

    let mut deer: Vec<Deer> = input
        .lines()
        .map(|l| Deer::new(re.captures(l).unwrap()))
        .collect();

    let mut timer: u32 = 0;
    while timer < RACE_TIME {
        for d in deer.iter_mut() {
            d.tick()
        }

        let lead_dist = deer.iter().max_by_key(|d| d.distance).unwrap().distance;
        deer.iter_mut()
            .filter(|d| d.distance.eq(&lead_dist))
            .for_each(|d| d.give_point());

        timer += 1;
    }

    deer.sort_by(|a, b| b.distance.cmp(&a.distance));
    println!("winning distance: {}", deer.first().unwrap().distance);

    deer.sort_by(|a, b| b.points.cmp(&a.points));
    println!("winning points: {}", deer.first().unwrap().points);
}
