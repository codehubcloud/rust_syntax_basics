/*
 * if let usage
 * if let 语法可以用于只匹配其中一种情况的场景, 而不是所有情况
 */

fn if_let_usage()
{
    let some_u8_value = Some(0u8);

    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("not three");
    }
}

fn main()
{
    if_let_usage();
}
