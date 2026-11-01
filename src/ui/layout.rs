//! Dashboard layout — top bar + sidebar navigation + content area
//!
//! ```text
//! +-----------------------------------------------------+
//! |  TOP BAR  [Server status] [Active model] [Metrics]   |
//! +------+----------------------------------------------+
//! |      |                                              |
//! | NAV  |           MAIN CONTENT AREA                  |
//! | SIDE |                                              |
//! |      |                                              |
//! +------+----------------------------------------------+
//! ```

use crate::icons::Icon;
use crate::state::Section;
use crate::theme::{self, text, HAIRLINE};
use crate::ui::{surface, widgets};
use eframe::egui::{
    self, Align2, Color32, FontId, Pos2, Rect, Response, Sense, Stroke, Vec2, WidgetInfo,
    WidgetType,
};

/// Sidebar width (icon + label when expanded)
const SIDEBAR_WIDTH: f32 = 56.0;
const SIDEBAR_WIDTH_EXPANDED: f32 = 160.0;

/// Point size of a navigation icon or glyph.
const NAV_ICON_PT: f32 = 16.0;
/// Padding of the rail.
const SIDEBAR_PAD_X: i8 = 4;
const SIDEBAR_PAD_Y: i8 = 8;
/// A navigation item: its height, its corner, the wash behind the active
/// one, the gap to the next.
const NAV_ITEM_H: f32 = 32.0;
const NAV_RADIUS: f32 = 4.0;
const NAV_ITEM_GAP: f32 = 2.0;
/// Space above a group heading.
const NAV_GROUP_GAP: f32 = 12.0;
/// The stripe beside the active item.
/// Inset of the icon from the stripe, and the gap between icon and label.
const NAV_PAD_X: f32 = 8.0;
const NAV_GAP: f32 = 6.0;
/// The toggle row, and the chevron drawn in it.
const TOGGLE_H: f32 = 24.0;
use crate::ui::surface::{CHEVRON_STROKE_W, CHEVRON_W};

/// Sidebar navigation item. `icon` is an Icon variant rather than an emoji
/// character. Terminal keeps a text glyph (">_"), which is a command-prompt
/// convention rather than an emoji.
struct NavItem {
    icon: NavIcon,
    label: &'static str,
    /// One-line description shown in the hover tooltip alongside the
    /// label. Helps users learn what each section does — especially
    /// useful in collapsed-sidebar mode where only the icon is
    /// visible.
    tip: &'static str,
    section: Section,
    /// A heading shown above this item, which starts a group.
    group: Option<&'static str>,
}

/// Sidebar icon variants. Either an SVG-backed Icon or a literal
/// text glyph (for the Terminal section's ">_" command-prompt mark
/// where a generic icon would lose the semantics).
enum NavIcon {
    Svg(Icon),
    Text(&'static str),
}

/// The section that Ctrl+`n` opens, `n` counting from 1 in the order of the sidebar.
pub fn section_for_shortcut(n: usize) -> Option<Section> {
    NAV_ITEMS.get(n.checked_sub(1)?).map(|item| item.section)
}

const NAV_ITEMS: &[NavItem] = &[
    NavItem {
        icon: NavIcon::Svg(Icon::Chat),
        label: "Chat",
        tip: "Talk to a loaded model: text, vision, image generation, speech.",
        section: Section::Chat,
        group: None,
    },
    NavItem {
        icon: NavIcon::Svg(Icon::Bolt),
        label: "Studio",
        tip: "Generate images, music, sound effects, MIDI, video and speech.",
        section: Section::MediaStudio,
        group: None,
    },
    NavItem {
        icon: NavIcon::Svg(Icon::Package),
        label: "Models",
        tip: "Browse, install, load and delete models.",
        section: Section::Models,
        group: None,
    },
    NavItem {
        icon: NavIcon::Svg(Icon::Server),
        label: "Logs",
        tip: "This app's own log, with level and search filters.",
        section: Section::ServerLog,
        group: Some("SYSTEM"),
    },
    NavItem {
        icon: NavIcon::Text(">_"),
        label: "Terminal",
        tip: "Model commands: list, loaded, load, unload, pull, delete, ps, clear, help.",
        section: Section::Terminal,
        group: None,
    },
    NavItem {
        icon: NavIcon::Svg(Icon::Gear),
        label: "Settings",
        tip: "Connections, theme, config.toml editor.",
        section: Section::Settings,
        group: None,
    },
];

/// Render the top bar
// egui 0.34 deprecates `Panel::show(&Context)` in favour of
// `Panel::show_inside(&mut Ui)`, but the new form requires the panel
// to be rendered inside a parent Ui — there is no top-level Ui in the
// eframe entry point yet, so we keep the deprecated top-level form.
// When egui exposes a top-level Ui scaffold we'll migrate; until then,
// allow the deprecated call locally rather than across the whole crate.
/// Gap between the blocks of the chrome row, and inside its tail.
const CHROME_GAP: f32 = 12.0;
const CHROME_TAIL_GAP: f32 = 8.0;
/// Fixed cells of the chrome row: the status word, the model name, the
/// loaded count; and how many characters of a model name are shown.
/// The size of the mark, and how much white its top face takes.
const BRAND_PT: f32 = 20.0;
const BRAND_TOP_ALPHA: u8 = 70;
/// How many characters of a connection name the switcher shows, and the width of its list.
const CONNECTION_CHARS: usize = 24;
const CONNECTION_MENU_W: f32 = 240.0;
/// The meter in the tail.
const METER_SIZE_CHROME: Vec2 = Vec2::new(60.0, 8.0);

/// What the top bar shows.
pub struct TopBarInput<'a> {
    pub connection: crate::state::ConnectionState,
    pub connection_detail: &'a str,
    pub active_model: Option<&'a str>,
    pub connections: &'a [crate::connection::Connection],
    pub active_connection: Option<&'a str>,
    pub loaded: usize,
    pub server_running: bool,
    pub refreshing: bool,
    /// Work in flight, 0..1, drawn as a meter in the tail. None hides it.
    pub progress: Option<f32>,
}

/// What the top bar was asked: the Refresh button, the theme toggle.
pub struct TopBarOutput {
    pub refresh_clicked: bool,
    pub theme_toggle_clicked: bool,
    /// A connection picked in the switcher.
    pub connection_chosen: Option<String>,
    /// The switcher's link to the connections in Settings was clicked.
    pub manage_connections_clicked: bool,
}

/// The active connection as a button. It opens the list of connections, to pick another
/// one or to reach their settings.
fn connection_switcher(ui: &mut egui::Ui, input: &TopBarInput, out: &mut TopBarOutput) {
    let name = input.active_connection.unwrap_or("No connection");
    let shown = crate::modality::truncate_with_ellipsis(name, CONNECTION_CHARS);
    let status = classify_top_bar_status(input.connection, input.server_running);
    let pill = egui::Frame::NONE
        .stroke(Stroke::new(HAIRLINE, theme::border()))
        .corner_radius(egui::CornerRadius::same(255))
        .inner_margin(egui::Margin::symmetric(10, 3))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                widgets::lamp_inline(ui, status.lit, status.color);
                ui.label(text::value(shown.as_ref()));
                let (rect, _) =
                    ui.allocate_exact_size(Vec2::splat(surface::CHEVRON_W), Sense::hover());
                surface::chevron(
                    ui,
                    rect,
                    surface::Chevron::Down,
                    Stroke::new(surface::CHEVRON_STROKE_W, theme::ink_dim()),
                );
            });
        });
    let button = ui
        .interact(
            pill.response.rect,
            ui.make_persistent_id("connection_pill"),
            Sense::click(),
        )
        .on_hover_text(format!("{}. Click to switch server.", status.caps));
    egui::Popup::menu(&button)
        .id(ui.make_persistent_id("connection_switcher"))
        .width(CONNECTION_MENU_W)
        .show(|ui| {
            egui::ScrollArea::vertical()
                .max_height(widgets::popup_max_h(ui))
                .show(ui, |ui| {
                    for connection in input.connections {
                        let is_active = input.active_connection == Some(connection.name.as_str());
                        let row = ui
                            .selectable_label(is_active, text::value(&connection.name))
                            .on_hover_text(&connection.url);
                        if row.clicked() && !is_active {
                            out.connection_chosen = Some(connection.name.clone());
                        }
                    }
                    ui.separator();
                    if ui.button(text::note("Manage connections")).clicked() {
                        out.manage_connections_clicked = true;
                    }
                });
        });
}

/// The mark of the application: a hexagon in the accent, lit on its top face.
fn brand_mark(ui: &mut egui::Ui) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(BRAND_PT), Sense::hover());
    let c = rect.center();
    let r = BRAND_PT / 2.0;
    let pt = |angle_deg: f32, radius: f32| {
        let a = angle_deg.to_radians();
        Pos2::new(c.x + radius * a.cos(), c.y + radius * a.sin())
    };
    let outline: Vec<Pos2> = (0..6).map(|i| pt(-90.0 + 60.0 * i as f32, r)).collect();
    ui.painter().add(egui::Shape::convex_polygon(
        outline.clone(),
        theme::accent(),
        Stroke::NONE,
    ));
    let top = vec![outline[0], outline[1], c, outline[5]];
    ui.painter().add(egui::Shape::convex_polygon(
        top,
        Color32::from_white_alpha(BRAND_TOP_ALPHA),
        Stroke::NONE,
    ));
}

/// The top bar: a plate of pinned height. Title, status lamp and word,
/// model name in a fixed cell; in the tail the theme toggle, Refresh, the
/// meter and the loaded count as a monospace readout.
#[allow(deprecated)]
pub fn top_bar(ui: &mut egui::Ui, input: &TopBarInput) -> TopBarOutput {
    let mut out = TopBarOutput {
        refresh_clicked: false,
        theme_toggle_clicked: false,
        connection_chosen: None,
        manage_connections_clicked: false,
    };
    let margin = egui::Margin::symmetric(widgets::CHROME_MARGIN_X, widgets::CHROME_MARGIN_Y);

    egui::Panel::top("top_bar")
        .frame(egui::Frame::NONE.fill(theme::panel()).inner_margin(margin))
        .show(ui, |ui| {
            let outer = ui.max_rect().expand2(margin.sum() / 2.0);
            ui.painter().hline(
                outer.x_range(),
                outer.bottom() - HAIRLINE / 2.0,
                Stroke::new(HAIRLINE, theme::border()),
            );
            ui.horizontal(|ui| {
                ui.set_min_height(widgets::CHROME_ROW_H);

                brand_mark(ui);
                ui.label(text::title("lokraft"));
                ui.add_space(CHROME_GAP);

                connection_switcher(ui, input, &mut out);
                ui.add_space(CHROME_GAP);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let theme_tip = if theme::is_dark() {
                        "Switch to light theme"
                    } else {
                        "Switch to dark theme"
                    };
                    if widgets::icon_button(ui, Icon::Palette, theme_tip).clicked() {
                        out.theme_toggle_clicked = true;
                    }
                    ui.add_space(CHROME_TAIL_GAP);

                    if input.refreshing {
                        ui.add_enabled_ui(false, |ui| {
                            widgets::icon_button_lit(
                                ui,
                                Icon::Refresh,
                                "Refreshing model list and hardware info",
                                true,
                            );
                        });
                    } else if widgets::icon_button(
                        ui,
                        Icon::Refresh,
                        "Refresh model list and hardware info",
                    )
                    .clicked()
                    {
                        out.refresh_clicked = true;
                    }
                    ui.add_space(CHROME_TAIL_GAP);

                    if let Some(progress) = input.progress {
                        surface::meter(ui, progress, METER_SIZE_CHROME, theme::accent());
                        ui.add_space(CHROME_TAIL_GAP);
                    }
                    widgets::pill(
                        ui,
                        &format!(
                            "{} model{} loaded",
                            input.loaded,
                            if input.loaded == 1 { "" } else { "s" }
                        ),
                    );
                });
            });
        });

    out
}

/// Render the left sidebar and return which section is active.
///
/// Returns `true` when the user clicked the collapse/expand toggle at
/// the top of the rail — the caller flips `sidebar_expanded` and
/// persists it. In the collapsed, icons-only mode the labels are hidden, so the
/// per-item hover tooltips - label plus a one-line description - are how a section
/// is identified.
#[allow(deprecated)] // see top_bar — Panel::show(&Context) until eframe top-level Ui exists.
pub fn sidebar(ui: &mut egui::Ui, current: &mut Section, expanded: bool) -> bool {
    let width = if expanded {
        SIDEBAR_WIDTH_EXPANDED
    } else {
        SIDEBAR_WIDTH
    };
    let margin = egui::Margin::symmetric(SIDEBAR_PAD_X, SIDEBAR_PAD_Y);
    let mut toggle_clicked = false;

    egui::Panel::left("nav_sidebar")
        .exact_size(width)
        .resizable(false)
        .frame(egui::Frame::NONE.fill(theme::panel()).inner_margin(margin))
        .show(ui, |ui| {
            // One hairline on the edge the content meets; the top bar owns
            // the other.
            let outer = ui.max_rect().expand2(margin.sum() / 2.0);
            ui.painter().vline(
                outer.right() - HAIRLINE / 2.0,
                outer.y_range(),
                Stroke::new(HAIRLINE, theme::border()),
            );

            if sidebar_toggle(ui, expanded).clicked() {
                toggle_clicked = true;
            }
            ui.add_space(NAV_GAP);

            for (index, item) in NAV_ITEMS.iter().enumerate() {
                if let Some(group) = item.group {
                    ui.add_space(NAV_GROUP_GAP);
                    if expanded {
                        ui.horizontal(|ui| {
                            ui.add_space(NAV_PAD_X);
                            ui.label(text::label(group));
                        });
                    } else {
                        ui.separator();
                    }
                    ui.add_space(NAV_ITEM_GAP);
                }
                let active = *current == item.section;
                let tip = format!("{}\nShortcut: Ctrl+{}", item.tip, index + 1);
                if nav_pill(
                    ui,
                    &item.icon,
                    item.label,
                    &tip,
                    active,
                    expanded,
                    &(index + 1).to_string(),
                )
                .clicked()
                {
                    *current = item.section;
                }
                ui.add_space(NAV_ITEM_GAP);
            }
        });

    toggle_clicked
}

/// The collapse toggle: a chevron pointing the way the rail will move.
fn sidebar_toggle(ui: &mut egui::Ui, expanded: bool) -> Response {
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), TOGGLE_H), Sense::click());
    let (dir, tip) = if expanded {
        (surface::Chevron::Left, "Collapse sidebar")
    } else {
        (surface::Chevron::Right, "Expand sidebar")
    };
    let hovered = response.hovered();
    if hovered {
        ui.painter().rect_filled(rect, NAV_RADIUS, theme::raised());
    }
    let ink = if hovered {
        theme::ink()
    } else {
        theme::ink_dim()
    };
    let glyph = Rect::from_center_size(rect.center(), Vec2::splat(CHEVRON_W));
    surface::chevron(ui, glyph, dir, Stroke::new(CHEVRON_STROKE_W, ink));
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, true, tip));
    response.on_hover_text(tip)
}

/// One navigation item. The active one carries the accent three ways: a
/// stripe, a wash and its tint; the others are dim ink.
fn nav_pill(
    ui: &mut egui::Ui,
    icon: &NavIcon,
    label: &str,
    tip: &str,
    active: bool,
    expanded: bool,
    shortcut: &str,
) -> Response {
    let accent = theme::accent();
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), NAV_ITEM_H), Sense::click());
    let painter = ui.painter();
    if active {
        painter.rect_filled(rect, NAV_RADIUS, theme::accent_soft());
    } else if response.hovered() {
        painter.rect_filled(rect, NAV_RADIUS, theme::raised());
    }
    let ink = if active { accent } else { theme::ink_dim() };

    let icon_cx = if expanded {
        rect.left() + NAV_PAD_X + NAV_ICON_PT / 2.0
    } else {
        rect.center().x
    };
    let icon_rect = Rect::from_center_size(
        Pos2::new(icon_cx, rect.center().y),
        Vec2::splat(NAV_ICON_PT),
    );
    match icon {
        NavIcon::Svg(ic) => ic.image(NAV_ICON_PT, ink).paint_at(ui, icon_rect),
        NavIcon::Text(s) => {
            ui.painter().text(
                icon_rect.center(),
                Align2::CENTER_CENTER,
                *s,
                FontId::monospace(NAV_ICON_PT),
                ink,
            );
        }
    }
    if expanded {
        ui.painter().text(
            Pos2::new(icon_rect.right() + NAV_GAP, rect.center().y),
            Align2::LEFT_CENTER,
            label,
            FontId::new(
                text::VALUE_PT,
                egui::FontFamily::Name(theme::FONT_MEDIUM.into()),
            ),
            ink,
        );
        ui.painter().text(
            Pos2::new(rect.right() - NAV_PAD_X, rect.center().y),
            Align2::RIGHT_CENTER,
            shortcut,
            FontId::monospace(text::READOUT_PT),
            theme::ink_dim(),
        );
    }

    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, true, label));
    response.on_hover_ui(|ui| {
        ui.label(text::value(label));
        ui.label(text::note(tip));
    })
}

/// Classify the top-bar connection status into a (dot colour, short
/// label) pair. Used by `top_bar` and unit-tested below.
///
/// Three states the user can be in, each must match consistently
/// across colour + label:
///   - Connected  → green   "Connected" (or "Online" if server_running)
///   - Connecting → amber   "Connecting…"
///   - other      → red     "Offline"
///
/// `server_running` forces "Online" (label only) — the embedded
/// server is up so the GUI knows it's reachable even if no /api/tags
/// has come back yet.
///
/// Colour + base label both derive from the `ConnectionState` enum
/// (single source of truth) — no more `.contains()` string sniffing.
/// The status lamp: its colour, its word, whether it is lit.
pub(crate) struct TopBarStatus {
    pub color: Color32,
    pub caps: &'static str,
    /// Off when there is no connection at all.
    pub lit: bool,
}

pub(crate) fn classify_top_bar_status(
    state: crate::state::ConnectionState,
    server_running: bool,
) -> TopBarStatus {
    use crate::state::ConnectionState;
    let caps = if server_running {
        "ONLINE"
    } else {
        match state {
            ConnectionState::Connected => "CONNECTED",
            ConnectionState::Connecting => "CONNECTING",
            _ => "OFFLINE",
        }
    };
    TopBarStatus {
        color: state.color(),
        caps,
        lit: state != ConnectionState::Disconnected,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Section;

    // ── classify_top_bar_status ───────────────────────────────────

    #[test]
    fn top_bar_status_connecting_uses_amber_and_matching_label() {
        // What this guards: a label deciding Online/Connected/Offline while the
        // colour decides Connected/Connecting/red, from two independent
        // `.contains()` ladders that can disagree. Both derive
        // from the ConnectionState enum, so they can't desync.
        use crate::state::ConnectionState;
        let status = classify_top_bar_status(ConnectionState::Connecting, false);
        assert_eq!(
            status.color,
            theme::warning(),
            "Connecting state must use the theme amber, not red"
        );
        assert_eq!(
            status.caps, "CONNECTING",
            "label must say Connecting, not Offline, during handshake"
        );
        assert!(status.lit);
    }

    #[test]
    fn top_bar_status_connected_is_green_and_says_connected() {
        use crate::state::ConnectionState;
        let status = classify_top_bar_status(ConnectionState::Connected, false);
        assert_eq!(status.color, theme::success());
        assert_eq!(status.caps, "CONNECTED");
        assert!(status.lit);
    }

    #[test]
    fn top_bar_status_disconnected_is_red_and_says_offline() {
        use crate::state::ConnectionState;
        let status = classify_top_bar_status(ConnectionState::Disconnected, false);
        assert_eq!(status.color, theme::error());
        assert_eq!(status.caps, "OFFLINE");
        assert!(!status.lit, "no connection: the lamp is off");
    }

    #[test]
    fn top_bar_status_server_running_label_wins_over_status_check() {
        // server_running=true: label says "Online" regardless of
        // connection_status text (the embedded server is up). The
        // dot colour is still driven by connection_status — so a
        // freshly-spawned embedded server in mid-handshake renders
        // a green-or-amber dot with "Online" label (informative
        // overlap, not contradictory).
        use crate::state::ConnectionState;
        let status = classify_top_bar_status(ConnectionState::Disconnected, true);
        assert_eq!(status.caps, "ONLINE");
    }

    /// The sidebar draws its collapse toggle once. The block was pasted twice, which put two
    /// identical chevron buttons at the top of the rail; it was invisible in review and
    /// obvious the moment the window was rendered.
    #[test]
    fn the_sidebar_has_one_collapse_toggle() {
        fn chevrons(expanded: bool) -> usize {
            use egui_kittest::kittest::NodeT;
            let mut section = Section::Chat;
            let mut harness = crate::theme::test_harness(None, move |ui| {
                sidebar(ui, &mut section, expanded);
            });
            harness.run();
            harness
                .root()
                .children_recursive()
                .filter(|n| {
                    let label = n.accesskit_node().label().unwrap_or_default();
                    label == "Collapse sidebar" || label == "Expand sidebar"
                })
                .count()
        }
        assert_eq!(chevrons(true), 1, "expanded rail");
        assert_eq!(chevrons(false), 1, "collapsed rail");
    }
}

#[cfg(test)]
mod nav_tests {
    use super::{section_for_shortcut, NAV_ITEMS};

    #[test]
    fn each_shortcut_opens_the_section_at_that_place_in_the_sidebar() {
        for (i, item) in NAV_ITEMS.iter().enumerate() {
            assert_eq!(section_for_shortcut(i + 1), Some(item.section));
        }
        assert_eq!(section_for_shortcut(0), None);
        assert_eq!(section_for_shortcut(NAV_ITEMS.len() + 1), None);
    }

    #[test]
    fn the_terminal_tip_names_every_command() {
        let tip = NAV_ITEMS
            .iter()
            .find(|i| i.label == "Terminal")
            .expect("a Terminal entry")
            .tip;
        for (entry, _) in crate::cli_tab::COMMAND_HELP {
            let word = entry.split_whitespace().next().unwrap_or(entry);
            assert!(tip.contains(word), "the Terminal tip omits `{word}`");
        }
    }
}
