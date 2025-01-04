/*
 生命周期
 Rust 的每个引用都有自己的生命周期。
 生命周期:引用保持有效的作用域。
 大多数情况：生命周期是隐式的、可被推断的
 当引用的生命周期可能以不同的方式互相关联时：手动标注生命周期。
*/

/*
 生命周期-避免悬垂引用(dangling reference)
 生命周期的主要目标：避免悬垂引用（dangling reference）
*/

fn dangling_reference1()
{
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}", r); // error: `x` borrowed value does not live long enough
}

/*
 借用检查器
 Rust编译器的借用检查器:比较作用域来判断所有的借用是否合法。
*/
fn dangling_reference2()
{
    // let r;                    // ---------+-- r
    // {                         //          |
    //     let x = 5;            // -+--x    |
    //     r = &x;               //  |       |
    // }                         // -+ x     |
    // println!("r: {}", r);     //          |- r

    let x = 5;            // ----------+-- x
    let r = &x;           // --+-- r   |
    println!("r: {}", r); //           |
}

/*
 函数中的泛型生命周期
*/
// 参数是字符串切片，返回值是字符串切片
// fn get_longest(x: &str, y: &str) -> &str // expected named lifetime paramete
// 需要显式的标注生命周期参数('字符)或者('字符串)
fn get_longest<'life>(x: &'life str, y: &'life str) -> &'life str
{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn get_longest_test()
{
    let string1 = String::from("long string is long");

    let string2 = String::from("xyz");
    let result = get_longest(string1.as_str(), string2.as_str()); // as_str()将String转换为&str(字符串切片)

    println!("The longest string is '{}'", result);
}

fn main()
{
    dangling_reference1();
    dangling_reference2();
    get_longest_test();
}
