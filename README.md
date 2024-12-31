# Pexels Project

[![Build](https://github.com/houseme/pexels/workflows/build/badge.svg)](https://github.com/houseme/pexels/actions?query=workflow%3ABuild)
[![crates.io](https://img.shields.io/crates/v/pexels-api.svg)](https://crates.io/crates/pexels-api)
[![docs.rs](https://docs.rs/pexels-api/badge.svg)](https://docs.rs/pexels-api/)
[![License](https://img.shields.io/crates/l/pexels-api)](../LICENSE-APACHE)
[![Crates.io](https://img.shields.io/crates/d/pexels-api)](https://crates.io/crates/pexels-api)

This project consists of two main components: `pexels-api` and `pexels-cli`. The `pexels-api` is a Rust library for interacting with the Pexels API, while the
`pexels-cli` is a command-line interface for using the `pexels-api`.

## Features

- **pexels-api**:
    - Search for photos and videos
    - Retrieve individual photos and videos by ID
    - Search for collections
    - Retrieve featured collections
    - Supports asynchronous operations

- **pexels-cli**:
    - Command-line interface for searching photos, videos, and collections
    - Retrieve individual media items by their ID

## Installation

### pexels-api

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

### pexels-cli

1. Clone the repository:
    ```sh
    git clone https://github.com/houseme/pexels.git
    ```

2. Navigate to the project directory:
    ```sh
    cd pexels/pexels-cli
    ```

3. Build the project:
    ```sh
    cargo build
    ```

## Usage

Before using the library or CLI, make sure to set your Pexels API key in a `.env` file:

```sh
PEXELS_API_KEY=your_api_key_here
```

### Pexels-api Example

Here is a basic example of how to use the `pexels-api` library:

```rust
use dotenv::dotenv;
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

### Pexels-cli Commands

- **Search Photos**:
    ```sh
    cargo run -- search-photos --query "nature" --per-page 10 --page 1
    ```

- **Search Videos**:
    ```sh
    cargo run -- search-videos --query "nature" --per-page 10 --page 1
    ```

- **Get Photo by ID**:
    ```sh
    cargo run -- get-photo --id 10967
    ```

- **Get Video by ID**:
    ```sh
    cargo run -- get-video --id 3401900
    ```

- **Search Collections**:
    ```sh
    cargo run -- search-collections --per-page 10 --page 1
    ```

- **Search Media**:
    ```sh
    cargo run -- search-media --query "nature" --per-page 10 --page 1 --type "photo" --sort "latest"
    ```

## Documentation

For detailed documentation, please refer to [pexels-api Documentation](https://docs.rs/pexels-api) and [pexels-cli Documentation](https://docs.rs/pexels-cli).

## License

Licensed under either of

* Apache License, Version 2.0, [LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0
* MIT license [LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 or MIT license, shall be dual licensed as above, without any additional terms or conditions.