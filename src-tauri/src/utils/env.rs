/*
 * @Date: 2024-11-01 15:25:14
 * @LastEditors: sunsjay sunsjay0806@gmail.com
 * @LastEditTime: 2024-11-01 15:39:11
 * @FilePath: /apple-control/src-tauri/src/utils/mod.rs
 * @Description:
 */
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::time::{SystemTime, UNIX_EPOCH};

use dotenv::dotenv;


pub fn get_env_var(key: &str) -> String {
    let mut env_file_path = String::new();

    // 根据操作系统类型设置不同的路径
    #[cfg(target_os = "windows")]
    {
        env_file_path = "C:\\Program Files\\apple".to_string();
    }

    #[cfg(target_os = "macos")]
    {
        env_file_path = "/Users/sunsjay/Desktop".to_string();
    }

    // 设置当前工作目录
    std::env::set_current_dir(&env_file_path).unwrap();

    // 加载.env文件中的环境变量
    match dotenv() {
        Ok(_) => {
            match env::var(key) {
                Ok(val) => val,
                Err(_) => "".to_string()
            }
        }
        Err(_) => {
            "".to_string()
        }
    }
}





