use std::fmt::{Debug, Display};

fn main() {
    let n = NewsArticle {
        headline: "My news headline".to_string(),
        location: "here".to_string(),
        author: "me".to_string(),
        content: "Just some test content".to_string(),
    };
    println!("news article summary: {}", n.summarize());

    let t = Tweet {
        username: "@me".to_string(),
        content: "A test tweet".to_string(),
        reply: true,
        retweet: true,
    };
    println!("tweet summary: {}", t.summarize());
}

pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more from{}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("by {} ({})", self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_trait_bound<T: Summary>(item: &T) {}
pub fn notify_trait_bound_2<T: Summary>(item1: &T, item2: &T) {}

pub fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: "test".to_string(),
        content: "test".to_string(),
        reply: false,
        retweet: false,
    }
}
