/*
 * 这是一个lib crate, 最多只有一个lib crate
*/

pub mod front_of_house;
pub mod back_of_house;


// 使用 pub use 导出模块
pub use crate::front_of_house as fh;
pub use crate::back_of_house as bh;

pub fn eat_at_restaurant()
{
    fh::hosting::add_to_waitlist();
    fh::serving::take_order();

    let mut breakfast = bh::Breakfast::summer("Rye");
    breakfast.toast = String::from("Wheat");
    println!("I'd like {} toast please", breakfast.toast);
    breakfast.juice = String::from("coffee");
}