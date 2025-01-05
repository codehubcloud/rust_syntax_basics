#[cfg(test)]
mod tests
{
    #[test]
    fn it_works()-> Result<(), String>
    {
        if 2 + 2 == 4
        {
            Ok(())
        }
        else
        {
            Err("2 + 2 != 4".to_string())
        }
    }
}