use manrex::{
    auth::{Credentials, OAuth}, model::{custom_list::ListInclude, manga::{MangaFilter, MangaInclude}, Relationship}, Client
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

    //let id = "47ab4765-d03f-4f6f-ab61-044c66170f8e";
    //let title = "Cleric of Decay";
    //let manga = client.get_manga(id, None).await?;
    //let relations = client.get_manga_relation_list(id, None).await?;
    //println!("{manga:#?}");
    //println!("{relations:#?}");

    // Self Published
    let a = "f66ebc10-ef89-46d1-be96-bb704559e04a";
    // Staff Picks
    let b = "805ba886-dd99-4aa4-b460-4bd7c7b71352";
    // Featured by Supporters
    let c = "5c5e6e39-0b4b-413e-be59-27b1ba03d1b9";

    let lists = [
        a,
        b,
        c
    ];

    for list in lists {
        let list = client.get_list(list, [ListInclude::User]).await?;
        let name = list.attributes.name.as_str();

        let ids = list.relationships
            .into_iter()
            .filter_map(|v| v.attributes.and_then(|a| if a.is_manga() { Some(v.id) } else { None }))
            .collect::<Vec<_>>();

        let manga = client.list_manga(
            MangaFilter::default()
                .limit(ids.len())
                .ids(ids)
                .includes([MangaInclude::CoverArt])
        ).await?;

        println!("{name}");
        for m in manga.data.iter() {
            if let Some(title) = m.attributes.title.get("en") {
                println!("  - {title}");
            }
        }
    }

    Ok(())
}
