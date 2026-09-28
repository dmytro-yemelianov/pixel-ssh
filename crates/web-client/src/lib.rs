#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code, unused_imports))]

use std::cell::RefCell;
use std::rc::Rc;

use pixel_ssh_core::App;
use pixel_ssh_framebuffer::Framebuffer;
use pixel_ssh_render_web::WebGlRenderer;
use pixel_ssh_view::{Button, InputEvent, Key};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

struct ClientState {
    app: App,
    framebuffer: Framebuffer,
    renderer: WebGlRenderer,
    dirty: bool,
    mouse_uv: (f32, f32),
    mouse_active: bool,
    start_time_ms: f64,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Setup panic hook if in browser
    console_error_panic_hook_init();

    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;
    let canvas = document
        .get_element_by_id("screen")
        .ok_or("Canvas element #screen not found")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let win_w = window.inner_width().ok().and_then(|w| w.as_f64()).unwrap_or(1280.0);
    let win_h = window.inner_height().ok().and_then(|h| h.as_f64()).unwrap_or(800.0);
    let is_portrait = win_w < win_h;
    let mut has_explicit_system = false;

    let mut app = App::new();
    let mut init_mouse_uv = (0.5f32, 0.5f32);
    let mut init_mouse_active = false;
    let mut init_time_offset = 0.0f32;

    if let Ok(search) = window.location().search() {
        if search.contains("tab=resume") {
            app.current_tab = pixel_ssh_core::Tab::Resume;
        } else if search.contains("tab=about") {
            app.current_tab = pixel_ssh_core::Tab::About;
        } else if search.contains("tab=visuals") {
            app.current_tab = pixel_ssh_core::Tab::Visuals;
        } else if search.contains("tab=contact") {
            app.current_tab = pixel_ssh_core::Tab::Contact;
        } else if search.contains("tab=help") {
            app.current_tab = pixel_ssh_core::Tab::Help;
        }
        if search.contains("detail=1") {
            app.show_detail = true;
        }
        for (pattern, mode) in [
            ("svga", pixel_ssh_view::SystemMode::Svga),
            ("sga", pixel_ssh_view::SystemMode::Sga),
            ("ega", pixel_ssh_view::SystemMode::Ega),
            ("cga", pixel_ssh_view::SystemMode::Cga),
            ("zx", pixel_ssh_view::SystemMode::ZxSpectrum),
            ("c64", pixel_ssh_view::SystemMode::C64),
            ("atari", pixel_ssh_view::SystemMode::Atari),
            ("amber", pixel_ssh_view::SystemMode::Amber),
            ("green", pixel_ssh_view::SystemMode::GreenCrt),
            ("vga", pixel_ssh_view::SystemMode::Vga),
        ] {
            if search.contains(&format!("system={}", pattern))
                || search.contains(&format!("palette={}", pattern))
                || search.contains(&format!("mode={}", pattern))
            {
                app.system_mode = mode;
                app.palette_mode = mode;
                has_explicit_system = true;
            }
        }
        if search.contains("modal=visuals") {
            app.active_modal = pixel_ssh_view::ActiveModal::Visuals;
        } else if search.contains("modal=system") {
            app.active_modal = pixel_ssh_view::ActiveModal::System;
        } else if search.contains("modal=help") {
            app.active_modal = pixel_ssh_view::ActiveModal::Help;
        }
        if search.contains("screensaver=1") {
            app.screensaver_active = true;
        }
        if let Some(idx_str) = search.split("project=").nth(1) {
            let num_str: String = idx_str.chars().take_while(|c| c.is_ascii_digit()).collect();
            if let Ok(idx) = num_str.parse::<usize>() {
                if idx < pixel_ssh_core::PROJECTS.len() {
                    app.selected_project = idx;
                }
            }
        }
        if let Some(scroll_str) = search.split("scroll=").nth(1) {
            let num_str: String = scroll_str.chars().take_while(|c| c.is_ascii_digit()).collect();
            if let Ok(s) = num_str.parse::<usize>() {
                app.detail_scroll = s.min(app.detail_max_scroll());
                app.resume_scroll = s.min(app.resume_max_scroll());
                app.about_scroll = s.min(app.about_max_scroll());
            }
        }
        if search.contains("preset=clean") {
            app.visual_effects = pixel_ssh_view::VisualEffects::clean();
        } else if search.contains("preset=arcade") {
            app.visual_effects = pixel_ssh_view::VisualEffects::crt_arcade();
        } else if search.contains("preset=bloom") {
            app.visual_effects = pixel_ssh_view::VisualEffects::phosphor_bloom();
        } else if search.contains("preset=glitch") {
            app.visual_effects = pixel_ssh_view::VisualEffects::retro_glitch();
        } else if search.contains("preset=crt") || search.contains("preset=trinitron") {
            app.visual_effects = pixel_ssh_view::VisualEffects::crt_trinitron();
        }

        if let Some(m_str) = search.split("magnet=").nth(1) {
            let num_str: String = m_str.chars().take_while(|c| c.is_ascii_digit() || *c == '.').collect();
            if let Ok(m) = num_str.parse::<f32>() {
                app.visual_effects.magnet = m.clamp(0.0, 1.0);
            }
        }
        if let Some(n_str) = search.split("noise=").nth(1) {
            let num_str: String = n_str.chars().take_while(|c| c.is_ascii_digit() || *c == '.').collect();
            if let Ok(n) = num_str.parse::<f32>() {
                app.visual_effects.noise = n.clamp(0.0, 1.0);
            }
        }
        if let Some(h_str) = search.split("hum=").nth(1) {
            let num_str: String = h_str.chars().take_while(|c| c.is_ascii_digit() || *c == '.').collect();
            if let Ok(h) = num_str.parse::<f32>() {
                app.visual_effects.antenna_hum = h.clamp(0.0, 1.0);
            }
        }
        if let Some(t_str) = search.split("time=").nth(1) {
            let num_str: String = t_str.chars().take_while(|c| c.is_ascii_digit() || *c == '.').collect();
            if let Ok(t) = num_str.parse::<f32>() {
                init_time_offset = t;
            }
        }
        if let Some(mx_str) = search.split("mouse_x=").nth(1) {
            let num_str: String = mx_str.chars().take_while(|c| c.is_ascii_digit() || *c == '.').collect();
            if let Ok(mx) = num_str.parse::<f32>() {
                init_mouse_uv.0 = mx.clamp(0.0, 1.0);
                init_mouse_active = true;
            }
        }
        if let Some(my_str) = search.split("mouse_y=").nth(1) {
            let num_str: String = my_str.chars().take_while(|c| c.is_ascii_digit() || *c == '.').collect();
            if let Ok(my) = num_str.parse::<f32>() {
                init_mouse_uv.1 = my.clamp(0.0, 1.0);
                init_mouse_active = true;
            }
        }
        if init_mouse_active {
            let (fb_w, fb_h) = app.system_mode.resolution();
            let fb_x = ((init_mouse_uv.0 * fb_w as f32).round() as u16).min(fb_w.saturating_sub(1));
            let fb_y = ((init_mouse_uv.1 * fb_h as f32).round() as u16).min(fb_h.saturating_sub(1));
            app.mouse_pos = Some((fb_x, fb_y));
        }
    }

    // Adaptive default: If loaded in portrait / mobile orientation and no explicit mode query param was passed,
    // default to 40-column C64 mode so characters are large (~10px), readable, and touchable!
    if is_portrait && !has_explicit_system {
        app.system_mode = pixel_ssh_view::SystemMode::C64;
        app.palette_mode = pixel_ssh_view::SystemMode::C64;
    }

    let (init_w, init_h) = app.system_mode.resolution();
    let scale: u32 = if init_w <= 320 { 4 } else { 2 };
    let draw_w = init_w as u32 * scale;
    let draw_h = init_h as u32 * scale;
    canvas.set_width(draw_w);
    canvas.set_height(draw_h);

    let renderer = WebGlRenderer::new(canvas.clone(), init_w, init_h)
        .map_err(|e| JsValue::from_str(&e))?;
    let framebuffer = Framebuffer::new(init_w, init_h);

    let start_time_ms = js_sys::Date::now() - (init_time_offset as f64 * 1000.0);

    let state = Rc::new(RefCell::new(ClientState {
        app,
        framebuffer,
        renderer,
        dirty: true,
        mouse_uv: init_mouse_uv,
        mouse_active: init_mouse_active,
        start_time_ms,
    }));

    // Initial render
    {
        let mut state_guard = state.borrow_mut();
        let s = &mut *state_guard;
        let view = s.app.render();
        s.framebuffer.draw_view(&view);
        let init_time = init_time_offset;
        s.renderer.render_frame_with_effects(&s.framebuffer, &s.app.visual_effects, init_time, s.mouse_uv, s.mouse_active);
        s.dirty = false;

        let tab_idx = match s.app.current_tab {
            pixel_ssh_core::Tab::Projects => 1,
            pixel_ssh_core::Tab::Resume => 2,
            pixel_ssh_core::Tab::About => 3,
            pixel_ssh_core::Tab::Contact => 4,
            _ => 1,
        };
        if let Some(deck) = document.get_element_by_id("cyberdeck") {
            let _ = deck.set_attribute("data-tab", &tab_idx.to_string());
            let modal_str = match s.app.active_modal {
                pixel_ssh_view::ActiveModal::None => "none",
                pixel_ssh_view::ActiveModal::Visuals => "visuals",
                pixel_ssh_view::ActiveModal::Help => "help",
                pixel_ssh_view::ActiveModal::System => "system",
            };
            let _ = deck.set_attribute("data-modal", modal_str);
            let _ = deck.set_attribute("data-mode", s.app.system_mode.short_name());
        }
        if let Some(indicator) = document.get_element_by_id("deck-mode-indicator") {
            indicator.set_text_content(Some(&format!("SYS: {}", s.app.system_mode.short_name())));
        }
    }

    // Keyboard event listener
    {
        let state = Rc::clone(&state);
        let closure = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(move |event: web_sys::KeyboardEvent| {
            let key = event.key();
            let parsed_key = match key.as_str() {
                "ArrowUp" => Some(Key::Up),
                "ArrowDown" => Some(Key::Down),
                "ArrowLeft" => Some(Key::Left),
                "ArrowRight" => Some(Key::Right),
                "Enter" => Some(Key::Enter),
                "Escape" => Some(Key::Escape),
                "Tab" => {
                    event.prevent_default();
                    Some(Key::Tab)
                }
                "Backspace" => Some(Key::Backspace),
                "F1" => { event.prevent_default(); Some(Key::F(1)) },
                "F2" => { event.prevent_default(); Some(Key::F(2)) },
                "F3" => { event.prevent_default(); Some(Key::F(3)) },
                "F4" => { event.prevent_default(); Some(Key::F(4)) },
                "F5" => { event.prevent_default(); Some(Key::F(5)) },
                "F6" => { event.prevent_default(); Some(Key::F(6)) },
                "F7" => { event.prevent_default(); Some(Key::F(7)) },
                "F8" => { event.prevent_default(); Some(Key::F(8)) },
                "F9" => { event.prevent_default(); Some(Key::F(9)) },
                "F10" => { event.prevent_default(); Some(Key::F(10)) },
                s if s.len() == 1 => {
                    let ch = s.chars().next().unwrap();
                    Some(Key::Char(ch))
                }
                _ => None,
            };

            if let Some(k) = parsed_key {
                let mut s = state.borrow_mut();
                if s.app.update(InputEvent::KeyDown(k)) {
                    s.dirty = true;
                }
            }
        });

        window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Mouse / Pointer click listener for interactive links, tabs, and project rows
    {
        let state = Rc::clone(&state);
        let canvas_clone = canvas.clone();
        let closure = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
            let rect = canvas_clone.get_bounding_client_rect();
            let rect_width = rect.width();
            let rect_height = rect.height();
            if rect_width <= 0.0 || rect_height <= 0.0 {
                return;
            }

            let client_x = event.client_x() as f64 - rect.left();
            let client_y = event.client_y() as f64 - rect.top();

            let mut s = state.borrow_mut();
            let (fb_w, fb_h) = s.app.system_mode.resolution();
            let fb_x = ((client_x / rect_width) * fb_w as f64).clamp(0.0, (fb_w - 1) as f64) as u16;
            let fb_y = ((client_y / rect_height) * fb_h as f64).clamp(0.0, (fb_h - 1) as f64) as u16;
            let view = s.app.render();

            // Check if user clicked on a link
            if let Some(link) = view.link_at(fb_x, fb_y) {
                if !link.url.starts_with('#') {
                    if let Some(w) = web_sys::window() {
                        let _ = w.open_with_url_and_target(&link.url, "_blank");
                    }
                }
            }

            // Also update app state (e.g. tabs, project selection, back button)
            if s.app.update(InputEvent::PointerDown {
                x: fb_x,
                y: fb_y,
                button: Button::Left,
            }) {
                s.dirty = true;
            }
        });

        canvas.add_event_listener_with_callback("pointerdown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Mouse move cursor styling & CRT magnet tracking listener
    {
        let state = Rc::clone(&state);
        let canvas_clone = canvas.clone();
        let closure = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
            let rect = canvas_clone.get_bounding_client_rect();
            let rect_width = rect.width();
            let rect_height = rect.height();
            if rect_width <= 0.0 || rect_height <= 0.0 {
                return;
            }

            let client_x = event.client_x() as f64 - rect.left();
            let client_y = event.client_y() as f64 - rect.top();

            let norm_x = (client_x / rect_width).clamp(0.0, 1.0) as f32;
            let norm_y = (client_y / rect_height).clamp(0.0, 1.0) as f32;

            let mut s = state.borrow_mut();
            s.mouse_uv = (norm_x, norm_y);
            s.mouse_active = true;

            let (fb_w, fb_h) = s.app.system_mode.resolution();
            let fb_x = ((client_x / rect_width) * fb_w as f64).clamp(0.0, (fb_w - 1) as f64) as u16;
            let fb_y = ((client_y / rect_height) * fb_h as f64).clamp(0.0, (fb_h - 1) as f64) as u16;

            if s.app.update(InputEvent::PointerMove { x: fb_x, y: fb_y }) {
                s.dirty = true;
            }
            if s.app.visual_effects.magnet > 0.001 {
                s.dirty = true;
            }

            let _ = canvas_clone.style().set_property("cursor", "none");
        });

        canvas.add_event_listener_with_callback("pointermove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Pointer leave listener (magnet removed and cursor hidden when mouse leaves screen)
    {
        let state = Rc::clone(&state);
        let closure = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |_event: web_sys::MouseEvent| {
            let mut s = state.borrow_mut();
            s.mouse_active = false;
            if s.app.update(InputEvent::PointerLeave) {
                s.dirty = true;
            }
            if s.app.visual_effects.magnet > 0.001 {
                s.dirty = true;
            }
        });

        canvas.add_event_listener_with_callback("pointerleave", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Pointer enter listener
    {
        let state = Rc::clone(&state);
        let canvas_clone = canvas.clone();
        let closure = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |_event: web_sys::MouseEvent| {
            let mut s = state.borrow_mut();
            s.mouse_active = true;
            let _ = canvas_clone.style().set_property("cursor", "none");
            if s.app.visual_effects.magnet > 0.001 {
                s.dirty = true;
            }
        });

        canvas.add_event_listener_with_callback("pointerenter", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Mouse wheel event listener for vertical scrolling (Resume, Project list)
    {
        let state = Rc::clone(&state);
        let closure = Closure::<dyn FnMut(web_sys::WheelEvent)>::new(move |event: web_sys::WheelEvent| {
            event.prevent_default();
            let raw_dy = event.delta_y();
            let dy = if raw_dy.abs() < 1.0 {
                0
            } else if raw_dy > 0.0 {
                (raw_dy / 30.0).clamp(1.0, 4.0).round() as i16
            } else {
                -((raw_dy.abs() / 30.0).clamp(1.0, 4.0).round() as i16)
            };
            if dy != 0 {
                let mut s = state.borrow_mut();
                if s.app.update(InputEvent::Wheel { dx: 0, dy }) {
                    s.dirty = true;
                }
            }
        });

        canvas.add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Touch event listeners for mobile touchscreens (vertical drag-scrolling, horizontal tab swiping, tap interaction)
    {
        let state = Rc::clone(&state);
        let canvas_clone = canvas.clone();

        let touch_start_pos = Rc::new(RefCell::new(None::<(f64, f64, f64)>));
        let touch_last_y = Rc::new(RefCell::new(0.0f64));
        let touch_dragged = Rc::new(RefCell::new(false));

        // touchstart
        {
            let touch_start_pos = Rc::clone(&touch_start_pos);
            let touch_last_y = Rc::clone(&touch_last_y);
            let touch_dragged = Rc::clone(&touch_dragged);
            let canvas_clone = canvas_clone.clone();
            let state = Rc::clone(&state);

            let closure = Closure::<dyn FnMut(web_sys::TouchEvent)>::new(move |event: web_sys::TouchEvent| {
                if let Some(touch) = event.touches().get(0) {
                    let rect = canvas_clone.get_bounding_client_rect();
                    if rect.width() <= 0.0 || rect.height() <= 0.0 {
                        return;
                    }
                    let cx = touch.client_x() as f64;
                    let cy = touch.client_y() as f64;
                    let now = js_sys::Date::now();

                    *touch_start_pos.borrow_mut() = Some((cx, cy, now));
                    *touch_last_y.borrow_mut() = cy;
                    *touch_dragged.borrow_mut() = false;

                    let norm_x = ((cx - rect.left()) / rect.width()).clamp(0.0, 1.0) as f32;
                    let norm_y = ((cy - rect.top()) / rect.height()).clamp(0.0, 1.0) as f32;
                    let mut s = state.borrow_mut();
                    s.mouse_uv = (norm_x, norm_y);
                    s.mouse_active = true;
                    if s.app.visual_effects.magnet > 0.001 {
                        s.dirty = true;
                    }
                }
            });
            canvas.add_event_listener_with_callback("touchstart", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // touchmove
        {
            let touch_last_y = Rc::clone(&touch_last_y);
            let touch_dragged = Rc::clone(&touch_dragged);
            let canvas_clone = canvas_clone.clone();
            let state = Rc::clone(&state);

            let closure = Closure::<dyn FnMut(web_sys::TouchEvent)>::new(move |event: web_sys::TouchEvent| {
                if let Some(touch) = event.touches().get(0) {
                    let rect = canvas_clone.get_bounding_client_rect();
                    if rect.width() <= 0.0 || rect.height() <= 0.0 {
                        return;
                    }
                    let cx = touch.client_x() as f64;
                    let cy = touch.client_y() as f64;

                    let last_y = *touch_last_y.borrow();
                    let dy = cy - last_y;

                    // Smooth vertical drag-scrolling when finger moves vertically
                    if dy.abs() >= 12.0 {
                        *touch_dragged.borrow_mut() = true;
                        *touch_last_y.borrow_mut() = cy;

                        // Dragging finger DOWN (dy > 0) scrolls UP (dy = -1 in Wheel)
                        // Dragging finger UP (dy < 0) scrolls DOWN (dy = 1 in Wheel)
                        let scroll_step: i16 = if dy > 0.0 { -1 } else { 1 };
                        let mut s = state.borrow_mut();
                        if s.app.update(InputEvent::Wheel { dx: 0, dy: scroll_step }) {
                            s.dirty = true;
                        }
                    }

                    // Update pointer / magnet tracking
                    let norm_x = ((cx - rect.left()) / rect.width()).clamp(0.0, 1.0) as f32;
                    let norm_y = ((cy - rect.top()) / rect.height()).clamp(0.0, 1.0) as f32;
                    let mut s = state.borrow_mut();
                    s.mouse_uv = (norm_x, norm_y);

                    let (fb_w, fb_h) = s.app.system_mode.resolution();
                    let fb_x = (((cx - rect.left()) / rect.width()) * fb_w as f64).clamp(0.0, (fb_w - 1) as f64) as u16;
                    let fb_y = (((cy - rect.top()) / rect.height()) * fb_h as f64).clamp(0.0, (fb_h - 1) as f64) as u16;
                    if s.app.update(InputEvent::PointerMove { x: fb_x, y: fb_y }) {
                        s.dirty = true;
                    }

                    event.prevent_default();
                }
            });
            canvas.add_event_listener_with_callback("touchmove", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // touchend
        {
            let touch_start_pos = Rc::clone(&touch_start_pos);
            let touch_dragged = Rc::clone(&touch_dragged);
            let canvas_clone = canvas_clone.clone();
            let state = Rc::clone(&state);

            let closure = Closure::<dyn FnMut(web_sys::TouchEvent)>::new(move |event: web_sys::TouchEvent| {
                let start = touch_start_pos.borrow_mut().take();
                if let Some((start_x, start_y, start_time)) = start {
                    let now = js_sys::Date::now();
                    let duration = now - start_time;

                    let end_pos = event.changed_touches().get(0).map(|t| (t.client_x() as f64, t.client_y() as f64));
                    if let Some((end_x, end_y)) = end_pos {
                        let total_dx = end_x - start_x;
                        let total_dy = end_y - start_y;

                        // 1. Horizontal swipe gesture
                        if duration < 500.0 && total_dx.abs() > 40.0 && total_dx.abs() > total_dy.abs() * 1.5 {
                            let mut s = state.borrow_mut();
                            if total_dx < -40.0 {
                                // Swipe left: next tab
                                if s.app.update(InputEvent::KeyDown(Key::Tab)) {
                                    s.dirty = true;
                                }
                            } else {
                                // Swipe right: previous tab
                                let prev_key = match (s.app.platform, s.app.current_tab) {
                                    (_, pixel_ssh_core::Tab::Projects) => Key::Char('4'),
                                    (_, pixel_ssh_core::Tab::Resume) => Key::Char('1'),
                                    (_, pixel_ssh_core::Tab::About) => Key::Char('2'),
                                    (_, pixel_ssh_core::Tab::Contact) => Key::Char('3'),
                                    _ => Key::Char('1'),
                                };
                                if s.app.update(InputEvent::KeyDown(prev_key)) {
                                    s.dirty = true;
                                }
                            }
                            return;
                        }

                        // 2. Tap gesture (no drag, short duration)
                        if !*touch_dragged.borrow() && total_dx.abs() < 12.0 && total_dy.abs() < 12.0 && duration < 500.0 {
                            let rect = canvas_clone.get_bounding_client_rect();
                            if rect.width() > 0.0 && rect.height() > 0.0 {
                                let client_x = start_x - rect.left();
                                let client_y = start_y - rect.top();

                                let mut s = state.borrow_mut();
                                let (fb_w, fb_h) = s.app.system_mode.resolution();
                                let fb_x = ((client_x / rect.width()) * fb_w as f64).clamp(0.0, (fb_w - 1) as f64) as u16;
                                let fb_y = ((client_y / rect.height()) * fb_h as f64).clamp(0.0, (fb_h - 1) as f64) as u16;
                                let view = s.app.render();

                                if let Some(link) = view.link_at(fb_x, fb_y) {
                                    if !link.url.starts_with('#') {
                                        if let Some(w) = web_sys::window() {
                                            let _ = w.open_with_url_and_target(&link.url, "_blank");
                                        }
                                    }
                                }

                                if s.app.update(InputEvent::PointerDown {
                                    x: fb_x,
                                    y: fb_y,
                                    button: Button::Left,
                                }) {
                                    s.dirty = true;
                                }
                            }
                        }
                    }
                }

                let mut s = state.borrow_mut();
                s.mouse_active = false;
                if s.app.visual_effects.magnet > 0.001 {
                    s.dirty = true;
                }
            });
            canvas.add_event_listener_with_callback("touchend", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
    }

    // Custom CRT preset listener dispatched from cyberdeck UI
    {
        let state = Rc::clone(&state);
        let closure = Closure::<dyn FnMut(web_sys::CustomEvent)>::new(move |event: web_sys::CustomEvent| {
            if let Some(detail) = event.detail().as_string() {
                let mut s = state.borrow_mut();
                match detail.as_str() {
                    "clean" => {
                        s.app.visual_effects = pixel_ssh_view::VisualEffects::clean();
                        s.app.status = "Preset: Clean (Pixel-Perfect)".to_string();
                        s.dirty = true;
                    }
                    "crt" => {
                        s.app.visual_effects = pixel_ssh_view::VisualEffects::crt_trinitron();
                        s.app.status = "Preset: 80s Trinitron CRT".to_string();
                        s.dirty = true;
                    }
                    "glitch" => {
                        s.app.visual_effects = pixel_ssh_view::VisualEffects::retro_glitch();
                        s.app.status = "Preset: Retro Glitch".to_string();
                        s.dirty = true;
                    }
                    _ => {}
                }
            }
        });
        window.add_event_listener_with_callback("deck-preset", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Window resize listener
    {
        let state = Rc::clone(&state);
        let closure = Closure::<dyn FnMut(web_sys::UiEvent)>::new(move |_event: web_sys::UiEvent| {
            let mut s = state.borrow_mut();
            s.dirty = true;
        });
        window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Event-driven render loop: requestAnimationFrame checks dirty flag & advances marquee ticker
    {
        let state = Rc::clone(&state);
        let canvas_render = canvas.clone();
        let document_render = document.clone();
        let mut frame_count: u32 = 0;
        let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::<dyn FnMut()>::new(move || {
            frame_count = (frame_count + 1) % 6;
            {
                let mut state_guard = state.borrow_mut();
                let s = &mut *state_guard;

                // Periodic marquee ticker for horizontal auto-scrolling & live clock
                if frame_count == 0 {
                    s.app.tick = s.app.tick.wrapping_add(1);
                    let date = js_sys::Date::new_0();
                    s.app.set_time(date.get_hours() as u8, date.get_minutes() as u8, date.get_seconds() as u8);
                    s.dirty = true;
                }

                // Animate jitter, magnetic flux oscillation, RF white noise, antenna hum, or screensaver dynamically if active
                if s.app.visual_effects.jitter > 0.001
                    || (s.app.visual_effects.magnet > 0.001 && s.mouse_active)
                    || s.app.visual_effects.noise > 0.001
                    || s.app.visual_effects.antenna_hum > 0.001
                    || s.app.screensaver_active
                {
                    s.dirty = true;
                }

                if s.dirty {
                    let view = s.app.render();
                    let (target_w, target_h) = view.system_mode.resolution();
                    let scale: u32 = if target_w <= 320 { 4 } else { 2 };
                    let draw_w = target_w as u32 * scale;
                    let draw_h = target_h as u32 * scale;
                    if canvas_render.width() != draw_w || canvas_render.height() != draw_h {
                        canvas_render.set_width(draw_w);
                        canvas_render.set_height(draw_h);
                    }
                    s.framebuffer.draw_view(&view);

                    let time = ((js_sys::Date::now() - s.start_time_ms) / 1000.0) as f32;

                    s.renderer.render_frame_with_effects(
                        &s.framebuffer,
                        &s.app.visual_effects,
                        time,
                        s.mouse_uv,
                        s.mouse_active,
                    );

                    // Sync Cyberdeck active state attributes
                    if let Some(deck) = document_render.get_element_by_id("cyberdeck") {
                        let tab_idx = match s.app.current_tab {
                            pixel_ssh_core::Tab::Projects => 1,
                            pixel_ssh_core::Tab::Resume => 2,
                            pixel_ssh_core::Tab::About => 3,
                            pixel_ssh_core::Tab::Contact => 4,
                            _ => 1,
                        };
                        let _ = deck.set_attribute("data-tab", &tab_idx.to_string());
                        let modal_str = match s.app.active_modal {
                            pixel_ssh_view::ActiveModal::None => "none",
                            pixel_ssh_view::ActiveModal::Visuals => "visuals",
                            pixel_ssh_view::ActiveModal::Help => "help",
                            pixel_ssh_view::ActiveModal::System => "system",
                        };
                        let _ = deck.set_attribute("data-modal", modal_str);
                        let _ = deck.set_attribute("data-mode", s.app.system_mode.short_name());
                    }
                    if let Some(indicator) = document_render.get_element_by_id("deck-mode-indicator") {
                        indicator.set_text_content(Some(&format!("SYS: {}", s.app.system_mode.short_name())));
                    }

                    s.dirty = false;
                }
            }

            // Schedule next frame check
            let window = web_sys::window().unwrap();
            let _ = window.request_animation_frame(
                f.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
            );
        }));

        let _ = window.request_animation_frame(
            g.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
        );
    }

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn console_error_panic_hook_init() {
    std::panic::set_hook(Box::new(|info| {
        web_sys::console::error_1(&JsValue::from_str(&info.to_string()));
    }));
}
