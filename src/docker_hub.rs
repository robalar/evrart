use color_eyre::eyre::{Context, Result};

#[derive(serde::Deserialize, Debug, Clone)]
struct AuthResponse {
    token: String,
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
    let manifest_response = client
        .get(format!(
            "https://registry.hub.docker.com/v2/library/{}/manifests/{}",
            image, version
        ))
        .header("Authorization", format!("Bearer {}", auth_response.token))
        .send()
        .wrap_err("failed to get manifest")?
        .text()
        .wrap_err("failed to get response text")?;

    println!("{}", manifest_response);

    Ok(())
}
