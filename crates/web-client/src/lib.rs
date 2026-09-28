#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code, unused_imports))]

use std::cell::RefCell;
use std::rc::Rc;

use pixel_ssh_core::App;
use pixel_ssh_framebuffer::{Framebuffer, DEFAULT_HEIGHT, DEFAULT_WIDTH};
use pixel_ssh_render_web::WebGlRenderer;
use pixel_ssh_view::{InputEvent, Key};

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

    canvas.set_width(DEFAULT_WIDTH as u32);
    canvas.set_height(DEFAULT_HEIGHT as u32);

    let renderer = WebGlRenderer::new(canvas.clone(), DEFAULT_WIDTH, DEFAULT_HEIGHT)
        .map_err(|e| JsValue::from_str(&e))?;
    let framebuffer = Framebuffer::new(DEFAULT_WIDTH, DEFAULT_HEIGHT);
    let app = App::new();

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

    // Event-driven render loop: requestAnimationFrame checks dirty flag
    {
        let state = Rc::clone(&state);
        let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::<dyn FnMut()>::new(move || {
            {
                let mut state_guard = state.borrow_mut();
                let s = &mut *state_guard;
                if s.dirty {
                    let view = s.app.render();
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
