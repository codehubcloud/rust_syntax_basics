pub fn greeting(name: &str) -> String
{
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn greeting_contains_name()
    {
        let result = greeting("Carol");
        // 第2个参数是错误信息, 如果断言失败, 将会打印这个错误信息, 第3个参数是断言失败时的值
        assert!(result.contains("Carol"), "Greeting did not contain name, value was `{}`", result);
    }
}
