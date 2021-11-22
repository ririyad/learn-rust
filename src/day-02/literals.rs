fn main() {

    // integer addition
    println!("1+2 = {}", 1u32+2);

    println!("1-2={}", 1i32-2);

    println!("true AND false is: {}", true && false);
    println!("true OR false is: {}", true || false);
    println!("not true is: {}", !true);

    // bitwise operation
    println!("0011 AND 0101 is {:4b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:4b}", 0b0011 | 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is:  {:x}", 0x80u32 >> 2);

    // one million
    println!("One million can be written as: {}", 1_00_000);


}