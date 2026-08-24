// ─────────────────────────────────────────────────────────────
// volunteering.rs — Volunteering page (/volunteering)
// ─────────────────────────────────────────────────────────────

use crate::data::*;
use crate::components::layout::*;
use crate::components::content::*;
use crate::pages::{render_html_wrapper, breadcrumb_schema};

pub fn render() -> String {
    let current_path = "/volunteering";
    let meta = &SECTION_VOLUNTEERING;
    let base_url = SITE.site_url.trim_end_matches('/');

    let lede = format!(
        r#"<p class="prose-form" style="max-width:56ch;margin-bottom:30px;color:color-mix(in srgb, var(--on-stock) 76%, var(--stock))">{}</p>"#,
        meta.info
    );

    let mut cards = String::new();
    for (i, v) in VOLUNTEERING.iter().enumerate() {
        let body = v.summary
            .map(|s| format!(r#"<p class="prose-form">{}</p>"#, s))
            .unwrap_or_default();
        let status_str = v.status.map(|s| s.as_str());
        cards.push_str(&format!(
            r#"<div class="section-reveal">{}</div>"#,
            render_ledger_card(
                v.stock,
                v.role,
                Some(v.org),
                Some(v.period),
                status_str,
                None,
                Some(&format!("no. VO-{:02}", i + 1)),
                &[],
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

    let breadcrumb = breadcrumb_schema(base_url, &[("Overview", "/"), ("Volunteering", "/volunteering")]);

    render_html_wrapper(
        &format!("Volunteering & Community · {}", SITE.name),
        &format!("Community involvement, open source contributions, and engineering mentorship by {}.", SITE.name),
        "/volunteering",
        &format!("Volunteering & Community · {}", SITE.name),
        &format!("Open-source contributions, technical speaking, and developer mentorship by {}.", SITE.name),
        &body,
        &breadcrumb,
    )
}
