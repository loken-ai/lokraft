//! The generation parameters a person has changed.
//!
//! Every field is empty until set, and an empty field is not sent: the server applies the
//! defaults of the model it runs. Only a value a person chose reaches a request.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GenParams {
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<u32>,
    pub repeat_penalty: Option<f32>,
    /// Context length in tokens.
    pub num_ctx: Option<u32>,
    /// Most tokens to generate. Sent as `num_predict`, the name the chat route reads.
    pub max_tokens: Option<i32>,
    pub seed: Option<u64>,
    pub stop: Vec<String>,
}

impl GenParams {
    /// True when nothing is set, so a request carries no options of its own.
    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }

    /// The `options` object of a chat request. `None` when nothing is set.
    pub fn to_options(&self) -> Option<serde_json::Value> {
        if self.is_empty() {
            return None;
        }
        let mut o = serde_json::Map::new();
        if let Some(v) = self.temperature {
            o.insert("temperature".into(), v.into());
        }
        if let Some(v) = self.top_p {
            o.insert("top_p".into(), v.into());
        }
        if let Some(v) = self.top_k {
            o.insert("top_k".into(), v.into());
        }
        if let Some(v) = self.repeat_penalty {
            o.insert("repeat_penalty".into(), v.into());
        }
        if let Some(v) = self.num_ctx {
            o.insert("num_ctx".into(), v.into());
        }
        if let Some(v) = self.max_tokens {
            o.insert("num_predict".into(), v.into());
        }
        if let Some(v) = self.seed {
            o.insert("seed".into(), v.into());
        }
        if !self.stop.is_empty() {
            o.insert("stop".into(), serde_json::json!(self.stop));
        }
        Some(serde_json::Value::Object(o))
    }

    /// Stop sequences as one comma separated line, the form the field is edited in.
    pub fn stop_line(&self) -> String {
        self.stop.join(", ")
    }

    /// Set the stop sequences from a comma separated line. Blank entries are dropped.
    pub fn set_stop_line(&mut self, line: &str) {
        self.stop = line
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nothing_set_sends_no_options() {
        assert!(GenParams::default().to_options().is_none());
    }

    #[test]
    fn only_the_values_set_are_sent() {
        let p = GenParams {
            temperature: Some(0.3),
            max_tokens: Some(256),
            ..Default::default()
        };
        let o = p.to_options().expect("options");
        let obj = o.as_object().expect("object");
        assert_eq!(obj.len(), 2);
        assert_eq!(o["num_predict"], 256);
        assert!(obj.get("max_tokens").is_none());
        assert!(obj.get("top_p").is_none());
    }

    #[test]
    fn the_stop_line_round_trips_and_drops_blanks() {
        let mut p = GenParams::default();
        p.set_stop_line(" ###, , ");
        assert_eq!(p.stop, ["###"]);
        p.set_stop_line("a, b");
        assert_eq!(p.stop_line(), "a, b");
        p.set_stop_line("");
        assert!(p.is_empty());
    }
}
