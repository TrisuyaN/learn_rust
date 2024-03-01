pub trait AnyExt {
    fn type_name(&self) -> &'static str;
}

impl<T> AnyExt for T {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

fn main(){
    // about str and String and &str
    
    //let astr: str = "cannot compile!";
    let s0 = "string"; // 字符串字面值：硬编码在二进制文件里，所以是不可变的切片&str
    let s0type = s0.type_name();
    println!("{}", s0type);

    let s1 = String::from("kfc vme50");
    let s1type = s1.type_name();
    println!("{}", s1type);


    let word = first_word(&s1);
    
    //s1.clear(); // mut ref here, however word (ref) still needs to be valid

    println!("{word}");


    arrslice();
}



// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

//fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {// also available for &String. DEREF COERCIONS
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}


fn deref(){
    let my_string = String::from("hello world");


    // let word = first_word(my_string); // Error
    // `first_word` 适用于 `String`（的 slice），部分或全部
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` 也适用于 `String` 的引用，
    // 这等价于整个 `String` 的 slice
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` 适用于字符串字面值，部分或全部
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // 因为字符串字面值已经 **是** 字符串 slice 了，
    // 这也是适用的，无需 slice 语法！
    let word = first_word(my_string_literal);
}

fn arrslice() {
    let a = [1, 1, 4, 5, 1, 4];
    let aslice = &a[0..2];

    for e in aslice {
        println!("{}", e);
    }

    assert_eq!(aslice, &[1, 1]); // what is this?
                                 // 也许是类似于字符串字面值一样的“数组字面值”
}