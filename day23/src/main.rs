use std::fs;

enum Instruction {
    Half(char),
    Triple(char),
    Increment(char),
    Jump(isize),
    JumpIfEven(char, isize),
    JumpIfOne(char, isize),
}

impl Instruction {
    fn init(s: &str) -> Instruction {
        let parts: Vec<&str> = s.splitn(2, " ").collect();
        match parts[0] {
            "hlf" => Instruction::Half(parts[1].parse().unwrap()),
            "tpl" => Instruction::Triple(parts[1].parse().unwrap()),
            "inc" => Instruction::Increment(parts[1].parse().unwrap()),
            "jmp" => Instruction::Jump(parts[1].parse().unwrap()),
            "jie" => {
                let split: Vec<&str> = parts[1].splitn(2, ", ").collect();
                Instruction::JumpIfEven(split[0].parse().unwrap(), split[1].parse().unwrap())
            }
            "jio" => {
                let split: Vec<&str> = parts[1].splitn(2, ", ").collect();
                Instruction::JumpIfOne(split[0].parse().unwrap(), split[1].parse().unwrap())
            }
            inst => panic!("invalid instruction: {}", inst),
        }
    }
}

struct Computer {
    register_a: usize,
    register_b: usize,
    current_index: usize,

    instructions: Vec<Instruction>,
}

impl Computer {
    fn boot_up(instructions: Vec<Instruction>) -> Computer {
        Computer {
            register_a: 0,
            register_b: 0,
            current_index: 0,
            instructions,
        }
    }

    fn update_register<F>(&mut self, register: char, func: F)
    where
        F: FnOnce(usize) -> usize,
    {
        match register {
            'a' => {
                self.register_a = func(self.register_a);
            }
            'b' => {
                self.register_b = func(self.register_b);
            }
            c => panic!("unknown register: {}", c),
        }
    }

    fn register_is_even(&self, register: char) -> bool {
        match register {
            'a' => self.register_a % 2 == 0,
            'b' => self.register_b % 2 == 0,
            c => panic!("unknown register: {}", c),
        }
    }

    fn register_is_one(&self, register: char) -> bool {
        match register {
            'a' => self.register_a == 1,
            'b' => self.register_b == 1,
            c => panic!("unknown register: {}", c),
        }
    }

    fn update_index(&mut self, change: isize) -> bool {
        let new_index = (self.current_index as isize) + change;
        if new_index < 0 || new_index >= self.instructions.len() as isize {
            return true;
        }

        self.current_index = new_index as usize;
        false
    }

    fn run(&mut self) {
        loop {
            let change = match self.instructions[self.current_index] {
                Instruction::Half(c) => {
                    self.update_register(c, |r| r / 2);
                    1
                }
                Instruction::Triple(c) => {
                    self.update_register(c, |r| r * 3);
                    1
                }
                Instruction::Increment(c) => {
                    self.update_register(c, |r| r + 1);
                    1
                }
                Instruction::Jump(j) => j,
                Instruction::JumpIfEven(c, j) => match self.register_is_even(c) {
                    true => j,
                    false => 1,
                },
                Instruction::JumpIfOne(c, j) => match self.register_is_one(c) {
                    true => j,
                    false => 1,
                },
            };

            if self.update_index(change) {
                break;
            }
        }
    }

    fn reset(&mut self, register_a: usize, register_b: usize) {
        self.register_a = register_a;
        self.register_b = register_b;
        self.current_index = 0;
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day23.txt").unwrap();
    let instructions: Vec<Instruction> =
        input.trim().lines().map(|i| Instruction::init(i)).collect();

    let mut computer = Computer::boot_up(instructions);
    computer.run();
    println!("register b: {}", computer.register_b);

    computer.reset(1, 0);
    computer.run();
    println!("register b: {}", computer.register_b);
}
