use dotenv::dotenv;
use pexels_api::Pexels;
use std::env;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "pexels-cli",
    about = "A CLI for interacting with the Pexels API"
)]
enum Opt {
    SearchPhotos {
        query: String,
        #[structopt(short, long, default_value = "15")]
        per_page: u32,
        #[structopt(short, long, default_value = "1")]
        page: u32,
    },
    SearchVideos {
        query: String,
        #[structopt(short, long, default_value = "15")]
        per_page: u32,
        #[structopt(short, long, default_value = "1")]
        page: u32,
    },
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("PEXELS_API_KEY").expect("PEXELS_API_KEY not set");

    let opt = Opt::from_args();
    let client = Pexels::new(api_key);

    // match opt {
    //     Opt::SearchPhotos {
    //         query,
    //         per_page,
    //         page,
    //     } => {
    //         let photos = client.search_photos(&query, per_page, page).await.unwrap();
    //         for photo in photos {
    //             println!("{:?}", photo);
    //         }
    //     }
    //     Opt::SearchVideos {
    //         query,
    //         per_page,
    //         page,
    //     } => {
    //         let videos = client.search_videos(&query, per_page, page).await.unwrap();
    //         for video in videos {
    //             println!("{:?}", video);
    //         }
    //     }
    // }
}
