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

    let id = "47ab4765-d03f-4f6f-ab61-044c66170f8e";

    //let title = "Cleric of Decay";
    let manga = client.get_manga(id, None).await?;
    let relations = client.get_manga_relation_list(id, None).await?;
    println!("{manga:#?}");
    println!("{relations:#?}");

    Ok(())
}
