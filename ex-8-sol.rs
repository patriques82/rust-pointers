fn main() {
    let mut s = String::from("Hello");
    watch(&s);
    change(&mut s);
    watch(&s);
    assert_eq!("Hello, Pointers", s);
}

fn watch(some_string: &String) {
    println!("{}", some_string);
}

fn change(some_string: &mut String) {
    some_string.push_str(", Pointers");
}
