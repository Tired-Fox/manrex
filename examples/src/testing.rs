use std::path::PathBuf;

use futures_util::StreamExt;
use manrex::{
    auth::{Credentials, OAuth},
    model::{chapter::ChapterFilter, manga::MangaInclude},
    Client,
};
use spinoff::{spinners, Spinner};
use tokio::io::AsyncWriteExt;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut spinner = Spinner::new(
        spinners::Dots,
        "Checking Authorization",
        spinoff::Color::Yellow,
    );

    let mut auth = OAuth::new(Credentials::from_env()?);

    // This block should only ever execute once, on initial run where it fetches/creates the token.
    //
    // Every other use of the api refreshes this token after it expires
    if !auth.logged_in() {
        spinner.update(
            spinners::Dots,
            "[PRIVATE CLIENT] Logging in as user",
            spinoff::Color::Yellow,
        );

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

    assert!(client.ping().await.is_ok());

    spinner.update(spinners::Dots, "Fetching Manga", spinoff::Color::Yellow);

    let id = "6cf34aaa-0799-48b6-a392-dcc5b1c9b8fc";
    let manga = client.get_manga(id, [MangaInclude::CoverArt]).await?;
    let title = manga.attributes.title.get("en").unwrap();

    spinner.success(title);
    spinner = Spinner::new(spinners::Dots, "Fetching Cover Art", spinoff::Color::Yellow);

    let base = PathBuf::from("manga").join(title);
    if !base.exists() {
        std::fs::create_dir_all(&base)?;
    }

    {
        let (mime, mut stream) = manga.get_cover_art(None)?.fetch().await?;
        let mut file = tokio::fs::OpenOptions::new()
            .truncate(true)
            .create(true)
            .write(true)
            .open(if let Some("image/jpeg") = mime.as_deref() {
                base.join("cover.jpg")
            } else {
                base.join("cover.png")
            })
            .await?;

        while let Some(Ok(chunk)) = stream.next().await {
            file.write_all(chunk.as_ref()).await?;
        }
    }

    spinner.update(spinners::Dots, "Fetching Chapters", spinoff::Color::Yellow);
    let chapters = client
        .list_chapters(ChapterFilter::default().manga(id).limit(50))
        .await?;
    spinner.success(&format!("{} chapters found", chapters.data.len()));

    if let Some(chapter) = chapters.data.first() {
        spinner = Spinner::new(
            spinners::Dots,
            "Downloading chapter 0",
            spinoff::Color::Yellow,
        );
        let server = client.get_at_home_server(&chapter.id, false).await?;

        let path = base.join(format!(
            "chapter-{}",
            chapter
                .attributes
                .chapter
                .clone()
                .unwrap_or("0".to_string())
        ));
        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }

        for (j, image) in server.saver_images().iter().enumerate() {
            spinner.update(
                spinners::Dots,
                format!("Downloading chapter 0 [{j}/{}]", chapter.attributes.pages),
                spinoff::Color::Yellow,
            );
            let (mime, mut stream) = image.fetch().await?;
            {
                let mut file = tokio::fs::OpenOptions::new()
                    .truncate(true)
                    .create(true)
                    .write(true)
                    .open(path.join(format!(
                        "page-{j}.{}",
                        if let Some("image/jpeg") = mime.as_deref() {
                            "jpg"
                        } else {
                            "png"
                        }
                    )))
                    .await?;

                // Stream chunks of bytes from the image response to the file
                while let Some(Ok(chunk)) = stream.next().await {
                    file.write_all(chunk.as_ref()).await?;
                }
            }
        }
        spinner.success("Chapter 0");
    }

    Ok(())
}
