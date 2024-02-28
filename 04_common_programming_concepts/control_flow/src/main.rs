use std::io;

fn main() {
    println!("Hello, world!");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error");

    let x = input.trim().parse().expect("Error");
    branch(x);

    
    loops();

}

fn branch(x: i32){
    if x > 114514 {
        println!("senbai");
    } else {
        println!("suki");
    }

    //if 3 {
    //    println!("111");
    //}else if 3!=1 {
    //    println!("1");
    //}else{
    //    println!("0");
    //}
}

fn if_in_let(){
    let x = if true {
        1
    } else {
        0
    }; // it is also a EXPRESSION!
}

fn loops(){
    let mut x=1;
    loop{
        x+=1; // no x++ or ++x !!
        if x==10 {
            break;
        }
    }
    
    //break with return value 
    let mut counter=0;
    let result=loop {
        counter+=1;
        if counter==10 {
            break counter*2;
        }
    };
    println!("The result is {}", result);

    let mut number=3;
    while number!=0 {
        println!("{}!", number);
        number-=1;
    }
    println!("LIFTOFF!!!");

    let arr = [1, 2, 3, 4, 5];
    for a in arr {
        println!("{a}");
    }

    for e in 1..4 { // 1 2 3
        println!("{e}");
    }

}
