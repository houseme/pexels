use crate::{User, VideoPicture};
use serde::{Deserialize, Serialize};

/// Photo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Photo {
    pub id: u64,
    pub width: u32,
    pub height: u32,
    pub url: String,
    pub photographer: String,
    pub photographer_url: Option<String>,
    pub photographer_id: Option<u64>,
    pub avg_color: Option<String>,
    pub src: PhotoSources,
    pub alt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoSources {
    pub original: String,
    pub large2x: String,
    pub large: String,
    pub medium: String,
    pub small: String,
    pub portrait: String,
    pub landscape: String,
    pub tiny: String,
}

/// Video
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    pub id: u64,
    pub width: u32,
    pub height: u32,
    pub url: String,
    pub image: String,
    pub duration: u32,
    pub user: User,
    pub video_files: Vec<VideoFile>,
    pub video_pictures: Vec<VideoPicture>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoFile {
    pub id: u64,
    pub quality: String,
    pub file_type: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub link: String,
}

/// Video picture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotosPage {
    pub page: u32,
    pub per_page: u32,
    pub photos: Vec<Photo>,
    pub total_results: u32,
    pub prev_page: Option<String>,
    pub next_page: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideosPage {
    pub page: u32,
    pub per_page: u32,
    pub videos: Vec<Video>,
    pub total_results: u32,
    pub prev_page: Option<String>,
    pub next_page: Option<String>,
}

/// Collection media model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    /// Collection ID
    pub id: String,

    /// Collection title
    pub title: String,

    /// Collection description
    pub description: Option<String>,

    /// Whether the collection is private
    pub private: bool,

    /// Media count in the collection
    pub media_count: u32,

    /// Collection photos count
    pub photos_count: u32,

    /// Collection videos count
    pub videos_count: u32,
}

/// Collections page response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionsPage {
    /// Collections in this page
    pub collections: Vec<Collection>,

    /// Current page number
    pub page: u32,

    /// Number of collections per page
    pub per_page: u32,

    /// Total number of collections
    pub total_results: u32,

    /// URL to the next page
    pub next_page: Option<String>,

    /// URL to the previous page
    pub prev_page: Option<String>,
}

/// Media item type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MediaItem {
    /// Photo media type
    #[serde(rename = "Photo")]
    Photo(Photo),

    /// Video media type
    #[serde(rename = "Video")]
    Video(Video),
}

/// Media page response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaPage {
    /// ID of the collection
    pub id: String,

    /// Media items in this page
    pub media: Vec<MediaItem>,

    /// Current page number
    pub page: u32,

    /// Number of media items per page
    pub per_page: u32,

    /// Total number of media items
    pub total_results: u32,

    /// URL to the next page
    pub next_page: Option<String>,

    /// URL to the previous page
    pub prev_page: Option<String>,
}
