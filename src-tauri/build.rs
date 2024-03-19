fn main() {
    tauri_build::build(); // 保持 Tauri 的构建逻辑
    // 添加下面的代码来链接 sqlite3
    println!("cargo:rustc-link-search=native=D:\\Projects\\app\\dev\\PathLinker");
    println!("cargo:rustc-link-lib=sqlite3");
}
