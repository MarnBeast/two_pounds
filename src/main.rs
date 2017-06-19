mod coin;       // bring in the coin.rs file with the Coin struct
use coin::Coin;

fn main()
{
    let onep = Coin::from_val(1);
    let twop = Coin::from_val(2);
    let fivp = Coin::from_val(5);
    let tenp = Coin::from_val(10);
    let twep = Coin::from_val(20);
    let fifp = Coin::from_val(50);
    let oneh = Coin::from_val(100);
    let twoh = Coin::from_val(200);

    let mut coins1: Vec<Coin> =
        vec![onep, twop, fivp, tenp,
             twep, fifp, oneh, twoh];

    let mut coins2: Vec<Coin> =
        vec![];

    println!("\nCalculating Initial Combos!");

    while coins1.len() > 0 {
        let mut coin = coins1.pop().expect("Empty vector!");
        println!("\n{}", coin);
        coin.calc_initial_combos(&coins1);
        coins2.push(coin);
    }

    println!("\nCalculating Replacement Combos!");

    while coins2.len() > 0 {
        let mut coin = coins2.pop().expect("Empty vector!");
        println!("\n{}", coin);
        coin.calc_replacement_combos(&coins1);
        println!("Combinations: {}", coin.recursive_combinations.len() + 1);    // + 1 is for self. I.e. 1x£2 is valid, but not included in calculation
        print_combos(&coin);
        coins1.push(coin);
    }
}

fn print_combos(coin: &Coin)
{
    if coin.value > 20 {
        return;
    }

    for combo in coin.recursive_combinations.iter() {
        println!("{}", combo);
    }
}
