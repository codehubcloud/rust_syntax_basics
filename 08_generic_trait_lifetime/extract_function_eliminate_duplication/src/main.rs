fn original_func()
{
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];
    for &number in number_list.iter() {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest = number_list[0];
    for &number in number_list.iter() {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);
}

fn get_largest(list: &[i32]) -> i32
{
    let mut largest = list[0];
    for &number in list {
        if number > largest {
            largest = number;
        }
    }
    return largest;
}

fn optimized_func()
{
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = get_largest(&number_list);
    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let largest = get_largest(&number_list);
    println!("The largest number is {}", largest);
}

fn main()
{
    original_func();
    optimized_func();
}
