fn main() {
    let mut count = 0;
    // creating a loop label
    'counting_up: loop {
        println!("Count: {}", count);
        let mut remaining = 10;
        loop {
            println!("Remaining: {}", remaining);
            if remaining == 8 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count: {}", count);
}