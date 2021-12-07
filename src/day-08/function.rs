// FizzBazz
// a function to hold the loop range
// a function to verify the FizzBazz check
// a function to return the result of FizzBuzz check

fn main() {
  fizzbuzz_to(200);
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
  if rhs == 0 {
    return false;
  }
  lhs % rhs == 0
}

// function that return a value, returns the unit type
fn fizzbuzz(n: u32) -> () {
  if is_divisible_by(n, 15) {
    println!("fizzbuzz");
  } else if is_divisible_by(n, 3) {
    println!("fizz");
  } else if is_divisible_by(n, 5) {
    println!("buzz");
  }else {
    println!("{}", n);
  }
}

// if the function returns (), the return type can be ommitted
fn fizzbuzz_to(n: u32) {
  for n in 1..=n {
    fizzbuzz(n);
  }  
}
