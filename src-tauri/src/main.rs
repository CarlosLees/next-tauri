// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;

use tauri::generate_handler;
use crate::command::application::app_list;
use crate::command::rss::*;
use crate::command::close_splashscreen;
use crate::command::local_ip::local_ip;

fn main() {
    tauri::Builder::default()
        .invoke_handler(generate_handler![local_ip,app_list,close_splashscreen])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
