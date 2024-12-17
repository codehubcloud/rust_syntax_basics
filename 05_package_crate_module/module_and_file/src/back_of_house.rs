// 加了pub关键字, 公有结构体，但是里面的字段不指定pubs的是私有字段
pub struct Breakfast
{
    pub toast: String,    // 加pub关键字, 公有字段
    pub juice: String,    // 加pub关键字, 公有字段
    season_fruit: String, // 私有字段
}

// 定义关联函数
impl Breakfast
{
    pub fn summer(toast: &str) -> Breakfast
    {
        Breakfast {
            toast: String::from(toast),
            juice: String::from("orange juice"),
            season_fruit: String::from("peaches"),
        }
    }
}

// 加了pub关键字, 公有枚举, 特别注意: 里面的所有字段都是公共的
pub enum Appetizer {
    Soup,
    Salad,
}
