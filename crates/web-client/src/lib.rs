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

    let mut app = App::new();

    if let Ok(search) = window.location().search() {
        if search.contains("tab=resume") {
            app.current_tab = pixel_ssh_core::Tab::Resume;
        } else if search.contains("tab=contact") {
            app.current_tab = pixel_ssh_core::Tab::Contact;
        } else if search.contains("tab=help") {
            app.current_tab = pixel_ssh_core::Tab::Help;
        }
        if search.contains("detail=1") {
            app.show_detail = true;
        }
        if search.contains("palette=zx") {
            app.palette_mode = pixel_ssh_view::PaletteMode::ZxSpectrum;
        } else if search.contains("palette=c64") {
            app.palette_mode = pixel_ssh_view::PaletteMode::C64;
        } else if search.contains("palette=atari") {
            app.palette_mode = pixel_ssh_view::PaletteMode::Atari;
        } else if search.contains("palette=amber") {
            app.palette_mode = pixel_ssh_view::PaletteMode::Amber;
        } else if search.contains("palette=green") {
            app.palette_mode = pixel_ssh_view::PaletteMode::GreenCrt;
        }
    }

    let (init_w, init_h) = app.palette_mode.resolution();
    let (ar_w, ar_h) = app.palette_mode.aspect_ratio();
    canvas.set_width(init_w as u32);
    canvas.set_height(init_h as u32);
    let _ = canvas.style().set_property("aspect-ratio", &format!("{}/{}", ar_w, ar_h));
    let _ = canvas.style().set_property("width", &format!("min(100vw, calc(100vh * {} / {}))", ar_w, ar_h));
    let _ = canvas.style().set_property("height", &format!("min(100vh, calc(100vw * {} / {}))", ar_h, ar_w));

    let renderer = WebGlRenderer::new(canvas.clone(), init_w, init_h)
        .map_err(|e| JsValue::from_str(&e))?;
    let framebuffer = Framebuffer::new(init_w, init_h);

    let state = Rc::new(RefCell::new(ClientState {
        app,
        framebuffer,
        renderer,
        dirty: true,
    }));

    // Initial render
    {
        let mut state_guard = state.borrow_mut();
        let s = &mut *state_guard;
        let view = s.app.render();
        s.framebuffer.draw_view(&view);
        s.renderer.render_frame(&s.framebuffer);
        s.dirty = false;
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

            let canvas_w = canvas_clone.width() as f64;
            let canvas_h = canvas_clone.height() as f64;
            let fb_x = ((client_x / rect_width) * canvas_w).clamp(0.0, canvas_w - 1.0) as u16;
            let fb_y = ((client_y / rect_height) * canvas_h).clamp(0.0, canvas_h - 1.0) as u16;

            let mut s = state.borrow_mut();
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

    // Mouse move cursor styling listener (shows pointer on hover over links/tabs)
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

            let canvas_w = canvas_clone.width() as f64;
            let canvas_h = canvas_clone.height() as f64;
            let fb_x = ((client_x / rect_width) * canvas_w).clamp(0.0, canvas_w - 1.0) as u16;
            let fb_y = ((client_y / rect_height) * canvas_h).clamp(0.0, canvas_h - 1.0) as u16;

            let s = state.borrow();
            let view = s.app.render();

            let (cols, _rows) = s.app.palette_mode.char_grid();
            let tab_y_range = if cols == 80 { 24..=48 } else { 10..=22 };
            let is_interactive = view.link_at(fb_x, fb_y).is_some()
                || tab_y_range.contains(&fb_y)
                || (!s.app.show_detail && fb_y >= (if cols == 80 { 56 } else { 24 }));

            let cursor_style = if is_interactive { "pointer" } else { "default" };
            let _ = canvas_clone.style().set_property("cursor", cursor_style);
        });

        canvas.add_event_listener_with_callback("pointermove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Mouse wheel event listener for vertical scrolling (Resume, Project list)
    {
        let state = Rc::clone(&state);
        let closure = Closure::<dyn FnMut(web_sys::WheelEvent)>::new(move |event: web_sys::WheelEvent| {
            event.prevent_default();
            let dy = event.delta_y() as i16;
            let mut s = state.borrow_mut();
            if s.app.update(InputEvent::Wheel { dx: 0, dy }) {
                s.dirty = true;
            }
        });

        canvas.add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())?;
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

                // Periodic marquee ticker for horizontal auto-scrolling
                if frame_count == 0 {
                    s.app.tick = s.app.tick.wrapping_add(1);
                    s.dirty = true;
                }

                if s.dirty {
                    let view = s.app.render();
                    let (target_w, target_h) = view.palette_mode.resolution();
                    if canvas_render.width() != target_w as u32 || canvas_render.height() != target_h as u32 {
                        canvas_render.set_width(target_w as u32);
                        canvas_render.set_height(target_h as u32);
                        let (ar_w, ar_h) = view.palette_mode.aspect_ratio();
                        let _ = canvas_render.style().set_property("aspect-ratio", &format!("{}/{}", ar_w, ar_h));
                        let _ = canvas_render.style().set_property("width", &format!("min(100vw, calc(100vh * {} / {}))", ar_w, ar_h));
                        let _ = canvas_render.style().set_property("height", &format!("min(100vh, calc(100vw * {} / {}))", ar_h, ar_w));
                    }
                    s.framebuffer.draw_view(&view);
                    s.renderer.render_frame(&s.framebuffer);
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
