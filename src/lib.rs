use std::io;
pub fn us_input () -> String {
    println!("what did you want to Print");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    input
}