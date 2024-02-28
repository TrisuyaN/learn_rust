// "borrow = use ref"

fn main() {
    let mut s1 = String::from("Genshin");
    let len = cal_length(&s1);
    println!("The length of '{}' is {}.", s1, len); // s1保留了所有权

    change(&mut s1);
    println!("{s1}");

    create_refs();

    let ref_to_nothing = danglingref();
}


fn cal_length(s: &String) -> usize {
    s.len()
} // 不会drop

fn change(s: &mut String) {
    s.push_str(" Start!");
}

fn create_refs() {
    let mut x = 114514; // 114514存储在栈区，是编译时确定大小的，不会有下述错误 // 你说的完全不对！
    //let mut x = String::from("senbai suki");
    let xref1 = &mut x;
    //let xref2 = &mut x;
    //let xref3 = &x;

    // 好奇怪如果**只使用最后一个引用**的话就能通过编译

    // 破案了，是在同一时间（意思是**同一个作用域**最多使用一个可变引用（此时连不可变引用都不可以））
    // 更新：不是同一个代码作用域（大括号），而是引用的作用域：即引用的声明到最后一次使用
    //*xref1+=1;
    //*xref2+=1;
    //*xref3+=1;
    //*xref3+=1;

    //{
    //    let xref = &mut x; // 代码作用域（大括号）也根本不好使
    //    println!("{}", xref);
    //}

    //let xref3 = x;
    println!("{}", xref1);
    let xref4 = &mut x;

    //println!("{}", xref2);
    //println!("{}", xref3);
    //println!("{}", xref4);


    let mut y = 1919810;
    let yref1 = &y;
    let yref2 = &mut y;
    //println!("{yref1}"); // 只要使用了就同样冲突，即使不可变引用在前。
                           // 不用的情况可以理解成引用作用域只有一行！！！
    println!("{yref2}");
}
// 总结：规则就是不可变引用和可变引用的引用作用域不能有重合部分！

// 尽管这些错误有时使人沮丧，
// 但请牢记这是 Rust 编译器在提前指出一个潜在的 bug（在编译时而不是在运行时）并精准显示问题所在。
// 这样你就不必去跟踪为何数据并不是你想象中的那样。

// 原文如此 好有道理 要是C++大概一定会出来乱七八糟的**未定义行为**，会好难找的

fn danglingref() -> &String{
    let s = String::from("hello");

    &s
}
// 哭了 它好智能
//
// error[E0106]: missing lifetime specifier
//   --> src/main.rs:70:21
//    |
// 70 | fn danglingref() -> &String{
//    |                     ^ expected named lifetime parameter
//    |
//    = help: this function's return type contains a borrowed value, but
// there is no value for it to be borrowed from
// help: consider using the `'static` lifetime, but this is uncommon unless
// you're returning a borrowed value from a `const` or a `static`
//    |
// 70 | fn danglingref() -> &'static String{
//    |                      +++++++
// help: instead, you are more likely to want to return an owned value
//    |
// 70 - fn danglingref() -> &String{
// 70 + fn danglingref() -> String{
//    |
// 目前可以选择将所有权随返回值移交
// 后续可以使用生命周期控制

// 概括：
// - 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。（这和我想的可变和不可变的引用作用域不重叠应该是一样）
// - 引用必须总是有效的。
