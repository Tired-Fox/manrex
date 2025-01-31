use manrex::{
    auth::{Credentials, OAuth}, model::manga::MangaFilter, Client
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

    let title = "Cleric of Decay";
    let manga = client.list_manga(MangaFilter::default().title(title)).await?;
    println!("{manga:#?}");

    Ok(())
}
