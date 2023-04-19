use reqwest::header::USER_AGENT;
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Make a GET request to https://www.rust-lang.org
    let resp = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    println!("Response: {}", resp);

    fetch_data().await?;

    Ok(())
}

async fn fetch_data() -> Result<(), Error> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );
    println!("Requesting {}", request_url);

    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header(USER_AGENT, "rust web-api-client demo")
        .send()
        .await?;

    let users: Vec<User> = response.json().await?;
    for user in users {
        println!("{} ({})", user.login, user.id);
    }
    Ok(())
}
