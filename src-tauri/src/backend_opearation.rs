// 处理传入程序的参数
pub mod process_software_params {
    use rusqlite::Connection;

    use libs::{
        crud::{mappings::get_hard_link_by_url , connect_db},
        utils::{log_to_file , LogType}
    };
    // 根据URL打开对应映射的文件
    pub fn open_file(conn: &Connection, url: &String) -> Result<(), String> {
        // 从URL映射中获取对应的文件路径
        let path: String = get_hard_link_by_url(conn, url);
        // 记录日志信息(当前已注释掉)
        // log_to_file(&format!("Attempting to open file: {}", &path));
        // 打开文件
        let flag: Result<(), std::io::Error> = open::that(&path);

        // 处理打开文件的结果
        match flag {
            // 成功打开文件
            Ok(..) => Ok(()),
            // 打开文件失败
            Err(e) => {
                println!("Failed to open the file: {}", e);
                // 返回错误信息
                Err(e.to_string())
            }
        }
    }

    // 处理自定义协议
    pub fn handle_custom_protocol (args: Vec<String>) -> () {
        // 检查参数长度是否大于1, 如果是, 说明接收到了URL参数
        if args.len() > 1 {
            // 获取第二个参数, 即URL
            let url: &String = &args[1];
            // 记录接收到的URL到日志文件
            // log_to_file(&format!("Received URL: {}", url));
            
            // 连接数据库
            let conn: Connection = connect_db();
            
            // 打印URL信息, 用于调试
            // println!("url:{url}");
            // 记录尝试打开URL的日志信息
            // log_to_file(&format!("Attempting to open URL: {}", url));
            
            // 尝试打开URL
            let flag: Result<(), String> = open_file(&conn, url);
            
            // 根据打开URL的结果进行处理
            match flag {
                Ok(..) => {
                    // 记录URL打开成功的日志信息
                    log_to_file("URL opened successfully.", None, LogType::Info);
                    // 退出程序, 状态码为0
                    std::process::exit(0);
                },
                Err(..) => {
                    // 记录URL打开失败的日志信息
                    log_to_file("Failed to open URL.", None, LogType::Info);
                    // 退出程序, 状态码为1
                    std::process::exit(1);
                },
            }
        }
    }
}

// 映射部分
pub mod mapping_api{
    use tauri::State;
    use std::fs;
    use std::path::PathBuf;

    use libs::{crud, db::{self, mapping_base_crud::Mapping}, path_struct, hard_link_create};
    use libs::utils::AppState;
    // 创建新映射
    #[tauri::command]
    pub async fn handle_selected_path(state: State<'_, AppState>, selected: Vec<String>) -> Result<(), String>{
        // 用于对前端所选中的文件进行提取路径，并生成随机的URL来进行路径的映射
        println!("收到文件: {:?}", selected);

        for path in selected.clone() {
            // 获取文件路径和随机URL
            if let Some(file_name) = std::path::Path::new(&path).file_name().and_then(|f| f.to_str()) {
                let random_url = path_struct::generate_random_url();
                let mut _hard_link = String::from("");
                match hard_link_create(&path) {
                    Ok(link) => {
                        println!("已为该文件创建硬链接: {}", file_name);
                        _hard_link = link.clone()
                    },
                    Err(e) => return Err(format!("Failed to create hard link: {}", e)),
                }
                println!("生成随机URL: {} for file: {}", random_url, file_name);
                let conn = state.conn.lock().unwrap();
                // 创建映射
                match crud::mappings::create_mapping(&*conn, file_name, &path, &random_url, &_hard_link){
                    Ok(_) => println!("已创建映射文件: {}", file_name),
                    Err(e) => eprintln!("创建映射文件失败: {}: {}", file_name, e),
                }
            } else {
                eprintln!("无法从路径中提取文件名: {}", path);
            }
        }

        Ok(())
    }

    // 用于让前端获取全部映射
    #[tauri::command]
    pub async fn get_mappings(state: State<'_, AppState>) -> Result<Vec<Mapping>, String> {
        let conn = state.conn.lock().unwrap();

        let mappings = db::mapping_base_crud::get_all_mappings(&*conn);

        match mappings {
            Ok(..) => mappings,
            Err(e) => Err(e.to_string())
        }

    }

    // 根据URL删除映射和对应的硬链接
    #[tauri::command]
    pub async fn delete_mapping(state: State<'_, AppState>, url: String) -> Result<(), String> {
        let conn = state.conn.lock().unwrap();

        // 首先尝试获取硬链接的路径
        let hard_link_path = match db::mapping_base_crud::get_hard_link_by_url(&*conn, &url) {
            Ok(path) => path,
            Err(e) => return Err(e.to_string()),
        };

        // 删除数据库中的映射
        match db::mapping_base_crud::delete_mapping_by_url(&*conn, &url) {
            Ok(..) => (),
            Err(e) => return Err(e.to_string()),
        };

        // 然后删除硬链接文件
        if let Some(path) = hard_link_path {
            let path = PathBuf::from(path);
            if path.exists() {
                if let Err(e) = fs::remove_file(&path) {
                    return Err(e.to_string());
                }
                else {
                    println!("文件删除成功: {} ", path.display())
                }
            }
        }

        Ok(())
    }

}

// 文件夹部分
// deving
mod folders_api{
    use tauri::State;
    use serde::{Serialize, Deserialize};

    use libs::{
        db::{
            folder_base_crud as crud,
            mapping_base_crud as mp_crud,
            mapping_base_crud::Mapping
        },
        utils::AppState
    };    

    #[tauri::command]
    pub async fn _get_folders (state: State<'_, AppState>) -> Result<Vec<crud::Folders>, String> {

        let conn = state.conn.lock().unwrap();
        let folders = crud::get_all_folders(&conn);
        match folders {
            Ok(folders) => return Ok(folders),
            Err(e) => return Err(e.to_string())
        }

    }

    #[tauri::command]
    pub async fn _create_folder (state: State<'_, AppState> , folder_name: String) -> Result<(), String> {

        let conn = state.conn.lock().unwrap();
        let flag = crud::create_folder(&conn, &folder_name, None);
        println!("后端接口处接收到文件夹名:{}", folder_name);
        match flag {
            Ok(row) => {
                if row > 0 {
                    Ok(())
                }
                else {
                    Err("创建文件夹失败".to_string())
                }
            },
            Err(e) => Err(e.to_string())
        }
    }

    #[tauri::command]
    pub async fn _get_folder_mappings_by_foldername(state: State<'_, AppState>, folder_name: String) -> Result<Vec<Mapping>, rusqlite::Error> {
        let conn = state.conn.lock().unwrap();
        let mapping_ids = crud::get_mappings_for_folder(&conn, &folder_name)?;
    
        let mut mappings: Vec<Mapping> = vec![];
    
        for id in mapping_ids {
            let mapping = mp_crud::get_mapping_by_id(&conn, id)?;
            if let Some(mapping) = mapping {
                mappings.push(mapping);
            }
        }
    
        Ok(mappings)
    }    
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FolderWithMappings {
        folder_name: String,
        mappings: Vec<Mapping>,
    }
    
}
