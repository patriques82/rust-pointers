use std::rc::Rc;

fn main() {
    let mut shared = Rc::new(6);
    {
        let mut copy = shared.clone();
    }
    println!("{:?}", Rc::get_mut(&mut shared));
}

