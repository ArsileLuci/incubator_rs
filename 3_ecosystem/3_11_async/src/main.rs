#[macro_use]
extern crate structopt;

use futures::{stream, StreamExt};
use std::fs::File;
use std::io::{Read, Write};
use std::iter::*;
use structopt::StructOpt;

#[tokio::main]
async fn main() {

    let settings = Settings::from_args();

    let mut links_file = File::open(settings.get_filename()).unwrap();
    let dir = std::fs::create_dir("./results");

    let client = reqwest::Client::new();
    let mut s: String = String::new();
    links_file.read_to_string(&mut s);

    let urls = s.split("\n").map(|x| x.to_owned());

    let bodies = stream::iter(urls)
        .map(|url:String| {
            let client = &client;
            async move {
                let resp = client.get(&url).send().await.unwrap();
                (resp.text().await.unwrap(), url)
            }
        })
        .buffer_unordered(settings.get_thread_count());

    bodies
        .for_each(|(text, link)| {
            async move {
                let f = &link[8..link.len()].to_owned().trim_end().to_owned()
                    .replace("/","_")
                    .replace(":","_")
                    .replace("?","_");
                let filename = format!("results/{}.html",f);
                println!("{}",filename);
                let mut f= File::create(filename).unwrap();
                f.write_all(text.as_bytes());
            }
        })
        .await;
}
#[derive(Debug, StructOpt)]
#[structopt(name = "3_11", about = "Downloads webpages from provided file")]
struct Settings {
    ///Sets max concurent thread count
    #[structopt(long = "max-threads")]
    max_threads : Option<usize>,
    ///File with urls
    file: String,
}

impl Settings {
    fn get_thread_count(&self) -> usize {
        match self.max_threads {
            Some(x) => x,
            None => num_cpus::get()
        }
    }

    fn get_filename(&self) -> &str {
        &self.file
    }
}