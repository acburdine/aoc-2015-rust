extern crate permutohedron;

use permutohedron::Heap;
use std::collections::HashSet;

#[derive(Clone, Copy)]
struct Person {
    hp: usize,
    damage: usize,
    armor: usize,
}

impl Person {
    fn new(hp: usize, damage: usize, armor: usize) -> Person {
        Person { hp, damage, armor }
    }

    fn equip(&mut self, item: Item) {
        self.damage += item.damage;
        self.armor += item.armor;
    }

    fn attacked_by(&mut self, attacker: &Person) -> bool {
        let mut damage = 1;
        if attacker.damage > self.armor {
            damage = attacker.damage - self.armor;
        }

        if damage >= self.hp {
            self.hp = 0;
            return true;
        }

        self.hp -= damage;
        false
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Item {
    cost: usize,
    damage: usize,
    armor: usize,
}

impl Item {
    fn new(cost: usize, damage: usize, armor: usize) -> Item {
        Item {
            cost,
            damage,
            armor,
        }
    }
}

fn fight(player: &mut Person, boss: &mut Person) -> bool {
    loop {
        if boss.attacked_by(player) {
            return true;
        }

        if player.attacked_by(boss) {
            return false;
        }
    }
}

fn shop_permutations(weapons: Vec<Item>, armor: Vec<Item>, rings: Vec<Item>) -> Vec<Vec<Item>> {
    let mut permutations: Vec<Vec<Item>> = Vec::new();

    // one weapon, no armor, no rings
    for w in weapons.iter() {
        permutations.push(vec![*w]);
    }

    // one weapon, one armor, no rings
    for a in armor.iter() {
        for w in weapons.iter() {
            permutations.push(vec![*w, *a]);
        }
    }

    for r in rings.iter() {
        // one weapon, no armor, one ring
        for w in weapons.iter() {
            permutations.push(vec![*w, *r]);
        }

        // one weapon, one armor, one ring
        for a in armor.iter() {
            for w in weapons.iter() {
                permutations.push(vec![*w, *a, *r]);
            }
        }
    }

    // lastly, build combos with 2 rings
    let mut cloned_rings = rings.clone();
    let heap = Heap::new(&mut cloned_rings);
    heap.map(|ring_mut| ring_mut.into_iter().take(2).collect::<Vec<Item>>())
        .collect::<HashSet<Vec<Item>>>()
        .into_iter()
        .for_each(|ring_mut| {
            // one weapon, no armor, two rings
            for w in weapons.iter() {
                let mut mutation = vec![*w];
                ring_mut.iter().for_each(|r| mutation.push(*r));
                permutations.push(mutation);
            }

            // one weapon, one armor, two rings
            for a in armor.iter() {
                for w in weapons.iter() {
                    let mut mutation = vec![*w, *a];
                    ring_mut.iter().for_each(|r| mutation.push(*r));
                    permutations.push(mutation);
                }
            }
        });

    permutations
}

fn main() {
    let player_tmpl = Person::new(100, 0, 0);
    let boss_tmpl = Person::new(103, 9, 2);

    let weapons = vec![
        Item::new(8, 4, 0),  // Dagger
        Item::new(10, 5, 0), // Shortsword
        Item::new(25, 6, 0), // Warhammer
        Item::new(40, 7, 0), // Longsword
        Item::new(74, 8, 0), // Greataxe
    ];

    let armor = vec![
        Item::new(13, 0, 1),  // Leather
        Item::new(31, 0, 2),  // Chainmail
        Item::new(53, 0, 3),  // Splintmail
        Item::new(75, 0, 4),  // Bandedmail
        Item::new(102, 0, 5), // Platemail
    ];

    let rings = vec![
        Item::new(25, 1, 0),  // Damage + 1
        Item::new(50, 2, 0),  // Damage + 2
        Item::new(100, 3, 0), // Damage + 3
        Item::new(20, 0, 1),  // Defense + 1
        Item::new(40, 0, 2),  // Defense + 2
        Item::new(80, 0, 3),  // Defense + 3
    ];

    let mut min_cost = usize::MAX;
    let mut max_cost: usize = 0;

    let permutations = shop_permutations(weapons, armor, rings);
    for items in permutations {
        let mut player = player_tmpl;
        let mut boss = boss_tmpl;

        let mut cost = 0;

        for item in items {
            player.equip(item);
            cost += item.cost;
        }

        if fight(&mut player, &mut boss) {
            min_cost = vec![min_cost, cost].into_iter().min().unwrap();
        } else {
            max_cost = vec![max_cost, cost].into_iter().max().unwrap();
        }
    }

    println!("minimum win cost: {}", min_cost);
    println!("maximum lose cost: {}", max_cost);
}
