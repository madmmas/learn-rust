fn hello_world() {
    let name = "Masud";
    println!("Hello, {}", name /* here as well */);
    let middle = 7/3;
    println!("Middle: {}", middle);
}

fn with_param(x: i32) {
    println!("Print the first value of x: {}", x);
    let x = x + 3;
    let x = x * 4;
    let x = x - 6;
    let x = x / 2;
    println!("The answer is {}", x);
}

fn sqrt(x: i32) -> i32 {
    x * x //; wrong as it will return - unit ()
}

fn this_works() {
    let temp = 20;
    if temp > 15 && temp < 27 {
        println!("It's fairly comfortable in here!");
        "Hello, world!"; // will not print
        // "will not print" but will provide error as this is an if-block
        () // this is correct as if-block return unit
    }
    if true {
        println!("Here this if will be called with no error for end of semicolon!")
    }
}

fn this_return() -> String { // here &str will not work as that is static string
    if true {
        return "hello world!".to_string();
    }
    "".to_string()
}

fn interesting_block() {
    let message = {
        "Hello, world!"
    };
    println!("Interesting {}", message)
}

fn interesting_if(temp: i32) {
    let message = if temp <= 10 {
        "It's cold!"
    } else if temp <= 25 {
        "It's nice"
    } else if temp <= 30 {
        "It's warm" // putting semicolon here will be a problem or any other type
        // 33 <- this one is error as well
        // the message variable assumed static string by the compiler
    } else { // a must block as the if block assigned in a variable
        "It's hot!"
    };

    println!("{}", message);
}

fn fact(x: u32) -> u32 {
    if x == 0 {
        1
    } else { // here else block is important
        x * fact(x - 1)
    }
}

fn fact2(x: u32) -> u32 {
    if x == 0 {
        return 1;
    }
    x * fact(x - 1)
}

fn sum(low: i32, high: i32) -> i32 {
    let mut sum = 0;
    let mut low = low; // shadowing low as func param low is immutable
    while low <= high {
        sum += low;
        low += 1;
    }
    sum
}

fn nested_loop(n: i32) {
    let mut row = 0;
    while row < n {
        let mut col = 0;
        while col < n {
            // if statement in the print macro
            print!("{}", if row == col { "X" } else { "O" });
            col += 1;
        }
        println!("");
        row += 1;
    }
}

fn let_without_assignment() {
    let x;
    let is_raining = true;
    if is_raining {
        x = 5;
    } else {
        x = 6;
    }
    println!("let_without_assignment: x == {}", x);

    let mut y;
    y = 5;
    y += 1;
    println!("let_without_assignment: y == {}", y);
}
/* Multi
line comments
 */
fn main(/* you can touch this */) -> () {

    hello_world();
    with_param(1200);
    println!("sqrt of {} is {}", 20, sqrt(20));

    let x = 5 + 3;
    let x = x * (6 - 2);
    println!("{}", x);

    let name: &str = "James";
    let age: i8 = 34;
    println!("The answer is {} and age {}", name, age);

    { // variable scope
        let x = 5;
        let x = x + 1;
        let x = x * 2;
        println!("The answer is {}", x);
    }

    // interesting RUST
    let x: () = (); //See: () works for both the type and the value.
    let y: () = println!("Hello, world!" /* here the macro returns () - means empty result */);
    assert_eq!(x, y); // asserting values of x and y where both are empty
    println!("All units are the same!");

    // put underscore for unused variable
    let _unused = "UNUSED";

    // it will print
    let _print = println!("This will print even though variable is unused.");

    // Interesting block
    let x: i32 = { 4 + 5 };
    println!("4 + 5 == {}", x);

    let x: i32 = {
        6 + 7; // this will give us a warning but not error
        let _ = 6 + 10; // this will not give us any warning or error
        println!("I will be print!");
        4 + 5 //; putting semicolon in the end of the block will give us error
        // because rust return () for this block
        // () is called "unit"
    };
    println!("4 + 5 == {}", x);

    let z: i32 = { // you can think the block is anonymous and run immediate
        let x = 3 * 5;
        println!("The value of z is {}", x);
        x
    };
    println!("z is {}", z);

    println!("4 + 5 == {}",  {{4 + 5}}); // this also valid in RUST, but will provide warning

    this_works();

    interesting_if(15);

    println!("Fact of 5: {}", fact(5));
    println!("Fact of 6: {}", fact2(6));

    // checking while loop
    assert_eq!(55, sum(1, 10));

    // nested loop example
    nested_loop(3);

    let_without_assignment();
    this_return(); //will give error if semicolon removed

    interesting_block()
}