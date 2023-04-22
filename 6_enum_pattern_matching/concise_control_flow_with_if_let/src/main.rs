#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alaska,
    Alabama,
    // --skip--
}

fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let coin = Coin::Quarter(UsState::Alabama);
    let mut _count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        _count += 1;
    }
}
