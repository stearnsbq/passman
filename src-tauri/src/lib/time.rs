use std::time;

pub fn get_current_secs() -> u64{
    let start = time::SystemTime::now();
    let since_the_epoch = start
    .duration_since(time::SystemTime::UNIX_EPOCH)
    .expect("Time went backwards");

    return since_the_epoch.as_secs()
}