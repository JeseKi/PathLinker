#![cfg(test)]
use libs::{crud, utils, hard_link_create};

#[test]
fn test() {
    let conn = crud::connect_db();
    let path = "/home/Jese__Ki/Downloads/icon.png";
    println!("Received selected path: {:?}", path);

        // 获取文件路径
        if let Some(file_name) = std::path::Path::new(&path).file_name().and_then(|f| f.to_str()) {
            let random_url = utils::generate_random_url();
            let mut _hard_link = String::from("");
            match hard_link_create(&path) {
                Ok(link) => {
                    println!("Hard link created for file: {}", file_name);
                    _hard_link = link.clone()
                },
                Err(e) => println!("Failed to create hard link: {}", e),
            }
            println!("Generated URL: {} for file: {}", random_url, file_name);
            // 创建映射
            match crud::create_mapping(&conn, file_name, &path, &random_url, &_hard_link){
                Ok(_) => println!("Mapping created for file: {}", file_name),
                Err(e) => eprintln!("Failed to create mapping for file: {}: {}", file_name, e),
            }
        } else {
            eprintln!("Failed to extract file name from path: {}", path);
        }
}