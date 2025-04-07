// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.

macro_rules! my_macro {
    ($val:expr) => {
        println!("Macro says: {}", $val);
    };
}

fn main() {
    my_macro!("Hello, world!");
}
