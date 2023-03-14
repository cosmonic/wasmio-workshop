use wasmbus_rpc::actor::prelude::RpcResult;
use wasmcloud_interface_httpserver::{HeaderMap, HttpResponse};
use wasmcloud_interface_logging::debug;

#[derive(rust_embed::RustEmbed)]
#[folder = "./dist"]
/// Helper struct that embeds UI static assets
pub struct UiAsset;

/// Given a path, retrieves a stored static asset for the Todo UI
/// If one can't be found, this returns a 404 [HttpResponse]
pub async fn get_asset(path: &str) -> RpcResult<HttpResponse> {
    let path = path.to_string();
    let trimmed = if path.trim() == "/" {
        // Default to index.html if the root path is given alone
        debug!("Found root path, assuming index.html");
        "index.html"
    } else {
        path.trim().trim_start_matches('/')
    };

    debug!("Got path {}, attempting to fetch", trimmed);
    if let Some(file) = UiAsset::get(trimmed) {
        debug!(
            "Found file {}, returning {} bytes",
            trimmed,
            file.data.len()
        );
        let mut header = HeaderMap::new();
        if let Some(content_type) = mime_guess::from_path(trimmed)
            .first()
            .map(|m| m.to_string())
        {
            debug!(
                "Found content type of {}, setting Content-Type header",
                content_type
            );
            header.insert("Content-Type".to_string(), vec![content_type]);
        }
        Ok(HttpResponse {
            status_code: 200,
            header,
            body: Vec::from(file.data),
        })
    } else {
        debug!("Did not find file {}, returning", trimmed);
        Ok(HttpResponse::not_found())
    }
}
