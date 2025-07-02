use reqwest::{header, Client, StatusCode};
use std::time::Duration;
use url::Url;

use crate::models::{CollectionsPage, MediaPage, Photo, PhotosPage, Video, VideosPage};
use crate::search::{PaginationParams, SearchParams, VideoSearchParams};
use crate::PexelsError;

/// Main client for the Pexels API
///
/// This client provides methods to interact with all endpoints of the Pexels API
/// and handles authentication, request building and response parsing.
pub struct PexelsClient {
    /// API key for authentication with Pexels API
    api_key: String,

    /// HTTP client with connection pooling and configurable timeouts
    client: Client,

    /// Base URL for the Pexels API
    base_url: String,
}

impl PexelsClient {
    /// Creates a new PexelsClient with the provided API key
    ///
    /// # Arguments
    ///
    /// * `api_key` - The Pexels API key
    ///
    /// # Returns
    ///
    /// A new instance of PexelsClient
    ///
    /// # Example
    ///
    /// ```
    /// use pexels_api::PexelsClient;
    ///
    /// let client = PexelsClient::new("your_api_key");
    /// ```
    pub fn new<S: Into<String>>(api_key: S) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .pool_max_idle_per_host(10)
            .build()
            .unwrap_or_default();

        Self { api_key: api_key.into(), client, base_url: "https://api.pexels.com/v1".to_string() }
    }

    /// Creates a new PexelsClient with custom configuration
    ///
    /// # Arguments
    ///
    /// * `api_key` - The Pexels API key
    /// * `timeout` - Request timeout in seconds
    /// * `max_idle_connections` - Maximum number of idle connections per host
    ///
    /// # Returns
    ///
    /// A new instance of PexelsClient
    pub fn with_config<S: Into<String>>(
        api_key: S,
        timeout: u64,
        max_idle_connections: usize,
    ) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout))
            .pool_max_idle_per_host(max_idle_connections)
            .build()
            .unwrap_or_default();

        Self { api_key: api_key.into(), client, base_url: "https://api.pexels.com/v1".to_string() }
    }

    /// Sets a custom base URL for the Pexels API
    ///
    /// # Arguments
    ///
    /// * `base_url` - The custom base URL
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_base_url<S: Into<String>>(mut self, base_url: S) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Search for photos matching the specified query and parameters
    ///
    /// # Arguments
    ///
    /// * `query` - The search query
    /// * `params` - Additional search parameters (pagination, filters, etc.)
    ///
    /// # Returns
    ///
    /// A Result containing the photos search response or an error
    ///
    /// # Example
    ///
    /// ```
    /// use pexels_api::{PexelsClient, SearchParams,Size};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = PexelsClient::new("your_api_key");
    ///     let params = SearchParams::new()
    ///         .page(1)
    ///         .per_page(15)
    ///         .size(Size::Large);
    ///
    ///     let photos = client.search_photos("nature", &params).await?;
    ///     println!("Found {} photos", photos.total_results);
    ///     Ok(())
    /// }
    /// ```
    pub async fn search_photos(
        &self,
        query: &str,
        params: &SearchParams,
    ) -> Result<PhotosPage, PexelsError> {
        let mut url = Url::parse(&format!("{}/search", self.base_url))?;

        // Add query parameter
        url.query_pairs_mut().append_pair("query", query);

        // Add all search parameters
        for (key, value) in params.to_query_params() {
            url.query_pairs_mut().append_pair(&key, &value);
        }

        let response = self.send_request(url).await?;

        match response.status() {
            StatusCode::OK => {
                let photos_page: PhotosPage = response.json().await?;
                Ok(photos_page)
            }
            StatusCode::UNAUTHORIZED => Err(PexelsError::AuthError("Invalid API key".to_string())),
            StatusCode::TOO_MANY_REQUESTS => Err(PexelsError::RateLimitError),
            status => {
                Err(PexelsError::ApiError(format!("Search photos failed with status: {status}")))
            }
        }
    }

    /// Fetch curated/featured photos
    ///
    /// # Arguments
    ///
    /// * `params` - Pagination parameters
    ///
    /// # Returns
    ///
    /// A Result containing the curated photos response or an error
    pub async fn curated_photos(
        &self,
        params: &PaginationParams,
    ) -> Result<PhotosPage, PexelsError> {
        let mut url = Url::parse(&format!("{}/curated", self.base_url))?;

        // Add pagination parameters
        if let Some(page) = params.page {
            url.query_pairs_mut().append_pair("page", &page.to_string());
        }

        if let Some(per_page) = params.per_page {
            url.query_pairs_mut().append_pair("per_page", &per_page.to_string());
        }

        let response = self.send_request(url).await?;

        match response.status() {
            StatusCode::OK => {
                let photos_page: PhotosPage = response.json().await?;
                Ok(photos_page)
            }
            StatusCode::UNAUTHORIZED => Err(PexelsError::AuthError("Invalid API key".to_string())),
            StatusCode::TOO_MANY_REQUESTS => Err(PexelsError::RateLimitError),
            status => {
                Err(PexelsError::ApiError(format!("Curated photos failed with status: {status}")))
            }
        }
    }

    /// Get a specific photo by its ID
    ///
    /// # Arguments
    ///
    /// * `id` - The photo ID
    ///
    /// # Returns
    ///
    /// A Result containing the photo or an error
    pub async fn get_photo(&self, id: u64) -> Result<Photo, PexelsError> {
        let url = Url::parse(&format!("{}/photos/{}", self.base_url, id))?;

        let response = self.send_request(url).await?;

        match response.status() {
            StatusCode::OK => {
                let photo: Photo = response.json().await?;
                Ok(photo)
            }
            StatusCode::NOT_FOUND => {
                Err(PexelsError::NotFound(format!("Photo with ID {id} not found")))
            }
            StatusCode::UNAUTHORIZED => Err(PexelsError::AuthError("Invalid API key".to_string())),
            StatusCode::TOO_MANY_REQUESTS => Err(PexelsError::RateLimitError),
            status => {
                Err(PexelsError::ApiError(format!("Get photo failed with status: {status}")))
            }
        }
    }

    /// Search for videos matching the specified query and parameters
    ///
    /// # Arguments
    ///
    /// * `query` - The search query
    /// * `params` - Additional search parameters (pagination, filters, etc.)
    ///
    /// # Returns
    ///
    /// A Result containing the videos search response or an error
    pub async fn search_videos(
        &self,
        query: &str,
        params: &VideoSearchParams,
    ) -> Result<VideosPage, PexelsError> {
        let mut url = Url::parse(&format!("{}/videos/search", self.base_url))?;

        // Add query parameter
        url.query_pairs_mut().append_pair("query", query);

        // Add all search parameters from VideoSearchParams
        if let Some(page) = params.page {
            url.query_pairs_mut().append_pair("page", &page.to_string());
        }

        if let Some(per_page) = params.per_page {
            url.query_pairs_mut().append_pair("per_page", &per_page.to_string());
        }

        if let Some(ref orientation) = params.orientation {
            url.query_pairs_mut().append_pair("orientation", orientation.as_str());
        }

        if let Some(ref size) = params.size {
            url.query_pairs_mut().append_pair("size", size.as_str());
        }

        if let Some(ref locale) = params.locale {
            url.query_pairs_mut().append_pair("locale", locale);
        }

        let response = self.send_request(url).await?;

        match response.status() {
            StatusCode::OK => {
                let videos_page: VideosPage = response.json().await?;
                Ok(videos_page)
            }
            StatusCode::UNAUTHORIZED => Err(PexelsError::AuthError("Invalid API key".to_string())),
            StatusCode::TOO_MANY_REQUESTS => Err(PexelsError::RateLimitError),
            status => {
                Err(PexelsError::ApiError(format!("Search videos failed with status: {status}")))
            }
        }
    }

    /// Fetch popular videos
    ///
    /// # Arguments
    ///
    /// * `params` - Pagination parameters
    ///
    /// # Returns
    ///
    /// A Result containing the popular videos response or an error
    pub async fn popular_videos(
        &self,
        params: &PaginationParams,
    ) -> Result<VideosPage, PexelsError> {
        let mut url = Url::parse(&format!("{}/videos/popular", self.base_url))?;

        // Add pagination parameters
        if let Some(page) = params.page {
            url.query_pairs_mut().append_pair("page", &page.to_string());
        }

        if let Some(per_page) = params.per_page {
            url.query_pairs_mut().append_pair("per_page", &per_page.to_string());
        }

        let response = self.send_request(url).await?;

        match response.status() {
            StatusCode::OK => {
                let videos_page: VideosPage = response.json().await?;
                Ok(videos_page)
            }
            StatusCode::UNAUTHORIZED => Err(PexelsError::AuthError("Invalid API key".to_string())),
            StatusCode::TOO_MANY_REQUESTS => Err(PexelsError::RateLimitError),
            status => {
                Err(PexelsError::ApiError(format!("Popular videos failed with status: {status}")))
            }
        }
    }

    /// Get a specific video by its ID
    ///
    /// # Arguments
    ///
    /// * `id` - The video ID
    ///
    /// # Returns
    ///
    /// A Result containing the video or an error
    pub async fn get_video(&self, id: u64) -> Result<Video, PexelsError> {
        let url = Url::parse(&format!("{}/videos/videos/{}", self.base_url, id))?;

        let response = self.send_request(url).await?;

        match response.status() {
            StatusCode::OK => {
                let video: Video = response.json().await?;
                Ok(video)
            }
            StatusCode::NOT_FOUND => {
                Err(PexelsError::NotFound(format!("Video with ID {id} not found")))
            }
            StatusCode::UNAUTHORIZED => Err(PexelsError::AuthError("Invalid API key".to_string())),
            StatusCode::TOO_MANY_REQUESTS => Err(PexelsError::RateLimitError),
            status => {
                Err(PexelsError::ApiError(format!("Get video failed with status: {status}")))
            }
        }
    }

    /// Get collections list
    ///
    /// # Arguments
    ///
    /// * `params` - Pagination parameters
    ///
    /// # Returns
    ///
    /// A Result containing the collections response or an error
    pub async fn get_collections(
        &self,
        params: &PaginationParams,
    ) -> Result<CollectionsPage, PexelsError> {
        let mut url = Url::parse(&format!("{}/collections", self.base_url))?;

        // Add pagination parameters
        if let Some(page) = params.page {
            url.query_pairs_mut().append_pair("page", &page.to_string());
        }

        if let Some(per_page) = params.per_page {
            url.query_pairs_mut().append_pair("per_page", &per_page.to_string());
        }

        let response = self.send_request(url).await?;

        match response.status() {
            StatusCode::OK => {
                let collections_page: CollectionsPage = response.json().await?;
                Ok(collections_page)
            }
            StatusCode::UNAUTHORIZED => Err(PexelsError::AuthError("Invalid API key".to_string())),
            StatusCode::TOO_MANY_REQUESTS => Err(PexelsError::RateLimitError),
            status => Err(PexelsError::ApiError(format!(
                "Get collections failed with status: {status}"
            ))),
        }
    }

    /// Get collection media items (photos and videos)
    ///
    /// # Arguments
    ///
    /// * `id` - The collection ID
    /// * `params` - Pagination parameters
    ///
    /// # Returns
    ///
    /// A Result containing the media response or an error
    pub async fn get_collection_media(
        &self,
        id: &str,
        params: &PaginationParams,
    ) -> Result<MediaPage, PexelsError> {
        let mut url = Url::parse(&format!("{}/collections/{}", self.base_url, id))?;

        // Add pagination parameters
        if let Some(page) = params.page {
            url.query_pairs_mut().append_pair("page", &page.to_string());
        }

        if let Some(per_page) = params.per_page {
            url.query_pairs_mut().append_pair("per_page", &per_page.to_string());
        }

        let response = self.send_request(url).await?;

        match response.status() {
            StatusCode::OK => {
                let media_page: MediaPage = response.json().await?;
                Ok(media_page)
            }
            StatusCode::NOT_FOUND => {
                Err(PexelsError::NotFound(format!("Collection with ID {id} not found")))
            }
            StatusCode::UNAUTHORIZED => Err(PexelsError::AuthError("Invalid API key".to_string())),
            StatusCode::TOO_MANY_REQUESTS => Err(PexelsError::RateLimitError),
            status => Err(PexelsError::ApiError(format!(
                "Get collection media failed with status: {status}"
            ))),
        }
    }

    /// Helper method to send authenticated requests to the Pexels API
    ///
    /// # Arguments
    ///
    /// * `url` - The fully constructed URL to send the request to
    ///
    /// # Returns
    ///
    /// A Result containing the HTTP response or an error
    async fn send_request(&self, url: Url) -> Result<reqwest::Response, PexelsError> {
        let response =
            self.client.get(url).header(header::AUTHORIZATION, &self.api_key).send().await?;

        Ok(response)
    }
}
