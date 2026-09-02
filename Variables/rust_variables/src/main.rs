fn main() {
    let mut x:i32 = 5; //by default, rust is immutable, so without mut command it will throw an error
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);
}
