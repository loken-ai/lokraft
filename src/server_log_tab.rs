//! Server Log Tab Rendering
//!
//! A chrome row with the server's lamp and the filters, and the log behind
//! the glass of a screen. An entry is one line: a stripe in the level's tint,
//! the time, the level, the target and the message in monospace. Only Warn
//! and Error are coloured; the level is also written, so the colour is never
//! the only carrier.

use eframe::egui::{self, Color32};

use crate::icons::Icon;
use crate::log_buffer::{LogBuffer, LogLevel};
use crate::state::{LogLevelFilter, ServerState, ServerStatus};
use crate::theme::{self, text};
use crate::ui::widgets;

/// Point size of the icons in the chrome row.
const ICON_PT: f32 = 14.0;
/// Fixed cells in the chrome row: the status word and the port.
/// Width of the search field.
const SEARCH_W: f32 = 200.0;
/// Fixed cells in an entry: the time and the level.
const TIME_W: f32 = 96.0;
const LEVEL_W: f32 = 48.0;
/// Padding inside an entry.
/// Alpha of the wash under a warning or an error line.
const LOG_ROW_WASH_A: u8 = 22;
const ROW_PAD_X: i8 = 8;
const ROW_PAD_Y: i8 = 3;
/// Margin kept under the screen, and the least height it keeps.
const SCREEN_BOTTOM_MARGIN: f32 = 20.0;
const SCREEN_MIN_H: f32 = 100.0;

/// Render the Server Log tab content
pub fn render(
    ui: &mut egui::Ui,
    server: &mut ServerState,
    embedded_port: Option<u16>,
    log_buffer: &LogBuffer,
) {
    ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
        render_log_header(ui, server, embedded_port, log_buffer);
        ui.add_space(widgets::GAP_WIDGETS);
        render_log_output(ui, server, embedded_port, log_buffer);
    });
}

/// The server's state as a lamp and a word.
fn server_lamp(status: &ServerStatus) -> (bool, Color32, &'static str) {
    match status {
        ServerStatus::NotStarted => (false, theme::ink_dim(), "NOT STARTED"),
        ServerStatus::Starting => (true, theme::warning(), "STARTING"),
        ServerStatus::Running { .. } => (true, theme::success(), "RUNNING"),
        ServerStatus::Failed(_) => (true, theme::error(), "FAILED"),
    }
}

/// The chrome row: title, the server lamp and its word, the port, the level
/// filters as pills, the search, Copy and Clear in the tail.
fn render_log_header(
    ui: &mut egui::Ui,
    server: &mut ServerState,
    embedded_port: Option<u16>,
    log_buffer: &LogBuffer,
) {
    widgets::chrome_row(ui, |ui| {
        Icon::Server.show(ui, ICON_PT, theme::ink());
        ui.label(text::title("Logs"));
        let (lit, tint, word) = server_lamp(&server.status);
        ui.add_space(widgets::GAP_WIDGETS);
        widgets::lamp_inline(ui, lit, tint);
        ui.label(text::note(word));
        if let Some(port) = embedded_port {
            widgets::pill(ui, &format!("localhost:{port}"));
        }
    });
    ui.add_space(widgets::GAP_WIDGETS);

    // The toolbar: the level as a segmented control, the search, and the actions.
    ui.horizontal(|ui| {
        let levels = [
            LogLevelFilter::All,
            LogLevelFilter::Info,
            LogLevelFilter::Warn,
            LogLevelFilter::Error,
        ];
        let labels: Vec<&str> = levels.iter().map(|l| l.display_name()).collect();
        let selected = levels
            .iter()
            .position(|l| *l == server.log_filter)
            .unwrap_or(0);
        if let Some(i) = widgets::segmented(ui, &labels, selected) {
            server.log_filter = levels[i];
        }
        ui.add_space(widgets::GAP_WIDGETS);

        Icon::Search.show(ui, ICON_PT, theme::ink_dim());
        ui.add(
            egui::TextEdit::singleline(&mut server.log_search)
                .desired_width(SEARCH_W)
                .hint_text("Search"),
        );

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .add(widgets::quiet_button("Clear"))
                .on_hover_text("Clear the log")
                .clicked()
            {
                server.log_clear_confirm_pending = true;
            }
            // The visible lines, as "HH:MM:SS.mmm LEVEL target: message".
            if ui
                .add(widgets::quiet_button("Copy"))
                .on_hover_text("Copy the visible (filtered) lines to the clipboard")
                .clicked()
            {
                let text = server
                    .filtered_logs(log_buffer)
                    .iter()
                    .map(|e| {
                        format!(
                            "{} {} {}: {}",
                            e.timestamp,
                            e.level.as_str(),
                            e.target,
                            e.message
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                crate::notice::copy(ui.ctx(), text, "the visible log lines");
            }
        });
    });
    ui.add_space(widgets::GAP_WIDGETS);
    if server.log_clear_confirm_pending {
        let outcome = widgets::confirm_dialog(
            ui.ctx(),
            &widgets::Confirm {
                title: "Clear log",
                question: "Clear the log?",
                subject: None,
                detail: "The lines shown here are forgotten.",
                action: "Clear",
            },
        );
        if let Some(confirmed) = outcome {
            server.log_clear_confirm_pending = false;
            if confirmed {
                log_buffer.clear();
            }
        }
    }
}

/// The log, behind the glass of a screen that fills the height and follows
/// its tail.
fn render_log_output(
    ui: &mut egui::Ui,
    server: &ServerState,
    embedded_port: Option<u16>,
    log_buffer: &LogBuffer,
) {
    let log_height =
        (ui.available_height() - SCREEN_BOTTOM_MARGIN - 2.0 * widgets::SECTION_PADDING)
            .max(SCREEN_MIN_H);
    widgets::screen_well(ui, |ui| {
        egui::ScrollArea::vertical()
            .stick_to_bottom(true)
            .max_height(log_height)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.set_width(ui.available_width());

                let filtered_logs = server.filtered_logs(log_buffer);

                if filtered_logs.is_empty() {
                    if log_buffer.count() == 0 {
                        // The buffer holds the app's own tracing output; an embedded
                        // server's lines go through the same buffer.
                        let hint = if embedded_port.is_some() {
                            "Server logs will appear here as the server runs."
                        } else {
                            "The requests this app makes and its own events appear here."
                        };
                        widgets::empty_state(ui, Icon::Server, "No logs yet", hint);
                    } else {
                        widgets::empty_state(
                            ui,
                            Icon::Search,
                            "No line matches",
                            "Change the level or the search.",
                        );
                    }
                } else {
                    for entry in filtered_logs.iter() {
                        render_log_entry(ui, entry);
                    }
                }
            });
    });
}

/// The tint of an entry's stripe and message: only Warn and Error are
/// coloured, the rest are dim ink.
fn level_stripe(level: LogLevel) -> Color32 {
    match level {
        LogLevel::Error => theme::error(),
        LogLevel::Warn => theme::warning(),
        LogLevel::Trace | LogLevel::Debug | LogLevel::Info => theme::ink_dim(),
    }
}

/// One entry: a stripe, the time and the level in fixed cells, the target,
/// the message. Selectable, so a line can be copied into a report.
fn render_log_entry(ui: &mut egui::Ui, entry: &crate::log_buffer::LogEntry) {
    let tint = level_stripe(entry.level);
    let coloured = matches!(entry.level, LogLevel::Warn | LogLevel::Error);
    let fill = if coloured {
        theme::tinted(tint, LOG_ROW_WASH_A)
    } else {
        egui::Color32::TRANSPARENT
    };
    egui::Frame::NONE
        .fill(fill)
        .corner_radius(theme::RADIUS)
        .inner_margin(egui::Margin::symmetric(ROW_PAD_X, ROW_PAD_Y))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.horizontal(|ui| {
                widgets::readout(ui, TIME_W, &entry.timestamp);
                let level_ink = if coloured { tint } else { theme::ink_dim() };
                widgets::fixed_label(
                    ui,
                    LEVEL_W,
                    text::mono(entry.level.as_str_padded()).color(level_ink),
                );
                let target_short = entry.target.rsplit("::").next().unwrap_or(&entry.target);
                ui.label(text::readout(target_short));
                ui.add(egui::Label::new(text::mono(&entry.message)).selectable(true));
            });
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Routine levels share the dim ink; Warn and Error each have their own
    /// colour, and not each other's.
    #[test]
    fn only_warn_and_error_are_coloured() {
        let _skin = theme::skin_lock();
        let dim = theme::ink_dim();
        assert_eq!(level_stripe(LogLevel::Trace), dim);
        assert_eq!(level_stripe(LogLevel::Debug), dim);
        assert_eq!(level_stripe(LogLevel::Info), dim);
        assert_ne!(level_stripe(LogLevel::Warn), dim);
        assert_ne!(level_stripe(LogLevel::Error), dim);
        assert_ne!(level_stripe(LogLevel::Warn), level_stripe(LogLevel::Error));
    }

    /// The level is written beside the stripe, so a reader who cannot tell
    /// the colours apart still reads it.
    #[test]
    fn the_level_is_also_written() {
        let words: std::collections::HashSet<&str> = [
            LogLevel::Trace,
            LogLevel::Debug,
            LogLevel::Info,
            LogLevel::Warn,
            LogLevel::Error,
        ]
        .into_iter()
        .map(LogLevel::as_str_padded)
        .collect();
        assert_eq!(words.len(), 5, "every level has its own word");
    }
}
