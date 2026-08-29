pub mod overview;
pub mod work;
pub mod projects;
pub mod education;
pub mod volunteering;
pub mod contact;
pub mod certifications;

use crate::data::*;

/// Shared HTML document wrapper with full SEO head.
pub fn render_html_wrapper(
    page_title: &str,
    meta_description: &str,
    canonical_path: &str,
    og_title: &str,
    og_description: &str,
    body_html: &str,
    extra_ld_json: &str,
) -> String {
    let base_url = SITE.site_url.trim_end_matches('/');

    let website_schema = format!(
        r#"{{"@context":"https://schema.org","@type":"WebSite","name":"{}","url":"{}","inLanguage":"en","description":"{}","image":"{}/logo.jpeg","publisher":{{"@type":"Person","name":"{}","url":"{}","image":"{}/logo.jpeg"}}}}"#,
        SITE.name, base_url, SITE.description, base_url, SITE.name, base_url, base_url
    );

    let same_as = r#"["https://linkedin.com/in/achhayapathak","https://github.com/achhayapathak","https://x.com/frozen_parantha","https://leetcode.com/u/achhayapathak/"]"#;
    let knows_about: Vec<String> = KNOWS_ABOUT.iter().map(|k| format!(r#""{}""#, k)).collect();
    let knows_about_json = format!("[{}]", knows_about.join(","));

    let person_schema = format!(
        r#"{{"@context":"https://schema.org","@type":"Person","@id":"{}/#person","name":"{}","alternateName":["frozen_parantha","Achhaya"],"url":"{}","image":"{}/logo.jpeg","logo":"{}/logo.jpeg","jobTitle":"{}","worksFor":{{"@type":"Organization","name":"JoinUp","url":"https://joinup.dev","logo":"{}/logo.jpeg"}},"alumniOf":[{{"@type":"EducationalOrganization","name":"Indian Institute of Technology, Guwahati","url":"https://www.iitg.ac.in/"}},{{"@type":"EducationalOrganization","name":"Hansraj College, University of Delhi","url":"https://www.hansrajcollege.ac.in/"}}],"email":"mailto:{}","address":{{"@type":"PostalAddress","addressLocality":"Gurugram","addressCountry":"IN"}},"knowsAbout":{},"sameAs":{}}}"#,
        base_url, SITE.name, base_url, base_url, base_url, SITE.title, base_url, SITE.email, knows_about_json, same_as
    );

    let profile_page_schema = format!(
        r#"{{"@context":"https://schema.org","@type":"ProfilePage","name":"{}'s Engineering Portfolio & Ledger","url":"{}","primaryImageOfPage":{{"@type":"ImageObject","url":"{}/logo.jpeg","caption":"{} Logo"}},"mainEntity":{{"@id":"{}/#person"}},"isPartOf":{{"@type":"WebSite","name":"{}","url":"{}"}}}}"#,
        SITE.name, base_url, base_url, SITE.name, base_url, SITE.name, base_url
    );

    let canonical_url = if canonical_path == "/" {
        format!("{}/", base_url)
    } else {
        format!("{}{}", base_url, canonical_path)
    };

    let mut out = String::with_capacity(8192);
    out.push_str("<!DOCTYPE html>\n<html lang=\"en\" itemscope itemtype=\"https://schema.org/WebPage\">\n<head>\n<meta charset=\"utf-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1, maximum-scale=5\">\n");
    out.push_str(&format!("<title>{}</title>\n", page_title));
    out.push_str(&format!("<meta name=\"description\" content=\"{}\">\n", meta_description));
    out.push_str(&format!("<meta itemprop=\"name\" content=\"{}\">\n", page_title));
    out.push_str(&format!("<meta itemprop=\"description\" content=\"{}\">\n", meta_description));
    out.push_str(&format!("<meta itemprop=\"image\" content=\"{}/logo.jpeg\">\n", base_url));
    out.push_str(&format!("<meta name=\"application-name\" content=\"{} Portfolio\">\n", SITE.name));
    out.push_str(&format!("<meta name=\"author\" content=\"{}\">\n", SITE.name));
    out.push_str("<meta name=\"generator\" content=\"Rust SSG + WebAssembly\">\n");
    out.push_str("<meta name=\"keywords\" content=\"Achhaya Pathak,Software Engineer,Backend Engineer,Distributed Systems,Cloud Infrastructure,IIT Guwahati,JoinUp,Go,Python,TypeScript,Rust,Kubernetes,Docker,Kafka,RabbitMQ,Redis,PostgreSQL,AWS,GCP,Next.js,Microservices,Event Driven Architecture,Portfolio\">\n");
    out.push_str(&format!("<meta name=\"creator\" content=\"{}\">\n", SITE.name));
    out.push_str(&format!("<meta name=\"publisher\" content=\"{}\">\n", SITE.name));
    out.push_str("<meta name=\"category\" content=\"technology\">\n");
    out.push_str("<meta name=\"classification\" content=\"Portfolio &amp; Engineering Ledger\">\n");
    out.push_str("<meta name=\"color-scheme\" content=\"light dark\">\n");
    out.push_str("<meta name=\"theme-color\" media=\"(prefers-color-scheme: light)\" content=\"#fdf6e3\">\n");
    out.push_str("<meta name=\"theme-color\" media=\"(prefers-color-scheme: dark)\" content=\"#08080b\">\n");
    out.push_str(&format!("<link rel=\"canonical\" href=\"{}\">\n", canonical_url));
    out.push_str(&format!("<link rel=\"image_src\" href=\"{}/logo.jpeg\">\n", base_url));
    out.push_str("<link rel=\"icon\" href=\"/icon.svg\" type=\"image/svg+xml\">\n");
    out.push_str("<link rel=\"icon\" href=\"/logo.jpeg\" sizes=\"192x192\" type=\"image/jpeg\">\n");
    out.push_str("<link rel=\"apple-touch-icon\" href=\"/logo.jpeg\" sizes=\"180x180\">\n");
    out.push_str("<link rel=\"shortcut icon\" href=\"/icon.svg\">\n");
    out.push_str("<link rel=\"manifest\" href=\"/manifest.webmanifest\">\n");
    out.push_str("<meta property=\"og:type\" content=\"website\">\n");
    out.push_str(&format!("<meta property=\"og:site_name\" content=\"{}\">\n", SITE.name));
    out.push_str("<meta property=\"og:locale\" content=\"en_US\">\n");
    out.push_str(&format!("<meta property=\"og:url\" content=\"{}\">\n", canonical_url));
    out.push_str(&format!("<meta property=\"og:title\" content=\"{}\">\n", og_title));
    out.push_str(&format!("<meta property=\"og:description\" content=\"{}\">\n", og_description));
    out.push_str(&format!("<meta property=\"og:image\" content=\"{}/logo.jpeg\">\n", base_url));
    out.push_str(&format!("<meta property=\"og:image:secure_url\" content=\"{}/logo.jpeg\">\n", base_url));
    out.push_str("<meta property=\"og:image:type\" content=\"image/jpeg\">\n");
    out.push_str("<meta property=\"og:image:width\" content=\"600\">\n");
    out.push_str("<meta property=\"og:image:height\" content=\"600\">\n");
    out.push_str(&format!("<meta property=\"og:image:alt\" content=\"{} Logo\">\n", SITE.name));
    out.push_str("<meta name=\"twitter:card\" content=\"summary_large_image\">\n");
    out.push_str(&format!("<meta name=\"twitter:title\" content=\"{}\">\n", og_title));
    out.push_str(&format!("<meta name=\"twitter:description\" content=\"{}\">\n", og_description));
    out.push_str("<meta name=\"twitter:creator\" content=\"@frozen_parantha\">\n");
    out.push_str("<meta name=\"twitter:site\" content=\"@frozen_parantha\">\n");
    out.push_str(&format!("<meta name=\"twitter:image\" content=\"{}/logo.jpeg\">\n", base_url));
    out.push_str(&format!("<meta name=\"twitter:image:alt\" content=\"{} Logo\">\n", SITE.name));
    out.push_str("<meta name=\"robots\" content=\"index, follow, max-image-preview:large, max-snippet:-1, max-video-preview:-1\">\n");
    out.push_str("<link rel=\"preconnect\" href=\"https://fonts.googleapis.com\">\n");
    out.push_str("<link rel=\"preconnect\" href=\"https://fonts.gstatic.com\" crossorigin>\n");
    out.push_str("<link href=\"https://fonts.googleapis.com/css2?family=Archivo+Black&family=JetBrains+Mono:ital,wght@0,100..800;1,100..800&family=Space+Grotesk:wght@300..700&display=swap\" rel=\"stylesheet\">\n");
    out.push_str("<link rel=\"stylesheet\" href=\"/styles.css\">\n");
    out.push_str(&format!("<script type=\"application/ld+json\">{}</script>\n", website_schema));
    out.push_str(&format!("<script type=\"application/ld+json\">{}</script>\n", person_schema));
    out.push_str(&format!("<script type=\"application/ld+json\">{}</script>\n", profile_page_schema));
    if !extra_ld_json.is_empty() {
        out.push_str(extra_ld_json);
        out.push('\n');
    }
    out.push_str("<script>\n(function(){try{var stored=localStorage.getItem(\"theme\");if(stored===\"dark\"){document.documentElement.classList.add(\"dark\")}else{document.documentElement.classList.remove(\"dark\")}}catch(e){}})();\n</script>\n");
    out.push_str("</head>\n<body>\n<a href=\"#main\" class=\"skip-link\">skip to content</a>\n<div style=\"display:block;max-width:1240px;margin:0 auto\">\n");
    out.push_str(body_html);
    out.push_str("\n</div>\n<script type=\"module\" src=\"/app.js\"></script>\n</body>\n</html>");

    out
}

/// Helper for breadcrumb LD+JSON
pub fn breadcrumb_schema(base_url: &str, items: &[(&str, &str)]) -> String {
    let mut list_items = Vec::new();
    for (i, (name, path)) in items.iter().enumerate() {
        list_items.push(format!(
            r#"{{"@type":"ListItem","position":{},"name":"{}","item":"{}{}"}}"#,
            i + 1, name, base_url, path
        ));
    }
    format!(
        r#"<script type="application/ld+json">{{"@context":"https://schema.org","@type":"BreadcrumbList","itemListElement":[{}]}}</script>"#,
        list_items.join(",")
    )
}
