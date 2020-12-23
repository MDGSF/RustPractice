use std::fmt;

pub trait Summary {
    fn summarize(&self) -> String;

    fn new_summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} {}", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        String::from("")
    }
}

impl fmt::Display for NewsArticle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, by {} {}", self.headline, self.author, self.location)
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

impl fmt::Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.username, self.content)
    }
}

impl Tweet {
    fn show(&self) {
        println!("{}: {}", self.username, self.content)
    }
}

/*
notify1, notify2, notify3 实现相同的行为。
*/
pub fn notify1(item: &impl Summary) {
    println!("notify1 Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item: &T) {
    println!("notify2 Breaking news! {}", item.summarize());
}

pub fn notify3<T>(item: &T)
where
    T: Summary,
{
    println!("notify3 Breaking news! {}", item.summarize());
}

/*
notify4, notify5, notify6 实现相同的行为。
*/
pub fn notify4(item: &(impl Summary + fmt::Display)) {
    println!("notify4 Breaking news! {}", item);
}

pub fn notify5<T: Summary + fmt::Display>(item: &T) {
    println!("notify5 Breaking news! {}", item);
}

pub fn notify6<T>(item: &T)
where
    T: Summary + fmt::Display,
{
    println!("notify6 Breaking news! {}", item);
}

/*
bignotify1, bignotify2, bignotify3 实现相同的行为。
*/
pub fn bignotify1(
    T: &(impl Summary + fmt::Display),
    U: &(impl Summary + fmt::Display + fmt::Debug),
) {
    println!("bignotify1 Breaking news! \n\t{}\n\t{}", T, U);
}

pub fn bignotify2<T: Summary + fmt::Display, U: Summary + fmt::Display + fmt::Debug>(t: &T, u: &U) {
    println!("bignotify2 Breaking news! \n\t{}\n\t{}", t, u);
}

pub fn bignotify3<T, U>(t: &T, u: &U)
where
    T: Summary + fmt::Display,
    U: Summary + fmt::Display + fmt::Debug,
{
    println!("bignotify3 Breaking news! \n\t{}\n\t{}", t, u);
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    tweet.show();
    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new tweet: {}", tweet.new_summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    println!();

    notify1(&tweet);
    notify1(&article);

    notify2(&tweet);
    notify2(&article);

    notify3(&tweet);
    notify3(&article);

    println!();

    notify4(&tweet);
    notify4(&article);

    notify5(&tweet);
    notify5(&article);

    notify6(&tweet);
    notify6(&article);

    println!();

    bignotify1(&tweet, &article);
    bignotify2(&tweet, &article);
    bignotify3(&tweet, &article);
}
