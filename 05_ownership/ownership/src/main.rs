#![feature(core_intrinsics)]
fn print_type_of<T>(_: T) {
    println!("{}", std::intrinsics::type_name::<T>() );
}

fn main() {
    scope();
    str_and_ownership();
    mov();
    clon();
    func_and_ownership();
}

fn scope(){
    {
        let s = "senbai";
    }
    //println!("{s}");


}

fn str_and_ownership(){
    let mut s1 = "LiteralString"; // On stack
    //s1.push_str("111");

    let mut s2 = String::from("Genshin"); // On heap
    s2.push_str(", start!");
    println!("{s2}");
}


// move = shallow copy + disable the old var!
// Rust never deep copy automatically
fn mov(){
    // fixed size, so copied x to y
    let x = 5;
    let y = x;
    println!("{x}");
    println!("{y}");

    let s1 = "foo";
    let s2 = s1;
    println!("{s1}");
    println!("{s2}");
    print_type_of(s1); // it is &str, without the copy TRAIT but the drop TRAIT (They are conflict
                       // with each other)

    let s3 = String::from("bar");
    //let s4 = s3;
    //println!("{s3}");
    //println!("{s4}");
    print_type_of(s3); // it is alloc::string::String
}

// use clone to deep copy
fn clon(){ 
    let s3 = String::from("bar");
    let s4 = s3.clone();
    println!("s3:{s3} s4:{s4}");
}



fn func_and_ownership(){
    let s = String::from("hello");  // s 进入作用域
    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效
    //println!("{s}");
    let x = 5;                      // x 进入作用域
    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，
                                    // 所以在后面可继续使用 x
    println!("{x}"); // ok
}

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处
