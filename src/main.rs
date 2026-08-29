// ─────────────────────────────────────────────────────────────
// main.rs — Static Site Generator binary.
// Generates all HTML pages, sitemap.xml, robots.txt, and
// manifest.webmanifest into dist/ directory.
// ─────────────────────────────────────────────────────────────

mod data;
mod components;
mod pages;

use std::fs;
use std::path::Path;
use data::SITE;

fn write_page(dir: &str, html: &str) {
    let path = Path::new("dist").join(dir);
    fs::create_dir_all(&path).unwrap_or_else(|_| panic!("Failed to create dir: {}", path.display()));
    let file_path = path.join("index.html");
    fs::write(&file_path, html).unwrap_or_else(|_| panic!("Failed to write: {}", file_path.display()));
    println!("  ✓ {}", file_path.display());
}

fn generate_sitemap(dist: &Path) {
    let base_url = SITE.site_url.trim_end_matches('/');
    let mut sitemap = String::with_capacity(1024);
    sitemap.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
    for path in &["/", "/work", "/projects", "/education", "/volunteering", "/contact"] {
        let (freq, prio) = match *path {
            "/" => ("weekly", "1.0"),
            "/work" | "/projects" => ("weekly", "0.9"),
            "/contact" => ("monthly", "0.8"),
            _ => ("monthly", "0.7"),
        };
        let loc = if *path == "/" {
            format!("{}/", base_url)
        } else {
            format!("{}{}", base_url, path)
        };
        sitemap.push_str(&format!("  <url>\n    <loc>{}</loc>\n    <changefreq>{}</changefreq>\n    <priority>{}</priority>\n  </url>\n", loc, freq, prio));
    }
    sitemap.push_str("</urlset>\n");
    fs::write(dist.join("sitemap.xml"), sitemap).expect("Failed to write sitemap.xml");
    println!("  ✓ dist/sitemap.xml");
}

fn generate_robots(dist: &Path) {
    let base_url = SITE.site_url.trim_end_matches('/');
    let robots = format!(
        "User-agent: *\nAllow: /\n\nSitemap: {}/sitemap.xml\nHost: {}\n",
        base_url, base_url
    );
    fs::write(dist.join("robots.txt"), robots).expect("Failed to write robots.txt");
    println!("  ✓ dist/robots.txt");
}

fn generate_manifest(dist: &Path) {
    let mut manifest = String::with_capacity(1024);
    manifest.push_str("{\n");
    manifest.push_str(&format!("  \"name\": \"{} — Portfolio & Engineering Ledger\",\n", SITE.name));
    manifest.push_str(&format!("  \"short_name\": \"{}\",\n", SITE.name));
    manifest.push_str(&format!("  \"description\": \"{}\",\n", SITE.description));
    manifest.push_str("  \"start_url\": \"/\",\n");
    manifest.push_str("  \"display\": \"standalone\",\n");
    manifest.push_str("  \"background_color\": \"#fdf6e3\",\n");
    manifest.push_str("  \"theme_color\": \"#fdf6e3\",\n");
    manifest.push_str("  \"icons\": [\n");
    manifest.push_str("    {\n      \"src\": \"/favicon-48x48.png\",\n      \"sizes\": \"48x48\",\n      \"type\": \"image/png\"\n    },\n");
    manifest.push_str("    {\n      \"src\": \"/favicon-192x192.png\",\n      \"sizes\": \"192x192\",\n      \"type\": \"image/png\"\n    },\n");
    manifest.push_str("    {\n      \"src\": \"/logo.jpeg\",\n      \"sizes\": \"512x512\",\n      \"type\": \"image/jpeg\"\n    }\n");
    manifest.push_str("  ]\n}\n");
    fs::write(dist.join("manifest.webmanifest"), manifest).expect("Failed to write manifest.webmanifest");
    println!("  ✓ dist/manifest.webmanifest");
}

fn main() {
    println!("🔨 Building portfolio static site...\n");

    // Create dist directory
    let dist = Path::new("dist");
    if dist.exists() {
        fs::remove_dir_all(dist).expect("Failed to clean dist/");
    }
    fs::create_dir_all(dist).expect("Failed to create dist/");

    // Generate pages
    println!("📄 Generating HTML pages:");

    // Overview (index)
    let overview_html = pages::overview::render();
    fs::write(dist.join("index.html"), &overview_html).expect("Failed to write index.html");
    println!("  ✓ dist/index.html");

    // Work
    write_page("work", &pages::work::render());

    // Projects
    write_page("projects", &pages::projects::render());

    // Education
    write_page("education", &pages::education::render());

    // Volunteering
    write_page("volunteering", &pages::volunteering::render());

    // Contact
    write_page("contact", &pages::contact::render());

    // Certifications
    write_page("certifications", &pages::certifications::render());

    // Generate SEO files
    println!("\n🔍 Generating SEO assets:");
    generate_sitemap(dist);
    generate_robots(dist);
    generate_manifest(dist);

    // Copy static assets
    println!("\n📁 Copying static assets:");
    let static_dir = Path::new("static");
    if static_dir.exists() {
        copy_dir_recursive(static_dir, dist);
    }

    println!("\n✅ Build complete! Output in dist/");
    println!("   Serve with: python3 -m http.server -d dist 3000");
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    for entry in fs::read_dir(src).expect("Failed to read static dir") {
        let entry = entry.expect("Failed to read entry");
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            fs::create_dir_all(&dst_path).expect("Failed to create dir");
            copy_dir_recursive(&src_path, &dst_path);
        } else {
            fs::copy(&src_path, &dst_path).unwrap_or_else(|_| panic!(
                "Failed to copy {} -> {}",
                src_path.display(),
                dst_path.display()
            ));
            println!("  ✓ {}", dst_path.display());
        }
    }
}
