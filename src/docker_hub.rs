#[derive(serde::Deserialize, Debug, Clone)]
struct AuthResponse {
    token: String,
}

pub fn download_image(image: &str) {
    let client = reqwest::blocking::Client::new();

    // Get auth token for docker registry
    let auth_response: AuthResponse = client
        //             ^^^^^^^^^^^^ new!
        .get("https://auth.docker.io/token?service=registry.docker.io")
        .query(&[("scope", format!("repository:library/{}:pull", image))])
        .send()
        .unwrap()
        .json()  // used to be .text()
        .unwrap();

    dbg!(auth_response.token);
}