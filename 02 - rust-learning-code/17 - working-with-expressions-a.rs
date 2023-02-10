fn print (is_gt_100 : bool) {
    match is_gt_100 {
        true => println!("it's big"),
        false => println!("it's small"),
    }
}

fn main() {
    let value = 100;
    let is_gt_100 = value > 100; 
    print(is_gt_100);
}
