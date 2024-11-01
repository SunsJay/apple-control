/*
 * @Date: 2024-11-01 12:48:18
 * @LastEditors: sunsjay sunsjay0806@gmail.com
 * @LastEditTime: 2024-11-01 15:40:48
 * @FilePath: /apple-control/src-tauri/src/lib.rs
 * @Description:
 */
pub mod utils;
pub mod vm;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .invoke_handler(tauri::generate_handler![
            utils::get_env_var,
            vm::vmrun_list,
            vm::vmrun_clone
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
