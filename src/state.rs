use std::sync::{Arc, Mutex};
use crate::models::User;

pub type AppState = Arc<Mutex<Vec<User>>>;

pub fn new_state() -> AppState {
    Arc::new(Mutex::new(vec![]))
}