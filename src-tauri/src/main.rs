// src-tauri/src/main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct PreviewResp {
    items: Vec<sorter_core::PlanItem>,
    total: usize,
    duplicates: usize,
}

#[tauri::command]
async fn cmd_scan_preview(dir: String) -> Result<PreviewResp, String> {
    let cfg_path = std::path::Path::new("rules/default.yaml");
    let cfg = sorter_core::load_config(cfg_path).map_err(|e| e.to_string())?;
    let (items, sum) = sorter_core::scan_preview(&dir, &cfg).map_err(|e| e.to_string())?;
    Ok(PreviewResp {
        items,
        total: sum.total,
        duplicates: sum.duplicates,
    })
}

#[derive(Debug, Deserialize)]
struct ApplyReq {
    dir: String,
    items: Vec<sorter_core::PlanItem>,
}

#[tauri::command]
async fn cmd_apply(req: ApplyReq) -> Result<usize, String> {
    let n = req.items.len();
    sorter_core::apply_plan(&req.dir, &req.items).map_err(|e| e.to_string())?;
    Ok(n)
}

#[tauri::command]
async fn cmd_undo(dir: String) -> Result<usize, String> {
    sorter_core::undo_last(&dir).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![cmd_scan_preview, cmd_apply, cmd_undo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
