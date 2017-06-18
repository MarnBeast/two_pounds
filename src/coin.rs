use std::fmt;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::BTreeMap;
use std::string::ToString;


/* STRUCTS */

#[derive(Eq)]
pub struct Coin {
    value: u32,
    shallow_combinations: Vec<CoinList>,
    recursive_combinations: HashSet<CoinList>,
}

/// This is just used to keep a count of how many of each coin type
/// are needed to create me in this combination.
#[derive(Eq, Hash)]
pub struct CoinList {
    is_empty: bool,
    counts: BTreeMap<u32, CoinCount>,   // using BTreeMap because it implements hash, which we want for our hashset of CoinLists
}

#[derive(Copy, Clone, Eq, Hash)]
pub struct CoinCount {
    value: u32,
    count: u32,
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

            if !combination.is_empty {
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
        self.is_empty = false;

        match self.counts.get_mut(&value) {
            Some(coin_count) => coin_count.count += how_many,
            None => println!("Invalid coin value {}", value)
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
        let mut new_list = CoinList {
            is_empty: true,
            counts: BTreeMap::new(),
        };

        new_list.counts.insert(200,   CoinCount {value: 200, count: 0});
        new_list.counts.insert(100,   CoinCount {value: 100, count: 0});
        new_list.counts.insert(50,    CoinCount {value: 50, count: 0});
        new_list.counts.insert(20,    CoinCount {value: 20, count: 0});
        new_list.counts.insert(10,    CoinCount {value: 10, count: 0});
        new_list.counts.insert(5,     CoinCount {value: 5, count: 0});
        new_list.counts.insert(2,     CoinCount {value: 2, count: 0});
        new_list.counts.insert(1,     CoinCount {value: 1, count: 0});

        new_list
    }
}

impl Clone for CoinList {
    fn clone(&self) -> CoinList {
        let mut clone = CoinList::new();
        clone.is_empty = self.is_empty;

        for (key, value) in self.counts.iter() {
            clone.counts.insert(key.clone(), value.clone());
        }
        clone
    }
}

impl PartialEq for CoinList {
    fn eq(&self, other: &CoinList) -> bool {
        let mut eq = self.is_empty == other.is_empty;
        if eq {
            for (key, coin_count) in self.counts.iter() {
                if let Some(other_count) = other.counts.get(&key) {
                    eq = coin_count.eq(other_count);
                    if !eq {
                        break;
                    }
                }
            }
        }

        eq
    }
}

impl fmt::Display for CoinCount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut message = String::from("");
        match self.value {
            200 =>  { message.push_str(&(self.count.to_string()[..])); message.push_str("x£2"); }
            100 => { message.push_str(&(self.count.to_string()[..])); message.push_str("x£1"); }
            50 => { message.push_str(&(self.count.to_string()[..])); message.push_str("x50p"); }
            20 => { message.push_str(&(self.count.to_string()[..])); message.push_str("x20p"); }
            10 => { message.push_str(&(self.count.to_string()[..])); message.push_str("x10p"); }
            5 => { message.push_str(&(self.count.to_string()[..])); message.push_str("x5p"); }
            2 => { message.push_str(&(self.count.to_string()[..])); message.push_str("x2p"); }
            1 => { message.push_str(&(self.count.to_string()[..])); message.push_str("x1p"); }
            _ => message.push_str("Invalid CoinCount"),
        }

        write!(f, "{}", message)
    }
}

impl fmt::Display for CoinList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut message = String::new();

        for (key, coin_count) in self.counts.iter() {
            if message.len() > 0 {
                message.push_str(", ");
            }
            message.push_str(&(coin_count.to_string()[..]));
        }

        if message.len() <= 0 {
            message.push_str("Empty CoinList");

            if !self.is_empty {
                message.push_str("ERROR! Empty CoinList marked as not empty!");
            }
        }

        write!(f, "{}", message)
    }
}



impl PartialEq for CoinCount {
    fn eq(&self, other: &CoinCount) -> bool {
        self.value == other.value
        && self.count == other.count
    }
}