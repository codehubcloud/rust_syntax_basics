enum SpreadsheetCell
{
    Int(i32),
    Float(f64),
    Text(String),
}

fn vector_enum_usage()
{
    // 这里是已经明确了存储类型的, 否则有无限种可能就没法存储
    let row = vec![SpreadsheetCell::Int(3), SpreadsheetCell::Text(String::from("blue")), SpreadsheetCell::Float(10.12)];
    for i in &row {
        match i {
            SpreadsheetCell::Int(i) => println!("{}", i),
            SpreadsheetCell::Float(f) => println!("{}", f),
            SpreadsheetCell::Text(s) => println!("{}", s),
        }
    }
}
fn main()
{
    vector_enum_usage();
}
