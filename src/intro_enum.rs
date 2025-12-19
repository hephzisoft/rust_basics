// use std::thread::LocalKey;

// enum IpAddressKind {
//     // to store actual data inside your enum
//     // you just write() and whatever data you want inside
//     V4(u8, u8, u8, u8),
//     V6(String),
// }
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 }, // stores anonymouse struct
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// struct IpAddress {
//     kind: IpAddressKind, // this is our enum
//     address: String,
// }

// impl Message {
//     fn function1() {
//         println!("Code of the Future!");
//     }
// }

// fn main() {
//     // instances of each of our variants
//     //
//     let four = IpAddressKind::V4;

//     let six = IpAddressKind::V6;

//     // we can create a function that will take in our enum type
//     //
//     //
//     fn route(ip_kind: IpAddressKind) {
//         // input something in here
//     }

//     // let local_host = IpAddress {
//     //     kind: IpAddressKind::V4,
//     //     address: String::from("127.0.0.1"),
//     // };

//     // println!("{}", local_host.address);

//     // let local_host = IpAddressKind::V4(127, 0, 0, 10);
//     //

// }
