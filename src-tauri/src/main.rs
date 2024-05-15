#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use homedir::get_my_home;
use serde::{Deserialize, Serialize};

#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct Item {
   is_file:bool,
   name:String,
}

#[tauri::command]
fn get_home_path() -> String {
 return get_my_home().unwrap().unwrap().to_str().unwrap().to_string();
}

#[tauri::command]
fn get_all_ff(path: String) -> Vec<Item> {
   let mut list:Vec<Item> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(path){
     for entry in entries {
       if let Ok(entry) = entry{
          let is_file = entry.file_type().unwrap().is_file();
          let name = entry.file_name().to_str().unwrap().to_string();
          let item = Item{is_file,name};
          list.push(item);
       }
     }
   }
    return list;
}


fn main() {
tauri::Builder::default()
   .invoke_handler(tauri::generate_handler![get_home_path,get_all_ff])
   .run(tauri::generate_context!())
   .expect("error while running tauri application");
}