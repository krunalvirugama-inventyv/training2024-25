pub trait Summary {
    // fn get_author(&self) -> String;
    fn get_author(&self) -> &str;

    fn summraize(&self) -> String {
        format!("{} Read More..", self.get_author())
    }
}

pub trait Demo {
    fn get_demo(&self) -> String;
}

struct News {
    author: String,
    content: String,
    headline: String,
    location: String,
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for News {
    // fn summraize(&self) -> String {
    //     format!(
    //         "This Is News Author is {} and content is {}.",
    //         self.author, self.content
    //     )
    // }

    // fn get_author(&self) -> String {
    //     self.author.to_string()
    // }

    fn get_author(&self) -> &str {
        self.author.as_str()
    }
}

impl Demo for News {
    fn get_demo(&self) -> String {
        "This is News Demo Print".to_string()
    }
}

impl Demo for Tweet {
    fn get_demo(&self) -> String {
        "This is Tweet Demo Print".to_string()
    }
}

impl Summary for Tweet {
    fn summraize(&self) -> String {
        format!(
            "-->This Is Tweet Username is {} and content is {}.",
            self.username, self.content
        )
    }

    // fn get_author(&self) -> String {
    //     self.username.to_string()
    // }

    fn get_author(&self) -> &str {
        self.username.as_str()
    }
}

fn main() {
    let news = News {
        author: String::from("Krunal Virugama"),
        content: String::from("This is news Artical"),
        headline: String::from("Day Of The Life"),
        location: String::from("Ahmedabad"),
    };

    let news2 = News {
        author: String::from("Deep Patel"),
        content: String::from("This BD Artical"),
        headline: String::from("BD Life"),
        location: String::from("Ahmedabad"),
    };

    news_aggregator(&news);
    println!("{}", news.summraize());

    let tweet = Tweet {
        username: String::from("virugamacoder"),
        content: String::from("My First Tweet"),
        reply: true,
        retweet: false,
    };

    news_aggregator(&tweet);
    println!("{}", tweet.summraize());
    get_news(&tweet);

    get_mixupnewssame(&news, &news2);
    get_mixupnewsdiffrent(&news, &tweet);

    bothtraittrun(&news);
}


fn news_aggregator(source: &impl Summary) 
{
    println!("\n---> There is news in Market (impl Summray)");
    println!("{}", source.summraize());
}

fn get_news<T: Summary>(source: &T) {
    println!("{}", source.summraize());
}

fn get_mixupnewssame<T: Summary>(source: &T, source2: &T) {
    println!(
        "\n\nMixup Same Type Use Generic {} {}",
        source.summraize(),
        source2.summraize()
    );
}

fn get_mixupnewsdiffrent(source: &News, source2: &Tweet) {
    println!(
        "\n\nMixup Diffrent Type But Does Not use generic \n{} \n{}",
        source.summraize(),
        source2.summraize()
    );
}

fn bothtraittrun<T: Summary + Demo>(source: &T)
// fn bothtraittrun(source : &(impl Summary + Demo))
{
    println!("\n---> There is Both Traint ( &(impl Summary + Demo) )");
    println!("{}\n {}", source.summraize(), source.get_demo());
}
