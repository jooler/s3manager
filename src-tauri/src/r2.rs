use crate::typ::{
    File, MultipartUpload, MultipartUploadListResponse, S3Object, S3ObjectListResponse,
    UploadHistory, UploadSource, UploadStatus,
};
use aws_config::timeout::TimeoutConfig;
use aws_config::ConfigLoader;
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::types::{CompletedMultipartUpload, CompletedPart};
use aws_sdk_s3::Client;
use aws_smithy_runtime::client::http::hyper_014::HyperClientBuilder;
use dashmap::DashMap;
use hyper::client::HttpConnector;
use hyper_proxy::ProxyConnector;
use mime_guess::from_path;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::{AppHandle, Emitter};
use tokio::io::AsyncReadExt;
use tokio::sync::Semaphore;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use chrono::Utc;

// 键是 file_id，值是一个元组，包含一个 JoinHandle 和一个 Option<String>，用于存储 upload_id，upload_id 用于分段上传
static UPLOAD_TASKS: Lazy<
    DashMap<String, (tokio::task::JoinHandle<Result<(), String>>, Option<String>)>,
> = Lazy::new(DashMap::new);

static UPLOAD_TASKS_INFO: Lazy<DashMap<String, (Arc<R2Client>, String)>> = Lazy::new(DashMap::new);

#[tauri::command]
pub async fn r2_ping(
    bucket_name: &str,
    account_id: &str,
    access_key: &str,
    secret_key: &str,
    endpoint: Option<&str>,
) -> Result<(), String> {
    let client = R2Client::new_with_endpoint(bucket_name, account_id, access_key, secret_key, None, endpoint).await?;
    client.ping().await
}

#[tauri::command]
pub async fn r2_upload(
    app: AppHandle,
    bucket_name: &str,
    account_id: &str,
    access_key: &str,
    secret_key: &str,
    domain: Option<&str>,
    endpoint: Option<&str>,
    files: Vec<File>,
) -> Result<(), String> {
    let client =
        Arc::new(R2Client::new_with_endpoint(bucket_name, account_id, access_key, secret_key, domain, endpoint).await?);

    for file in files {
        let client = client.clone();
        let app = app.clone();
        let filename = file.remote_filename.clone();
        let file_id = file.id.clone();

        let handle = tokio::spawn(async move {
            let result = match &file.source {
                UploadSource::FilePath(path) => {
                    client
                        .stream_upload_file(&app, &path, &filename, &file_id.clone())
                        .await
                }
                UploadSource::FileContent(content) => {
                    emit_progress(
                        &app,
                        format!("{}/{}", client.domain, filename),
                        file_id.clone(),
                        filename.clone(),
                        UploadStatus::Uploading {
                            progress: 0.0,
                            bytes_uploaded: 0,
                            total_bytes: content.len() as u64,
                            speed: 0.0,
                        },
                    );
                    client.upload_content(content, &filename).await
                }
            };

            emit_progress(
                &app,
                format!("{}/{}", client.domain, filename),
                file_id,
                filename,
                match &result {
                    Ok(_) => UploadStatus::Success,
                    Err(e) => UploadStatus::Error {
                        message: e.to_string(),
                        code: "UPLOAD_ERROR".to_string(),
                    },
                },
            );

            result
        });

        UPLOAD_TASKS.insert(file.id.clone(), (handle, None));
    }

    Ok(())
}

pub fn emit_progress(
    app: &AppHandle,
    url: String,
    file_id: String,
    filename: String,
    status: UploadStatus,
) {
    let _ = app.emit(
        "upload-progress",
        UploadHistory {
            url,
            file_id,
            filename,
            status,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        },
    );
}

#[tauri::command]
pub async fn r2_cancel_upload(app: AppHandle, file_id: String) -> Result<(), String> {
    // First get all the information we need
    let task_info = UPLOAD_TASKS
        .get(&file_id)
        .map(|entry| (entry.0.abort(), entry.1.clone()));

    let client_info = UPLOAD_TASKS_INFO
        .get(&file_id)
        .map(|entry| (entry.0.clone(), entry.1.clone()));

    let mut filename = "".to_string();

    // Then handle the cancellation
    if let Some((_, upload_id)) = task_info {
        if let Some(upload_id) = upload_id {
            if let Some((client, remote_filename)) = client_info {
                filename = remote_filename.clone();
                let _ = client
                    .abort_multipart_upload(&remote_filename, &upload_id)
                    .await;
            }
        }

        // Finally remove the entries
        UPLOAD_TASKS.remove(&file_id);
        UPLOAD_TASKS_INFO.remove(&file_id);

        // emit
        emit_progress(
            &app,
            "".to_string(),
            file_id,
            filename,
            UploadStatus::Cancelled,
        );
    }

    Ok(())
}

#[tauri::command]
pub async fn r2_list_objects(
    bucket_name: &str,
    account_id: &str,
    access_key: &str,
    secret_key: &str,
    max_keys: u32,
    continuation_token: Option<String>,
    endpoint: Option<&str>,
) -> Result<S3ObjectListResponse, String> {
    let client = R2Client::new_with_endpoint(bucket_name, account_id, access_key, secret_key, None, endpoint).await?;
    client
        .list_objects(max_keys, continuation_token.as_deref())
        .await
}

#[tauri::command]
pub async fn r2_list_multipart_uploads(
    bucket_name: &str,
    account_id: &str,
    access_key: &str,
    secret_key: &str,
    endpoint: Option<&str>,
) -> Result<MultipartUploadListResponse, String> {
    let client = R2Client::new_with_endpoint(bucket_name, account_id, access_key, secret_key, None, endpoint).await?;
    client.list_multipart_uploads().await
}

#[tauri::command]
pub async fn r2_delete_object(
    bucket_name: &str,
    account_id: &str,
    access_key: &str,
    secret_key: &str,
    key: &str,
    endpoint: Option<&str>,
) -> Result<(), String> {
    let client = R2Client::new_with_endpoint(bucket_name, account_id, access_key, secret_key, None, endpoint).await?;
    client.delete_object(key).await
}

#[tauri::command]
pub async fn r2_abort_multipart_upload_cmd(
    bucket_name: &str,
    account_id: &str,
    access_key: &str,
    secret_key: &str,
    key: &str,
    upload_id: &str,
    endpoint: Option<&str>,
) -> Result<(), String> {
    let client = R2Client::new_with_endpoint(bucket_name, account_id, access_key, secret_key, None, endpoint).await?;
    client.abort_multipart_upload(key, upload_id).await
}

#[tauri::command]
pub async fn r2_get_presigned_url(
    bucket_name: &str,
    account_id: &str,
    access_key: &str,
    secret_key: &str,
    key: &str,
    endpoint: Option<&str>,
    expires_in: Option<u64>,
) -> Result<String, String> {
    let client = R2Client::new_with_endpoint(bucket_name, account_id, access_key, secret_key, None, endpoint).await?;
    client.get_presigned_url(key, expires_in.unwrap_or(3600)).await
}

#[derive(Clone)]
pub struct R2Client {
    client: Client,
    bucket_name: String,
    domain: String,
    endpoint: Option<String>,
    access_key: String,
    secret_key: String,
    account_id: String,
}

impl R2Client {
    #[allow(dead_code)]
    pub async fn new(
        bucket_name: &str,
        account_id: &str,
        access_key: &str,
        secret_key: &str,
        domain: Option<&str>,
    ) -> Result<Self, String> {
        Self::new_with_endpoint(bucket_name, account_id, access_key, secret_key, domain, None).await
    }

    pub async fn new_with_endpoint(
        bucket_name: &str,
        account_id: &str,
        access_key: &str,
        secret_key: &str,
        domain: Option<&str>,
        endpoint: Option<&str>,
    ) -> Result<Self, String> {
        println!("new r2 client...");
        // 设置环境变量 AWS_REQUEST_CHECKSUM_CALCULATION
        std::env::set_var("AWS_REQUEST_CHECKSUM_CALCULATION", "WHEN_REQUIRED");

        let credentials = Credentials::new(access_key, secret_key, None, None, "R2Uploader");

        // 设置超时配置
        let timeout_config = TimeoutConfig::builder()
            .connect_timeout(Duration::from_secs(30)) // 连接超时 30 秒
            .read_timeout(Duration::from_secs(30)) // 读取超时 30 秒
            .build();

        // 如果提供了自定义 endpoint，使用它；否则使用 R2 默认 endpoint
        let endpoint_url = if let Some(ep) = endpoint {
            ep.to_string()
        } else {
            format!("https://{}.r2.cloudflarestorage.com", account_id)
        };

        let mut config_loader = ConfigLoader::default()
            .region(Region::new("auto"))
            .endpoint_url(endpoint_url)
            .timeout_config(timeout_config)
            .credentials_provider(credentials);

        if let Some(proxy_connector) = create_proxy_connector() {
            config_loader =
                config_loader.http_client(HyperClientBuilder::new().build(proxy_connector));
        }

        let config = config_loader.load().await;

        Ok(Self {
            client: Client::new(&config),
            bucket_name: bucket_name.to_string(),
            domain: domain.unwrap_or("").to_string(),
            endpoint: endpoint.map(|s| s.to_string()),
            access_key: access_key.to_string(),
            secret_key: secret_key.to_string(),
            account_id: account_id.to_string(),
        })
    }

    // 上传文件内容，一般是文字或图片，内容不会太大，直接上传，且不需要进度
    pub async fn upload_content(&self, content: &str, remote_filename: &str) -> Result<(), String> {
        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(remote_filename)
            .body(content.as_bytes().to_vec().into())
            .content_type(
                from_path(remote_filename)
                    .first_or_octet_stream()
                    .to_string(),
            )
            .send()
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    // 创建多部分上传
    async fn create_multipart_upload(&self, remote_filename: &str) -> Result<String, String> {
        self.client
            .create_multipart_upload()
            .bucket(&self.bucket_name)
            .key(remote_filename)
            .content_type(from_path(remote_filename).first_or_octet_stream().as_ref())
            .send()
            .await
            .map_err(|e| e.to_string())?
            .upload_id()
            .ok_or_else(|| "Failed to get upload ID".to_string())
            .map(|id| id.to_string())
    }

    async fn complete_multipart_upload(
        &self,
        remote_filename: &str,
        upload_id: &str,
        parts: Vec<CompletedPart>,
    ) -> Result<(), String> {
        self.client
            .complete_multipart_upload()
            .bucket(&self.bucket_name)
            .key(remote_filename)
            .upload_id(upload_id)
            .multipart_upload(
                CompletedMultipartUpload::builder()
                    .set_parts(Some(parts))
                    .build(),
            )
            .send()
            .await
            .map_err(|e| {
                println!("完成多部分上传时遇到错误：{}", e.to_string());
                e.to_string()
            })?;
        Ok(())
    }

    async fn upload_part(
        &self,
        remote_filename: &str,
        upload_id: &str,
        part_number: i32,
        body: Vec<u8>,
    ) -> Result<CompletedPart, String> {
        self.client
            .upload_part()
            .bucket(&self.bucket_name)
            .key(remote_filename)
            .upload_id(upload_id)
            .part_number(part_number)
            .body(aws_sdk_s3::primitives::ByteStream::from(body))
            .send()
            .await
            .map_err(|e| e.to_string())?
            .e_tag()
            .ok_or_else(|| "Failed to get ETag".to_string())
            .map(|e_tag| {
                CompletedPart::builder()
                    .e_tag(e_tag)
                    .part_number(part_number)
                    .build()
            })
    }

    async fn stream_upload_file(
        &self,
        app: &tauri::AppHandle,
        path: &str,
        remote_filename: &str,
        file_id: &str,
    ) -> Result<(), String> {
        const CHUNK_SIZE: usize = 5 * 1024 * 1024; // 5MB chunks
        const MAX_CONCURRENT_TASKS: usize = 16; // 最大并发任务数

        // 读取文件信息
        let mut file = tokio::fs::File::open(path)
            .await
            .map_err(|e| e.to_string())?;
        let file_size = file.metadata().await.map_err(|e| e.to_string())?.len() as usize;

        // 首次报告
        emit_progress(
            &app,
            format!("{}/{}", self.domain, remote_filename),
            file_id.to_string(),
            remote_filename.to_string(),
            UploadStatus::Uploading {
                progress: 0.0,
                bytes_uploaded: 0,
                total_bytes: file_size as u64,
                speed: 0.0,
            },
        );

        // 如果文件小于 CHUNK_SIZE，直接上传
        if file_size < CHUNK_SIZE {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)
                .await
                .map_err(|e| e.to_string())?;
            self.client
                .put_object()
                .bucket(&self.bucket_name)
                .key(remote_filename)
                .body(buffer.into())
                .content_type(
                    from_path(remote_filename)
                        .first_or_octet_stream()
                        .to_string(),
                )
                .send()
                .await
                .map_err(|e| e.to_string())?;
            return Ok(());
        }

        // 大文件，分块上传
        let upload_id = self.create_multipart_upload(remote_filename).await?;

        // Store upload_id in UPLOAD_TASKS
        if let Some(mut entry) = UPLOAD_TASKS.get_mut(file_id) {
            entry.1 = Some(upload_id.clone());
        }

        // Store client and remote_filename for potential abort
        UPLOAD_TASKS_INFO.insert(
            file_id.to_string(),
            (Arc::new(self.clone()), remote_filename.to_string()),
        );

        let start_time = SystemTime::now();
        let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_TASKS)); // 限制并发任务数
        let mut tasks = Vec::new();
        let mut part_number = 1;
        let bytes_uploaded = Arc::new(AtomicUsize::new(0)); // 用于跟踪实际上传的字节数
        let mut file_offset = 0; // 用于跟踪文件的读取偏移量

        // 读取文件并分块上传
        loop {
            // 获取 Semaphore 许可
            let semaphore = semaphore.clone();
            let permit = semaphore.acquire_owned().await.map_err(|e| e.to_string())?;

            let remaining_bytes = file_size - file_offset; // 基于文件偏移量计算剩余字节
            let buffer_size = if remaining_bytes > CHUNK_SIZE {
                CHUNK_SIZE
            } else {
                remaining_bytes
            };

            if buffer_size == 0 {
                break; // 文件已读取完毕
            }

            let mut buffer = vec![0; buffer_size];
            file.read_exact(&mut buffer)
                .await
                .map_err(|e| e.to_string())?;

            // 克隆需要的变量以在任务中使用
            let client = self.clone();
            let remote_filename = remote_filename.to_string();
            let upload_id = upload_id.clone();
            let app = app.clone();
            let file_id = file_id.to_string();
            let domain = self.domain.clone();
            let bytes_uploaded = bytes_uploaded.clone();

            // 启动并行上传任务
            let task = tokio::spawn(async move {
                let part = client
                    .upload_part(&remote_filename, &upload_id, part_number, buffer.to_vec())
                    .await?;

                // 更新实际上传的字节数
                bytes_uploaded.fetch_add(buffer_size, Ordering::SeqCst);

                // 更新进度
                let elapsed = SystemTime::now()
                    .duration_since(start_time)
                    .unwrap_or_default();
                let uploaded = bytes_uploaded.load(Ordering::SeqCst);
                let speed = uploaded as f64 / elapsed.as_secs_f64();
                emit_progress(
                    &app,
                    format!("{}/{}", domain, remote_filename),
                    file_id,
                    remote_filename,
                    UploadStatus::Uploading {
                        progress: uploaded as f64 / file_size as f64,
                        bytes_uploaded: (part_number as usize * CHUNK_SIZE) as u64,
                        total_bytes: file_size as u64,
                        speed,
                    },
                );

                // 释放 Semaphore 许可
                drop(permit);

                Ok::<_, String>(part)
            });

            tasks.push(task);
            file_offset += buffer_size; // 更新文件读取偏移量
            part_number += 1;
        }

        // 等待所有任务完成
        let results = futures::future::try_join_all(tasks).await.unwrap();
        let completed_parts: Vec<_> = results.into_iter().collect::<Result<_, _>>()?;

        // 完成分块上传
        self.complete_multipart_upload(remote_filename, &upload_id, completed_parts)
            .await
    }

    async fn abort_multipart_upload(
        &self,
        remote_filename: &str,
        upload_id: &str,
    ) -> Result<(), String> {
        self.client
            .abort_multipart_upload()
            .bucket(&self.bucket_name)
            .key(remote_filename)
            .upload_id(upload_id)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn ping(&self) -> Result<(), String> {
        println!("ping...");
        self.client
            .head_bucket()
            .bucket(&self.bucket_name)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn list_objects(
        &self,
        max_keys: u32,
        continuation_token: Option<&str>,
    ) -> Result<S3ObjectListResponse, String> {
        let mut request = self
            .client
            .list_objects_v2()
            .bucket(&self.bucket_name)
            .max_keys(max_keys as i32);

        if let Some(token) = continuation_token {
            request = request.continuation_token(token);
        }

        let response = request.send().await.map_err(|e| e.to_string())?;

        let objects: Vec<S3Object> = response
            .contents()
            .iter()
            .map(|obj| S3Object {
                key: obj.key().unwrap_or("").to_string(),
                size: obj.size().unwrap_or(0) as u64,
                last_modified: obj
                    .last_modified()
                    .and_then(|dt| {
                        dt.secs()
                            .try_into()
                            .ok()
                    })
                    .unwrap_or(0),
                etag: obj.e_tag().unwrap_or("").to_string(),
            })
            .collect();

        Ok(S3ObjectListResponse {
            objects,
            is_truncated: response.is_truncated().unwrap_or(false),
            continuation_token: response.next_continuation_token().map(|s| s.to_string()),
            total_count: response.key_count().unwrap_or(0) as usize,
        })
    }

    pub async fn list_multipart_uploads(&self) -> Result<MultipartUploadListResponse, String> {
        let response = self
            .client
            .list_multipart_uploads()
            .bucket(&self.bucket_name)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let uploads: Vec<MultipartUpload> = response
            .uploads()
            .iter()
            .map(|upload| MultipartUpload {
                key: upload.key().unwrap_or("").to_string(),
                upload_id: upload.upload_id().unwrap_or("").to_string(),
                initiated: upload
                    .initiated()
                    .and_then(|dt| {
                        dt.secs()
                            .try_into()
                            .ok()
                    })
                    .unwrap_or(0),
            })
            .collect();

        Ok(MultipartUploadListResponse {
            uploads,
            is_truncated: response.is_truncated().unwrap_or(false),
            continuation_token: response.key_marker().map(|s| s.to_string()),
        })
    }

    pub async fn delete_object(&self, key: &str) -> Result<(), String> {
        self.client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    // 生成 OSS 预签名 URL（使用 OSS V4 签名）
    fn generate_oss_presigned_url(
        &self,
        key: &str,
        expires_in: u64,
    ) -> Result<String, String> {
        let endpoint = self.endpoint.as_ref().ok_or("Endpoint is required for OSS")?;

        // 移除协议前缀
        let endpoint_host = endpoint
            .trim_start_matches("https://")
            .trim_start_matches("http://");

        // 从 endpoint 中提取 region，例如从 "oss-cn-shanghai.aliyuncs.com" 提取 "cn-shanghai"
        let region = if let Some(region_part) = endpoint_host.split('.').next() {
            if region_part.starts_with("oss-") {
                region_part.trim_start_matches("oss-")
            } else {
                "auto"
            }
        } else {
            "auto"
        };

        // 获取当前时间
        let now = Utc::now();
        let date_stamp = now.format("%Y%m%d").to_string();
        let date_time = now.format("%Y%m%dT%H%M%SZ").to_string();

        // 构建 credential
        let credential = format!(
            "{}/{}/{}/oss/aliyun_v4_request",
            self.access_key, date_stamp, region
        );

        // 构建 URL（使用 virtual-hosted-style）
        let host = format!("{}.{}", self.bucket_name, endpoint_host);

        // 构建 canonical URI
        // 注意：路径分隔符 / 不应该被编码，只编码每个路径段
        let canonical_uri = if key.contains('/') {
            // 如果 key 包含路径分隔符，分别编码每个段
            let segments: Vec<String> = key
                .split('/')
                .map(|segment| urlencoding::encode(segment).to_string())
                .collect();
            format!("/{}", segments.join("/"))
        } else {
            // 如果没有路径分隔符，直接编码
            format!("/{}", urlencoding::encode(key))
        };

        // 构建查询参数（按字母顺序排序）
        // 注意：必须包含 x-oss-additional-headers=host
        let mut query_params = vec![
            ("x-oss-additional-headers", "host".to_string()),
            ("x-oss-credential", urlencoding::encode(&credential).to_string()),
            ("x-oss-date", date_time.clone()),
            ("x-oss-expires", expires_in.to_string()),
            ("x-oss-signature-version", "OSS4-HMAC-SHA256".to_string()),
        ];
        query_params.sort_by(|a, b| a.0.cmp(&b.0));

        // 构建 canonical query string
        let canonical_query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");

        // 构建 canonical headers
        let canonical_headers = format!("host:{}\n", host);

        // 构建 additional headers
        let additional_headers = "host";

        // 构建 canonical request
        // 格式：HTTP-Verb\nCanonical-URI\nCanonical-Query-String\nCanonical-Headers\n\nAdditional-Headers\nUNSIGNED-PAYLOAD
        let canonical_request = format!(
            "GET\n{}\n{}\n{}\n{}\nUNSIGNED-PAYLOAD",
            canonical_uri, canonical_query_string, canonical_headers, additional_headers
        );

        // 计算 canonical request 的 SHA256
        let mut hasher = Sha256::new();
        hasher.update(canonical_request.as_bytes());
        let canonical_request_hash = hex::encode(hasher.finalize());

        // 构建 string to sign
        let scope = format!("{}/{}/oss/aliyun_v4_request", date_stamp, region);
        let string_to_sign = format!(
            "OSS4-HMAC-SHA256\n{}\n{}\n{}",
            date_time, scope, canonical_request_hash
        );

        // 计算签名
        type HmacSha256 = Hmac<Sha256>;

        let k_date = HmacSha256::new_from_slice(format!("aliyun_v4{}", self.secret_key).as_bytes())
            .map_err(|e| e.to_string())?
            .chain_update(date_stamp.as_bytes())
            .finalize()
            .into_bytes();

        let k_region = HmacSha256::new_from_slice(&k_date)
            .map_err(|e| e.to_string())?
            .chain_update(region.as_bytes())
            .finalize()
            .into_bytes();

        let k_service = HmacSha256::new_from_slice(&k_region)
            .map_err(|e| e.to_string())?
            .chain_update(b"oss")
            .finalize()
            .into_bytes();

        let k_signing = HmacSha256::new_from_slice(&k_service)
            .map_err(|e| e.to_string())?
            .chain_update(b"aliyun_v4_request")
            .finalize()
            .into_bytes();

        let signature = HmacSha256::new_from_slice(&k_signing)
            .map_err(|e| e.to_string())?
            .chain_update(string_to_sign.as_bytes())
            .finalize()
            .into_bytes();

        let signature_hex = hex::encode(signature);

        // 构建最终 URL
        let final_url = format!(
            "https://{}{}?{}&x-oss-signature={}",
            host, canonical_uri, canonical_query_string, signature_hex
        );

        Ok(final_url)
    }

    pub async fn get_presigned_url(&self, key: &str, expires_in: u64) -> Result<String, String> {
        // 判断是否是 OSS（通过 endpoint 是否包含 "aliyuncs.com"）
        let is_oss = self.endpoint.as_ref().map_or(false, |ep| ep.contains("aliyuncs.com"));

        if is_oss {
            // OSS 使用自定义的签名算法
            self.generate_oss_presigned_url(key, expires_in)
        } else {
            // R2 使用 AWS SDK 的预签名 URL
            let presigning_config = aws_sdk_s3::presigning::PresigningConfig::builder()
                .expires_in(std::time::Duration::from_secs(expires_in))
                .build()
                .map_err(|e| e.to_string())?;

            let presigned_request = self
                .client
                .get_object()
                .bucket(&self.bucket_name)
                .key(key)
                .presigned(presigning_config)
                .await
                .map_err(|e| e.to_string())?;

            Ok(presigned_request.uri().to_string())
        }
    }
}

fn create_proxy_connector() -> Option<ProxyConnector<HttpConnector>> {
    #[cfg(any(target_os = "ios", target_os = "android"))]
    return None;

    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    match sysproxy::Sysproxy::get_system_proxy() {
        Ok(proxy) if !proxy.host.is_empty() && proxy.port > 0 && proxy.enable => {
            // Try to create proxy URI and connector
            let proxy_uri = format!("http://{}:{}", proxy.host, proxy.port)
                .parse()
                .ok()?;
            let proxy = hyper_proxy::Proxy::new(hyper_proxy::Intercept::All, proxy_uri);
            ProxyConnector::from_proxy(HttpConnector::new(), proxy).ok()
        }
        _ => None, // Return None if no proxy or error getting proxy
    }
}
