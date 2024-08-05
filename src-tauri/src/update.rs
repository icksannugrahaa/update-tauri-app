use tauri::Manager;
use tauri::updater::Update;

#[tauri::command]
async fn check_update(app_handle: tauri::AppHandle) -> Result<Update, String> {
    let update = app_handle.updater().check().await;
    match update {
        Ok(update) => Ok(update),
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    // Check for updates every hour
                    tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
                    if let Ok(update) = check_update(app_handle.clone()).await {
                        if update.is_update_available() {
                            // Notify the user or automatically install
                            app_handle.updater().install(update).unwrap();
                        }
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![check_update])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}