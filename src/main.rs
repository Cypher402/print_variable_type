fn type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let x = "Hello";
    let y = 42;
    let var = 42.69;
    let b = true;

    type_of(&x);
    type_of(&y);
    type_of(&var);
    type_of(&b);

    type_inference();
}

fn type_inference() {
    let vartan: u8 = 3;
    type_of(&vartan);
    let narton = vartan;
    type_of(&narton);
    type_of(&vartan);
    println!("The Vars  {} {}", vartan, narton);
}