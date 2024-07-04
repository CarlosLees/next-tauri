use rss::Channel;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let content = reqwest::get("https://suiyan.cc/rss")
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    println!("{:?}",channel);
    Ok(())
}