fn main() {
    let fruit = Fruit::Apple(FruitSize::Ultra);
    if let Fruit::Apple(FruitSize::Ultra) = fruit {
        println!("Pro Max!");
    }

    let x = Some(114514);
    let y: Option<i32> = None;
    if let Some(val) = x {
        println!("Ok {}", val);
    }
    if let Some(val) = y {
        println!("Ok {}", val);
    }
}

enum FruitSize {
    Small,
    Medium,
    Plus,
    Big,
    Max,
    Ultra,
}

enum Fruit {
    Apple(FruitSize),
    Banana,
    Coconut,
}
