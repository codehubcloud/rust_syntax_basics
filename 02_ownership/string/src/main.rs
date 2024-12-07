fn string_type()
{
    let mut str = String::from("Hello"); // String是在heap中申请的内存，是可变的
    str.push_str(", world");
    println!("String is {}", str);
} // 系统调用str的drop函数释放了str

fn main()
{
    string_type();
}