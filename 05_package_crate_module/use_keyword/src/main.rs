// 使用use引入第三方模块, 类似C/C++的include
use std::collections::HashMap;
use std::fmt;
use std::io::Result as IoResult;

fn map_usage()
{
    let mut scores = HashMap::new();
    scores.insert(String::from("Sony"), 10);
    scores.insert(String::from("Tony"), 50);
}

fn io_result_usage() -> IoResult<()>
{
    return Ok(());
}

fn fmt_result_usage() -> fmt::Result
{
    return Ok(());
}

fn main()
{
    map_usage();
    // Call other functions to demonstrate usage
    if let Err(e) = io_result_usage() {
        eprintln!("IO Error: {}", e);
    }
    if let Err(e) = fmt_result_usage() {
        eprintln!("Formatting Error: {:?}", e);
    }
}
