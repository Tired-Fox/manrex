use tokio::io::{AsyncReadExt, AsyncWriteExt};
use manrex::{auth::{Credentials, OAuth}, model::manga::{MangaFilter, MangaInclude}, Client};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut auth = OAuth::new(Credentials::from_env()?);

    if !auth.logged_in() {
        println!("[PRIVATE CLIENT] Logging in as user");
        auth.login_with(std::env::var("MANGADEX_USERNAME")?, std::env::var("MANGADEX_PASSWORD")?).await?;
    }
    let mut client = Client::new(auth);

    assert!(client.ping().await.is_ok());

    //println!("{:#?}", client.get_at_home_server("7c03d00c-55f5-4370-bb00-8bc3dafee36c", false).await?);

    let manga = client.list_manga(
        MangaFilter::default()
            .title("The Legendary Hero is an Academy Honors Student")
            .includes([MangaInclude::CoverArt])
    ).await?;
    let manga = manga.data.first().expect("failed to find at least one result for manga");

    let cover_art = manga.relationships
        .iter()
        .filter(|r| matches!(r.attributes.as_ref().map(|v| v.is_cover_art()), Some(true)))
        .map(|r| r.attributes.clone().unwrap().as_cover_art())
        .collect::<Vec<_>>();
    let cover_art = cover_art.first().expect("failed to get cover art");

    let cover_art_filename = cover_art.file_name.as_str();

    let mut stream = client.retrieve_cover(&manga.id, cover_art_filename, None).await?;
    let mut file = tokio::fs::OpenOptions::new()
        .truncate(true)
        .create(true)
        .write(true)
        .open("test.png")
        .await?;

    {
        let mut buffer = [0;1024];
        while let Ok(chunk) = stream.read(&mut buffer).await {
            if chunk == 0 { break; }
            file.write_all(&buffer[0..chunk]).await?;
        }
    }

    Ok(())
}
