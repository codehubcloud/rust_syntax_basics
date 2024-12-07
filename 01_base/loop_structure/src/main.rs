fn loop_func()
{
    let mut cnt = 0;
    let result = loop {
        cnt += 1;

        if cnt == 10 {
            cnt = cnt * 2;
            break cnt; // break 需要带一个值，下面的写法更好
        }

        // if cnt == 10 {
        //     break cnt * 2;
        // }
    };
    println!("The result of loop is:{}", result);
}

fn while_func()
{
    let mut number = 10;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("Go....");
}

fn while_foreach_array_func()
{
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut index = 0;
    while index < arr.len() {
        println!("{}^v^", arr[index]);
        index += 1;
    }
}

fn for_func()
{
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    for element in arr.iter() {
        println!("{}^v^", element);
    }
}

fn for_range_func()
{
    let range_start = 1;
    let range_end = 10;
    for number in range_start..range_end {
        println!("{}", number);
    }

    for number in (range_start..range_end).rev() {
        println!("{}!", number);
    }
    println!("Go....");
}

fn main()
{
    loop_func();
    println!("---------------------");
    while_func();
    println!("---------------------");
    while_foreach_array_func();
    println!("---------------------");
    for_func();
    println!("---------------------");
    for_range_func();
}
