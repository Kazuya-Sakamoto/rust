trait Fruits {
    fn price(&self) -> u32;
    
}
struct Apple;
impl Fruits for Apple {
    fn price(&self) -> u32 {
        10
    }
}
struct Banana;
impl Fruits for Banana {
    fn price(&self) -> u32 {
        5
    }
}

trait Summary {
    fn summarize(&self) -> String;
}
trait Message {
    fn message(&self) -> String {
        String::from("Hello")
    }
}
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
impl Message for NewsArticle {}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn run() {
    let apple = Apple {};
    let banana = Banana {};
    get_price(apple);
    get_price(banana);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best"),
    };
    println!("{}", article.summarize());

    notify(&article);
    nortify_another(&article);

}

fn get_price<T: Fruits>(fruit: T) {
    println!("The price is {}", fruit.price());
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
fn nortify_another(item: &(impl Summary + Message)) {
    println!("Breaking news! {}", item.summarize());
    println!("Message {}", item.message());
}