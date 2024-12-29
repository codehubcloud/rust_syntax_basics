use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;

fn get_file_content() -> Result<String, io::Error>
{
    let f = File::open("hello0.txt");

    let mut f = match f {
        Ok(file) => file,                // File::open操作成功, 获取文件句柄
        Err(error) => return Err(error), // File::open操作失败, 返回错误信息
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        // read_to_string从文件中读取内容并将内容存入字符串s, 也可能失败
        // 读取文件内容
        Ok(_) => Ok(s),           // 读取文件内容成功, 返回文件内容, 封装在s中
        Err(error) => Err(error), // 读取文件内容失败, 返回Err, 包裹着错误信息
    }
}

fn get_file_content_use_question_mark() -> Result<String, io::Error>
{
    let mut f = File::open("hello0.txt")?; // ? 传播错误的一种快捷方式, 运算符会将 Result 的值作为整个函数的返回值 ？代表了下面注释代码
                                           // let mut f = match f {
                                           //     Ok(file) => file,
                                           //     Err(error) => return Err(error),
                                           // };
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

/*
 Trait std::convert::From上的from函数:
 - 用于错误之间的转换被
 ?所应用的错误,会隐式的被from函数处理
 当？调用 from 函数时：
 -它所接收的错误类型会被转化为当前函数返回类型所定义的错误类型
 用于：针对不同错误原因, 返回同一种错误类型
 -只要每个错误类型实现了转换为所返回的错误类型的from函数

 注：?运算符只能用于返回Result的函数
*/

fn get_file_content_use_question_mark_from() -> Result<String, io::Error>
{
    let mut s = String::new();
    File::open("hello0.txt")?.read_to_string(&mut s)?; // 链式操作, 当任何一个操作失败时, 整个表达式的结果就是 Err
    Ok(s)
}

fn question_mark_can_only_use_in_return_result()
{
    // let f = File::open("hello0.txt")?; // the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
}

/*

？运算符与 main 函数main 函数返回类型是：（）
-（例子）
main 函数的返回类型也可以是： Result<T, E>
-（例子）
Box<dyn Error> 是 trait 对象:
-简单理解: “任何可能的错误类型”
*/
fn question_mark_func() -> Result<(), Box<dyn Error>>
{
    let f = File::open("hello0.txt")?;
    Ok(()) // 返回单元类型, 相当于什么都没做
}

fn print_result(result: &Result<String, io::Error>)
{
    match result {
        Ok(s) => println!("{}", s),
        Err(e) => println!("Error: {}", e),
    }
}

fn print_dyn_result(result: &Result<(), Box<dyn Error>>)
{
    match result {
        Ok(_) => println!("{}", "Ok"),
        Err(e) => println!("Error: {}", e),
    }
}

fn main()
{
    let result0 = get_file_content();
    print_result(&result0);
    let result1 = get_file_content_use_question_mark();
    print_result(&result1);
    let result2 = get_file_content_use_question_mark_from();
    print_result(&result2);
    question_mark_can_only_use_in_return_result();
    let result3 = question_mark_func();
    print_dyn_result(&result3);
}
