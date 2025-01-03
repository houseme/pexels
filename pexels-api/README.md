# Pexels API

[![Build](https://github.com/houseme/pexels/actions/workflows/build.yml/badge.svg)](https://github.com/houseme/pexels/actions/workflows/build.yml)
[![crates.io](https://img.shields.io/crates/v/pexels-api.svg)](https://crates.io/crates/pexels-api)
[![docs.rs](https://docs.rs/pexels-api/badge.svg)](https://docs.rs/pexels-api/)
[![Crates.io License](https://img.shields.io/crates/l/pexels-api)](../LICENSE-APACHE)
[![Crates.io](https://img.shields.io/crates/d/pexels-api)](https://crates.io/crates/pexels-api)

Pexels API is a Rust library for interacting with the Pexels API. It allows you to search for photos, videos, and collections, as well as retrieve individual media items by their ID.

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
pexels-api = { version = "0.0.1" }
reqwest = { version = "0.12.11", features = ["json"] }
serde = { version = "1.0.217", features = ["derive"] }
serde_json = "1.0.134"
thiserror = "2.0.9 "
tokio = { version = "1", features = ["full"] }
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
use pexels_api::{Pexels, MediaType, MediaSort};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("PEXELS_API_KEY")?;
    let client = Pexels::new(api_key);

    // Search for photos
    let photos = client.search_photos("nature", 10, 1).await?;
    for photo in photos.photos {
        println!("{:?}", photo);
    }

    // Get a photo by ID
    let photo = client.get_photo(10967).await?;
    println!("{:?}", photo);

    // Search for videos
    let videos = client.search_videos("nature", 10, 1).await?;
    for video in videos.videos {
        println!("{:?}", video);
    }

    // Get a video by ID
    let video = client.get_video(3401900).await?;
    println!("{:?}", video);

    // Search for collections
    let collections = client.search_collections(10, 1).await?;
    for collection in collections.collections {
        println!("{:?}", collection);
    }

    // Search for media
    let media_response = client.search_media("nature", 10, 1, MediaType::Photo, MediaSort::Latest).await?;
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
- `search_photos(query: &str, per_page: usize, page: usize) -> Result<PhotosResponse, PexelsError>`: Searches for photos.
- `get_photo(id: u32) -> Result<Photo, PexelsError>`: Retrieves a photo by its ID.
- `search_videos(query: &str, per_page: usize, page: usize) -> Result<VideosResponse, PexelsError>`: Searches for videos.
- `get_video(id: u32) -> Result<Video, PexelsError>`: Retrieves a video by its ID.
- `search_collections(per_page: usize, page: usize) -> Result<CollectionsResponse, PexelsError>`: Searches for collections.
- `search_media(query: &str, per_page: usize, page: usize, media_type: MediaType, sort: MediaSort) -> Result<MediaResponse, PexelsError>`: Searches for media.

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

