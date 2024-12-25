fn panic_usage()
{
    panic!("crash and burn");
}

fn panic_overflow()
{
    let v = vec![1, 2, 3];
    v[99];
}

fn main()
{
    // panic_usage();
    panic_overflow();
}
