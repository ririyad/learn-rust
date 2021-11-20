fn main() {

    println!("Hello from Rust");
    
    // here, println!() prints text to the console and it's "!" indicate that 
    //its a macro rather than a function
    println!("Hello from Rust again");
    println!("{} days", 31);

    // positional argument
    println!("{0} this is {1}, {1} this is {0}", "Alice", "Bob");

    //named argument
    println!("{subject} {verb} {object}",
            subject="the quick brown fox",
            verb="jumps over",
            object="the lazy dog");

    //special formatting (converting to binary)
    println!("{} is {:b} people know binary, the other one doesn't", 1, 2);

    // special formatting (padding numbers)
    println!("{number:>width$}", number=1, width=6);

    // special formatting (padding numbers with extra 0)
    println!("{number:0>width$}", number=1, width=6);

    // activity
    let pi = 3.141592;
    // rounding pi value to 3 digits
    println!("pi is {:.3}", pi);


}