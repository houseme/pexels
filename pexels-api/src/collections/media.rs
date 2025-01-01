use crate::{
    MediaResponse, MediaSort, MediaType as LibType, Pexels, PexelsError, PEXELS_API,
    PEXELS_COLLECTIONS_PATH, PEXELS_VERSION,
};
use url::Url;

/// Represents a request to fetch a specific media item by its ID from the Pexels API.
/// This endpoint returns all media items (photos and videos) within a single collection.
/// Use the `type` parameter to filter results to only photos or only videos.
pub struct Media {
    id: String,
    r#type: Option<LibType>,
    sort: Option<MediaSort>,
    page: Option<usize>,
    per_page: Option<usize>,
}

impl Media {
    /// Creates a new `MediaBuilder` for constructing a `Media` request.
    pub fn builder() -> MediaBuilder {
        MediaBuilder::new()
    }

    /// Constructs the URI for the media request based on the builder's parameters.
    pub fn create_uri(&self) -> crate::BuilderResult {
        let uri =
            format!("{}/{}/{}/{}", PEXELS_API, PEXELS_VERSION, PEXELS_COLLECTIONS_PATH, self.id);

        let mut url = Url::parse(uri.as_str())?;

        if let Some(r#type) = &self.r#type {
            match r#type {
                LibType::Empty => {}
                _ => {
                    url.query_pairs_mut().append_pair("type", r#type.as_str());
                }
            }
        }

        if let Some(sort) = &self.sort {
            url.query_pairs_mut().append_pair("sort", sort.as_str());
        }

        if let Some(page) = &self.page {
            url.query_pairs_mut().append_pair("page", page.to_string().as_str());
        }

        if let Some(per_page) = &self.per_page {
            url.query_pairs_mut().append_pair("per_page", per_page.to_string().as_str());
        }

        Ok(url.into())
    }

    /// Fetches the media data from the Pexels API.
    pub async fn fetch(&self, client: &Pexels) -> Result<MediaResponse, PexelsError> {
        let url = self.create_uri()?;
        let response = client.make_request(url.as_str()).await?;
        let media_response: MediaResponse = serde_json::from_value(response)?;
        Ok(media_response)
    }
}

/// Builder for constructing a `Media` request.
#[derive(Default)]
pub struct MediaBuilder {
    id: String,
    r#type: Option<LibType>,
    sort: Option<MediaSort>,
    page: Option<usize>,
    per_page: Option<usize>,
}

impl MediaBuilder {
    /// Creates a new `MediaBuilder`.
    pub fn new() -> Self {
        Self { id: "".to_string(), r#type: None, sort: None, page: None, per_page: None }
    }

    /// Sets the ID of the media item to be fetched.
    pub fn id(mut self, id: String) -> Self {
        self.id = id;
        self
    }

    /// Sets the type of media to be fetched (photo or video).
    pub fn r#type(mut self, r#type: LibType) -> Self {
        self.r#type = Some(r#type);
        self
    }

    /// Sets the sorting order of the media items.
    pub fn sort(mut self, sort: MediaSort) -> Self {
        self.sort = Some(sort);
        self
    }

    /// Sets the page number for the request.
    pub fn page(mut self, page: usize) -> Self {
        self.page = Some(page);
        self
    }

    /// Sets the number of results per page for the request.
    pub fn per_page(mut self, per_page: usize) -> Self {
        self.per_page = Some(per_page);
        self
    }

    /// Builds a `Media` instance from the `MediaBuilder`.
    pub fn build(self) -> Media {
        Media {
            id: self.id,
            r#type: self.r#type,
            sort: self.sort,
            page: self.page,
            per_page: self.per_page,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        let uri = MediaBuilder::new().id("123".to_string()).build();
        assert_eq!("https://api.pexels.com/v1/collections/123", uri.create_uri().unwrap());
    }
}
