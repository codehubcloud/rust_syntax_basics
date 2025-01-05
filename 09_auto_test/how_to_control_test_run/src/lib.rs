/*
 控制测试如何运行
 改变cargo test的行为:添加命令行参数
 默认行为: 
 - 并行运行
 - 所有测试
 - 捕获(不显示)所有输出,使读取与测试结果相关的输出更容易。
 命令行参数: 
 -针对cargo test的参数:紧跟cargo test后
 -针对测试可执行程序:放在 -- 之后
   cargo test --help
   cargo test -- --help
*/

/*
 并行运行测试
 运行多个测试: 默认使用多个线程并行运行。
 - 运行快
 确保测试之间: 
 - 不会互相依赖
 - 不依赖于某个共享状态(环境、工作目录、环境变量等等)
*/

/*
 --test-threads 参数
 传递给 二进制文件
 不想以并行方式运行测试, 或想对线程数进行细粒度控制
 可以使用--test-threads参数, 后边跟着线程的数量
 例如: cargo test -- --test-threads=1
*/

/*
 显式函数输出
 默认, 如测试通过, Rust的test库会捕获所有打印到标准输出的内容。
 例如, 如果被测试代码中用到了println!:
 - 如果测试通过: 不会在终端看到 println!打印的内容
 - 如果测试失败: 会看到println!打印的内容和失败信息
 如果想在成功的测试中看到打印的内容: --show-output
 例如: cargo test -- --show-output
 */

pub fn prints_and_returns_10(a: i32) -> i32
{
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn this_test_will_pass()
    {
        let result = prints_and_returns_10(8);
        assert_eq!(10, result);
    }

    #[test]
    fn this_test_will_fail()
    {
        let result = prints_and_returns_10(4);
        assert_eq!(5, result);
    }
}