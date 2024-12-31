use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(
    name = "pexels-cli",
    version = "0.1.0",
    about = "A CLI for interacting with the Pexels API"
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Search for photos
    SearchPhotos {
        #[clap(short, long)]
        query: String,
        #[clap(short, long, default_value = "15")]
        per_page: usize,
        #[clap(short, long, default_value = "1")]
        page: usize,
    },
    /// Search for videos
    SearchVideos {
        #[clap(short, long)]
        query: String,
        #[clap(short, long, default_value = "15")]
        per_page: usize,
        #[clap(short, long, default_value = "1")]
        page: usize,
    },
    /// Get a specific photo by ID
    GetPhoto {
        #[clap(short, long)]
        id: usize,
    },
    /// Get a specific video by ID
    GetVideo {
        #[clap(short, long)]
        id: usize,
    },
    /// Search for collections
    SearchCollections {
        #[clap(short, long, default_value = "15")]
        per_page: usize,
        #[clap(short, long, default_value = "1")]
        page: usize,
    },
    /// Search for media
    SearchMedia {
        #[clap(short, long)]
        query: String,
        #[clap(short, long, default_value = "15")]
        per_page: usize,
        #[clap(short, long, default_value = "1")]
        page: usize,
        #[clap(short, long, default_value = "")]
        r#type: String,
        #[clap(short, long, default_value = "asc")]
        sort: String,
    },
}
