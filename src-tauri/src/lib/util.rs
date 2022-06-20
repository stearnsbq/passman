use std::sync::{Mutex, MutexGuard};

use crate::Context;

pub fn acquire_context_lock<'a>(state: &'a tauri::State<Mutex<Context>>) -> Result<MutexGuard<'a, Context>, String>{

    let context = state.lock().expect("Failed to acquire lock on context");

    if !context.logged_in {
      return Err("Not Logged In!".into())
    }
  
    Ok(context)
}