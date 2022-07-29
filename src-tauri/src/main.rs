#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::error::Error;
use rss::Channel;

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![oval_feed])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tauri::command]
async fn oval_feed() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get("https://oval.mitre.org/repository/data/rss")
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
