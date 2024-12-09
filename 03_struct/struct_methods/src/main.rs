struct Rectangle
{
    length: u32,
    width: u32,
}

// 使用impl实现一个块, 每个结构体可以有多个impl块, 在块里定义方法, 方法的第一个函数参数是&self
impl Rectangle
{
    // 定义一个关联函数::
    fn build_rectangle(length: u32, width: u32) -> Rectangle
    {
        return Rectangle {
            length: length,
            width: width,
        };
    }
    fn get_area(&self) -> u32
    {
        return self.length * self.width;
    }
}

impl Rectangle
{
    fn can_hold(&self, other: &Rectangle) -> bool
    {
        return self.width > other.width && self.length > other.length;
    }

    // 定义一个关联函数::
    fn build_square(length: u32) -> Rectangle
    {
        return Rectangle {
            length: length,
            width: length,
        };
    }
}

fn cal_area()
{
    let rect1 = Rectangle::build_rectangle(20, 50);
    let rect2 = Rectangle::build_rectangle(10, 40);
    let rect3 = Rectangle::build_rectangle(30, 70);
    println!("rect1 can hold rect2 ? {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3 ? {}", rect1.can_hold(&rect3));
    println!("rect2 can hold rect3 ? {}", rect2.can_hold(&rect3));
    println!("rect3 can hold rect1 ? {}", rect3.can_hold(&rect1));
    let mut area = rect1.get_area();
    println!("The area of rectangle length {} width {} is {}", rect1.length, rect1.width, area);

    let square1 = Rectangle::build_square(30);
    area = square1.get_area();
    println!("The area of square length {} is {}", square1.length, area);
}

fn main()
{
    cal_area();
}
