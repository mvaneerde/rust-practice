fn main() {
    // simple comment

    // a comment with multiple lines
    // every line has to start with the double-slashes
    // so this is just the same as multiple comments in a row

    let lucky_number = "7"; // comments can follow code on the same line
    println!("My lucky number is: {lucky_number}");

    // putting the comment on the line before the code is more usual, though
    let lucky_number: i32 = lucky_number.parse().expect("Cannot parse {lucky_number} as i32");

    println!("My lucky number is still: {lucky_number}");

    // for completeness, there are also documentation comments
    // these will be explained later
    // see Chapter 14 > Publishing a Crate to Crates.io

    /* The Book doesn't mention it, but this also works */
    let lucky_number: i8 /* even inside a line */ = lucky_number as i8;
    /*
     * or across multiple lines
     * like this
     * (the leading * is just pretty) 
     */
    println!("My lucky number yet remains: {lucky_number}");
}
