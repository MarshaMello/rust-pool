//index start with 0 num :)


fn main() {
    let size = (2, 3);
    println!("{:?}, {:?}", size.0, size.1);


    let (x, y) = (2, 3);
    println!("{:?}, {:?}", x, y);

    let (name, age) = ("Hamed", 20);

    let favorites = ("red", 14, "TX", "pizza", "WA", "home");
    let state = favorites.2;
    let place = favorites.5;
}