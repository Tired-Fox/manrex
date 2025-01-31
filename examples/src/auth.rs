use manrex::{
    auth::{Credentials, OAuth}, Client
};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut auth = OAuth::new(Credentials::from_env()?);

    if !auth.logged_in() {
        auth.login_with(
            std::env::var("MANGADEX_USERNAME")?,
            std::env::var("MANGADEX_PASSWORD")?,
        )
        .await?;
    }

    let client = Client::new(auth);
    println!("Authorized: {}", client.ping().await.is_ok());

    Ok(())
}
