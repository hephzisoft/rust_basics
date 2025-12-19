// fn main() {
//     let number = Some(10);

//     let booleans = Some(true);

//     let nothing: Option<i32> = None;

//     let x: Option<i32> = Some(10);

//     // Additoin
//     //
//     let y: i32 = 14;
//     // let sum = x + something;

//     //
//     let sum = x.unwrap_or(0) + y;

//     println!("{}", sum);

//     // match expression
//     // we have defined an ennum named movement
//     //
//     let player1 = Movement::Jump;
//     let player2 = Movement::Left;
//     let player3 = Movement::Right;

//     move_player(player1);
//     move_player(player2);
//     move_player(player3);
// }

// enum Movement {
//     //varaints
//     Left,
//     Right,
//     Jump,
// }

// fn move_player(m: Movement) {
//     // perform action depending on information
//     //
//     match m {
//         Movement::Left => println!("Moving Left"),
//         Movement::Right => println!("Moving Right"),
//         Movement::Jump => println!("Jumping up"),
//     }
// }
