use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let resp = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    println!("Response: {}", resp);

    Ok(())
}
