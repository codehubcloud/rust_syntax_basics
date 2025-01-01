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

fn main()
{
    news_article_instance();
    tweet_instance();
}
