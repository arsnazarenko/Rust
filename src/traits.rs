use study::{self, NewsArticle, Tweet, Summary, WebSitePost, notify};

pub fn main() {
    let tweet: Tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let news: NewsArticle = NewsArticle {
        content: String::from("Hello, is very seriously news!"),
        author: String::from("Arseniy Nazarenko"),
        location: "Saint-P".to_string(),
        headline: "I been waiting this for the long time...".to_string(),
    };

    let post: WebSitePost = WebSitePost {
        author: String::from("Nikolay Z"),
        author_nickname: String::from("RUSToman"),
        content: "It is my first post!".to_string(),
        likes: 1234
    };
    println!("Tweet: {}", tweet.summarize());
    println!("News: {}", news.summarize());
    println!("Post: {}", post.summarize());

    notify(&tweet);
    notify(&news);
    notify(&post);

}