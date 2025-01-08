use crate::{Pexels, PexelsError, Video, PEXELS_API, PEXELS_VIDEO_PATH};
use url::Url;
/// Path to get a specific video.
const PEXELS_GET_VIDEO_PATH: &str = "videos";

/// Represents a request to fetch a specific video by its ID from the Pexels API.
pub struct FetchVideo {
    id: usize,
}

impl FetchVideo {
    /// Creates a new `FetchVideoBuilder` for building URIs.
    pub fn builder() -> FetchVideoBuilder {
        FetchVideoBuilder::default()
    }

    /// Creates a URI from the provided values.
    pub fn create_uri(&self) -> crate::BuilderResult {
        let uri =
            format!("{}/{}/{}/{}", PEXELS_API, PEXELS_VIDEO_PATH, PEXELS_GET_VIDEO_PATH, self.id);

        let url = Url::parse(uri.as_str())?;

        Ok(url.into())
    }

    /// Fetches the video data from the Pexels API.
    pub async fn fetch(&self, client: &Pexels) -> Result<Video, PexelsError> {
        let url = self.create_uri()?;
        let response = client.make_request(url.as_str()).await?;
        let video: Video = serde_json::from_value(response)?;
        Ok(video)
    }
}

/// Builder for `FetchVideo`.
#[derive(Default)]
pub struct FetchVideoBuilder {
    id: usize,
}

impl FetchVideoBuilder {
    /// Creates a new `FetchVideoBuilder`.
    pub fn new() -> Self {
        Self { id: 0 }
    }

    /// Sets the ID of the video to be fetched.
    pub fn id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    /// Builds a `FetchVideo` instance from the `FetchVideoBuilder`.
    pub fn build(self) -> FetchVideo {
        FetchVideo { id: self.id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        let uri = FetchVideoBuilder::new().id(123).build();
        assert_eq!("https://api.pexels.com/videos/videos/123", uri.create_uri().unwrap());
    }
}
