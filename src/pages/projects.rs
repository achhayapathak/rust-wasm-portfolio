// ─────────────────────────────────────────────────────────────
// projects.rs — Projects page (/projects)
// ─────────────────────────────────────────────────────────────

use crate::data::*;
use crate::components::layout::*;
use crate::components::content::*;
use crate::pages::{render_html_wrapper, breadcrumb_schema};

pub fn render() -> String {
    let current_path = "/projects";
    let meta = &SECTION_PROJECTS;
    let base_url = SITE.site_url.trim_end_matches('/');

    let lede = format!(
        r#"<p class="prose-form" style="max-width:56ch;margin-bottom:30px;color:color-mix(in srgb, var(--on-stock) 76%, var(--stock))">{}</p>"#,
        meta.info
    );

    let mut cards = String::new();
    for (i, p) in PROJECTS.iter().enumerate() {
        let body = format!(r#"<p class="prose-form">{}</p>"#, p.summary);
        let mut action = String::new();
        if p.github.is_some() || p.website.is_some() {
            action.push_str(r#"<div style="display:flex;gap:10px;margin-top:18px;flex-wrap:wrap">"#);
            if let Some(gh) = p.github {
                action.push_str(&format!(
                    r#"<a href="{}" target="_blank" rel="noopener noreferrer" class="app-btn">Source Code →</a>"#,
                    gh
                ));
            }
            if let Some(ws) = p.website {
                action.push_str(&format!(
                    r#"<a href="{}" target="_blank" rel="noopener noreferrer" class="app-btn">Live Website →</a>"#,
                    ws
                ));
            }
            action.push_str("</div>");
        }
        cards.push_str(&format!(
            r#"<div class="section-reveal">{}</div>"#,
            render_ledger_card(
                p.stock,
                p.name,
                Some(p.tagline),
                Some(p.period),
                Some(p.status.as_str()),
                None,
                Some(&format!("no. PR-{:02}", i + 1)),
                p.tags,
                &action,
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

    let breadcrumb = breadcrumb_schema(base_url, &[("Overview", "/"), ("Projects", "/projects")]);

    render_html_wrapper(
        &format!("Projects & Systems · {}", SITE.name),
        &format!("Production-grade distributed systems and engineering projects built by {} — including JoinUp, Distributed Document Search, Event Aggregation Engine, Job Assist AI, and Termtalk.", SITE.name),
        "/projects",
        &format!("Projects & Systems · {}", SITE.name),
        &format!("Production systems, distributed search engines, AI automation agents, and open-source packages built by {}.", SITE.name),
        &body,
        &breadcrumb,
    )
}
