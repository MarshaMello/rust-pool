enum color {
    Red,
    Pink,
    Blue,
    Yellow
}
fn paint(my_color: color) {
    match my_color {
        color :: Red => println!("Red"),
        color :: Pink => println!("Pink"),
        color :: Blue => println!("Blue"),
        color :: Yellow => println!("Yellow")

    }

}
fn main() {
    paint(color :: Blue);
}