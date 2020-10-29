use std::collections::HashMap;

const STARTING_HP: isize = 50;
const STARTING_MANA: usize = 500;

const MAGIC_MISSILE_COST: usize = 53;
const DRAIN_COST: usize = 73;
const SHIELD_COST: usize = 113;
const POISON_COST: usize = 173;
const RECHARGE_COST: usize = 229;

enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn cost(&self) -> usize {
        match self {
            Spell::MagicMissile => MAGIC_MISSILE_COST,
            Spell::Drain => DRAIN_COST,
            Spell::Shield => SHIELD_COST,
            Spell::Poison => POISON_COST,
            Spell::Recharge => RECHARGE_COST,
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Game {
    player_hp: isize,
    player_mana: usize,

    boss_hp: isize,
    boss_damage: usize,

    shield_timer: usize,
    poison_timer: usize,
    recharge_timer: usize,

    hard_mode: bool,
}

impl Game {
    fn new(boss_hp: isize, boss_damage: usize, hard_mode: bool) -> Game {
        let mut game = Game {
            player_hp: STARTING_HP,
            player_mana: STARTING_MANA,

            boss_hp,
            boss_damage,

            shield_timer: 0,
            poison_timer: 0,
            recharge_timer: 0,

            hard_mode,
        };

        if hard_mode {
            game.player_hp -= 1;
        }

        game
    }

    fn has_winner(&self) -> Option<bool> {
        if self.player_hp <= 0 {
            return Some(false);
        }
        if self.boss_hp <= 0 {
            return Some(true);
        }
        return None;
    }

    fn apply_effects(&mut self) {
        if self.shield_timer > 0 {
            self.shield_timer -= 1;
        }
        if self.poison_timer > 0 {
            self.poison_timer -= 1;
            self.boss_hp -= 3
        }
        if self.recharge_timer > 0 {
            self.recharge_timer -= 1;
            self.player_mana += 101;
        }
    }

    fn cast_spell(&mut self, spell: Spell) {
        match spell {
            Spell::MagicMissile => {
                if self.player_mana < MAGIC_MISSILE_COST {
                    panic!("not enough mana to cast Magic Missile");
                }

                self.player_mana -= MAGIC_MISSILE_COST;
                self.boss_hp -= 4;
            }
            Spell::Drain => {
                if self.player_mana < DRAIN_COST {
                    panic!("not enough mana to cast Drain");
                }

                self.player_mana -= DRAIN_COST;
                self.boss_hp -= 2;
                self.player_hp += 2;
            }
            Spell::Shield => {
                if self.player_mana < SHIELD_COST {
                    panic!("not enough mana to cast Shield");
                }
                if self.shield_timer > 0 {
                    panic!("Shield is already in effect");
                }

                self.player_mana -= SHIELD_COST;
                self.shield_timer = 6;
            }
            Spell::Poison => {
                if self.player_mana < POISON_COST {
                    panic!("not enough mana to cast Poison");
                }
                if self.poison_timer > 0 {
                    panic!("Poison is already in effect");
                }

                self.player_mana -= POISON_COST;
                self.poison_timer = 6;
            }
            Spell::Recharge => {
                if self.player_mana < RECHARGE_COST {
                    panic!("not enough mana to cast Recharge");
                }
                if self.recharge_timer > 0 {
                    panic!("Recharge is already in effect");
                }

                self.player_mana -= RECHARGE_COST;
                self.recharge_timer = 5;
            }
        }
    }

    fn boss_attack(&mut self) {
        let mut armor: isize = 0;
        if self.shield_timer > 0 {
            armor = 7;
        }

        let damage = vec![1, (self.boss_damage as isize) - armor]
            .into_iter()
            .max()
            .unwrap();

        self.player_hp -= damage;
    }

    fn take_turn(&mut self, spell: Spell) -> Option<bool> {
        self.cast_spell(spell);
        if let Some(winner) = self.has_winner() {
            return Some(winner);
        }

        self.apply_effects();
        if let Some(winner) = self.has_winner() {
            return Some(winner);
        }

        self.boss_attack();
        if let Some(winner) = self.has_winner() {
            return Some(winner);
        }

        if self.hard_mode {
            self.player_hp -= 1;
        }

        self.apply_effects();
        self.has_winner()
    }

    fn available_spells(&self) -> Vec<Spell> {
        vec![
            Spell::MagicMissile,
            Spell::Drain,
            Spell::Shield,
            Spell::Poison,
            Spell::Recharge,
        ]
        .into_iter()
        .filter(|s| match s {
            Spell::Shield => s.cost() <= self.player_mana && self.shield_timer == 0,
            Spell::Poison => s.cost() <= self.player_mana && self.poison_timer == 0,
            Spell::Recharge => s.cost() <= self.player_mana && self.recharge_timer == 0,
            sp => sp.cost() <= self.player_mana,
        })
        .collect()
    }
}

fn min_mana_used(seen_states: &mut HashMap<Game, usize>, game: Game, current_mana: usize) -> usize {
    if let Some(mana) = seen_states.get(&game) {
        if *mana <= current_mana {
            return usize::MAX;
        }
    }
    seen_states.insert(game, current_mana);

    let available_spells = game.available_spells();
    if available_spells.is_empty() {
        return usize::MAX;
    }

    game.available_spells()
        .into_iter()
        .map(|s| {
            let mut copy = game;
            let cost = current_mana + s.cost();

            match copy.take_turn(s) {
                Some(v) => {
                    if v {
                        return cost;
                    }
                    usize::MAX
                }
                None => min_mana_used(seen_states, copy, cost),
            }
        })
        .min()
        .unwrap()
}

fn main() {
    let game = Game::new(51, 9, false);
    println!("min mana: {}", min_mana_used(&mut HashMap::new(), game, 0));

    let game_hard = Game::new(51, 9, true);
    println!(
        "min mana (hard): {}",
        min_mana_used(&mut HashMap::new(), game_hard, 0)
    );
}
