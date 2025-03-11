/* Multi
line comments
 */
fn main(/* you can touch this */) {
    let name = "Masud";
    println!("Hello, {}", name /* here as well */);
    let middle = 7/3;
    println!("Middle: {}", middle);

    let x = 5 + 3;
    let x = x * 2;
    let x = x - 6;
    let x = x / 2;
    println!("The answer is {}", x);

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
}