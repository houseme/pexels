# Pexels API

[![Build](https://github.com/houseme/pexels/actions/workflows/build.yml/badge.svg)](https://github.com/houseme/pexels/actions/workflows/build.yml)
[![crates.io](https://img.shields.io/crates/v/pexels-api.svg)](https://crates.io/crates/pexels-api)
[![docs.rs](https://docs.rs/pexels-api/badge.svg)](https://docs.rs/pexels-api/)
[![Crates.io License](https://img.shields.io/crates/l/pexels-api)](../LICENSE-APACHE)
[![Crates.io](https://img.shields.io/crates/d/pexels-api)](https://crates.io/crates/pexels-api)

Pexels API is a Rust library for interacting with the Pexels API. It allows you to search for photos, videos, and
collections, as well as retrieve individual media items by their ID.

## Features

- Search for photos and videos
- Retrieve individual photos and videos by ID
- Search for collections
- Retrieve featured collections
- Supports asynchronous operations

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
dotenvy = "0.15.7"
pexels-api = { version = "0.1.0" }
reqwest = { version = "0.12.11", features = ["json"] }
serde = { version = "1.0.217", features = ["derive"] }
serde_json = "1.0.135"
thiserror = "2.0.9 "
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
url = "2.5.4"
```

## Usage

Before using the library, make sure to set your Pexels API key in a `.env` file:

```sh
PEXELS_API_KEY=your_api_key_here
```

### Example

Here is a basic example of how to use the library:

```rust
use dotenvy::dotenv;
use pexels_api::{
    CollectionMediaParams, MediaSort, MediaType, PaginationParams, PexelsClient,
    PopularVideoParams, SearchParams, VideoSearchParams,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// Load the environment variables from the .env file
    dotenv().ok();

    /// Get the Pexels API key from the environment
    let api_key = env::var("PEXELS_API_KEY")?;

    /// Create a new Pexels client
    let client = PexelsClient::new(api_key);

    /// Search for photos
    let photos = client
        .search_photos("nature", &SearchParams::new().per_page(10).page(1))
        .await?;
    for photo in photos.photos {
        println!("{:?}", photo);
    }

    /// Get a photo by ID
    let photo = client.get_photo(10967).await?;
    println!("{:?}", photo);

    /// Search for videos
    let videos = client
        .search_videos("nature", &VideoSearchParams::new().per_page(10).page(1))
        .await?;
    for video in videos.videos {
        println!("{:?}", video);
    }

    /// Fetch popular videos with documented filters
    let popular_videos = client
        .popular_videos_with_params(
            &PopularVideoParams::new().per_page(10).page(1).min_width(1280).min_duration(5),
        )
        .await?;
    println!("{:?}", popular_videos);

    /// Get a video by ID
    /// Note: The video ID is just an example. You should replace it with a valid video ID.
    /// You can get a video ID by searching for videos or collections.
    /// # Example video ID: 25460961   
    let video = client.get_video(25460961).await?;
    println!("{:?}", video);

    /// List featured collections
    let collections = client
        .get_featured_collections(&PaginationParams::new().per_page(10).page(1))
        .await?;
    for collection in collections.collections {
        println!("{:?}", collection);
    }

    /// Fetch collection media with type and sort filters
    let media_response = client
        .get_collection_media_with_params(
            "tszhfva",
            &CollectionMediaParams::new()
                .per_page(10)
                .page(1)
                .media_type(MediaType::Photo)
                .sort(MediaSort::Desc),
        )
        .await?;
    for media in media_response.media {
        println!("{:?}", media);
    }

    Ok(())
}
```

## API Documentation

### Pexels

The main client for interacting with the Pexels API.

#### Methods

- `new(api_key: String) -> Self`: Creates a new Pexels client.
- `search_photos(query: &str, params: &SearchParams) -> Result<PhotosPage, PexelsError>`: Searches for photos.
- `get_photo(id: u32) -> Result<Photo, PexelsError>`: Retrieves a photo by its ID.
- `search_videos(query: &str, params: &VideoSearchParams) -> Result<VideosPage, PexelsError>`: Searches for videos.
- `popular_videos_with_params(params: &PopularVideoParams) -> Result<VideosPage, PexelsError>`: Retrieves popular videos with optional size and duration filters.
- `get_video(id: u32) -> Result<Video, PexelsError>`: Retrieves a video by its ID.
- `get_collections(params: &PaginationParams) -> Result<CollectionsPage, PexelsError>`: Retrieves collections.
- `get_featured_collections(params: &PaginationParams) -> Result<CollectionsPage, PexelsError>`: Retrieves featured collections.
- `get_collection_media_with_params(id: &str, params: &CollectionMediaParams) -> Result<MediaPage, PexelsError>`: Retrieves collection media with optional `type` and `sort` filters.

## Documentation

For detailed documentation, please refer to [Documentation](https://docs.rs/pexels-api).

## License

Licensed under either of

* Apache License, Version 2.0, [LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0
* MIT license [LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 or MIT license, shall be dual licensed as above, without any additional terms or conditions.
