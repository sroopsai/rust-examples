fn main() {
    // Integers 1, floats 1.2, characters 'a', strings "abc"
    // booleans true, unit type () can be expressed using literals

    // Integers can be expressed using hexadecimal, octal or binary
    // notation using prefixes 0x, 0o, 0b

    // Insert underscores to improve readability
    // 1_000_000 = 1000000
    // 0.000_001 = 0.000001

    // Support E-notation 1e6, type associated is f64
    // Tell compiler type of literals we use

    println!("1 + 2 = {}", 1u32 + 2);

    // Integer Subtraction
    // attempt to compute `1_u32 - 2_u32`, which would overflow
    println!("1-2 = {}", 1i32 - 2);

    // Scientific Notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // use underscores to improve readablity
    println!("One million is written as {}", 1_000_000u32);



}
