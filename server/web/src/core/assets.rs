use axum::{
    body::Body,
    extract::{Path, Query},
    http::{header, HeaderMap, StatusCode},
    response::Response,
};
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Assets;

/// 存储文件名到哈希的映射，用于生成带版本号的 URL
/// 格式: (原始文件名, 前8位sha256哈希)
pub static ASSET_HASHES: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for filename in Assets::iter() {
        if let Some(file) = Assets::get(&filename) {
            let hash = file.metadata.sha256_hash();
            // 取前8字节（16个hex字符）作为版本号
            let hash_str = hex::encode(&hash[..8]);
            map.insert(filename.to_string(), hash_str);
        }
    }
    map
});

/// 生成带版本号的资源 URL
/// 例如: asset_url("js/app.js") -> "/static/js/app.js?v=a3f2b1c0"
pub fn asset_url(path: &str) -> String {
    if let Some(hash) = ASSET_HASHES.get(path) {
        format!("/static/{}?v={}", path, hash)
    } else {
        format!("/static/{}", path)
    }
}

#[derive(Deserialize)]
pub struct AssetQuery {
    v: Option<String>,
}

pub async fn serve_static(
    Path(path): Path<String>,
    Query(query): Query<AssetQuery>,
    headers: HeaderMap,
) -> Response<Body> {
    match Assets::get(&path) {
        Some(content) => {
            // 计算 ETag: 使用 sha256 前8位 + 文件大小
            let hash = content.metadata.sha256_hash();
            let hash_str = hex::encode(&hash[..8]);
            let etag = format!(r#""{}-{}""#, hash_str, content.data.len());

            // 检查查询参数是否匹配（可选的安全层）
            if let Some(ref v) = query.v {
                if v != &hash_str {
                    // 版本不匹配，返回 400 Bad Request
                    // 这样用户会知道 URL 过期了
                    return Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .header(header::CONTENT_TYPE, "text/plain")
                        .body(Body::from(format!(
                            "Asset version mismatch. Expected: {}, Got: {}",
                            hash_str, v
                        )))
                        .unwrap();
                }
            }

            // 检查 If-None-Match 头
            if let Some(if_none_match) = headers.get(header::IF_NONE_MATCH) {
                if if_none_match.as_bytes() == etag.as_bytes() {
                    return Response::builder()
                        .status(StatusCode::NOT_MODIFIED)
                        .body(Body::empty())
                        .unwrap();
                }
            }

            let mime = mime_guess::from_path(&path).first_or_octet_stream();
            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .header(header::ETAG, etag)
                .header(
                    header::CACHE_CONTROL,
                    "public, max-age=31536000, immutable",
                )
                .body(Body::from(content.data))
                .unwrap()
        }
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not Found"))
            .unwrap(),
    }
}
