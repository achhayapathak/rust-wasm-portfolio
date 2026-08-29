// ─────────────────────────────────────────────────────────────
// layout.rs — Ticker, Masthead, Footer, IndexNav, MobileNav,
// DarkModeToggle, CarbonTitle.
// ─────────────────────────────────────────────────────────────

use crate::data::*;
use crate::components::decorations::*;

/// Carbon-copy title with offset shadow text.
pub fn render_carbon_title(text: &str) -> String {
    format!(
        r#"<span class="carbon-title"><span class="carbon-wrap" data-text="{}"><span class="carbon-top">{}</span></span></span>"#,
        text, text
    )
}

/// Full-width scrolling marquee ticker at the top of the page.
pub fn render_ticker() -> String {
    let ticker_count = TICKER.len() + 1;
    let duration = ticker_count * 10;

    let mut cells = String::new();
    // First set
    for t in TICKER.iter() {
        cells.push_str(&format!(
            r#"<span class="ticker-cell"><span class="ticker-sym">∆</span>{} · {} · {} · {}</span>"#,
            t.date, t.org, t.detail, t.status
        ));
    }
    cells.push_str(r#"<span class="ticker-cell"><span class="ticker-sym">∆</span>balance carried forward ·············· portfolio</span>"#);
    // Duplicate for seamless loop
    for t in TICKER.iter() {
        cells.push_str(&format!(
            r#"<span class="ticker-cell"><span class="ticker-sym">∆</span>{} · {} · {} · {}</span>"#,
            t.date, t.org, t.detail, t.status
        ));
    }
    cells.push_str(r#"<span class="ticker-cell"><span class="ticker-sym">∆</span>balance carried forward ·············· portfolio</span>"#);

    format!(
        r#"<div class="stock-cream ticker-wrap"><div aria-hidden="true" class="ticker-track"><div class="ticker-tape" style="animation-duration:{}s">{}</div></div></div>"#,
        duration, cells
    )
}

/// Desktop navigation — horizontal row of tabs with color swatches.
pub fn render_index_nav(current_path: &str) -> String {
    let mut items = String::new();
    for item in NAV_ITEMS.iter() {
        let is_active = current_path == item.href;
        let active_class = if is_active { " on" } else { "" };
        items.push_str(&format!(
            r#"<li><a href="{}" class="part stock-{}{}"><span class="swatch"></span><span class="part-label">{}</span></a></li>"#,
            item.href, item.stock.as_str(), active_class, item.label
        ));
    }

    format!(
        r#"<nav class="index-nav" aria-label="Section index"><ul class="index-list">{}</ul></nav>"#,
        items
    )
}

/// Mobile navigation trigger button (the sheet is initially hidden, toggled by WASM).
pub fn render_mobile_nav(current_path: &str) -> String {
    let mut nav_rows = String::new();
    for item in NAV_ITEMS.iter() {
        let is_active = current_path == item.href;
        let active_class = if is_active { " on" } else { "" };
        nav_rows.push_str(&format!(
            r#"<li><a href="{}" class="mobile-row stock-{}{}"><span class="mobile-swatch"></span><span class="mobile-name">{}</span><span class="mobile-arrow">→</span></a></li>"#,
            item.href, item.stock.as_str(), active_class, item.label
        ));
    }

    format!(
        r#"<button id="mobile-trigger" class="mobile-trigger" aria-label="Open navigation" aria-expanded="false">{}<span class="lbl" style="color:var(--stock)">index</span></button><div id="mobile-sheet" class="mobile-sheet" role="dialog" aria-label="Navigation" style="display:none"><div class="mobile-sheet-inner">{}{}<div class="mobile-bar"><span class="lbl lbl-ink">FORM NV-00 &nbsp;·&nbsp; INDEX OF RECORDS</span><button id="mobile-close" class="mobile-close" aria-label="Close navigation">{}</button></div><nav class="mobile-list"><ul>{}</ul></nav></div></div>"#,
        render_glyph("menu", 17),
        render_grid_background(),
        render_marks_overlay(DEFAULT_MARKS),
        render_glyph("close", 18),
        nav_rows
    )
}

/// Dark mode toggle button.
pub fn render_dark_mode_toggle() -> String {
    format!(
        r#"<button id="dark-mode-toggle" class="toggle-btn" aria-label="Switch to dark mode"><span id="dark-mode-icon">{}</span><span id="dark-mode-label" class="lbl" style="color:var(--stock)">light</span></button>"#,
        render_glyph("moon", 17)
    )
}

/// Masthead header — the main branded panel at the top.
pub fn render_masthead(current_path: &str) -> String {
    format!(
        r#"<header class="masthead app-panel stock-cream">{}{}<div class="brand"><a href="/" class="brand-link"><span class="brand-name">{}</span></a><p class="lbl lbl-ink brand-sub">{} &nbsp;·&nbsp; {}</p></div><div class="controls">{}{}</div>{}</header>"#,
        render_grid_background(),
        render_marks_overlay(DEFAULT_MARKS),
        render_carbon_title(SITE.name),
        SITE.title,
        SITE.location,
        render_dark_mode_toggle(),
        render_mobile_nav(current_path),
        render_index_nav(current_path)
    )
}

/// Footer with grid overlay, 3-col info grid, MICR barcode.
pub fn render_footer() -> String {
    let footer_marks = &[
        Mark { mark_type: "target", left: "15%", top: "30%", width: "40px", rotate: "10deg" },
        Mark { mark_type: "squares", left: "35%", top: "65%", width: "30px", rotate: "-20deg" },
        Mark { mark_type: "crosshair", left: "58%", top: "71%", width: "28px", rotate: "-30deg" },
        Mark { mark_type: "target", left: "67%", top: "23%", width: "44px", rotate: "15deg" },
        Mark { mark_type: "arrow", left: "79%", top: "68%", width: "50px", rotate: "30deg" },
        Mark { mark_type: "asterisk", left: "92%", top: "40%", width: "32px", rotate: "0deg" },
    ];

    // Contact links
    let mut contact_links = String::new();
    // Email
    contact_links.push_str(&format!(
        r#"<li><a class="num" href="https://mail.google.com/mail/?view=cm&amp;fs=1&amp;to={}" target="_blank" rel="noopener noreferrer">{}<span><!--email_off-->{}<!--/email_off--></span></a></li>"#,
        SITE.email,
        render_glyph("mail", 16),
        SITE.email
    ));
    // Social links shown in footer
    for s in SOCIAL.iter().filter(|s| s.show_in_footer) {
        contact_links.push_str(&format!(
            r#"<li><a class="num" href="{}" target="_blank" rel="noopener noreferrer">{}<span>{}</span></a></li>"#,
            s.url,
            render_glyph(s.icon, 16),
            s.label
        ));
    }

    format!(
        r#"<footer class="foot app-panel stock-cream">{}{}<div class="foot-grid"><div><p class="lbl">name</p><p class="num foot-strong">{}</p><p class="lbl">based in &nbsp;·&nbsp; {}</p></div><div><p class="lbl">contact</p><ul class="links">{}</ul></div><div class="foot-note"><p class="lbl">colophon</p><p class="prose-form">Forged in Rust &amp; WebAssembly with ruthless precision. Zero bloat, sub-millisecond execution, and an ultra-lean footprint. Every mark — stamps, ledger grids, perforated edges — is rendered in pure code and vector math.</p></div></div><div class="foot-micr">{}</div></footer>"#,
        render_grid_background(),
        render_marks_overlay(footer_marks),
        SITE.name.to_lowercase(),
        SITE.location.to_lowercase(),
        contact_links,
        render_micr_barcode(SITE.name)
    )
}
