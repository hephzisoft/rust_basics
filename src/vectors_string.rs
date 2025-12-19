// use std::fmt::Debug;

// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5];

//     {
//         let first = &v[0];
//         println!("The first element is: {first}");
//     }
//     v.push(6);

//     // let third  = &v[2];

//     // println!("The third element is {third}");

//     // let third   = v.get(2);
//     // match third {
//     //     Some(third) => println!("The third element is {third}"),

//     //     None=> println!("There is no third element")
//     // }
//     //

//     // let row = vec![
//     //     SpreadsheetCell::Int(3),
//     //     SpreadsheetCell::Text(String::from("blue")),
//     //     SpreadsheetCell::Float(10.12),
//     // ];
//     // println!("{:#?}", row);
//     //
//     let s1 = String::from("Hello, ");
//     let s2 = String::from("world!");
//     let s3 = s1 + &s2;
//     println!("{s3}");

//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");

//     let s = s1 + "-" + &s2 + "-" + &s3;
//     println!("{s}");

//     let hello = "Здравствуйте";
//     let answer = &hello[0..4];
//     println!("{answer}");
// }

// #[derive(Debug)]
// enum SpreadsheetCell {
//     Int(i32),
//     Float(f64),
//     Text(String),
// }
