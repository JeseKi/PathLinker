use rdev::{listen, Event, EventType, Key};
use rfd::FileDialog;
use arboard::Clipboard;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use once_cell::sync::Lazy;

use libs::utils::{log_to_file , LogType };

static mut SHIFT_PRESSED: bool = false;
static mut CTRL_PRESSED: bool = false;
static FILE_PATH: Lazy<Mutex<Vec<PathBuf>>> = Lazy::new(|| {
    Mutex::new(vec![])
});
static CREATED_PATHS: Lazy<Mutex<Vec<PathBuf>>> = Lazy::new(|| {
    Mutex::new(vec![])
});

// 启动剪贴板键盘监听器
pub fn start_clipboard_key_listener() {
    // 创建剪贴板和上一次内容的共享锁
    let clipboard = Arc::new(Mutex::new(Clipboard::new().unwrap()));
    let last_content = Arc::new(Mutex::new(String::new()));

    // 监听剪贴板变化的线程
    // 这个线程会定期检查剪贴板内容是否发生变化,如果变化则记录到日志中
    thread::spawn({
        let clipboard = Arc::clone(&clipboard);
        let last_content = Arc::clone(&last_content);
        move || {
            loop {
                let mut clip = clipboard.lock().unwrap();
                let mut last = last_content.lock().unwrap();
                match clip.get_text() {
                    Ok(content) if content != *last => {
                        // 剪贴板内容已更改,记录到日志
                        log_to_file(
                            &format!("剪贴板内容已变更: {}", content), 
                            None, 
                            LogType::Debug
                        );
                        // 尝试将剪贴板内容存储到FILE_PATH_MAP中
                        let new_file_path_map = quickly_create_mapping_by_clipboard::handle_clipboard_content(&content);
                        common::update_file_path_map(new_file_path_map);
                        *last = content;
                    },
                    Ok(_) => {},
                    Err(e) => log_to_file(
                        &format!("读取剪贴板时发生错误: {}", e), 
                        None, 
                        LogType::Warning
                    ),
                }
                // 每500毫秒检查一次剪贴板
                thread::sleep(Duration::from_millis(500));
            }
        }
    });
    // 监听键盘事件的线程
    // 这个线程会监听键盘事件,如果按下Ctrl+Shift+C,则打开文件选择器,选择要创建映射的文件
    // 如果按下Ctrl+Shift+V,则从剪切板进行创建映射
    thread::spawn(move || {
        let callback = move |event: Event| {

            unsafe {
                match event.event_type {
                    EventType::KeyPress(key) => match key {
                        Key::ShiftLeft | Key::ShiftRight => SHIFT_PRESSED = true,
                        Key::ControlLeft | Key::ControlRight => CTRL_PRESSED = true,
                        _ => {}
                    },
                    EventType::KeyRelease(key) => match key {
                        Key::ShiftLeft | Key::ShiftRight => SHIFT_PRESSED = false,
                        Key::ControlLeft | Key::ControlRight => CTRL_PRESSED = false,
                        _ => {}
                    },
                    _ => {}
                }

                if CTRL_PRESSED && SHIFT_PRESSED {
                    match event.event_type {
                        EventType::KeyPress(Key::KeyC) => {
                            // 启动新的线程来处理文件选择和映射
                            thread::spawn(|| {
                                let mappings = quickly_create_mapping_by_open_file_selector();
                                if mappings.is_empty() {
                                    log_to_file(
                                        "未选择任何文件",
                                        None,
                                        LogType::Info
                                    );
                                } else {
                                    log_to_file(&format!("已尝试创建如下文件的映射:{:?}", mappings),
                                                None,
                                                LogType::Info);

                                    let paste_content = common::format_filenames_url(&mappings);

                                    let mut clipboard = Clipboard::new().unwrap();
                                    clipboard.set_text(paste_content.join("\n").to_owned()).expect("创建的映射写入剪切板失败");

                                    log_to_file(
                                        &format!("成功创建映射,并已将内容粘贴到剪切板:{:?}", paste_content),
                                        None,
                                        LogType::Debug
                                    );
                                }
                            });
                        },
                        EventType::KeyPress(Key::KeyV) => {
                            // 如果按下Ctrl+Shift+V,则模拟粘贴一段固定的文本
                            quickly_create_mapping_by_clipboard::create_paste_by_clipboard();
                        },
                        _ => {}
                    }
                }
            }
        };

        // 启动键盘监听器,如果发生错误则记录到日志中
        if let Err(error) = listen(callback) {
            log_to_file(
                &format!("Error starting keyboard listener: {:?}", error),
                None,
                LogType::Warning
            );
        }
    });
}

// 从文件选择器快捷打开并选择文件来创建映射，返回一个`文件名-URL`的哈希表
fn quickly_create_mapping_by_open_file_selector() -> HashMap<String , String> {
    // 创建一个新的文件选择对话框
    let files = FileDialog::new()
        // 设置默认目录为桌面
        .set_directory("~/Desktop")
        // 调用 pick_files() 方法打开文件选择器
        .pick_files();

    let mut filenames_paths_map: HashMap<String , String> = HashMap::new();
    let mut err_links: Vec<String> = vec![];
    
    // 检查是否选择了文件
    if let Some(files) = files {
        // 遍历选择的文件
        for file in files {
            // 打印出选择的文件路径
            log_to_file(
                &format!("选择的文件路径: {:?}", &file.to_path_buf()) , 
                None , 
                LogType::Info
            );

            // 尝试根据文件路径创建映射
            let filename_url = common::create_mapping_by_path(&file);

            match filename_url {
                Ok(filename_url) => {
                    filenames_paths_map.insert(filename_url.0, filename_url.1);
                }
                Err(e) => {
                        log_to_file(
                        &format!("根据路径创建映射失败: {}", e),
                        None,
                        LogType::Warning
                    );
                    err_links.push(String::from(file.to_str().unwrap_or("")));
                }
            }
            
        }
        if !err_links.is_empty() {
            let failed_paths = err_links.join("\n");
            let message = format!("以下路径映射创建失败:\n{}", failed_paths);
            log_to_file(
                    &message, 
                    None, 
                LogType::Warning
            )
        }

        return filenames_paths_map;
    }

    else {
        // 如果没有选择文件,打印提示信息
        log_to_file("未选择任何文件" , None , LogType::Info);
        return filenames_paths_map;
    }
}

// 从剪切板快速创建映射
mod quickly_create_mapping_by_clipboard {

    use std::path::PathBuf;
    use std::collections::HashMap;
    use arboard::Clipboard;
    
    use libs::utils::{log_to_file , LogType};
    use super::common::{self, create_mapping_by_path};

    // 创建映射并将格式化后的内容粘贴到剪切板
    pub fn create_paste_by_clipboard() {

        // 获取文件路径向量
        let paths = &super::FILE_PATH.lock().unwrap();
        // 创建一个存储文件名-URL映射的哈希表
        let mut filenames_urls_map: HashMap<String, String> = HashMap::new();
        // 创建剪切板对象
        let mut clipboard = Clipboard::new().unwrap();
        // 检查是否已创建所有路径
        let flag = common::check_paths_created();

        log_to_file(
            &format!("准备创建如下文件的映射: {:?}", paths), 
            None, 
            LogType::Debug
        );

        // 遍历文件路径映射表
        for path in paths.iter() {
            
            if flag {
                log_to_file(
                    &format!("对于路径:{:?}已创建过链接，跳过" , path), 
                    None, 
                    LogType::Debug
                );
                break;
            }

            log_to_file(
                &format!("尝试创建映射: {}", &path.to_str().unwrap_or("")), 
                None, 
                LogType::Debug
            );

            // 根据路径创建映射URL
            let filename_url: Result<(String , String), String> = create_mapping_by_path(path);

            // 处理URL创建结果
            match filename_url {
                Ok(filename_url) => {
                    // 记录成功创建映射的日志
                    log_to_file(
                        &format!("从剪切板创建映射成功: {:?}", filename_url),
                        None,
                        LogType::Info
                    );
                    // 将文件名-URL映射添加到哈希表中
                    filenames_urls_map.insert(filename_url.0, filename_url.1);

                    // 格式化文件名-URL映射内容并写入剪切板
                    let paste_content = common::format_filenames_url(&filenames_urls_map).join("\n");
                    clipboard.set_text(paste_content.to_owned()).expect("创建的映射写入剪切板失败");

                    // 记录写入剪切板的日志
                    log_to_file(
                        &format!("从剪贴板粘贴内容{}", &paste_content),
                        None,
                        LogType::Debug
                    );
                }
                Err(e) => {
                    // 记录创建映射失败的日志
                    log_to_file(
                        &format!("从剪切板创建映射失败: {}", e),
                        None,
                        LogType::Warning
                    );
                }
            }
        }
    }

    // 检查剪切板中的内容是否为文件路径，并返回一向量，其中元素文件路径
    pub fn handle_clipboard_content(content: &str) -> Vec<PathBuf> {
        // 创建一个新的向量用于存储路径
        let mut paths: Vec<PathBuf> = vec![];
        
        // 将剪贴板内容按换行符分割成一个个路径
        let clipboard_content = content.split('\n');
        
        // 遍历每个路径
        for path_str in clipboard_content {
            // 创建一个 PathBuf 对象表示路径
            let path = PathBuf::from(path_str.trim());
            
            // 检查该路径是否存在且是文件
            if path.exists() && path.is_file() {
                paths.push(path);
            }
        }
        
        // 返回填充好的路径向量
        paths
    }

}

mod common {

    use std::path::PathBuf;
    use std::collections::HashMap;
    
    use libs::{
        utils::{log_to_file , LogType },
        crud::{connect_db , mappings::create_mapping},
        path_struct::generate_random_url,
        hard_link_create
    };

    // 只根据路径创建映射，并返回这个路径所对应的映射的`文件名,URL`的元组
    pub fn create_mapping_by_path(path: &PathBuf) -> Result<(String , String), String>{

        // 数据库连接
        let conn = connect_db();
        // 获取文件名
        let file_name = path.file_name().unwrap().to_str().unwrap_or("");
        // 获取原始路径
        let origin_path = path.to_str().unwrap_or("");
        // 生成随机URL
        let url = generate_random_url();

        // 创建硬链接
        if let Some(path_str) = path.to_str() {
            // 尝试创建硬链接
            match hard_link_create(path_str) {
                Ok(hard_link) => {
                    // 如果硬链接创建成功，打印并存储硬链接路径
                    log_to_file(
                        &format!("硬链接创建成功: {}", &url), 
                        None, 
                        LogType::Info
                    );
                    
                    // 并创建映射以及返回对应的URL
                    match create_mapping(&conn, file_name, origin_path, &url, &hard_link) {
                        Ok(()) => return Ok(
                            (String::from(file_name) , url)
                        ),
                        Err(e) => {
                            // 如果创建映射失败，记录错误
                            log_to_file(
                                &format!("映射创建失败: {}", e), 
                                None, 
                                LogType::Warning
                            );

                            return Err(format!("映射创建失败: {}", e));
                        }
                    }
                },
                Err(e) => {
                    // 如果创建硬链接失败，记录错误
                    log_to_file(
                        &format!("硬链接创建失败: {}", e), 
                        None, 
                        LogType::Warning
                    );

                    return Err(format!("硬链接创建失败: {}", e));
                }
            }
        } else {
            // 处理错误，例如打印错误信息或者采取其他错误处理措施
            log_to_file(
                &format!("Failed to convert path : \"{:?}\"to string" , &path.to_path_buf()), 
                None, 
                LogType::Warning
            );
            return Err(format!("Failed to convert path : \"{:?}\"to string" , &path.to_path_buf()));
        }
    }

    // 更新存储的路径向量
    pub fn update_file_path_map(new_paths: Vec<PathBuf>) {
        // 获取路径向量的可写锁
        let mut paths = super::FILE_PATH.lock().unwrap();
        // 遍历新的路径向量
        for path in new_paths {
            // 将新的文件路径添加到路径向量中
            paths.push(path);
        }
    }

    // 将文件名-URL的哈希表进行格式化，返回一个字符串向量，这个字符串向量的每个为`[filename](url)`的格式，可用于粘贴到markdown中作为超链接
    pub fn format_filenames_url (filenames_urls_map: &HashMap<String , String>) -> Vec<String> {
        
        let mut filenames_url_vec: Vec<String> = Vec::new();

        for (filename , url) in filenames_urls_map {
            
            let filenames_url = format!("[{}]({})", filename , url);
            filenames_url_vec.push(filenames_url);
        }

        filenames_url_vec
    }

    // 检查路径是否已经创建过映射
    pub fn check_paths_created() -> bool {

        log_to_file(
            "正在检查是否已创建过映射...", 
            None, 
            LogType::Debug
        );

        let file_paths_lock = super::FILE_PATH.try_lock();
        let created_paths_lock = super::CREATED_PATHS.try_lock();
    
        match (file_paths_lock, created_paths_lock) {
            (Ok(file_paths), Ok(created_paths)) => {
                if *file_paths == *created_paths {
                    println!("这些路径已创建过映射了");
                    true
                } else {
                    println!("这些路径未创建映射");
                    false
                }
            },
            _ => {
                println!("Unable to lock both maps, avoiding deadlock");
                false
            }
        }
    }    

}