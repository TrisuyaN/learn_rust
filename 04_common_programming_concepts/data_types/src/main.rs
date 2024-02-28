use std::io;

fn main() {
    println!("Hello, world!");
    let num: u32 = "114514".parse().expect("Invalid string");

    //let bin1: u8 = 0b1111_11111;
    let bin2: u16 = 0b1111_1111_1111_1111;
    let char1: u8 = b'A';
    let dec1: i8 = 127;
    let dec2: i32 = 114_514_191;
    let hex1: u8 = 0xff;
    //let hex2: u8 = 0xffff;
    //let hex3: i8 = 0xff;

    let float1: f32 = 1.145141919;
    let float2: f64 = 1.145141919;
    let float3: f32 = 1.0;
    let float4: f32 = 2.0;
    let float5: f32 = float3 + float4;
    println!("{float1}");
    println!("{float2}");
    println!("{float5}");

    let t = true;
    let f: bool = false; // with explicit type annotation
    
    let c = '😻';
    //let c1: u8 = b'😻';
    //let c2: u8 = '😻';
    //let c3: char = b'😻';
    let c4: char = '😻';

    

    let tup1: (i32, f32, char) = (114, 5.14, '😻');
    let (e1, e2, e3) = tup1;
    //e1 = tup1.0;
    print!("e1");
    let tup2: (i32, (i32, i32));

    

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    //let arr2: [3; 5];
    let arr3 = [3; 5];

    access_arr();
}


fn access_arr() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}



