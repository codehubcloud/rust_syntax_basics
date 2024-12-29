
fn welcome()
{
    println!("welcome to my restaurant")
}

pub fn add_to_waitlist()
{
    welcome(); // 使用上层模块的函数, 使用super关键字
    println!("add to waitlist");
}
pub fn seat_at_table()
{
    welcome(); // 使用crate根模块的函数, 使用crate关键字
    println!("seat at table");
}
