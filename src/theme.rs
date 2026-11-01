//! Colour palette and egui visuals.
//!
//! The application is painted from one `Palette`, of which two instances exist:
//! `DARK` and `LIGHT`. Both are flat: a floor, a panel on it, a hover fill, a hairline
//! border, a field fill, one accent and three status colours. Only the ink flips polarity
//! between the two. The relations between the fields are pinned by the tests at the bottom
//! of this file.

use eframe::egui::{self, Color32, CornerRadius, Stroke};

/// One skin of the application. Every colour the app paints is a field here.
pub struct Palette {
    pub dark: bool,

    // The floor the content sits on, a panel on it, and the fill under a pointer.
    pub bg: Color32,
    pub panel: Color32,
    pub raised: Color32,
    pub border: Color32,
    // What a text field and a rail are filled with.
    pub field: Color32,

    // Ink. `ink_dim` clears WCAG AA (4.5:1) on `bg`, `panel`, `raised` and `field`.
    pub ink: Color32,
    pub ink_dim: Color32,

    // The one accent, a quieter variant, and the ink that sits on a solid accent
    // fill.
    pub accent: Color32,
    pub accent_dim: Color32,
    pub on_accent: Color32,

    // Status. Never used as an accent; each clears 3:1 on `bg` and `panel`.
    pub success: Color32,
    pub warning: Color32,
    pub error: Color32,

    // An indicator that is off.
    pub lamp_off: Color32,
}

pub const DARK: Palette = Palette {
    dark: true,
    bg: Color32::from_rgb(0x11, 0x12, 0x16),
    panel: Color32::from_rgb(0x18, 0x1A, 0x20),
    raised: Color32::from_rgb(0x20, 0x23, 0x2B),
    border: Color32::from_rgb(0x2B, 0x2E, 0x39),
    field: Color32::from_rgb(0x11, 0x12, 0x16),
    ink: Color32::from_rgb(0xE7, 0xE9, 0xF0),
    ink_dim: Color32::from_rgb(0x9C, 0xA2, 0xB3),
    accent: Color32::from_rgb(0xA5, 0x8C, 0xF5),
    accent_dim: Color32::from_rgb(0x7A, 0x66, 0xC4),
    on_accent: Color32::from_rgb(0x14, 0x11, 0x1F),
    success: Color32::from_rgb(0x5C, 0xC9, 0x8A),
    warning: Color32::from_rgb(0xE0, 0xB0, 0x4A),
    error: Color32::from_rgb(0xEE, 0x7D, 0x76),
    lamp_off: Color32::from_rgb(0x3A, 0x3E, 0x4C),
};

pub const LIGHT: Palette = Palette {
    dark: false,
    bg: Color32::from_rgb(0xF5, 0xF6, 0xF9),
    panel: Color32::from_rgb(0xFF, 0xFF, 0xFF),
    raised: Color32::from_rgb(0xEC, 0xEE, 0xF4),
    border: Color32::from_rgb(0xDC, 0xDF, 0xE8),
    field: Color32::from_rgb(0xF5, 0xF6, 0xF9),
    ink: Color32::from_rgb(0x18, 0x1A, 0x22),
    ink_dim: Color32::from_rgb(0x5B, 0x60, 0x70),
    accent: Color32::from_rgb(0x64, 0x44, 0xD4),
    accent_dim: Color32::from_rgb(0x8E, 0x76, 0xE6),
    on_accent: Color32::from_rgb(0xFF, 0xFF, 0xFF),
    success: Color32::from_rgb(0x1F, 0x7A, 0x46),
    warning: Color32::from_rgb(0x9A, 0x6A, 0x0A),
    error: Color32::from_rgb(0xB0, 0x3A, 0x34),
    lamp_off: Color32::from_rgb(0xC4, 0xC8, 0xD4),
};

// The active skin. `apply()` records it; every colour is read through
// `palette()` so a theme switch reaches the whole app.
use std::sync::atomic::{AtomicBool, Ordering};
static DARK_ACTIVE: AtomicBool = AtomicBool::new(true);

pub fn is_dark() -> bool {
    DARK_ACTIVE.load(Ordering::Relaxed)
}

pub fn palette() -> &'static Palette {
    if is_dark() {
        &DARK
    } else {
        &LIGHT
    }
}

/// The skin is one process-wide value, and the test binary runs its tests
/// on parallel threads. A test that applies a skin or compares two colours
/// of the palette holds this for its duration.
#[cfg(test)]
pub(crate) fn skin_lock() -> std::sync::MutexGuard<'static, ()> {
    static SKIN_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    SKIN_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

pub fn bg() -> Color32 {
    palette().bg
}
pub fn panel() -> Color32 {
    palette().panel
}
pub fn raised() -> Color32 {
    palette().raised
}
pub fn border() -> Color32 {
    palette().border
}
pub fn ink() -> Color32 {
    palette().ink
}
pub fn ink_dim() -> Color32 {
    palette().ink_dim
}
pub fn success() -> Color32 {
    palette().success
}
pub fn warning() -> Color32 {
    palette().warning
}
pub fn error() -> Color32 {
    palette().error
}
pub fn accent() -> Color32 {
    palette().accent
}
pub fn accent_dim() -> Color32 {
    palette().accent_dim
}
pub fn on_accent() -> Color32 {
    palette().on_accent
}

/// Build a tinted (unmultiplied-alpha) variant of `color`. `alpha` is the
/// unmultiplied alpha byte (0 = transparent, 255 = opaque); typical values
/// for tinted fills are 15-35.
pub fn tinted(color: Color32, alpha: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha)
}

// ============================================================================
// Spacing and Sizing
// ============================================================================

/// Corner radius of every card, button and well, in points.
pub const RADIUS_PX: u8 = 6;

/// `RADIUS_PX` on all four corners.
pub const RADIUS: CornerRadius = CornerRadius::same(RADIUS_PX);

// ============================================================================
// Theme Application
// ============================================================================

/// Stroke width of every border, rule and ring.
pub const HAIRLINE: f32 = 1.0;

/// Alpha of the shadow of a floating surface.
const FLOAT_SHADOW_A: u8 = 64;

/// Alpha of the accent wash under a selection.
const SELECTION_A: u8 = 90;

/// Alpha of the accent wash on a pressed widget.
const ACTIVE_WASH_A: u8 = 60;

/// Strength of the focus ring on hover; the full ring appears when pressed.
const FOCUS_RING_HOVER_GAMMA: f32 = 0.5;

/// Gap between widgets, horizontally, and the padding inside a button.
pub const GAP_WIDGETS: f32 = 8.0;

/// Gap between a control and its label, and the vertical pitch of a form.
pub const GAP_LABEL: f32 = 4.0;

/// Height of every interactive control.
pub const CONTROL_H: f32 = 28.0;

/// Width sliders and combos share so form columns line up.
pub const FORM_CONTROL_W: f32 = 240.0;

/// Thickness of a slider rail.
const SLIDER_RAIL_H: f32 = 4.0;

/// Padding inside windows and menus.
const PANEL_PADDING: i8 = 8;

/// Build the egui visuals and style for `dark` and install them on `ctx`.
/// Records the active skin for `palette()`.
pub fn apply(ctx: &egui::Context, dark: bool) {
    let p = if dark { &DARK } else { &LIGHT };
    DARK_ACTIVE.store(dark, Ordering::Relaxed);
    ctx.set_visuals(visuals_for(p));
    ctx.all_styles_mut(|style| style_for(p, style));
}

/// One widget state: a fill, a weaker fill for buttons, a border and an ink.
/// No expansion, so nothing grows under the pointer.
fn wv(
    bg_fill: Color32,
    weak_bg_fill: Color32,
    bg_stroke: Color32,
    fg: Color32,
) -> egui::style::WidgetVisuals {
    egui::style::WidgetVisuals {
        bg_fill,
        weak_bg_fill,
        bg_stroke: Stroke::new(HAIRLINE, bg_stroke),
        corner_radius: RADIUS,
        fg_stroke: Stroke::new(HAIRLINE, fg),
        expansion: 0.0,
    }
}

/// The shadow under a window or a menu: soft, and only where something floats.
fn float_shadow() -> egui::epaint::Shadow {
    egui::epaint::Shadow {
        offset: [0, 8],
        blur: 24,
        spread: 0,
        color: Color32::from_black_alpha(FLOAT_SHADOW_A),
    }
}

/// The egui visuals of one skin. Everything is flat: panels and fields are plain fills
/// with a hairline border, a hovered control takes the raised fill, and the accent
/// appears as a ring on hover and a wash when pressed.
pub fn visuals_for(p: &Palette) -> egui::Visuals {
    let mut v = if p.dark {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };
    v.panel_fill = p.panel;
    v.window_fill = p.panel;
    v.extreme_bg_color = p.field;
    v.text_edit_bg_color = Some(p.field);
    v.code_bg_color = p.field;
    v.faint_bg_color = p.raised;
    v.window_stroke = Stroke::new(HAIRLINE, p.border);
    v.window_corner_radius = RADIUS;
    v.menu_corner_radius = RADIUS;
    v.window_shadow = float_shadow();
    v.popup_shadow = float_shadow();
    v.selection.bg_fill = tinted(p.accent, SELECTION_A);
    v.selection.stroke = Stroke::new(HAIRLINE, p.accent);
    v.hyperlink_color = p.accent;
    v.warn_fg_color = p.warning;
    v.error_fg_color = p.error;
    v.widgets.noninteractive = wv(p.panel, p.panel, p.border, p.ink);
    v.widgets.inactive = wv(p.field, p.panel, p.border, p.ink);
    v.widgets.hovered = wv(
        p.raised,
        p.raised,
        p.accent.gamma_multiply(FOCUS_RING_HOVER_GAMMA),
        p.ink,
    );
    let wash = tinted(p.accent, ACTIVE_WASH_A);
    v.widgets.active = wv(wash, wash, p.accent, p.ink);
    v.widgets.open = wv(p.raised, p.raised, p.accent, p.ink);
    // A form is not zebra-striped: the label column and the rules carry it.
    v.striped = false;
    v.slider_trailing_fill = true;
    v.handle_shape = egui::style::HandleShape::Circle;
    v
}

/// The spacing and the type scale of one skin: a 4 px grid, 28 px controls,
/// one form width, five text styles.
pub fn style_for(_p: &Palette, style: &mut egui::Style) {
    style.text_styles = text::styles();
    style.spacing.item_spacing = egui::vec2(GAP_WIDGETS, GAP_LABEL);
    style.spacing.button_padding = egui::vec2(GAP_WIDGETS, GAP_LABEL);
    style.spacing.interact_size.y = CONTROL_H;
    style.spacing.slider_width = FORM_CONTROL_W;
    style.spacing.combo_width = FORM_CONTROL_W;
    style.spacing.slider_rail_height = SLIDER_RAIL_H;
    style.spacing.window_margin = egui::Margin::same(PANEL_PADDING);
    style.spacing.menu_margin = egui::Margin::same(PANEL_PADDING);
}

/// The type scale. Five sizes and a weight each; the colour is the palette's.
pub mod text {
    use super::palette;
    use eframe::egui::{FontFamily, FontId, RichText, TextStyle};
    use std::collections::BTreeMap;

    /// A label over or beside a control: capitals, dim. Callers pass capitals.
    pub const LABEL_PT: f32 = 11.0;
    /// The label of a field, above it.
    pub const LABEL_FIELD_PT: f32 = 12.0;
    /// A value, a button, a name: the reading size of a control.
    pub const VALUE_PT: f32 = 13.0;
    /// A section title: capitals in the accent.
    pub const SECTION_PT: f32 = 11.0;
    /// Body prose.
    pub const BODY_PT: f32 = 13.0;
    /// The title in a chrome row.
    pub const TITLE_PT: f32 = 16.0;
    /// Monospace readouts.
    pub const MONO_PT: f32 = 12.0;
    /// A dim monospace readout in a chrome tail.
    pub const READOUT_PT: f32 = 11.0;
    /// Prose under a control, and its line pitch.
    pub const CAPTION_PT: f32 = 11.0;
    pub const CAPTION_PITCH: f32 = 14.0;
    /// Tracked capitals over a hairline, and their letter spacing.
    pub const HEADING_PT: f32 = 11.0;
    pub const TRACKING: f32 = 1.0;
    /// The icon of an empty state.
    pub const EMPTY_STATE_ICON_PT: f32 = 32.0;

    /// egui's named styles on the same scale.
    pub fn styles() -> BTreeMap<TextStyle, FontId> {
        [
            (
                TextStyle::Body,
                FontId::new(BODY_PT, FontFamily::Proportional),
            ),
            (
                TextStyle::Button,
                FontId::new(VALUE_PT, FontFamily::Proportional),
            ),
            (
                TextStyle::Heading,
                FontId::new(TITLE_PT, FontFamily::Proportional),
            ),
            (
                TextStyle::Monospace,
                FontId::new(MONO_PT, FontFamily::Monospace),
            ),
            (
                TextStyle::Small,
                FontId::new(LABEL_PT, FontFamily::Proportional),
            ),
        ]
        .into()
    }

    pub fn label(caps: &str) -> RichText {
        RichText::new(caps).size(LABEL_PT).color(palette().ink_dim)
    }
    pub fn value(s: &str) -> RichText {
        RichText::new(s)
            .size(VALUE_PT)
            .strong()
            .color(palette().ink)
    }
    /// The label of a field, above it: medium weight, in the ink.
    pub fn field_label(s: &str) -> RichText {
        RichText::new(s)
            .font(FontId::new(
                LABEL_FIELD_PT,
                FontFamily::Name(super::FONT_MEDIUM.into()),
            ))
            .color(palette().ink)
    }
    pub fn section(caps: &str) -> RichText {
        RichText::new(caps)
            .size(SECTION_PT)
            .strong()
            .color(palette().accent)
    }
    pub fn title(s: &str) -> RichText {
        RichText::new(s)
            .size(TITLE_PT)
            .strong()
            .color(palette().ink)
    }
    pub fn note(s: &str) -> RichText {
        RichText::new(s).size(VALUE_PT).color(palette().ink_dim)
    }
    pub fn mono(s: &str) -> RichText {
        RichText::new(s)
            .monospace()
            .size(MONO_PT)
            .color(palette().ink)
    }
    pub fn readout(s: &str) -> RichText {
        RichText::new(s)
            .monospace()
            .size(READOUT_PT)
            .color(palette().ink_dim)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The two skins, named for assertion messages.
    const SKINS: [(&str, &Palette); 2] = [("dark", &DARK), ("light", &LIGHT)];

    /// WCAG AA floor for normal text.
    const AA_TEXT: f64 = 4.5;

    /// WCAG AA floor for user-interface components and large text.
    const AA_COMPONENT: f64 = 3.0;

    #[test]
    fn tinted_forwards_to_from_rgba_unmultiplied() {
        // Pin that tinted(c, a) == Color32::from_rgba_unmultiplied(c.r(),
        // c.g(), c.b(), a) for every alpha; egui's internal encoding is
        // premultiplied, so channel equality with the source cannot be
        // asserted directly.
        for &color in &[LIGHT.accent, DARK.success, DARK.warning, DARK.error] {
            for alpha in [0u8, 1, 18, 25, 30, 180, 200, 255] {
                let expected =
                    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha);
                let got = tinted(color, alpha);
                assert_eq!(
                    got, expected,
                    "tinted({color:?}, {alpha}) must equal the direct \
                     Color32::from_rgba_unmultiplied call it replaces"
                );
            }
        }
    }

    #[test]
    fn tinted_opaque_alpha_equals_source() {
        assert_eq!(tinted(LIGHT.accent, 255), LIGHT.accent);
        assert_eq!(tinted(DARK.success, 255), DARK.success);
    }

    #[test]
    fn tinted_zero_alpha_is_fully_transparent() {
        let t = tinted(LIGHT.accent, 0);
        assert_eq!(t, Color32::TRANSPARENT);
    }

    /// WCAG 2.1 relative luminance, in [0.0, 1.0].
    fn rel_luminance(c: Color32) -> f64 {
        fn chan(v: u8) -> f64 {
            let s = v as f64 / 255.0;
            if s <= 0.03928 {
                s / 12.92
            } else {
                ((s + 0.055) / 1.055).powf(2.4)
            }
        }
        0.2126 * chan(c.r()) + 0.7152 * chan(c.g()) + 0.0722 * chan(c.b())
    }

    fn contrast_ratio(a: Color32, b: Color32) -> f64 {
        let la = rel_luminance(a);
        let lb = rel_luminance(b);
        let (hi, lo) = if la > lb { (la, lb) } else { (lb, la) };
        (hi + 0.05) / (lo + 0.05)
    }

    /// Luminance on raw bytes, for ordering surfaces against each other.
    fn byte_lum(c: Color32) -> f64 {
        0.2126 * c.r() as f64 + 0.7152 * c.g() as f64 + 0.0722 * c.b() as f64
    }

    fn assert_contrast(what: &str, fg: Color32, bg: Color32, floor: f64) {
        let ratio = contrast_ratio(fg, bg);
        assert!(ratio >= floor, "{what}: contrast {ratio:.2}:1 < {floor}:1");
    }

    /// Manhattan distance in RGB bytes; two colours closer than
    /// `HUE_APART` read as the same hue at a glance.
    fn rgb_distance(a: Color32, b: Color32) -> i32 {
        (a.r() as i32 - b.r() as i32).abs()
            + (a.g() as i32 - b.g() as i32).abs()
            + (a.b() as i32 - b.b() as i32).abs()
    }
    const HUE_APART: i32 = 90;

    #[test]
    fn the_accent_is_not_a_status_colour() {
        // The accent marks selection and focus; a status colour marks an
        // outcome. Neither may be mistaken for the other, and no two
        // outcomes may share a hue.
        for (skin, p) in SKINS {
            let named = [
                ("accent", p.accent),
                ("success", p.success),
                ("warning", p.warning),
                ("error", p.error),
            ];
            for (i, (na, ca)) in named.iter().enumerate() {
                for (nb, cb) in named.iter().skip(i + 1) {
                    let d = rgb_distance(*ca, *cb);
                    assert!(
                        d > HUE_APART,
                        "{skin}: {na} and {nb} are {d} apart, under {HUE_APART}"
                    );
                }
            }
        }
    }

    #[test]
    fn ink_meets_wcag_aa_on_bg_and_panel_both_palettes() {
        for (skin, p) in SKINS {
            assert_contrast(&format!("{skin} ink on bg"), p.ink, p.bg, AA_TEXT);
            assert_contrast(&format!("{skin} ink on panel"), p.ink, p.panel, AA_TEXT);
        }
    }

    #[test]
    fn ink_dim_meets_wcag_aa_on_bg_and_panel_both_palettes() {
        // ink_dim carries hints, disabled labels and secondary metadata on
        // both the floor and the panels; a value that clears one and not the
        // other takes half of those sites under the floor.
        for (skin, p) in SKINS {
            assert_contrast(&format!("{skin} ink_dim on bg"), p.ink_dim, p.bg, AA_TEXT);
            assert_contrast(
                &format!("{skin} ink_dim on panel"),
                p.ink_dim,
                p.panel,
                AA_TEXT,
            );
        }
    }

    #[test]
    fn ink_dim_reads_on_the_hover_fill_and_the_field() {
        for (skin, p) in SKINS {
            assert_contrast(
                &format!("{skin} ink_dim on raised"),
                p.ink_dim,
                p.raised,
                AA_TEXT,
            );
            assert_contrast(
                &format!("{skin} ink_dim on field"),
                p.ink_dim,
                p.field,
                AA_TEXT,
            );
        }
    }

    #[test]
    fn status_reads_on_bg_and_panel_both_palettes() {
        for (skin, p) in SKINS {
            for (name, c) in [
                ("success", p.success),
                ("warning", p.warning),
                ("error", p.error),
            ] {
                assert_contrast(&format!("{skin} {name} on bg"), c, p.bg, AA_COMPONENT);
                assert_contrast(&format!("{skin} {name} on panel"), c, p.panel, AA_COMPONENT);
            }
        }
    }

    #[test]
    fn on_accent_reads_on_the_accent() {
        for (skin, p) in SKINS {
            assert_contrast(
                &format!("{skin} on_accent on accent"),
                p.on_accent,
                p.accent,
                AA_TEXT,
            );
        }
    }

    #[test]
    fn the_surfaces_step_away_from_the_floor() {
        // The panel stands one step off the floor and the hover fill one step
        // further, in the direction the skin lightens: up on dark, up on light too,
        // since the floor is the dimmest surface of each.
        for (skin, p) in SKINS {
            let (floor, panel, raised) = (byte_lum(p.bg), byte_lum(p.panel), byte_lum(p.raised));
            if p.dark {
                assert!(
                    floor < panel && panel < raised,
                    "{skin}: floor, panel, raised"
                );
            } else {
                assert!(
                    panel > floor && floor > raised,
                    "{skin}: panel, floor, raised"
                );
            }
            assert!(
                (byte_lum(p.field) - floor).abs() < 1.0,
                "{skin}: a field is the colour of the floor"
            );
        }
    }

    #[test]
    fn the_ink_is_stronger_than_the_dim_ink() {
        assert!(byte_lum(DARK.ink) > byte_lum(DARK.ink_dim));
        assert!(byte_lum(DARK.ink_dim) > byte_lum(DARK.panel));
        assert!(byte_lum(LIGHT.ink) < byte_lum(LIGHT.ink_dim));
        assert!(byte_lum(LIGHT.ink_dim) < byte_lum(LIGHT.panel));
    }

    #[test]
    fn the_light_skin_inverts_only_the_ink() {
        let surfaces = |p: &Palette| [p.bg, p.panel, p.border, p.field, p.lamp_off];
        for (d, l) in surfaces(&DARK).iter().zip(surfaces(&LIGHT).iter()) {
            assert!(
                byte_lum(*l) > byte_lum(*d),
                "every light surface is lighter than its dark twin"
            );
        }
        for (d, l) in [(DARK.ink, LIGHT.ink), (DARK.ink_dim, LIGHT.ink_dim)] {
            assert!(
                byte_lum(l) < byte_lum(d),
                "every light ink is darker than its dark twin"
            );
        }
    }

    #[test]
    fn apply_installs_the_palette() {
        let _skin = skin_lock();
        let ctx = egui::Context::default();
        apply(&ctx, false);
        assert!(!is_dark());
        assert_eq!(ctx.global_style().visuals.panel_fill, LIGHT.panel);
        assert_eq!(ctx.global_style().visuals.extreme_bg_color, LIGHT.field);
        assert_eq!(ctx.global_style().spacing.interact_size.y, CONTROL_H);
        assert_eq!(
            ctx.global_style().text_styles[&egui::TextStyle::Body].size,
            text::BODY_PT
        );
        apply(&ctx, true);
        assert!(is_dark());
        assert_eq!(ctx.global_style().visuals.panel_fill, DARK.panel);
        assert_eq!(ctx.global_style().visuals.extreme_bg_color, DARK.field);
        assert_eq!(ctx.global_style().visuals.widgets.hovered.expansion, 0.0);
    }
}
