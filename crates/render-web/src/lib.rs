use pixel_ssh_framebuffer::Framebuffer;
use pixel_ssh_view::VisualEffects;

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
precision highp float;
in vec2 v_uv;
uniform sampler2D u_framebuffer;
uniform vec4 u_palette[256];
uniform vec2 u_resolution;

uniform float u_scanlines;
uniform float u_pixel_grid;
uniform float u_chromatic;
uniform float u_afterglow;
uniform float u_curvature;
uniform float u_jitter;
uniform float u_time;

uniform float u_magnet;
uniform vec2 u_mouse;
uniform float u_mouse_active;

uniform float u_noise;
uniform float u_antenna_hum;
uniform int u_monochrome_mode;

out vec4 outColor;

// Authentic non-trigonometric 2D hash for 60 FPS decorrelated TV static (Dave Hoskins hash12)
float hash12(vec2 p) {
    vec3 p3 = fract(vec3(p.xyx) * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

void main() {
    vec2 uv = v_uv;

    // 1. Barrel / CRT Curvature with subtle dynamic flyback anode breathing
    if (u_curvature > 0.001) {
        float anode_breath = 1.0 + 0.0035 * (sin(u_time * 1.7) * cos(u_time * 0.43));
        vec2 cc = (uv * 2.0 - 1.0) * anode_breath;
        vec2 offset = cc.yx / 3.0;
        uv = cc + cc * offset * offset * (u_curvature * 0.28);
        uv = (uv + 1.0) * 0.5;

        // Cut off outside CRT bounds (bezel border)
        if (uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0) {
            outColor = vec4(0.02, 0.02, 0.03, 1.0);
            return;
        }
    }

    // 2. Sporadic Antenna Hum & RF Interference (Real-life AC mains ripple + stochastic burst sync tear)
    float is_antenna_burst = 0.0;
    float hum_waveform = 0.0;
    if (u_antenna_hum > 0.001) {
        // Slow vertical rolling 50Hz/60Hz AC mains hum with wandering drift speed (0.2..1.5 Hz beat frequency)
        float hum_wander = u_time * 0.95 + sin(u_time * 0.31) * 0.85 + cos(u_time * 0.17) * 0.45;
        float hum_phase = fract(uv.y * 1.5 - hum_wander);
        // Asymmetric rectified capacitor charge/discharge (rapid diode surge, exponential drop + harmonics)
        hum_waveform = exp(-hum_phase * 2.8) * 0.70 + sin(hum_phase * 6.28318) * 0.20 + sin(hum_phase * 12.56636) * 0.10;

        // Irregular stochastic burst windows (Poisson-like stepped hash, NOT continuous sine!)
        float slot = floor(u_time * 1.6);
        float burst_rand = hash12(vec2(slot, 83.19));
        // Triggered irregularly ~22% of time slots
        float in_burst = step(0.78, burst_rand);
        float slot_fract = fract(u_time * 1.6);
        // Sharp immediate burst spike followed by rapid physical decay
        is_antenna_burst = in_burst * exp(-slot_fract * 4.5);

        // Authentic horizontal sync slip & jagged RF carrier hash across scanlines
        float burst_y = hash12(vec2(slot, 39.41));
        float tear_band = smoothstep(0.14, 0.0, abs(uv.y - burst_y));
        float line_rf_hash = (hash12(vec2(floor(uv.y * u_resolution.y), floor(u_time * 60.0))) - 0.5) * 2.0;
        float hum_tear = (tear_band * is_antenna_burst * (0.016 + 0.024 * line_rf_hash)) * u_antenna_hum;
        uv.x = clamp(uv.x + hum_tear, 0.0, 1.0);
    }

    // 3. Horizontal scanline jitter / sync noise with non-repeating wander
    if (u_jitter > 0.001) {
        float jitter_wander = 1.0 + 0.35 * sin(u_time * 0.58 + sin(u_time * 0.23));
        float line_seed = floor(uv.y * u_resolution.y) + floor(u_time * 60.0) * 17.31;
        float j_line = (hash12(vec2(line_seed, line_seed * 1.414)) - 0.5) * 2.0;
        float j_wave = sin(uv.y * 110.0 + u_time * 19.0 + sin(u_time * 0.35) * 4.0);
        uv.x += (j_line * 0.65 + j_wave * 0.35) * (u_jitter * 0.0035 * jitter_wander);
        uv.x = clamp(uv.x, 0.0, 1.0);
    }

    // 4. Point CRT Magnet Distortion (Lorentz force deflection around mouse cursor)
    vec2 uv_r = uv;
    vec2 uv_g = uv;
    vec2 uv_b = uv;
    float magnet_lum = 1.0;

    if (u_magnet > 0.001 && u_mouse_active > 0.001) {
        vec2 m_diff = uv - u_mouse;
        vec2 aspect_m = vec2(m_diff.x * (u_resolution.x / u_resolution.y), m_diff.y);
        float dist = length(aspect_m);

        float magnet_radius = 0.28;
        if (dist < magnet_radius) {
            float t = dist / magnet_radius;
            float falloff = (1.0 - t * t) * (1.0 - t * t);
            float strength = u_magnet * falloff;

            vec2 dir = (dist > 0.0001) ? (m_diff / dist) : vec2(0.0, 1.0);
            vec2 perp = vec2(-dir.y, dir.x);

            float radial_disp = (sin(dist * 22.0 - u_time * 2.5) * 0.012 - 0.040) * strength;
            float swirl_disp = 0.060 * strength;

            vec2 base_disp = dir * radial_disp + perp * swirl_disp;

            if (u_monochrome_mode == 0) {
                float chrom_spread = 0.030 * strength;
                uv_r = clamp(uv + base_disp + perp * chrom_spread + dir * (chrom_spread * 0.4), 0.0, 1.0);
                uv_g = clamp(uv + base_disp, 0.0, 1.0);
                uv_b = clamp(uv + base_disp - perp * chrom_spread - dir * (chrom_spread * 0.4), 0.0, 1.0);
            } else {
                uv_r = clamp(uv + base_disp, 0.0, 1.0);
                uv_g = uv_r;
                uv_b = uv_r;
            }

            magnet_lum = 1.0 + strength * (0.35 * sin(dist * 28.0) + 0.20);
        }
    }

    // 5. Chromatic Aberration & Color Sampling with thermal deflection drift
    vec3 color;
    if (u_monochrome_mode == 0 && (u_chromatic > 0.001 || (u_magnet > 0.001 && u_mouse_active > 0.001))) {
        float chrom_drift = 1.0 + 0.18 * sin(u_time * 0.28);
        vec2 global_shift = vec2(u_chromatic * 0.0035 * chrom_drift, 0.0);
        uv_r = clamp(uv_r + global_shift, 0.0, 1.0);
        uv_b = clamp(uv_b - global_shift, 0.0, 1.0);

        float r_idx = texture(u_framebuffer, uv_r).r;
        float g_idx = texture(u_framebuffer, uv_g).r;
        float b_idx = texture(u_framebuffer, uv_b).r;
        vec4 col_r = u_palette[int(r_idx * 255.0 + 0.5)];
        vec4 col_g = u_palette[int(g_idx * 255.0 + 0.5)];
        vec4 col_b = u_palette[int(b_idx * 255.0 + 0.5)];
        color = vec3(col_r.r, col_g.g, col_b.b) * magnet_lum;
    } else {
        float r = texture(u_framebuffer, uv_g).r;
        int index = int(r * 255.0 + 0.5);
        color = u_palette[index].rgb * magnet_lum;
    }

    // 6. Phosphor Afterglow / Bloom with thermal persistence drift
    if (u_afterglow > 0.001) {
        float glow_drift = 1.0 + 0.15 * sin(u_time * 0.35);
        vec2 texel = 1.0 / u_resolution;
        float b1 = texture(u_framebuffer, clamp(uv + vec2(texel.x * 1.5, 0.0), 0.0, 1.0)).r;
        float b2 = texture(u_framebuffer, clamp(uv - vec2(texel.x * 1.5, 0.0), 0.0, 1.0)).r;
        float b3 = texture(u_framebuffer, clamp(uv + vec2(0.0, texel.y * 1.5), 0.0, 1.0)).r;
        float b4 = texture(u_framebuffer, clamp(uv - vec2(0.0, texel.y * 1.5), 0.0, 1.0)).r;
        vec3 glow = (u_palette[int(b1 * 255.0 + 0.5)].rgb +
                     u_palette[int(b2 * 255.0 + 0.5)].rgb +
                     u_palette[int(b3 * 255.0 + 0.5)].rgb +
                     u_palette[int(b4 * 255.0 + 0.5)].rgb) * 0.25;
        color = mix(color, max(color, glow), u_afterglow * 0.55 * glow_drift);
    }

    // 7. Authentic CRT Electron Beam Scanlines
    if (u_scanlines > 0.001) {
        float pos_y = fract(uv.y * u_resolution.y);
        float dist = abs(pos_y - 0.5) * 2.0;
        float beam = exp(-dist * dist * 3.5);
        float lum = dot(color, vec3(0.299, 0.587, 0.114));
        float scan_factor = mix(1.0, mix(beam * 1.08, 1.0, lum * 0.35), u_scanlines);
        color *= scan_factor;
    }

    // 8. Authentic Phosphor Mask & Pixel Separation Grid
    if (u_pixel_grid > 0.001) {
        vec2 grid = fract(uv * u_resolution);
        vec2 edge_dist2 = min(grid, 1.0 - grid);
        float edge_dist = min(edge_dist2.x, edge_dist2.y);
        float border = smoothstep(0.0, 0.14, edge_dist);
        color *= mix(1.0, border, u_pixel_grid * 0.65);

        if (u_monochrome_mode > 0) {
            // Authentic monochrome phosphor dot/strip pitch (pure phosphor luminance)
            float dot_phase = (gl_FragCoord.x + gl_FragCoord.y * 0.5) * 1.5707963;
            float mask_mono = 0.82 + 0.18 * sin(dot_phase);
            color *= mix(1.0, mask_mono, u_pixel_grid * 0.55);
        } else {
            // Color Sub-pixel Phosphor Triad (Aperture Grille / Shadow Mask RGB bars)
            float triad_phase = gl_FragCoord.x * 2.0943951; // 2*pi / 3
            vec3 mask = vec3(
                0.5 + 0.5 * sin(triad_phase),
                0.5 + 0.5 * sin(triad_phase + 2.0943951),
                0.5 + 0.5 * sin(triad_phase + 4.1887902)
            );
            color *= mix(vec3(1.0), mask * 1.35, u_pixel_grid * 0.55);
        }
    }

    // 9. Antenna Hum bar luminance modulation (non-linear ripple + stochastic burst dip)
    if (u_antenna_hum > 0.001) {
        float hum_lum = 1.0 + (hum_waveform * 0.35 - 0.12) * u_antenna_hum;
        hum_lum *= (1.0 - is_antenna_burst * 0.24 * u_antenna_hum);
        color *= hum_lum;
    }

    // 10. Dynamic Analog TV Snow / RF Demodulation Static with realistic parameter drift
    if (u_noise > 0.001) {
        // AGC (Automatic Gain Control) breathing: slow organic drift in receiver noise floor
        float agc_drift = 1.0 + 0.25 * sin(u_time * 0.52 + sin(u_time * 0.19))
                              + 0.12 * cos(u_time * 0.93 + cos(u_time * 0.33));
        float eff_noise = clamp(u_noise * agc_drift, 0.0, 1.0);

        vec2 raster_coord = vec2(floor(uv.x * u_resolution.x * 1.25), floor(uv.y * u_resolution.y));
        float frame = mod(floor(u_time * 60.0), 10000.0);
        float frame_seed = frame * 17.13;

        // Drifting clumping ratio (demodulator bandwidth wander)
        float clump_ratio = 0.35 + 0.18 * sin(u_time * 0.37 + 1.2);
        float n_fine = hash12(raster_coord + vec2(frame_seed + 19.19, frame_seed * 1.618 + 73.73));
        float n_clust = hash12(floor(raster_coord * vec2(0.5, 1.0)) + vec2(frame_seed * 2.37 + 37.11, frame_seed * 0.79 + 91.37));
        float n = mix(n_fine, n_clust, clump_ratio);

        // Drifting flake threshold
        float flake_thresh = mix(0.96, 0.68, eff_noise) + 0.025 * sin(u_time * 0.71);
        float flake = smoothstep(flake_thresh, 1.0, n_fine) * (eff_noise * 1.85);

        // Pepper dropouts cutting into bright characters
        float drop_thresh = mix(0.04, 0.28, eff_noise);
        float drop = smoothstep(drop_thresh, 0.0, n_fine) * (eff_noise * 1.35);
        color *= (1.0 - clamp(drop, 0.0, 0.88));

        // Base RF tuner static grain
        float base_grain = (n - 0.5) * (eff_noise * 0.35);

        // Intermittent horizontal RF spark streak across random scanlines
        float line_spark_seed = hash12(vec2(floor(uv.y * u_resolution.y * 0.5), floor(u_time * 60.0)));
        float spark_streak = step(0.993, line_spark_seed) * hash12(vec2(floor(uv.x * 24.0), floor(u_time * 60.0) * 1.3)) * eff_noise * 1.3;

        if (u_monochrome_mode == 1) {
            // Amber CRT phosphor snow (strictly amber, blue = 0)
            vec3 phosphor = vec3(1.0, 0.65, 0.0);
            color += phosphor * (flake + spark_streak) + phosphor * base_grain;
        } else if (u_monochrome_mode == 2) {
            // Green CRT phosphor snow (strictly green, red = 0, blue = 0)
            vec3 phosphor = vec3(0.0, 1.0, 0.0);
            color += phosphor * (flake + spark_streak) + phosphor * base_grain;
        } else {
            // Color CRT: RF snow with subtle drifting subcarrier phase tint
            float rf_tint = hash12(raster_coord + vec2(frame_seed * 3.71, 41.13));
            float chroma_wander = sin(u_time * 0.45);
            vec3 tint = mix(vec3(1.0), vec3(0.85 + 0.25 * rf_tint + 0.1 * chroma_wander, 0.95, 1.15 - 0.25 * rf_tint - 0.1 * chroma_wander), 0.25);
            color += tint * (flake + spark_streak) + vec3(base_grain);
        }

        color = clamp(color, 0.0, 1.0);
    }

    // 11. Vignette when curvature is present
    if (u_curvature > 0.001) {
        vec2 vin_uv = uv * (1.0 - uv.yx);
        float vig = vin_uv.x * vin_uv.y * 15.0;
        vig = clamp(pow(vig, 0.15), 0.0, 1.0);
        color *= vig;
    }

    outColor = vec4(color, 1.0);
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

        let draw_w = gl.drawing_buffer_width();
        let draw_h = gl.drawing_buffer_height();
        gl.viewport(0, 0, draw_w, draw_h);

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
        let draw_w = self.gl.drawing_buffer_width();
        let draw_h = self.gl.drawing_buffer_height();
        self.gl.viewport(0, 0, draw_w, draw_h);
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
        self.render_frame_with_effects(fb, &VisualEffects::default(), 0.0, (0.5, 0.5), false);
    }

    #[cfg(target_arch = "wasm32")]
    pub fn render_frame_with_effects(
        &mut self,
        fb: &Framebuffer,
        fx: &VisualEffects,
        time: f32,
        mouse_uv: (f32, f32),
        mouse_active: bool,
    ) {
        if fb.width != self.width || fb.height != self.height {
            self.resize(fb.width, fb.height);
        }
        let draw_w = self.gl.drawing_buffer_width();
        let draw_h = self.gl.drawing_buffer_height();
        self.gl.viewport(0, 0, draw_w, draw_h);
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

        // Upload CRT post-processing effect uniforms
        let scan_loc = self.gl.get_uniform_location(&self.program, "u_scanlines");
        self.gl.uniform1f(scan_loc.as_ref(), fx.scanlines);

        let grid_loc = self.gl.get_uniform_location(&self.program, "u_pixel_grid");
        self.gl.uniform1f(grid_loc.as_ref(), fx.pixel_grid);

        let chrom_loc = self.gl.get_uniform_location(&self.program, "u_chromatic");
        self.gl.uniform1f(chrom_loc.as_ref(), fx.chromatic);

        let glow_loc = self.gl.get_uniform_location(&self.program, "u_afterglow");
        self.gl.uniform1f(glow_loc.as_ref(), fx.afterglow);

        let curv_loc = self.gl.get_uniform_location(&self.program, "u_curvature");
        self.gl.uniform1f(curv_loc.as_ref(), fx.curvature);

        let jit_loc = self.gl.get_uniform_location(&self.program, "u_jitter");
        self.gl.uniform1f(jit_loc.as_ref(), fx.jitter);

        let time_loc = self.gl.get_uniform_location(&self.program, "u_time");
        self.gl.uniform1f(time_loc.as_ref(), time);

        // Point CRT magnet deflection uniforms
        let magnet_loc = self.gl.get_uniform_location(&self.program, "u_magnet");
        self.gl.uniform1f(magnet_loc.as_ref(), fx.magnet);

        let mouse_loc = self.gl.get_uniform_location(&self.program, "u_mouse");
        self.gl.uniform2f(mouse_loc.as_ref(), mouse_uv.0, mouse_uv.1);

        let active_loc = self.gl.get_uniform_location(&self.program, "u_mouse_active");
        self.gl.uniform1f(active_loc.as_ref(), if mouse_active { 1.0 } else { 0.0 });

        let noise_loc = self.gl.get_uniform_location(&self.program, "u_noise");
        self.gl.uniform1f(noise_loc.as_ref(), fx.noise);

        let hum_loc = self.gl.get_uniform_location(&self.program, "u_antenna_hum");
        self.gl.uniform1f(hum_loc.as_ref(), fx.antenna_hum);

        let mono_val = match fb.font_mode {
            pixel_ssh_view::SystemMode::Amber => 1,
            pixel_ssh_view::SystemMode::GreenCrt => 2,
            _ => 0,
        };
        let mono_loc = self.gl.get_uniform_location(&self.program, "u_monochrome_mode");
        self.gl.uniform1i(mono_loc.as_ref(), mono_val);

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

    #[cfg(not(target_arch = "wasm32"))]
    pub fn render_frame_with_effects(
        &mut self,
        _fb: &Framebuffer,
        _fx: &VisualEffects,
        _time: f32,
        _mouse_uv: (f32, f32),
        _mouse_active: bool,
    ) {}
}
