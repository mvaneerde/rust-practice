const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    f1();
    f2();
    f3();
    f4();
    f5();
}

fn f1() {
    // let x = 5; compile error when we assign 6 to it later
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn f2() {
    println!("The number of hours in three seconds is {THREE_HOURS_IN_SECONDS}");
}

fn f3() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn f4() {
    let spaces = "    ";
    let spaces = spaces.len();

    println!("Number of spaces: {spaces}");
}

fn f5() {
    #[allow(unused_mut)]
    let mut spaces = "    ";
    // spaces = spaces.len(); compile error when changing types
    println!("Spaces: \"{spaces}\"");
}