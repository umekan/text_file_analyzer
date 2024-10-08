// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod text_file_analyzer;
mod db_control;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            db_control::set_local_db_file_path(app.path_resolver().app_data_dir().unwrap());
            // データベースの初期化処理を呼び出す
            if let Err(e) = db_control::create_db_and_table_if_needed() {
                eprintln!("Failed to create database and table: {}", e);
            } else {
                println!("Database and table created successfully. Path: {}", db_control::LOCAL_DB_FILE_PATH.read().clone());
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            text_file_analyzer::file_concat,
            text_file_analyzer::extract_characters,
            db_control::insert_diary,
            db_control::get_all_diaries,
            db_control::update_diary,
            db_control::insert_tag,
            db_control::get_tag_list,
            db_control::insert_diary_tag_relation,
            db_control::get_all_diary_tag_relations,
            db_control::get_diaries_by_tag_id,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
