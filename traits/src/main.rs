
pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    // we only define method signatures except we want to have a default implementation
    fn summarize_author(&self) -> String;
    
    fn summarize(&self) -> String{
        format!("(Read more from {:?}...)", self.summarize_author())
    }
   
}

impl Summary for NewsArticle{
    // using the default implementation
    // fn summarize(&self) -> String {
    //     format!("{}, by {}", self.headline, self.author)
    // }

    // declaring a custom implementation
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

impl Summary for Tweet{
    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// this function takes in item which is a reference to something that implements summary e.g struct tweet
pub fn notify(item: &impl Summary){
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let tweet: Tweet = Tweet {
        username: String::from( "@johndoe"),
        content: String::from( "Hello World!"),
        reply: false,
        retweet: false
    };
    
    let article: NewsArticle = NewsArticle {
        author: String::from( "John Doe"),
        headline: String::from( "The Sky is Falling!"),
        content: String::from( "The sky is not actually falling.")
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());
    println!("Tweet author: {}", tweet.summarize_author());
    println!("Article author: {}", article.summarize_author());

    notify(&article);
}
