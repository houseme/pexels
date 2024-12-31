use crate::{
    MediaResponse, MediaSort, MediaType as LibType, Pexels, PexelsError, PEXELS_API,
    PEXELS_COLLECTIONS_PATH, PEXELS_VERSION,
};
use url::Url;

pub struct Media {
    id: String,
    r#type: Option<LibType>,
    sort: Option<MediaSort>,
    page: Option<usize>,
    per_page: Option<usize>,
}

impl Media {
    pub fn builder() -> MediaBuilder {
        MediaBuilder::new()
    }

    pub fn create_uri(&self) -> crate::BuilderResult {
        let uri = format!(
            "{}/{}/{}/{}",
            PEXELS_API, PEXELS_VERSION, PEXELS_COLLECTIONS_PATH, self.id
        );

        let mut url = Url::parse(uri.as_str())?;

        if let Some(r#type) = &self.r#type {
            url.query_pairs_mut().append_pair("type", r#type.as_str());
        }

        if let Some(sort) = &self.sort {
            url.query_pairs_mut().append_pair("sort", sort.as_str());
        }

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

    pub async fn fetch(&self, client: &Pexels) -> Result<MediaResponse, PexelsError> {
        let url = self.create_uri()?;
        let response = client.make_request(url.as_str()).await?;
        let media_response: MediaResponse = serde_json::from_value(response)?;
        Ok(media_response)
    }
}

#[derive(Default)]
pub struct MediaBuilder {
    id: String,
    r#type: Option<LibType>,
    sort: Option<MediaSort>,
    page: Option<usize>,
    per_page: Option<usize>,
}

impl MediaBuilder {
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            r#type: None,
            sort: None,
            page: None,
            per_page: None,
        }
    }

    pub fn r#type(mut self, r#type: LibType) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn sort(mut self, sort: MediaSort) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn page(mut self, page: usize) -> Self {
        self.page = Some(page);
        self
    }

    pub fn per_page(mut self, per_page: usize) -> Self {
        self.per_page = Some(per_page);
        self
    }

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
