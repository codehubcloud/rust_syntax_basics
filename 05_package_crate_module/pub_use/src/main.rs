use pub_use::fh;
use pub_use::bh;


fn eat_at_restaurant()
{
    fh::hosting::add_to_waitlist();
    fh::serving::take_order();

    let mut breakfast = bh::Breakfast::summer("Rye");
    breakfast.toast = String::from("Wheat");
    println!("I'd like {} toast please", breakfast.toast);
    breakfast.juice = String::from("coffee");
}
fn main()
{
    eat_at_restaurant();
}
