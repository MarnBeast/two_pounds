use std::fmt;
use std::cmp::Ordering;
use std::collections::HashSet;


/* STRUCTS */

#[derive(Eq)]
pub struct Coin {
    value: u32,
    shallow_combinations: Vec<CoinList>,
    recursive_combinations: HashSet<CoinList>,
}

/// This is just used to keep a count of how many of each coin type
/// are needed to create me in this combination.
#[derive(Copy, Clone, Eq, Hash)]
pub struct CoinList {
    isempty: bool,
    oneps: u32,
    twops: u32,
    fivps: u32,
    tenps: u32,
    tweps: u32,
    fifps: u32,
    onehs: u32,
    twohs: u32,
}




/* MAIN IMPL */

impl Coin {
    pub fn from_val(val: u32) -> Coin {
        Coin {value: val, ..Default::default() }
    }

    pub fn calc_initial_combos(&mut self, coins: &[Coin]) {   // coin vector slice

        // I was going to sort, but then realized the algorithm doesn't require it.
        for (i, coin) in coins.iter().enumerate() {
            let times = self.value / coin.value;
            let remainder = self.value % coin.value;

            let mut combination = CoinList::new();

            if times > 0 {
                combination.add_coins(times, coin.value);

                if remainder > 0 {
                    combination.calc_combination(remainder, &coins[i..]);       // We don't need the whole thing checked, just the things smaller than our current coin
                }
            }

            if !combination.isempty {
                println!("{}", combination);
                self.shallow_combinations.push(combination);
            }
        }
    }

    pub fn calc_replacement_combos(&mut self, coins: &[Coin])
    {
        for combo in self.shallow_combinations.iter() {
            let copy = combo.clone();
            self.recursive_combinations.insert(copy);
        }

        // for each of the coins smaller than us (those passed in here are smaller than us)
        // for each of that smaller coins combos, create a new combo for each of our combos.

//        for self_combo in self.shallow_combinations.iter() {
//            for coin in coins.iter() {
//                for combo in coin.shallow_combinations.iter() {
//                    let new_combo = self_combo.clone();
//                    match value {
//                        1 => new_combo.oneps -= 1,
//                        2 => new_combo.twops -= 1,
//                        5 => new_combo.fivps -= 1,
//                        10 => new_combo.tenps -= 1,
//                        20 => new_combo.tweps -= 1,
//                        50 => new_combo.fifps -= 1,
//                        100 => new_combo.onehs -= 1,
//                        200 => new_combo.twohs -= 1,
//                        _ => {
//                            println!("Invalid coin value {}", value)
//                        }
//                    };
//                }
//            }
//        }

    }
}

impl CoinList {
    pub fn new() -> CoinList {
        CoinList {..Default::default() }
    }

    pub fn add_coins(&mut self, how_many: u32, value: u32){
        self.isempty = false;
        match value {
            1 => self.oneps += how_many,
            2 => self.twops += how_many,
            5 => self.fivps += how_many,
            10 => self.tenps += how_many,
            20 => self.tweps += how_many,
            50 => self.fifps += how_many,
            100 => self.onehs += how_many,
            200 => self.twohs += how_many,
            _ => {
                self.isempty = true;
                println!("Invalid coin value {}", value)
            }
        };
    }

    pub fn calc_combination(&mut self, remainder: u32, coins: &[Coin]) {   // coin vector slice

        // Loop through the list until we find something that can
        // divide our remainder.
        for (i, coin) in coins.iter().enumerate() {
            let times = remainder / coin.value;
            let remainder = remainder % coin.value;

            if times > 0 {
                self.add_coins(times, coin.value);

                if remainder > 0 {
                    self.calc_combination(remainder, &coins[i..]);       // We don't need the whole thing checked, just the things smaller than our current coin
                }

                // Once we've found something that can divide our remainder
                break;
            }
        }
    }
}


/* SUPPORTING IMPL */

impl Default for Coin {
    fn default () -> Coin {
        Coin {value: 0, shallow_combinations: vec![], recursive_combinations: HashSet::new()}
    }
}

impl Ord for Coin {
    fn cmp(&self, other: &Coin) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Coin {
    fn partial_cmp(&self, other: &Coin) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Coin {
    fn eq(&self, other: &Coin) -> bool {
        self.value == other.value
    }
}

impl fmt::Display for Coin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Coin({})", self.value)
    }
}


impl Default for CoinList {
    fn default () -> CoinList {
        CoinList {
            isempty: true,
            oneps: 0,
            twops: 0,
            fivps: 0,
            tenps: 0,
            tweps: 0,
            fifps: 0,
            onehs: 0,
            twohs: 0,
        }
    }
}

impl PartialEq for CoinList {
    fn eq(&self, other: &CoinList) -> bool {
        self.oneps == other.oneps
        && self.twops == other.twops
        && self.fivps == other.fivps
        && self.tenps == other.tenps
        && self.tweps == other.tweps
        && self.fifps == other.fifps
        && self.onehs == other.onehs
        && self.twohs == other.twohs
    }
}

impl fmt::Display for CoinList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut message = String::new();
        let mut empty = true;

        if self.twohs > 0 {
            message.push_str(&(self.twohs.to_string()));
            message.push_str("x£2, ");
            empty = false;
        }
        if self.onehs > 0 {
            message.push_str(&(self.onehs.to_string()));
            message.push_str("x£1, ");
            empty = false;
        }
        if self.fifps > 0 {
            message.push_str(&(self.fifps.to_string()));
            message.push_str("x50p, ");
            empty = false;
        }
        if self.tweps > 0 {
            message.push_str(&(self.tweps.to_string()));
            message.push_str("x20p, ");
            empty = false;
        }
        if self.tenps > 0 {
            message.push_str(&(self.tenps.to_string()));
            message.push_str("x10p, ");
            empty = false;
        }
        if self.fivps > 0 {
            message.push_str(&(self.fivps.to_string()));
            message.push_str("x5p, ");
            empty = false;
        }
        if self.twops > 0 {
            message.push_str(&(self.twops.to_string()));
            message.push_str("x2p, ");
            empty = false;
        }
        if self.oneps > 0 {
            message.push_str(&(self.oneps.to_string()));
            message.push_str("x1p, ");
            empty = false;
        }

        if empty {
            message.push_str("Empty CoinList");

            if !self.isempty {
                message.push_str("ERROR! Empty CoinList marked as not empty!");
            }
        }

        write!(f, "{}", message)
    }
}