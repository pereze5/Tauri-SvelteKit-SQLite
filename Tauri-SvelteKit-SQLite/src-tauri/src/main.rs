// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;

#[tokio::main]
async fn main() {
    let pool = db::init_db().await;

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(pool)
        .invoke_handler(tauri::generate_handler![
            commands::create_project,
            commands::list_projects,
            commands::set_active_project,
            commands::get_active_project,
            commands::create_counter,
            commands::list_counters,
            commands::increment_counter,
            commands::start_session,
            commands::end_session,
            commands::list_sessions,
            commands::add_pattern,
            commands::list_patterns,
            commands::delete_pattern,
            commands::update_pattern_position,
            commands::add_inventory_item,
            commands::list_inventory_items,
            commands::delete_inventory_item,
            commands::link_inventory_to_project,
            commands::list_project_inventory,
            commands::remove_project_inventory_link,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
