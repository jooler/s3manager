use crate::typ::FileDetail;
use base64::{engine::general_purpose, Engine};
use mime_guess::from_path;
use uuid::Uuid;

async fn get_file_details_internal(
    path: String,
    base_path: &str,
) -> Result<Vec<FileDetail>, String> {
    let metadata = tokio::fs::metadata(&path)
        .await
        .map_err(|e| format!("无法获取文件元数据：{}", e))?;

    let mut result = Vec::new();
    if metadata.is_dir() {
        let mut entries = tokio::fs::read_dir(&path)
            .await
            .map_err(|e| format!("无法读取目录：{}", e))?;

        while let Some(entry) = entries.next_entry().await.map_err(|e| e.to_string())? {
            let child_path = entry.path().to_string_lossy().to_string();
            let child_details = Box::pin(get_file_details_internal(child_path, base_path)).await?;
            result.extend(child_details);
        }
    } else {
        let relative_path = path
            .strip_prefix(base_path)
            .map(|p| p.to_string())
            .unwrap_or_else(|| path.clone());

        result.push(FileDetail {
            id: Uuid::new_v4().to_string(),
            path: path.clone(),
            relative_path,
            is_dir: false,
        });
    }

    Ok(result)
}

#[tauri::command]
pub async fn get_file_details(path: String) -> Result<Vec<FileDetail>, String> {
    Box::pin(async move {
        let base_path = std::path::Path::new(&path)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "".to_string());

        get_file_details_internal(path, &base_path).await
    })
    .await
}

#[tauri::command]
pub async fn preview_file(path: String) -> Result<String, String> {
    let metadata = tokio::fs::metadata(&path)
        .await
        .map_err(|e| format!("无法获取文件元数据：{}", e))?;

    if metadata.len() > 10 * 1024 * 1024 {
        return Err("文件大小超过 10MB 限制".to_string());
    }

    let mime_type = from_path(&path).first_or_octet_stream();

    // 处理图片文件
    if mime_type.type_() == "image" {
        let supported_formats = ["png", "jpg", "jpeg", "webp", "gif", "bmp", "tiff"];
        if supported_formats.contains(&mime_type.subtype().as_ref()) {
            let data = tokio::fs::read(&path)
                .await
                .map_err(|e| format!("无法读取图片文件：{}", e))?;
            let base64 = general_purpose::STANDARD.encode(data);
            return Ok(format!("data:{};base64,{}", mime_type, base64));
        }
    }

    // 处理文本文件
    if mime_type.type_() == "text"
        || mime_type.subtype() == "json"
        || mime_type.subtype() == "csv"
        || mime_type.subtype() == "markdown"
    {
        let content = tokio::fs::read_to_string(&path)
            .await
            .map_err(|e| format!("无法读取文件：{}", e))?;
        let lines: Vec<&str> = content.lines().take(100).collect();
        return Ok(lines.join("\n"));
    }

    Err(format!("不支持的文件类型：{}", mime_type))
}
