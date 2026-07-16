use crate::{MediaSort, MediaType, Orientation, Size};

#[derive(Debug, Clone, Default)]
pub struct SearchParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub orientation: Option<Orientation>,
    pub size: Option<Size>,
    pub color: Option<String>,
    pub locale: Option<String>,
}

impl SearchParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn per_page(mut self, per_page: u32) -> Self {
        self.per_page = Some(per_page);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }

    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }

    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(page) = self.page {
            params.push(("page".to_string(), page.to_string()));
        }

        if let Some(per_page) = self.per_page {
            params.push(("per_page".to_string(), per_page.to_string()));
        }

        if let Some(orientation) = &self.orientation {
            params.push(("orientation".to_string(), orientation.to_string()));
        }

        if let Some(size) = &self.size {
            params.push(("size".to_string(), size.to_string()));
        }

        if let Some(color) = &self.color {
            params.push(("color".to_string(), color.clone()));
        }

        if let Some(locale) = &self.locale {
            params.push(("locale".to_string(), locale.clone()));
        }

        params
    }
}

// Pagination parameters for API requests
#[derive(Debug, Clone, Default)]
pub struct PaginationParams {
    /// Page number to retrieve
    pub page: Option<u32>,

    /// Number of items per page
    pub per_page: Option<u32>,
}

impl PaginationParams {
    /// Create a new PaginationParams instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the page number
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Set the number of items per page
    pub fn per_page(mut self, per_page: u32) -> Self {
        self.per_page = Some(per_page);
        self
    }

    pub(crate) fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(page) = self.page {
            params.push(("page".to_string(), page.to_string()));
        }

        if let Some(per_page) = self.per_page {
            params.push(("per_page".to_string(), per_page.to_string()));
        }

        params
    }
}

/// Video search parameters
#[derive(Debug, Clone, Default)]
pub struct VideoSearchParams {
    /// Page number to retrieve
    pub page: Option<u32>,

    /// Number of videos per page
    pub per_page: Option<u32>,

    /// Orientation filter (landscape, portrait, square)
    pub orientation: Option<String>,

    /// Size filter (large, medium, small)
    pub size: Option<String>,

    /// Locale for localized results
    pub locale: Option<String>,
}

impl VideoSearchParams {
    /// Create a new VideoSearchParams instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the page number
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Set the number of items per page
    pub fn per_page(mut self, per_page: u32) -> Self {
        self.per_page = Some(per_page);
        self
    }

    /// Set the orientation filter
    pub fn orientation(mut self, orientation: impl Into<String>) -> Self {
        self.orientation = Some(orientation.into());
        self
    }

    /// Set the size filter
    pub fn size(mut self, size: impl Into<String>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// Set the locale for localized results
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }

    pub(crate) fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(page) = self.page {
            params.push(("page".to_string(), page.to_string()));
        }

        if let Some(per_page) = self.per_page {
            params.push(("per_page".to_string(), per_page.to_string()));
        }

        if let Some(orientation) = &self.orientation {
            params.push(("orientation".to_string(), orientation.clone()));
        }

        if let Some(size) = &self.size {
            params.push(("size".to_string(), size.clone()));
        }

        if let Some(locale) = &self.locale {
            params.push(("locale".to_string(), locale.clone()));
        }

        params
    }
}

/// Popular video parameters.
#[derive(Debug, Clone, Default)]
pub struct PopularVideoParams {
    /// Page number to retrieve
    pub page: Option<u32>,

    /// Number of videos per page
    pub per_page: Option<u32>,

    /// Minimum video width in pixels
    pub min_width: Option<u32>,

    /// Minimum video height in pixels
    pub min_height: Option<u32>,

    /// Minimum video duration in seconds
    pub min_duration: Option<u32>,

    /// Maximum video duration in seconds
    pub max_duration: Option<u32>,
}

impl PopularVideoParams {
    /// Create a new PopularVideoParams instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Create popular video params from simple pagination params.
    pub fn from_pagination(params: &PaginationParams) -> Self {
        Self { page: params.page, per_page: params.per_page, ..Self::default() }
    }

    /// Set the page number
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Set the number of items per page
    pub fn per_page(mut self, per_page: u32) -> Self {
        self.per_page = Some(per_page);
        self
    }

    /// Set the minimum video width in pixels
    pub fn min_width(mut self, min_width: u32) -> Self {
        self.min_width = Some(min_width);
        self
    }

    /// Set the minimum video height in pixels
    pub fn min_height(mut self, min_height: u32) -> Self {
        self.min_height = Some(min_height);
        self
    }

    /// Set the minimum video duration in seconds
    pub fn min_duration(mut self, min_duration: u32) -> Self {
        self.min_duration = Some(min_duration);
        self
    }

    /// Set the maximum video duration in seconds
    pub fn max_duration(mut self, max_duration: u32) -> Self {
        self.max_duration = Some(max_duration);
        self
    }

    pub(crate) fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(page) = self.page {
            params.push(("page".to_string(), page.to_string()));
        }

        if let Some(per_page) = self.per_page {
            params.push(("per_page".to_string(), per_page.to_string()));
        }

        if let Some(min_width) = self.min_width {
            params.push(("min_width".to_string(), min_width.to_string()));
        }

        if let Some(min_height) = self.min_height {
            params.push(("min_height".to_string(), min_height.to_string()));
        }

        if let Some(min_duration) = self.min_duration {
            params.push(("min_duration".to_string(), min_duration.to_string()));
        }

        if let Some(max_duration) = self.max_duration {
            params.push(("max_duration".to_string(), max_duration.to_string()));
        }

        params
    }
}

/// Collection media parameters.
#[derive(Debug, Clone, Default)]
pub struct CollectionMediaParams {
    /// Page number to retrieve
    pub page: Option<u32>,

    /// Number of media items per page
    pub per_page: Option<u32>,

    /// Media type filter
    pub media_type: Option<MediaType>,

    /// Sort order
    pub sort: Option<MediaSort>,
}

impl CollectionMediaParams {
    /// Create a new CollectionMediaParams instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Create collection media params from simple pagination params.
    pub fn from_pagination(params: &PaginationParams) -> Self {
        Self { page: params.page, per_page: params.per_page, ..Self::default() }
    }

    /// Set the page number
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Set the number of items per page
    pub fn per_page(mut self, per_page: u32) -> Self {
        self.per_page = Some(per_page);
        self
    }

    /// Filter collection media to photos or videos.
    pub fn media_type(mut self, media_type: MediaType) -> Self {
        self.media_type = Some(media_type);
        self
    }

    /// Set collection media sort order.
    pub fn sort(mut self, sort: MediaSort) -> Self {
        self.sort = Some(sort);
        self
    }

    pub(crate) fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(page) = self.page {
            params.push(("page".to_string(), page.to_string()));
        }

        if let Some(per_page) = self.per_page {
            params.push(("per_page".to_string(), per_page.to_string()));
        }

        if let Some(media_type) = &self.media_type {
            if !matches!(media_type, MediaType::Empty) {
                params.push(("type".to_string(), media_type.as_str().to_string()));
            }
        }

        if let Some(sort) = &self.sort {
            params.push(("sort".to_string(), sort.as_str().to_string()));
        }

        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn popular_video_params_include_documented_filters() {
        let params = PopularVideoParams::new()
            .page(2)
            .per_page(20)
            .min_width(1280)
            .min_height(720)
            .min_duration(5)
            .max_duration(60)
            .to_query_params();

        assert_eq!(
            params,
            vec![
                ("page".to_string(), "2".to_string()),
                ("per_page".to_string(), "20".to_string()),
                ("min_width".to_string(), "1280".to_string()),
                ("min_height".to_string(), "720".to_string()),
                ("min_duration".to_string(), "5".to_string()),
                ("max_duration".to_string(), "60".to_string()),
            ]
        );
    }

    #[test]
    fn collection_media_params_include_type_and_sort() {
        let params = CollectionMediaParams::new()
            .page(1)
            .per_page(10)
            .media_type(MediaType::Photo)
            .sort(MediaSort::Desc)
            .to_query_params();

        assert_eq!(
            params,
            vec![
                ("page".to_string(), "1".to_string()),
                ("per_page".to_string(), "10".to_string()),
                ("type".to_string(), "photos".to_string()),
                ("sort".to_string(), "desc".to_string()),
            ]
        );
    }
}
