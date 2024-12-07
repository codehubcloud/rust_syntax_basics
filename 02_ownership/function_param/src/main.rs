fn take_owership(in_string: String)
{
    println!("{}", in_string);
} // 这里系统自动drop了

fn makes_copy(in_number: i32)
{
    println!("{}", in_number);
} // 这里系统自动drop了

fn function_param_ownership_move_or_copy()
{
    let str = String::from("Hello World");
    take_owership(str); // 这里str已经move了，在这之后，str失效
                        // println!("{}", str); // ^^^ value borrowed here after move
    let x = 5;
    makes_copy(x); // 这里传入的是x的copy trait副本，下面的x仍然有效

    println!("{}", x);
}

fn gives_ownership() -> String
{
    let some_strig = String::from("Hello Ownership");
    return some_strig;
}

fn takes_and_gives_back(in_string: String) -> String
{
    return in_string;
}

fn function_param_ownership_return_value()
{
    let str1 = gives_ownership(); // 返回值所有权移动
    println!("{}", str1);
    let str2 = String::from("Ownership");
    let str3 = takes_and_gives_back(str2); // 先把str2的所有权移动给takes_and_gives_back函数内部，然后内部通过返回值又移动给str3
    println!("{}", str3);
}

fn get_string_length(str: String) -> (String, usize)
{
    let length = str.len();
    return (str, length);
}

fn function_use_value_not_get_ownership()
{
    let str1 = String::from("use_value_not_get_ownership");
    let (str2, len) = get_string_length(str1); // 先把str1的所有权移动给get_string_length函数内部，然后内部通过返回值又移动给str2
    println!("{} {}", str2, len);
}

/*
 使用&符号使用引用方式传参，避免所有权移动, 注意: &符号是在类型前，不是在参数前
 借用(指的是行为): 指把引用作为函数参数的行为叫借用
*/
fn get_string_length_by_reference(str: &String) -> usize
{
    let length = str.len();
    return length;
}

fn function_param_reference()
{
    let str1 = String::from("param_reference");
    let len = get_string_length_by_reference(&str1); // str1引用未发生所有权移动
    println!("{} {}", str1, len);
}

// 可以改变值的引用 &mut
fn get_string_length_by_can_change_reference(str: &mut String) -> usize
{
    str.push_str(" is can change");
    let length = str.len();
    return length;
}

fn function_param_can_change_reference()
{
    let mut str1 = String::from("can_change_reference");
    println!("{}", str1);
    let len = get_string_length_by_can_change_reference(&mut str1); // str1引用未发生所有权移动
    println!("{} {}", str1, len);
}

/*
 在相同作用域，不能存在多个可变引用
*/
fn can_change_reference_can_only_use_once_in_same_scope()
{
    let mut str1 = String::from("can_only_use_once_in_same_scope");
    let str2 = &mut str1;
    // let str3 = &mut str1; // cannot borrow `str1` as mutable more than once at a time
    println!("{}", str2);
}

fn can_change_reference_can_use_in_difference_scope()
{
    // scope 1 strart
    let mut str1 = String::from("can_use_in_difference_scope");
    {
        // scope2 start
        let str2 = &mut str1;
        println!("{}", str2);
    } // scope2 end str2 is end

    let str3 = &mut str1;
    println!("{}", str3);
} // scope 1 end

/*
 在相同作用域，可以同时存在多个不可变引用
 不能同时存在可变引用和不可变引用
*/
fn can_not_use_can_change_and_cannot_change_reference_as_same_time()
{
    // let mut str1 = String::from("can_use_in_difference_scope");
    let str1 = String::from("can_use_in_difference_scope");
    let str2 = &str1; // 不可变引用
    let str3 = &str1; // 不可变引用
                      // let str4 = &mut str1; // 可变引用 ^^^^^^^^^ mutable borrow occurs here
    println!("{} {} {}", str1, str2, str3);
}

// fn dangle() -> &String // ^ expected named lifetime parameter
// {
//     let str = String::from("dangle");
//     return &str; // cannot return reference to local variable `str`
// }

// // 悬空引用
// fn dangling_references()
// {
//     let str = dangle();
//     println!("{}", str);
// }

/*
 不持有所有权的数据类型 切片(slice)
*/
// 获取字符串第一个单词结束位置
fn get_first_word_pos_in_string(str: &String) -> usize
{
    let bytes = str.as_bytes(); // 获取字符串的字节数组

    // 遍历bytes数组
    for (pos, &item) in bytes.iter().enumerate() {
        // enumerate返回的是元组
        if item == b' ' {
            return pos;
        }
    }
    return str.len();
}

/*
 字符串切片: 指向字符串中一部分内容的"引用"
 形式: &字符串[索引开始..索引结束]
 索引开始: 是切片的起始位置
 索引结束: 是切片结束的下一个位置, 即不包含此位置
*/

fn slice_for_string()
{
    let str = String::from("Hello Slice");
    let word1 = &str[0..5]; // 不包含5
    let word2 = &str[6..11]; // 不包含11
    let word3 = &str[..5]; // 不包含5, 0的起始位置可不写
    let word4 = &str[6..]; // 不包含11, 最后的位置可不写
    let whole_string1 = &str[0..str.len()]; // 整个字符串切片
    let whole_string2 = &str[..]; // 整个字符串切片, 起始结束都不写

    println!("{}, {}, {}, {}, {}, {}", word1, word2, word3, word4, whole_string1, whole_string2);
}

// &str 返回值表示字符串切片
// fn get_first_word_in_string_by_slice(str: &String) -> &str
// 参数可直接使用字符串切片&str, 会使API更加通用，功能不会有任何损失
fn get_first_word_in_string_by_slice(str: &str) -> &str
{
    let bytes = str.as_bytes(); // 获取字符串的字节数组

    // 遍历bytes数组
    for (pos, &item) in bytes.iter().enumerate() {
        // enumerate返回的是元组
        if item == b' ' {
            return &str[0..pos];
        }
    }
    return &str[0..str.len()];
}

fn get_first_word_in_string()
{
    // let mut str = String::from("get first word in string");
    let str = String::from("get first word in string");
    let pos = get_first_word_pos_in_string(&str);
    println!("The string \"{}\" first word pos is {}", str, pos);
    let first_word = get_first_word_in_string_by_slice(&str);
    // str.clear(); // cannot borrow `str` as mutable because it is also borrowed as immutable
    println!("The string \"{}\" first word is {}", str, first_word);
}

// 其他类型的切片
fn other_slice()
{
    let arr = [1, 2, 3, 4, 5, 6];
    let slice = &arr[1..4];
    println!("Pointer: {:p}, Length: {}", slice.as_ptr(), slice.len());
    // {:?}单行输出  {:#?}多行输出
    println!("integer slice: {:?}", slice);
}

fn main()
{
    function_param_ownership_move_or_copy();
    function_param_ownership_return_value();
    function_use_value_not_get_ownership();
    function_param_reference();
    function_param_can_change_reference();
    can_change_reference_can_only_use_once_in_same_scope();
    can_change_reference_can_use_in_difference_scope();
    can_not_use_can_change_and_cannot_change_reference_as_same_time();
    slice_for_string();
    get_first_word_in_string();
    other_slice();
}
