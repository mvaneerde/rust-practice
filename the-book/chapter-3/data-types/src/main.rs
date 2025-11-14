fn main() {
    f1();
}

fn f1() {
    let x_i8 : i8 = 0b1000_0000u8 as i8;
    let x_u8 : u8 = 0b1111_1111u8;
    let x_i16 : i16 = 1;
    let x_u16 : u16 = 1;
    let x_i32 : i32 = 1;
    let x_u32 : u32 = 1;
    let x_i64 : i64 = 1;
    let x_u64 : u64 = 1;
    let x_i128 : i128 = 1;
    let x_u128 : u128 = 1;
    let x_isize : isize = 1;
    let x_usize : usize = 1;

    println!("{x_i8}, {x_u8}, {x_i16}, {x_u16}, {x_i32}, {x_u32}, {x_i64}, {x_u64}, {x_i128}, {x_u128}, {x_isize}, {x_usize}");
}