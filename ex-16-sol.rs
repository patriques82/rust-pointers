use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    val: i32,
    tail: RefCell<Rc<Option<Node>>>,
}

fn main() {
    let nil = Rc::new(None);
    let b = Node { val: 3, tail: RefCell::new(nil.clone()) };
    let c = Node { val: 4, tail: RefCell::new(nil.clone()) };
    let a = Rc::new(Some(Node { val: 5, tail: RefCell::new(Rc::new(Some(
                                Node { val: 10, tail: RefCell::new(nil.clone()) }))) }));

    let mut btail = b.tail.borrow_mut();
    *btail = a.clone();

    let mut ctail = c.tail.borrow_mut();
    *ctail = a.clone();

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}
