fn main() {
    // shadowing
    let x = 5;
    println!("The value of x is: {}", x);

    let x = 6;
    println!("The value of x is: {}", x);

    // In Rust, you can shadow a variable by reusing its name.
    // This is useful when you want to change the
    // type of a variable or when you want to reuse
    // a variable name in a different scope.
}
