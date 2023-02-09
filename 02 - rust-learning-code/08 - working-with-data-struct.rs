struct box_size {
    depth : i32,
    width : i32,
    height : i32,
}
let my_box = box_size {
    depth : 3,
    width : 2,
    height : 5,
}

let tall = my_box.box_size;
println!("the box is {:?} units tall", tall);

