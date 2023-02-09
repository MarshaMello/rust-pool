enum Menu {
    Burger,
    Fries,
    Drink,
}

let paid = true;
let item = Menu :: Drink;
let dirnk_type = "water";
let order_placed = match item {
    Menu :: Drink => {
        if dirnk_type == "water" {
            true
        } else {
            false
        }
    }
    _ => true,
};