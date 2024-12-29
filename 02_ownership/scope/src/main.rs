fn scope()
{
    // str 不可用, 生命未开始
    let str = "scope"; // str 生命开始, 可用

    /* 此处可对 scope 进行操作 */
    println!("String is {}", str);
} // 作用域结束,  str 生命结束, 不可用


fn string_type()
{
    let mut str = String::from("Hello"); // String是在heap中申请的内存, 是可变的
    str.push_str(", world");
    println!("String is {}", str);
} // 系统调用str的drop函数释放了str

fn main()
{
    scope();
    string_type();
}
