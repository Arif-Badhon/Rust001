fn main() {
    let name1:String = String::from("John");
    let name2:String = name1;
    println!("The name is {}", name2); // name1 is moved to name2
}
