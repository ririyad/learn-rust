fn main() {
    let number = 4;
    if number < 5 {
        println!("Number is less than 5");
    } else {
        println!("Number is greater than 5");
    }

    // using if in a let statement
    // as if is an expression, we can use it on the right side on a let statement
    // type of if arm and else arm should be the same type otherwise it will thorw an error
    let condition = true;
    let number = if condition { 5 } else { 6 };
    let number2 = if condition { 6 } else { 9 };
    println!("number is: {}", number);
    println!("number is: {}", number2);
}