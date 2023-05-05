fn main() {
    println!("Hello, world!");

    // Calling a function
    another_function(9);

    // Calling a function with implicit return
    let x = five();
    println!("Function five(): The value of x is {x}");

    // Example of let/if construct (using if as an expression)
    let condition = true;
    let y = if condition { 10 } else { 100 };
    println!("Let/If: The value of y is {y}");

    // loop construct
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;  // Why do we need a ';' here. The code works without it too. What does that signify?
        }
    };
    println!("Loop counter: The value of counter is {result}");


    // Using inner loops with labeled break
    let mut count = 0;
    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;
        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count: {count}");

    // Using for loop for collections
    let int_array = [1, 2, 4, 8, 16];

    for element in int_array {
        println!("element: {element}");
    }

    // Using for loop with Range (in reverse)
    for i in (1..4).rev() {
        println!("Countdown: {i}");
    }
    println!("Liftoff!");
}

fn another_function(x: i32) {
    println!("Another function: The value of x is {x}");
}

fn five() -> i32 {
    5
}
