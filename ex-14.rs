use std::cell::RefCell;

fn main() {
    let mut hello = RefCell::new("World");
    let mut world = RefCell::new("Hello");
    // TODO: swap hello and world
    println!("{}, {}", hello.get_mut(), world.get_mut());
}
