
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let data = Rc::new(RefCell::new(Vec::new()));
    data.borrow_mut().push(5);
    data.borrow_mut().push(6);
    println!("{:?}", data.borrow());
}
