#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use homedir::get_my_home;

#[tauri::command]
fn get_home_path() -> String {
 return get_my_home().unwrap().unwrap().to_str().unwrap().to_string();
}

#[tauri::command]
fn greet() -> String {
 "Hello From Rust".into()
}

fn main() {
tauri::Builder::default()
   .invoke_handler(tauri::generate_handler![greet,get_home_path])
   .run(tauri::generate_context!())
   .expect("error while running tauri application");
}