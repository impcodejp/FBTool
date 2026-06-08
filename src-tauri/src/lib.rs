mod commands;
mod domain;
mod error;
mod infrastructure;
mod usecase;

use commands::{
    generate::generate_fb,
    header::{export_header_info, import_header_info},
    template::export_csv_template,
    validate::read_csv_records,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            generate_fb,
            read_csv_records,
            export_csv_template,
            export_header_info,
            import_header_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
