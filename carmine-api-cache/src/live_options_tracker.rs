use std::time::{Duration, SystemTime};

use carmine_api_core::types::Messenger;

pub struct LiveOptionsUpdateTracker<M: Messenger> {
    last_updated: SystemTime,
    last_reported: SystemTime,
    messenger: M,
}

impl<M: Messenger> LiveOptionsUpdateTracker<M> {
    pub fn new(messenger: M) -> Self {
        let now = SystemTime::now();
        Self {
            last_updated: now,
            last_reported: now,
            messenger,
        }
    }

    pub fn report(&mut self) {
        let now = SystemTime::now();

        if let Ok(elapsed_since_update) = now.duration_since(self.last_updated) {
            if elapsed_since_update > Duration::new(300, 0) {
                // 5 minutes
                if let Ok(elapsed_since_report) = now.duration_since(self.last_reported) {
                    if elapsed_since_report > Duration::new(600, 0) {
                        let elapsed_secs = elapsed_since_update.as_secs();
                        let mins = elapsed_secs / 60;
                        let secs = elapsed_secs % 60;
                        let message =
                            format!("Live Options last updated {}mins {}secs ago.", mins, secs);
                        self.messenger.send_message(&message); // Call Messenger's method
                        self.last_reported = now;
                    }
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.last_updated = SystemTime::now();
    }
}
