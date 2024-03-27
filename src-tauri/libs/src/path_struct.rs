use std::path::PathBuf;
use uuid::Uuid;
#[cfg(target_os = "linux")]
use df_command_parse as df_parse;

/// Build hard link path
///
/// Generate a unique hard link path for the file based on the original file path and username.
/// This function adds a UUID to the filename to ensure uniqueness of the link.
///
/// # Parameters
/// * `original_path` - Reference to `PathBuf` of the original file.
/// * `user_name` - Username used to identify user directory in the hard link path.
///
/// # Returns
/// Returns a `PathBuf` containing the new hard link path.
///
/// # Example
/// ```
/// let original_path = PathBuf::from("/path/to/original/file.txt");
/// let user_name = "user123";
/// let hard_link_path = build_hard_link_path(&original_path, user_name);
/// println!("Hard Link Path: {:?}", hard_link_path);
 ///```
 ///
 /// # Platform Specific
 /// - On Linux platform, use the `df_parse` library to determine filesystem information and construct paths.
 /// - On Windows platform, use drive letters to construct paths.
pub fn build_hard_link_path(original_path: &PathBuf, user_name: &str) -> PathBuf {
    let file_name = original_path.file_name().unwrap_or_default().to_str().unwrap_or_default();
    let uuid = Uuid::new_v4().simple().to_string();
    let short_uuid = &uuid[..5]; // 限制UUID为5位
    let extension = original_path.extension().unwrap_or_default().to_str().unwrap_or_default();
    let new_file_name = format!("{}.{}.{}", file_name, short_uuid, extension);
    

    #[cfg(target_os = "linux")]
    {
        let file_system = df_parse::get_file_system_info(original_path.to_str().unwrap())
            .unwrap_or_else(|_| ("/".to_string(), "/".to_string()));
        println!("文件系统:{}", file_system.1);

        let mut hard_link_path = PathBuf::from(&file_system.1);
        hard_link_path.push(user_name);
        hard_link_path.push(".pathlinker");
        hard_link_path.push(new_file_name);
        hard_link_path
    }

    #[cfg(target_os = "windows")]
    {
        let drive_letter = original_path.to_str().unwrap().chars().next().unwrap_or('C');
        println!("驱动器:{}", drive_letter);
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

// parse df command on linux
#[cfg(target_os = "linux")]
mod df_command_parse {
    use std::process::{Command, Output};

    pub fn get_file_system_info(file_path: &str) -> Result<(String, String), String> {
        let output = Command::new("df")
            .arg(file_path)
            .output()
            .map_err(|e| e.to_string())?;
    
        if !output.status.success() {
            return Err(String::from("无法执行df命令"));
        }
    
        parse_df_output(output)
    }
    
    pub fn parse_df_output(output: Output) -> Result<(String, String), String> {
        let output_str = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
        let lines: Vec<&str> = output_str.split('\n').collect();
        if lines.len() > 1 {
            let second_line = lines[1];
            let parts: Vec<&str> = second_line.split_whitespace().collect();
            if parts.len() >= 6 {
                Ok((String::from(parts[0]), String::from(parts[5])))
            } else {
                Err(String::from("解析df输出失败"))
            }
        } else {
            Err(String::from("df命令输出格式不符合预期"))
        }
    }
    
}