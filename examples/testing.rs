use std::path::PathBuf;

use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;
use manrex::{auth::{Credentials, OAuth}, model::{chapter::ChapterFilter, manga::MangaInclude}, Client};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut auth = OAuth::new(Credentials::from_env()?);

    // This block should only ever execute once, on initial run where it fetches/creates the token.
    //
    // Every other use of the api refreshes this token after it expires
    if !auth.logged_in() {
        println!("[PRIVATE CLIENT] Logging in as user");
        auth.login_with(std::env::var("MANGADEX_USERNAME")?, std::env::var("MANGADEX_PASSWORD")?).await?;
    }
    let mut client = Client::new(auth);

    assert!(client.ping().await.is_ok());

    let id = "6cf34aaa-0799-48b6-a392-dcc5b1c9b8fc";

    let manga = client.get_manga(id, [MangaInclude::CoverArt]).await?;
    let (mime, mut stream) = manga.get_cover_art(None)?.fetch().await?;

    let base = PathBuf::from(manga.attributes.title.get("en").unwrap());
    if !base.exists() {
        std::fs::create_dir_all(&base)?;
    }

    {
        let mut file = tokio::fs::OpenOptions::new()
            .truncate(true)
            .create(true)
            .write(true)
            .open(if let Some("image/jpeg") = mime.as_deref() { base.join("cover.jpg") } else { base.join("cover.png") })
            .await?;

        while let Some(Ok(chunk)) = stream.next().await {
            file.write_all(chunk.as_ref()).await?;
        }
    }

    let chapters = client.list_chapters(ChapterFilter::default().manga(id).limit(50)).await?;

    for (i, chapter) in chapters.data.iter().enumerate() {
        let server = client.get_at_home_server(&chapter.id, false).await?;

        let path = base.join(format!("chapter-{}", chapter.attributes.chapter.clone().unwrap_or(i.to_string())));
        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }

        for (j, image) in server.saver_images().iter().enumerate() {
            let (mime, mut stream) = image.fetch().await?;
            {
                let mut file = tokio::fs::OpenOptions::new()
                    .truncate(true)
                    .create(true)
                    .write(true)
                    .open(path.join(format!("page-{j}.{}", if let Some("image/jpeg") = mime.as_deref() { "jpg" } else { "png" })))
                    .await?;

                // Stream chunks of bytes from the image response to the file
                while let Some(Ok(chunk)) = stream.next().await {
                    file.write_all(chunk.as_ref()).await?;
                }
            }
        }
    }

    Ok(())
}
