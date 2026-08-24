// ─────────────────────────────────────────────────────────────
// work.rs — Work page (/work)
// ─────────────────────────────────────────────────────────────

use crate::data::*;
use crate::components::layout::*;
use crate::components::content::*;
use crate::pages::{render_html_wrapper, breadcrumb_schema};

pub fn render() -> String {
    let current_path = "/work";
    let meta = &SECTION_WORK;
    let base_url = SITE.site_url.trim_end_matches('/');

    let lede = format!(
        r#"<p class="prose-form" style="max-width:56ch;margin-bottom:30px;color:color-mix(in srgb, var(--on-stock) 76%, var(--stock))">{}</p>"#,
        meta.info
    );

    let mut cards = String::new();
    for (i, w) in WORK.iter().enumerate() {
        let mut body = format!(r#"<p class="prose-form">{}</p>"#, w.summary);
        if !w.bullets.is_empty() {
            body.push_str(r#"<ul class="bullet-list">"#);
            for b in w.bullets.iter() {
                body.push_str(&format!(r#"<li class="prose-form">{}</li>"#, b));
            }
            body.push_str("</ul>");
        }
        cards.push_str(&format!(
            r#"<div class="section-reveal">{}</div>"#,
            render_ledger_card(
                w.stock,
                w.role,
                Some(w.company),
                Some(w.period),
                Some(w.status.as_str()),
                None,
                Some(&format!("no. WK-{:02}", i + 1)),
                w.tags,
                "",
                &body,
            )
        ));
    }

    let sheet_body = format!(r#"{}<div class="content-grid">{}</div>"#, lede, cards);

    let page_sheet = render_page_sheet(
        meta.stock,
        &format!("form {}", meta.form),
        meta.title,
        Some(meta.subtitle),
        &sheet_body,
    );

    let body = format!(
        r#"{}{}<main id="main" tabindex="-1" style="display:block;margin-top:34px;outline:none">{}</main>{}"#,
        render_ticker(),
        render_masthead(current_path),
        page_sheet,
        render_footer()
    );

    let breadcrumb = breadcrumb_schema(base_url, &[("Overview", "/"), ("Work", "/work")]);

    render_html_wrapper(
        &format!("Work Experience · {}", SITE.name),
        &format!("{}'s professional engineering experience across JoinUp, Marlin, Gist Impact, and Aristocrat Gaming — building distributed systems, autonomous AI agents, and high-throughput pipelines.", SITE.name),
        "/work",
        &format!("Work Experience · {}", SITE.name),
        &format!("{}'s work experience — engineering leadership, distributed systems, and cloud infrastructure.", SITE.name),
        &body,
        &breadcrumb,
    )
}
