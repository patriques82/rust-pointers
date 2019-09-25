use std::cell::RefCell;

fn main() {
    let refc = RefCell::new(vec![12]);
    let mut inner = refc.borrow_mut();
    inner.push(24);
    println!("{:?}", *inner);
}
