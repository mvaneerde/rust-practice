fn main() {
    f1();
    f2();
    f3();
    f4();
}

fn f1() {
    another_function(5);
    print_labelled_measurement(5, 'h');
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labelled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn f2() {
    // "let" is not an expression, but the block as a whole is
    // the value of the block is the last line
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn f3() {
    let x = five();
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}

fn f4() {
    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1 // this is an expression
    // Adding a semicolon would make it a statement and there would be nothing to return
    // this would be a compile error
}
