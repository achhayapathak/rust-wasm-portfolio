// ─────────────────────────────────────────────────────────────
// decorations.rs — SVG icons, grid background, marks overlay,
// ledger stamp, MICR barcode.
// ─────────────────────────────────────────────────────────────

/// Glyph icon system — stroke-only SVGs matching the reference exactly.
pub fn render_glyph(name: &str, size: u32) -> String {
    let paths = match name {
        "mail" => r#"<path d="M3 5 H21 V19 H3 Z M3 5 L12 13 L21 5"/>"#,
        "card" => r#"<path d="M3 5 H21 V19 H3 Z"/><path d="M6 9 H11 V13 H6 Z"/><path d="M6 16 H11"/><path d="M14 9.5 H18 M14 13 H18 M14 16 H18"/>"#,
        "more" => r#"<path d="M12 3.6 V20.4 M3.6 12 H20.4" stroke-width="3"/>"#,
        "arrow" => r#"<path d="M3 12 H21"/><path d="M14 5 L21 12 L14 19"/>"#,
        "code" => r#"<path d="M8 6 L2 12 L8 18"/><path d="M16 6 L22 12 L16 18"/>"#,
        "link" => r#"<path d="M10 14 L14 10"/><path d="M15 9 L18 6 A3 3 0 0 0 14 2 L11 5"/><path d="M9 15 L6 18 A3 3 0 0 0 10 22 L13 19"/>"#,
        "sun" => r#"<circle cx="12" cy="12" r="4"/><path d="M12 2 V5 M12 19 V22 M2 12 H5 M19 12 H22 M4.9 4.9 L7 7 M17 17 L19.1 19.1 M19.1 4.9 L17 7 M7 17 L4.9 19.1"/>"#,
        "moon" => r#"<path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>"#,
        "menu" => r#"<path d="M3 6 H21 M3 12 H21 M3 18 H21"/>"#,
        "close" => r#"<path d="M6 6 L18 18 M18 6 L6 18"/>"#,
        "chevron" => r#"<path d="M9 6 L15 12 L9 18"/>"#,
        _ => "",
    };

    format!(
        r#"<span class="glyph-icon" style="width:{}px;display:inline-block;flex-shrink:0"><svg viewBox="0 0 24 24" aria-hidden="true" focusable="false" style="display:block;height:auto;width:100%;fill:none;stroke:currentColor;stroke-width:2.1;stroke-linecap:square;stroke-linejoin:miter">{}</svg></span>"#,
        size, paths
    )
}

/// 4-layer repeating CSS grid-lines overlay.
pub fn render_grid_background() -> String {
    String::from(
        r#"<div aria-hidden="true" class="grid-bg" style="color:var(--on-stock);opacity:0.08;background-image:repeating-linear-gradient(to right, currentColor 0 1px, transparent 1px calc(14px * 5)),repeating-linear-gradient(to bottom, currentColor 0 1px, transparent 1px calc(14px * 5)),repeating-linear-gradient(to right, color-mix(in srgb, currentColor 50%, transparent) 0 1px, transparent 1px 14px),repeating-linear-gradient(to bottom, color-mix(in srgb, currentColor 50%, transparent) 0 1px, transparent 1px 14px)"></div>"#,
    )
}

/// Mark types for the overlay.
pub struct Mark {
    pub mark_type: &'static str,
    pub left: &'static str,
    pub top: &'static str,
    pub width: &'static str,
    pub rotate: &'static str,
}

pub static DEFAULT_MARKS: &[Mark] = &[
    Mark { mark_type: "squares", left: "12%", top: "18%", width: "38px", rotate: "15deg" },
    Mark { mark_type: "crosshair", left: "28%", top: "55%", width: "32px", rotate: "-30deg" },
    Mark { mark_type: "target", left: "45%", top: "25%", width: "44px", rotate: "15deg" },
    Mark { mark_type: "arrow", left: "62%", top: "70%", width: "50px", rotate: "30deg" },
    Mark { mark_type: "asterisk", left: "80%", top: "35%", width: "28px", rotate: "0deg" },
    Mark { mark_type: "squares", left: "90%", top: "60%", width: "36px", rotate: "-15deg" },
];

fn render_mark_svg(mark_type: &str) -> &'static str {
    match mark_type {
        "crosshair" => r#"<svg viewBox="0 0 24 24" focusable="false" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="square" stroke-linejoin="miter"><path d="M12 2 V22 M2 12 H22 M5 5 L19 19 M19 5 L5 19" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="square" stroke-linejoin="miter"/></svg>"#,
        "target" => r#"<svg viewBox="0 0 24 24" focusable="false" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="square" stroke-linejoin="miter"><circle cx="12" cy="12" r="7" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="square" stroke-linejoin="miter"/><path d="M12 1 V7 M12 17 V23 M1 12 H7 M17 12 H23" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="square" stroke-linejoin="miter"/></svg>"#,
        "squares" => r#"<svg viewBox="0 0 24 24" focusable="false" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="square" stroke-linejoin="miter"><path d="M2 2 H22 V22 H2 Z" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="square" stroke-linejoin="miter"/><path d="M7 7 H17 V17 H7 Z" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="square" stroke-linejoin="miter"/><path d="M11 11 H13 V13 H11 Z" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="square" stroke-linejoin="miter"/></svg>"#,
        "arrow" => r#"<svg viewBox="0 0 24 24" focusable="false" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="square" stroke-linejoin="miter"><path d="M3 12 H21" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="square" stroke-linejoin="miter"/><path d="M14 5 L21 12 L14 19" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="square" stroke-linejoin="miter"/></svg>"#,
        "asterisk" => r#"<svg viewBox="0 0 24 24" focusable="false" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="square" stroke-linejoin="miter"><path d="M12 2 V22 M2 12 H22 M5 5 L19 19 M19 5 L5 19" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="square" stroke-linejoin="miter"/></svg>"#,
        _ => "",
    }
}

/// SVG marks overlay.
pub fn render_marks_overlay(marks: &[Mark]) -> String {
    let mut html = String::from(r#"<div aria-hidden="true" class="marks-overlay" style="color:var(--on-stock);opacity:0.1">"#);
    for m in marks {
        html.push_str(&format!(
            r#"<div class="mark-item" style="left:{};top:{};width:{};translate:-50% -50%;transform-origin:50% 50%;rotate:{}">{}</div>"#,
            m.left, m.top, m.width, m.rotate,
            render_mark_svg(m.mark_type)
        ));
    }
    html.push_str("</div>");
    html
}

/// Rubber-stamp effect with noise mask.
pub fn render_ledger_stamp(status: &str, rotation: i32) -> String {
    format!(
        r#"<span role="status" class="app-stamp" style="--rot:{}deg" aria-label="Status: {}"><span class="app-stamp-box">{}</span></span>"#,
        rotation, status, status
    )
}

/// MICR-style barcode rendered as SVG rects.
struct Bar {
    w: u32,
    h: &'static str,
}

fn encode_char(ch: u8) -> Vec<Bar> {
    let code = ch as u32;
    let count = 2 + (code % 3);
    let mut bars = Vec::new();
    for i in 0..count {
        bars.push(Bar {
            w: 2 + ((code + i * 7) % 4),
            h: if (code + i) % 3 == 0 { "half" } else { "full" },
        });
    }
    bars
}

pub fn render_micr_barcode(name: &str) -> String {
    let mut rects = Vec::new();
    let mut x: u32 = 2;

    for ch in name.to_lowercase().bytes() {
        if ch == b' ' {
            x += 8;
            continue;
        }
        let bars = encode_char(ch);
        for bar in &bars {
            let (y, h) = if bar.h == "full" { (2, 18) } else { (8, 12) };
            rects.push(format!(r#"<rect x="{}" width="{}" y="{}" height="{}"/>"#, x, bar.w, y, h));
            x += bar.w + 3;
        }
        x += 2;
    }

    format!(
        r#"<div class="micr-barcode" aria-hidden="true"><svg preserveAspectRatio="none" role="img" viewBox="0 0 640 22" aria-label="Routing band {}" style="display:block;height:100%;width:100%;fill:currentColor">{}</svg></div>"#,
        name,
        rects.join("")
    )
}
