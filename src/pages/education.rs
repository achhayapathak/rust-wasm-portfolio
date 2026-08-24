// ─────────────────────────────────────────────────────────────
// education.rs — Education page (/education)
// ─────────────────────────────────────────────────────────────

use crate::data::*;
use crate::components::layout::*;
use crate::components::content::*;
use crate::pages::{render_html_wrapper, breadcrumb_schema};

pub fn render() -> String {
    let current_path = "/education";
    let meta = &SECTION_EDUCATION;
    let base_url = SITE.site_url.trim_end_matches('/');

    let lede = format!(
        r#"<p class="prose-form" style="max-width:56ch;margin-bottom:30px;color:color-mix(in srgb, var(--on-stock) 76%, var(--stock))">{}</p>"#,
        meta.info
    );

    let mut cards = String::new();
    for (i, e) in EDUCATION.iter().enumerate() {
        let body = e.summary
            .map(|s| format!(r#"<p class="prose-form">{}</p>"#, s))
            .unwrap_or_default();
        cards.push_str(&format!(
            r#"<div class="section-reveal">{}</div>"#,
            render_ledger_card(
                e.stock,
                e.degree,
                Some(e.institution),
                Some(e.period),
                Some(e.status.as_str()),
                None,
                Some(&format!("no. ED-{:02}", i + 1)),
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

    let breadcrumb = breadcrumb_schema(base_url, &[("Overview", "/"), ("Education", "/education")]);

    render_html_wrapper(
        &format!("Education & Academics · {}", SITE.name),
        &format!("Academic background of {} — Master's in Mathematics and Computing from Indian Institute of Technology Guwahati (IIT Guwahati), Bachelor's in Mathematics from Hansraj College, University of Delhi.", SITE.name),
        "/education",
        &format!("Education & Academics · {}", SITE.name),
        &format!("Academic background of {} — M.S. Mathematics & Computing from IIT Guwahati, B.S. Mathematics from Hansraj College, DU.", SITE.name),
        &body,
        &breadcrumb,
    )
}
