fn function_with_two_param(param_x: i32, param_y: i32)
{
    println!("x: {}, y: {}", param_x, param_y);
}

fn function_statement()
{
    let y = {
        // 块表达式
        let x = 1;
        x + 3 // 块的返回值
    };

    println!("The value of yis : {}", y);
}

fn function_default_return_value() -> i32
{
    let x = 5;
    let y = x;
    y + 20 // 最后一个表达式没有带分号就是函数的返回值
}

fn function_return_value(param_x: i32) -> i32
{
    if param_x < 0 {
        return param_x + 20; // 使用return带分号作为函数的返回值
    }

    return param_x + 10; // 使用return带分号作为函数的返回值
}

fn main()
{
    function_with_two_param(128, 256);
    function_statement();
    println!("function_default_return_value: {}", function_default_return_value());
    println!("function_return_value: {}", function_return_value(12));
    println!("function_return_value: {}", function_return_value(-12));
}
