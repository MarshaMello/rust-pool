//this code will work properly :)

let a =99;
if a > 500 {
    println!("huge number");
} else if a > 99 {
    println!("big number");
} else {
    println!("small number");
}

// this code won't work properly :(

    if a > 99 {
        println!("big number");
    } else if a > 300 {
        println!("huge number");
    } else {
        println!("small number");
    }