#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            // The web content always updates itself (it lives on the server).
            // This only updates the shell binary: window, icons, native code.
            let handle = app.handle().clone();
            std::thread::spawn(move || {
                tauri::async_runtime::block_on(async move {
                    if let Err(err) = check_for_update(handle).await {
                        eprintln!("[updater] {err}");
                    }
                });
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running the Shotlist shell");
}

async fn check_for_update(app: tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
    use tauri_plugin_updater::UpdaterExt;

    let Some(update) = app.updater()?.check().await? else {
        return Ok(());
    };

    let yes = app
        .dialog()
        .message(format!(
            "Shotlist {} is available (you have {}).\n\nDownload it now? The app will restart.",
            update.version, update.current_version
        ))
        .title("Update available")
        .kind(MessageDialogKind::Info)
        .buttons(MessageDialogButtons::OkCancelCustom("Update now".into(), "Later".into()))
        .blocking_show();

    if !yes {
        return Ok(());
    }

    update
        .download_and_install(|_chunk, _total| {}, || {})
        .await?;

    app.restart();
}
