// 
//- calculate the area of a rectangle with functions and methods
//- auto ref and deref
//


fn main() {
    use_struct();
    use_dbg_or_println();
    auto_ref_and_deref();
}


fn area1(a: u32, b: u32) -> u32 {
    a * b
}
// use tuple to increase readability
fn area2(rec: (u32, u32)) -> u32 {
    rec.0 * rec.1 // we still don't know what is 0 and 1
}
// use ref because we don't want it's owership
fn area3(rec: &Rectangle) -> u32 {
    rec.height * rec.width
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// associated functions
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}
impl Rectangle {
    fn contain(&self, rec: &Rectangle) -> bool {
        self.width >= rec.width && self.height >= rec.height
    }
}
// not a associated functions
impl Rectangle {
    fn ok(){ // without arg self
        println!("nothing to do a specific rectangle");
    }
}

fn use_struct(){
    let a = 114;
    let b = 514;
    //let b = -514; // compiler assumed it to be a u32 when checking?!
    //let c = -19; // ok! SAFE!
    println!("The area is {}", area1(a, b));


    let rec1 = (1919, 810);
    println!("The area is {}", area2(rec1));

    let rec2 = Rectangle {
        height: 123,
        width: 456,
    };
    println!("The area is {}", area3(&rec2));

    //println!("Rectangle: {}", rec2);
    println!("Rectangle: {:#?}", rec2);

    println!("Rectangle: {}", rec2.area());
    if rec2.width() {
        println!("ok");
    }

    let rec3 = Rectangle {
        height: 124,
        width: 456,
    };
    if rec2.contain(&rec3) {
        println!("can contain");
    } else {
        println!("TT");
    }
    
    // use unassociated function
    rec3.ok(); // impossible
    Rectangle::ok();
}




#[derive(Debug)]
struct S {
    a: String,
    b: String,
}
impl S {
    // if use fn ok() without parameter &self, 
    // then can't invoke it with '.ok()'
    fn useref(&self) {
        println!("Use ref");
    }
}
impl S {
    fn usemutref(&mut self){
        println!("Use mut ref");
    }
}
impl S {
    fn useself(self){
        println!("Use self");
    }
}


fn use_dbg_or_println(){
    let s = S {
        a: String::from("asdasd"),
        b: String::from("asdasdasda"),
    };
    println!("{:#?}", s);
    println!("{:#?}", s); // use ref

    //dbg!(s); // get and return ownership
    let t = dbg!(s); // or s = dbg!(s) if s is mut
    println!("{:#?}", t);
}

fn auto_ref_and_deref(){
    let mut s = S {
        a: String::from("d"),
        b: String::from("c"),
    };

    s.useref(); // auto ref
    (&s).useref(); // no need

    s.usemutref(); // auto ref
    (&mut s).usemutref(); // no need

    let sref = &s;
    s.useself(); // auto deref 
}
// is there ref's ref in Rust?