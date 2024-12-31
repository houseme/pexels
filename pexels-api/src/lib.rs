/*!
The `pexels_api` crate provides API wrapper for Pexels. It based on [Pexels API Documentation](https://www.pexels.com/api/documentation/).

To get the API key, you have to request the access from [Request API Access – Pexels](https://www.pexels.com/api/new/).

This library depends on [serde-json](https://github.com/serde-rs/json) crate to handle the result, thus you have to read the documentation [serde_json - Rust](https://docs.serde.rs/serde_json/index.html), especially [serde_json::Value - Rust](https://docs.serde.rs/serde_json/enum.Value.html).

# Setup

Add this line to your `Cargo.toml` file, below `[dependencies]`

```toml
pexels_api = "*"
```

and this to your crate root file e.g. `main.rs`:

```rust
use pexels_api;
```

Done! Now you can use this API wrapper.

# Example

This example shows how to get the list of *mountains* photos.

```rust
use dotenv::dotenv;
use std::env;
use pexels_api::{Pexels,SearchBuilder};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("PEXELS_API_KEY").expect("PEXELS_API_KEY not set");
    let pexels_api_client = Pexels::new(api_key);
    let builder = SearchBuilder::new()
        .query("mountains")
        .per_page(15)
        .page(1);
    let response = pexels_api_client.search_photos(builder).await.expect("Failed to get photos");
    println!("{:?}", response);
}
```

and you can run it using `cargo run`! Simply as that.

# Random photo

If you want to get a random photo, you can use the `curated_photo` function and set per_page to 1 and page to a random number between 1 and 1000 to get a beautiful random photo. You can do the same with popular searches if you want to get a random photo with a specific topic.

# Image formats

* original - The size of the original image is given with the attribute width and height.
* large - This image has a maximum width of 940px and a maximum height of 650px. It has the aspect ratio of the original image.
* large2x - This image has a maximum width of 1880px and a maximum height of 1300px. It has the aspect ratio of the original image.
* medium - This image has a height of 350px and a flexible width. It has the aspect ratio of the original image.
* small - This image has a height of 130px and a flexible width. It has the aspect ratio of the original image.
* portrait - This image has a width of 800px and a height of 1200px.
* landscape - This image has a width of 1200px and height of 627px.
* tiny - This image has a width of 280px and height of 200px.
*/
mod collections;
mod domain;
mod photos;
mod videos;

/// collections module
pub use collections::featured::Featured;
pub use collections::featured::FeaturedBuilder;
pub use collections::items::Collections;
pub use collections::items::CollectionsBuilder;
pub use collections::media::Media;
pub use collections::media::MediaBuilder;
/// domain module
pub use domain::models::Collection;
pub use domain::models::CollectionsResponse;
pub use domain::models::MediaResponse;
pub use domain::models::Photo;
pub use domain::models::PhotoSrc;
pub use domain::models::PhotosResponse;
pub use domain::models::User;
pub use domain::models::Video;
pub use domain::models::VideoFile;
pub use domain::models::VideoPicture;
pub use domain::models::VideoResponse;
/// photos module
pub use photos::curated::Curated;
pub use photos::curated::CuratedBuilder;
pub use photos::photo::FetchPhoto;
pub use photos::photo::FetchPhotoBuilder;
pub use photos::search::Color;
pub use photos::search::Hex;
pub use photos::search::Search;
pub use photos::search::SearchBuilder;
/// videos module
pub use videos::popular::Popular;
pub use videos::popular::PopularBuilder;
pub use videos::search::Search as VideoSearch;
pub use videos::search::SearchBuilder as VideoSearchBuilder;
pub use videos::video::FetchVideo;
pub use videos::video::FetchVideoBuilder;

/// import crate
use reqwest::Client;
use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;
use serde_json::Value;
use std::env::VarError;
use std::str::FromStr;
use thiserror::Error;
use url::ParseError;

/// Pexels API version
const PEXELS_VERSION: &str = "v1";

/// Path for videos
const PEXELS_VIDEO_PATH: &str = "videos";

/// Path for collections
const PEXELS_COLLECTIONS_PATH: &str = "collections";

/// Pexels API URL
const PEXELS_API: &str = "https://api.pexels.com";

/// Desired photo orientation.
#[derive(PartialEq, Debug)]
pub enum Orientation {
    Landscape,
    Portrait,
    Square,
}

impl Orientation {
    fn as_str(&self) -> &str {
        match self {
            Orientation::Landscape => "landscape",
            Orientation::Portrait => "portrait",
            Orientation::Square => "square",
        }
    }
}

impl FromStr for Orientation {
    type Err = PexelsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "landscape" => Ok(Orientation::Landscape),
            "portrait" => Ok(Orientation::Portrait),
            "square" => Ok(Orientation::Square),
            _ => Err(PexelsError::ParseMediaSortError),
        }
    }
}

/// The order of items in the media collection.
/// Supported values are: asc, desc. Default: asc
#[derive(PartialEq, Debug)]
pub enum MediaSort {
    Asc,
    Desc,
}

impl MediaSort {
    fn as_str(&self) -> &str {
        match self {
            MediaSort::Asc => "asc",
            MediaSort::Desc => "desc",
        }
    }
}

impl FromStr for MediaSort {
    type Err = PexelsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "asc" => Ok(MediaSort::Asc),
            "desc" => Ok(MediaSort::Desc),
            _ => Err(PexelsError::ParseMediaSortError),
        }
    }
}

/// The type of media you are requesting.
/// If not given or if given with an invalid value, all media will be returned.
/// Supported values are photos and videos.
#[derive(PartialEq, Debug)]
pub enum MediaType {
    Photo,
    Video,
    Empty,
}

impl MediaType {
    fn as_str(&self) -> &str {
        match self {
            MediaType::Photo => "photos",
            MediaType::Video => "videos",
            MediaType::Empty => "",
        }
    }
}

impl FromStr for MediaType {
    type Err = PexelsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "photo" => Ok(MediaType::Photo),
            "video" => Ok(MediaType::Video),
            "" => Ok(MediaType::Empty),
            _ => Err(PexelsError::ParseMediaTypeError),
        }
    }
}

/// The locale of the search you are performing.
#[allow(non_camel_case_types)]
pub enum Locale {
    en_US,
    pt_BR,
    es_ES,
    ca_ES,
    de_DE,
    it_IT,
    fr_FR,
    sv_SE,
    id_ID,
    pl_PL,
    ja_JP,
    zh_TW,
    zh_CN,
    ko_KR,
    th_TH,
    nl_NL,
    hu_HU,
    vi_VN,
    cs_CZ,
    da_DK,
    fi_FI,
    uk_UA,
    el_GR,
    ro_RO,
    nb_NO,
    sk_SK,
    tr_TR,
    ru_RU,
}

impl Locale {
    fn as_str(&self) -> &str {
        match self {
            Locale::en_US => "en-US",
            Locale::pt_BR => "pt-BR",
            Locale::es_ES => "es-ES",
            Locale::ca_ES => "ca-ES",
            Locale::de_DE => "de-DE",
            Locale::it_IT => "it-IT",
            Locale::fr_FR => "fr-FR",
            Locale::sv_SE => "sv-SE",
            Locale::id_ID => "id-ID",
            Locale::pl_PL => "pl-PL",
            Locale::ja_JP => "ja-JP",
            Locale::zh_TW => "zh-TW",
            Locale::zh_CN => "zh-CN",
            Locale::ko_KR => "ko-KR",
            Locale::th_TH => "th-TH",
            Locale::nl_NL => "nl-NL",
            Locale::hu_HU => "hu-HU",
            Locale::vi_VN => "vi-VN",
            Locale::cs_CZ => "cs-CZ",
            Locale::da_DK => "da-DK",
            Locale::fi_FI => "fi-FI",
            Locale::uk_UA => "uk-UA",
            Locale::el_GR => "el-GR",
            Locale::ro_RO => "ro-RO",
            Locale::nb_NO => "nb-NO",
            Locale::sk_SK => "sk-SK",
            Locale::tr_TR => "tr-TR",
            Locale::ru_RU => "-ES",
        }
    }
}

/// Minimum videos/photo size.
pub enum Size {
    Large,
    Medium,
    Small,
}

impl Size {
    fn as_str(&self) -> &str {
        match self {
            Size::Large => "large",
            Size::Medium => "medium",
            Size::Small => "small",
        }
    }
}

impl FromStr for Size {
    type Err = PexelsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "large" => Ok(Size::Large),
            "medium" => Ok(Size::Medium),
            "small" => Ok(Size::Small),
            _ => Err(PexelsError::ParseSizeError),
        }
    }
}

/// Builder result type
pub(crate) type BuilderResult = Result<String, PexelsError>;

/// Errors that can occur while interacting with the Pexels API.
#[derive(Debug, Error)]
pub enum PexelsError {
    #[error("Failed to send HTTP request: {0}")]
    RequestError(#[from] ReqwestError),
    #[error("Failed to parse JSON response: {0}")]
    JsonParseError(#[from] JsonError),
    #[error("API key not found in environment variables: {0}")]
    EnvVarError(#[from] VarError),
    #[error("API key not found in environment variables")]
    ApiKeyNotFound,
    #[error("Failed to parse URL: {0}")]
    ParseError(#[from] ParseError),
    #[error("Invalid hex color code: {0}")]
    HexColorCodeError(String),
    #[error("Failed to parse media type: invalid value")]
    ParseMediaTypeError,
    #[error("Failed to parse media sort: invalid value")]
    ParseMediaSortError,
    #[error("Failed to parse orientation: invalid value")]
    ParseOrientationError,
    #[error("Failed to parse size: invalid value")]
    ParseSizeError,
}

// Manual implementation PartialEq
impl PartialEq for PexelsError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Compare RequestError
            (PexelsError::RequestError(e1), PexelsError::RequestError(e2)) => {
                e1.to_string() == e2.to_string()
            }
            // Compare JsonParseError
            (PexelsError::JsonParseError(e1), PexelsError::JsonParseError(e2)) => {
                e1.to_string() == e2.to_string()
            }
            // Compare ApiKeyNotFound
            (PexelsError::ApiKeyNotFound, PexelsError::ApiKeyNotFound) => true,
            // Compare ParseError
            (PexelsError::ParseError(e1), PexelsError::ParseError(e2)) => {
                e1.to_string() == e2.to_string()
            }
            // Compare HexColorCodeError
            (PexelsError::HexColorCodeError(msg1), PexelsError::HexColorCodeError(msg2)) => {
                msg1 == msg2
            }
            // Other things are not equal
            _ => false,
        }
    }
}

/// Client for interacting with the Pexels API
pub struct Pexels {
    client: Client,
    api_key: String,
}

impl Pexels {
    /// Create a new Pexels client.
    pub fn new(api_key: String) -> Self {
        Pexels { client: Client::new(), api_key }
    }

    /// Sends an HTTP GET request to the specified URL and returns the JSON response.
    /// Utilizes the `reqwest` crate for making HTTP requests.
    ///
    /// # Errors
    /// Returns a `PexelsError` if the request fails or the response cannot be parsed as JSON.
    async fn make_request(&self, url: &str) -> Result<Value, PexelsError> {
        let json_response = self
            .client
            .get(url)
            .header("Authorization", &self.api_key)
            .send()
            .await?
            .json::<Value>()
            .await?;
        Ok(json_response)
    }

    /// Retrieves a list of photos from the Pexels API based on the search criteria.
    ///
    /// # Arguments
    /// * `builder` - A `SearchBuilder` instance with the search parameters.
    ///
    /// # Errors
    /// Returns a `PexelsError` if the request fails or the response cannot be parsed as JSON.
    pub async fn search_photos(
        &self,
        builder: SearchBuilder<'_>,
    ) -> Result<PhotosResponse, PexelsError> {
        builder.build().fetch(self).await
    }

    /// Retrieves a photo by its ID from the Pexels API.
    ///
    /// # Arguments
    /// * `id` - The ID of the photo to retrieve.
    ///
    /// # Errors
    /// Returns a `PexelsError` if the request fails or the response cannot be parsed as JSON.
    pub async fn get_photo(&self, id: usize) -> Result<Photo, PexelsError> {
        FetchPhotoBuilder::new().id(id).build().fetch(self).await
    }

    /// Retrieves a list of videos from the Pexels API based on the search criteria.
    ///
    /// # Arguments
    /// * `builder` - A `VideoSearchBuilder` instance with the search parameters.
    ///
    /// # Errors
    /// Returns a `PexelsError` if the request fails or the response cannot be parsed as JSON.
    pub async fn search_videos(
        &self,
        builder: VideoSearchBuilder<'_>,
    ) -> Result<VideoResponse, PexelsError> {
        builder.build().fetch(self).await
    }

    /// Retrieves a video by its ID from the Pexels API.
    ///
    /// # Arguments
    /// * `id` - The ID of the video to retrieve.
    ///
    /// # Errors
    /// Returns a `PexelsError` if the request fails or the response cannot be parsed as JSON.
    pub async fn get_video(&self, id: usize) -> Result<Video, PexelsError> {
        FetchVideoBuilder::new().id(id).build().fetch(self).await
    }

    /// Retrieves a list of collections from the Pexels API.
    ///
    /// # Arguments
    /// * `per_page` - The number of collections to retrieve per page.
    /// * `page` - The page number to retrieve.
    ///
    /// # Errors
    /// Returns a `PexelsError` if the request fails or the response cannot be parsed as JSON.
    pub async fn search_collections(
        &self,
        per_page: usize,
        page: usize,
    ) -> Result<CollectionsResponse, PexelsError> {
        CollectionsBuilder::new().per_page(per_page).page(page).build().fetch(self).await
    }

    /// Retrieves all media (photos and videos) within a single collection.
    ///
    /// # Arguments
    /// * `builder` - A `MediaBuilder` instance with the search parameters.
    ///
    /// # Errors
    /// Returns a `PexelsError` if the request fails or the response cannot be parsed as JSON.
    pub async fn search_media(&self, builder: MediaBuilder) -> Result<MediaResponse, PexelsError> {
        builder.build().fetch(self).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[test]
    fn test_pexels_error_partial_eq() {
        let err1 = PexelsError::ApiKeyNotFound;
        let err2 = PexelsError::ApiKeyNotFound;
        assert_eq!(err1, err2);

        let err3 = PexelsError::HexColorCodeError(String::from("Invalid color"));
        let err4 = PexelsError::HexColorCodeError(String::from("Invalid color"));
        assert_eq!(err3, err4);

        let err9 = PexelsError::ParseError(ParseError::EmptyHost);
        let err10 = PexelsError::ParseError(ParseError::EmptyHost);
        assert_eq!(err9, err10);

        // 测试不相等的情况
        let err11 = PexelsError::ApiKeyNotFound;
        let err12 = PexelsError::HexColorCodeError(String::from("Invalid color"));
        assert_ne!(err11, err12);
    }

    #[test]
    fn test_parse_photo() {
        let input = "photo";
        let media_type = input.parse::<MediaType>();
        assert_eq!(media_type, Ok(MediaType::Photo));
    }

    #[test]
    fn test_parse_video() {
        let input = "video";
        let media_type = input.parse::<MediaType>();
        assert_eq!(media_type, Ok(MediaType::Video));
    }

    #[test]
    fn test_parse_invalid() {
        let input = "audio";
        let media_type = input.parse::<MediaType>();
        assert!(matches!(media_type, Err(PexelsError::ParseMediaTypeError)));
    }

    #[tokio::test]
    async fn test_make_request() {
        dotenv().ok();
        let api_key = std::env::var("PEXELS_API_KEY").expect("PEXELS_API_KEY not set");
        let client = Pexels::new(api_key);
        let url = "https://api.pexels.com/v1/curated";
        let response = client.make_request(url).await;
        assert!(response.is_ok());
    }
}
