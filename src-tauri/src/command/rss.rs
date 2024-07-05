use rss::{Channel, Error};

#[tauri::command]
pub async fn rss_list() -> Result<Channel,()> {
    let content = reqwest::get("https://suiyan.cc/rss")
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    let result = Channel::read_from(&content[..]);
    Ok(result.unwrap())
}