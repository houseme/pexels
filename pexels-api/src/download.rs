use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use futures::stream::StreamExt;
use reqwest::header::HeaderMap;
use reqwest::{header, Client};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::sync::Semaphore;

use crate::models::{Photo, Video};
use crate::PexelsError;

/// Picture quality enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageQuality {
    Original,
    Large2x,
    Large,
    Medium,
    Small,
    Portrait,
    Landscape,
    Tiny,
}

/// Video quality enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoQuality {
    HD,
    SD,
    Tiny,
}

/// The type of progress callback function
pub type ProgressCallback = fn(current: u64, total: u64);

/// Result type alias
type Result<T> = std::result::Result<T, PexelsError>;

pub struct DownloadManager {
    client: Client,
    max_concurrent: usize,
}

impl DownloadManager {
    /// Create a new 'DownloadManager' and specify the maximum number of concurrent downloads
    /// The default timeout is set to 60 seconds
    ///
    /// # Arguments
    /// * `max_concurrent` - Maximum number of concurrent downloads
    pub fn new(max_concurrent: usize) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .pool_max_idle_per_host(20)
            .build()
            .unwrap_or_default();

        Self { client, max_concurrent }
    }

    /// Create a 'DownloadManager' with a custom 'Client'
    pub fn with_client(client: Client, max_concurrent: usize) -> Self {
        Self { client, max_concurrent }
    }

    /// Download the photos from the given URL and save to the specified output directory
    /// Resumable upload is supported
    ///
    /// # Arguments
    /// * `photo` - Photos to download
    /// * `output_dir` - Output directory
    /// * `quality` - Download quality
    ///
    /// # Returns
    /// The path to download the file
    pub async fn download_photo<P: AsRef<Path>>(
        &self,
        photo: &Photo,
        output_dir: P,
        quality: ImageQuality,
    ) -> Result<PathBuf> {
        let url = self.get_photo_url(photo, quality);
        let file_name = format!("photo_{}.jpg", photo.id);
        self.download_file(&url, output_dir, &file_name).await
    }

    /// Download the video from the given URL and save to the specified output directory
    /// Resumable upload is supported
    ///
    /// # Arguments
    /// * `video` - Video to download
    /// * `output_dir` - Output directory
    /// * `quality` - Download quality
    ///
    /// # Returns
    /// The path to download the file
    pub async fn download_video<P: AsRef<Path>>(
        &self,
        video: &Video,
        output_dir: P,
        quality: VideoQuality,
    ) -> Result<PathBuf> {
        let url = self.get_video_url(video, quality);
        let file_name = format!("video_{}.mp4", video.id);
        self.download_file(&url, output_dir, &file_name).await
    }

    /// Download photos in batches
    ///
    /// # Arguments
    /// * `photos` - A list of photos to download
    /// * `output_dir` - Output directory
    /// * `quality` - Download quality
    /// * `progress_callback` - Optional progress callback function
    ///
    /// # Returns
    /// A list of files that have been successfully downloaded
    pub async fn batch_download_photos<P: AsRef<Path>>(
        &self,
        photos: &[Photo],
        output_dir: P,
        quality: ImageQuality,
        progress_callback: Option<ProgressCallback>,
    ) -> Result<Vec<PathBuf>> {
        let output_dir = output_dir.as_ref().to_path_buf();
        let semaphore = Arc::new(Semaphore::new(self.max_concurrent));

        let mut handles = Vec::with_capacity(photos.len());

        for photo in photos {
            let permit = Arc::clone(&semaphore).acquire_owned();
            let photo = photo.clone();
            let dir = output_dir.clone();
            let client = self.client.clone();
            let callback = progress_callback;

            let handle = tokio::spawn(async move {
                let _permit = permit.await.map_err(|_| PexelsError::AsyncError)?;

                let url = match quality {
                    ImageQuality::Original => &photo.src.original,
                    ImageQuality::Large2x => &photo.src.large2x,
                    ImageQuality::Large => &photo.src.large,
                    ImageQuality::Medium => &photo.src.medium,
                    ImageQuality::Small => &photo.src.small,
                    ImageQuality::Portrait => &photo.src.portrait,
                    ImageQuality::Landscape => &photo.src.landscape,
                    ImageQuality::Tiny => &photo.src.tiny,
                };

                let file_name = format!("photo_{}.jpg", photo.id);
                let path = dir.join(&file_name);

                // Make sure the directory exists
                if !dir.exists() {
                    fs::create_dir_all(&dir).await?;
                }

                // Resumable upload logic
                let mut headers = HeaderMap::new();
                let mut range_start = 0;

                if path.exists() {
                    if let Ok(metadata) = fs::metadata(&path).await {
                        range_start = metadata.len();
                        headers.insert(
                            header::RANGE,
                            format!("bytes={range_start}-").parse().unwrap(),
                        );
                    }
                }

                // Download the file
                let response = client.get(url).headers(headers).send().await?;

                if !response.status().is_success() {
                    return Err(PexelsError::DownloadError(format!(
                        "Failed to download file: {}",
                        response.status()
                    )));
                }

                // Get the file size
                let total_size = response.content_length().unwrap_or(0) + range_start;

                let mut file = if range_start > 0 {
                    fs::OpenOptions::new().append(true).open(&path).await?
                } else {
                    fs::File::create(&path).await?
                };

                let mut stream = response.bytes_stream();
                let mut downloaded = range_start;

                while let Some(chunk) = stream.next().await {
                    let chunk = chunk?;
                    file.write_all(&chunk).await?;

                    downloaded += chunk.len() as u64;

                    // Call progress callback (if provided)
                    if let Some(cb) = callback {
                        cb(downloaded, total_size);
                    }
                }

                Ok::<PathBuf, PexelsError>(path)
            });

            handles.push(handle);
        }

        // wait for all downloads to complete
        let results = futures::future::join_all(handles).await;

        // Process the results
        let mut successful_downloads = Vec::new();
        for result in results {
            match result {
                Ok(Ok(path)) => successful_downloads.push(path),
                Ok(Err(e)) => eprintln!("Download error: {e}"),
                Err(e) => eprintln!("Task join error: {e}"),
            }
        }

        Ok(successful_downloads)
    }

    /// Download videos in batches
    ///
    /// # Arguments
    /// * `videos` - A list of videos to download
    /// * `output_dir` - Output directory
    /// * `quality` - Download quality
    /// * `progress_callback` - Optional progress callback function
    ///
    /// # Returns
    /// A list of files that have been successfully downloaded
    pub async fn batch_download_videos<P: AsRef<Path>>(
        &self,
        videos: &[Video],
        output_dir: P,
        quality: VideoQuality,
        progress_callback: Option<ProgressCallback>,
    ) -> Result<Vec<PathBuf>> {
        let output_dir = output_dir.as_ref().to_path_buf();
        let semaphore = Arc::new(Semaphore::new(self.max_concurrent));

        let mut handles = Vec::with_capacity(videos.len());

        for video in videos {
            let permit = Arc::clone(&semaphore).acquire_owned();
            let video = video.clone();
            let dir = output_dir.clone();
            let client = self.client.clone();
            let callback = progress_callback;

            let handle = tokio::spawn(async move {
                let _permit = permit.await.map_err(|_| PexelsError::AsyncError)?;

                // 获取对应质量的视频 URL
                let video_file = video
                    .video_files
                    .iter()
                    .find(|file| match quality {
                        VideoQuality::HD => file.quality == "hd" || file.quality == "HD",
                        VideoQuality::SD => file.quality == "sd",
                        VideoQuality::Tiny => {
                            file.file_type == "video/mp4"
                                && (file.width.unwrap_or(0) <= 640
                                    || file.height.unwrap_or(0) <= 360)
                        }
                    })
                    .ok_or_else(|| {
                        PexelsError::DownloadError("No suitable video file found".to_string())
                    })?;

                let url = &video_file.link;
                let file_name = format!("video_{}.mp4", video.id);
                let path = dir.join(&file_name);

                // Make sure the directory exists
                if !dir.exists() {
                    fs::create_dir_all(&dir).await?;
                }

                // Resumable upload logic
                let mut headers = HeaderMap::new();
                let mut range_start = 0;

                if path.exists() {
                    if let Ok(metadata) = fs::metadata(&path).await {
                        range_start = metadata.len();
                        headers.insert(
                            header::RANGE,
                            format!("bytes={range_start}-").parse().unwrap(),
                        );
                    }
                }

                // Download the file
                let response = client.get(url).headers(headers).send().await?;

                if !response.status().is_success() {
                    return Err(PexelsError::DownloadError(format!(
                        "Failed to download file: {}",
                        response.status()
                    )));
                }

                // Get the file size
                let total_size = response.content_length().unwrap_or(0) + range_start;

                let mut file = if range_start > 0 {
                    fs::OpenOptions::new().append(true).open(&path).await?
                } else {
                    fs::File::create(&path).await?
                };

                let mut stream = response.bytes_stream();
                let mut downloaded = range_start;

                while let Some(chunk) = stream.next().await {
                    let chunk = chunk?;
                    file.write_all(&chunk).await?;

                    downloaded += chunk.len() as u64;

                    // Call progress callback (if provided)
                    if let Some(cb) = callback {
                        cb(downloaded, total_size);
                    }
                }

                Ok::<PathBuf, PexelsError>(path)
            });

            handles.push(handle);
        }

        // Wait for all downloads to complete
        let results = futures::future::join_all(handles).await;

        // Process the results
        let mut successful_downloads = Vec::new();
        for result in results {
            match result {
                Ok(Ok(path)) => successful_downloads.push(path),
                Ok(Err(e)) => eprintln!("Download error: {e}"),
                Err(e) => eprintln!("Task join error: {e}"),
            }
        }

        Ok(successful_downloads)
    }

    /// Download a single file
    ///
    /// # Arguments
    /// * `url` - File URL
    /// * `output_dir` - Output directory
    /// * `file_name` - Filename
    ///
    /// # Returns
    /// The path to download the file
    async fn download_file<P: AsRef<Path>>(
        &self,
        url: &str,
        output_dir: P,
        file_name: &str,
    ) -> Result<PathBuf> {
        let output_dir = output_dir.as_ref().to_path_buf();
        let path = output_dir.join(file_name);

        // Make sure the directory exists
        if !output_dir.exists() {
            fs::create_dir_all(&output_dir).await?;
        }

        // Resumable upload logic
        let mut headers = HeaderMap::new();
        let mut range_start = 0;

        if path.exists() {
            if let Ok(metadata) = fs::metadata(&path).await {
                range_start = metadata.len();
                headers.insert(header::RANGE, format!("bytes={range_start}-").parse().unwrap());
            }
        }

        // Send a request
        let response = self.client.get(url).headers(headers).send().await?;

        if !response.status().is_success() {
            return Err(PexelsError::DownloadError(format!(
                "Failed to download file: {}",
                response.status()
            )));
        }

        // Get the file size
        let _total_size = response.content_length().unwrap_or(0) + range_start;

        let mut file = if range_start > 0 {
            fs::OpenOptions::new().append(true).open(&path).await?
        } else {
            fs::File::create(&path).await?
        };

        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;
        }

        Ok(path)
    }

    /// Get the photo URL
    fn get_photo_url(&self, photo: &Photo, quality: ImageQuality) -> String {
        match quality {
            ImageQuality::Original => photo.src.original.clone(),
            ImageQuality::Large2x => photo.src.large2x.clone(),
            ImageQuality::Large => photo.src.large.clone(),
            ImageQuality::Medium => photo.src.medium.clone(),
            ImageQuality::Small => photo.src.small.clone(),
            ImageQuality::Portrait => photo.src.portrait.clone(),
            ImageQuality::Landscape => photo.src.landscape.clone(),
            ImageQuality::Tiny => photo.src.tiny.clone(),
        }
    }

    /// Get the video URL
    fn get_video_url(&self, video: &Video, quality: VideoQuality) -> String {
        let video_file = video
            .video_files
            .iter()
            .find(|file| match quality {
                VideoQuality::HD => file.quality == "hd" || file.quality == "HD",
                VideoQuality::SD => file.quality == "sd",
                VideoQuality::Tiny => {
                    file.file_type == "video/mp4"
                        && (file.width.unwrap_or(0) <= 640 || file.height.unwrap_or(0) <= 360)
                }
            })
            .unwrap_or_else(|| {
                // If you can't find the specified quality, return the first video file
                video.video_files.first().unwrap_or_else(|| {
                    panic!("No video files available for video ID: {}", video.id)
                })
            });

        video_file.link.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::PhotoSources;
    use tokio::test;

    // Simulate the Photo data structure
    fn mock_photo() -> Photo {
        Photo {
            id: 1,
            width: 800,
            height: 600,
            url: "https://www.pexels.com/photo/1".to_string(),
            photographer: "Test Photographer".to_string(),
            photographer_url: Some("https://www.pexels.com/photographer".to_string()),
            photographer_id: Some(1),
            avg_color: Some("#FFFFFF".to_string()),
            src: PhotoSources {
                original: "https://images.pexels.com/photos/1/original.jpg".to_string(),
                large2x: "https://images.pexels.com/photos/1/large2x.jpg".to_string(),
                large: "https://images.pexels.com/photos/1/large.jpg".to_string(),
                medium: "https://images.pexels.com/photos/1/medium.jpg".to_string(),
                small: "https://images.pexels.com/photos/1/small.jpg".to_string(),
                portrait: "https://images.pexels.com/photos/1/portrait.jpg".to_string(),
                landscape: "https://images.pexels.com/photos/1/landscape.jpg".to_string(),
                tiny: "https://images.pexels.com/photos/1/tiny.jpg".to_string(),
            },
            alt: Some("Test Photo".to_string()),
        }
    }

    #[test]
    async fn test_get_photo_url() {
        let manager = DownloadManager::new(5);
        let photo = mock_photo();

        assert_eq!(
            manager.get_photo_url(&photo, ImageQuality::Original),
            "https://images.pexels.com/photos/1/original.jpg"
        );
        assert_eq!(
            manager.get_photo_url(&photo, ImageQuality::Large2x),
            "https://images.pexels.com/photos/1/large2x.jpg"
        );
    }
}
