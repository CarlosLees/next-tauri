use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use base64::{alphabet, engine, Engine};
use base64::engine::general_purpose;
use plist::Value;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()>{
    #[cfg(target_os = "macos")]
        let mut apps:Vec<AppModel> = vec![];

    list_applications("/Applications",&mut apps);
    list_applications("/System/Applications", &mut apps);
    list_applications("/System/Library/CoreServices", &mut apps);

    let tasks = apps.into_iter().map(|item| {
        let icon = item.icon.clone();
        tokio::spawn(async move {
            if let Ok(mut file) = File::open(icon) {
                let mut buffer = Vec::new();

                // 读取文件内容
                file.read_to_end(&mut buffer).unwrap();
                // 将字节数组转换为 Base64
                const CUSTOM_ENGINE: engine::GeneralPurpose =
                    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

                let b64_url = CUSTOM_ENGINE.encode(buffer);
                let model = item.update_icon_base(b64_url);
                return Some(model)
            } else {
                return None;
            }
        })
    });

    let response = futures::future::join_all(tasks).await;
    Ok(())
}


#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all="camelCase")]
pub struct AppModel {
    name: String,
    icon: PathBuf,
    icon_base: String
}

impl AppModel {
    fn new(name: String, icon: PathBuf) -> Self {
        Self {
            name,
            icon,
            icon_base: String::new()
        }
    }

    fn update_icon_base(self,base_url: String) -> Self{
        Self {
            icon_base: base_url,
            name: self.name,
            icon: self.icon
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