/*
 忽略某些测试, 运行剩余测试
 ignore 属性（attribute）
 运行被忽略（ignore）的测试: 
 - cargo test -- --ignored
*/
#[cfg(test)]
mod tests
{
    #[test]
    fn it_works()
    {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore] // 该注解表示该测试被忽略
    fn expensive_test()
    {
        assert_eq!(5, 1 + 1 + 1 + 1 + 1);
    }
}
