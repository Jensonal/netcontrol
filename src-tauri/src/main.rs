// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod new_rules;
mod exe_info_struct;
mod files_rules;

use files_rules::*;
use std::path::PathBuf;
use rfd::FileDialog;

use tokio::task;

use std::path::Path;
use std::sync::{Arc, Mutex};
use crate::exe_info_struct::{ExeInfo, PathInfo};

#[tokio::main]
async fn main() {


    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![select_folder,select_exe,clear_all,operate_rules_from_paths])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
#[tauri::command]
async fn clear_all(exe_info: Vec<ExeInfo>) {
    println!("清理开始");

    exe_info.into_iter()
        .filter(|exe_item| exe_item.switch_status)
        .flat_map(|exe_item| {
            let mut paths = Vec::new(); // 初始化一个空的向量
            // 检查 exe_item 的路径是否以 ".exe" 结尾
            if exe_item.path.ends_with(".exe") {
                // 如果是，将路径添加到 paths 向量
                paths.push(exe_item.path);
            }
            if let Some(children) = exe_item.children {
                paths.extend(children.into_iter().filter_map(|child| {
                    if child.switch_status {
                        Some(child.path)
                    } else {
                        None
                    }
                }));
            }
            paths
        })
        .for_each(|path| {
            // 这里调用处理路径的方法
            process_path_item(&path);
        });
}
#[tauri::command]
async fn operate_rules_from_paths(opt: String, paths: Vec<PathInfo>) -> Result<(), String> {
    // println!("指定exe{:?}", paths);
    let mut handles = vec![];

    for exe_path in paths {
        let path = PathBuf::from(&exe_path.path.as_str());

        // Clone the path to be moved into the spawned task
        let cloned_path = path.clone();
        let cloned_opt =opt.clone();
        let handle = task::spawn(async move {

            process_single_path(cloned_opt, cloned_path).await.expect("process_single_path_fails");
        });

        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.map_err(|_| "Failed to await task".to_string())?;
    }

    Ok(())
}

#[tauri::command]
fn select_exe() -> Vec<ExeInfo> {
    println!("指定exe");
    // 选取文件
    let mut exe_info_list = Vec::new();
    if let Some(ss) = FileDialog::new()
        .add_filter("", &["exe"])
        .set_directory("/")
        .pick_file(){
        println!("{}",ss.display());
        // add_rules(&ss)
        // let root_dir = Path::new(folder_path.as_path());
        let exe_name = ss.file_name().unwrap().to_string_lossy();
        println!("{}",exe_name);
        let exe_info = ExeInfo {
            app_name: exe_name.parse().unwrap(),
            path: ss.to_string_lossy().to_string(),
            switch_status: false,
            children: Some(Vec::new()),
        };
        exe_info_list = vec![exe_info];
    }
    exe_info_list
}

#[tauri::command]
fn select_folder() -> Vec<ExeInfo> {

    let mut exe_info_list = Vec::new();

    // 选取文件夹
    if let Some(folder_path) = FileDialog::new().pick_folder() {
        let root_dir = Path::new(folder_path.as_path());

        let dir_name = root_dir.file_name().unwrap().to_string_lossy();

        let exe_info = Arc::new(Mutex::new(ExeInfo {
            app_name: dir_name.parse().unwrap(),
            path: root_dir.to_string_lossy().to_string(),
            switch_status: false,
            children: None,
        }));
        if let Err(err) = visit_dirs(&root_dir, exe_info.clone()) {
            eprintln!("Error: {}", err);
        }
        let exe_info = Arc::try_unwrap(exe_info).ok().unwrap().into_inner().unwrap();
        // println!("ExeInfo: {:#?}", exe_info);

        exe_info_list = if let Some(children) = &exe_info.children {
            if children.is_empty() {
                println!("children为空");
                Vec::new()
            } else {
                println!("children不为空");
                vec![exe_info]
            }
        } else {
            println!("children为None");
            Vec::new()
        };

        // exe_info_list = vec![exe_info];
    }
    // println!("ExeInfo: {:#?}", exe_info_list);

    exe_info_list
}