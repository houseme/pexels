use crate::{
    CollectionsResponse, Pexels, PexelsError, PEXELS_API, PEXELS_COLLECTIONS_PATH, PEXELS_VERSION,
};
use url::Url;

/// Represents a request to fetch a list of collections from the Pexels API.
pub struct Collections {
    page: Option<usize>,
    per_page: Option<usize>,
}

impl Collections {
    /// Creates a new `CollectionsBuilder` for constructing a `Collections` request.
    pub fn builder() -> CollectionsBuilder {
        CollectionsBuilder::default()
    }

    /// Constructs the URI for the collections request based on the builder's parameters.
    pub fn create_uri(&self) -> crate::BuilderResult {
        let uri = format!("{PEXELS_API}/{PEXELS_VERSION}/{PEXELS_COLLECTIONS_PATH}");

        let mut url = Url::parse(uri.as_str())?;

        if let Some(page) = &self.page {
            url.query_pairs_mut().append_pair("page", page.to_string().as_str());
        }

        if let Some(per_page) = &self.per_page {
            url.query_pairs_mut().append_pair("per_page", per_page.to_string().as_str());
        }

        Ok(url.into())
    }

    /// Fetches the collections data from the Pexels API.
    pub async fn fetch(&self, client: &Pexels) -> Result<CollectionsResponse, PexelsError> {
        let url = self.create_uri()?;
        let response = client.make_request(url.as_str()).await?;
        let collections_response: CollectionsResponse = serde_json::from_value(response)?;
        Ok(collections_response)
    }
}

/// Builder for constructing a `Collections` request.
#[derive(Default)]
pub struct CollectionsBuilder {
    page: Option<usize>,
    per_page: Option<usize>,
}

impl CollectionsBuilder {
    /// Creates a new `CollectionsBuilder`.
    pub fn new() -> Self {
        Self { page: None, per_page: None }
    }

    /// Sets the page number for the collection request.
    pub fn page(mut self, page: usize) -> Self {
        self.page = Some(page);
        self
    }

    /// Sets the number of results per page for the collection request.
    pub fn per_page(mut self, per_page: usize) -> Self {
        self.per_page = Some(per_page);
        self
    }

    /// Builds the `Collections` request from the `CollectionsBuilder` parameters
    pub fn build(self) -> Collections {
        Collections { page: self.page, per_page: self.per_page }
    }
}
