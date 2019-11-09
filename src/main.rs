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

extern crate rand;
use rand::Rng;

extern crate colored;                                                                   
use colored::*;


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

static mut LAST_COLOR: i32 = 0;

fn paint(s: &String) -> colored::ColoredString {

    let mut color: i32 = rand::thread_rng().gen_range(1, 7);
    unsafe {
        while color == LAST_COLOR {
            color = rand::thread_rng().gen_range(1, 7);
        }
        LAST_COLOR = color;
    }

    match color {
        1 => return s.color("white").on_color("red"),
        2 => return s.color("white").on_color("green"),
        3 => return s.color("white").on_color("yellow"),
        4 => return s.color("white").on_color("blue"),
        5 => return s.color("white").on_color("magenta"),
        _ => return s.color("white").on_color("cyan"),
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
    let re2 = Regex::new(r"  +").unwrap();

    let future = TwitterStreamBuilder::filter(token)
        .track(Some(cfg.track_word.as_str()))
        .listen()
	    .unwrap()
        .flatten_stream()
        .for_each(move |json| {
            match serde_json::from_str(&json) {
                Ok(j) => {
                    let tweet: Tweet = j;
                    let mut text:String = String::from(&tweet.text);
                    text = text.trim().replace("\n", " ");
                    text = text.trim().replace("\r", " ");
                    text = String::from(re1.replace_all(text.trim(), ""));
                    text = String::from(re2.replace_all(text.trim(), " "));
                    //print!("----- {} ", re1.replace_all(&text, "").trim());
                    print!("{}", paint(&format!(" {} ", &text)));
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
