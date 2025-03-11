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
}