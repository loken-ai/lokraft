//! Shared widgets. Everything here allocates layout and reads the palette
//! through `theme`; the painting is `surface`'s. Two rules hold throughout:
//! nothing grows or moves under the pointer, and a label passed as
//! capitals is drawn as given.

use eframe::egui::{
    self, text::LayoutJob, Align, Color32, CornerRadius, FontId, Label, Layout, Margin, Rect,
    Response, RichText, Role, Sense, Shape, Stroke, TextFormat, Ui, Vec2, WidgetInfo,
};

use crate::icons::Icon;
use crate::theme::{self, text, HAIRLINE};
use crate::ui::surface;

/// The same shape for every screen that has nothing to show: an icon, what is missing,
/// and what to do about it.
pub fn empty_state(ui: &mut Ui, icon: Icon, title: &str, hint: &str) {
    ui.add_space(GAP_SECTIONS);
    ui.vertical_centered(|ui| {
        icon.show(ui, text::EMPTY_STATE_ICON_PT, theme::ink_dim());
        ui.add_space(GAP_WIDGETS);
        ui.label(text::value(title));
        if !hint.is_empty() {
            ui.add_space(GAP_LABEL);
            ui.label(text::note(hint));
        }
    });
}

/// Height of a slider, which sets the size of its handle.
const SLIDER_H: f32 = 20.0;

/// Style the slider added next in `ui`: a handle that is a plain accent dot. Call it
/// inside a scope, so the rest of the row is left alone.
pub fn style_slider(ui: &mut Ui) {
    let style = ui.style_mut();
    style.spacing.interact_size.y = SLIDER_H;
    let w = &mut style.visuals.widgets;
    for state in [&mut w.inactive, &mut w.hovered, &mut w.active] {
        state.bg_fill = theme::accent();
        state.fg_stroke = Stroke::NONE;
    }
}

/// A small rounded label for a fact about what is shown, such as a size or a step count.
pub fn pill(ui: &mut Ui, label: &str) -> Response {
    egui::Frame::NONE
        .stroke(Stroke::new(HAIRLINE, theme::border()))
        .corner_radius(CornerRadius::same(255))
        .inner_margin(Margin::symmetric(8, 2))
        .show(ui, |ui| ui.label(text::note(label)))
        .response
}

/// A small tag that names a state, such as the active connection.
pub fn badge(ui: &mut Ui, label: &str) -> Response {
    egui::Frame::NONE
        .fill(theme::accent_soft())
        .corner_radius(CornerRadius::same(4))
        .inner_margin(Margin::symmetric(6, 1))
        .show(ui, |ui| {
            ui.label(
                RichText::new(label)
                    .font(FontId::new(
                        text::LABEL_PT,
                        egui::FontFamily::Name(theme::FONT_MEDIUM.into()),
                    ))
                    .color(theme::accent()),
            )
        })
        .response
}

/// A card that holds a list: a border, rounded corners, and rows set apart by hairlines.
pub fn list_card<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    egui::Frame::NONE
        .fill(theme::panel())
        .stroke(Stroke::new(HAIRLINE, theme::border()))
        .corner_radius(CornerRadius::same(PANEL_RADIUS))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = 0.0;
            add(ui)
        })
        .inner
}

/// One row of a `list_card`. `first` is false for every row after the first, which then gets
/// a hairline above it.
pub fn list_row<R>(ui: &mut Ui, first: bool, add: impl FnOnce(&mut Ui) -> R) -> R {
    if !first {
        let y = ui.cursor().top();
        let x = ui.max_rect().x_range();
        ui.painter()
            .hline(x, y, Stroke::new(HAIRLINE, theme::border()));
    }
    egui::Frame::NONE
        .inner_margin(Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            add(ui)
        })
        .inner
}

/// A group of exclusive choices in one bordered strip, the chosen one filled. Returns the
/// index clicked this frame.
pub fn segmented(ui: &mut Ui, labels: &[&str], selected: usize) -> Option<usize> {
    let mut clicked = None;
    egui::Frame::NONE
        .stroke(Stroke::new(HAIRLINE, theme::border()))
        .corner_radius(CornerRadius::same(theme::RADIUS_PX))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.horizontal(|ui| {
                for (i, label) in labels.iter().enumerate() {
                    let on = i == selected;
                    let (fill, ink) = if on {
                        (theme::accent_soft(), theme::accent())
                    } else {
                        (Color32::TRANSPARENT, theme::ink_dim())
                    };
                    let text = if on {
                        text::value(label).color(ink)
                    } else {
                        text::note(label).color(ink)
                    };
                    let button = egui::Button::new(text)
                        .fill(fill)
                        .stroke(Stroke::NONE)
                        .corner_radius(CornerRadius::same(theme::RADIUS_PX));
                    if ui.add(button).clicked() {
                        clicked = Some(i);
                    }
                }
            });
        });
    clicked
}

/// Size of the icon on a button.
const BUTTON_ICON_PT: f32 = 11.0;

/// The main action of a form: a solid accent fill.
pub fn primary_button(label: &str) -> egui::Button<'static> {
    egui::Button::new(text::value(label).color(theme::on_accent())).fill(theme::accent())
}

/// A main action that carries an icon.
pub fn primary_button_icon(icon: Icon, label: &str) -> egui::Button<'static> {
    egui::Button::image_and_text(
        icon.image(BUTTON_ICON_PT, theme::on_accent()),
        text::value(label).color(theme::on_accent()),
    )
    .fill(theme::accent())
}

/// An action that stops or destroys: a solid error fill.
pub fn danger_button(label: &str) -> egui::Button<'static> {
    egui::Button::new(text::value(label).color(theme::on_accent())).fill(theme::error())
}

/// A destructive or stopping action that carries an icon.
pub fn danger_button_icon(icon: Icon, label: &str) -> egui::Button<'static> {
    egui::Button::image_and_text(
        icon.image(BUTTON_ICON_PT, theme::on_accent()),
        text::value(label).color(theme::on_accent()),
    )
    .fill(theme::error())
}

/// A secondary action, drawn as dim text without a frame.
pub fn quiet_button(label: &str) -> egui::Button<'static> {
    egui::Button::new(text::note(label)).frame(false)
}

/// Width of a confirmation dialog.
const CONFIRM_W: f32 = 380.0;

/// What a confirmation asks, in the same words on every screen.
pub struct Confirm<'a> {
    pub title: &'a str,
    pub question: &'a str,
    /// What the action applies to, shown in monospace.
    pub subject: Option<&'a str>,
    pub detail: &'a str,
    /// The label of the action, which is also the label of its button.
    pub action: &'a str,
}

/// A confirmation before an action that cannot be undone. `Some(true)` when confirmed,
/// `Some(false)` when cancelled with the button or Escape, `None` while it waits.
/// The action never answers to Enter: a key held down must not destroy anything.
pub fn confirm_dialog(ctx: &egui::Context, c: &Confirm) -> Option<bool> {
    let mut confirmed = false;
    let mut cancelled = false;
    egui::Window::new(c.title)
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.set_min_width(CONFIRM_W);
            ui.add_space(GAP_LABEL);
            ui.label(text::value(c.question));
            if let Some(subject) = c.subject {
                ui.add_space(GAP_LABEL);
                well(ui, |ui| {
                    ui.label(text::mono(subject));
                });
            }
            if !c.detail.is_empty() {
                ui.add_space(GAP_WIDGETS);
                ui.label(text::note(c.detail));
            }
            ui.add_space(GAP_WIDGETS);
            ui.horizontal(|ui| {
                if ui.add(egui::Button::new(text::note("Cancel"))).clicked() {
                    cancelled = true;
                }
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.add(danger_button(c.action)).clicked() {
                        confirmed = true;
                    }
                });
            });
            ui.add_space(GAP_LABEL);
        });
    if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        cancelled = true;
    }
    if confirmed {
        Some(true)
    } else if cancelled {
        Some(false)
    } else {
        None
    }
}

/// Padding inside a section.
pub const SECTION_PADDING: f32 = 12.0;
/// Gap between widgets.
pub const GAP_WIDGETS: f32 = 8.0;
/// Gap between sections.
pub const GAP_SECTIONS: f32 = 16.0;
/// Gap between a control and its label.
pub const GAP_LABEL: f32 = 4.0;

/// The accent stripe beside a section title.
/// The stripe along the left edge of a row.
pub const ROW_STRIPE_W: f32 = 3.0;
/// The accent wash behind a section title row, and its overhang.
/// Space after a section title.
const HEADER_GAP_AFTER: f32 = 6.0;
/// A panel's corner and padding.
const PANEL_RADIUS: u8 = 8;
const PANEL_PADDING: i8 = 12;
/// A pill's corner.
const PILL_RADIUS: u8 = 3;
/// The underline of the selected tab, the gap between tabs, the lead-in.
const TAB_UNDERLINE_W: f32 = 2.0;
const TAB_GAP: f32 = 4.0;
const TAB_LEAD: f32 = 8.0;
/// Widths of fixed cells: a numeric readout, a wide readout, a slider track.
pub const READOUT_W: f32 = 56.0;
pub const READOUT_WIDE_W: f32 = 96.0;
pub const SLIDER_TRACK_W: f32 = 160.0;
/// A meter in a form.
pub const METER_SIZE: Vec2 = Vec2::new(240.0, 6.0);
/// The label column of a form and of a parameter grid.
pub const FORM_LABEL_W: f32 = 110.0;
pub const PARAM_LABEL_W: f32 = 96.0;
/// A history thumbnail.
pub const THUMB_PX: f32 = 84.0;
/// Height of a painted heading row.
const HEADING_ROW_H: f32 = 16.0;
/// A lamp and the cell it sits in.
const LAMP_R: f32 = 4.0;
pub const LAMP_CELL: f32 = 16.0;
/// A chrome row: its pinned height and its margins.
pub const CHROME_ROW_H: f32 = 26.0;
pub const CHROME_MARGIN_X: i8 = 10;
pub const CHROME_MARGIN_Y: i8 = 5;
/// An icon button's cell and the icon inside it.
const ICON_BUTTON_PX: f32 = 24.0;
const ICON_PT: f32 = 14.0;
/// An LED's halo and glint.
const LED_HALO_PX: f32 = 2.0;
const LED_HALO_A: u8 = 40;
const LED_GLINT_A_ON: u8 = 80;
const LED_GLINT_A_OFF: u8 = 20;
const LED_GLINT_OFFSET: f32 = 0.25;
const LED_GLINT_R: f32 = 0.25;
/// A close button's inset and stroke.
const CLOSE_PAD: f32 = 3.0;
const CLOSE_STROKE_W: f32 = 1.5;
/// Strength of the focus ring on a hovered icon button.
const RING_HOVER_GAMMA: f32 = 0.5;

/// A section title: an accent stripe, capitals in the accent, a wash behind
/// the row. `caps` is drawn as given.
pub fn section_header(ui: &mut Ui, caps: &str) {
    ui.label(text::section(caps));
    ui.add_space(HEADER_GAP_AFTER);
}

/// A flat panel on the floor: the panel fill, a hairline border, filling the
/// width it is given so panels tile.
pub fn panel_frame<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    egui::Frame::NONE
        .fill(theme::panel())
        .corner_radius(CornerRadius::same(PANEL_RADIUS))
        .inner_margin(PANEL_PADDING)
        .stroke(Stroke::new(HAIRLINE, theme::border()))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            add(ui)
        })
        .inner
}

/// A panel with a section title.
pub fn section_panel<R>(ui: &mut Ui, caps: &str, add: impl FnOnce(&mut Ui) -> R) -> R {
    let out = ui
        .vertical(|ui| {
            ui.set_width(ui.available_width());
            section_header(ui, caps);
            add(ui)
        })
        .inner;
    ui.add_space(GAP_SECTIONS / 2.0);
    ui.painter().hline(
        ui.min_rect().x_range(),
        ui.cursor().top(),
        Stroke::new(HAIRLINE, theme::border()),
    );
    ui.add_space(GAP_SECTIONS / 2.0);
    out
}

/// Lays `add` out with `SECTION_PADDING` around it, then paints `shape` of
/// the rect it took underneath it.
fn on_shape<R>(
    ui: &mut Ui,
    add: impl FnOnce(&mut Ui) -> R,
    shape: impl FnOnce(Rect) -> Shape,
) -> R {
    let under = ui.painter().add(Shape::Noop);
    let inner = egui::Frame::NONE
        .inner_margin(SECTION_PADDING as i8)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            add(ui)
        });
    ui.painter().set(under, shape(inner.response.rect));
    inner.inner
}

/// Content sunk in a well.
pub fn well<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    on_shape(ui, add, |rect| {
        surface::recess_shape(rect, surface::WELL_RADIUS)
    })
}

/// Content behind the glass of a screen.
pub fn screen_well<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    on_shape(ui, add, surface::screen_shape)
}

/// Content on a raised plate.
pub fn on_plate<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    on_shape(ui, add, |rect| {
        surface::plate_shape(rect, surface::PLATE_RADIUS)
    })
}

/// A chrome row: raised, bordered, one pinned height, its content centred
/// on one line.
pub fn chrome_row<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    let shown = egui::Frame::NONE
        .inner_margin(Margin::symmetric(CHROME_MARGIN_X, CHROME_MARGIN_Y))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.set_min_height(CHROME_ROW_H);
                add(ui)
            })
            .inner
        });
    let rect = shown.response.rect;
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(HAIRLINE, theme::border()),
    );
    shown.inner
}

/// A painted heading across the available width. Silkscreen: painter text,
/// so never for a label a test or a reader must find.
pub fn heading_row(ui: &mut Ui, caps: &str) {
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), HEADING_ROW_H),
        Sense::hover(),
    );
    surface::heading(ui, rect.left_top(), caps, rect.right());
}

/// Prose under a control, wrapped to the available width at the caption
/// pitch.
pub fn caption_row(ui: &mut Ui, s: &str) {
    let ink = theme::ink_dim();
    let mut job = LayoutJob::default();
    job.wrap.max_width = ui.available_width();
    job.append(
        s,
        0.0,
        TextFormat {
            font_id: FontId::proportional(text::CAPTION_PT),
            color: ink,
            line_height: Some(text::CAPTION_PITCH),
            ..Default::default()
        },
    );
    let galley = ui.fonts_mut(|f| f.layout_job(job));
    let (rect, _) = ui.allocate_exact_size(galley.size(), Sense::hover());
    ui.painter().galley(rect.min, galley, ink);
}

/// One option of a set, seated when selected.
pub fn selector_pill(ui: &mut Ui, label: &str, selected: bool) -> Response {
    let (fill, ink) = if selected {
        (theme::accent(), theme::on_accent())
    } else {
        (theme::raised(), theme::ink_dim())
    };
    ui.add(
        egui::Button::new(RichText::new(label).size(text::SECTION_PT).color(ink))
            .fill(fill)
            .corner_radius(CornerRadius::same(PILL_RADIUS)),
    )
}

/// A row of tabs; the selected one is underlined in the accent and never
/// bold, so its width does not change. Returns the tab clicked.
pub fn tab_bar(ui: &mut Ui, selected: usize, labels: &[&str]) -> Option<usize> {
    let mut clicked = None;
    ui.horizontal(|ui| {
        ui.add_space(TAB_LEAD);
        for (i, label) in labels.iter().enumerate() {
            let is_selected = i == selected;
            let ink = if is_selected {
                theme::accent()
            } else {
                theme::ink_dim()
            };
            let response = ui.add(
                egui::Button::new(RichText::new(*label).size(text::SECTION_PT).color(ink))
                    .frame(false),
            );
            if is_selected {
                let r = response.rect;
                ui.painter().hline(
                    r.x_range(),
                    r.bottom() + HAIRLINE,
                    Stroke::new(TAB_UNDERLINE_W, theme::accent()),
                );
            }
            if response.clicked() {
                clicked = Some(i);
            }
            ui.add_space(TAB_GAP);
        }
    });
    clicked
}

/// A label in a cell of exactly `width`; a long text truncates rather than
/// shoving what follows.
pub fn fixed_label(ui: &mut Ui, width: f32, text: RichText) -> Response {
    let size = Vec2::new(width, ui.spacing().interact_size.y);
    ui.allocate_ui_with_layout(size, Layout::left_to_right(Align::Center), |ui| {
        ui.set_min_width(width);
        ui.set_max_width(width);
        ui.add(Label::new(text).truncate());
    })
    .response
}

/// A monospace readout in a fixed cell.
pub fn readout(ui: &mut Ui, width: f32, s: &str) -> Response {
    fixed_label(ui, width, text::mono(s))
}

/// A drag value boxed to `width`, announced to assistive tools as `name`.
pub fn drag_fixed(ui: &mut Ui, dv: egui::DragValue<'_>, width: f32, name: &str) -> Response {
    let response = ui.add_sized(Vec2::new(width, ui.spacing().interact_size.y), dv);
    response.widget_info(|| WidgetInfo::labeled(Role::SpinButton, true, name));
    response
}

/// A lamp in its cell.
pub fn lamp_inline(ui: &mut Ui, on: bool, tint: Color32) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(LAMP_CELL), Sense::hover());
    surface::lamp(ui, rect.center(), LAMP_R, on, tint);
    response
}

/// A stripe along the left edge of `rect`.
pub fn stripe(ui: &Ui, rect: Rect, tint: Color32) {
    let bar = Rect::from_min_size(rect.left_top(), Vec2::new(ROW_STRIPE_W, rect.height()));
    ui.painter().rect_filled(bar, 0.0, tint);
}

/// An LED: a halo when lit, a body, a glint.
pub fn led(ui: &mut Ui, on: bool, tint: Color32, size: f32) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    let centre = rect.center();
    let r = size / 2.0 - LED_HALO_PX;
    let painter = ui.painter();
    if on {
        painter.circle_filled(centre, r + LED_HALO_PX, theme::tinted(tint, LED_HALO_A));
    }
    painter.circle_filled(centre, r, if on { tint } else { theme::palette().lamp_off });
    painter.circle_filled(
        centre - Vec2::splat(r * LED_GLINT_OFFSET),
        r * LED_GLINT_R,
        Color32::from_white_alpha(if on { LED_GLINT_A_ON } else { LED_GLINT_A_OFF }),
    );
    response
}

/// An icon in a square cell; a ring appears on hover, nothing grows.
pub fn icon_button(ui: &mut Ui, icon: Icon, tip: &str) -> Response {
    icon_button_lit(ui, icon, tip, false)
}

/// `icon_button`, drawn in the accent while `lit`: the state of the thing
/// the button drives.
pub fn icon_button_lit(ui: &mut Ui, icon: Icon, tip: &str, lit: bool) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(ICON_BUTTON_PX), Sense::click());
    let hovered = response.hovered();
    if hovered {
        let painter = ui.painter();
        painter.rect_filled(rect, theme::RADIUS, theme::raised());
        painter.rect_stroke(
            rect,
            theme::RADIUS,
            Stroke::new(HAIRLINE, theme::accent().gamma_multiply(RING_HOVER_GAMMA)),
            egui::StrokeKind::Inside,
        );
    }
    let ink = if lit {
        theme::accent()
    } else if hovered {
        theme::ink()
    } else {
        theme::ink_dim()
    };
    icon.image(ICON_PT, ink)
        .paint_at(ui, rect.shrink((ICON_BUTTON_PX - ICON_PT) / 2.0));
    response.widget_info(|| WidgetInfo::labeled(Role::Button, true, tip));
    response.on_hover_text(tip)
}

/// A close mark drawn with two strokes.
pub fn close_button(ui: &mut Ui, size: f32) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::click());
    let hovered = response.hovered();
    let painter = ui.painter();
    if hovered {
        painter.rect_filled(rect, theme::RADIUS, theme::raised());
    }
    let ink = if hovered {
        theme::error()
    } else {
        theme::ink_dim()
    };
    let stroke = Stroke::new(CLOSE_STROKE_W, ink);
    let r = rect.shrink(CLOSE_PAD);
    painter.line_segment([r.left_top(), r.right_bottom()], stroke);
    painter.line_segment([r.right_top(), r.left_bottom()], stroke);
    response.widget_info(|| WidgetInfo::labeled(Role::Button, true, "Close"));
    response
}

/// The tallest a popup may be: most of the window, so what overflows scrolls.
pub fn popup_max_h(ui: &Ui) -> f32 {
    ui.ctx().content_rect().height() * 0.7
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Run a confirmation in a headless frame and return what it answered.
    fn answer(act: impl FnOnce(&mut egui_kittest::Harness<'_>)) -> Option<bool> {
        let outcome = std::rc::Rc::new(std::cell::Cell::new(None));
        let sink = outcome.clone();
        let mut harness = crate::theme::test_harness(None, move |ui| {
            let asked = confirm_dialog(
                ui.ctx(),
                &Confirm {
                    title: "Delete model",
                    question: "Delete this model from disk?",
                    subject: Some("m"),
                    detail: "",
                    action: "Delete",
                },
            );
            if asked.is_some() {
                sink.set(asked);
            }
        });
        harness.run();
        act(&mut harness);
        harness.run();
        outcome.get()
    }

    #[test]
    fn a_confirmation_answers_to_its_buttons_and_escape_but_not_to_enter() {
        use egui_kittest::kittest::Queryable;
        assert_eq!(answer(|h| h.key_press(egui::Key::Enter)), None);
        assert_eq!(answer(|h| h.key_press(egui::Key::Escape)), Some(false));
        assert_eq!(answer(|h| h.get_by_label("Cancel").click()), Some(false));
        assert_eq!(answer(|h| h.get_by_label("Delete").click()), Some(true));
    }

    fn frame(dark: bool, body: impl FnMut(&mut Ui)) {
        let _skin = theme::skin_lock();
        let mut harness = crate::theme::test_harness(None, body);
        theme::apply(&harness.ctx, dark);
        harness.run();
    }

    /// Every widget lays out in both states and both skins without panicking.
    #[test]
    fn the_widgets_paint() {
        for dark in [true, false] {
            frame(dark, |ui| {
                section_panel(ui, "SECTION", |ui| {
                    ui.label("body");
                });
                well(ui, |ui| {
                    ui.label("in a well");
                });
                screen_well(ui, |ui| {
                    ui.label("on a screen");
                });
                on_plate(ui, |ui| {
                    ui.label("on a plate");
                });
                chrome_row(ui, |ui| {
                    ui.label(text::title("CHROME"));
                    readout(ui, READOUT_W, "42");
                });
                heading_row(ui, "HEADING");
                caption_row(ui, "a caption that wraps when the width runs out");
                caption_row(ui, "");
                selector_pill(ui, "ON", true);
                selector_pill(ui, "OFF", false);
                tab_bar(ui, 1, &["ONE", "TWO", "THREE"]);
                fixed_label(ui, READOUT_W, text::label("FIXED"));
                let mut v = 0.5f32;
                drag_fixed(ui, egui::DragValue::new(&mut v), READOUT_W, "Value");
                lamp_inline(ui, true, theme::accent());
                lamp_inline(ui, false, theme::accent());
                led(ui, true, theme::success(), 12.0);
                led(ui, false, theme::success(), 12.0);
                icon_button(ui, Icon::Refresh, "Refresh");
                close_button(ui, 16.0);
                let (rect, _) = ui.allocate_exact_size(Vec2::new(100.0, 20.0), Sense::hover());
                stripe(ui, rect, theme::accent());
            });
        }
    }

    #[test]
    fn fixed_label_never_grows() {
        frame(true, |ui| {
            let long = "x".repeat(200);
            let response = fixed_label(ui, 80.0, text::label(&long));
            assert_eq!(response.rect.width(), 80.0);
        });
    }

    #[test]
    fn a_tab_bar_reports_the_tab_clicked() {
        use egui_kittest::kittest::Queryable;
        let mut clicked = None;
        let mut harness = crate::theme::test_harness(None, |ui| {
            if let Some(i) = tab_bar(ui, 0, &["ONE", "TWO"]) {
                clicked = Some(i);
            }
        });
        harness.run();
        harness.get_by_label("TWO").click();
        harness.run();
        drop(harness);
        assert_eq!(clicked, Some(1));
    }
}
