use std::{
    sync::{
        mpsc::{channel, Sender},
        Arc,
    },
    thread::JoinHandle,
    time::Duration,
};

use super::solution_progress_bar::SolutionProgressBar;

/// Command line progress bar spawned and updated in a separate thread
pub struct SolutionProgressBarThread {
    progress: Arc<SolutionProgressBar>,
    thread_handle: Option<JoinHandle<()>>,
    tx_stop_signal: Option<Sender<()>>,
}

impl SolutionProgressBarThread {
    /// Creates new instance
    pub fn new(prefix: &str) -> Self {
        Self {
            progress: Arc::new(SolutionProgressBar::new(prefix)),
            thread_handle: None,
            tx_stop_signal: None,
        }
    }

    /// Starts the execution of the progress bar in a separate thread until terminated
    pub fn run(&mut self) {
        // Create channel to stop of the thread
        let (tx, rx) = channel();
        self.tx_stop_signal = Some(tx);

        // Clone data needed in a thread
        let progress = self.progress.clone();

        let handle = std::thread::spawn(move || {
            // Update progress bar once per second until stopped
            loop {
                // Wait up to one second
                let wait_result = rx.recv_timeout(Duration::from_secs(1));

                // If we have received stop signal end thread
                if wait_result.is_ok() {
                    break;
                }

                // Not stopped yet => move progress bar
                progress.tick();
            }
        });

        self.thread_handle = Some(handle);
    }

    /// Stops the execution of the progress bar and finalize output
    fn finish(&mut self) {
        if let Some(ref tx) = self.tx_stop_signal {
            if let Err(err) = tx.send(()) {
                panic!(
                    "Failed to send signal to stop ProgressBar with error '{}'",
                    err
                );
            }
        }

        self.progress.finish();
    }
}

/// Automatically stop progress bar on destruction
impl Drop for SolutionProgressBarThread {
    fn drop(&mut self) {
        self.finish();
    }
}
