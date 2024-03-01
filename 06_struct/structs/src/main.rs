fn main() {
    let a1 = Account {
        name: String::from("114"),
        age: 514,
        email: String::from("1919"),
        available: true,
    };

    let mut a2: Account;
    a2 = Account{
        name: String::from("114"),
        age: 514,
        email: String::from("1919"),
        available: true,
    };
    a2.age = 810;
    println!("{}", a2.email);
    
}

struct Account {
    name: String,
    age: i32,
    email: String,
    available: bool,
}

fn build_acc(name: &str, email: &str) -> Account {
    Account {
        name: String::from(name),
        age: 514,
        email: String::from(email),
        available: true,
    }
}
// fn build_acc2(name: &str, email: &str) -> Account {
//     Account {
//         name,
//         age: 514,
//         email,
//         available: true,
//     }
// }
fn build_acc3(name: String, email: String) -> Account {
    Account {
        name,
        age: 514,
        email,
        available: true,
    }
}

fn upd(){
    let a1 = Account {
        name: String::from("114"),
        age: 514,
        email: String::from("1919"),
        available: true,
    };

    let a2 = Account {
        name: a1.name, // move from a1
        age: 515,
        email: a1.email, // move from a1
        available: a1.available,
    };

    let a3 = Account {
        email: String::from("genshin@mihoyo.cn"),
        name: String::from("honkai"),
        ..a1 // move (String) and copy (others)
    };

    let a4 = Account {
        ..a2 // move and copy
    };
}

fn struct_tuple(){
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(1, 1, 4);
    let p = Point(5, 1, 4);
    //let p2: Point = black;
}


// 我们也可以定义一个没有任何字段的结构体！它们被称为 类单元结构体（unit-like structs）因为它们类似于 ()，
// 即“元组类型”一节中提到的 unit 类型。类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储
// 数据的时候发挥作用。
fn unit_like_structs(){
    struct Eatable;

    struct Food{
        u: Eatable,
        name: String,
    }
}