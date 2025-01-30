use manrex::{
    auth::{Credentials, OAuth}, model::manga::MangaFilter, Client
};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut auth = OAuth::new(Credentials::from_env()?);

    if !auth.logged_in() {
        // Prompt for username and password to "login" and
        // authenticate while fetching the token

        let username: String = dialoguer::Input::new()
            .with_prompt("Enter your MangaDex username:")
            .interact()?;

        let password: String = dialoguer::Password::new()
            .with_prompt("Enter your MangaDex password:")
            .interact()?;

        auth.login_with(
            //std::env::var("MANGADEX_USERNAME")?,
            //std::env::var("MANGADEX_PASSWORD")?,
            username, password,
        )
        .await?;
    }

    let mut client = Client::new(auth);

    let title = "Cleric of Decay";
    let manga = client.list_manga(MangaFilter::default().title(title)).await?;
    println!("{manga:#?}");

    Ok(())
}
