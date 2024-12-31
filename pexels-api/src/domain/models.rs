use serde::{Deserialize, Serialize};

/// Represents the response for a list of collections.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionsResponse {
    pub collections: Vec<Collection>,
    pub page: u32,
    pub per_page: u32,
    pub total_results: u32,
    pub next_page: Option<String>,
    pub prev_page: Option<String>,
}

/// Represents a Pexels collection.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Collection {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub private: bool,
    pub media_count: u32,
    pub photos_count: u32,
    pub videos_count: u32,
}

/// Represents the response for a list of media items.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaResponse {
    pub id: String,
    pub media: Vec<MediaType>, // An array of media objects. Each object has an extra type attribute to indicate the type of object.
    pub page: u32,
    pub per_page: u32,
    pub total_results: u32,
    pub next_page: Option<String>,
    pub prev_page: Option<String>,
}

/// Enum representing the type of media.
/// Supported values are `photos` and `videos`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MediaType {
    Photo(MediaPhoto),
    Video(MediaVideo),
}

/// Represents a photo media object.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaPhoto {
    pub type_: String,
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub url: Option<String>,
    pub photographer: Option<String>,
    pub photographer_url: Option<String>,
    pub photographer_id: u32,
    pub avg_color: String,
    pub src: PhotoSrc,
    pub liked: bool,
}

/// Represents a video media object.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaVideo {
    pub type_: String,
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub duration: u32,
    pub full_res: Option<String>,
    pub tags: Vec<String>,
    pub url: Option<String>,
    pub image: Option<String>,
    pub avg_color: Option<String>,
    pub user: User,
    pub video_files: Vec<VideoFile>,
    pub video_pictures: Vec<VideoPicture>,
}

/// Represents a Pexels photo.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Photo {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub url: String,
    pub photographer: String,
    pub photographer_url: String,
    pub photographer_id: u32,
    pub avg_color: String,
    pub src: PhotoSrc,
    pub liked: bool,
    pub alt: String,
}

/// Represents different image sizes for a photo.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhotoSrc {
    pub original: String,
    pub large2x: String,
    pub large: String,
    pub medium: String,
    pub small: String,
    pub portrait: String,
    pub landscape: String,
    pub tiny: String,
}

/// Represents the response for a list of photos.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhotosResponse {
    pub total_results: u32,
    pub page: u32,
    pub per_page: u32,
    pub photos: Vec<Photo>,
    pub next_page: Option<String>,
    pub prev_page: Option<String>,
}

/// Represents a Pexels video.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Video {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub url: String,
    pub image: String,
    pub full_res: Option<String>,
    pub tags: Vec<String>,
    pub duration: u32,
    pub user: User,
    pub video_files: Vec<VideoFile>,
    pub video_pictures: Vec<VideoPicture>,
}

/// Represents the response for a list of videos.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoResponse {
    pub page: u32,
    pub per_page: u32,
    pub total_results: u32,
    pub url: String,
    pub videos: Vec<Video>,
    pub prev_page: Option<String>,
    pub next_page: Option<String>,
}

/// Represents a user who created a media item.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub url: String,
}

/// Represents a video file with different qualities.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoFile {
    pub id: u32,
    pub quality: String,
    pub file_type: String,
    pub width: u32,
    pub height: u32,
    pub fps: f32, // Note that this should be of type f32 because fps is a decimal
    pub link: String,
}

/// Represents a preview picture of a video.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoPicture {
    pub id: u32,
    pub picture: String,
    pub nr: u32,
}
