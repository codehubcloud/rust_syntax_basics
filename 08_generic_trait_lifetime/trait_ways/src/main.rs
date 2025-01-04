// trait_ways 是Cargo.toml中的包名, 也是lib.rs中的模块名
use trait_ways::NewsArticle;
use trait_ways::Summary;
use trait_ways::Tweet;

// 实例化一个 NewsArticle
fn news_article_instance()
{
    let news_article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("{}", news_article.summarize());
}

// 实例化一个 Tweet
fn tweet_instance()
{
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("{}", tweet.summarize());
}


fn get_largest_test()
{
    let number_list = vec![34, 50, 25, 100, 65];
    let result = trait_ways::get_largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = trait_ways::get_largest(&char_list);
    println!("The largest char is {}", result);
}

/*
 也可以为实现了其它Trait的任意类型有条件的实现某个Trait
 为满足Trait Bound的所有类型上实现Trait叫做覆盖实现(blanketdimplementations)
*/
fn to_string_fn()
{
    let s = 3.to_string();
    println!("{}", s);
}

fn main()
{
    news_article_instance();
    tweet_instance();
    get_largest_test();
    to_string_fn();
}
