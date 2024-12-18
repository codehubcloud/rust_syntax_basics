fn vector_usage()
{
    let mut vec1: Vec<i32> = Vec::new();
    vec1.push(5);
    let num: &i32 = &vec1[0];
    println!("num is {}", num);

    let vec2 = vec![1, 2, 3]; // 使用vec!宏创建一个新的vector并初始化

    // 索引合法, 安全访问
    match vec2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // 索引越界, 非法访问
    match vec2.get(100) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}
// vector的可变引用和不可变引用
fn vector_mut_and_immut_reference()
{
    let mut vec1 = vec![1, 2, 3, 4, 5];
    vec1.push(6); // 可以push
    let first = &vec1[0]; // 不可变引用
                          // vec1.push(6); // 可变引用 cannot borrow `vec1` as mutable because it is also borrowed as immutable
    println!("The first element is: {}", first); // 不可变引用
}

fn vector_foreach()
{
    let vec1 = vec![1, 2, 3];
    // 使用不可变引用遍历
    for i in &vec1 {
        println!("{}", i);
    }

    let mut vec2 = vec![4, 5, 6];
    // 使用可变引用遍历
    for i in &mut vec2 {
        *i += 50; // 可变引用需要解引用, 修改值
    }
    for i in &vec2 {
        println!("{}", i);
    }
}

fn main()
{
    vector_usage();
    vector_mut_and_immut_reference();
    vector_foreach();
}
