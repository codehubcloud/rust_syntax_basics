use std::fs::File;
use std::io::ErrorKind;

fn file_open()
{
    let f = File::open("hello0.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error);
        },
    };
}

fn file_open_and_create()
{
    let f = File::open("hello1.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // error.kind() 也是枚举类型, 是IO操错误类型
            ErrorKind::NotFound => match File::create("hello1.txt") {
                // 如果文件不存在, 创建文件
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Problem creating the file: {:?}", e);
                },
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            },
        },
    };
}

fn file_open_and_create_use_other_way()
{
    let f = File::open("hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello2.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn file_open_and_create_use_unwrap()
{
    // let f = File::open("hello3.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error);
    //     },
    // };
    let f = File::open("hello3.txt").unwrap(); // unwrap()错误信息不可自定义, Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "系统找不到指定的文件。" }
}

fn file_open_and_create_use_expect()
{
    let f = File::open("hello4.txt").expect("Failed to open hello4.txt"); // expect()错误信息可自定义 Failed to open hello4.txt:
}

fn main()
{
    // file_open();
    file_open_and_create();
    file_open_and_create_use_other_way();
    // file_open_and_create_use_unwrap();
    file_open_and_create_use_expect();
}
