// traitsはgoのinterfaceに似ている
// ある型が特定の振る舞いをすることを保証するための仕組み
// 例えば、ある型が特定のメソッドを持つことを保証したい場合に使う
trait Fruit {
    fn price(&self) -> u32;
}

struct Apple;

impl Fruit for Apple {
    fn price(&self) -> u32 {
        5
    }
}

struct Banana;
impl Fruit for Banana {
    fn price(&self) -> u32 {
        10
    }
}

trait Summary {
    fn summarize(&self) -> String {
        // デフォルト実装
        String::from("(Read more...)")
    }
}

trait Message {
    fn message(&self) -> String {
        String::from("This is a message.")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {} in {}", self.headline, self.author, self.location)
    }
}

impl Message for NewsArticle {}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

struct Default {
    content: String,
}

impl Summary for Default {}

pub fn run() {
    let apple = Apple;
    let banana = Banana;

    println!("Apple price: {}", get_price(&apple));
    println!("Banana price: {}", get_price(&banana));

    let tweet = Tweet {
        username: String::from("user1"),
        content: String::from("Hello, world!"),
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Rust is awesome!"),
        location: String::from("Internet"),
        author: String::from("user2"),
    };
    println!("New article available: {}", article.summarize());

    let default = Default {
        content: String::from("This is a default summary."),
    };
    notify(&tweet);
    notify(&article);

    // Defaultはsummarizeメソッドを実装していないが、デフォルト実装があるので呼び出せる
    notify(&default);

    notify2(&article);
}

fn get_price<T: Fruit>(fruit: &T) -> u32 {
    fruit.price()
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn notify2(item: &(impl Summary + Message)) {
    println!("Breaking news! {}", item.summarize());
    println!("Message: {}", item.message());
}
