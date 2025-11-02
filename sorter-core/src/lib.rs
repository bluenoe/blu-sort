use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{anyhow, Result};
use chrono::Utc;
use fs_extra::file::CopyOptions;
use serde::{Serialize, Deserialize};

/// Đại diện cho một hành động di chuyển file
#[derive(Clone, Serialize, Deserialize)]
pub struct TransactionItem {
    pub from: String,
    pub to: String,
    pub reason: Option<String>,
    pub duplicate: bool,
}

/// Đại diện cho một transaction (nhiều hành động)
#[derive(Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub actions: Vec<TransactionItem>,
    pub when: chrono::DateTime<chrono::Utc>,
}

/// Copy file fallback khi rename thất bại
pub fn move_file(from: &Path, to: &Path, opts: &CopyOptions) -> Result<()> {
    fs_extra::copy_items(&[from], to, opts)?;
    fs::remove_file(from)?;
    Ok(())
}

/// Áp dụng kế hoạch sắp xếp: di chuyển file theo items
pub fn apply_plan(root: &Path, items: &[TransactionItem], opts: &CopyOptions) -> Result<Transaction> {
    for it in items {
        let from = root.join(&it.from);
        let mut to = root.join(&it.to);

        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent)?;
        }

        if to.exists() {
            // conflict: thêm hậu tố -1, -2,...
            let mut i = 1;
            let stem = to.file_stem().unwrap().to_string_lossy().to_string();
            let ext = to
                .extension()
                .map(|e| format!(".{}", e.to_string_lossy()))
                .unwrap_or_default();

            loop {
                let cand = to.parent().unwrap().join(format!("{}-{}{}", stem, i, ext));
                if !cand.exists() {
                    to = cand;
                    break;
                }
                i += 1;
            }
        }

        // Thử rename, nếu lỗi thì fallback sang copy + remove
        match fs::rename(&from, &to) {
            Ok(_) => {},
            Err(_) => {
                move_file(&from, &to, &opts)?; 
            }
        }
    }

    let txn = Transaction { actions: items.to_vec(), when: Utc::now() };
    write_txn(root, &txn)?;
    Ok(txn)
}

/// Hoàn tác (undo) transaction cuối cùng
pub fn undo_last(root: impl AsRef<Path>) -> Result<usize> {
    let root = root.as_ref();
    let (txn_path, txn) = read_last_txn(root)?;
    let mut undone = 0usize;

    for it in txn.actions.iter().rev() {
        let from = root.join(&it.to);
        let to = root.join(&it.from);

        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent)?;
        }

        if from.exists() {
            match fs::rename(&from, &to) {
                Ok(_) => undone += 1,
                Err(_) => {
                    let mut opts = CopyOptions::new();
                    opts.overwrite = true;
                    move_file(&from, &to, &opts)?;
                    undone += 1;
                }
            }
        }
    }

    // Lưu transaction vào thư mục archived
    if let Some(parent) = txn_path.parent() {
        fs::create_dir_all(parent.join("archived"))?;
    }
    let arch = txn_path.parent().unwrap().join("archived").join(txn_path.file_name().unwrap());
    let _ = fs::rename(&txn_path, &arch);

    Ok(undone)
}

/// Ghi transaction vào file .blusorter/txn-<timestamp>.json
fn write_txn(root: &Path, txn: &Transaction) -> Result<()> {
    let dir = root.join(".blusorter");
    fs::create_dir_all(&dir)?;
    let name = format!("txn-{}.json", txn.when.format("%Y%m%d-%H%M%S"));
    let path = dir.join(name);
    fs::write(path, serde_json::to_vec_pretty(txn)?)?;
    fs::write(dir.join("last"), b"ok")?;
    Ok(())
}

/// Đọc transaction gần nhất từ .blusorter/
fn read_last_txn(root: &Path) -> Result<(PathBuf, Transaction)> {
    let dir = root.join(".blusorter");
    let mut newest: Option<PathBuf> = None;

    for e in fs::read_dir(&dir)? {
        let e = e?;
        let p = e.path();

        if p.extension().and_then(|s| s.to_str()) == Some("json") {
            if newest.is_none() {
                newest = Some(p);
            } else if let Ok(md) = fs::metadata(&p) {
                let np = newest.as_ref().unwrap();
                let nmd = fs::metadata(np)?;
                if md.modified()? > nmd.modified()? {
                    newest = Some(p);
                }
            }
        }
    }

    let path = newest.ok_or_else(|| anyhow!("No transaction found"))?;
    let data = fs::read(&path)?;
    let txn: Transaction = serde_json::from_slice(&data)?;
    Ok((path, txn))
}
