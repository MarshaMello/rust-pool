enum Dir {
    left,
    right
}

fn main() {
    let go = Dir :: left;
    match go {
        Dir :: left => println!("go left"),
        Dir :: right => println!("go right"),
    }
}