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
    dotenv().ok();
    let args = Cli::parse();

    match args.command {
        cli::Command::SearchPhotos {
            query,
            per_page,
            page,
        } => {
            let photos = search_photos(&query, per_page, page).await?;
            for photo in photos.photos {
                println!("{:?}", photo);
            }
        }
        cli::Command::SearchVideos {
            query,
            per_page,
            page,
        } => {
            let videos = search_videos(&query, per_page, page).await?;
            for video in videos.videos {
                println!("{:?}", video);
            }
        }
        cli::Command::GetPhoto { id } => {
            let photo = get_photo(id).await?;
            println!("{:?}", photo);
        }
        cli::Command::GetVideo { id } => {
            let video = get_video(id).await?;
            println!("{:?}", video);
        }
        cli::Command::SearchCollections { per_page, page } => {
            let collections = search_collections(per_page, page).await?;
            for collection in collections.collections {
                println!("{:?}", collection);
            }
        }
        cli::Command::SearchMedia {
            query,
            per_page,
            page,
            r#type,
            sort,
        } => {
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
