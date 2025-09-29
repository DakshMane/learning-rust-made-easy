use std::fmt::{Debug, Display};

// creating a trait
pub trait Summary {
    // equivalent to interface ..
    /*Here, we declare a trait using the trait keyword and then the trait’s name, which is Summary in this case. We also declare the trait as pub so that crates depending on this crate can make use of this trait too, as we’ll see in a few examples */
    // fn summarize(&self) -> String;

    // default implementation .
    fn summarize(&self) -> String {
        format!("(Read more... from {} )", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

//implementing our trait

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

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// trait as params

pub fn notify(item: &impl Summary) {
    /*Instead of a concrete type for the item parameter, we specify the impl keyword and the trait name. This parameter accepts any type that implements the specified trait. In the body of notify, we can call any methods on item that come from the Summary trait, such as summarize. We can call notify and pass in any instance of NewsArticle or SocialPost */
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound syntax

//This longer form is equivalent to the example in the previous section but is more verbose
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking new ! {}", item.summarize());
}

pub fn notifies(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

pub fn notifies2<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

pub fn notifywithDisplay<T: Summary + Display>(item: &T) {}

/*Using too many trait bounds has its downsides. Each generic has its own trait bounds, so functions with multiple generic type parameters can contain lots of trait bound information between the function’s name and its parameter list,
making the function signature hard to read. */
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    2
}
// this is the better way to do 👇👇
fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    2
}
