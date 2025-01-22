use manrex::{auth::{Credentials, OAuth}, Client};

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
    println!("{:#?}", client.list_authors(None).await?);

    Ok(())
}
