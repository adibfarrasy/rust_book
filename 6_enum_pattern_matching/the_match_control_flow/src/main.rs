#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[allow(dead_code)]
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
// fn reroll() {}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => reroll(),
        other => (), // empty tuple indicates unit value
    }
}
