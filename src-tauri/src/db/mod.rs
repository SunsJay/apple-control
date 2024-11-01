use sqlx::MySqlPool;
use crate::global::{DATABASE_URL, POOL};


pub async fn init_database() {
    let pool = MySqlPool::connect(&*DATABASE_URL.lock().unwrap()).expect("Failed to create pool");
    *POOL.lock().unwrap() = Some(pool);
}