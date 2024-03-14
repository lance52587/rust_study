pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String{
        // String::from("(Read more...)")
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for NewsArticle{
    fn summarize_author(&self) -> String{
        format!("@{}", self.author)
    }
    // fn summarize(&self) -> String{
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}


impl Summary for Tweet{
    fn summarize_author(&self) -> String{
        format!("@{}", self.username)
    }
    // fn summarize(&self) -> String{
    //     format!("{}: {}", self.username, self.content)
    // }
}

// pub fn notify<T: Summary>(item: T){
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify(item: impl Summary){
    println!("Breaking news! {}", item.summarize());
}

fn returns_summarizable() -> impl Summary{
    Tweet{
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}