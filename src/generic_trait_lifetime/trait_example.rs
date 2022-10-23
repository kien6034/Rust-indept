use std::{fmt::Debug, iter::Sum};

pub fn run(){
    summarize();
    conditional_trait_bound();
}

// This trait can containts shared behavior between New article and tweet 
pub trait Summary{
    fn summarize(&self)-> String;

    fn default_welcome(&self)-> String {
        return "Welcome".to_string();
    } 
}

#[derive(Debug)]
pub struct Article {
    pub name: String,
    pub author: String,
    pub content: String
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub retweet: u32
}


impl Summary for Article {
    fn summarize(&self)-> String {
        format!("Author: {} with content {}", self.author, self.content)
    }
   
}

impl Summary for Tweet {
    fn summarize(&self)-> String {
        format!("Tweet of: {} - {} retweet,  with content {}", self.username, self.retweet, self.content)
    }
}

fn summarize(){
    println!("\n-----------\n Summarize");

    let article = Article{
        name: "machine leanring".to_string(),
        author: "andrew ng".to_string(),
        content: "This is a fake content".to_string()
    };

    let tweet = Tweet {
        username : "elonmusk".to_string(),
        content : "we are going to mars".to_string(),
        retweet : 1000
    };

    println!("Article: {}", article.summarize());
    println!("Tweet: {}", tweet.summarize());

    println!("Default: {}", article.default_welcome());

    traits_as_param(&article);
    traits_bound(&tweet);
    multiple_trait_bound(&tweet);
    multiple_trait_bound_with_where(&article);
}


fn traits_as_param(item: &impl Summary){
    println!("Trait with params:{:?}", item.summarize());
}

/// This function is equivelent with the traits_as_param function
fn traits_bound<T:Summary > (item: &T) {
    println!("Trait bound:{:?}", item.summarize());
}

/// Generic type T implment the Summary and Debug trait
fn multiple_trait_bound<T:Summary + Debug> (item: &T) {
    println!("Debug: {:?}", item);
}

/// Same as multiple_trait_bound, but clearer
fn multiple_trait_bound_with_where<T> (item: &T)
where 
    T: Summary + Debug
{
    println!("Multiple trait bound with where:{:?}", item.summarize());
    println!("Print Item: {:?}", item);
}

// Trait can be used for return type
fn _get_summarizable_object ()-> impl Summary {
    Article{
        name: "machine leanring".to_string(),
        author: "andrew ng".to_string(),
        content: "This is a fake content".to_string()
    }
}

// This is not allowed, will cover in chapter 17
// fn return_summarizable(switch: bool) -> impl Summary
// {
//     if switch {
//         Article{
//             name: "machine leanring".to_string(),
//             author: "andrew ng".to_string(),
//             content: "This is a fake content".to_string()
//         }
//     }
//     else {
//         Tweet {
//             username : "elonmusk".to_string(),
//             content : "we are going to mars".to_string(),
//             retweet : 1000
//         }
//     }
// }


/// TRAIT BOUNDS TO CONDITIONALLY IMPLEMENT METHODS

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

struct PersonalInfo {
    name: String,
    age: i32
}

// Todo: Dive deep into how derive work
#[derive(PartialEq, PartialOrd,)]
struct PersonalInfo2 {
    name: String,
    age: i32
}

impl Display for PersonalInfo2{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.name, self.age)
    }
}

fn conditional_trait_bound(){
    println!("\n-----------\n Contitional Traibound");

    let pair = Pair::new("vietnam".to_string(), "kien6034".to_string());
    pair.cmp_display();

    let kien = PersonalInfo {name: "kien".to_string(), age: 23};
    let kevin = PersonalInfo {name: "kevin".to_string(), age: 22};

    let _pair2 = Pair::new(kien, kevin);
    // _pair2.cmp_display(); // will fail since PersonalInfo doesnt implement PartialOrd

    let kien = PersonalInfo2 {name: "kien".to_string(), age: 23};
    let kevin = PersonalInfo2 {name: "kevin".to_string(), age: 22};

    let pair2 = Pair::new(kien, kevin);
    pair2.cmp_display(); // This will work since we implement display and PartialOrd trait for PersonalInfo2
}