// the function
fn add(a: i32, b: i32) -> i32 {
    a + b
}
//
// declare a varible with let
// and use mute to note that the variable is mmutable
fn main() {
    let a = 123;
    let mut b = 456;

    b = 10;
    // a = 10; //the a connot be changed because it is not mutable

    println!("{}", a);
    println!("{}", b);

    // the mutable variable can be assigned a new value
    b = 1000;
    // and b can only be the int instead of the float like 12.12 or the string like "hello"
    // the float can not be assigned to the int variable becuase there are loss of precision
    // the immutable variable can be redeclared
    let a = 132;
    println!("{}", a);
    println!("{}", b);

    // the type of the variable can be declared
    let c: i32 = 123;
    println!("{}", c);
    let d: f64 = 123.123;
    println!("{}", d);

    // the float64 and boolean value
    let e: f64 = 123.123;
    println!("{}", e);
    let f: bool = true;
    println!("{}", f);

    // the const value can not be changed or redeclared
    const G: i32 = 213;
    // the following code will cause the error
    // G = 123;
    // let G = 123;
    println!("{}", G);

    // use the function
    println!("{}", add(1, 2));
    // use the conditional statement
    if f {
        println!("this your last chance");
    }

    // the loop statement
    let mut count: u32 = 0;
    loop {
        count += 1;
        if count > 10 {
            println!("breaking");
            break;
        }
        println!("this your last chance");
    }

    // for loop
    for i in 0..10 {
        println!("{}", i);
    }

    // OWNERSHIP
    let s = String::from("this is a test");
    // simple test
    println!("{}", s);
    // moving the ownership
    let mut s2 = s;
    // the s should be moved to s2
    // and any ref or use of s will cause the error
    // println!("{}", s); // INVALID NOW
    println!("{}", s2);

    // if we want to copy the value of s2 to s3
    // we can use the clone method
    let s3 = s2.clone();
    println!("{}", s3);

    // if we want to just borrow the value of s2
    let s4 = &s2;
    let s5 = &s2;
    println!("{}, {}", s4, s5);

    // if we want to change the value of s2 using the reference
    let s6 = &mut s2;
    s6.push_str("sadfhasdk");
    println!("{}", s6);

    // structure :
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        username: String::from("test_username"),
        email: String::from("test@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("{}, {}", four as i32, six as i32);

    // match
    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Nickel));
    println!("{}", value_in_cents(Coin::Dime));


}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

enum Result <T, E> {
    Ok(T),
    Err(E),
}

fn Result<T, E> 
{
    if b == 0 {
        if b == 0 {
            Err(String::from ("Division by zero"))
        } else {
            Ok(a / b)
        }
    }
}

