use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct ExeInfo {
    pub app_name: String,
    pub path: String,
    pub switch_status: bool,
    pub children: Option<Vec<PathInfo>>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct PathInfo {
    pub app_name: String,
    pub path: String,
    pub switch_status: bool,
}