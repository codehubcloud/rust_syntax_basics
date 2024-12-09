fn option_usage()
{
    let some_number = Some(5);
    let some_string = Some("hello");
    let absent_number: Option<i32> = None;
    println!("{:?}", some_number);
    println!("{:?}", some_string);
    println!("{:?}", absent_number);
}

fn option_error_usage()
{
    let number_x = 5;
    let number_y = Some(10);
    // let result = number_x + number_y; // ^ no implementation for `{integer} + Option<{integer}>`
    let result = number_x + number_y.unwrap(); // √
    println!("{}", result);
}

fn main()
{
    option_usage();
    option_error_usage();
}
