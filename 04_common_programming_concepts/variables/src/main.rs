const SENBAI: u32 = 114514;

fn main() {
    //let x = 5;
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    let y = 7;
    let mut y = 8;

    //SENBAI = 1919810;

    let z = 6;
    let z = 7;
    let z = z + 1;
    {
        let z = z / 2;
        println!("z: {z}");
    }
    println!("z: {z}");

}
