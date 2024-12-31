use crate::{Pexels, PexelsError, VideoResponse, PEXELS_API, PEXELS_VIDEO_PATH};
use url::Url;

/// Path to get popular videos.
const PEXELS_POPULAR_PATH: &str = "popular";

/// Fetches the current popular Pexels videos.
pub struct Popular {
    min_width: Option<usize>,
    min_height: Option<usize>,
    min_duration: Option<usize>,
    max_duration: Option<usize>,
    page: Option<usize>,
    per_page: Option<usize>,
}

impl Popular {
    /// Creates [`PopularBuilder`] for building URI's.
    pub fn builder() -> PopularBuilder {
        PopularBuilder::default()
    }

    /// Create URI from inputted vales from the [`PopularBuilder`].
    pub fn create_uri(&self) -> crate::BuilderResult {
        let uri = format!("{}/{}/{}", PEXELS_API, PEXELS_VIDEO_PATH, PEXELS_POPULAR_PATH);

        let mut url = Url::parse(uri.as_str())?;

        if let Some(min_width) = &self.min_width {
            url.query_pairs_mut().append_pair("min_width", min_width.to_string().as_str());
        }

        if let Some(min_height) = &self.min_height {
            url.query_pairs_mut().append_pair("min_height", min_height.to_string().as_str());
        }

        if let Some(min_duration) = &self.min_duration {
            url.query_pairs_mut().append_pair("min_duration", min_duration.to_string().as_str());
        }

        if let Some(max_duration) = &self.max_duration {
            url.query_pairs_mut().append_pair("max_duration", max_duration.to_string().as_str());
        }

        if let Some(page) = &self.page {
            url.query_pairs_mut().append_pair("page", page.to_string().as_str());
        }

        if let Some(per_page) = &self.per_page {
            url.query_pairs_mut().append_pair("per_page", per_page.to_string().as_str());
        }

        Ok(url.into())
    }

    /// Fetches the list of popular videos from the Pexels API.
    pub async fn fetch(&self, client: &Pexels) -> Result<VideoResponse, PexelsError> {
        let url = self.create_uri()?;
        let response = client.make_request(url.as_str()).await?;
        let response_video: VideoResponse = serde_json::from_value(response)?;
        Ok(response_video)
    }
}

/// Builder for [`Popular`].
#[derive(Default)]
pub struct PopularBuilder {
    min_width: Option<usize>,
    min_height: Option<usize>,
    min_duration: Option<usize>,
    max_duration: Option<usize>,
    page: Option<usize>,
    per_page: Option<usize>,
}

impl PopularBuilder {
    /// Creates a new [`PopularBuilder`].
    pub fn new() -> Self {
        Self {
            min_width: None,
            min_height: None,
            min_duration: None,
            max_duration: None,
            page: None,
            per_page: None,
        }
    }

    /// The minimum width in pixels of the returned videos.
    pub fn min_width(mut self, min_width: usize) -> Self {
        self.min_width = Some(min_width);
        self
    }

    /// The minimum height in pixels of the returned videos.
    pub fn min_height(mut self, min_height: usize) -> Self {
        self.min_height = Some(min_height);
        self
    }

    /// The minimum duration in seconds of the returned videos.
    pub fn min_duration(mut self, min_duration: usize) -> Self {
        self.min_duration = Some(min_duration);
        self
    }

    /// The maximum duration in seconds of the returned videos.
    pub fn max_duration(mut self, max_duration: usize) -> Self {
        self.max_duration = Some(max_duration);
        self
    }

    /// The page number you are requesting.
    pub fn page(mut self, page: usize) -> Self {
        self.page = Some(page);
        self
    }

    /// The number of results you are requesting per page.
    pub fn per_page(mut self, per_page: usize) -> Self {
        self.per_page = Some(per_page);
        self
    }

    /// Create [`Popular`] from the [`PopularBuilder`]
    pub fn build(self) -> Popular {
        Popular {
            page: self.page,
            per_page: self.per_page,
            min_width: self.min_width,
            min_height: self.min_height,
            min_duration: self.min_duration,
            max_duration: self.max_duration,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_width() {
        let uri = PopularBuilder::new().min_width(1).build();
        assert_eq!("https://api.pexels.com/videos/popular?min_width=1", uri.create_uri().unwrap());
    }

    #[test]
    fn test_min_height() {
        let uri = PopularBuilder::new().min_height(1).build();
        assert_eq!("https://api.pexels.com/videos/popular?min_height=1", uri.create_uri().unwrap());
    }

    #[test]
    fn test_min_duration() {
        let uri = PopularBuilder::new().min_duration(10).build();
        assert_eq!(
            "https://api.pexels.com/videos/popular?min_duration=10",
            uri.create_uri().unwrap()
        );
    }

    #[test]
    fn test_max_duration() {
        let uri = PopularBuilder::new().max_duration(100).build();
        assert_eq!(
            "https://api.pexels.com/videos/popular?max_duration=100",
            uri.create_uri().unwrap()
        );
    }

    #[test]
    fn test_page() {
        let uri = PopularBuilder::new().page(1).build();
        assert_eq!("https://api.pexels.com/videos/popular?page=1", uri.create_uri().unwrap());
    }

    #[test]
    fn test_per_page() {
        let uri = PopularBuilder::new().per_page(1).build();
        assert_eq!("https://api.pexels.com/videos/popular?per_page=1", uri.create_uri().unwrap());
    }
}
