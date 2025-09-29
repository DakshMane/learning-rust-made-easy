use traits::{NewsArticle, SocialPost, Summary};
fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course , as you probably already knew , people  , "),

        reply: false,
        retweet: false,
    };

    println!("New Post : {}", post.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}
