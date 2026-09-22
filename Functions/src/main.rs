fn another_function(num: i32){
    println!("Another function is {}", num);
}

fn sum(num1: i32, num2: i32) -> i32{
    num1 + num2
}

fn sum_diff(num1: i32, num2: i32) -> (i32, i32){
    (num1 + num2, num2 - num1)
}


fn main() {
    another_function(32);
    println!("Hello, world!");

    //statements and Expressions
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    //Return value function

    let z = sum(102, 321);
    println!("The value of sum is: {}", z);

    let z2 = sum_diff(3, 5);
    println!("the sum = {} and diff =  {}", z2.0, z2.1);
    println!("The sum and diff is {:?}", z2);
}




