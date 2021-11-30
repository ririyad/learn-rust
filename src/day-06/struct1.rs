// creating struct
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// // a unit struct
// struct Unit;

// // a tuple struct
// struct Pair(i32, f32);

// // a struct with two fields
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person{name, age};

    println!("{:?}", peter);

    // instantiate a point
    let point: Point = Point {x: 10.5, y: 11.5};
    println!("Point coorditaes: ({}, {})", point.x, point.y);

    // make a new point by using struct update syntax to use the value of other point
    let bottom_right = Point { x: 5.3, ..point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);
}