use std::fmt::Debug;
use std::fmt::Display;
// 定义一个 trait, 该 trait 包含一个方法 summarize, 该方法返回一个 String 类型的值, 没有具体的实现, 只有方法签名, 以; 结尾
pub trait Summary
{
    fn summarize(&self) -> String; // 方法签名, 以; 结尾

    // 也可以有个默认实现
    // fn summarize(&self) -> String
    // {
    //     String::from("(Read more...)")
    // }
}

// 定义一个结构体, 包四个字段都是 String 类型
pub struct NewsArticle
{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 实现 Summary trait for NewsArticle
impl Summary for NewsArticle
{
    // 对NewsArticle具体实现 summarize 方法
    fn summarize(&self) -> String
    {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// 定义一个结构体, 包两个字段都是 String 类型, 两个字段是 bool 类型
pub struct Tweet
{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 实现 Summary trait for Tweet
impl Summary for Tweet
{
    // 对Tweet具体实现 summarize 方法
    fn summarize(&self) -> String
    {
        format!("{}: {}", self.username, self.content)
    }
}

/* impl trait语法: 适用于简单情况
   trait 作为参数, 实现调用 trait 的summarize方法
*/
pub fn impl_trait_as_parameter_for_summary1(item1: impl Summary)
{
    println!("Breaking news! {}", item1.summarize());
}

/*Trait Bound 语法: 适用于复杂情况
  trait 作为参数, 实现调用 trait 的summarize方法
*/
// pub fn impl_trait_as_parameter_for_summary1(item1 : &impl Summary)
// {
//     println!("Breaking news! {}", item1.summarize());
// }
pub fn trait_bound_as_parameter_for_summary1<T: Summary>(item1: T)
{
    println!("Breaking news! {}", item1.summarize());
}

// 传递多个参数, 使用 impl trait 语法
pub fn impl_trait_as_parameter_for_summary2(item1: impl Summary, item2: impl Summary)
{
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}

// 使用 trait bound 语法简洁不少
pub fn trait_bound_as_parameter_for_summary2<T: Summary>(item1: T, item2: T)
{
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}

// 使用 + 语法, 可以传递多个 trait bound
pub fn impl_trait_as_parameter_for_summary3(item1: impl Summary + Display)
{
    println!("Breaking news! {}", item1.summarize());
}

pub fn plus_trait_bound_as_parameter_for_summary3<T: Summary + Display>(item1: T)
{
    println!("Breaking news! {}", item1.summarize());
}

// 使用 where 语法, 可以传递多个 trait bound, 方法签名没有那么乱
pub fn trait_bound_as_parameter_for_summary4<T: Summary + Display, U: Clone + Debug>(item1: T, item2: U) -> String
{
    format!("Breaking news! {}", item1.summarize())
}

pub fn where_trait_bound_as_parameter_for_summary4<T, U>(item1: T, item2: U) -> String
where
    T: Summary + Display, // T 必须实现 Summary 和 Display trait
    U: Clone + Debug,     // U 必须实现 Clone 和 Debug trait
{
    format!("Breaking news! {}", item1.summarize())
}

/*
 实现trait 作为返回值类型
 注意: impl Trait 只能返回确定的同一种类型, 返回可能不同类型的代码会报错
*/
pub fn impl_trait_as_return_value(s: &str, flag: bool) -> impl Summary
{
    // if flag {
    NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(s),
    }
    // } else { // xpected `NewsArticle`, found `Tweet`
    //     Tweet {
    //         username: String::from("horse_ebooks"),
    //         content: String::from(s),
    //         reply: false,
    //         retweet: false,
    //     }
    // }
}

// Trait bound 作为返回值类型
pub fn get_largest<T: PartialOrd + Clone>(list: &[T]) -> &T
{
    let mut largest = &list[0];
    for item in list.iter() {
        // >的方法:  std::cmp::PartialOrd
        if item > &largest {
            largest = item;
        }
    }
    largest
}

/*
 在使用泛型类型参数的 impl 块上使用Trait bound,我们可以有条件的为实现了特定Trait的类型来实现方法
*/
struct Pair<T>
{
    x: T,
    y: T,
}

impl<T> Pair<T>
{
    // 无论T是什么类型,  都实现了new方法
    fn new(x: T, y: T) -> Self
    {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T>
{
    // 只有实现了Display和PartialOrd的类型才能调用cmp_display方法
    fn cmp_display(&self)
    {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

/*
 也可以为实现了其它Trait的任意类型有条件的实现某个Trait
 为满足Trait Bound的所有类型上实现Trait叫做覆盖实现(blanketdimplementations)
*/
// 在标准库中string.rs中, 实现了Display的类型都可以调用to_string方法
/*
#[stable(feature = "rust1", since = "1.0.0")]
impl<T: fmt::Display> ToString for T
{
    #[inline]
    default fn to_string(&self) -> String
    {
        use fmt::Write;
        let mut buf = String::new();
        buf.write_fmt(format_args!("{}", self)).expect("a Display implementation returned an error unexpectedly");
        buf.shrink_to_fit();
        buf
    }
}
*/
