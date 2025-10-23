use tauri::Manager;

mod manager;
mod r2;
mod typ;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init());

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }));
    }

    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    let builder = builder.plugin(tauri_plugin_clipboard::init());

    builder
        .invoke_handler(tauri::generate_handler![
            manager::preview_file,
            manager::get_file_details,
            r2::r2_ping,
            r2::r2_upload,
            r2::r2_cancel_upload,
            r2::r2_list_objects,
            r2::r2_list_multipart_uploads,
            r2::r2_delete_object,
            r2::r2_abort_multipart_upload_cmd,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
