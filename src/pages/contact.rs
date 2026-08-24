// ─────────────────────────────────────────────────────────────
// contact.rs — Contact page (/contact)
// ─────────────────────────────────────────────────────────────

use crate::data::*;
use crate::components::layout::*;
use crate::components::content::*;
use crate::components::decorations::render_glyph;
use crate::pages::{render_html_wrapper, breadcrumb_schema};

pub fn render() -> String {
    let current_path = "/contact";
    let meta = &SECTION_CONTACT;
    let base_url = SITE.site_url.trim_end_matches('/');

    let lede = format!(
        r#"<p class="prose-form" style="max-width:56ch;margin-bottom:30px;color:color-mix(in srgb, var(--on-stock) 76%, var(--stock))">{}</p>"#,
        meta.info
    );

    let mut cards = String::new();
    for (i, c) in CONTACT_CARDS.iter().enumerate() {
        let is_mailto = c.url.starts_with("mailto:");
        let target = if is_mailto { "" } else { r#" target="_blank""# };
        let rel = if is_mailto { "" } else { r#" rel="noopener noreferrer""# };

        let body = format!(
            r#"<a href="{}"{}{} class="contact-link">{}<span>{}</span></a><p class="prose-form" style="margin-top:10px;font-size:0.82rem;opacity:0.8">{}</p>"#,
            c.url, target, rel,
            render_glyph(c.icon, 18),
            c.label,
            c.description
        );
        cards.push_str(&render_ledger_card(
            c.stock,
            c.title,
            None,
            None,
            None,
            None,
            Some(&format!("no. CT-{:02}", i + 1)),
            &[],
            "",
            &body,
        ));
    }

    let sheet_body = format!(
        r#"{}<div class="section-reveal"><div class="contact-cards-grid">{}</div></div>"#,
        lede, cards
    );

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

    let breadcrumb = breadcrumb_schema(base_url, &[("Overview", "/"), ("Connect", "/contact")]);

    let contact_page_schema = format!(
        r#"<script type="application/ld+json">{{"@context":"https://schema.org","@type":"ContactPage","name":"Connect with {}","url":"{}/contact","description":"Get in touch with {} — {}.","mainEntity":{{"@type":"Person","name":"{}","email":"mailto:{}","url":"{}"}}}}</script>"#,
        SITE.name, base_url, SITE.name, SITE.title, SITE.name, SITE.email, base_url
    );

    let extra_ld = format!("{}{}", breadcrumb, contact_page_schema);

    render_html_wrapper(
        &format!("Connect & Contact · {}", SITE.name),
        &format!("Get in touch with {} — {} & Backend Architect. Connect via Email, LinkedIn, GitHub, LeetCode, or X (Twitter).", SITE.name, SITE.title),
        "/contact",
        &format!("Connect & Contact · {}", SITE.name),
        &format!("Direct contact channels and social profiles for {}. Open to backend, distributed systems, and platform engineering roles.", SITE.name),
        &body,
        &extra_ld,
    )
}
