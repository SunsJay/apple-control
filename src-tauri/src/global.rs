use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::utils::env;
use sqlx::mysql::MySqlPool;




// 定义全局变量，使用 lazy_static 进行延迟初始化
lazy_static! {
    pub static ref DATABASE_URL: Mutex<String> = Mutex::new("".to_string());
    pub static ref POOL: Mutex<Option<MySqlPool>> = Mutex::new(None);
}


pub  async  fn init_global_var() {
    *DATABASE_URL.lock().unwrap() = env::get_env_var("DATABASE_URL");
    println!(r#"
    初始化全局变量
    DATABASE_URL: {},
    "#, DATABASE_URL.lock().unwrap())
}