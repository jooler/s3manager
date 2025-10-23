use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UploadSource {
    FilePath(String),
    FileContent(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub id: String,
    pub source: UploadSource,
    pub remote_filename: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDetail {
    pub id: String,
    pub path: String,
    pub relative_path: String,
    pub is_dir: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum UploadStatus {
    Success,
    Cancelled,
    Uploading {
        progress: f64,
        #[serde(rename = "bytesUploaded")]
        bytes_uploaded: u64,
        #[serde(rename = "totalBytes")]
        total_bytes: u64,
        speed: f64,
    },
    Error {
        message: String,
        code: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UploadHistory {
    pub file_id: String,
    pub filename: String,
    pub url: String,
    pub status: UploadStatus,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct S3Object {
    pub key: String,
    pub size: u64,
    pub last_modified: u64,
    pub etag: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct S3ObjectListResponse {
    pub objects: Vec<S3Object>,
    pub is_truncated: bool,
    pub continuation_token: Option<String>,
    pub total_count: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MultipartUpload {
    pub key: String,
    pub upload_id: String,
    pub initiated: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MultipartUploadListResponse {
    pub uploads: Vec<MultipartUpload>,
    pub is_truncated: bool,
    pub continuation_token: Option<String>,
}
