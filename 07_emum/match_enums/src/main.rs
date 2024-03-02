fn main() {
    let rmb = RMB::Fen(RMBVer::V4);
    values_in_FEN(rmb);

    solve_option();

    use_pattern();
}

#[derive(Debug)]
enum RMBVer {
    V4,
    V5,
}
enum RMB {
    Yuan,
    Jiao,
    Fen(RMBVer),
}

fn values_in_FEN(rmb: RMB) -> u8 {
    match rmb {
        RMB::Yuan => 100, // TYPE => EXPRESSION
        RMB::Jiao => 10,
        RMB::Fen(ver) => {
            println!("Fen: {:?}", ver);
            1
        }
    }
}

fn plus_option(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // expicitly solve the NONE
        Some(i32v) => Some(i32v+1),
    }// ATTENTION: match must match all arms!
}

fn solve_option(){
    let five = Some(5);
    let six = plus_option(five);
    let none = plus_option(None);
}

fn use_pattern(){
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        //other => move_player(other),
        //other => remove_fancy_hat(), // not used may warning
        _ => (), // unit means to do nothing; and _ means ignore it
    }
    // wow Rust can define funcs inside a func !
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

}