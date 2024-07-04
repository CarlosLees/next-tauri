use rss::Channel;

#[tauri::command]
pub async fn rss() -> Result<Channel,String> {
    let content = reqwest::get("https://suiyan.cc/rss")
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    let result = Channel::read_from(&content[..]).unwrap();
    Ok(result)
}