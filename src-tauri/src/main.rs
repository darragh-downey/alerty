#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::error::Error;
use rss::Channel;

struct Database;

#[derive(serde::Serialize)]
struct CustomResponse {
    message: String,
    other_val: usize,
}


fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![load_feed])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


async fn oval_feed() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get("https://oval.mitre.org/repository/data/rss")
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}


async fn some_other_function() -> Option<String> {
    Some("response".into())
}

#[tauri::command]
async fn load_feed(
    window: tauri::Window,
    number: usize,
    database: tauri::State<'_, Database>,
    ) -> Result<CustomResponse, String> {
    println!("Called from {}", window.label());
    
    // Need to process the data from the channel before passing it to the 
    // Result<CustomResponse, String>
    // let result: Result<Channel, Box<dyn Error>> = oval_feed().await;

    let result: Option<String> = some_other_function().await;

    if let Some(message) = result { 
        Ok(CustomResponse {
            message,
            other_val: 42,
        })
    }else {
        Err("No result".into())
    }
}
