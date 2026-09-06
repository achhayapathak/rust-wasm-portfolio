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
        r#"{{"@context":"https://schema.org","@type":"WebSite","@id":"{}/#website","name":"{}","alternateName":["Achhaya","Achhaya Pathak Portfolio","frozen_parantha","achhaya.com"],"url":"{}/","inLanguage":"en","description":"{}","image":"{}/logo.jpeg","logo":"{}/logo.jpeg","publisher":{{"@type":"Person","name":"{}","url":"{}/","image":"{}/logo.jpeg","logo":"{}/logo.jpeg"}}}}"#,
        base_url, SITE.name, base_url, SITE.description, base_url, base_url, SITE.name, base_url, base_url, base_url
    );

    let same_as = r#"["https://linkedin.com/in/achhayapathak","https://github.com/achhayapathak","https://x.com/frozen_parantha","https://leetcode.com/u/achhayapathak/"]"#;
    let knows_about: Vec<String> = KNOWS_ABOUT.iter().map(|k| format!(r#""{}""#, k)).collect();
    let knows_about_json = format!("[{}]", knows_about.join(","));

    let person_schema = format!(
        r#"{{"@context":"https://schema.org","@type":"Person","@id":"{}/#person","name":"{}","alternateName":["frozen_parantha","Achhaya"],"url":"{}","image":"{}/logo.jpeg","logo":"{}/logo.jpeg","jobTitle":"{}","worksFor":{{"@type":"Organization","name":"JoinUp","url":"https://joinup.dev","logo":"{}/logo.jpeg"}},"alumniOf":[{{"@type":"EducationalOrganization","name":"Indian Institute of Technology, Guwahati","url":"https://www.iitg.ac.in/"}},{{"@type":"EducationalOrganization","name":"Hansraj College, University of Delhi","url":"https://www.hansrajcollege.ac.in/"}}],"email":"mailto:{}","address":{{"@type":"PostalAddress","addressLocality":"Gurugram","addressCountry":"IN"}},"knowsAbout":{},"sameAs":{}}}"#,
        base_url, SITE.name, base_url, base_url, base_url, SITE.title, base_url, SITE.email, knows_about_json, same_as
    );

    let org_schema = format!(
        r#"{{"@context":"https://schema.org","@type":"Organization","name":"{}","url":"{}/","logo":"{}/logo.jpeg","image":"{}/logo.jpeg","sameAs":{}}}"#,
        SITE.name, base_url, base_url, base_url, same_as
    );

    let profile_page_schema = format!(
        r#"{{"@context":"https://schema.org","@type":"ProfilePage","name":"{}'s Engineering Portfolio & Ledger","url":"{}","primaryImageOfPage":{{"@type":"ImageObject","url":"{}/logo.jpeg","caption":"{} Logo"}},"mainEntity":{{"@id":"{}/#person"}},"isPartOf":{{"@id":"{}/#website"}}}}"#,
        SITE.name, base_url, base_url, SITE.name, base_url, base_url
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
    out.push_str(&format!("<meta name=\"application-name\" content=\"{}\">\n", SITE.name));
    out.push_str(&format!("<meta name=\"apple-mobile-web-app-title\" content=\"{}\">\n", SITE.name));
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
    out.push_str("<link rel=\"icon\" href=\"/favicon.ico\" sizes=\"48x48\">\n");
    out.push_str("<link rel=\"icon\" type=\"image/svg+xml\" href=\"/favicon.svg\" sizes=\"any\">\n");
    out.push_str("<link rel=\"icon\" type=\"image/png\" sizes=\"48x48\" href=\"/favicon-48x48.png\">\n");
    out.push_str("<link rel=\"icon\" type=\"image/png\" sizes=\"96x96\" href=\"/favicon-96x96.png\">\n");
    out.push_str("<link rel=\"icon\" type=\"image/png\" sizes=\"192x192\" href=\"/favicon-192x192.png\">\n");
    out.push_str("<link rel=\"apple-touch-icon\" sizes=\"180x180\" href=\"/apple-touch-icon.png\">\n");
    out.push_str("<link rel=\"shortcut icon\" href=\"/favicon.ico\">\n");
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
    // Preload critical local typography for 0-latency paint
    out.push_str("<link rel=\"preload\" href=\"/fonts/space-grotesk.woff2\" as=\"font\" type=\"font/woff2\" crossorigin>\n");
    out.push_str("<link rel=\"preload\" href=\"/fonts/archivo-black.woff2\" as=\"font\" type=\"font/woff2\" crossorigin>\n");
    out.push_str("<link rel=\"preload\" href=\"/fonts/jetbrains-mono.woff2\" as=\"font\" type=\"font/woff2\" crossorigin>\n");
    // Preload critical JS & WASM to break waterfall chains
    out.push_str("<link rel=\"modulepreload\" href=\"/app.js\">\n");
    out.push_str("<link rel=\"modulepreload\" href=\"/pkg/portfolio_wasm.js\">\n");
    out.push_str("<link rel=\"preload\" href=\"/pkg/portfolio_wasm_bg.wasm\" as=\"fetch\" type=\"application/wasm\" crossorigin>\n");
    // Inlined Critical CSS — 0 render-blocking CSS requests, 0 external network requests
    out.push_str("<style>\n");
    out.push_str(include_str!("../../static/styles.min.css"));
    out.push_str("\n</style>\n");
    out.push_str(&format!("<script type=\"application/ld+json\">{}</script>\n", website_schema));
    out.push_str(&format!("<script type=\"application/ld+json\">{}</script>\n", person_schema));
    out.push_str(&format!("<script type=\"application/ld+json\">{}</script>\n", org_schema));
    out.push_str(&format!("<script type=\"application/ld+json\">{}</script>\n", profile_page_schema));
    if !extra_ld_json.is_empty() {
        out.push_str(extra_ld_json);
        out.push('\n');
    }
    // Instant synchronous UI script (0ms latency dark mode & menu)
    out.push_str(r#"<script>
(function(){
  function syncUI(d){
    var ic=document.getElementById("dark-mode-icon"),lb=document.getElementById("dark-mode-label"),dt=document.getElementById("dark-mode-toggle");
    if(ic)ic.innerHTML=d?'<span class="glyph-icon" style="width:17px;display:inline-block;flex-shrink:0"><svg viewBox="0 0 24 24" aria-hidden="true" focusable="false" style="display:block;height:auto;width:100%;fill:none;stroke:currentColor;stroke-width:2.1;stroke-linecap:square;stroke-linejoin:miter"><circle cx="12" cy="12" r="4"/><path d="M12 2 V5 M12 19 V22 M2 12 H5 M19 12 H22 M4.9 4.9 L7 7 M17 17 L19.1 19.1 M19.1 4.9 L17 7 M7 17 L4.9 19.1"/></svg></span>':'<span class="glyph-icon" style="width:17px;display:inline-block;flex-shrink:0"><svg viewBox="0 0 24 24" aria-hidden="true" focusable="false" style="display:block;height:auto;width:100%;fill:none;stroke:currentColor;stroke-width:2.1;stroke-linecap:square;stroke-linejoin:miter"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/></svg></span>';
    if(lb)lb.textContent=d?"dark":"light";
    if(dt)dt.setAttribute("aria-label",d?"Switch to light mode":"Switch to dark mode");
  }
  try{
    var s=localStorage.getItem("theme");
    var isDark = s==="dark";
    if(isDark){document.documentElement.classList.add("dark")}
    else{document.documentElement.classList.remove("dark")}
  }catch(e){}
  document.addEventListener("DOMContentLoaded",function(){
    syncUI(document.documentElement.classList.contains("dark"));
    var dt=document.getElementById("dark-mode-toggle");
    if(dt){
      dt.onclick=function(){
        var d=document.documentElement.classList.toggle("dark");
        try{localStorage.setItem("theme",d?"dark":"light")}catch(e){}
        syncUI(d);
      };
    }
    var mt=document.getElementById("mobile-trigger"),ms=document.getElementById("mobile-sheet"),mc=document.getElementById("mobile-close");
    if(mt&&ms){mt.onclick=function(){ms.style.display="block";document.body.classList.add("nav-open");mt.setAttribute("aria-expanded","true")}}
    if(mc&&ms){mc.onclick=function(){ms.style.display="none";document.body.classList.remove("nav-open");if(mt)mt.setAttribute("aria-expanded","false")}}
    document.querySelectorAll(".mobile-row").forEach(function(r){r.onclick=function(){if(ms)ms.style.display="none";document.body.classList.remove("nav-open");if(mt)mt.setAttribute("aria-expanded","false")}});
    if("IntersectionObserver" in window){
      var obs=new IntersectionObserver(function(es){es.forEach(function(e){if(e.isIntersecting){e.target.classList.add("revealed");obs.unobserve(e.target)}})},{rootMargin:"-50px"});
      document.querySelectorAll(".section-reveal").forEach(function(el){obs.observe(el)});
    }else{
      document.querySelectorAll(".section-reveal").forEach(function(el){el.classList.add("revealed")});
    }
  });
})();
</script>
"#);
    out.push_str("</head>\n<body>\n<a href=\"#main\" class=\"skip-link\">skip to content</a>\n<div style=\"display:block;max-width:1240px;margin:0 auto\">\n");
    out.push_str(body_html);
    out.push_str("\n</div>\n<script type=\"module\" src=\"/app.js\" defer></script>\n</body>\n</html>");

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
