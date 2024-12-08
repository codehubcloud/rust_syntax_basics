fn get_area_of_rectangle(length: u32, width: u32) -> u32
{
    return length * width;
}

fn get_area_of_rectangle_tuple(dim: (u32, u32)) -> u32
{
    return dim.0 * dim.1;
}

struct Rectangle
{
    length: u32,
    width: u32,
}

// 使用不可变的引用，实例不可变的借用
fn get_area_of_rectangle_struct(rect: &Rectangle) -> u32
{
    return rect.length * rect.width;
}

fn cal_area()
{
    let length = 20;
    let width = 50;
    let mut area = get_area_of_rectangle(length, width);
    println!("The area of rectangle length {} width {} is {}", length, width, area);

    let dim = (20, 50);
    area = get_area_of_rectangle_tuple(dim);
    println!("The area of rectangle length {} width {} is {}", dim.0, dim.1, area);

    let rect = Rectangle { length: 20, width: 50 };
    area = get_area_of_rectangle_struct(&rect);
    println!("The area of rectangle length {} width {} is {}", rect.length, rect.width, area);
}

fn main()
{
    cal_area();
}
