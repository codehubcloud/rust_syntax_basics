pub struct Guess
{
    value: i32,
}

impl Guess
{
    pub fn new(value: i32) -> Guess
    {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess {
            value,
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    #[should_panic] // 该注解表示测试通过的条件是函数 panic
    fn greater_than_100()
    {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100, got 0.")] // 该注解表示测试通过的条件是函数 panic 并抛出的错误信息与
                                                                                // expected 的值相同或包含, 如果不相同或不包含expected内容则测试失败
    fn less_than_1()
    {
        Guess::new(0);
    }
}
