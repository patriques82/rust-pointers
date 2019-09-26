use std::rc::Rc;

fn main() {
    let shared = Rc::new(6);
    println!("{}", Rc::strong_count(&shared));
    print(shared.clone());
    println!("{}", Rc::strong_count(&shared));
}

fn print(copy: Rc<i32>) {
    println!("{}", Rc::strong_count(&copy));
}
