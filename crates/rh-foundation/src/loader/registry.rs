use url::Url;

use crate::http::HttpClient;

use super::{LoaderError, LoaderResult, PackageManifest, RegistryResponse};

pub(super) async fn fetch_registry_response(
    http_client: &HttpClient,
    registry_url: &str,
    package_name: &str,
) -> LoaderResult<RegistryResponse> {
    let registry_url = Url::parse(registry_url)?;
    let package_url = registry_url.join(package_name)?;

    tracing::debug!("Fetching registry response from: {}", package_url);

    http_client
        .download_json(package_url.as_str())
        .await
        .map_err(|_| LoaderError::PackageNotFound {
            package: package_name.to_string(),
            version: "any".to_string(),
        })
}

pub(super) fn sorted_versions(response: &RegistryResponse) -> Vec<String> {
    let mut versions: Vec<String> = response.versions.keys().cloned().collect();
    versions.sort();
    versions
}

pub(super) fn latest_version(
    package_name: &str,
    response: &RegistryResponse,
) -> LoaderResult<String> {
    if let Some(dist_tags) = &response.dist_tags {
        if let Some(latest) = dist_tags.get("latest") {
            return Ok(latest.clone());
        }
    }

    sorted_versions(response)
        .last()
        .cloned()
        .ok_or_else(|| LoaderError::PackageNotFound {
            package: package_name.to_string(),
            version: "latest".to_string(),
        })
}

pub(super) fn package_manifest(
    package_name: &str,
    version: &str,
    response: &RegistryResponse,
) -> LoaderResult<PackageManifest> {
    response
        .versions
        .get(version)
        .cloned()
        .ok_or_else(|| LoaderError::PackageNotFound {
            package: package_name.to_string(),
            version: version.to_string(),
        })
}
