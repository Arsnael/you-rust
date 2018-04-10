extern crate reqwest;
extern crate url;
extern crate clap;

use clap::{Arg, App};

use reqwest::Client;
use reqwest::Response;

use url::Url;

use std::io::copy;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let args = App::new("you-rust")
        .version("0.1.0")
        .arg(Arg::with_name("vid")
            .help("The ID of the video to download from Youtube.")
            .required(true)
            .index(1))
        .get_matches();

    let vid = args.value_of("vid").unwrap();
    let url = format!("https://youtube.com/get_video_info?video_id={}", vid);
    youtube_dl(&url);
}

fn youtube_dl(url: &str) {
    let client = Client::new();
    
    //getting video info
    println!("Getting video info");
    let body = client.get(url).send().and_then(|mut res| {
        Ok(res.text().unwrap())
    }).unwrap();

    //selecting first video with extension
    println!("Extracting video info");
    let map = parse_content(&body);
    let streams = map.get("url_encoded_fmt_stream_map").unwrap();
    let videos: Vec<&str> = streams.split(',').collect();
    let info = parse_content(videos[0]);
    
    let url = info["url"].to_string();
    let extension = info["type"].split('/').nth(1).unwrap().split(';').next().unwrap();

    let filename = format!("{}.{}", map["title"], extension);

    println!("About to download video {} at the url: {}", filename, url);

    //getting video
    let resp = client.get(&url).send().unwrap();

    println!("Download starting...");

    write(resp, &filename);
}

fn parse_content(input: &str) -> HashMap<String, String> {
    let u = format!("{}{}", "http://yourust.com?", input);
    let parsed_url = Url::parse(&u).unwrap();
    parsed_url.query_pairs().into_owned().collect()
}

fn write(mut resp: Response, filename: &str) {
    let mut file = File::create(filename).unwrap();

    match copy(&mut resp, &mut file) {
        Ok(_) => println!("Download finished!"),
        Err(msg) => println!("Error! {}", msg)
    }
}
