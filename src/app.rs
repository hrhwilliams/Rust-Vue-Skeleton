use std::sync::{Arc, atomic::AtomicU32};

use crate::database::PostgresDatabase;

/// Stores the value of the counter. This is passed around to the endpoints
/// and modified by them.
#[derive(Clone)]
pub struct AppState {
    pub db: PostgresDatabase,
    counter: Arc<AtomicU32>,
}

impl AppState {
    pub fn new(db: PostgresDatabase) -> Self {
        Self {
            db,
            counter: Arc::new(AtomicU32::new(0)),
        }
    }

    /// Return the value of the counter
    pub fn read(&self) -> u32 {
        self.counter.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Increment the counter
    pub fn inc(&self) {
        self.counter
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}

pub struct App;
