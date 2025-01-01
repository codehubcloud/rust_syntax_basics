/*
何时使用panic!

1.编写示例、原型代码、测试可以使用 panic!
    - 演示某些概念：unwrap
    - 原型代码: unwrap、 expect
    - 测试代码: unwrap、 expect

2.有时你比编译器掌握更多的信息
    你可以确定Result就是Ok: unwrap

3.错误处理的指导性建议
    当代码最终可能处于损坏状态时, 最好使用 panic!
    损坏状态(Bad state) :某些假设、保证、约定或不可变性被打破
    - 例如非法的值、矛盾的值或空缺的值被传入代码
    - 以及下列中的一条：
        这种损坏状态并不是预期能够偶尔发生的事情。
        在此之后, 您的代码如果处于这种损坏状态就无法运行。
        在您使用的类型中没有一个好的方法来将这些信息（处于损坏状态）进行编码。


4.场景建议
    调用你的代码, 传入无意义的参数值： panic!
    调用外部不可控代码, 返回非法状态, 你无法修复：panic!
    如果失败是可预期的：Result
    当你的代码对值进行操作,首先应该验证这些值: panic!

5. 为验证场景创建自定义类型
    (例子)
    创建新的类型,把验证逻辑放在构造实例的函数里。
    (例子)
    getter：返回字段数据
        - 字段是私有的（上例中）：外部无法直接对字段赋值
*/

use std::net::IpAddr;

/*
 2.有时你比编译器掌握更多的信息你可以确定Result就是Ok: unwrap
*/
fn get_local_ip_address() -> IpAddr
{
    let local_ip: IpAddr = "127.0.0.1".parse().unwrap();
    return local_ip;
}

/*
 为验证场景创建自定义类型
    创建新的类型,把验证逻辑放在构造实例的函数里。
*/

fn guess_number()
{
    let mut cnt = 0;
    loop {
        let guess = "32";
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("Guess value must be between 1 and 100, got {}", guess);
            continue;
        }
        cnt += 1;
        if cnt > 10 {
            break;
        }
    }
}
/*
getter：返回字段数据
- 字段是私有的（上例中）：外部无法直接对字段赋值
*/
pub struct Guess
{
    value: i32,
}

impl Guess
{
    pub fn new(value: i32) -> Guess
    {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        // 创建Guess实例
        Guess {
            value,
        }
    }

    pub fn value(&self) -> i32
    {
        self.value
    }
}

fn guess_number_optimized()
{
    let mut cnt = 0;
    loop {
        let guess = "32";
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = Guess::new(guess);
        println!("Guess value: {}", guess.value());
        cnt += 1;
        if cnt > 10 {
            break;
        }
    }
}

fn main()
{
    let local_ip = get_local_ip_address();
    println!("local ip address: {}", local_ip);
    guess_number();
    guess_number_optimized();
}

