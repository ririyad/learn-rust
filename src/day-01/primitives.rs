fn main() {
    let logical: bool = true;
    let a_float: f64 = 1.0; // regular annotation
    let an_integer = 5i32; // suffix annotation

    // Or a default will be used
    let default_float = 3.0; // f64
    let default_integer = 7; // i32

    //logical = false;
    println!("{}", logical);
    println!("{:.2}", a_float);
    println!("{}", an_integer);
    println!("{}", default_float);
    println!("{}", default_integer);

    let mut inferred_type = 12;
    println!("{}", inferred_type);
    inferred_type = 354545454;
    println!("{}", inferred_type);
}