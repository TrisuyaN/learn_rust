fn func_can_before_or_after_main(){
    println!("A useless function!");
}

fn main() {
    println!("Hello, world!");
    foo_bar_func();
    func_can_before_or_after_main();
    func_with_args(114514);

    let mut x = func_with_retvals();
    println!("{x}");
   
    x = senbai();
    println!("{x}");
}

fn foo_bar_func(){
    println!("A useless function!");
}

fn func_with_args(x: i32){
    println!("A useless function got x: {x}");
}

fn func_with_retvals() -> i32 {
    let x = {
        let t =114513;
        t + 1 // no ';'
    }; // between '{' and '}' is a EXPRESSION
    x + 1 // it's a EXPRESSION
}

fn senbai() -> i32{
    114514
}
