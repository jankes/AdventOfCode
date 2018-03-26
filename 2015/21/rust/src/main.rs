use std::slice::Iter;

struct Item {
    name: &'static str,
}

const ITEMS: [Item; 3] = [Item {name: "A"}, Item {name: "B"}, Item {name: "C"}];

struct ItemIter {
    index: usize
}

impl Iterator for ItemIter {
    type Item = &'static Item;

    fn next(&mut self) -> Option<&'static Item> {
        if self.index < ITEMS.len() {
            let item = &ITEMS[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

fn items_iter() -> ItemIter {
    ItemIter { index: 0 }
}

fn main() {
    for item in items_iter() {
        // panics or crashes
        println!("{}", item.name);

        // works okay
        // let name = item.name;
        // println!("{}", name);
    }
}

/*
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

fn weapons_iter<'a>() -> Iter<'a, Item> {
    WEAPONS.iter()
}

fn armors_iter<'a>() -> Iter<'a, Item> {
    ARMORS.iter()
}

fn one_ring_iter<'a>() -> Iter<'a, Item> {
    RINGS.iter()
}

fn two_rings_iter<'a>() -> TwoRingsIter {
    TwoRingsIter::new()
}

struct TwoRingsIter {
    first: usize,
    second: usize
}

impl TwoRingsIter {
    fn new() -> TwoRingsIter {
        TwoRingsIter {
            first: 1, second: 2
        }
    }
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

fn main() {
    
    for (ring_1, ring_2) in two_rings_iter() {
        // let a = ring_1.name;
        // let b = ring_2.name;
        // println!("{} {}", a, b);

        println!("{} {}", ring_1.name, ring_2.name);
    }
}
*/
