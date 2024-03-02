fn main() {
    // use enum as structs
    // bind data to enumtypes
    // let loopback = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V4(127, 0, 0, 1);

    use_option();
}

enum IpAddrKind {
    V4,
    V6,
}

// enum IpAddr {
//     V4(String),
//     V6(String),
// }

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}


// a enum is better than structs
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
struct QuitMessage; // 类单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体


// enum Option<T> {
//     None,
//     Some(T),
// }
fn use_option(){
    let some_number = Some(5);
    let some_char = Some('e');
    let some_ip = Some(IpAddr::V4(114, 5, 4, 19));

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // not allow to use a value which is possible to be None with normal values
    // use MATCH
    let sum = x + y;

}