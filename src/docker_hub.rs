use color_eyre::eyre::{Context, Result};

#[derive(serde::Deserialize, Debug, Clone)]
struct AuthResponse {
    token: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
struct ManifestList {
    manifests: Vec<ManifestListItem>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ManifestListItem {
    digest: String,
    platform: Platform,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Platform {
    architecture: String,
    os: String,
}

pub fn download_image(image: &str) -> Result<()> {
    let (image, version) = match image.split_once(":") {
        Some(t) => t,
        None => (image, "latest"),
    };
    println!("Pulling image '{}' version '{}'", image, version);

    let client = reqwest::blocking::Client::new();

    // Get auth token for docker registry
    let auth_response: AuthResponse = client
        .get("https://auth.docker.io/token?service=registry.docker.io")
        .query(&[("scope", format!("repository:library/{}:pull", image))])
        .send()
        .wrap_err("auth token request failed")?
        .json()
        .wrap_err("decoding auth token request failed")?;

    // Get manifest
    let manifest_response: ManifestList = client
        .get(format!(
            "https://registry.hub.docker.com/v2/library/{}/manifests/{}",
            image, version
        ))
        .header("Authorization", format!("Bearer {}", auth_response.token))
        .header("Accept", "application/vnd.docker.distribution.manifest.v2+json")  // new
        .header("Accept", "application/vnd.docker.distribution.manifest.list.v2+json")  // new
        .send()
        .wrap_err("failed to get manifest")?
        .json()  // used to be .text()
        .wrap_err("failed to deserialize response")?;

    dbg!(manifest_response);

    Ok(())
}
