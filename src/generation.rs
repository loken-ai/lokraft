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

/// The text a person typed in each field of the parameters panel.
/// Kept apart from `GenParams` so a half typed number such as `0.` survives a frame.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GenFields {
    pub temperature: String,
    pub top_p: String,
    pub top_k: String,
    pub repeat_penalty: String,
    pub num_ctx: String,
    pub max_tokens: String,
    pub seed: String,
    pub stop: String,
}

/// One field of the panel, to name the ones that do not parse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenField {
    Temperature,
    TopP,
    TopK,
    RepeatPenalty,
    NumCtx,
    MaxTokens,
    Seed,
}

impl GenFields {
    pub fn from_params(p: &GenParams) -> Self {
        fn show<T: ToString>(v: Option<T>) -> String {
            v.map(|v| v.to_string()).unwrap_or_default()
        }
        Self {
            temperature: show(p.temperature),
            top_p: show(p.top_p),
            top_k: show(p.top_k),
            repeat_penalty: show(p.repeat_penalty),
            num_ctx: show(p.num_ctx),
            max_tokens: show(p.max_tokens),
            seed: show(p.seed),
            stop: p.stop_line(),
        }
    }

    /// Write the fields that parse into `params`. An empty field clears its parameter.
    /// A field that does not parse leaves its parameter as it was and is returned.
    pub fn apply(&self, params: &mut GenParams) -> Vec<GenField> {
        fn parse<T: std::str::FromStr>(
            text: &str,
            ok: impl Fn(&T) -> bool,
        ) -> Result<Option<T>, ()> {
            let t = text.trim();
            if t.is_empty() {
                return Ok(None);
            }
            t.parse::<T>().ok().filter(|v| ok(v)).map(Some).ok_or(())
        }
        let mut bad = Vec::new();
        macro_rules! set {
            ($field:ident, $which:expr, $ok:expr) => {
                match parse(&self.$field, $ok) {
                    Ok(v) => params.$field = v,
                    Err(()) => bad.push($which),
                }
            };
        }
        set!(temperature, GenField::Temperature, |v: &f32| v.is_finite()
            && *v >= 0.0);
        set!(top_p, GenField::TopP, |v: &f32| *v > 0.0 && *v <= 1.0);
        set!(top_k, GenField::TopK, |_: &u32| true);
        set!(repeat_penalty, GenField::RepeatPenalty, |v: &f32| v
            .is_finite()
            && *v > 0.0);
        set!(num_ctx, GenField::NumCtx, |v: &u32| *v > 0);
        set!(max_tokens, GenField::MaxTokens, |v: &i32| *v >= -1);
        set!(seed, GenField::Seed, |_: &u64| true);
        params.set_stop_line(&self.stop);
        bad
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

    #[test]
    fn typed_fields_set_clear_and_reject() {
        let mut p = GenParams::default();
        let mut f = GenFields {
            temperature: "0.3".into(),
            top_k: "40".into(),
            stop: "a, b".into(),
            ..Default::default()
        };
        assert!(f.apply(&mut p).is_empty());
        assert_eq!(p.temperature, Some(0.3));
        assert_eq!(p.top_k, Some(40));
        assert_eq!(p.stop, ["a", "b"]);

        // A value that does not parse keeps the previous one and is named.
        f.temperature = "0.".into();
        f.top_p = "1.5".into();
        let bad = f.apply(&mut p);
        assert_eq!(p.temperature, Some(0.0), "0. parses as 0");
        assert_eq!(bad, [GenField::TopP]);
        assert_eq!(p.top_p, None);

        f.temperature = "abc".into();
        assert_eq!(f.apply(&mut p), [GenField::Temperature, GenField::TopP]);
        assert_eq!(p.temperature, Some(0.0), "the last good value stays");

        // An empty field clears its parameter.
        f.temperature = String::new();
        f.top_p = String::new();
        f.top_k = String::new();
        f.stop = String::new();
        assert!(f.apply(&mut p).is_empty());
        assert!(p.is_empty());
    }

    #[test]
    fn fields_show_the_parameters_they_were_made_from() {
        let p = GenParams {
            temperature: Some(0.5),
            max_tokens: Some(-1),
            stop: vec!["x".into()],
            ..Default::default()
        };
        let f = GenFields::from_params(&p);
        assert_eq!(f.temperature, "0.5");
        assert_eq!(f.max_tokens, "-1");
        assert_eq!(f.stop, "x");
        assert_eq!(f.top_p, "");
    }
}
