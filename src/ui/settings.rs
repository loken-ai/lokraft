//! Settings section: one column of section panels, every field labelled in
//! capitals and explained by a caption.
//!
//! Reads/writes: AppConfig, SettingsState, ConfigEditorState

use crate::config::AppConfig;
use crate::config_editor::ConfigEditorState;
use crate::icons::Icon;
use crate::settings::{SettingsAction, SettingsState};
use crate::theme::{self, text};
use crate::ui::widgets;
use eframe::egui::{self, TextEdit};

/// Point size of the icons in the chrome row and the picker.
const ICON_PT: f32 = 14.0;
/// Width of the column of panels.
const SETTINGS_COLUMN_W: f32 = 640.0;
/// Room kept beside a field for the button that follows it.
const FIELD_BUTTON_RESERVE: f32 = 70.0;
/// The folder picker: its default size, the room kept for its action row,
/// the least height of its list, the room beside its path field.
const PICKER_W: f32 = 520.0;
const PICKER_H: f32 = 420.0;
const PICKER_ACTIONS_RESERVE: f32 = 80.0;
const PICKER_LIST_MIN_H: f32 = 120.0;
const PICKER_GO_RESERVE: f32 = 50.0;

/// Render the settings section
pub fn render(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    config: &mut AppConfig,
    settings_state: &mut SettingsState,
    connection_state: crate::state::ConnectionState,
) -> Vec<SettingsAction> {
    let mut actions = Vec::new();

    // Auto-initialize config editor state if not yet loaded
    if settings_state.config_editor.is_none() {
        if let Ok(cfg) = crate::config_editor::Config::load_default() {
            settings_state.config_editor = Some(ConfigEditorState::from_config(&cfg));
        }
    }

    // In-app folder picker modal (independent of rfd/portal/zenity).
    // Renders when editor.inapp_picker is Some — opens via the "..."
    // button next to Browse, lets the user navigate with parent / sub
    // links + a "Use this folder" commit button, and writes the result
    // straight into the editor field. Always works, even on systems
    // where the native picker is broken.
    if let Some(editor) = settings_state.config_editor.as_mut() {
        if editor.inapp_picker.is_some() {
            render_inapp_folder_picker(ctx, editor);
        }
    }

    // (Previous rfd worker-thread drain removed — Browse now uses the
    // pure-egui in-app picker rendered above. No more pending_browse /
    // pending_browse_error Mutex round-trip.)

    // Reset GUI defaults confirmation modal. Two-step confirm matches
    // the pattern used for the model-delete confirmation (41088d1) —
    // single-click Reset would wipe profiles, server URL, and the
    // selected model without recovery, so guard against fat-fingers
    // and accidental clicks.
    if settings_state.reset_confirm_pending {
        let outcome = widgets::confirm_dialog(
            ctx,
            &widgets::Confirm {
                title: "Reset GUI defaults",
                question: "Reset all GUI settings to defaults?",
                subject: None,
                detail: "This wipes:\n  - All connections and their API keys\n  - Generation parameters\n  - Selected model\n  - Theme and window preferences\n\nLocal model files on disk are not affected.\nA backup of the current config.json is saved as config.json.before_reset.<timestamp> in the same folder.",
                action: "Reset",
            },
        );
        let confirmed = outcome == Some(true);
        let close_modal = outcome == Some(false);
        if confirmed {
            // Snapshot the current config.json to
            // <path>.before_reset.<unix_ts> so an accidental reset
            // can be undone by copying the backup back. Best-effort:
            // failures log + the reset still proceeds (a backup
            // hiccup shouldn't block the user's intent).
            if let Some(dirs) = directories::ProjectDirs::from("com", "loken", "lokraft") {
                AppConfig::backup_before_reset(dirs.config_dir());
            }
            *config = AppConfig::default();
            settings_state.connection_form = Default::default();
            config.save();
            settings_state.reset_confirm_pending = false;
        } else if close_modal {
            settings_state.reset_confirm_pending = false;
        }
    }

    widgets::chrome_row(ui, |ui| {
        Icon::Gear.show(ui, ICON_PT, theme::ink());
        ui.label(text::title("Settings"));
    });
    ui.add_space(widgets::GAP_WIDGETS);

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.set_width(ui.available_width());

        // ── Connections ──
        connections_panel(ui, config, settings_state, connection_state);

        ui.add_space(widgets::GAP_WIDGETS);

        // ── Appearance ──
        settings_panel(ui, "APPEARANCE", |ui| {
            form_row(ui, "THEME", |ui| {
                let selected = if config.dark_theme { 0 } else { 1 };
                if let Some(i) = widgets::segmented(ui, &["Dark", "Light"], selected) {
                    let dark = i == 0;
                    if config.dark_theme != dark {
                        config.dark_theme = dark;
                        theme::apply(ctx, dark);
                        config.save();
                    }
                }
            });
        });

        ui.add_space(widgets::GAP_WIDGETS);

        // ── Server Configuration (config.toml) — inline ──
        if let Some(editor) = settings_state.config_editor.as_mut() {
            // Error banner
            if let Some(error) = &editor.error_message {
                ui.allocate_ui(egui::vec2(SETTINGS_COLUMN_W, 0.0), |ui| {
                    widgets::panel_frame(ui, |ui| {
                        ui.horizontal(|ui| {
                            widgets::lamp_inline(ui, true, theme::error());
                            ui.label(text::note(error).color(theme::error()));
                        });
                    });
                });
                ui.add_space(widgets::GAP_WIDGETS);
            }

            // Model Directories — Browse buttons spawn worker threads
            // for the file dialog rather than blocking the egui main
            // thread. On Linux the synchronous rfd path deadlocks
            // against the XDG portal because the portal needs the main
            // loop alive to dispatch events to it. Two non-obvious
            // pieces required to make this work reliably:
            //
            //   1. set_directory only fires for non-empty start paths.
            //      Passing "" through to rfd causes the XDG portal to
            //      reject the FileChooser request (returns immediately
            //      without showing UI) — looks identical to "the button
            //      did nothing" from the user's perspective.
            //
            //   2. ctx.request_repaint() must fire on the worker thread
            //      AFTER writing the result. The drain in render() only
            //      sees the new slot value if a frame draws — without
            //      a repaint signal, egui sleeps until the user moves
            //      the mouse, and the picked path silently sits in the
            //      Mutex unconsumed.
            // Browse uses the in-app folder picker (pure egui, no rfd
            // / XDG portal / zenity dependency) — guaranteed to work
            // regardless of the desktop environment. Removed the
            // previous Browse + "…" two-button layout: showing two
            // affordances that do nearly the same thing was confusing.
            // The rfd path is gone with it; no more silent-None failures
            // when zenity isn't installed.
            settings_panel(ui, "MODEL DIRECTORIES", |ui| {
                ui.label(text::label("OLLAMA MODELS"));
                ui.horizontal(|ui| {
                    TextEdit::singleline(&mut editor.ollama_models_dir)
                        .desired_width(ui.available_width() - FIELD_BUTTON_RESERVE)
                        .hint_text("~/.ollama/models")
                        .show(ui);
                    if ui
                        .add(egui::Button::new(text::note("Browse")))
                        .on_hover_text("Open the folder picker")
                        .clicked()
                    {
                        let start = inapp_picker_start(&editor.ollama_models_dir);
                        editor.inapp_picker = Some((
                            crate::config_editor::BrowseTarget::OllamaDir,
                            start,
                        ));
                    }
                });
                widgets::caption_row(ui, "Folder where Ollama keeps its downloaded models. Leave empty to use the standard location.");
                ui.add_space(widgets::GAP_LABEL);

                ui.label(text::label("HUGGINGFACE MODELS"));
                ui.horizontal(|ui| {
                    TextEdit::singleline(&mut editor.huggingface_models_dir)
                        .desired_width(ui.available_width() - FIELD_BUTTON_RESERVE)
                        .hint_text("~/.cache/huggingface/hub")
                        .show(ui);
                    if ui
                        .add(egui::Button::new(text::note("Browse")))
                        .on_hover_text("Open the folder picker")
                        .clicked()
                    {
                        let start = inapp_picker_start(&editor.huggingface_models_dir);
                        editor.inapp_picker = Some((
                            crate::config_editor::BrowseTarget::HuggingFaceDir,
                            start,
                        ));
                    }
                });
                widgets::caption_row(ui, "Folder where HuggingFace models are cached after download. Leave empty to use the standard location.");
            });

            ui.add_space(widgets::GAP_WIDGETS);

            // Server bind
            settings_panel(ui, "SERVER BIND", |ui| {
                config_row_pair(ui,
                    ("HOST", &mut editor.server_host),
                    ("PORT", &mut editor.server_port_str),
                );
                hint_row(ui,
                    "Use 127.0.0.1 for this computer only, or 0.0.0.0 to allow connections from other devices on your network",
                    "Port number the server listens on. Default 11435. Change if another app already uses this port.",
                );
            });

            ui.add_space(widgets::GAP_WIDGETS);

            // Inference
            settings_panel(ui, "INFERENCE", |ui| {
                ui.label(text::label("MODEL ID"));
                TextEdit::singleline(&mut editor.model_id)
                    .desired_width(ui.available_width())
                    .show(ui);
                widgets::caption_row(ui, "Name of the AI model to load when the server starts. Must match a model you have downloaded (e.g. devstral-small-2).");
                ui.add_space(widgets::GAP_WIDGETS);

                config_row_pair(ui,
                    ("MAX TOKENS", &mut editor.max_tokens_str),
                    ("CONTEXT LENGTH", &mut editor.context_length_str),
                );
                hint_row(ui,
                    "Limits how long each response can be. Higher = longer answers but slower.",
                    "How much conversation history the model can remember. Larger values use more memory.",
                );
                ui.add_space(widgets::GAP_LABEL);
                config_row_pair(ui,
                    ("TEMPERATURE", &mut editor.temperature_str),
                    ("TOP P", &mut editor.top_p_str),
                );
                hint_row(ui,
                    "Controls creativity. Low (0.1) = focused and predictable. High (1.5) = creative and varied.",
                    "Filters unlikely words. Lower values (e.g. 0.5) make output more focused. Usually fine at 0.9.",
                );
                ui.add_space(widgets::GAP_LABEL);
                config_row_pair(ui,
                    ("TOP K", &mut editor.top_k_str),
                    ("SEED", &mut editor.seed_str),
                );
                hint_row(ui,
                    "How many word choices the model considers at each step. Lower = more predictable.",
                    "A number that makes outputs repeatable. Same seed + same prompt = same answer.",
                );
            });

            ui.add_space(widgets::GAP_WIDGETS);

            // GPU & Device
            settings_panel(ui, "GPU AND DEVICE", |ui| {
                config_row_pair(ui,
                    ("DEVICE INDEX", &mut editor.device_index_str),
                    ("GPU MEMORY %", &mut editor.max_gpu_memory_fraction_str),
                );
                hint_row(ui,
                    "Which GPU to use if you have multiple. Leave empty to pick automatically.",
                    "How much of your GPU's memory to use (0.9 = 90%). Lower this if you get out-of-memory errors.",
                );
                ui.add_space(widgets::GAP_LABEL);
                config_row_pair(ui,
                    ("FORCE GPU LAYERS", &mut editor.force_gpu_layers_str),
                    ("CPU THREADS", &mut editor.cpu_threads_str),
                );
                hint_row(ui,
                    "Force a specific number of model layers onto the GPU. Leave empty to let the server decide based on available memory.",
                    "Number of CPU cores to use for processing. Set to 0 to use all available cores automatically.",
                );
                ui.add_space(widgets::GAP_LABEL);
                ui.checkbox(&mut editor.use_quantized_gpu, text::note("Use quantized GPU"));
                widgets::caption_row(ui, "Compresses the model to use less GPU memory at a small quality cost. Recommended if your GPU has limited memory (8 GB or less).");
            });

            ui.add_space(widgets::GAP_WIDGETS);

            // Save / Reload / Reset row
            settings_panel(ui, "ACTIONS", |ui| {
                ui.horizontal(|ui| {
                    // Save config.toml
                    let save_btn = widgets::primary_button("Save config.toml");
                    if ui
                        .add(save_btn)
                        .on_hover_text(
                            "Write the model-directory + server + inference parameters\n\
                             above to config.toml. Requires a server restart for the\n\
                             server-config changes to take effect.",
                        )
                        .clicked()
                    {
                        match editor.to_config() {
                            Ok(cfg) => {
                                actions.push(SettingsAction::SaveConfig(cfg));
                                editor.error_message = None;
                            }
                            Err(e) => {
                                editor.error_message = Some(e);
                            }
                        }
                    }

                    ui.add_space(widgets::GAP_WIDGETS);

                    // Reload from disk
                    let reload_btn = egui::Button::new(text::note("Reload from disk"));
                    if ui
                        .add(reload_btn)
                        .on_hover_text(
                            "Discard any unsaved edits in this section and reload\n\
                             the values from config.toml on disk.",
                        )
                        .clicked()
                    {
                        if let Ok(cfg) = crate::config_editor::Config::load_default() {
                            *editor = ConfigEditorState::from_config(&cfg);
                        }
                    }

                    ui.add_space(widgets::GAP_WIDGETS);

                    // Reset GUI defaults — two-step confirm via modal,
                    // since this wipes profiles + server URL + selected
                    // model in one click. The modal lives at the top
                    // of this render fn; this button only sets the
                    // pending flag. While the modal is already open
                    // disable the button so re-clicking it doesn't
                    // re-trigger or visually hint that a second click
                    // is needed.
                    let reset_btn = egui::Button::new(text::note("Reset GUI defaults").color(theme::error()));
                    if ui
                        .add_enabled(!settings_state.reset_confirm_pending, reset_btn)
                        .clicked()
                    {
                        settings_state.reset_confirm_pending = true;
                    }
                });
            });
        }
    });

    actions
}

// ── Helper components ──

/// The servers: each with what it is and what can be done with it, then the form that
/// adds one or edits one.
fn connections_panel(
    ui: &mut egui::Ui,
    config: &mut AppConfig,
    settings_state: &mut SettingsState,
    connection_state: crate::state::ConnectionState,
) {
    settings_panel(ui, "CONNECTIONS", |ui| {
        widgets::caption_row(
            ui,
            "The servers this app can talk to. The active one serves the chat, the studio and the models.",
        );
        ui.add_space(widgets::GAP_LABEL);

        let active = config.active_connection.clone();
        let only_one = config.connections.len() <= 1;
        let mut activate: Option<String> = None;
        let mut edit: Option<crate::connection::Connection> = None;
        let mut remove: Option<String> = None;

        widgets::list_card(ui, |ui| {
            for (index, connection) in config.connections.iter().enumerate() {
                let is_active = active.as_deref() == Some(connection.name.as_str());
                widgets::list_row(ui, index == 0, |ui| {
                    ui.horizontal(|ui| {
                        // The lamp of the active connection follows the connection itself.
                        let lit = is_active
                            && connection_state != crate::state::ConnectionState::Disconnected;
                        widgets::lamp_inline(ui, lit, connection_state.color());
                        ui.vertical(|ui| {
                            ui.label(text::value(&connection.name));
                            let mut detail = connection.url.clone();
                            if connection.api_key.is_some() {
                                detail.push_str("   key set");
                            }
                            ui.label(text::readout(&detail));
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let tip = if only_one {
                                "Keep at least one connection"
                            } else {
                                "Delete this connection"
                            };
                            if ui
                                .add_enabled(!only_one, widgets::quiet_button("Delete"))
                                .on_hover_text(tip)
                                .clicked()
                            {
                                remove = Some(connection.name.clone());
                            }
                            if ui
                                .add(widgets::quiet_button("Edit"))
                                .on_hover_text("Change the name, the address or the key")
                                .clicked()
                            {
                                edit = Some(connection.clone());
                            }
                            if is_active {
                                widgets::badge(ui, "Active");
                            } else if ui
                                .add(egui::Button::new(text::note("Use")))
                                .on_hover_text("Connect to this server")
                                .clicked()
                            {
                                activate = Some(connection.name.clone());
                            }
                        });
                    });
                });
            }
        });
        ui.add_space(widgets::GAP_WIDGETS);

        if let Some(name) = activate {
            config.activate_connection(&name);
            config.save();
        }
        if let Some(connection) = edit {
            settings_state.connection_form.start_edit(&connection);
        }
        if let Some(name) = remove {
            if config.remove_connection(&name).is_ok() {
                if settings_state.connection_form.editing.as_deref() == Some(name.as_str()) {
                    settings_state.connection_form.start_new(config);
                }
                config.save();
            }
        }

        ui.add_space(widgets::GAP_LABEL);
        let editing = settings_state.connection_form.editing.is_some();
        ui.label(text::label(if editing {
            "EDIT CONNECTION"
        } else {
            "ADD A CONNECTION"
        }));
        ui.add_space(widgets::GAP_LABEL);
        let form = &mut settings_state.connection_form;
        form_row(ui, "NAME", |ui| {
            ui.add(TextEdit::singleline(&mut form.name).desired_width(f32::INFINITY));
        });
        form_row(ui, "ADDRESS", |ui| {
            ui.add(
                TextEdit::singleline(&mut form.url)
                    .hint_text(crate::config::DEFAULT_LOKEN_URL)
                    .desired_width(f32::INFINITY),
            );
        });
        form_row(ui, "API KEY", |ui| {
            ui.add(
                TextEdit::singleline(&mut form.api_key)
                    .password(true)
                    .hint_text("only if the server requires one")
                    .desired_width(f32::INFINITY),
            );
        });
        if let Some(why) = form.error {
            ui.add_space(widgets::GAP_LABEL);
            ui.label(text::note(why).color(theme::error()));
        }
        ui.add_space(widgets::GAP_WIDGETS);
        let mut submit = false;
        let mut cancel = false;
        ui.horizontal(|ui| {
            let label = if editing { "Save" } else { "Add connection" };
            let btn = widgets::primary_button(label);
            if ui.add(btn).clicked() {
                submit = true;
            }
            if editing && ui.add(egui::Button::new(text::note("Cancel"))).clicked() {
                cancel = true;
            }
        });
        if submit && form.submit(config) {
            config.save();
        }
        if cancel {
            form.start_new(config);
        }
        widgets::caption_row(
            ui,
            "The key is sent as a bearer token on every request. Leave it empty unless the server requires one.",
        );
    });
}

/// A section panel in the settings column.
fn settings_panel(ui: &mut egui::Ui, caps: &str, add_body: impl FnOnce(&mut egui::Ui)) {
    ui.allocate_ui(egui::vec2(SETTINGS_COLUMN_W, 0.0), |ui| {
        widgets::section_panel(ui, caps, add_body);
    });
}

/// A form row: the label in capitals in a column of fixed width, right
/// aligned, and the content beside it.
fn form_row(ui: &mut egui::Ui, caps: &str, add_content: impl FnOnce(&mut egui::Ui)) {
    ui.add_space(widgets::GAP_LABEL);
    ui.label(text::field_label(&sentence_case(caps)));
    ui.add_space(widgets::GAP_LABEL);
    add_content(ui);
    ui.add_space(widgets::GAP_WIDGETS);
}

/// `API KEY` becomes `API key`, `SERVER URL` becomes `Server URL`.
fn sentence_case(caps: &str) -> String {
    const ACRONYMS: [&str; 5] = ["API", "URL", "GPU", "VRAM", "MIDI"];
    caps.split(' ')
        .enumerate()
        .map(|(i, word)| {
            if ACRONYMS.contains(&word) {
                return word.to_string();
            }
            let lower = word.to_lowercase();
            let mut chars = lower.chars();
            match (i, chars.next()) {
                (0, Some(first)) => first.to_uppercase().chain(chars).collect(),
                _ => lower,
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Two labelled text fields side by side.
fn config_row_pair(
    ui: &mut egui::Ui,
    (caps_a, val_a): (&str, &mut String),
    (caps_b, val_b): (&str, &mut String),
) {
    ui.columns(2, |cols| {
        cols[0].label(text::label(caps_a));
        TextEdit::singleline(val_a)
            .desired_width(f32::INFINITY)
            .show(&mut cols[0]);

        cols[1].label(text::label(caps_b));
        TextEdit::singleline(val_b)
            .desired_width(f32::INFINITY)
            .show(&mut cols[1]);
    });
}

/// Two captions side by side, under a `config_row_pair`.
fn hint_row(ui: &mut egui::Ui, hint_a: &str, hint_b: &str) {
    ui.columns(2, |cols| {
        widgets::caption_row(&mut cols[0], hint_a);
        widgets::caption_row(&mut cols[1], hint_b);
    });
}

/// Pick a starting directory for the in-app folder picker. Falls
/// back through: current field value → $HOME → "/" so the picker
/// always opens on a valid directory.
fn inapp_picker_start(current: &str) -> std::path::PathBuf {
    let trimmed = current.trim();
    if !trimmed.is_empty() {
        let p = std::path::PathBuf::from(trimmed);
        if p.is_dir() {
            return p;
        }
    }
    if let Some(home) = std::env::var_os("HOME") {
        return std::path::PathBuf::from(home);
    }
    std::path::PathBuf::from("/")
}

/// Render the in-app folder picker modal. Pure egui — no rfd, no XDG
/// portal, no zenity. Lists subdirectories of the current path, lets
/// the user navigate via ".." (parent), click any subdir to descend,
/// or "Use this folder" to commit the displayed path to the editor
/// field. Cancel / Escape closes without committing.
fn render_inapp_folder_picker(ctx: &egui::Context, editor: &mut ConfigEditorState) {
    let Some((target, current)) = editor.inapp_picker.clone() else {
        return;
    };
    let mut new_path: Option<std::path::PathBuf> = None;
    let mut commit = false;
    let mut close = false;

    egui::Window::new("Choose a folder")
        .collapsible(false)
        .resizable(true)
        .default_width(PICKER_W)
        .default_height(PICKER_H)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            // Editable path bar — lets the user paste a full path and
            // press Enter to jump there instead of clicking subfolders
            // one at a time. The Go button is disabled when the typed
            // path isn't a directory (visible feedback that the path
            // is wrong instead of silent "Enter does nothing").
            ui.horizontal(|ui| {
                ui.label(text::label("PATH"));
                let mut path_str = current.display().to_string();
                let resp = ui.add(
                    egui::TextEdit::singleline(&mut path_str)
                        .desired_width(ui.available_width() - PICKER_GO_RESERVE)
                        .font(egui::TextStyle::Monospace),
                );
                let typed_path = std::path::PathBuf::from(path_str.trim());
                let typed_valid = typed_path.is_dir();
                let go_btn_resp = ui.add_enabled(typed_valid, egui::Button::new(text::note("Go")));
                if !typed_valid && path_str.trim() != current.display().to_string() {
                    go_btn_resp
                        .clone()
                        .on_hover_text("Path doesn't exist or isn't a directory");
                }
                let go = go_btn_resp.clicked()
                    || (resp.lost_focus()
                        && ui.input(|i| i.key_pressed(egui::Key::Enter))
                        && typed_valid);
                if go {
                    new_path = Some(typed_path);
                }
            });
            ui.add_space(widgets::GAP_LABEL);

            // Quick jumps: $HOME, /
            ui.horizontal(|ui| {
                if let Some(home) = std::env::var_os("HOME") {
                    let home_resp = ui.add(
                        egui::Button::image_and_text(
                            Icon::Home.image(ICON_PT, theme::ink_dim()),
                            text::note("Home"),
                        )
                        .frame(false),
                    );
                    if home_resp.on_hover_text("Jump to $HOME").clicked() {
                        new_path = Some(std::path::PathBuf::from(home));
                    }
                }
                if ui
                    .add(egui::Button::new(text::note("/ Root")).frame(false))
                    .on_hover_text("Jump to /")
                    .clicked()
                {
                    new_path = Some(std::path::PathBuf::from("/"));
                }
            });
            ui.add_space(widgets::GAP_LABEL);

            // The list, keeping room for the action row under it.
            let list_h = (ui.available_height() - PICKER_ACTIONS_RESERVE).max(PICKER_LIST_MIN_H);
            egui::ScrollArea::vertical()
                .id_salt("inapp_picker_scroll")
                .max_height(list_h)
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());

                    // Parent ("..") link — only renders when there IS
                    // a parent (we're not already at "/"). Plain ".."
                    // is the universal file-picker convention; no
                    // exotic-arrow Unicode required.
                    if current.parent().is_some()
                        && ui
                            .add(egui::Button::new(text::mono("..")).frame(false))
                            .clicked()
                    {
                        new_path = current.parent().map(std::path::PathBuf::from);
                    }

                    // List subdirectories, sorted alphabetically. Hide
                    // ones we can't read (permission denied) — they'd
                    // just frustrate the user with click-no-result.
                    let mut entries: Vec<std::path::PathBuf> = match std::fs::read_dir(&current) {
                        Ok(rd) => rd
                            .filter_map(|e| e.ok())
                            .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
                            .map(|e| e.path())
                            .collect(),
                        Err(e) => {
                            ui.colored_label(
                                theme::error(),
                                format!("Can't read directory: {}", e),
                            );
                            Vec::new()
                        }
                    };
                    entries.sort();

                    for entry in &entries {
                        let name = entry
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| entry.display().to_string());
                        // Show all directories including dotfiles —
                        // Ollama models live in ~/.ollama/models and
                        // HuggingFace cache in ~/.cache/... so hiding
                        // dotted entries would block the primary use
                        // case for this picker.
                        if ui
                            .add(
                                egui::Button::image_and_text(
                                    Icon::Folder.image(ICON_PT, theme::ink_dim()),
                                    text::note(&name).color(theme::ink()),
                                )
                                .frame(false),
                            )
                            .clicked()
                        {
                            new_path = Some(entry.clone());
                        }
                    }
                });

            ui.add_space(widgets::GAP_LABEL);
            ui.separator();
            ui.horizontal(|ui| {
                if ui.add(egui::Button::new(text::note("Cancel"))).clicked() {
                    close = true;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let commit_btn = widgets::primary_button("Use this folder");
                    if ui.add(commit_btn).clicked() {
                        commit = true;
                    }
                });
            });
        });

    // Escape closes without committing — standard modal shortcut.
    if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        close = true;
    }

    if commit {
        let p = current.to_string_lossy().to_string();
        match target {
            crate::config_editor::BrowseTarget::OllamaDir => editor.ollama_models_dir = p,
            crate::config_editor::BrowseTarget::HuggingFaceDir => editor.huggingface_models_dir = p,
        }
        editor.inapp_picker = None;
    } else if close {
        editor.inapp_picker = None;
    } else if let Some(p) = new_path {
        editor.inapp_picker = Some((target, p));
    }
}

#[cfg(test)]
mod label_tests {
    use super::sentence_case;

    #[test]
    fn a_capitalised_label_reads_as_a_sentence() {
        assert_eq!(sentence_case("NAME"), "Name");
        assert_eq!(sentence_case("API KEY"), "API key");
        assert_eq!(sentence_case("SERVER URL"), "Server URL");
        assert_eq!(sentence_case("MODEL ID"), "Model id");
    }
}
