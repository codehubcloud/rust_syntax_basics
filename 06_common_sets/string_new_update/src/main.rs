fn string_new_usage()
{
    let mut s1 = String::new();
    s1.push('a');
    println!("s1 is {}", s1);
}

fn to_string_usage()
{
    let data_content = "init data content";
    let s1 = data_content.to_string();
    println!("s1 is {}", s1);

    let s2 = "init data content".to_string();
    println!("s2 is {}", s2);
}

fn string_from_usage()
{
    let s1 = String::from("init data content");
    println!("s1 is {}", s1);
    let s1 = String::from("Dobrý den");
    println!("s1 is {}", s1);
    let s1 = String::from("Hello");
    println!("s1 is {}", s1);
    let s1 = String::from("नमस्ते");
    println!("s1 is {}", s1);
    let s1 = String::from("こんにちは");
    println!("s1 is {}", s1);
    let s1 = String::from("안녕하세요");
    println!("s1 is {}", s1);
    let s1 = String::from("你好");
    println!("s1 is {}", s1);
    let s1 = String::from("Olá");
    println!("s1 is {}", s1);
    let s1 = String::from("Здравствуйте");
    println!("s1 is {}", s1);
    let s1 = String::from("Hola");
    println!("s1 is {}", s1);
}

fn string_update_usage()
{
    let mut s1 = String::from("init data content");

    // push_str()方法：把一个字符串切片附加到 String （例子）
    s1.push_str("append data content"); // push_str方法不会获取所有权, 而是借用

    //push()方法:把单个字符附加到String (例子)
    s1.push('!');
    println!("s1 is {}", s1);
}

fn plus_operator_usage()
{
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 这里使用了+运算符, 但是s1和s2都是String类型, 所以需要使用&符号来借用s2, 否则会报错, s3取得s1的所有权
    println!("s3 is {}", s3);
    // println!("s1 is {}", s1); // s1的所有权已经被移动, 不能再使用
    println!("s2 is {}", s2);
}

fn format_usage()
{
    let s1 = String::from("192");
    let s2 = String::from("168");
    let s3 = String::from("1");
    let s4 = String::from("1");
    let ip = format!("{}.{}.{}.{}", s1, s2, s3, s4); // 使用format!宏来格式化字符串, 不会获取所有权
    println!("ip is {}", ip);
}

fn main()
{
    string_new_usage();
    to_string_usage();
    string_from_usage();
    string_update_usage();
    plus_operator_usage();
    format_usage();
}
