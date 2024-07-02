use std::{fs, vec};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use anyhow::Result;
use plist::Value;
use serde::{Deserialize, Serialize};
use image::io::Reader as ImageReader;

#[tauri::command]
pub fn local_ip() -> Result<String,()> {
    // let uri = "https://api.ipify.org/?format=text";
    // let response =  reqwest::blocking::get(uri).expect("Failed to send request");
    // //println!(" response.status:{}", response.status());
    // if let Ok(body) = response.text() {
    //     //println!("Response body:\n{}", body);
    //     println!("{:?}", body);
    //     return Ok(body)
    // }
    Ok("".into())
}

#[tauri::command]
pub fn app_list() -> Result<Vec<AppModel>,()> {
    #[cfg(target_os = "windows")]
    Ok(());

    #[cfg(target_os = "macos")]
    let mut apps:Vec<AppModel> = vec![];

    list_applications("/Applications",&mut apps);
    list_applications("/System/Applications", &mut apps);
    list_applications("/System/Library/CoreServices", &mut apps);

    println!("{:?}",apps);

    apps.iter().map(|item| {
        let icon = item.icon.clone();
        tokio::spawn(async move {
            let mut file = File::open(icon).unwrap();
            let mut buffer = Vec::new();

            // 读取文件内容
            file.read_to_end(&mut buffer)?;

            // 将图片解码为动态图片
            let img = ImageReader::new(std::io::Cursor::new(buffer)).with_guessed_format()?.decode()?;

            // 将图片编码为 PNG 格式的字节数组
            let mut bytes = Vec::new();
            img.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;

            // 将字节数组转换为 Base64
            let base64_string = encode(&bytes);

            Ok(base64_string)
        })
    });
    Ok(apps)
}

#[derive(Serialize,Deserialize,Debug)]
pub struct AppModel {
    name: String,
    icon: PathBuf,
}

impl AppModel {
    fn new(name: String, icon: PathBuf) -> Self {
        Self {
            name,
            icon
        }
    }
}

fn list_applications(dir: &str, vecs: &mut Vec<AppModel>) {
    let path = Path::new(dir);
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() && path.extension().and_then(|s| s.to_str()) == Some("app") {
                    if let Some((name, icon_path)) = get_app_info(&path) {
                        vecs.push(AppModel::new(name,icon_path));
                    }
                }
            }
        }
    }
}

fn get_app_info(app_path: &Path) -> Option<(String, PathBuf)> {
    let info_plist_path = app_path.join("Contents/Info.plist");
    if let Ok(plist) = plist::Value::from_file(&info_plist_path) {
        if let Value::Dictionary(dict) = plist {
            if let Some(Value::String(name)) = dict.get("CFBundleName") {
                let icon_file = dict.get("CFBundleIconFile").and_then(|value| {
                    if let Value::String(icon_name) = value {
                        Some(icon_name.clone())
                    } else {
                        None
                    }
                });

                if let Some(icon_file) = icon_file {
                    let icon_path = app_path
                        .join("Contents/Resources")
                        .join(icon_file)
                        .with_extension("icns");
                    return Some((name.clone(), icon_path));
                }
            }
        }
    }
    None
}