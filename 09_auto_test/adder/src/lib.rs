pub fn add(left: u64, right: u64) -> u64
{
    left + right
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn it_works()
    {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another_test()
    {
        // let result = add(3, 3);
        // assert_eq!(result, 6);
        panic!("Make this test fail");
    }
}
