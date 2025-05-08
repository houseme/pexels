use crate::{Orientation, Size};

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
}
