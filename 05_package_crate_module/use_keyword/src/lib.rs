/*
 * 这是一个lib crate, 最多只有一个lib crate
*/

fn welcome()
{
    println!("welcome to my restaurant")
}

pub mod front_of_house
{
    pub mod hosting
    {
        pub fn add_to_waitlist()
        {
            super::super::welcome(); // 使用上层模块的函数, 使用super关键字
            println!("add to waitlist");
        }
        pub fn seat_at_table()
        {
            crate::welcome(); // 使用crate根模块的函数, 使用crate关键字
            println!("seat at table");
        }
    }
    pub mod serving
    {
        pub fn take_order()
        {
            println!("take order");
        }
        pub fn serve_order()
        {
            println!("serve order");
        }
        pub fn take_payment()
        {
            println!("take payment");
        }
    }
}

pub mod back_of_house
{
    // 加了pub关键字, 公有结构体, 但是里面的字段不指定pubs的是私有字段
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

    
}

// use关键字, 使用绝对路径较好
use crate::front_of_house::hosting;
use crate::back_of_house::Breakfast;

pub fn eat_at_restaurant()
{
    hosting::add_to_waitlist();
}
