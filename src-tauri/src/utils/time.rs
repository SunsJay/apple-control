use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_timestamp() -> String {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_secs().to_string()
}
