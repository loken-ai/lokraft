//! Short messages any screen can raise.
//! The application drains them once per frame and shows each as a toast.

use std::sync::Mutex;

use crate::toast::ToastSeverity;

static NOTICES: Mutex<Vec<(ToastSeverity, String)>> = Mutex::new(Vec::new());

fn queue() -> std::sync::MutexGuard<'static, Vec<(ToastSeverity, String)>> {
    NOTICES
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Raise a notice. It reaches the user at the next frame.
pub fn notify(severity: ToastSeverity, message: impl Into<String>) {
    queue().push((severity, message.into()));
}

/// Take every notice raised since the last call, oldest first.
pub fn drain() -> Vec<(ToastSeverity, String)> {
    std::mem::take(&mut *queue())
}

/// Put `text` on the clipboard and say so. `what` names what was copied.
pub fn copy(ctx: &eframe::egui::Context, text: String, what: &str) {
    ctx.copy_text(text);
    notify(ToastSeverity::Success, format!("Copied {what}."));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notices_come_out_once_and_in_order() {
        drain();
        notify(ToastSeverity::Success, "first");
        notify(ToastSeverity::Error, "second");
        let taken = drain();
        assert_eq!(taken.len(), 2);
        assert_eq!(taken[0].1, "first");
        assert_eq!(taken[1].1, "second");
        assert!(drain().is_empty());
    }
}
