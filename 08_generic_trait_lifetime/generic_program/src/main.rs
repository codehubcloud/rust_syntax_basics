// 泛型结构体
struct Point2D<T>
{
    x: T,
    y: T,
}

impl<T> Point2D<T>
{
    fn get_x(&self) -> &T
    {
        &self.x
    }

    fn get_y(&self) -> &T
    {
        &self.y
    }
}

// 泛型结构体
struct Point3D<T, U>
{
    x: T,
    y: T,
    z: U,
}

impl<T, U> Point3D<T, U>
{
    fn get_x(&self) -> &T
    {
        &self.x
    }

    fn get_y(&self) -> &T
    {
        &self.y
    }

    fn get_z(&self) -> &U
    {
        &self.z
    }

    // 泛型方法, 返回一个新的Point3D实例, 其中x和y来自self, z来自other
    fn mixup<V, W>(self, other: Point3D<V, W>) -> Point3D<T, W>
    {
        Point3D {
            x: self.x,
            y: self.y,
            z: other.z,
        }
    }
}

// 泛型枚举
enum Option<T>
{
    Some(T),
    None,
}

// 泛型枚举
enum Result<T, E>
{
    Ok(T),
    Err(E),
}

// Generic function 泛型函数
// fn get_largest<T>(list: &[T]) -> T
// {
    // let mut largest = list[0];
    // for &number in list {
    //     // binary operation `>` cannot be applied to type `T`
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    // return largest;
// }

// fn optimized_func()
// {
//     let number_list = vec![34, 50, 25, 100, 65];
//     let largest = get_largest(&number_list);
//     println!("The largest number is {}", largest);

//     let char_list = vec!['a', 'q', 'c', 'z', 'm'];
//     let largest = get_largest(&char_list);
//     println!("The largest number is {}", largest);
// }

fn use_generic()
{
    let p1 = Point2D {
        x: 1,
        y: 2,
    };
    println!("p1.x = {}, p1.y = {}", p1.get_x(), p1.get_y());
    let p2 = Point3D {
        x: 3,
        y: 4,
        z: 5,
    };
    let p3 = Point3D {
        x: 3,
        y: 4,
        z: 'h',
    };
    let p4 = p2.mixup(p3);
    println!("p4.x = {}, p4.y = {}, p4.z = {}", p4.get_x(), p4.get_y(), p4.get_z());
}

fn main()
{
    // optimized_func();
    use_generic();
}
