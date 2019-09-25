enum List<'a> {
    Cons(i32, &'a List<'a>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    Cons(1, &Cons(2, &Cons(3, &Nil)));
}
