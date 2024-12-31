/*!
Pexels CLI - A command-line interface for interacting with the Pexels API.
*/
mod api;
mod cli;

use crate::api::{
    get_photo, get_video, search_collections, search_media, search_photos, search_videos,
};
use crate::cli::Cli;
use clap::Parser;
use dotenv::dotenv;
use pexels_api::{MediaSort, MediaType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from a .env file
    dotenv().ok();

    // Parse command-line arguments
    let args = Cli::parse();

    // Match the command and execute the corresponding function
    match args.command {
        cli::Command::SearchPhotos { query, per_page, page } => {
            // Search for photos based on the query
            let photos = search_photos(&query, per_page, page).await?;
            for photo in photos.photos {
                println!("{:?}", photo);
            }
        }
        cli::Command::SearchVideos { query, per_page, page } => {
            // Search for videos based on the query
            let videos = search_videos(&query, per_page, page).await?;
            for video in videos.videos {
                println!("{:?}", video);
            }
        }
        cli::Command::GetPhoto { id } => {
            // Get a photo by its ID
            let photo = get_photo(id).await?;
            println!("{:?}", photo);
        }
        cli::Command::GetVideo { id } => {
            // Get a video by its ID
            let video = get_video(id).await?;
            println!("{:?}", video);
        }
        cli::Command::SearchCollections { per_page, page } => {
            // Search for collections
            let collections = search_collections(per_page, page).await?;
            for collection in collections.collections {
                println!("{:?}", collection);
            }
        }
        cli::Command::SearchMedia { query, per_page, page, r#type, sort } => {
            // Search for media (photos and videos) based on the query
            let mtype = r#type.parse::<MediaType>()?;
            let msort = sort.parse::<MediaSort>()?;
            let media_response = search_media(query, per_page, page, mtype, msort).await?;
            for media in media_response.media {
                println!("{:?}", media);
            }
        }
    }

    Ok(())
}
