//! Terminal output and desktop notifications.

use std::io::{IsTerminal as _, Write as _};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use oci_arm_catcher_core::{Environment, Event, RetryReason, catcher};

/// Prints progress, counts down between attempts, and notifies on the way out.
pub struct Console {
    started: Instant,
    interactive: bool,
}

impl Console {
    pub fn new() -> Self {
        Self {
            started: Instant::now(),
            interactive: std::io::stdout().is_terminal(),
        }
    }

    /// `[+hh:mm:ss]` since start: on a multi-day hunt, more useful than wall-clock time.
    fn stamp(&self) -> String {
        let s = self.started.elapsed().as_secs();
        format!("[+{:02}:{:02}:{:02}]", s / 3600, s / 60 % 60, s % 60)
    }
}

impl Environment for Console {
    fn event(&mut self, event: Event<'_>) {
        match event {
            Event::Attempt {
                number,
                availability_domain,
            } => println!(
                "{} Attempt #{number}  {}",
                dim(&self.stamp()),
                dim(&format!("({availability_domain})"))
            ),
            Event::Retrying { error, reason, .. } => {
                let label = match reason {
                    RetryReason::OutOfCapacity => "no capacity yet",
                    RetryReason::Throttled => "throttled",
                    RetryReason::ServerError => "OCI server error",
                    RetryReason::Network => "network error, resending the same request",
                };
                // "Code: message" -> code in red, so a new kind of error stands out.
                let text = error.to_string();
                let line = match text.split_once(": ") {
                    Some((code, message)) => format!("{}: {message}", red(code)),
                    None => red(&text),
                };
                println!("  -> {line}  {}", dim(&format!("[{label}]")));
            }
        }
    }

    fn sleep(&mut self, duration: Duration) {
        if !self.interactive {
            std::thread::sleep(duration);
            return;
        }
        let end = Instant::now() + duration;
        let mut out = std::io::stdout();
        while let Some(left) = end.checked_duration_since(Instant::now()) {
            let s = left.as_secs();
            let _ = write!(
                out,
                "\r  {}",
                yellow(&format!("... next attempt in {:02}:{:02} ", s / 60, s % 60))
            );
            let _ = out.flush();
            std::thread::sleep(left.min(Duration::from_secs(1)));
        }
        let _ = write!(out, "\r\x1b[K");
        let _ = out.flush();
    }

    fn new_token(&mut self) -> String {
        catcher::random_token()
    }
}

/// ANSI styling only on a terminal, and never with `NO_COLOR` set (https://no-color.org).
fn color() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::io::stdout().is_terminal() && std::env::var_os("NO_COLOR").is_none_or(|v| v.is_empty())
    })
}

fn paint(code: &str, text: &str) -> String {
    if color() {
        format!("\x1b[{code}m{text}\x1b[0m")
    } else {
        text.to_owned()
    }
}

pub fn dim(text: &str) -> String {
    paint("2", text)
}

pub fn red(text: &str) -> String {
    paint("31", text)
}

pub fn yellow(text: &str) -> String {
    paint("33", text)
}

pub fn green(text: &str) -> String {
    paint("1;32", text)
}

/// Best-effort desktop notification; the terminal output is the record.
pub fn notify(message: &str) {
    let _ = notify_rust::Notification::new()
        .summary("oci-arm-catcher")
        .body(message)
        .show();
}

/// Print an error and its causes to stderr.
pub fn report_error(error: &anyhow::Error) {
    eprintln!("error: {error}");
    for cause in error.chain().skip(1) {
        eprintln!("  caused by: {cause}");
    }
}
