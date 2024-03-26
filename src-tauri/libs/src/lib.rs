pub mod utils;
pub mod db;
pub mod crud;

mod path_struct;

use std::fs;
use std::path::Path;
use whoami;

pub fn hard_link_create(path: &str) -> std::io::Result<String> {
    let user_name = whoami::username();
    let original_path = Path::new(path);
    println!("源路径:{}", original_path.display());

    if !original_path.exists() {
        eprintln!("指定的文件不存在: {}", original_path.display());
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "文件不存在"));
    }

    // 将 Path 转换为 PathBuf
    let original_path_buf = original_path.to_path_buf();

    // 现在传递 PathBuf 的引用
    let hard_link_path = path_struct::build_hard_link_path(&original_path_buf, &user_name);

    println!("即将创建的硬链接: {:?}", hard_link_path);

    // 创建硬链接的目录（如果不存在）
    if let Some(parent) = hard_link_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::hard_link(&original_path, &hard_link_path)?;

    println!("硬链接创建成功: {}", hard_link_path.display());
    Ok(hard_link_path.display().to_string())
}
