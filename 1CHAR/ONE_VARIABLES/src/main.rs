fn variables() {
    let i8_int: i8 = 127;
    let i16_int: i16 = 10;
    let i32_int: i32 = 10;
    let i64_int: i64 = 10;
    let i128_int: i128 = 10;

    let u8_int: u8 = 10;
    let u16_int: u16 = 10;
    let u32_int: u32 = 10;
    let u64_int: u64 = 10;
    let u128_int: u128 = 10;
    println!("i8: {}", i8_int);
    println!("i16: {}", i16_int);
    println!("i32: {}", i32_int);
    println!("i64: {}", i64_int);
    println!("i128: {}", i128_int);
    println!("u8: {}", u8_int);
    println!("u16: {}", u16_int);
    println!("u32: {}", u32_int);
    println!("u64: {}", u64_int);
    println!("u128: {}", u128_int);
}

fn fornat() {
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    println!("decimal: {}", decimal);
    println!("hex: {}", hex);
    println!("octal: {}", octal);
    println!("binary: {}", binary);
    println!("byte: {}", byte);
}

fn main() {
    variables();
    fornat();
}
