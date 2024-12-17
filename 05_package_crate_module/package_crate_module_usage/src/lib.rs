/*
 * 这是一个lib crate, 最多只有一个lib crate
*/

mod front_of_house
{
    mod hosting
    {
        fn add_to_waitlist()
        {
        }
        fn seat_at_table()
        {
        }
    }
    mod serving
    {
        fn take_order()
        {
        }
        fn serve_order()
        {
        }
        fn take_payment()
        {
        }
    }
}
