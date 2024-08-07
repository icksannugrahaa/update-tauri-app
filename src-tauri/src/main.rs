// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::Manager;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct UpdateResponse {
    update_available: bool,
    message: String,
}

#[tauri::command]
async fn check_update(app_handle: tauri::AppHandle) -> Result<UpdateResponse, String> {
    let updater = app_handle.updater();
    match updater.check().await {
        Ok(update) => {
            if update.is_update_available() {
                Ok(UpdateResponse {
                    update_available: true,
                    message: "Update available".to_string(),
                })
            } else {
                Ok(UpdateResponse {
                    update_available: false,
                    message: "No update available".to_string(),
                })
            }
        },
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn perform_update(app_handle: tauri::AppHandle) -> Result<(), String> {
    let updater = app_handle.updater();
    match updater.check().await {
        Ok(update) => {
            if update.is_update_available() {
                update.download_and_install().await.map_err(|e| e.to_string())
            } else {
                Err("No update available".to_string())
            }
        },
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            std::thread::spawn(move || {
                loop {
                    // Check for updates every hour
                    std::thread::sleep(std::time::Duration::from_secs(3600));
                    let update_result = tauri::async_runtime::block_on(check_update(app_handle.clone()));
                    match update_result {
                        Ok(response) => {
                            if response.update_available {
                                // Notify the user or automatically install
                                if let Err(e) = tauri::async_runtime::block_on(perform_update(app_handle.clone())) {
                                    eprintln!("Failed to perform update: {}", e);
                                }
                            }
                        },
                        Err(e) => eprintln!("Error checking for updates: {}", e),
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![check_update, perform_update])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}