use std::time::Duration;
use tauri::Manager;
use tokio::time::sleep;

#[tauri::command]
pub async fn close_splashscreen(window: tauri::Window) {
    // 关闭初始屏幕
    if let Some(splashscreen) = window.get_window("splashscreen") {
        sleep(Duration::from_secs(5)).await;

        splashscreen.close().unwrap();
    }
    // 显示主窗口
    window.get_window("main").unwrap().show().unwrap();
}