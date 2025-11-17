fn main() {
    let conditional1 = 5 <= 7;
    println!("{}", conditional1);

    // both types must be the same.
    //
    // Type casting can be used to offset unnecessary problem
    let conditonal2 = (5 as f32) < 7.2;
    println!("{}", conditonal2);
}
