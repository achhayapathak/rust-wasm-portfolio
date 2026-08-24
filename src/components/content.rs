// ─────────────────────────────────────────────────────────────
// content.rs — PageSheet, SectionBlock, LedgerCard, TagList,
// ExpandableText.
// ─────────────────────────────────────────────────────────────

use crate::data::*;
use crate::components::decorations::*;

/// Render a list of tag pills.
pub fn render_tag_list(tags: &[&str]) -> String {
    if tags.is_empty() {
        return String::new();
    }
    let mut items = String::new();
    for tag in tags {
        items.push_str(&format!(r#"<li><span class="app-tag">{}</span></li>"#, tag));
    }
    format!(r#"<ul class="tag-list">{}</ul>"#, items)
}

/// Expandable text wrapper — clamped with a toggle button.
/// The actual expand/collapse is handled by WASM client code.
pub fn render_expandable_text(inner_html: &str, lines: u32) -> String {
    format!(
        r#"<div class="expandable-wrapper"><div class="expandable-content clamped" data-clamp-lines="{}">{}</div><button class="more-btn expand-toggle" style="display:none">+ read full entry</button></div>"#,
        lines, inner_html
    )
}

/// LedgerCard — the core brutalist card component.
pub fn render_ledger_card(
    stock: StockColor,
    head_title: &str,
    head_subtitle: Option<&str>,
    date: Option<&str>,
    status: Option<&str>,
    stamp_rotation: Option<i32>,
    number: Option<&str>,
    tags: &[&str],
    action_html: &str,
    body_html: &str,
) -> String {
    let subtitle_html = head_subtitle
        .map(|s| format!(r#"<p class="card-subtitle">{}</p>"#, s))
        .unwrap_or_default();

    let num_html = number
        .map(|n| format!(r#"<span class="card-num lbl">{}</span>"#, n))
        .unwrap_or_default();

    let date_bar = date
        .map(|d| format!(r#"<div class="card-date-bar"><span class="lbl">{}</span></div>"#, d))
        .unwrap_or_default();

    let tags_html = render_tag_list(tags);

    let stamp_html = if let Some(st) = status {
        let rot = stamp_rotation.unwrap_or(if st == "In-Progress" { -13 } else { -8 });
        format!(
            r#"<div class="stamp-pos">{}</div>"#,
            render_ledger_stamp(st, rot)
        )
    } else {
        String::new()
    };

    let expandable_body = render_expandable_text(body_html, 4);

    format!(
        r#"<div class="slot stock-{}"><div class="card-wrapper"><div class="plate" aria-hidden="true"></div><article class="app-card app-notch"><header class="app-card-head"><div class="card-head-inner"><div class="card-head-text"><h3 class="card-title">{}</h3>{}</div>{}</div></header>{}<div class="card-body">{}{}{}</div></article></div>{}</div>"#,
        stock.as_str(),
        head_title,
        subtitle_html,
        num_html,
        date_bar,
        expandable_body,
        tags_html,
        action_html,
        stamp_html
    )
}

/// SectionBlock — a section within a page with heading and grid.
pub fn render_section_block(
    form_label: &str,
    subtitle: &str,
    title: &str,
    id: &str,
    is_first: bool,
    action_label: Option<&str>,
    action_href: Option<&str>,
    children_html: &str,
) -> String {
    let first_class = if is_first { " sec-first" } else { "" };

    let action_html = match (action_label, action_href) {
        (Some(label), Some(href)) => format!(
            r#"<a href="{}" class="app-btn sec-action">{}</a>"#,
            href, label
        ),
        _ => String::new(),
    };

    format!(
        r#"<div class="section-reveal"><div class="sec{}"><header class="sec-head"><div><p class="lbl">{} &nbsp;·&nbsp; {}</p><h2 class="sec-h" id="sec-{}">{}</h2></div>{}</header><div class="sec-grid">{}</div></div></div>"#,
        first_class,
        form_label,
        subtitle,
        id,
        title,
        action_html,
        children_html
    )
}

/// PageSheet — the main colored section wrapper.
pub fn render_page_sheet(
    stock: StockColor,
    form_label: &str,
    title: &str,
    subtitle: Option<&str>,
    children_html: &str,
) -> String {
    let subtitle_html = subtitle
        .map(|s| format!(" &nbsp;·&nbsp; {}", s))
        .unwrap_or_default();

    format!(
        r#"<section class="app-panel sheet stock-{} page-sheet">{}{}<div class="sheet-inner"><header class="sheet-head"><div class="sheet-head-inner"><p class="lbl">{}{}</p><h1 class="sheet-title">{}</h1></div></header><div class="sheet-body">{}</div></div></section>"#,
        stock.as_str(),
        render_grid_background(),
        render_marks_overlay(DEFAULT_MARKS),
        form_label,
        subtitle_html,
        title,
        children_html
    )
}
