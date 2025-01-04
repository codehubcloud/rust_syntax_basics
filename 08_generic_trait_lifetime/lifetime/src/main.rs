use std::fmt::Display;
/*
 生命周期
 Rust 的每个引用都有自己的生命周期。
 生命周期:引用保持有效的作用域。
 大多数情况: 生命周期是隐式的、可被推断的
 当引用的生命周期可能以不同的方式互相关联时: 手动标注生命周期。
*/

/*
 生命周期-避免悬垂引用(dangling reference)
 生命周期的主要目标: 避免悬垂引用（dangling reference）
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

    let x = 5; // ----------+-- x
    let r = &x; // --+-- r   |
    println!("r: {}", r); //           |
}

/*
 函数中的泛型生命周期
*/
// 参数是字符串切片, 返回值是字符串切片
// fn get_longest1(x: &str, y: &str) -> &str // expected named lifetime paramete
// 需要显式的标注生命周期参数('字符)或者('字符串)
fn get_longest1<'life>(x: &'life str, y: &'life str) -> &'life str
{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn get_longest_test1()
{
    let string1 = String::from("long string is long");

    let string2 = String::from("xyz");
    let result = get_longest1(string1.as_str(), string2.as_str()); // as_str()将String转换为&str(字符串切片)

    println!("The longest string is '{}'", result);
}

/*
 生命周期标注语法
 生命周期的标注不会改变引用的生命周期长度
 当指定了泛型生命周期参数,函数可以接收带有任何生命周期的引用
 生命周期的标注: 描述了多个引用的生命周期间的关系, 但不影响生命周期

 生命周期标注- 语法
 生命周期参数名:
 - 以'开头
 - 通常全小写且非常短
 - 很多人使用'a, 推荐使用'life
 生命周期标注的位置:
 - 在引用的 & 符号后
 - 使用空格将标注和引用类型分开
 单个生命周期标注本身没有意义, 只有在多个引用之间才有意义

 函数签名中的生命周期标注
 泛型生命周期参数声明在:函数名和参数列表之间的<>里
 fn get_longest<'life>(x: &'life str, y: &'life str) -> &'life str // x和y的生命周期不小于'life
 生命周期'life的实际生命周期是 x 和 y 的生命周期z中较小的那个
*/

fn get_longest_test2()
{
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     // result= get_longest(string1.as_str(), string2.as_str()); // error[E0597]: `string2` does not live long enough
    // }
    // println!("The longest string is '{}'", result);
}

/*
 深入理解生命周期
*/

/*
 指定生命周期参数的方式依赖于函数所做的事情
 返回类型只有一个引用, 所以生命周期参数只需要一个, 这里只与x的生命周期相关, 与y的生命周期无关, y不需要生命周期参数
*/
fn get_longest2<'life>(x: &'life str, y: &str) -> &'life str
{
    x
}

/*
 从函数返回引用时, 返回类型的生命周期参数需要与其中一个参数的生命周期匹配:
 如果返回的引用没有指向任何参数, 那么它只能引用函数内创建的值:
 -这就是悬垂引用:该值在函数结束时就走出了作用域
*/
fn get_longest3<'life>(x: &'life str, y: &str) -> String // &'life str
{
    let result = String::from("really long string");
    // result.as_str() // cannot return value referencing local variable `result`
    result
}

/*
 结构体Struct 中的生命周期标注
 Struct 里可包括:
 -自持有的类型
 -引用: 需要在每个引用上添加生命周期标注
*/
struct ImportantExcerpt<'life>
{
    part: &'life str,
}

fn struct_lifetime()
{
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

/*
 生命周期的省略
 我们知道:
 - 每个引用都有生命周期
 - 需要为使用生命周期的函数或struct指定生命周期参数
 生命周期省略规则
 在Rust引用分析中所编入的模式称为生命周期省略规则。
 - 这些规则无需开发者来遵守
 - 它们是一些特殊情况, 由编译器来考虑
 - 如果你的代码符合这些情况,那么就无需显式标注生命周期
 生命周期省略规则不会提供完整的推断:
 - 如果应用规则后, 引用的生命周期仍然模糊不清→编译错误
 - 解决办法: 添加生命周期标注,表明引用间的相互关系
*/
fn first_word0(s: &str) -> &str // 函数签名中的生命周期参数没有显示标注, 使用的是默认的生命周期省略规则
{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

/*
 输入、输出生命周期
 生命周期在:
 - 函数/方法的参数:输入生命周期
 - 函数/方法的返回值:输出生命周期
 生命周期省略的三个规则
 编译器使用3个规则在没有显式标注生命周期的情况下,来确定引用的生命周期
 - 规则1应用于输入生命周期
 - 规则2、3应用于输出生命周期
 - 如果编译器应用完3个规则之后,仍然有无法确定生命周期的引用→报错
 - 这些规则适用于fn定义和impl块
 规则1: 每个引用类型的参数都有自己的生命周期
 规则2: 如果只有1个输入生命周期参数, 那么该生命周期被赋给所有的输出生命周期参数
 规则3: 如果有多个输入生命周期参数, 但其中一个是&self或&mut self (是方法), 那么self 的生命周期会被赋给所有的输出生命周期参数
*/

/*
生命周期省略的三个规则*/
//  -例子假设我们是编译器
fn first_word1(s: &str) -> &str
{
    s
}

// 1. 规则1: 每个引用类型的参数都有自己的生命周期 , 规则2不适用
fn first_word2<'a>(s: &'a str) -> &str
{
    s
}

// 2. 规则2: 如果只有1个输入生命周期参数, 那么该生命周期被赋给所有的输出生命周期参数
fn first_word3<'a>(s: &'a str) -> &'a str
{
    s
}

// 例子: 有多个输入生命周期参数
// fn longest1(x: &str, y: &str) -> &str // ^ expected named lifetime parameter
// {
//     x
// }

// longest2是函数不是方法, 所以没有self参数, 规则1, 规则1, 规则3不适用
// fn longest2<'a, 'b>(x: &'a str, y: &'b str) -> &str // expected named lifetime parameter
// {
//     x
// }

/*
 方法定义中的生命周期标注
 在struct上使用生命周期实现方法,语法和泛型参数的语法一样
 在哪声明和使用生命周期参数, 依赖于:
 - 生命周期参数是否和字段、方法的参数或返回值有关
 struct 字段的生命周期名:
 - 在 impl 后声明
 - 在 struct 名后使用
 - 这些生命周期是struct类型的一部分
 impl 块内的方法签名中:
 - 引用必须绑定于struct字段引用的生命周期,或者引用是独立的也可以
 - 生命周期省略规则经常使得方法中的生命周期标注不是必须的
*/

/*
方法定义中的生命周期标注*/
struct ImportantExcerpt1<'life>
{
    part: &'life str,
}

impl<'life> ImportantExcerpt1<'life>
{
    fn level(&self) -> i32
    {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str
    {
        println!("Attention please: {}", announcement);
        self.part
    }
}

/*
 静态生命周期
 'static 是一个特殊的生命周期: 整个程序的持续时间。
 -例如: 所有的字符串字面值都拥有'static生命周期
 let s: &'static str = "I have a static lifetime.";
 为引用指定'static 生命周期前要三思:
 -是否需要引用在程序整个生命周期内都存活。
*/

/*
 泛型参数类型、Trait Bound、生命周期
*/
fn logest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main()
{
    dangling_reference1();
    dangling_reference2();
    get_longest_test1();
    get_longest_test2();
    struct_lifetime();
}
