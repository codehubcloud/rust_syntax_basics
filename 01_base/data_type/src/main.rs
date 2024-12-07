fn variable_func()
{
    let var_x: i32 = 5;
    let mut mut_var_y: i64 = 6;
    println!("The value of mut_var_y is: {}", mut_var_y);
    mut_var_y = 8;

    println!("The value of var_x is: {}", var_x);
    println!("The value of mut_var_y is: {}", mut_var_y);
}

fn guess_func()
{
    let guess: u32 = "42".parse().expect("not a number");
    println!("guess number: {guess}");
    print!("guess number: {guess}\n")
}

fn interger_type_func()
{
    let i8_a: i8 = -8;
    let i16_a: i16 = -16;
    let i32_a: i32 = -32;
    let i64_a: i64 = -64;
    let i128_a: i128 = -128;
    let isize_a: isize = 3;

    println!("{i8_a} {i16_a} {i32_a} {i64_a} {i128_a} {isize_a}\n");

    let u8_b: u8 = 8;
    let u16_b: u16 = 16;
    let u32_b: u32 = 32;
    let u64_b: u64 = 64;
    let u128_b: u128 = 128;
    let usize_b: usize = 3;
    println!("{u8_b} {u16_b} {u32_b} {u64_b} {u128_b} {usize_b}\n");
}

fn float_type_func()
{
    let f32_a: f32 = 32.5;
    let f64_a: f64 = 25.0000000002;
    println!("{f32_a}, {f64_a}");
}

fn number_operation()
{
    let add: i32 = 10 + 5;
    let sub: f64 = 95.00000000005 - 25.3;
    let multi: u32 = 30 * 4;
    let divide: f32 = 56.7 / 32.2;
    let remainder: i32 = 54 % 5;
    println!("{add}, {sub}, {multi}, {divide}, {remainder}");
}

fn bool_type_func()
{
    let bl_t: bool = true;
    let bl_f: bool = false;
    println!("{bl_t}, {bl_f}");
}

fn char_type_func()
{
    let x: char = 'z';
    let y: char = '√';
    let z: char = '\u{1F378}';
    println!("{x}, {y}, {z}");
}

fn tuple_type_func()
{
    let tup: (i32, f64, u8) = (500, 6.0000000004, 1);
    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    let (x, y, z) = tup;
    println!("{x}, {y}, {z}");
}

fn array_type_func()
{
    let arr1 = [1, 2, 3, 4];
    // 数组的类型以这种形式表示：[类型;长度]
    let arr2: [i32; 6] = [1, 2, 3, 4, 5, 6];
    // let arr3: [i32; 5] = [6; 5]; 相当于 let arr3: [i32; 5] = [6, 6, 6, 6, 6];
    let arr3: [i32; 5] = [6; 5];
    println!("{}, {}, {}, {}", arr1[0], arr2[3], arr3[1], arr2[1]);
}

fn use_if_statement_as_let_right(param_x: i32)
{
    let x = if param_x > 0 { "Greater" } else { "Less" };
    println!("The value of x and param_x comp is {}", x);
}

fn main()
{
    variable_func();
    guess_func();
    interger_type_func();
    float_type_func();
    number_operation();
    bool_type_func();
    char_type_func();
    tuple_type_func();
    array_type_func();
    use_if_statement_as_let_right(5);
}
