use indicatif::{ProgressBar, ProgressStyle};

/// Command line progress bar to show progress of the computation in case of longer tasks.
pub struct SolutionProgressBar {
    progress_bar: ProgressBar,
}

impl SolutionProgressBar {
    /// Create new instance of the progress bar instance
    ///
    /// # Arguments
    ///
    /// _prefix_ - Progress bar prefix
    ///
    /// # Returns
    ///
    /// New instance of Progress
    pub fn new(prefix: &str) -> Self {
        let progress_style = ProgressStyle::default_spinner()
            .template("{prefix} {spinner}")
            .expect("Failed to create ProgressStyle")
            .tick_strings(&[
                "◯◯◯◯◯◯◯◯◯◯",
                "⬤◯◯◯◯◯◯◯◯◯",
                "⬤⬤◯◯◯◯◯◯◯◯",
                "⬤⬤⬤◯◯◯◯◯◯◯",
                "⬤⬤⬤⬤◯◯◯◯◯◯",
                "⬤⬤⬤⬤⬤◯◯◯◯◯",
                "⬤⬤⬤⬤⬤⬤◯◯◯◯",
                "⬤⬤⬤⬤⬤⬤⬤◯◯◯",
                "⬤⬤⬤⬤⬤⬤⬤⬤◯◯",
                "⬤⬤⬤⬤⬤⬤⬤⬤⬤◯",
                "⬤⬤⬤⬤⬤⬤⬤⬤⬤⬤",
                "",
            ]);

        let progress_bar = ProgressBar::new_spinner()
            .with_style(progress_style)
            .with_prefix(prefix.to_string());

        // Show first progress bar status
        progress_bar.tick();

        Self { progress_bar }
    }

    /// Update progress bar with next tick
    pub fn tick(&self) {
        self.progress_bar.inc(1);
    }

    /// Finish progress bar
    pub fn finish(&self) {
        self.progress_bar.finish_and_clear();
    }
}
