// ─────────────────────────────────────────────────────────────
// certifications.rs — Certifications page (/certifications)
// ─────────────────────────────────────────────────────────────

use crate::data::*;
use crate::components::layout::*;
use crate::components::content::*;
use crate::pages::render_html_wrapper;

pub fn render() -> String {
    let current_path = "/certifications";
    let meta = &SECTION_CERTIFICATIONS;

    let lede = format!(
        r#"<p class="prose-form" style="max-width:56ch;margin-bottom:30px;color:color-mix(in srgb, var(--on-stock) 76%, var(--stock))">{}</p>"#,
        meta.info
    );

    let mut cards = String::new();
    for c in CERTIFICATIONS.iter() {
        let body = c.credential_id
            .map(|id| format!(r#"<p class="lbl" style="margin-top:4px">credential id: {}</p>"#, id))
            .unwrap_or_default();
        cards.push_str(&format!(
            r#"<div class="section-reveal">{}</div>"#,
            render_ledger_card(
                c.stock,
                c.name,
                Some(c.issuer),
                Some(c.date),
                Some(c.status.as_str()),
                None,
                None,
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

    render_html_wrapper(
        &format!("Certifications · {}", SITE.name),
        &format!("{}'s certifications and credentials.", SITE.name),
        "/certifications",
        &format!("Certifications · {}", SITE.name),
        &format!("{}'s certifications and credentials.", SITE.name),
        &body,
        "",
    )
}
