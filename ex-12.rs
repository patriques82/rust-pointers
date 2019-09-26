use std::rc::Rc;

fn main() {
    let shared = Rc::new(6);
    println!("{}", Rc::strong_count(&shared));
    println!("{}", Rc::strong_count(&shared));
    println!("{}", Rc::strong_count(&shared));

    // TODO: get strong_count to print 1 2 1
}

