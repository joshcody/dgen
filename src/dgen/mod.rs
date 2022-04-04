use rand;
use rand::prelude::SliceRandom;
use rand::distributions::{Distribution, Uniform};
use serde::Deserialize; 
use lazy_static::lazy_static;
use include_bytes;


lazy_static! {
    static ref WORDLIST:Vec<String> = load_words();
    static ref POPLIST:Vec<POP> = load_pops();
}

pub fn ipaddress() -> String {
    let octet = Uniform::from(1..255);
    let mut octets = vec![];
    for _ in 0..4 {
        octets.push(octet.sample(&mut rand::thread_rng()).to_string());
    }
    return String::from(octets.join("."));
}

pub fn pop() -> String {
    let p = POPLIST.choose(&mut rand::thread_rng()).unwrap();
    let out = format!("{} ({}, {})", p.code, p.city,  p.country);
    return out.clone();
}

pub fn word() -> String {
    let w = WORDLIST.choose(&mut rand::thread_rng()).unwrap();
    return w.clone();
}

pub fn choice(items: &Vec<String>) -> String {
    let r = items.choose(&mut rand::thread_rng());
    return match r {
        Some(v) => v.to_string(),// v.to_owned(),
        None => "".to_string()
    };
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct POP {
    #[serde(rename(deserialize = "Code"))]
    code: String,
    #[serde(rename(deserialize = "Name"))]    
    name: String,
    #[serde(rename(deserialize = "City"))]
    city: String,
    #[serde(rename(deserialize = "State"), default)]
    state: Option<String>,
    #[serde(rename(deserialize = "Country"))]
    country: String,
}

fn load_pops() -> Vec<POP> {
    let src_bytes = include_bytes!("../../data/airports.jsonl");
    let src_string = String::from_utf8_lossy(src_bytes);
    let pops = serde_json::from_str(&src_string).unwrap();
    return pops;
}

fn load_words() -> Vec<String> {
    let src_bytes = include_bytes!("../../data/words");
    let src_string = String::from_utf8_lossy(src_bytes);
    let words = src_string.split("\n").map(|x| String::from(x)).collect::<Vec<String>>();
    return words
}
