use std::cell::Cell;

fn main() {
    let a = 5;
    let c = Cell::new(a);
    c.set(20);
    println!("{}", c.get());
}
