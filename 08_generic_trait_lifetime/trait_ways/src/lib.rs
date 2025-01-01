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
