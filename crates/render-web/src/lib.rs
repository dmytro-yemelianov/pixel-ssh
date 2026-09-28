use pixel_ssh_framebuffer::Framebuffer;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlTexture};

pub struct WebGlRenderer {
    #[cfg(target_arch = "wasm32")]
    gl: WebGl2RenderingContext,
    #[cfg(target_arch = "wasm32")]
    program: WebGlProgram,
    #[cfg(target_arch = "wasm32")]
    texture: WebGlTexture,
    #[cfg(target_arch = "wasm32")]
    #[allow(dead_code)]
    quad_buffer: WebGlBuffer,
    pub width: u16,
    pub height: u16,
}

#[cfg(target_arch = "wasm32")]
const VERTEX_SHADER_SRC: &str = r#"#version 300 es
layout(location = 0) in vec2 a_pos;
out vec2 v_uv;

void main() {
    v_uv = (a_pos + 1.0) * 0.5;
    v_uv.y = 1.0 - v_uv.y; // Flip Y for WebGL texture orientation
    gl_Position = vec4(a_pos, 0.0, 1.0);
}
"#;

#[cfg(target_arch = "wasm32")]
const FRAGMENT_SHADER_SRC: &str = r#"#version 300 es
precision mediump float;
in vec2 v_uv;
uniform sampler2D u_framebuffer;
uniform vec4 u_palette[256];
uniform vec2 u_resolution;
out vec4 outColor;

void main() {
    float r = texture(u_framebuffer, v_uv).r;
    int index = int(r * 255.0 + 0.5);
    vec4 baseColor = u_palette[index];

    // Subtle pixel grid separation (retro CRT / dot-matrix phosphor mask)
    vec2 grid = fract(v_uv * u_resolution);
    float edge_dist = min(min(grid.x, 1.0 - grid.x), min(grid.y, 1.0 - grid.y));
    float border = smoothstep(0.0, 0.12, edge_dist);
    float dim = 0.84 + 0.16 * border; // 16% dimming at pixel outer seam

    outColor = vec4(baseColor.rgb * dim, baseColor.a);
}
"#;

impl WebGlRenderer {
    #[cfg(target_arch = "wasm32")]
    pub fn new(canvas: HtmlCanvasElement, width: u16, height: u16) -> Result<Self, String> {
        let gl = canvas
            .get_context("webgl2")
            .map_err(|e| format!("Failed to get WebGL2 context: {:?}", e))?
            .ok_or_else(|| "WebGL2 not supported".to_string())?
            .dyn_into::<WebGl2RenderingContext>()
            .map_err(|_| "Failed to cast to WebGl2RenderingContext".to_string())?;

        let vert_shader = Self::compile_shader(&gl, WebGl2RenderingContext::VERTEX_SHADER, VERTEX_SHADER_SRC)?;
        let frag_shader = Self::compile_shader(&gl, WebGl2RenderingContext::FRAGMENT_SHADER, FRAGMENT_SHADER_SRC)?;

        let program = gl.create_program().ok_or_else(|| "Failed to create program".to_string())?;
        gl.attach_shader(&program, &vert_shader);
        gl.attach_shader(&program, &frag_shader);
        gl.link_program(&program);

        if !gl.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS).as_bool().unwrap_or(false) {
            let log = gl.get_program_info_log(&program).unwrap_or_default();
            return Err(format!("WebGL link error: {}", log));
        }

        gl.use_program(Some(&program));

        // Create Fullscreen Quad Buffer [-1, -1] to [1, 1]
        let quad_vertices: [f32; 8] = [
            -1.0, -1.0,
             1.0, -1.0,
            -1.0,  1.0,
             1.0,  1.0,
        ];
        let quad_buffer = gl.create_buffer().ok_or_else(|| "Failed to create VBO".to_string())?;
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&quad_buffer));
        unsafe {
            let view = js_sys::Float32Array::view(&quad_vertices);
            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_with_i32(0, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);

        // Create R8 indexed texture
        let texture = gl.create_texture().ok_or_else(|| "Failed to create texture".to_string())?;
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::NEAREST as i32);
        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::NEAREST as i32);
        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);

        // Allocate empty R8 texture
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::R8 as i32,
            width as i32,
            height as i32,
            0,
            WebGl2RenderingContext::RED,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            None,
        ).map_err(|e| format!("Failed to allocate texture: {:?}", e))?;

        let sampler_loc = gl.get_uniform_location(&program, "u_framebuffer");
        gl.uniform1i(sampler_loc.as_ref(), 0);

        Ok(Self {
            gl,
            program,
            texture,
            quad_buffer,
            width,
            height,
        })
    }

    #[cfg(target_arch = "wasm32")]
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&self.texture));
        let _ = self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::R8 as i32,
            width as i32,
            height as i32,
            0,
            WebGl2RenderingContext::RED,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            None,
        );
        self.gl.viewport(0, 0, width as i32, height as i32);
    }

    #[cfg(target_arch = "wasm32")]
    fn compile_shader(gl: &WebGl2RenderingContext, shader_type: u32, source: &str) -> Result<web_sys::WebGlShader, String> {
        let shader = gl.create_shader(shader_type).ok_or_else(|| "Failed to create shader".to_string())?;
        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);

        if !gl.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false) {
            let log = gl.get_shader_info_log(&shader).unwrap_or_default();
            return Err(format!("Shader compile error: {}", log));
        }

        Ok(shader)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn render_frame(&mut self, fb: &Framebuffer) {
        if fb.width != self.width || fb.height != self.height {
            self.resize(fb.width, fb.height);
        }
        self.gl.use_program(Some(&self.program));

        // Upload palette uniform array
        let mut palette_flat = [0.0f32; 256 * 4];
        for i in 0..256 {
            let c = fb.palette[i];
            palette_flat[i * 4 + 0] = (c[0] as f32) / 255.0;
            palette_flat[i * 4 + 1] = (c[1] as f32) / 255.0;
            palette_flat[i * 4 + 2] = (c[2] as f32) / 255.0;
            palette_flat[i * 4 + 3] = (c[3] as f32) / 255.0;
        }

        let pal_loc = self.gl.get_uniform_location(&self.program, "u_palette");
        self.gl.uniform4fv_with_f32_array(pal_loc.as_ref(), &palette_flat);

        let res_loc = self.gl.get_uniform_location(&self.program, "u_resolution");
        self.gl.uniform2f(res_loc.as_ref(), fb.width as f32, fb.height as f32);

        // Upload 8-bit indexed pixel buffer to R8 texture
        self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&self.texture));
        unsafe {
            let view = js_sys::Uint8Array::view(&fb.pixels);
            let _ = self.gl.tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_array_buffer_view(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                0,
                0,
                fb.width as i32,
                fb.height as i32,
                WebGl2RenderingContext::RED,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                Some(&view),
            );
        }

        // Draw fullscreen quad
        self.gl.draw_arrays(WebGl2RenderingContext::TRIANGLE_STRIP, 0, 4);
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_mock(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn render_frame(&mut self, _fb: &Framebuffer) {}
}
