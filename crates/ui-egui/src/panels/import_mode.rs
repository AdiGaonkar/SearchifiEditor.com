//! Import mode (header "Import", also Home).
//!
//! Matches the design mockup:
//!   - large two-line light title ("Media" / "WorkSpace") + subtitle
//!   - left: "Get Started" action rows
//!   - right: "Recent Projects" panel, same height as the action list
//!   - "Community & Resources" label with a divider and link chips underneath

use egui::{Align2, Color32, Painter, Rect, Sense, Stroke, pos2, vec2};
use serde_json::json;

use crate::FilmcraftApp;
use crate::theme::Tokens;

const MAX_CONTENT_W: f32 = 1400.0;
const SIDE_PAD: f32 = 40.0;
const COL_GAP: f32 = 16.0;
const ACTION_H: f32 = 66.0;
const ACTION_GAP: f32 = 14.0;
const RECENT_ROWS: usize = 6;
const RECENT_ROW_H: f32 = 48.0;

// ---------------------------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------------------------

fn with_alpha(c: Color32, a: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), a)
}

/// Linear blend of two opaque colors (k = 0 -> a, k = 1 -> b).
fn mix(a: Color32, b: Color32, k: f32) -> Color32 {
    let l = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * k).round() as u8;
    Color32::from_rgb(l(a.r(), b.r()), l(a.g(), b.g()), l(a.b(), b.b()))
}

/// Rounded rect with a 1px border (border drawn first, opaque fill on top).
fn bordered(p: &Painter, r: Rect, radius: f32, fill: Color32, border: Color32) {
    p.rect_filled(r, radius, border);
    p.rect_filled(r.shrink(1.0), (radius - 1.0).max(0.0), fill);
}

fn section_label(p: &Painter, t: &Tokens, pos: egui::Pos2, text: &str) {
    p.text(pos, Align2::LEFT_CENTER, text, Tokens::semibold(13.0), t.text);
}

// ---------------------------------------------------------------------------------------------
// Page
// ---------------------------------------------------------------------------------------------

pub fn show(app: &mut FilmcraftApp, ui: &mut egui::Ui, rect: Rect) {
    let t = app.tokens;
    let ctx = ui.ctx().clone();
    let p = ui.painter().with_clip_rect(rect);
    let line = with_alpha(t.text, 30);

    // Page background
    p.rect_filled(rect, t.radius, t.panel_bg);

    let w = (rect.width() - SIDE_PAD * 2.0).clamp(0.0, MAX_CONTENT_W);
    let left = rect.center().x - w / 2.0;
    let top = rect.min.y + 96.0;

    // ---- Header ---------------------------------------------------------------------------
    p.text(
        pos2(left, top),
        Align2::LEFT_CENTER,
        tl!("Media"),
        Tokens::ui(52.0),
        t.text,
    );
    p.text(
        pos2(left, top + 60.0),
        Align2::LEFT_CENTER,
        tl!("WorkSpace"),
        Tokens::ui(52.0),
        t.text,
    );
    p.text(
        pos2(left, top + 108.0),
        Align2::LEFT_CENTER,
        tl!("Bring your footage, audio and creative assets into your project."),
        Tokens::ui(14.0),
        t.text_dim,
    );

    let label_y = top + 172.0;
    let list_top = label_y + 28.0;
    let wide = w >= 760.0;
    let left_w = if wide { ((w - COL_GAP) * 0.57).floor() } else { w };

    // ---- Left: Get Started ----------------------------------------------------------------
    section_label(&p, &t, pos2(left, label_y), &tl!("Get Started"));

    let actions = [
        (
            tl!("Browse files…"),
            "file.import",
            tl!("Import media from your computer"),
            crate::icons::Icon::Folder,
            Color32::from_rgb(133, 120, 255),
        ),
        (
            tl!("Demo footage"),
            "file.importDemoFootage",
            tl!("Explore sample clips and audio"),
            crate::icons::Icon::Film,
            Color32::from_rgb(70, 175, 255),
        ),
        (
            tl!("Demo project"),
            "file.openDemoProject",
            tl!("Open a ready-made editing project"),
            crate::icons::Icon::Sequence,
            Color32::from_rgb(74, 207, 166),
        ),
        (
            tl!("Bars and Tone"),
            "file.newBarsAndTone",
            tl!("Generate SMPTE bars and reference tone"),
            crate::icons::Icon::Grid,
            Color32::from_rgb(255, 174, 92),
        ),
    ];

    for (i, (title, cmd, subtitle, icon, accent)) in actions.iter().enumerate() {
        let r = Rect::from_min_size(
            pos2(left, list_top + i as f32 * (ACTION_H + ACTION_GAP)),
            vec2(left_w, ACTION_H),
        );
        if action_row(app, ui, &p, &t, r, cmd, title, subtitle, *icon, *accent) {
            let _ = crate::menus::invoke(app, &ctx, cmd, json!({}));
            app.ui.mode = crate::state::Mode::Edit;
        }
    }
    let actions_bottom =
        list_top + actions.len() as f32 * ACTION_H + (actions.len() - 1) as f32 * ACTION_GAP;

    // ---- Right: Recent Projects -----------------------------------------------------------
    let paths: Vec<String> = app
        .session
        .prefs
        .general
        .recent_projects
        .iter()
        .take(RECENT_ROWS)
        .cloned()
        .collect();

    let (rx, rw, rlabel_y, rtop) = if wide {
        (left + left_w + COL_GAP, w - left_w - COL_GAP, label_y, list_top)
    } else {
        let ly = actions_bottom + 36.0;
        (left, w, ly, ly + 28.0)
    };
    let panel_h = if wide {
        actions_bottom - list_top
    } else if paths.is_empty() {
        160.0
    } else {
        paths.len() as f32 * RECENT_ROW_H + 16.0
    };

    section_label(&p, &t, pos2(rx, rlabel_y), &tl!("Recent Projects"));
    let panel = Rect::from_min_size(pos2(rx, rtop), vec2(rw, panel_h));
    let to_open = recent_panel(app, ui, &p, &t, panel, &paths);

    let body_bottom = panel.max.y.max(actions_bottom);

    // ---- Community & Resources ------------------------------------------------------------
    let foot_y = body_bottom + 46.0;
    p.text(
        pos2(left, foot_y),
        Align2::LEFT_CENTER,
        tl!("Community & Resources"),
        Tokens::semibold(13.0),
        t.text,
    );
    p.line_segment(
        [pos2(left, foot_y + 16.0), pos2(left + w, foot_y + 16.0)],
        Stroke::new(1.0, line),
    );

    let mut x = left;
    for (i, (id, label, url)) in crate::links::ALL.iter().take(4).enumerate() {
        let primary = i == 0;
        let label = if primary {
            tl!("Join us on Discord")
        } else {
            crate::i18n::t(label)
        };

        let galley = p.layout_no_wrap(label.to_string(), Tokens::ui(12.0), t.text);
        let r = Rect::from_min_size(
            pos2(x, foot_y + 32.0),
            vec2(galley.size().x + 40.0, 34.0),
        );

        let resp = ui
            .interact(r, egui::Id::new(("imp-link", *id)), Sense::click())
            .on_hover_cursor(egui::CursorIcon::PointingHand)
            .on_hover_text(*url);

        app.auto.add(
            &format!("import.link.{}", id.trim_start_matches("help.")),
            r,
            label,
        );

        let (fill, border, fg) = if primary {
            let f = if resp.hovered() { t.accent_hover } else { t.accent };
            (f, f, Color32::WHITE)
        } else {
            (
                if resp.hovered() { t.hover } else { t.tl_header_bg },
                line,
                t.text,
            )
        };
        bordered(&p, r, 8.0, fill, border);

        let icon = [
            crate::icons::Icon::Chat,
            crate::icons::Icon::Globe,
            crate::icons::Icon::Globe,
            crate::icons::Icon::Code,
        ][i];
        crate::icons::paint(
            &p,
            Rect::from_center_size(pos2(r.min.x + 18.0, r.center().y), vec2(14.0, 14.0)),
            icon,
            fg,
        );
        p.galley(
            pos2(r.min.x + 32.0, r.center().y - galley.size().y / 2.0),
            galley,
            fg,
        );

        if resp.clicked() {
            crate::links::open(app, &ctx, url);
        }
        x = r.max.x + 10.0;
    }

    // ---- Deferred: open a recent project --------------------------------------------------
    if let Some(path) = to_open {
        if crate::menus::invoke(app, &ctx, "file.open", json!({ "path": path })).is_ok() {
            app.ui.mode = crate::state::Mode::Edit;
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Action row
// ---------------------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn action_row(
    app: &mut FilmcraftApp,
    ui: &mut egui::Ui,
    p: &Painter,
    t: &Tokens,
    r: Rect,
    cmd: &str,
    title: &str,
    subtitle: &str,
    icon: crate::icons::Icon,
    accent: Color32,
) -> bool {
    let resp = ui
        .interact(r, egui::Id::new(("imp", cmd)), Sense::click())
        .on_hover_cursor(egui::CursorIcon::PointingHand);
    app.auto.add(&format!("import.{cmd}"), r, title);

    let fill = if resp.hovered() { t.hover } else { t.tl_header_bg };
    p.rect_filled(r, 10.0, fill);

    // Icon tile
    let tile = Rect::from_min_size(
        pos2(r.min.x + 16.0, r.center().y - 20.0),
        vec2(40.0, 40.0),
    );
    p.rect_filled(tile, 9.0, mix(fill, accent, 0.20));
    crate::icons::paint(p, tile.shrink(9.0), icon, accent);

    // Text
    let tx = tile.max.x + 16.0;
    p.text(
        pos2(tx, r.center().y - 9.0),
        Align2::LEFT_CENTER,
        title,
        Tokens::semibold(14.0),
        t.text,
    );
    p.text(
        pos2(tx, r.center().y + 11.0),
        Align2::LEFT_CENTER,
        subtitle,
        Tokens::ui(11.5),
        t.text_dim,
    );

    resp.clicked()
}

// ---------------------------------------------------------------------------------------------
// Recent projects panel
// ---------------------------------------------------------------------------------------------

/// Draws the panel; returns the path the user clicked, if any.
fn recent_panel(
    app: &mut FilmcraftApp,
    ui: &mut egui::Ui,
    p: &Painter,
    t: &Tokens,
    panel: Rect,
    paths: &[String],
) -> Option<String> {
    p.rect_filled(panel, 10.0, t.tl_header_bg);

    if paths.is_empty() {
        let c = panel.center();
        crate::icons::paint(
            p,
            Rect::from_center_size(pos2(c.x, c.y - 26.0), vec2(26.0, 26.0)),
            crate::icons::Icon::Sequence,
            t.text_dim,
        );
        p.text(
            pos2(c.x, c.y + 6.0),
            Align2::CENTER_CENTER,
            tl!("No recent projects"),
            Tokens::semibold(13.0),
            t.text,
        );
        p.text(
            pos2(c.x, c.y + 26.0),
            Align2::CENTER_CENTER,
            tl!("Projects you open will appear here."),
            Tokens::ui(11.5),
            t.text_dim,
        );
        return None;
    }

    let mut open = None;
    for (i, path) in paths.iter().enumerate() {
        let row = Rect::from_min_size(
            pos2(panel.min.x + 8.0, panel.min.y + 8.0 + i as f32 * RECENT_ROW_H),
            vec2((panel.width() - 16.0).max(0.0), RECENT_ROW_H),
        );
        if row.max.y > panel.max.y {
            break;
        }

        let name = crate::menus::recent_name(path);
        let resp = ui
            .interact(row, egui::Id::new(("imp-recent", i)), Sense::click())
            .on_hover_cursor(egui::CursorIcon::PointingHand)
            .on_hover_text(path.as_str());
        app.auto.add(&format!("import.recent.{i}"), row, &name);

        let row_fill = if resp.hovered() {
            p.rect_filled(row, 8.0, t.hover);
            t.hover
        } else {
            t.tl_header_bg
        };

        let rp = p.with_clip_rect(row);

        let tile = Rect::from_center_size(pos2(row.min.x + 24.0, row.center().y), vec2(32.0, 32.0));
        rp.rect_filled(tile, 8.0, mix(row_fill, t.accent, 0.16));
        crate::icons::paint(&rp, tile.shrink(8.0), crate::icons::Icon::Sequence, t.accent);

        let tx = tile.max.x + 12.0;
        rp.text(
            pos2(tx, row.center().y - 8.0),
            Align2::LEFT_CENTER,
            &name,
            Tokens::ui(13.0),
            t.text,
        );
        if let Some(folder) = std::path::Path::new(path).parent() {
            let folder = folder.to_string_lossy();
            if !folder.is_empty() {
                rp.text(
                    pos2(tx, row.center().y + 9.0),
                    Align2::LEFT_CENTER,
                    folder,
                    Tokens::ui(11.0),
                    t.text_dim,
                );
            }
        }

        if resp.clicked() {
            open = Some(path.clone());
        }
    }
    open
}