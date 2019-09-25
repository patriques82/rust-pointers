fn main() {
    let mut s = String::from("Hello");
    change(&mut s);
    watch(&s);
}

fn watch(some_string: &String) {
    println!("{}", some_string);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
