//! Settings shared types.
//!
//!   - `SettingsState` / `SettingsAction` types (consumed by ui::settings
//!     and ui::models for action dispatch).
//!   - `ConnectionForm`, the form that adds or edits a connection.

use crate::config::AppConfig;
use crate::config_editor::ConfigEditorState;
use crate::connection::Connection;

/// Settings panel state for handling edits
pub struct SettingsState {
    pub connection_form: ConnectionForm,
    pub config_editor: Option<ConfigEditorState>,
    /// True when the user clicked "Reset GUI defaults" but hasn't
    /// confirmed yet. Renders a centered confirmation Window in
    /// ui::settings::render that wipes the AppConfig on Confirm or
    /// clears the flag on Cancel/Escape.
    pub reset_confirm_pending: bool,
}

impl SettingsState {
    pub fn new(_config: &AppConfig) -> Self {
        Self {
            connection_form: ConnectionForm::default(),
            config_editor: None,
            reset_confirm_pending: false,
        }
    }
}

/// The form that adds a connection, or edits one.
#[derive(Debug, Default)]
pub struct ConnectionForm {
    /// The name of the connection being edited. `None` when the form adds one.
    pub editing: Option<String>,
    pub name: String,
    pub url: String,
    pub api_key: String,
    /// Why the last submit was refused.
    pub error: Option<&'static str>,
}

impl ConnectionForm {
    /// Empty the form, ready to add a connection.
    pub fn start_new(&mut self, config: &AppConfig) {
        *self = Self {
            name: config.unused_connection_name(),
            ..Self::default()
        };
    }

    /// Fill the form with a connection to edit.
    pub fn start_edit(&mut self, connection: &Connection) {
        *self = Self {
            editing: Some(connection.name.clone()),
            name: connection.name.clone(),
            url: connection.url.clone(),
            api_key: connection.api_key.clone().unwrap_or_default(),
            error: None,
        };
    }

    /// Add or update the connection the form describes. True when the config changed.
    /// On a refusal the form keeps what was typed and says why.
    pub fn submit(&mut self, config: &mut AppConfig) -> bool {
        let key = Some(self.api_key.as_str());
        let result = match self.editing.as_deref() {
            Some(old) => config.update_connection(old, &self.name, &self.url, key),
            None => config.add_connection(&self.name, &self.url, key),
        };
        match result {
            Ok(()) => {
                self.start_new(config);
                true
            }
            Err(why) => {
                self.error = Some(why);
                false
            }
        }
    }
}

/// Actions that can be triggered from the settings panel.
///
/// SaveConfig carries the whole Config (~hundreds of bytes); the other arms
/// are short Strings or unit. Same large_enum_variant rationale as
/// ConfigEditorAction — emitted per UI tick, drained immediately.
#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum SettingsAction {
    RefreshModels,
    LoadModel(String),
    UnloadModel(String),
    DeleteModel(String),
    PullModel(String, String), // (model_name, source)
    SaveConfig(crate::config_editor::Config),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn filled(name: &str, url: &str) -> ConnectionForm {
        ConnectionForm {
            name: name.into(),
            url: url.into(),
            ..ConnectionForm::default()
        }
    }

    #[test]
    fn a_new_connection_is_added_with_a_clean_address() {
        let mut config = AppConfig::default();
        let mut form = filled("Workstation", " 10.0.0.5:11435/ ");
        assert!(form.submit(&mut config));
        let added = config
            .connections
            .iter()
            .find(|c| c.name == "Workstation")
            .expect("added");
        assert_eq!(added.url, "http://10.0.0.5:11435");
        assert_eq!(added.api_key, None, "a blank key is no key");
    }

    #[test]
    fn a_refused_submit_keeps_what_was_typed_and_says_why() {
        let mut config = AppConfig::default();
        let mut form = filled("Local", "http://x:1");
        assert!(!form.submit(&mut config), "the name is taken");
        assert_eq!(
            form.error,
            Some("Another connection already has this name.")
        );
        assert_eq!(form.url, "http://x:1");

        let mut empty = filled("Other", "  ");
        assert!(!empty.submit(&mut config));
        assert_eq!(config.connections.len(), 1);
    }

    #[test]
    fn renaming_the_active_connection_keeps_it_active() {
        let mut config = AppConfig::default();
        let mut form = ConnectionForm::default();
        form.start_edit(&config.connections[0].clone());
        form.name = "Home".into();
        assert!(form.submit(&mut config));
        assert_eq!(config.active().map(|c| c.name.as_str()), Some("Home"));
    }

    #[test]
    fn the_last_connection_stays_and_a_removed_active_one_is_replaced() {
        let mut config = AppConfig::default();
        assert!(config.remove_connection("Local").is_err());
        config.add_connection("Other", "http://o:1", None).unwrap();
        assert!(config.remove_connection("Local").is_ok());
        assert_eq!(config.active().map(|c| c.name.as_str()), Some("Other"));
    }

    #[test]
    fn a_session_address_is_dropped_when_a_connection_is_chosen() {
        let mut config = AppConfig {
            session_url: Some("http://cli:1".into()),
            ..AppConfig::default()
        };
        assert_eq!(config.server_url(), "http://cli:1");
        assert!(config.activate_connection("Local"));
        assert_eq!(config.server_url(), crate::config::DEFAULT_LOKEN_URL);
        assert!(!config.activate_connection("Nope"));
    }
}
