use std::cmp;
use std::slice::Iter;

#[derive(Copy, Clone)]
struct Item {
    name: &'static str,
    cost: u16,
    damage: i8,
    armor: i8
}

const WEAPONS: [Item; 5] = [Item {name: "Dagger",     cost: 8,  damage: 4, armor: 0},
                            Item {name: "Shortsword", cost: 10, damage: 5, armor: 0},
                            Item {name: "Warhammer",  cost: 25, damage: 6, armor: 0},
                            Item {name: "Longsword",  cost: 40, damage: 7, armor: 0},
                            Item {name: "Greataxe",   cost: 74, damage: 8, armor: 0}];

const ARMORS: [Item; 6] = [Item {name: "None",       cost: 0,   damage: 0, armor: 0},
                           Item {name: "Leather",    cost: 13,  damage: 0, armor: 1},
                           Item {name: "Chainmail",  cost: 31,  damage: 0, armor: 2},
                           Item {name: "Splintmail", cost: 53,  damage: 0, armor: 3},
                           Item {name: "Bandedmail", cost: 75,  damage: 0, armor: 4},
                           Item {name: "Platemail",  cost: 102, damage: 0, armor: 5}];

const RINGS: [Item; 7] = [Item {name: "None",       cost: 0,   damage: 0, armor: 0},
                          Item {name: "Damage +1",  cost: 25,  damage: 1, armor: 0},
                          Item {name: "Damage +2",  cost: 50,  damage: 2, armor: 0},
                          Item {name: "Damage +3",  cost: 100, damage: 3, armor: 0},
                          Item {name: "Defense +1", cost: 20,  damage: 0, armor: 1},
                          Item {name: "Defense +2", cost: 40,  damage: 0, armor: 2},
                          Item {name: "Defense +3", cost: 80,  damage: 0, armor: 3}];

struct OneRingIter<'a> {
    iter: std::slice::Iter<'a, Item>
}

impl Iterator for OneRingIter<'static> {
    type Item = (&'static Item, &'static Item);

    fn next(&mut self) -> Option<(&'static Item, &'static Item)> {
        match self.iter.next() {
            Some(item) => Some((&RINGS[0], item)),
            None       => None
        }
    }
}

struct TwoRingsIter {
    first: usize,
    second: usize
}

impl Iterator for TwoRingsIter {
    type Item = (&'static Item, &'static Item);

    fn next(&mut self) -> Option<(&'static Item, &'static Item)> {
        if self.first < RINGS.len() - 1 {
            let item = (&RINGS[self.first], &RINGS[self.second]);
            self.second += 1;
            if self.second >= RINGS.len() {
                self.first += 1;
                self.second = self.first + 1;
            }
            Some(item)
        } else {
            None
        }
    }
}

fn weapons_iter<'a>() -> Iter<'a, Item> {
    WEAPONS.iter()
}

fn armors_iter<'a>() -> Iter<'a, Item> {
    ARMORS.iter()
}

fn one_ring_iter<'a>() -> OneRingIter<'a> {
    OneRingIter {
        iter: RINGS.iter()
    }
}

fn two_rings_iter<'a>() -> TwoRingsIter {
    TwoRingsIter {
        first: 1, second: 2
    }
}

fn rings_iter<'a>() -> std::iter::Chain<OneRingIter<'a>, TwoRingsIter> {
    one_ring_iter().chain(two_rings_iter())
}

fn battle_boss(weapon: &Item, armor: &Item, ring_1: &Item, ring_2: &Item) -> bool {
    let player_damage = weapon.damage + ring_1.damage + ring_2.damage;
    let player_armor = armor.armor + ring_1.armor + ring_2.armor;

    let boss_damage = 8;
    let boss_armor = 2;

    let damage_to_boss = cmp::max(player_damage - boss_armor, 1i8);
    let damage_to_player = cmp::max(boss_damage - player_armor, 1i8);

    let mut player_hp = 100i8;
    let mut boss_hp = 100i8;

    loop {
        boss_hp -= damage_to_boss;
        if boss_hp <= 0 {
            return true;
        }
        player_hp -= damage_to_player;
        if player_hp <= 0 {
            return false;
        }
    }    
}

fn main() {
    
    let dummy: std::iter::Chain<OneRingIter, TwoRingsIter> = one_ring_iter().chain(two_rings_iter());
    
    let mut optimal_weapon: Option<&'static Item> = None;
    let mut optimal_armor: Option<&'static Item> = None;
    let mut optimal_ring_1: Option<&'static Item> = None;
    let mut optimal_ring_2: Option<&'static Item> = None;
    let mut min_cost = u16::max_value();

    let mut suboptimal_weapon: Option<&'static Item> = None;
    let mut suboptimal_armor: Option<&'static Item> = None;
    let mut suboptimal_ring_1: Option<&'static Item> = None;
    let mut suboptimal_ring_2: Option<&'static Item> = None;
    let mut max_cost = u16::min_value();

    for weapon in weapons_iter() {
        for armor in armors_iter() {
            for (ring_1, ring_2) in rings_iter() {
                let total_cost = weapon.cost + armor.cost + ring_1.cost + ring_2.cost;
                if battle_boss(weapon, armor, ring_1, ring_2) {
                    if total_cost < min_cost {
                        min_cost = total_cost;
                        optimal_weapon = Some(weapon);
                        optimal_armor = Some(armor);
                        optimal_ring_1 = Some(ring_1);
                        optimal_ring_2 = Some(ring_2);
                    }
                } else {
                    if total_cost > max_cost {
                        max_cost = total_cost;
                        suboptimal_weapon = Some(weapon);
                        suboptimal_armor = Some(armor);
                        suboptimal_ring_1 = Some(ring_1);
                        suboptimal_ring_2 = Some(ring_2);
                    }
                }
            }
        }
    }

    println!("Least expensive equipment to beat the boss costs {} gold", min_cost);
    println!("Weapon: {}", optimal_weapon.unwrap().name);
    println!("Armor:  {}", optimal_armor.unwrap().name);
    // These line cause a crash!
    //println!("Ring 1: {}", optimal_ring_1.unwrap().name);
    //println!("Ring 2: {}", optimal_ring_2.unwrap().name);

    println!("Most expensive equipment to buy and still loos to the boss costs {} gold", max_cost);
    println!("Weapon: {}", suboptimal_weapon.unwrap().name);
    println!("Armor:  {}", suboptimal_armor.unwrap().name);
    // These line cause a crash!
    //println!("Ring 1: {}", suboptimal_ring_1.unwrap().name);
    //println!("Ring 2: {}", suboptimal_ring_2.unwrap().name);
}
