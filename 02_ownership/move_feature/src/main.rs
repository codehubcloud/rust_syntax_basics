fn var_data_swap_way_move()
{
    let str1 = String::from("Move");
    let str2 = str1; // 这里就是进行了移动(move)

    // println!("str1: {}",str1); // ^^^^ value borrowed here after move, str1 String类型分配了heap资源, 无法使用简单的copy
    println!("str2: {}", str2);
}

fn shallow_copy()
{
    let x = 5;
    let y = x; // shallow copy, copy trait, 没有分配heap资源
    println!("x: {}, y: {}", x, y);
}

fn deep_copy_clone()
{
    let str1 = String::from("Move");
    let str2 = str1.clone(); // 这里用clone进行深拷贝

    println!("str1: {}", str1); // str1仍然可用
    println!("str2: {}", str2);
}

fn main()
{
    var_data_swap_way_move();
    shallow_copy();
    deep_copy_clone();
}
