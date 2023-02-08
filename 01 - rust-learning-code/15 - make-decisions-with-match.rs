fn main() {
    let my_name = "John";
    match my_name {
        "Hamed" => println!("that is my name"),
        "John" => println!("not my name"),
        "Alice" => println!("hello alice"),
        _ => println!("great to see you !")
    }
}