use crate::{Pexels, PexelsError, Photo, PEXELS_API, PEXELS_VERSION};
use url::Url;

/// Path to get a specific photo.
const PEXELS_GET_PHOTO_PATH: &str = "photos";

/// Retrieve a specific Photo from its id.
pub struct FetchPhoto {
    id: usize,
}

impl FetchPhoto {
    /// Creates [`FetchPhotoBuilder`] for building URI's.
    pub fn builder() -> FetchPhotoBuilder {
        FetchPhotoBuilder::default()
    }

    /// Creates a URI from the values provided by the [`FetchPhotoBuilder`].
    pub fn create_uri(&self) -> crate::BuilderResult {
        let uri =
            format!("{}/{}/{}/{}", PEXELS_API, PEXELS_VERSION, PEXELS_GET_PHOTO_PATH, self.id);

        let url = Url::parse(uri.as_str())?;

        Ok(url.into())
    }

    /// Fetches the photo data from the Pexels API using the provided client.
    pub async fn fetch(&self, client: &Pexels) -> Result<Photo, PexelsError> {
        let url = self.create_uri()?;
        let response = client.make_request(url.as_str()).await?;
        let photo: Photo = serde_json::from_value(response)?;
        Ok(photo)
    }
}

/// Builder for [`FetchPhoto`].
#[derive(Default)]
pub struct FetchPhotoBuilder {
    id: usize,
}

impl FetchPhotoBuilder {
    /// Create a new [`FetchPhotoBuilder`].
    pub fn new() -> Self {
        Self { id: 0 }
    }

    /// Sets the ID of the photo to be requested.
    pub fn id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    /// Create [`FetchPhoto`] from the [`FetchPhotoBuilder`]
    pub fn build(self) -> FetchPhoto {
        FetchPhoto { id: self.id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;
    use std::env;
    

    #[test]
    fn test_id() {
        let uri = FetchPhotoBuilder::new().id(123).build();
        assert_eq!("https://api.pexels.com/v1/photos/123", uri.create_uri().unwrap());
    }

    #[tokio::test]
    async fn test_fetch_photo() {
        dotenv().ok();
        let api_key = env::var("PEXELS_API_KEY").expect("PEXELS_API_KEY not set");
        let client = Pexels::new(api_key);

        let get_photo = FetchPhoto::builder().id(10967).build();
        let result = get_photo.fetch(&client).await;
        println!("get_photo result: {result:?}");
        assert!(result.is_ok());
    }
}
