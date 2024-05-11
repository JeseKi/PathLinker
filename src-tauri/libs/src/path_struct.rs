use std::path::PathBuf;
use uuid::Uuid;
use rand::{distributions::Alphanumeric, Rng};

#[cfg(target_os = "linux")]
use df_command_parse as df_parse;

// 构建硬链接路径的函数
pub fn build_hard_link_path(original_path: &PathBuf, user_name: &str) -> PathBuf {
    // 获取原文件名称
    let file_name = original_path.file_name().unwrap_or_default().to_str().unwrap_or_default();
    // 生成一个新的UUID
    let uuid = Uuid::new_v4().simple().to_string();
    // 截取UUID的前5位
    let short_uuid = &uuid[..5]; 
    // 获取原文件的扩展名
    let extension = original_path.extension().unwrap_or_default().to_str().unwrap_or_default();
    // 构建新的文件名称, 格式为 "原文件名.UUID前5位.扩展名"
    let new_file_name = format!("{}.{}.{}", file_name, short_uuid, extension);

    // 针对Linux系统的逻辑
    #[cfg(target_os = "linux")]
    {
        // 获取原文件所在的文件系统信息
        let file_system = df_parse::get_file_system_info(original_path.to_str().unwrap())
            .unwrap_or_else(|_| ("/".to_string(), "/".to_string()));
        println!("文件系统:{}", file_system.1);

        // 构建硬链接路径, 格式为 "文件系统挂载点/用户名/.pathlinker/新文件名"
        let mut hard_link_path = PathBuf::from(&file_system.1);
        hard_link_path.push(user_name);
        hard_link_path.push(".pathlinker");
        hard_link_path.push(new_file_name);
        hard_link_path
    }

    // 针对Windows系统的逻辑
    #[cfg(target_os = "windows")]
    {
        // 获取原文件所在的驱动器盘符
        let drive_letter = original_path.to_str().unwrap().chars().next().unwrap_or('C');
        println!("驱动器:{}", drive_letter);
        // 构建硬链接路径, 格式为 "驱动器盘符:/Users/用户名/.pathlinker/新文件名"
        let mut hard_link_path = PathBuf::from(format!("{}:/", drive_letter));
        if drive_letter == 'C' {
            hard_link_path.push("Users");
        }
        hard_link_path.push(user_name);
        hard_link_path.push(".pathlinker");
        hard_link_path.push(new_file_name);
        hard_link_path
    }
}

/// 生成随机URL
pub fn generate_random_url() -> String {
    let random_path: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    format!("pathlinker://{}/", random_path)
}

// 解析Linux上的df命令的输出
#[cfg(target_os = "linux")]
mod df_command_parse {
    use std::process::{Command, Output};

    // 获取指定文件路径的文件系统信息
    pub fn get_file_system_info(file_path: &str) -> Result<(String, String), String> {
        // 执行df命令并获取输出
        let output = Command::new("df")
            .arg(file_path)
            .output()
            .map_err(|e| e.to_string())?;
    
        // 检查命令是否执行成功
        if !output.status.success() {
            return Err(String::from("无法执行df命令"));
        }
    
        // 解析df命令的输出
        parse_df_output(output)
    }
    
    // 解析df命令的输出
    pub fn parse_df_output(output: Output) -> Result<(String, String), String> {
        // 将命令输出转换为字符串
        let output_str = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
        
        // 将输出按行分割为向量
        let lines: Vec<&str> = output_str.split('\n').collect();
        
        // 检查输出是否符合预期格式
        if lines.len() > 1 {
            let second_line = lines[1];
            let parts: Vec<&str> = second_line.split_whitespace().collect();
            
            // 检查第二行是否包含足够的信息
            if parts.len() >= 6 {
                // 提取文件系统名称和可用空间百分比并返回
                Ok((String::from(parts[0]), String::from(parts[5])))
            } else {
                Err(String::from("解析df输出失败"))
            }
        } else {
            Err(String::from("df命令输出格式不符合预期"))
        }
    }
}