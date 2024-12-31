use crate::{
    CollectionsResponse, Pexels, PexelsError, PEXELS_API, PEXELS_COLLECTIONS_PATH, PEXELS_VERSION,
};
use url::Url;

/// Collection list
pub struct Collections {
    page: Option<usize>,
    per_page: Option<usize>,
}

impl Collections {
    /// Creates [`CollectionsBuilder`] for building URI's.
    pub fn builder() -> CollectionsBuilder {
        CollectionsBuilder::default()
    }

    /// Create URI from inputted vales from the [`CollectionsBuilder`].
    pub fn create_uri(&self) -> crate::BuilderResult {
        let uri = format!(
            "{}/{}/{}",
            PEXELS_API, PEXELS_VERSION, PEXELS_COLLECTIONS_PATH
        );

        let mut url = Url::parse(uri.as_str())?;

        if let Some(page) = &self.page {
            url.query_pairs_mut()
                .append_pair("page", page.to_string().as_str());
        }

        if let Some(per_page) = &self.per_page {
            url.query_pairs_mut()
                .append_pair("per_page", per_page.to_string().as_str());
        }

        Ok(url.into())
    }

    /// Fetch the photo data from the Pexels API.
    pub async fn fetch(&self, client: &Pexels) -> Result<CollectionsResponse, PexelsError> {
        let url = self.create_uri()?;
        let response = client.make_request(url.as_str()).await?;
        let collections_response: CollectionsResponse = serde_json::from_value(response)?;
        Ok(collections_response)
    }
}

/// Builder for [`Collections`].
#[derive(Default)]
pub struct CollectionsBuilder {
    page: Option<usize>,
    per_page: Option<usize>,
}

impl CollectionsBuilder {
    pub fn new() -> Self {
        Self {
            page: None,
            per_page: None,
        }
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

    /// Create [`Collections`] from the [`CollectionsBuilder`]
    pub fn build(self) -> Collections {
        Collections {
            page: self.page,
            per_page: self.per_page,
        }
    }
}
