fn main() {
    let name1:String = String::from("John");
    let name2:&String = &name1; //name1 is copied to name2
    println!("The name is {}", name2); // name1 is copied to name2
    println!("The name is {}", name1);
}
