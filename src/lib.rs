// ─────────────────────────────────────────────────────────────
// lib.rs — WASM client library for interactive behaviors.
// Handles: dark mode toggle, mobile nav, expandable text,
// section reveal animations.
// ─────────────────────────────────────────────────────────────

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Document, Element, HtmlElement, IntersectionObserver, IntersectionObserverInit};

#[wasm_bindgen(start)]
pub fn init() {
    let window = match window() {
        Some(w) => w,
        None => return,
    };
    let document = match window.document() {
        Some(d) => d,
        None => return,
    };

    setup_dark_mode(&document);
    setup_mobile_nav(&document);
    setup_expandable_text(&document);
    setup_section_reveal(&window, &document);
}

// ── Dark Mode Toggle ─────────────────────────────────────────

fn setup_dark_mode(document: &Document) {
    let toggle = match document.get_element_by_id("dark-mode-toggle") {
        Some(el) => el,
        None => return,
    };

    let doc = document.clone();
    let closure = Closure::wrap(Box::new(move || {
        let html = doc.document_element().unwrap();
        let is_dark = html.class_list().contains("dark");

        if is_dark {
            html.class_list().remove_1("dark").unwrap();
            if let Ok(Some(storage)) = window().unwrap().local_storage() {
                let _ = storage.set_item("theme", "light");
            }
        } else {
            html.class_list().add_1("dark").unwrap();
            if let Ok(Some(storage)) = window().unwrap().local_storage() {
                let _ = storage.set_item("theme", "dark");
            }
        }

        update_dark_mode_ui(&doc);
    }) as Box<dyn FnMut()>);

    toggle
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();

    // Set initial UI state
    update_dark_mode_ui(document);
}

fn update_dark_mode_ui(document: &Document) {
    let html = match document.document_element() {
        Some(h) => h,
        None => return,
    };
    let is_dark = html.class_list().contains("dark");

    if let Some(icon_el) = document.get_element_by_id("dark-mode-icon") {
        let icon_svg = if is_dark {
            // Sun icon
            r#"<span class="glyph-icon" style="width:17px;display:inline-block;flex-shrink:0"><svg viewBox="0 0 24 24" aria-hidden="true" focusable="false" style="display:block;height:auto;width:100%;fill:none;stroke:currentColor;stroke-width:2.1;stroke-linecap:square;stroke-linejoin:miter"><circle cx="12" cy="12" r="4"/><path d="M12 2 V5 M12 19 V22 M2 12 H5 M19 12 H22 M4.9 4.9 L7 7 M17 17 L19.1 19.1 M19.1 4.9 L17 7 M7 17 L4.9 19.1"/></svg></span>"#
        } else {
            // Moon icon
            r#"<span class="glyph-icon" style="width:17px;display:inline-block;flex-shrink:0"><svg viewBox="0 0 24 24" aria-hidden="true" focusable="false" style="display:block;height:auto;width:100%;fill:none;stroke:currentColor;stroke-width:2.1;stroke-linecap:square;stroke-linejoin:miter"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/></svg></span>"#
        };
        icon_el.set_inner_html(icon_svg);
    }

    if let Some(label_el) = document.get_element_by_id("dark-mode-label") {
        label_el.set_text_content(Some(if is_dark { "dark" } else { "light" }));
    }

    if let Some(toggle) = document.get_element_by_id("dark-mode-toggle") {
        let label = if is_dark {
            "Switch to light mode"
        } else {
            "Switch to dark mode"
        };
        let _ = toggle.set_attribute("aria-label", label);
    }
}

// ── Mobile Navigation ────────────────────────────────────────

fn setup_mobile_nav(document: &Document) {
    let trigger = match document.get_element_by_id("mobile-trigger") {
        Some(el) => el,
        None => return,
    };

    // Open handler
    let doc = document.clone();
    let open_closure = Closure::wrap(Box::new(move || {
        if let Some(sheet) = doc.get_element_by_id("mobile-sheet") {
            if let Some(html_sheet) = sheet.dyn_ref::<HtmlElement>() {
                let _ = html_sheet.style().set_property("display", "block");
            }
        }
        if let Some(body) = doc.body() {
            let _ = body.class_list().add_1("nav-open");
        }
        if let Some(trigger) = doc.get_element_by_id("mobile-trigger") {
            let _ = trigger.set_attribute("aria-expanded", "true");
        }
    }) as Box<dyn FnMut()>);

    trigger
        .add_event_listener_with_callback("click", open_closure.as_ref().unchecked_ref())
        .unwrap();
    open_closure.forget();

    // Close handler
    let close = match document.get_element_by_id("mobile-close") {
        Some(el) => el,
        None => return,
    };

    let doc2 = document.clone();
    let close_closure = Closure::wrap(Box::new(move || {
        close_mobile_nav(&doc2);
    }) as Box<dyn FnMut()>);

    close.add_event_listener_with_callback("click", close_closure.as_ref().unchecked_ref()).unwrap();
    close_closure.forget();

    // Close on nav link click
    let doc3 = document.clone();
    if let Ok(links) = document.query_selector_all(".mobile-row") {
        for i in 0..links.length() {
            if let Some(link) = links.item(i) {
                let doc_clone = doc3.clone();
                let link_closure = Closure::wrap(Box::new(move || {
                    close_mobile_nav(&doc_clone);
                }) as Box<dyn FnMut()>);
                let _ = link.add_event_listener_with_callback("click", link_closure.as_ref().unchecked_ref());
                link_closure.forget();
            }
        }
    }
}

fn close_mobile_nav(document: &Document) {
    if let Some(sheet) = document.get_element_by_id("mobile-sheet") {
        if let Some(html_sheet) = sheet.dyn_ref::<HtmlElement>() {
            let _ = html_sheet.style().set_property("display", "none");
        }
    }
    if let Some(body) = document.body() {
        let _ = body.class_list().remove_1("nav-open");
    }
    if let Some(trigger) = document.get_element_by_id("mobile-trigger") {
        let _ = trigger.set_attribute("aria-expanded", "false");
    }
}

// ── Expandable Text ──────────────────────────────────────────

fn setup_expandable_text(document: &Document) {
    let wrappers = match document.query_selector_all(".expandable-wrapper") {
        Ok(w) => w,
        Err(_) => return,
    };
    if wrappers.length() == 0 {
        return;
    }

    // Schedule measurements inside request_animation_frame to prevent forced synchronous reflow
    let doc = document.clone();
    let r_closure = Closure::wrap(Box::new(move || {
        let wrappers = match doc.query_selector_all(".expandable-wrapper") {
            Ok(w) => w,
            Err(_) => return,
        };

        // Phase 1 (Batch Reads): Read layout dimensions without writing to the DOM
        let mut overflow_elements = Vec::new();
        for i in 0..wrappers.length() {
            if let Some(node) = wrappers.item(i) {
                if let Some(wrapper_el) = node.dyn_ref::<Element>() {
                    if let Ok(Some(content_el)) = wrapper_el.query_selector(".expandable-content") {
                        if let Some(html_content) = content_el.dyn_ref::<HtmlElement>() {
                            let overflows = html_content.scroll_height() > html_content.client_height();
                            if overflows {
                                if let Ok(Some(btn)) = wrapper_el.query_selector(".expand-toggle") {
                                    if let Ok(html_btn) = btn.dyn_into::<HtmlElement>() {
                                        overflow_elements.push((content_el, html_btn));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Phase 2 (Batch Writes): Update classes and attach event handlers
        for (content_el, html_btn) in overflow_elements {
            let _ = html_btn.style().set_property("display", "inline-block");
            let content_clone = content_el.clone();
            let btn_clone = html_btn.clone();
            let closure = Closure::wrap(Box::new(move || {
                let is_clamped = content_clone.class_list().contains("clamped");
                if is_clamped {
                    let _ = content_clone.class_list().remove_1("clamped");
                    btn_clone.set_text_content(Some("– show less"));
                } else {
                    let _ = content_clone.class_list().add_1("clamped");
                    btn_clone.set_text_content(Some("+ read full entry"));
                }
            }) as Box<dyn FnMut()>);

            let _ = html_btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    }) as Box<dyn FnMut()>);

    if let Some(w) = window() {
        let _ = w.request_animation_frame(r_closure.as_ref().unchecked_ref());
    }
    r_closure.forget();
}

// ── Section Reveal ───────────────────────────────────────────

fn setup_section_reveal(_window: &web_sys::Window, document: &Document) {
    let sections = match document.query_selector_all(".section-reveal") {
        Ok(s) => s,
        Err(_) => return,
    };
    if sections.length() == 0 {
        return;
    }

    let callback = Closure::wrap(Box::new(move |entries: js_sys::Array, observer: IntersectionObserver| {
        for i in 0..entries.length() {
            let entry: web_sys::IntersectionObserverEntry = entries.get(i).unchecked_into();
            if entry.is_intersecting() {
                let target = entry.target();
                let _ = target.class_list().add_1("revealed");
                observer.unobserve(&target);
            }
        }
    }) as Box<dyn FnMut(js_sys::Array, IntersectionObserver)>);

    let options = IntersectionObserverInit::new();
    options.set_root_margin("-50px");

    if let Ok(observer) = IntersectionObserver::new_with_options(
        callback.as_ref().unchecked_ref(),
        &options,
    ) {
        for i in 0..sections.length() {
            if let Some(node) = sections.item(i) {
                if let Some(el) = node.dyn_ref::<Element>() {
                    observer.observe(el);
                }
            }
        }
    }

    callback.forget();
}
