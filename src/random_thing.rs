// // fn main() {
// //     let mut color_option1 = Colour {
// //         red: 0,
// //         green: 255,
// //         blue: 0,
// //     };

// //     color_option1.blue = 200;

// //     println!(
// //         "Colors:{}, {}, {}",
// //         color_option1.red, color_option1.blue, color_option1.green
// //     );
// //     generate_color(Colour {
// //         red: 0,
// //         blue: 90,
// //         green: 50,
// //     });
// // }
// struct Colour {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// fn generate_color(color: Colour) -> Colour {
//     return color;
// }

// use crossterm::{
//     execute,
//     style::{Color, Print, ResetColor, SetForegroundColor},
// };
// use std::io::{self, stdout};

// fn main() -> io::Result<()> {
//     let new_color = generate_color(Colour {
//         red: 200,
//         blue: 200,
//         green: 200,
//     });

//     // Set foreground color to a custom RGB value
//     execute!(
//         stdout(),
//         SetForegroundColor(Color::Rgb {
//             r: new_color.red,
//             g: new_color.green,
//             b: new_color.blue
//         }),
//         Print("This text is a custom RGB color!"),
//         ResetColor // Reset color to default
//     )?;

//     println!(); // Add a newline for better formatting

//     Ok(())
// }
