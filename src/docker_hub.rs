use color_eyre::eyre::{Context, Result}; // new!


#[derive(serde::Deserialize, Debug, Clone)]
struct AuthResponse {
    token: String,
}

pub fn download_image(image: &str) -> Result<()> {
    //                             ^^^^^^^^^^^^^ new!
    let client = reqwest::blocking::Client::new();

    // Get auth token for docker registry
    let auth_response: AuthResponse = client
        .get("https://auth.docker.io/token?service=registry.docker.io")
        .query(&[("scope", format!("repository:library/{}:pull", image))])
        .send()
        .wrap_err("auth token request failed")?  // replaces .unwrap()
        .json()
        .wrap_err("decoding auth token request failed")?;  // replaces .unwrap()

    dbg!(auth_response.token);

    Ok(())  // new!
}