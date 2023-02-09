 struct Items {
    stock : i32,
    price : f64,

 }


 fn main() {
    let list = Items {
        stock : 10,
        price : 2.99,
    };
    println!("stock : {:?}", list.stock);
    println!("price : {:?}", list.price);
 }