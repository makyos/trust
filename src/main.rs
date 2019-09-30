#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::env;
use std::io::{stdout, Write};
extern crate twitter_stream;
use twitter_stream::{Token, TwitterStreamBuilder};
use twitter_stream::rt::{self, Future, Stream};

extern crate regex;
use regex::Regex;

#[derive(Deserialize, Debug)]
struct Tweet {
    text: String,
}

struct Cfg {
    track_word: String,
}

impl Cfg {
    fn new(args: &[String]) -> Cfg {
        let track_word = args[1].clone();
        Cfg { track_word }
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let cfg = Cfg::new(&args);

    let token = Token::new(
        "IhpRxSgkk257Q97cWodoPg", 
        "c2PsEJQMgA3REMUM3AU9xsoxaFNo4QOQexD6uXjh33M", 
        "5735002-1yk7kztSaYgK5Ohcz0zxgy54jpo82lJWI1H35tmI3R", 
        "BGIgwvSyKmOEdt9W5ZAHXp7IHLA6cfspyf1l2B54A8U");

    let re1 = Regex::new(r"#.*|http.*").unwrap();
    let _re2 = Regex::new(r" *").unwrap();

    let future = TwitterStreamBuilder::filter(token)
        .track(Some(cfg.track_word.as_str()))
        .listen()
	    .unwrap()
        .flatten_stream()
        .for_each(move |json| {
            match serde_json::from_str(&json) {
                Ok(j) => {
                    let tweet: Tweet = j;
                    let text = &tweet.text.replace("\n", " ");
                    print!("----- {} ", re1.replace_all(&text, "").trim());
                    stdout().flush().unwrap();
                }
                Err(_) => {
                    println!("\nImpossible to cast");
                    println!("{}", json);
                }
            }
            Ok(())
        })
        .map_err(|e| println!("error: {}", e));

    rt::run(future);
}
