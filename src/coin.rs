use std::fmt;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::BTreeMap;
use std::string::ToString;



/* STRUCTS */

#[derive(Eq)]
pub struct Coin {
    pub value: u32,
    shallow_combinations: Vec<CoinCombination>,
    pub recursive_combinations: HashSet<CoinCombination>,
}

/// This is just used to keep a count of how many of each coin type
/// are needed to create me in this combination.
#[derive(Eq, Hash)]
pub struct CoinCombination {
    is_empty: bool,
    counts: BTreeMap<u32, CoinCount>,   // using BTreeMap because it implements hash, which we want for our hashset of CoinCombination
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

            let mut combination = CoinCombination::new();

            if times > 0 {
                combination.add_coins(times, coin.value);

                if remainder > 0 {
                    // We don't need the whole thing checked, just the things smaller than our current coin
                    combination.calc_combination(remainder, &coins[..i]);   // we're going small to big
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
//            println!("{}", copy);
            self.recursive_combinations.insert(copy);
        }

        // for each of our shallow combinations,
        // for each of the coin counts in that combination
        // for each coin in the replacements list
        //   if the coin matches the coin count
        //      make a copy of the

        for shallow_combo in self.shallow_combinations.iter() {         // shallow_combo is a CoinCombination
//            for (_, shallow_count) in shallow_combo.counts.iter() {   // shallow_count is a CoinCount
                for replace_coin in coins.iter() {                      // replace_coin is a Coin
//                    println!("{}", replace_coin);
                    // if the coin matches one of the coins in our shallow_combo, it means we can
                    // make more combos by copying this one and one by one replacing counts of our
                    // replace coin value with counts in the replace coin's recursive_combinations.
//                    if shallow_count.value == replace_coin.value {
                        shallow_combo.calc_replacement_combos(replace_coin, &mut self.recursive_combinations);
//                    }

                }
//            }
        }
    }
}

impl CoinCombination {
    pub fn new() -> CoinCombination {
        CoinCombination {..Default::default() }
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


    /// This method makes a copy of its CoinCombination and then modifies that copy by splitting one
    /// of its combination counts matching the passed in "coin_from" coin value. The new combo is
    /// then added to the "coin_to" Coin's recursive_combinations if it didn't already exist in the
    /// set.
    /// Furthermore, if the new CoinCombination didn't exist, it will have calc_replacement_combos
    /// called on it in order to attempt to split it up further.
    ///
    /// So if the passed in coin_from was 5p, the new CoinCombination would have 1 less 5p and have
    /// its other combination counts incremented based on the first combination in the 5p coin's
    /// recursive_combinations set. It would then call this method again on the new combo as long as
    /// there's more 5p pieces to potentially split up.
    pub fn calc_replacement_combos(&self, coin_from: &Coin, coin_to_recursive_combinations: &mut HashSet<CoinCombination>)
    {
        for combo_from in coin_from.recursive_combinations.iter() {
            let mut combo_new = self.clone();
            let keep_going;

            // Get the count matching the coin_from value. So if coin_from was 5p, we want the 5p count.
            {   // using scope braces to release coin_count's ownership of combo_new
                let mut coin_count = combo_new.counts.get_mut(&coin_from.value).expect("NOT FOUND");
                if coin_count.count == 0 {
                    return; // we can't split this up
                }
                coin_count.count -= 1;  // because we're taking that 1 denomination, and splitting it.
                keep_going = coin_count.count > 0;
            }
            // go through the combo getting the count of each coin type, and increment our new combo's
            // counts to match those.
            for (_, count_from) in combo_from.counts.iter() {
                let mut count_new = combo_new.counts.get_mut(& count_from.value).expect("NOT FOUND 2");
                count_new.count += count_from.count;
            }

            // If the resultant new combination does not already exist in our recursive_combinations
            // list, we want to add it and then see if this new combo can be broken down further.
            if !coin_to_recursive_combinations.contains(&combo_new)
            && keep_going {

                // Insert a clone of the new combo so we can continue working off of it after insert.
                coin_to_recursive_combinations.insert(combo_new.clone());
//                println!("{}", combo_new);
                combo_new.calc_replacement_combos(&coin_from, coin_to_recursive_combinations);
            }
            else {
                coin_to_recursive_combinations.insert(combo_new.clone());
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


impl Default for CoinCombination {
    fn default () -> CoinCombination {
        let mut new_list = CoinCombination {
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

impl Clone for CoinCombination {
    fn clone(&self) -> CoinCombination {
        let mut clone = CoinCombination::new();
        clone.is_empty = self.is_empty;

        for (key, value) in self.counts.iter() {
            clone.counts.insert(key.clone(), value.clone());
        }
        clone
    }
}

impl PartialEq for CoinCombination {
    fn eq(&self, other: &CoinCombination) -> bool {
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

impl fmt::Display for CoinCombination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut message = String::new();

        for (_, coin_count) in self.counts.iter() {
            if coin_count.count > 0 {
                if message.len() > 0 {
                    message.push_str(", ");
                }
                message.push_str(&(coin_count.to_string()[..]));
            }
        }

        if message.len() <= 0 {
            message.push_str("Empty CoinCombination");

            if !self.is_empty {
                message.push_str("ERROR! Empty CoinCombination marked as not empty!");
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