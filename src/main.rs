use lazy_static::lazy_static;
use toml;
use serde::Deserialize;
use serde_json;
use clap::Parser;
use std::collections::HashMap;

use regex::Regex;

use crate::toml::Value::{
    Array, 
    String as TOMLString,
};


mod dgen;

#[derive(Parser)]
#[clap(name = "dgen")]
#[clap(author = "Josh Cody <josh@thatdot.com")]
#[clap(version = "0.1")]
struct Cli {
    #[clap(long, required=false)]
    config: Option<String>,
    #[clap(long, takes_value=false)]
    ipaddress: bool,
    #[clap(long, takes_value=false)]
    pop: bool,
    #[clap(long, required=false)]
    from_list: Option<String>,
    #[clap(long, takes_value=false)]
    word: bool,
    #[clap(long, required=false)]
    from_file: Option<String>,
}

// CLI options are file <OR> the other values

#[derive(Debug, Default, Deserialize)]
struct Config {
    record_count: Option<i64>,
    fields: HashMap<String, toml::Value>,
}

impl Config {
    fn new(path: String) -> Config {
        let f = std::fs::read_to_string(path).expect("Could not read file");
        toml::from_str(&f).unwrap()
    }
}

fn main() {
    // generate magically awesome data
    let cli = Cli::parse();

    if let Some(_) = cli.config.as_ref() {
        process_file(Config::new(cli.config.unwrap()));
    } else {
        if cli.ipaddress {
            println!("{}", dgen::ipaddress());
        }
        if cli.pop {
            // check if the user provided a list
            if let Some(v) = cli.from_list {
                let items = v.split(",").map(|x| String::from(x)).collect();
                println!("{}", dgen::choice(&items));
            } else if let Some(v) = cli.from_file {
                let data:String = std::fs::read_to_string(v).unwrap().parse().unwrap();
                let items:Vec<String> = data.split("\n").map(|x| String::from(x)).collect();
                println!("{}", dgen::choice(&items));


            } else {
                println!("{}",dgen::pop());
            }
        }
        if cli.word {
            println!("{}",dgen::word());
        }
    }
}


fn word_replacer(target: String) -> String {
    lazy_static! {
        static ref WORD: Regex = Regex::new("<word>").unwrap();
    }

    if WORD.is_match(&target) {
        let res = WORD.replace(&target, dgen::word()).to_owned();
        word_replacer(String::from(res))
    } else {
        target
    }
}

// process the config file down here.
fn process_file(conf: Config) {
    // load our "database" of values
    lazy_static! {
        static ref ADDR: Regex = Regex::new("<ipaddress>").unwrap();
        static ref POP: Regex = Regex::new("<pop>").unwrap();
    }

    let ct = match conf.record_count {
        Some(v) => v,
        None => 0 // defaults to a single record in the loop
    };

    for _ in 0..ct {
        let mut data:HashMap<String, String> = HashMap::new();

        for field in &conf.fields {
            let val = match field.1 {
                TOMLString(v) => {
                    // loop through our options
                    let that = v.to_string().clone();
                    let this = ADDR.replace(&that, dgen::ipaddress()).to_owned();
                    let this = word_replacer(String::from(this));
                    let this = POP.replace(&this, dgen::pop()).to_owned();

                    String::from(this)
                },
                Array(v) => {
                    let r:Vec<String> = v.iter().map(|x| x.as_str().unwrap().to_string()).collect();
                    dgen::choice(&r)
                },
                _ => "".to_string()
            };
            data.insert(field.0.to_string(), val.to_string());
        }
        println!("{}", serde_json::to_string(&data).unwrap());
    };
}