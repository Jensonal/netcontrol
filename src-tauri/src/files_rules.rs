use std::{fs, thread};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use crate::exe_info_struct::{ExeInfo, PathInfo};
use crate::new_rules::{add_rules, delete_rules};

pub fn visit_dirs(dir: &Path, exe_info: Arc<Mutex<ExeInfo>>) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir() {
        let mut handles = vec![];

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let dir_name = path.file_name().unwrap().to_string_lossy();
                // println!("{}", dir_name);

                // 排除文件夹
                if dir_name == "steamapps" { continue }

                let exe_info_clone = Arc::clone(&exe_info);
                let handle = thread::spawn(move || {
                    visit_dirs(&path, exe_info_clone).unwrap_or_else(|e| eprintln!("Error: {}", e));
                });
                handles.push(handle);
            } else if let Some(extension) = path.extension() {
                if extension == "exe" {
                    let mut exe_info = exe_info.lock().unwrap();
                    // 如果 children 为 None，初始化为一个空的 Vec
                    if exe_info.children.is_none() {
                        exe_info.children = Some(Vec::new());
                    }
                    exe_info.children.get_or_insert(vec![]).push(PathInfo {
                        app_name: "".to_string(),
                        path: path.to_string_lossy().to_string(),
                        switch_status: false,
                    });
                }
            }
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    Ok(())
}


pub async fn process_single_path(opt: String, path: PathBuf) -> Result<(), String> {

    let name = format!("blocked {} via script", path.display());
    for dir in &["in", "out"] {
        if opt == "add" {
            add_rules(name.clone(), dir, path.display()).expect("add_fails");
        } else {
            delete_rules(name.as_str()).expect("delete_rules_fails");
        }
    }

    Ok(())
}

pub fn process_path_item(path_str: &str) {
    let path = PathBuf::from(path_str);
    // println!("{:?}", path);

    tokio::spawn(async move {
        process_single_path("delete".to_string(), path)
            .await
            .expect("process_single_path_fails");
    });
}