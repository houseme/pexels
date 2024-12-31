use pexels_api::{
    CollectionsResponse, MediaBuilder, MediaResponse, MediaSort, MediaType, Pexels, PexelsError,
    Photo, PhotosResponse, SearchBuilder, Video, VideoResponse, VideoSearchBuilder,
};
use std::env;

pub async fn search_photos(
    query: &str,
    per_page: usize,
    page: usize,
) -> Result<PhotosResponse, PexelsError> {
    let api_key = env::var("PEXELS_API_KEY")?;
    let client = Pexels::new(api_key);
    let builder = SearchBuilder::new()
        .query(query)
        .per_page(per_page)
        .page(page);
    let photos = client.search_photos(builder).await?;
    Ok(photos)
}

pub async fn search_videos(
    query: &str,
    per_page: usize,
    page: usize,
) -> Result<VideoResponse, PexelsError> {
    let api_key = env::var("PEXELS_API_KEY")?;
    let client = Pexels::new(api_key);
    let builder = VideoSearchBuilder::new()
        .query(query)
        .per_page(per_page)
        .page(page);
    let videos = client.search_videos(builder).await?;
    Ok(videos)
}

pub async fn get_photo(id: usize) -> Result<Photo, PexelsError> {
    let api_key = env::var("PEXELS_API_KEY")?;
    let client = Pexels::new(api_key);
    let photo = client.get_photo(id).await?;
    Ok(photo)
}

pub async fn get_video(id: usize) -> Result<Video, PexelsError> {
    let api_key = env::var("PEXELS_API_KEY")?;
    let client = Pexels::new(api_key);
    let video = client.get_video(id).await?;
    Ok(video)
}

pub async fn search_collections(
    per_page: usize,
    page: usize,
) -> Result<CollectionsResponse, PexelsError> {
    let api_key = env::var("PEXELS_API_KEY")?;
    let client = Pexels::new(api_key);
    let collections = client.search_collections(per_page, page).await?;
    Ok(collections)
}

pub async fn search_media(
    id: String,
    per_page: usize,
    page: usize,
    r#type: MediaType,
    sort: MediaSort,
) -> Result<MediaResponse, PexelsError> {
    let api_key = env::var("PEXELS_API_KEY")?;
    let client = Pexels::new(api_key);
    let builder = MediaBuilder::new()
        .id(id)
        .per_page(per_page)
        .page(page)
        .r#type(r#type)
        .sort(sort);
    let media_response = client.search_media(builder).await?;
    Ok(media_response)
}
