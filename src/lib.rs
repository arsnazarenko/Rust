use std::fmt::Display;

pub trait Summary {

    fn summarize_author(&self) -> String;

    //  Поведение по умолчанию
    //  Мы определили тело у метода трэйта
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}



pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
     fn summarize(&self) -> String {
        format!("{}, by: {}", self.headline, self.summarize_author())
    }

    fn summarize_author(&self) -> String {
        format!("(@{}, {})", self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    //  более длинаая форма self
   fn summarize(self: &Tweet) -> String {
       format!("{}: {}", self.summarize_author(), self.content)
   }

}

pub struct WebSitePost {
    pub author: String,
    pub author_nickname: String,
    pub content: String,
    pub likes: usize
}

impl Summary for WebSitePost {
    fn summarize_author(&self) -> String {
        format!("@{} ({})", self.author_nickname, self.author)
    }
}

//  указание ГРАНИЦЫ ТИПАЖА для обобщенных функций

pub fn notify<T: Summary>(item: &T) {
    println!("New information!\n {}", item.summarize())
}


// ВОЗВРАЩЕНИЕ значения, реализующего типаж

// Используя impl Summary для возвращаемого типа, мы указываем,
// что функция returns_summarizable возвращает некоторый тип,
// который реализует типаж Summary без обозначения конкретного типа.
// В этом случае returns_summarizable возвращает Tweet, но код,
// вызывающий эту функцию, этого не знает.

// Однако, impl Trait возможно использовать, если возвращаете только один тип.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}