fn main() {

    //Basic control flow
    let number = 3;
    if number < 5{
        println!("condition was true");
    }
    else{
        println!("condition was false");
    }

    //if condition is an expression
    //another example

    let condition = false;

    let number = if condition{
        5
    } else {
        7
    };
    println!("the value of number is {}", number);

    //Nested If Else statements

    let num = 15;

    if num % 2 == 0{
        println!("{} is even", num)
    } else {
        println!("{} is odd", num);

        if num > 10 {
            println!("{} is greater than 10", num)
        } else {
            println!("{} is less than 10", num)
        }
    }

    // && and || operator

    let a = 10;
    let b = 5;
    let c = 20;

    //Both the conditions must be true
    if a > b && b > c{
        println!("a is grater than b and b is greater than c");
    } else {
        println!("Condition && did not meet");
    }

    //At least one of the conditions must be true
    if a > b || b > c{
        println!("a is grater than b and b is greater than c");
    } else {
        println!("The condition || did not meet");
    }


    //Match control flow
    //with enum we can define a type with a fixed set of values

    enum Coin{
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8{
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    //define coin
    let coin1 = Coin::Penny;
    let coin2 = Coin::Nickel;
    let coin3 = Coin::Dime;
    let coin4 = Coin::Quarter;
    println!("A has: {}", value_in_cents(coin1));
    println!("B has: {}", value_in_cents(coin2));
    println!("C has: {}", value_in_cents(coin3));
    println!("D has: {}", value_in_cents(coin4));

}
