fn main() {
    let mut s = String::from("Hello");
    watch(&s);
    change(&mut s);
    watch(&s);
    assert_eq!("Hello, Pointers", s);
}

// TODO implement watch

// TODO implement change
