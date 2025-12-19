use std::{collections::HashMap, io};

fn main() {
    // design the text interface
    let mut text_interface = String::new();

    io::stdin()
        .read_line(&mut text_interface)
        .expect("Failed to read line");

    let instructions: Vec<&str> = text_interface.split_ascii_whitespace().collect();

    let departments: [&str; 4] = ["Science", "Technology", "Sales", "Finance"];

    let all_people_in_department : HashMap<&str,Vec<String> > =HashMap::new();
}
