#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod handler;
mod model;

use std::sync::Mutex;
use rusqlite::Connection;
use std::path::PathBuf;
use crate::handler::{get_orders, create_order, create_product, get_products, get_order_sequence};

#[derive(Clone)]
pub struct AppState {
    db: std::sync::Arc<Mutex<Connection>>,
}

fn main() {
    // Get app data directory (cross-platform compatible)
    let app_dir = get_app_data_dir();
    std::fs::create_dir_all(&app_dir).ok();

    // Initialize database
    let db_path = app_dir.join("pos_system.db");

    let conn = db::init_db(&db_path)
        .expect("failed to initialize database");

    let state = AppState {
        db: std::sync::Arc::new(Mutex::new(conn)),
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            create_order,
            get_orders,
            create_product,
            get_products,
            get_order_sequence,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Get app data directory - cross-platform
fn get_app_data_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        let app_data = std::env::var("APPDATA")
            .unwrap_or_else(|_| std::env::var("USERPROFILE").unwrap_or_default());
        PathBuf::from(app_data).join("restaurant-pos")
    }

    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").expect("HOME not set");
        PathBuf::from(home).join("Library/Application Support/restaurant-pos")
    }

    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME").expect("HOME not set");
        let xdg_data = std::env::var("XDG_DATA_HOME")
            .unwrap_or_else(|_| format!("{}/.local/share", home));
        PathBuf::from(xdg_data).join("restaurant-pos")
    }
}
