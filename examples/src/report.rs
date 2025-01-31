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

    let mut client = Client::new(auth);

    // Max out report endpoint rate limit while avoiding general request rate limit
    for _ in 0..5 {
        let reports = client.list_user_reports(None).await?;
        println!("{reports:#?}");
    }

    std::thread::sleep(std::time::Duration::from_secs(2));

    for _ in 0..5 {
        let reports = client.list_user_reports(None).await?;
        println!("{reports:#?}");
    }

    // One more to get a rate limit error
    println!("{:#?}", client.list_user_reports(None).await);

    // One more to get a rate limit error from client with cooldown
    println!("{:#?}", client.list_user_reports(None).await);

    println!("Cooldown to reset report ratelimit...");
    std::thread::sleep(std::time::Duration::from_secs(60));

    Ok(())
}
