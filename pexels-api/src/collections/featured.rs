use crate::{
    CollectionsResponse, Pexels, PexelsError, PEXELS_API, PEXELS_COLLECTIONS_PATH, PEXELS_VERSION,
};
use url::Url;

/// Path to get featured collections.
const PEXELS_FEATURED_PATH: &str = "featured";

/// Represents a request to fetch all featured collections from the Pexels API.
pub struct Featured {
    page: Option<usize>,
    per_page: Option<usize>,
}

impl Featured {
    /// Creates a new `FeaturedBuilder` for constructing a `Featured` request.
    pub fn builder() -> FeaturedBuilder {
        FeaturedBuilder::default()
    }

    /// Constructs the URI for the featured collections request based on the [`FeaturedBuilder`] builder's parameters.
    pub fn create_uri(&self) -> crate::BuilderResult {
        let uri = format!(
            "{PEXELS_API}/{PEXELS_VERSION}/{PEXELS_COLLECTIONS_PATH}/{PEXELS_FEATURED_PATH}"
        );

        let mut url = Url::parse(uri.as_str())?;

        if let Some(page) = &self.page {
            url.query_pairs_mut().append_pair("page", page.to_string().as_str());
        }

        if let Some(per_page) = &self.per_page {
            url.query_pairs_mut().append_pair("per_page", per_page.to_string().as_str());
        }

        Ok(url.into())
    }

    /// Fetches the featured collections data from the Pexels API.
    pub async fn fetch(&self, client: &Pexels) -> Result<CollectionsResponse, PexelsError> {
        let url = self.create_uri()?;
        let response = client.make_request(url.as_str()).await?;
        let collection_response: CollectionsResponse = serde_json::from_value(response)?;
        Ok(collection_response)
    }
}

/// Builder for constructing a `Featured` request.
#[derive(Default)]
pub struct FeaturedBuilder {
    page: Option<usize>,
    per_page: Option<usize>,
}

impl FeaturedBuilder {
    /// Creates a new `FeaturedBuilder`.
    pub fn new() -> Self {
        Self { page: None, per_page: None }
    }

    /// Sets the page number for the featured collections request.
    pub fn page(mut self, page: usize) -> Self {
        self.page = Some(page);
        self
    }

    /// Sets the number of results per page for the featured collections request.
    pub fn per_page(mut self, per_page: usize) -> Self {
        self.per_page = Some(per_page);
        self
    }

    /// Build the `Featured` request from the `FeaturedBuilder` parameters.
    pub fn build(self) -> Featured {
        Featured { page: self.page, per_page: self.per_page }
    }
}
