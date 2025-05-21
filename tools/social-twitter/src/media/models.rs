use {
    crate::{
        error::{TwitterApiError, TwitterError, TwitterErrorKind, TwitterErrorResponse},
        impl_twitter_response_parser,
        twitter_client::TwitterApiParsedResponse,
    },
    schemars::JsonSchema,
    serde::{Deserialize, Serialize},
};

/// Available options for media category
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub enum MediaCategory {
    #[serde(rename = "amplify_video")]
    AmplifyVideo,
    #[serde(rename = "tweet_gif")]
    TweetGif,
    #[serde(rename = "tweet_image")]
    TweetImage,
    #[serde(rename = "tweet_video")]
    TweetVideo,
    #[serde(rename = "dm_gif")]
    DmGif,
    #[serde(rename = "dm_image")]
    DmImage,
    #[serde(rename = "dm_video")]
    DmVideo,
    #[serde(rename = "subtitles")]
    Subtitles,
}

/// Available media types for upload
#[derive(Debug, Deserialize, Serialize, JsonSchema, PartialEq)]
pub enum MediaType {
    #[serde(rename = "video/mp4")]
    VideoMp4,
    #[serde(rename = "video/webm")]
    VideoWebm,
    #[serde(rename = "video/mp2t")]
    VideoMp2t,
    #[serde(rename = "text/srt")]
    TextSrt,
    #[serde(rename = "text/vtt")]
    TextVtt,
    #[serde(rename = "image/jpeg")]
    ImageJpeg,
    #[serde(rename = "image/gif")]
    ImageGif,
    #[serde(rename = "image/bmp")]
    ImageBmp,
    #[serde(rename = "image/png")]
    ImagePng,
    #[serde(rename = "image/webp")]
    ImageWebp,
    #[serde(rename = "image/pjpeg")]
    ImagePjpeg,
    #[serde(rename = "image/tiff")]
    ImageTiff,
    #[serde(rename = "model/gltf-binary")]
    ModelGltfBinary,
    #[serde(rename = "model/vnd.usdz+zip")]
    ModelUsdzZip,
}

/// Available options for media upload command
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub enum MediaCommand {
    INIT,
    APPEND,
    FINALIZE,
}

/// Media upload response
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct MediaUploadResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<MediaUploadData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<TwitterApiError>>,
}

/// Empty response for APPEND command
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct EmptyResponse {}

impl TwitterApiParsedResponse for EmptyResponse {
    type Output = ();

    fn parse_twitter_response(self) -> Result<Self::Output, TwitterErrorResponse> {
        // For empty response, we just return unit () on success
        Ok(())
    }
}

/// Media upload data
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct MediaUploadData {
    pub id: String,
    pub media_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_secs: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_info: Option<ProcessingInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

/// Processing information for media uploads
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ProcessingInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_after_secs: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_percent: Option<i32>,
    /// State of upload
    /// Available options: succeeded, in_progress, pending, failed
    pub state: ProcessingState,
}

/// State of media upload processing
#[derive(Debug, Deserialize, Serialize, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProcessingState {
    /// Upload has succeeded
    Succeeded,
    /// Upload is in progress
    InProgress,
    /// Upload is pending
    Pending,
    /// Upload has failed
    Failed,
}

impl_twitter_response_parser!(MediaUploadResponse, MediaUploadData);

impl std::fmt::Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MediaType::ImageJpeg => write!(f, "image/jpeg"),
            MediaType::ImageGif => write!(f, "image/gif"),
            MediaType::ImagePng => write!(f, "image/png"),
            MediaType::ImageWebp => write!(f, "image/webp"),
            MediaType::ImagePjpeg => write!(f, "image/pjpeg"),
            MediaType::ImageTiff => write!(f, "image/tiff"),
            MediaType::ImageBmp => write!(f, "image/bmp"),
            MediaType::VideoMp4 => write!(f, "video/mp4"),
            MediaType::VideoWebm => write!(f, "video/webm"),
            MediaType::VideoMp2t => write!(f, "video/mp2t"),
            MediaType::TextSrt => write!(f, "text/srt"),
            MediaType::TextVtt => write!(f, "text/vtt"),
            MediaType::ModelGltfBinary => write!(f, "model/gltf-binary"),
            MediaType::ModelUsdzZip => write!(f, "model/vnd.usdz+zip"),
        }
    }
}

impl std::fmt::Display for MediaCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MediaCategory::TweetImage => write!(f, "tweet_image"),
            MediaCategory::TweetGif => write!(f, "tweet_gif"),
            MediaCategory::TweetVideo => write!(f, "tweet_video"),
            MediaCategory::DmImage => write!(f, "dm_image"),
            MediaCategory::DmGif => write!(f, "dm_gif"),
            MediaCategory::DmVideo => write!(f, "dm_video"),
            MediaCategory::AmplifyVideo => write!(f, "amplify_video"),
            MediaCategory::Subtitles => write!(f, "subtitles"),
        }
    }
}
