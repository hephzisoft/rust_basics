// fn main() {
//     let conditional1 = 5 <= 7;
//     println!("{}", conditional1);

//     // both types must be the same.
//     //
//     // Type casting can be used to offset unnecessary problem
//     let conditonal2 = (5 as f32) > 7.2;
//     println!("{}", conditonal2);

//     let conditon3 = true && !conditonal2;
//     println!("{}", conditon3);
//     // control flow

//     let number = 10;

//     if number == 100 {
//         println!("The number is 10");
//     } else {
//         println!("The number is not 10")
//     }

//     // short hand control flow
//     //
//     let age = 105;
//     let teenager = if age >= 13 && age < 18 { true } else { false };

//     println!("{}", teenager);

//     // loops
//     // these are used to iterate until a conditon is met
//     //
//     //
//     let mut count = 0;
//     // loop {
//     //     count += 1;
//     //     println!("Number count is: {}", count);

//     //     if count == 10 {
//     //         break;
//     //     }
//     // }

//     // multiplication table
//     //
//     // let mut no = 1;
//     // let table = 3;

//     // loop {
//     //     println!("{no} * {table} = {multiply}", multiply = no * table);
//     //     no += 1;
//     //     if no > 12 {
//     //         break;
//     //     }
//     // }
//     // //
//     // while count <= 100 {
//     //     if count % 15 == 0 {
//     //         println!("Fizzbuzz");
//     //     } else if count % 5 == 0 {
//     //         println!("Buzz");
//     //     } else if count % 3 == 0 {
//     //         println!("Fizz");
//     //     } else {
//     //         println!("{count}");
//     //     }
//     //     count += 1;
//     // }

//     // For range loop
//     for x in 0..10 {
//         println!("{x}");
//     }

//     // Iterating loop
//     let a = [10, 20, 30, 40];

//     for element in a.iter() {
//         println!("value is {element}");
//     }
// }
