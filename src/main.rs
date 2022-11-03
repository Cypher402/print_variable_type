fn main() {
    let x = "Hello";
    let y = 42;
    let var = 42.69;
    let b = true;

    TypeOf(&x);
    TypeOf(&y);
    TypeOf(&var);
    TypeOf(&b);
}

fn TypeOf<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());


}