// 没有成员名字的struct, 又像tuple

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

fn tuple_struct()
{
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black: { } {} {} ", black.0, black.1, black.2);
    println!("black: {} {} {}", origin.0, origin.1, origin.2);
}

fn main()
{
    tuple_struct();
}
