extern crate regex;

use regex::Regex;
use std::cell::Cell;
use std::collections::HashMap;
use std::fs;

#[derive(Copy, Clone, Debug)]
enum Operation {
    And,
    Lshift,
    Rshift,
    Not,
    Or,
}

impl Operation {
    fn run(&self, a: u16, b: u16) -> u16 {
        match self {
            Operation::And => a & b,
            Operation::Lshift => a << b,
            Operation::Rshift => a >> b,
            Operation::Not => !b,
            Operation::Or => a | b,
        }
    }

    fn from(s: &str) -> Operation {
        match s {
            "AND" => Operation::And,
            "LSHIFT" => Operation::Lshift,
            "RSHIFT" => Operation::Rshift,
            "NOT" => Operation::Not,
            "OR" => Operation::Or,
            _ => panic!("unsupported operation: {}", s),
        }
    }
}

#[derive(Debug)]
enum Input {
    Wire(String),
    Number(u16),
}

impl Input {
    fn get_value(&self, circuit: &HashMap<String, Wire>) -> Option<u16> {
        match self {
            Input::Number(i) => Some(*i),
            Input::Wire(s) => match circuit.get(s) {
                None => panic!("missing wire in circuit: {}", s),
                Some(w) => w.get_output(circuit),
            },
        }
    }

    fn from(s: &str) -> Input {
        if let Ok(n) = s.parse::<u16>() {
            return Input::Number(n);
        }

        return Input::Wire(s.to_string());
    }
}

#[derive(Debug)]
struct Wire {
    a: Option<Input>,
    b: Input,
    op: Option<Operation>,
    cached: Cell<Option<u16>>,
    override_input: Cell<Option<u16>>,
}

impl Wire {
    fn get_output(&self, circuit: &HashMap<String, Wire>) -> Option<u16> {
        if self.cached.get().is_some() {
            return self.cached.get();
        }

        if self.override_input.get().is_some() {
            return self.override_input.get();
        }

        if let Some(op) = self.op {
            if self.a.is_none() {
                match op {
                    Operation::Not => (),
                    _ => panic!("unexpected empty input a"),
                };
            }

            let a_val = self
                .a
                .as_ref()
                .unwrap_or(&Input::Number(0))
                .get_value(circuit);
            let b_val = self.b.get_value(circuit);

            if a_val.is_none() || b_val.is_none() {
                return None;
            }

            let result = Some(op.run(a_val.unwrap(), b_val.unwrap()));
            self.cached.set(result);

            return result;
        } else {
            let result = self.b.get_value(circuit);
            self.cached.set(result);
            return result;
        }
    }

    fn reset(&self) {
        self.cached.set(None)
    }

    fn set_override_input(&self, input: u16) {
        self.override_input.set(Some(input));
    }

    fn new(a: Option<Input>, b: Input, op: Option<Operation>) -> Wire {
        Wire {
            a,
            b,
            op,
            cached: Cell::new(None),
            override_input: Cell::new(None),
        }
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day7.txt").unwrap();
    let lines: Vec<&str> = input.trim().split('\n').collect();

    let re = Regex::new(
        r"^(?:(?P<a>\d{1,5}|[a-z]{1,2}) )?(?:(?P<op>[A-Z]+) )?(?:(?P<b>\d{1,5}|[a-z]{1,2})) -> (?P<out>[a-z]{1,2})",
    ).unwrap();

    let mut circuit: HashMap<String, Wire> = HashMap::new();

    for l in lines {
        let caps = re.captures(l.trim()).unwrap();
        let a = match caps.name("a") {
            Some(v) => Some(Input::from(v.as_str())),
            None => None,
        };
        let op = match caps.name("op") {
            Some(v) => Some(Operation::from(v.as_str())),
            None => None,
        };
        let b = Input::from(caps.name("b").unwrap().as_str());
        let out = caps.name("out").unwrap().as_str();

        circuit.insert(out.to_string(), Wire::new(a, b, op));
    }

    let mut a_val = circuit.get("a").unwrap().get_output(&circuit).unwrap();
    println!("initial wire a: {}", a_val);

    for (out, w) in circuit.iter() {
        w.reset();

        if out == "b" {
            w.set_override_input(a_val);
        }
    }

    a_val = circuit.get("a").unwrap().get_output(&circuit).unwrap();
    println!("second value a: {}", a_val);
}
