// ─────────────────────────────────────────────────────────────
// overview.rs — Home page (/)
// ─────────────────────────────────────────────────────────────

use crate::data::*;
use crate::components::layout::*;
use crate::components::content::*;
use crate::pages::render_html_wrapper;

pub fn render() -> String {
    let current_path = "/";

    // Build work cards (top 2)
    let mut work_cards = String::new();
    for (i, w) in WORK.iter().take(2).enumerate() {
        let body = format!(r#"<p class="prose-form card-summary">{}</p>"#, w.summary);
        work_cards.push_str(&render_ledger_card(
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
        ));
    }

    // Build project cards (top 2)
    let mut project_cards = String::new();
    for (i, p) in PROJECTS.iter().take(2).enumerate() {
        let body = format!(r#"<p class="prose-form card-summary">{}</p>"#, p.summary);
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
        project_cards.push_str(&render_ledger_card(
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
        ));
    }

    // Build education cards (top 2)
    let mut edu_cards = String::new();
    for (i, e) in EDUCATION.iter().take(2).enumerate() {
        let body = e.summary
            .map(|s| format!(r#"<p class="prose-form card-summary">{}</p>"#, s))
            .unwrap_or_default();
        edu_cards.push_str(&render_ledger_card(
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
        ));
    }

    // Build volunteering cards (top 2)
    let mut vol_cards = String::new();
    for (i, v) in VOLUNTEERING.iter().take(2).enumerate() {
        let body = v.summary
            .map(|s| format!(r#"<p class="prose-form card-summary">{}</p>"#, s))
            .unwrap_or_default();
        let status_str = v.status.map(|s| s.as_str());
        vol_cards.push_str(&render_ledger_card(
            v.stock,
            v.role,
            Some(v.org),
            Some(v.period),
            status_str,
            None,
            Some(&format!("no. VL-{:02}", i + 1)),
            &[],
            "",
            &body,
        ));
    }

    let work_count = WORK.len();
    let project_count = PROJECTS.len();
    let edu_count = EDUCATION.len();
    let vol_count = VOLUNTEERING.len();

    let lede = format!(
        r#"<p class="prose-form" style="max-width:56ch;margin-bottom:30px;color:color-mix(in srgb, var(--on-stock) 76%, var(--stock))">{}</p>"#,
        SECTION_OVERVIEW.info
    );

    let work_section = render_section_block(
        &format!("form {}", SECTION_WORK.form),
        SECTION_WORK.subtitle,
        SECTION_WORK.title,
        "work",
        true,
        Some(&format!("All {} Roles →", work_count)),
        Some("/work"),
        &work_cards,
    );

    let projects_section = render_section_block(
        &format!("form {}", SECTION_PROJECTS.form),
        SECTION_PROJECTS.subtitle,
        SECTION_PROJECTS.title,
        "projects",
        false,
        Some(&format!("All {} Projects →", project_count)),
        Some("/projects"),
        &project_cards,
    );

    let edu_section = render_section_block(
        &format!("form {}", SECTION_EDUCATION.form),
        SECTION_EDUCATION.subtitle,
        SECTION_EDUCATION.title,
        "education",
        false,
        Some(&format!("All {} Entries →", edu_count)),
        Some("/education"),
        &edu_cards,
    );

    let vol_section = render_section_block(
        &format!("form {}", SECTION_VOLUNTEERING.form),
        SECTION_VOLUNTEERING.subtitle,
        SECTION_VOLUNTEERING.title,
        "volunteering",
        false,
        Some(&format!("All {} Roles →", vol_count)),
        Some("/volunteering"),
        &vol_cards,
    );

    let sheet_body = format!("{}{}{}{}{}", lede, work_section, projects_section, edu_section, vol_section);

    let page_sheet = render_page_sheet(
        SECTION_OVERVIEW.stock,
        &format!("form {}", SECTION_OVERVIEW.form),
        SECTION_OVERVIEW.title,
        Some(SECTION_OVERVIEW.subtitle),
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
        &format!("{} · Software Engineer & Backend Architect", SITE.name),
        SITE.description,
        "/",
        &format!("{} · Software Engineer & Backend Architect", SITE.name),
        SITE.description,
        &body,
        "",
    )
}
