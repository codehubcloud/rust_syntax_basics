/*
 按名称运行测试的子集
 选择运行的测试: 将测试的名称（一个或多个）作为 cargo test 的参数
 运行单个测试: 指定测试名
 运行多个测试: 指定测试名的一部分（模块名也可以）
 cargo test add_three_and_two
 cargo test add_mod
 cargo test other_mod
*/

pub fn add_two(a: i32) -> i32
{
    a + 2
}

#[cfg(test)]
mod add_mod
{
    use super::*;

    #[test]
    fn add_two_and_two()
    {
        let result = add_two(2);
        assert_eq!(4, result);
    }

    #[test]
    fn add_three_and_two()
    {
        let result = add_two(3);
        assert_eq!(5, result);
    }

    #[test]
    fn on_hundred()
    {
        let result = add_two(100);
        assert_eq!(102, result);
    }
}

#[cfg(test)]
mod other_mod
{
    use super::*; // Import all items from the parent module

    #[test]
    fn one_hundred()
    {
        let result = add_two(100);
        assert_eq!(102, result);
    }

    #[test]
    fn two_hundred()
    {
        let result = add_two(200);
        assert_eq!(202, result);
    }
}
