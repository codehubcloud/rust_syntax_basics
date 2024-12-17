/*
 * 这是一个lib crate, 最多只有一个lib crate
*/

mod front_of_house
{
    pub mod hosting
    {
        pub fn add_to_waitlist()
        {
            println!("add to waitlist");
        }
        pub fn seat_at_table()
        {
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

pub fn eat_at_restaurant()
{
    // 绝对路径调用, 绝对路径是指从 crate 根开始的路径, (推荐使用绝对路径)
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径调用, 相对路径是指从当前模块开始的路径, (不推荐使用相对路径)
    front_of_house::hosting::seat_at_table();
}
