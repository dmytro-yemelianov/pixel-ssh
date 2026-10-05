#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code, unused_imports))]

use std::cell::RefCell;
use std::rc::Rc;

use pixel_ssh_core::App;
use pixel_ssh_framebuffer::Framebuffer;
use pixel_ssh_render_web::WebGlRenderer;
use pixel_ssh_view::{Button, InputEvent, Key, ResolutionMode};

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

fn portrait_view_height(mode: ResolutionMode, css_width: i32, css_height: i32) -> Option<u16> {
    if css_width <= 0 || css_height <= css_width {
        return None;
    }
    let (native_width, native_height) = mode.resolution();
    let row_height = mode.line_height() as u32;
    let proportional = native_width as u32 * css_height as u32 / css_width as u32;
    let rounded = ((proportional + row_height / 2) / row_height) * row_height;
    Some(rounded.clamp(native_height as u32, 4096) as u16)
}

#[cfg(test)]
mod viewport_tests {
    use super::*;

    #[test]
    fn portrait_height_tracks_display_width_and_rotation() {
        assert_eq!(
            portrait_view_height(ResolutionMode::C64, 390, 844),
            Some(696)
        );
        assert_eq!(
            portrait_view_height(ResolutionMode::ZxSpectrum, 390, 844),
            Some(552)
        );
        assert_eq!(portrait_view_height(ResolutionMode::C64, 844, 390), None);
    }
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

    let win_w = window
        .inner_width()
        .ok()
        .and_then(|w| w.as_f64())
        .unwrap_or(1280.0);
    let win_h = window
        .inner_height()
        .ok()
        .and_then(|h| h.as_f64())
        .unwrap_or(800.0);
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
            app.active_modal = pixel_ssh_view::ActiveModal::Visuals;
        } else if search.contains("tab=contact") {
            app.current_tab = pixel_ssh_core::Tab::Contact;
        } else if search.contains("tab=help") {
            app.active_modal = pixel_ssh_view::ActiveModal::Help;
        }
        if search.contains("detail=1") {
            app.show_detail = true;
        }
        let params: Vec<(&str, &str)> = search
            .trim_start_matches('?')
            .split('&')
            .filter_map(|part| part.split_once('='))
            .collect();
        for (key, value) in params {
            if ["system", "mode", "resolution"].contains(&key) {
                let resolution = match value {
                    "svga" => Some(pixel_ssh_view::ResolutionMode::Svga),
                    "sga" => Some(pixel_ssh_view::ResolutionMode::Sga),
                    "vga" => Some(pixel_ssh_view::ResolutionMode::Vga),
                    "ega" => Some(pixel_ssh_view::ResolutionMode::Ega),
                    "cga" => Some(pixel_ssh_view::ResolutionMode::Cga),
                    "c64" => Some(pixel_ssh_view::ResolutionMode::C64),
                    "atari" => Some(pixel_ssh_view::ResolutionMode::Atari),
                    "zx" => Some(pixel_ssh_view::ResolutionMode::ZxSpectrum),
                    _ => None,
                };
                if let Some(resolution) = resolution {
                    app.set_resolution(resolution);
                    has_explicit_system = true;
                }
            }
        }
        if search.contains("modal=visuals") {
            app.active_modal = pixel_ssh_view::ActiveModal::Visuals;
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
            let num_str: String = scroll_str
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
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
            let num_str: String = m_str
                .chars()
                .take_while(|c| c.is_ascii_digit() || *c == '.')
                .collect();
            if let Ok(m) = num_str.parse::<f32>() {
                app.visual_effects.magnet = m.clamp(0.0, 1.0);
            }
        }
        if let Some(n_str) = search.split("noise=").nth(1) {
            let num_str: String = n_str
                .chars()
                .take_while(|c| c.is_ascii_digit() || *c == '.')
                .collect();
            if let Ok(n) = num_str.parse::<f32>() {
                app.visual_effects.noise = n.clamp(0.0, 1.0);
            }
        }
        if let Some(h_str) = search.split("hum=").nth(1) {
            let num_str: String = h_str
                .chars()
                .take_while(|c| c.is_ascii_digit() || *c == '.')
                .collect();
            if let Ok(h) = num_str.parse::<f32>() {
                app.visual_effects.antenna_hum = h.clamp(0.0, 1.0);
            }
        }
        if let Some(t_str) = search.split("time=").nth(1) {
            let num_str: String = t_str
                .chars()
                .take_while(|c| c.is_ascii_digit() || *c == '.')
                .collect();
            if let Ok(t) = num_str.parse::<f32>() {
                init_time_offset = t;
            }
        }
        if let Some(mx_str) = search.split("mouse_x=").nth(1) {
            let num_str: String = mx_str
                .chars()
                .take_while(|c| c.is_ascii_digit() || *c == '.')
                .collect();
            if let Ok(mx) = num_str.parse::<f32>() {
                init_mouse_uv.0 = mx.clamp(0.0, 1.0);
                init_mouse_active = true;
            }
        }
        if let Some(my_str) = search.split("mouse_y=").nth(1) {
            let num_str: String = my_str
                .chars()
                .take_while(|c| c.is_ascii_digit() || *c == '.')
                .collect();
            if let Ok(my) = num_str.parse::<f32>() {
                init_mouse_uv.1 = my.clamp(0.0, 1.0);
                init_mouse_active = true;
            }
        }
    }

    // In portrait, use a readable 40-column grid without changing palette or theme.
    if is_portrait && !has_explicit_system {
        app.set_resolution(pixel_ssh_view::ResolutionMode::C64);
    }

    app.set_web_height(portrait_view_height(
        app.resolution,
        canvas.client_width(),
        canvas.client_height(),
    ));
    if init_mouse_active {
        let (fb_w, fb_h) = app.view_dimensions();
        let fb_x = ((init_mouse_uv.0 * fb_w as f32).round() as u16).min(fb_w.saturating_sub(1));
        let fb_y = ((init_mouse_uv.1 * fb_h as f32).round() as u16).min(fb_h.saturating_sub(1));
        app.mouse_pos = Some((fb_x, fb_y));
    }

    let (init_w, init_h) = app.view_dimensions();
    let scale: u32 = if init_w <= 320 && init_h <= 400 { 4 } else { 2 };
    let draw_w = init_w as u32 * scale;
    let draw_h = init_h as u32 * scale;
    canvas.set_width(draw_w);
    canvas.set_height(draw_h);

    let renderer =
        WebGlRenderer::new(canvas.clone(), init_w, init_h).map_err(|e| JsValue::from_str(&e))?;
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
        s.renderer.render_frame_with_effects(
            &s.framebuffer,
            &s.app.visual_effects,
            init_time,
            s.mouse_uv,
            s.mouse_active,
            true,
        );
        s.dirty = false;
    }

    // Keyboard event listener
    {
        let state = Rc::clone(&state);
        let closure = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(
            move |event: web_sys::KeyboardEvent| {
                let key = event.key();
                let parsed_key = match key.as_str() {
                    "ArrowUp" => Some(Key::Up),
                    "ArrowDown" => Some(Key::Down),
                    "ArrowLeft" => Some(Key::Left),
                    "ArrowRight" => Some(Key::Right),
                    "PageUp" => Some(Key::PageUp),
                    "PageDown" => Some(Key::PageDown),
                    "Home" => Some(Key::Home),
                    "End" => Some(Key::End),
                    "Enter" => Some(Key::Enter),
                    "Escape" => Some(Key::Escape),
                    "Tab" => {
                        event.prevent_default();
                        Some(Key::Tab)
                    }
                    "Backspace" => Some(Key::Backspace),
                    "F1" => {
                        event.prevent_default();
                        Some(Key::F(1))
                    }
                    "F2" => {
                        event.prevent_default();
                        Some(Key::F(2))
                    }
                    "F3" => {
                        event.prevent_default();
                        Some(Key::F(3))
                    }
                    "F4" => {
                        event.prevent_default();
                        Some(Key::F(4))
                    }
                    "F5" => {
                        event.prevent_default();
                        Some(Key::F(5))
                    }
                    "F6" => {
                        event.prevent_default();
                        Some(Key::F(6))
                    }
                    "F7" => {
                        event.prevent_default();
                        Some(Key::F(7))
                    }
                    "F8" => {
                        event.prevent_default();
                        Some(Key::F(8))
                    }
                    "F9" => {
                        event.prevent_default();
                        Some(Key::F(9))
                    }
                    "F10" => {
                        event.prevent_default();
                        Some(Key::F(10))
                    }
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
            },
        );

        window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Mouse / Pointer click listener for interactive links, tabs, and project rows
    {
        let state = Rc::clone(&state);
        let canvas_clone = canvas.clone();
        let closure = Closure::<dyn FnMut(web_sys::PointerEvent)>::new(
            move |event: web_sys::PointerEvent| {
                // Touch taps are dispatched on touchend below, after swipe detection.
                if event.pointer_type() == "touch" {
                    return;
                }
                let rect = canvas_clone.get_bounding_client_rect();
                let rect_width = rect.width();
                let rect_height = rect.height();
                if rect_width <= 0.0 || rect_height <= 0.0 {
                    return;
                }

                let client_x = event.client_x() as f64 - rect.left();
                let client_y = event.client_y() as f64 - rect.top();

                let mut s = state.borrow_mut();
                let (fb_w, fb_h) = s.app.view_dimensions();
                let fb_x =
                    ((client_x / rect_width) * fb_w as f64).clamp(0.0, (fb_w - 1) as f64) as u16;
                let fb_y =
                    ((client_y / rect_height) * fb_h as f64).clamp(0.0, (fb_h - 1) as f64) as u16;
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
            },
        );

        canvas.add_event_listener_with_callback("pointerdown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Mouse move cursor styling & CRT magnet tracking listener
    {
        let state = Rc::clone(&state);
        let canvas_clone = canvas.clone();
        let closure =
            Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
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

                let (fb_w, fb_h) = s.app.view_dimensions();
                let fb_x =
                    ((client_x / rect_width) * fb_w as f64).clamp(0.0, (fb_w - 1) as f64) as u16;
                let fb_y =
                    ((client_y / rect_height) * fb_h as f64).clamp(0.0, (fb_h - 1) as f64) as u16;

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
        let closure =
            Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |_event: web_sys::MouseEvent| {
                let mut s = state.borrow_mut();
                s.mouse_active = false;
                if s.app.update(InputEvent::PointerLeave) {
                    s.dirty = true;
                }
                if s.app.visual_effects.magnet > 0.001 {
                    s.dirty = true;
                }
            });

        canvas
            .add_event_listener_with_callback("pointerleave", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Pointer enter listener
    {
        let state = Rc::clone(&state);
        let canvas_clone = canvas.clone();
        let closure =
            Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |_event: web_sys::MouseEvent| {
                let mut s = state.borrow_mut();
                s.mouse_active = true;
                let _ = canvas_clone.style().set_property("cursor", "none");
                if s.app.visual_effects.magnet > 0.001 {
                    s.dirty = true;
                }
            });

        canvas
            .add_event_listener_with_callback("pointerenter", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Mouse wheel event listener for vertical scrolling (Resume, Project list)
    {
        let state = Rc::clone(&state);
        let closure =
            Closure::<dyn FnMut(web_sys::WheelEvent)>::new(move |event: web_sys::WheelEvent| {
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

            let closure = Closure::<dyn FnMut(web_sys::TouchEvent)>::new(
                move |event: web_sys::TouchEvent| {
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
                },
            );
            canvas
                .add_event_listener_with_callback("touchstart", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // touchmove
        {
            let touch_last_y = Rc::clone(&touch_last_y);
            let touch_dragged = Rc::clone(&touch_dragged);
            let canvas_clone = canvas_clone.clone();
            let state = Rc::clone(&state);

            let closure = Closure::<dyn FnMut(web_sys::TouchEvent)>::new(
                move |event: web_sys::TouchEvent| {
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

                            // Content follows the finger on every page: dragging up
                            // (dy < 0) sends TouchDrag +1, dragging down sends -1.
                            let scroll_step: i16 = if dy > 0.0 { -1 } else { 1 };
                            let mut s = state.borrow_mut();
                            if s.app.update(InputEvent::TouchDrag { dy: scroll_step }) {
                                s.dirty = true;
                            }
                        }

                        // Update pointer / magnet tracking
                        let norm_x = ((cx - rect.left()) / rect.width()).clamp(0.0, 1.0) as f32;
                        let norm_y = ((cy - rect.top()) / rect.height()).clamp(0.0, 1.0) as f32;
                        let mut s = state.borrow_mut();
                        s.mouse_uv = (norm_x, norm_y);

                        let (fb_w, fb_h) = s.app.view_dimensions();
                        let fb_x = (((cx - rect.left()) / rect.width()) * fb_w as f64)
                            .clamp(0.0, (fb_w - 1) as f64)
                            as u16;
                        let fb_y = (((cy - rect.top()) / rect.height()) * fb_h as f64)
                            .clamp(0.0, (fb_h - 1) as f64)
                            as u16;
                        if s.app.update(InputEvent::PointerMove { x: fb_x, y: fb_y }) {
                            s.dirty = true;
                        }

                        event.prevent_default();
                    }
                },
            );
            canvas
                .add_event_listener_with_callback("touchmove", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // touchend
        {
            let touch_start_pos = Rc::clone(&touch_start_pos);
            let touch_dragged = Rc::clone(&touch_dragged);
            let canvas_clone = canvas_clone.clone();
            let state = Rc::clone(&state);

            let closure = Closure::<dyn FnMut(web_sys::TouchEvent)>::new(
                move |event: web_sys::TouchEvent| {
                    let start = touch_start_pos.borrow_mut().take();
                    if let Some((start_x, start_y, start_time)) = start {
                        let now = js_sys::Date::now();
                        let duration = now - start_time;

                        let end_pos = event
                            .changed_touches()
                            .get(0)
                            .map(|t| (t.client_x() as f64, t.client_y() as f64));
                        if let Some((end_x, end_y)) = end_pos {
                            let total_dx = end_x - start_x;
                            let total_dy = end_y - start_y;

                            // 1. Horizontal swipe gesture
                            if duration < 500.0
                                && total_dx.abs() > 40.0
                                && total_dx.abs() > total_dy.abs() * 1.5
                            {
                                let mut s = state.borrow_mut();
                                let direction = if total_dx < 0.0 {
                                    Key::Right
                                } else {
                                    Key::Left
                                };
                                if s.app.update(InputEvent::KeyDown(direction)) {
                                    s.dirty = true;
                                }
                                return;
                            }

                            // 2. Tap gesture (no drag, short duration)
                            if !*touch_dragged.borrow()
                                && total_dx.abs() < 12.0
                                && total_dy.abs() < 12.0
                                && duration < 500.0
                            {
                                let rect = canvas_clone.get_bounding_client_rect();
                                if rect.width() > 0.0 && rect.height() > 0.0 {
                                    let client_x = start_x - rect.left();
                                    let client_y = start_y - rect.top();

                                    let mut s = state.borrow_mut();
                                    let (fb_w, fb_h) = s.app.view_dimensions();
                                    let fb_x = ((client_x / rect.width()) * fb_w as f64)
                                        .clamp(0.0, (fb_w - 1) as f64)
                                        as u16;
                                    let fb_y = ((client_y / rect.height()) * fb_h as f64)
                                        .clamp(0.0, (fb_h - 1) as f64)
                                        as u16;
                                    let view = s.app.render();

                                    if let Some(link) = view.link_at(fb_x, fb_y) {
                                        if !link.url.starts_with('#') {
                                            if let Some(w) = web_sys::window() {
                                                let _ =
                                                    w.open_with_url_and_target(&link.url, "_blank");
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
                },
            );
            canvas
                .add_event_listener_with_callback("touchend", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
    }

    // Window resize listener
    {
        let state = Rc::clone(&state);
        let closure =
            Closure::<dyn FnMut(web_sys::UiEvent)>::new(move |_event: web_sys::UiEvent| {
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
                    s.app.tick();
                    let date = js_sys::Date::new_0();
                    s.app.set_time(
                        date.get_hours() as u8,
                        date.get_minutes() as u8,
                        date.get_seconds() as u8,
                    );
                    s.dirty = true;
                }

                // Animate jitter, magnetic flux oscillation, RF white noise, antenna hum, or screensaver dynamically if active
                let shader_animated = s.app.visual_effects.jitter > 0.001
                    || (s.app.visual_effects.magnet > 0.001 && s.mouse_active)
                    || s.app.visual_effects.noise > 0.001
                    || s.app.visual_effects.antenna_hum > 0.001;

                if s.dirty || shader_animated {
                    let content_dirty = s.dirty;
                    if content_dirty {
                        s.app.set_web_height(portrait_view_height(
                            s.app.resolution,
                            canvas_render.client_width(),
                            canvas_render.client_height(),
                        ));
                        let view = s.app.render();
                        let (target_w, target_h) = (view.width, view.height);
                        let scale: u32 = if target_w <= 320 && target_h <= 400 {
                            4
                        } else {
                            2
                        };
                        let draw_w = target_w as u32 * scale;
                        let draw_h = target_h as u32 * scale;
                        if canvas_render.width() != draw_w || canvas_render.height() != draw_h {
                            canvas_render.set_width(draw_w);
                            canvas_render.set_height(draw_h);
                        }
                        s.framebuffer.draw_view(&view);
                    }

                    let time = ((js_sys::Date::now() - s.start_time_ms) / 1000.0) as f32;

                    s.renderer.render_frame_with_effects(
                        &s.framebuffer,
                        &s.app.visual_effects,
                        time,
                        s.mouse_uv,
                        s.mouse_active,
                        content_dirty,
                    );

                    s.dirty = false;
                }
            }

            // Schedule next frame check
            let window = web_sys::window().unwrap();
            let _ = window
                .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref());
        }));

        let _ =
            window.request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref());
    }

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn console_error_panic_hook_init() {
    std::panic::set_hook(Box::new(|info| {
        web_sys::console::error_1(&JsValue::from_str(&info.to_string()));
    }));
}
