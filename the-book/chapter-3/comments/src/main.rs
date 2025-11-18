fn main() {
    // simple comment

    // a comment with multiple lines
    // every line has to start with the double-slashes
    // so this is just the same as multiple comments in a row

    let lucky_number = "7"; // comments can follow code on the same line

    // putting the comment on the line before the code is more usual, though
    let lucky_number: i32 = lucky_number.parse().expect("Cannot parse {lucky_number} as i32");

    println!("My lucky number is: {lucky_number}");

    // for completeness, there are also documentation comments
    // these will be explained later
    // see Chapter 14 > Publishing a Crate to Crates.io
}
