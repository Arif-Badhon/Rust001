
struct Person{
    name:String,
    age:u8,
}

enum TrafficLights{
    Green,
    Yellow,
    Red,
}

fn main() {

    //Data Types
    // Scalar Types
    // -Integers
    let small_number: u8 = 255;
    let big_number: u128 = 2147483648;
    let small_number2: i8 = -127;
    let big_number2: i128 = -123456789098765432;

    println!("The value of small_number is: {}", small_number);
    println!("The value of big_number is: {}", big_number);
    println!("The value of small_number2 is: {}", small_number2);
    println!("The value of big_number2 is: {}", big_number2);


    let decimal =  98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let _byte = b'A';
    println!("The value of decimal is: {}", decimal);
    println!("The value of hex is: {}", hex);
    println!("The value of octal is: {}", octal);
    println!("The value of binary is: {}", binary);
    println!("The value of byte is: {}", _byte);

    //Floating Points
    let x = 2.0; //f64 default
    let y: f32 = 3.0;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    //Numeric Operations
    let sum = x + y;
    let difference = x - y;
    let product = x * y;
    let quotient = x / y;
    let remainder = x % y;
    println!("The value of sum is: {}", sum);
    println!("The value of difference is: {}", difference);
    println!("The value of product is: {}", product);
    println!("The value of quotient is: {}", quotient);
    println!("The value of remainder is: {}", remainder);


    //Booleans
    let t = true; //implicit declaration
    let f: bool = false; //explicit declaration

    println!("The value of t is: {} and f is: {}", t,f);

    //if and else situation
    if t{
        println!("t is true")
    } else { println!("t is false") }

    let not_t = !t;
    println!("The value of not_t is: {}", not_t);

    //There is no default value assigned to variable let b:bool will return error


    //Characters
    let c = 'z';
    let x = 'X';

    println!("The value of c and x is: {} and {}", c,x);


    //Tuples
    let tup: (i32, f64, char) = (500, 6.4, 'X');

    //destructuring
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    println!("The value of x is: {}", x);

    //Accessing by index
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let x_char = tup.2;

    println!("The value of x_char is: {}", x_char);
    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);


    //Arrays
    let arr = [1, 2, 3, 4, 5];

    let first = arr[0];
    let second = arr[1];

    println!("first: {}, second: {}", first, second);


    for i in arr.iter() {
        println!("i: {}", i);
    }

    //Structs
    // defined out of main function
    let person1 = Person{
        name: String::from("David"),
        age: 18,
    };

    let person2 = Person{
      name: String::from("Miles"),
        age: 30,
    };

    println!("The person1 name is: {} and age is {}", person1.name, person1.age);
    println!("The person2 name is: {} and age is {}", person2.name, person2.age);

    //Enums
    //defined out of main function
    let light1 = TrafficLights::Green;

    match light1 {
        TrafficLights::Green => println!("Go!"),
        TrafficLights::Yellow => println!("Slow Down"),
        TrafficLights::Red => println!("Stop!"),
    }

    let light2 = TrafficLights::Red;

    match light2 {
        TrafficLights::Green => println!("Go!"),
        TrafficLights::Yellow => println!("Slow Down"),
        TrafficLights::Red => println!("Stop!"),
    }

    let light3 = TrafficLights::Yellow;

    match light3 {
        TrafficLights::Green => println!("Go!"),
        TrafficLights::Yellow => println!("Slow Down"),
        TrafficLights::Red => println!("Stop!"),
    }
}
