use std::collections::HashMap;

fn hashmap_usage()
{
    let mut scores = HashMap::new();
    scores.insert(String::from("Sony"), 10);
    scores.insert(String::from("Tony"), 50);
    println!("{:?}", scores);
}

fn hashmap_update()
{
    let mut scores = HashMap::new();
    scores.insert(String::from("Tony"), 50);
    scores.insert(String::from("Tony"), 100);
    println!("{:?}", scores);
}

fn hashmap_entry()
{
    let mut scores = HashMap::new();
    scores.insert(String::from("Tony"), 50);
    let e = scores.entry(String::from("Hony"));
    println!("{:?}", e); // Entry(VacantEntry("Hony"))表示空, Hony不存在
    scores.entry(String::from("Tony")).or_insert(100); // 判断Tony是否存在, 不存在则插入, 否则不执行
    scores.entry(String::from("Sony")).or_insert(1000); // 判断Sony不存在, 执行插入
    println!("{:?}", scores);
}

fn update_value()
{
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}   

fn main()
{
    hashmap_usage();
    hashmap_update();
    hashmap_entry();
    update_value();
}
