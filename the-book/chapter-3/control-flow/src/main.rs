fn main() {
    r#if();
    if_non_bool();
    if_nonzero();
    if_else_chain();
    if_as_an_expression();
    if_expression_different_types();
}

// I wanted to name the function if() but this is a compile error
// so I used r# to escape the keyword at the compiler's suggestion
fn r#if() {
    let number = 3;
    if number < 5 { 
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn if_non_bool() {
    // this is a compile error
    // commenting it out so the crate will compile and run

    // let number = 3;
    // if number { // error: expected `bool`, found integer
    //     println!("number was three");
    // }
}

fn if_nonzero() {
    let number = 3;
    if number != 0 { // the right way to do "if number" is to compare against 0
        println!("number was non-zero");
    }
}

fn if_else_chain() {
    let number = 6;

    // "match" is a better fit for this kind of thing
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_as_an_expression() {
    let condition = true;

    // compare to C++
    // number = (condition ? 5 : 6);
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}

fn if_expression_different_types() {

    // the compiler tries to figure out what type `number` should be
    // but the different branches of the `if` evaluate to different types
    //     true branch: integer
    //     false branch: string

    // this is a compile error
    // commenting it out so the crate will compile and run
    // let condition = true;
    // let number = if condition { 5 } else { "six" }; // expected integer, found `&str`
    // println!("The value of number is: {}", number);
}