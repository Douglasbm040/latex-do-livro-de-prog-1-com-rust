fn modify(x: &mut i32) {
    *x += 1;
}

fn main() {
    let mut my_var = 42;
    modify(&mut my_var);
    println!("{}", my_var);
}
