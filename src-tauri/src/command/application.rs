use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
#[cfg(target_os = "macos")]
use plist::Value;
use serde::{Deserialize, Serialize};
#[cfg(target_os = "macos")]
use tauri_icns::{IconFamily, IconType};

#[tauri::command]
pub async  fn app_list() -> anyhow::Result<Vec<AppModel>, ()> {

    #[cfg(target_os = "windows")]
    let applications = vec![];

    #[cfg(target_os = "macos")]
    let applications = handle_macos_application().await;
    Ok(applications)
}

#[cfg(target_os = "macos")]
async fn handle_macos_application() -> Vec<AppModel> {
    let mut apps:Vec<AppModel> = vec![];
    // list_applications("/Applications", &mut apps);
    // list_applications("/System/Applications", &mut apps);
    // list_applications("/System/Library/CoreServices", &mut apps);

    let tasks = apps.into_iter().map(|item| {
        let icon = item.icon.clone();
        tokio::spawn(async move {
            return if let Ok(file) = File::open(icon.clone()) {
                if let Some(last) = icon.clone().to_str().unwrap().split("/").last() {
                    let split = last.split(".").collect::<Vec<&str>>();

                    let file = BufReader::new(file);
                    let icon_family = IconFamily::read(file).unwrap();

                    // Extract an icon from the family and save it as a PNG.
                    let image = icon_family.get_icon_with_type(IconType::RGBA32_512x512_2x).unwrap();
                    // let file = BufWriter::new(File::create(
                    //     format!("../public/{}.png", split[0]))
                    //     .unwrap());
                    // image.write_png(file).unwrap();
                    let model = item.update_icon_base(image.data().to_vec());
                    return Some(model)
                }
                None
            } else {
                None
            }
        })
    });

    let response = futures::future::join_all(tasks).await;
    let result: Vec<AppModel> = response
        .into_iter()
        .filter_map(|result| match result {
            Ok(Some(model)) => Some(model),
            _ => None,
        })
        .collect();
    result
}


#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all="camelCase")]
pub struct AppModel {
    name: String,
    icon: PathBuf,
    icon_base: Vec<u8>
}

impl AppModel {
    fn new(name: String, icon: PathBuf) -> Self {
        Self {
            name,
            icon,
            icon_base: vec![]
        }
    }

    fn update_icon_base(self,base_url: Vec<u8>) -> Self{
        Self {
            icon_base: base_url,
            name: self.name,
            icon: self.icon
        }
    }
}

#[cfg(target_os = "macos")]
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

#[cfg(target_os = "macos")]
fn get_app_info(app_path: &Path) -> Option<(String, PathBuf)> {
    let info_plist_path = app_path.join("Contents/Info.plist");
    if let Ok(plist) = Value::from_file(&info_plist_path) {
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