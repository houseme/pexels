mod collections;
mod domain;
mod photos;
mod videos;

/// collections module
pub use collections::collections::Collections;
pub use collections::collections::CollectionsBuilder;
pub use collections::featured::Featured;
pub use collections::featured::FeaturedBuilder;
pub use collections::media::Media;
pub use collections::media::MediaBuilder;
/// domain module
pub use domain::domain::Collection;
pub use domain::domain::CollectionsResponse;
pub use domain::domain::MediaResponse;
pub use domain::domain::Photo;
pub use domain::domain::PhotoSrc;
pub use domain::domain::PhotosResponse;
pub use domain::domain::User;
pub use domain::domain::Video;
pub use domain::domain::VideoFile;
pub use domain::domain::VideoPicture;
pub use domain::domain::VideoResponse;
/// photos module
pub use photos::curated::Curated;
pub use photos::curated::CuratedBuilder;
pub use photos::photo::FetchPhoto;
pub use photos::photo::FetchPhotoBuilder;
pub use photos::search::Color;
pub use photos::search::Hex;
pub use photos::search::Search;
pub use photos::search::SearchBuilder;
/// videos module
pub use videos::popular::Popular;
pub use videos::popular::PopularBuilder;
pub use videos::search::Search as VideoSearch;
pub use videos::search::SearchBuilder as VideoSearchBuilder;
pub use videos::video::FetchVideo;
pub use videos::video::FetchVideoBuilder;


use reqwest::Client;
use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;
use serde_json::Value;
use thiserror::Error;
use url::ParseError;

/// Pexels API version
const PEXELS_VERSION: &str = "v1";

/// Path for videos
const PEXELS_VIDEO_PATH: &str = "videos";

/// Path for collections
const PEXELS_COLLECTIONS_PATH: &str = "collections";

/// Pexels API URL
const PEXELS_API: &str = "https://api.pexels.com";

/// Desired photo orientation.
pub enum Orientation {
    Landscape,
    Portrait,
    Square,
}

impl Orientation {
    fn as_str(&self) -> &str {
        match self {
            Orientation::Landscape => "landscape",
            Orientation::Portrait => "portrait",
            Orientation::Square => "square",
        }
    }
}

/// the sort for media
pub enum MediaSort {
    Asc,
    Desc,
}

impl MediaSort {
    fn as_str(&self) -> &str {
        match self {
            MediaSort::Asc => "asc",
            MediaSort::Desc => "desc",
        }
    }
}

// The type for media
pub enum MediaType {
    Photo,
    Video,
}

impl MediaType {
    fn as_str(&self) -> &str {
        match self {
            MediaType::Photo => "photos",
            MediaType::Video => "videos",
        }
    }
}

/// The locale of the search you are performing.
#[allow(non_camel_case_types)]
pub enum Locale {
    en_US,
    pt_BR,
    es_ES,
    ca_ES,
    de_DE,
    it_IT,
    fr_FR,
    sv_SE,
    id_ID,
    pl_PL,
    ja_JP,
    zh_TW,
    zh_CN,
    ko_KR,
    th_TH,
    nl_NL,
    hu_HU,
    vi_VN,
    cs_CZ,
    da_DK,
    fi_FI,
    uk_UA,
    el_GR,
    ro_RO,
    nb_NO,
    sk_SK,
    tr_TR,
    ru_RU,
}

impl Locale {
    fn as_str(&self) -> &str {
        match self {
            Locale::en_US => "en-US",
            Locale::pt_BR => "pt-BR",
            Locale::es_ES => "es-ES",
            Locale::ca_ES => "ca-ES",
            Locale::de_DE => "de-DE",
            Locale::it_IT => "it-IT",
            Locale::fr_FR => "fr-FR",
            Locale::sv_SE => "sv-SE",
            Locale::id_ID => "id-ID",
            Locale::pl_PL => "pl-PL",
            Locale::ja_JP => "ja-JP",
            Locale::zh_TW => "zh-TW",
            Locale::zh_CN => "zh-CN",
            Locale::ko_KR => "ko-KR",
            Locale::th_TH => "th-TH",
            Locale::nl_NL => "nl-NL",
            Locale::hu_HU => "hu-HU",
            Locale::vi_VN => "vi-VN",
            Locale::cs_CZ => "cs-CZ",
            Locale::da_DK => "da-DK",
            Locale::fi_FI => "fi-FI",
            Locale::uk_UA => "uk-UA",
            Locale::el_GR => "el-GR",
            Locale::ro_RO => "ro-RO",
            Locale::nb_NO => "nb-NO",
            Locale::sk_SK => "sk-SK",
            Locale::tr_TR => "tr-TR",
            Locale::ru_RU => "-ES",
        }
    }
}

/// Minimum videos/photo size.
pub enum Size {
    Large,
    Medium,
    Small,
}

impl Size {
    fn as_str(&self) -> &str {
        match self {
            Size::Large => "large",
            Size::Medium => "medium",
            Size::Small => "small",
        }
    }
}

/// Builder result type
pub(crate) type BuilderResult = Result<String, PexelsError>;

/// Errors that can occur while interacting with the Pexels API.
#[derive(Debug, Error)]
pub enum PexelsError {
    #[error("Failed to send HTTP request: {0}")]
    RequestError(#[from] ReqwestError),
    #[error("Failed to parse JSON response: {0}")]
    JsonParseError(#[from] JsonError),
    #[error("API key not found in environment variables")]
    ApiKeyNotFound,
    #[error("Failed to parse URL: {0}")]
    ParseError(#[from] ParseError),
    #[error("Invalid hex color code: {0}")]
    HexColorCodeError(String),
}

// Manual implementation PartialEq
impl PartialEq for PexelsError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Compare RequestError
            (PexelsError::RequestError(e1), PexelsError::RequestError(e2)) => {
                e1.to_string() == e2.to_string()
            }
            // Compare JsonParseError
            (PexelsError::JsonParseError(e1), PexelsError::JsonParseError(e2)) => {
                e1.to_string() == e2.to_string()
            }
            // Compare ApiKeyNotFound
            (PexelsError::ApiKeyNotFound, PexelsError::ApiKeyNotFound) => true,
            // Compare ParseError
            (PexelsError::ParseError(e1), PexelsError::ParseError(e2)) => {
                e1.to_string() == e2.to_string()
            }
            // Compare HexColorCodeError
            (PexelsError::HexColorCodeError(msg1), PexelsError::HexColorCodeError(msg2)) => {
                msg1 == msg2
            }
            // Other things are not equal
            _ => false,
        }
    }
}

/// Client for interacting with the Pexels API
pub struct Pexels {
    client: Client,
    api_key: String,
}

impl Pexels {
    pub fn new(api_key: String) -> Self {
        Pexels {
            client: Client::new(),
            api_key,
        }
    }

    /// Unified HTTP request method
    /// # Errors    
    /// If the request fails, an error is returned.  
    async fn make_request(&self, url: &str) -> Result<Value, PexelsError> {
        let json_response = self
            .client
            .get(url)
            .header("Authorization", &self.api_key)
            .send()
            .await?
            .json::<Value>()
            .await?;
        Ok(json_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pexels_error_partial_eq() {
        let err1 = PexelsError::ApiKeyNotFound;
        let err2 = PexelsError::ApiKeyNotFound;
        assert_eq!(err1, err2);

        let err3 = PexelsError::HexColorCodeError(String::from("Invalid color"));
        let err4 = PexelsError::HexColorCodeError(String::from("Invalid color"));
        assert_eq!(err3, err4);

        let err9 = PexelsError::ParseError(ParseError::EmptyHost);
        let err10 = PexelsError::ParseError(ParseError::EmptyHost);
        assert_eq!(err9, err10);

        // 测试不相等的情况
        let err11 = PexelsError::ApiKeyNotFound;
        let err12 = PexelsError::HexColorCodeError(String::from("Invalid color"));
        assert_ne!(err11, err12);
    }
}
