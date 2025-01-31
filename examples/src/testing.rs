use std::{collections::VecDeque, path::PathBuf, sync::Arc};

use manrex::{
    auth::{Credentials, OAuth},
    model::{chapter::ChapterFilter, manga::MangaInclude},
    Client, Error,
};
use spinoff::{spinners, Spinner};
use tokio::sync::Mutex;

#[tokio::main(flavor = "multi_thread", worker_threads=6)]
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

        auth.login_with(
            std::env::var("MANGADEX_USERNAME")?,
            std::env::var("MANGADEX_PASSWORD")?,
        )
        .await?;
    }
    let mut client = Client::new(auth);

    assert!(client.ping().await.is_ok());

    spinner.update(spinners::Dots, "Fetching Manga", spinoff::Color::Yellow);

    let id = "6cf34aaa-0799-48b6-a392-dcc5b1c9b8fc";
    //let id = "7f491e32-3934-4e1a-a8b5-2510aecd40d9"; // Cleric of Decay
    let manga = client.get_manga(id, [MangaInclude::CoverArt]).await?;
    println!("{}", serde_json::to_string_pretty(&manga)?);
    let title = manga.attributes.title.get("en").unwrap();

    spinner.success(title);
    spinner = Spinner::new(spinners::Dots, "Fetching Cover Art", spinoff::Color::Yellow);

    let base = PathBuf::from("manga").join(title);
    if !base.exists() {
        std::fs::create_dir_all(&base)?;
    }

    {
        let stream = manga.get_cover_art(None)?.fetch().await?;
        let ext = if stream.mime.as_str() == "image/jpeg" { ".jpg" } else { ".png" };

        let mut file = tokio::fs::OpenOptions::new()
            .truncate(true)
            .create(true)
            .write(true)
            .open(base.join(format!("cover{ext}")))
            .await?;

        stream.stream_to(&mut file).await?;
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

        // Use async tasks to split up the workload of fetching and downloading the images.
        //
        // 5 images will be downloaded at any given moment.
        let queue = Arc::new(Mutex::new(server.saver_images().into_iter().enumerate().collect::<VecDeque<_>>()));
        let (done, mut dout) = tokio::sync::mpsc::unbounded_channel::<Result<(), Error>>();
        for _ in 0..5 {
            let queue = queue.clone();
            let done = done.clone();
            let path = path.clone();
            tokio::spawn(async move {
                loop {
                    let next = queue.lock().await.pop_front();
                    match next {
                        None => break,
                        Some((index, image)) => {
                            match image.fetch().await {
                                Ok(stream) => {
                                    let ext = if stream.mime.as_str() == "image/jpeg" { ".jpg" } else { ".png" };
                                    match  tokio::fs::OpenOptions::new()
                                        .truncate(true)
                                        .create(true)
                                        .write(true)
                                        .open(path.join(format!("page-{index}{ext}")))
                                    .await
                                    {
                                        Err(err) => {
                                            let _ = done.send(Err(Error::from(err)));
                                        },
                                        Ok(mut file) => match stream.stream_to(&mut file).await {
                                            Err(err) => {
                                                let _ = done.send(Err(err));
                                            },
                                            Ok(_) => {
                                                let _ = done.send(Ok(()));
                                            } 
                                        }
                                    }
                                },
                                Err(err) => {
                                    let _ = done.send(Err(err));
                                }
                            }
                        }
                    }
                }
            });
        }

        let mut finished = 0;
        while let Some(done) = dout.recv().await {
            done?;

            finished += 1;
            spinner.update(
                spinners::Dots,
                format!("Downloading chapter 0 [{finished}/{}]", chapter.attributes.pages),
                spinoff::Color::Yellow,
            );

            if finished >= chapter.attributes.pages {
                break;
            }
        }
        spinner.success("Chapter 0");
    }

    Ok(())
}
