fn main() {
    println!("Hello, world!");

    another_function(9);

    let x = five();
    println!("Function five(): The value of x is {x}");
}

fn another_function(x: i32) {
    println!("Another function: The value of x is {x}");
}

fn five() -> i32 {
    5
}
