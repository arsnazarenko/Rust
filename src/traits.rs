use study::{self, NewsArticle, Tweet, Summary, WebSitePost, notify};
use std::path::Display;

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
    {
        #[derive(Clone, Debug)]
        //  попросили компилятору самому вывести реализацию clone()
        //  здесь это работает, так как i32 - copy тип, b - объект, реализущий Clone trait
        struct TestStruct {
            a: i32,
            b: String
        }
        //  trait Default
        impl Default for TestStruct {
            fn default() -> Self {
                TestStruct {
                    a: 0,
                    b: "".to_string()
                }
            }
        }

        //  Copy

        {   //  #[derive(Clone, Copy)]
            struct Wrapper<T> {
                ptr: *const T,
            }

            // Наша структура - это обертка над константным указателем, который всегда является Copy
            // Но derive имеет одно фундоментальное ограничение:он проверяет, если структура с парметром T реализует трэйт,
            // то параметр T тоже должен реализовыать трэйт
            // [derive(Clone)] создает примерно такую конструкцию:
            // impl<T: Clone> Clone for Wrapper<T> {
            //     fn clone(&self) -> Self {
            //         //...
            //     }
            // }
            // Тогда Wrapper<Vec<i32>> не будет copy, так как Vec<i32> не copy,
            // хотя константный указатель всегда должен быть Copy
            // Решение - определить самому и делегировать поведение Clone к Copy
            impl<T> Copy for Wrapper<T> {}

            impl<T> Clone for Wrapper<T> {
                fn clone(&self) -> Self {
                    //  Делаем простое копирование, так как тип Copy
                    *self
                }
            }

        }

        let t = TestStruct::default();
        let test_struct = t.clone();
        println!("{:?}, {:?}", t, test_struct)

    }


}

