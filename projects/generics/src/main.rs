use std::string::String;
use std::cmp::PartialOrd;
use std::marker::Copy;

fn main() {
    println!("Hello, world!");

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);


    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("{}", tweet.summarize_author());
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}


struct Point<T> {
    x: T,
    y: T,
}


impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Point1<T, U> {
    x: T,
    y: U,
}

pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_author(&self) -> String {
        format!("(Read more from {}...)", self.summarize())
    }
}

pub trait Display {
    fn display(&self) -> String;
}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify3<T: Summary + Display>(item: T) {}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Display for NewsArticle {
    fn display(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
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
}