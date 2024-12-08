struct User
{
    username: String,
    email: String,
    sign_in_cnt: u64,
    is_active: bool,
}

// 不可变实例
fn struct_instantiation()
{
    let user1 = User { username: String::from("ZhangSan"), email: String::from("123456@gmail.cm"), sign_in_cnt: 10, is_active: true };
    println!("{} {} {} {}", user1.username, user1.email, user1.sign_in_cnt, user1.is_active);
}

// 可变实例，可变实例所有字段都可变，不允许单独设置成员可变或不可变
fn struct_mut_instantiation()
{
    let mut user1 = User { username: String::from("ZhangSan"), email: String::from("123456@gmail.cm"), sign_in_cnt: 10, is_active: true };
    println!("{} {} {} {}", user1.username, user1.email, user1.sign_in_cnt, user1.is_active);
    user1.email = String::from("789@gmail.com");
    println!("{} {} {} {}", user1.username, user1.email, user1.sign_in_cnt, user1.is_active);
}

// 结构体作为函数的返回值
fn build_user(username: String, email: String) -> User
{
    // let user = User { username, email, sign_in_cnt: 1, is_active: true }; // 可以简写 username, email 和入参同名，但是不推荐，可读性差
    let user = User { username: username, email: email, sign_in_cnt: 1, is_active: true };
    return user;
}

fn use_user()
{
    let user = build_user(String::from("WanWu"), String::from("1024.gmail.com"));
    println!("{} {} {} {}", user.username, user.email, user.sign_in_cnt, user.is_active);
}

fn struct_update()
{
    let user1 = build_user(String::from("WanWu"), String::from("1024.gmail.com"));
    println!("{} {} {} {}", user1.username, user1.email, user1.sign_in_cnt, user1.is_active);

    let user2 = User {
        username: String::from("LiSi"),
        email: String::from("2048@gmail.com"),
        ..user1 // 这里就是更新语法，其他字段和user1一致
    };
    println!("{} {} {} {}", user2.username, user2.email, user2.sign_in_cnt, user2.is_active);
}

fn main()
{
    struct_instantiation();
    struct_mut_instantiation();
    use_user();
    struct_update();
}
