fn main() {
    f1();
    f2();
    f3();
    f4();
    f5();
    f6();
    f7();
    f8();
    f9();
}

fn f1() {
    // integer literals
    let x_i8 : i8 = -0b1000_0000i8;
    let x_u8 : u8 =  0b1111_1111u8;
    let x_i16 : i16 = -0b1000_0000_0000_0000i16;
    let x_u16 : u16 =  0b1111_1111_1111_1111u16;
    let x_i32 : i32 = -0b1000_0000_0000_0000_0000_0000_0000_0000i32;
    let x_u32 : u32 =  0b1111_1111_1111_1111_1111_1111_1111_1111u32;
    let x_i64 : i64 = -0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000i64;
    let x_u64 : u64 =  0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111u64;
    let x_i128 : i128 = -0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000i128;
    let x_u128 : u128 =  0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111u128;
    let x_isize : isize = isize::MIN;
    let x_usize : usize = usize::MAX;

    println!("i8/u8      : {x_i8}/{x_u8}");
    println!("i16/u16    : {x_i16}/{x_u16}");
    println!("i32/u32    : {x_i32}/{x_u32}");
    println!("i64/u64    : {x_i64}/{x_u64}");
    println!("i128/u128  : {x_i128}/{x_u128}");
    println!("isize/usize: {x_isize}/{x_usize}");
}

fn f2() {
    // floating-point data types
    let x = 2.0; // f64
    let y: f32 = 3.0; // explicit
    println!("x = {x}");
    println!("y = {y}"); 
}

fn f3() {
    // operations

    // addition
    let sum = 5 + 10;
    println!("5 + 10 = {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3 = {difference}");

    // multiplication
    let product = 4 * 30;
    println!("4 * 30 = {product}");

    // quotient
    let quotient = 56.7 / 32.2;
    println!("56.7 / 32.2 = {quotient}");

    let truncated = -5 / 3;
    println!("-5 / 3 = {truncated} -- note this is truncated");

    // remainder
    let remainder = 43 % 5;
    println!("43 % 5 = {remainder}");
}

fn f4() {
    // booleans

    let t = true;
    println!("t = {t}");

    let f: bool = false; // explicit type
    println!("f = {f}");
}

fn f5() {
    // characters

    let c = 'z';
    println!("c = {c}");

    let z: char = 'ℤ'; // explicit type
    println!("z = {z}");

    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat = {heart_eyed_cat}");
}

fn f6() {
    // tuples
    let tup = (500, 6.4, 1);
    println!("tup = {:?}", tup);

    let (x, y, z) = tup;
    println!("x = {x}");
    println!("y = {y}");
    println!("z = {z}");
}

fn f7() {
    // accessing elements of a tuple
    let x : (i32, f64, u8) = (500, 6.4, 1);
    println!("x = {:?}", x);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("five_hundred = {five_hundred}");
    println!("six_point_four = {six_point_four}");
    println!("one = {one}");
}

fn f8() {
    // the "unit" - that is, the empty tuple
    let u : () = ();
    println!("u = {:?}", u);

    // can compare the unit to itself
    println!("u == (): {}", u == ());

    // comparing to other types fails compile
    // this includes comparing to non-empty tuples
}

fn f9() {
    // arrays
    let a = [1, 2, 3, 4, 5];
    println!("a = {:?}", a);

    let first = a[0];
    let second = a[1];
    println!("first = {first}");
    println!("second = {second}");
}