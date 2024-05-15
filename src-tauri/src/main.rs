#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::DirEntry;

use homedir::get_my_home;
use serde::{Deserialize, Serialize};
use serde_json::map::Entry;

#[derive(Clone,Debug,Deserialize,Serialize)]
pub enum FileTypes{
   File,
   Folder,
   Symlink,
}

impl FileTypes{
   pub fn get_file_type(entry:&DirEntry) -> Self{
      if entry.file_type().unwrap().is_file(){
       return FileTypes::File;  
     }else if entry.file_type().unwrap().is_dir(){
      return FileTypes::Folder;
     }
   return FileTypes::Symlink;
  }
}

#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct Item {
   name:String,
   file_type:FileTypes,
   hidden:bool,
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
          let file_type = FileTypes::get_file_type(&entry);
          let name = entry.file_name().to_str().unwrap().to_string();
          let hidden = name.chars().collect::<Vec<char>>()[0] == '.';
          let item = Item{name,file_type,hidden};
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