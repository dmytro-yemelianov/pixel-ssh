//! Unit tests for core application state and view rendering.

use super::*;
use pixel_ssh_view::*;

    #[test]
    fn test_all_systems_and_tabs_render_within_bounds() {
        for mode in SystemMode::ALL {
            let (w, h) = mode.resolution();
            let (cols, rows) = mode.char_grid();

            assert!(w > 0 && h > 0);
            assert!(cols > 0 && rows > 0);

            let mut app = App::new();
            app.system_mode = mode;
            app.palette_mode = mode;

            for tab in [Tab::Projects, Tab::Resume, Tab::About, Tab::Visuals, Tab::Contact, Tab::Help] {
                app.current_tab = tab;
                app.show_detail = false;

                let view = app.render();
                assert_eq!(view.width, w);
                assert_eq!(view.height, h);
                assert_eq!(view.system_mode, mode);

                for elem in &view.elements {
                    match elem {
                        Element::Text(t) => {
                            assert!(t.x < w, "Text '{:?}' x={} out of bounds w={}", t.text, t.x, w);
                            assert!(t.y < h, "Text '{:?}' y={} out of bounds h={}", t.text, t.y, h);
                            let text_len = t.text.chars().count() as u16;
                            assert!(
                                t.x + text_len * 8 <= w + 8, // slight margin for right-aligned items
                                "Text '{:?}' (len {}) at x={} overflows w={} for mode {:?}",
                                t.text, text_len, t.x, w, mode
                            );
                        }
                        Element::Rect(r) => {
                            assert!(r.x <= w, "Rect x={} out of bounds w={}", r.x, w);
                            assert!(r.y <= h, "Rect y={} out of bounds h={}", r.y, h);
                        }
                        Element::Link(l) => {
                            assert!(l.x < w, "Link '{:?}' x={} out of bounds w={}", l.text, l.x, w);
                            assert!(l.y < h, "Link '{:?}' y={} out of bounds h={}", l.text, l.y, h);
                        }
                        Element::Sprite(s) => {
                            assert!(s.x < w && s.y < h);
                        }
                    }
                }

                // Also test project detail view
                if tab == Tab::Projects {
                    app.show_detail = true;
                    for p_idx in 0..PROJECTS.len().min(4) {
                        app.selected_project = p_idx;
                        let detail_view = app.render();
                        for elem in &detail_view.elements {
                            if let Element::Text(t) = elem {
                                assert!(t.x < w && t.y < h);
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_system_switching_key_and_click() {
        // On Web: 's' opens System modal, digit key selects mode
        let mut app = App::new_web();
        assert_eq!(app.system_mode, SystemMode::Vga);
        assert_eq!(app.system_mode.char_grid(), (80, 25));
        assert_eq!(app.system_mode.resolution(), (640, 400));

        // Press 's' opens System modal
        assert!(app.update(InputEvent::KeyDown(Key::Char('s'))));
        assert_eq!(app.active_modal, ActiveModal::System);

        // Key '8' in System modal selects ZxSpectrum (8th mode in ALL)
        assert!(app.update(InputEvent::KeyDown(Key::Char('8'))));
        assert_eq!(app.system_mode, SystemMode::ZxSpectrum);
        assert_eq!(app.system_mode.char_grid(), (32, 24));
        assert_eq!(app.system_mode.resolution(), (256, 192));
        assert_eq!(app.active_modal, ActiveModal::None);

        // On Terminal: 's' cycles system mode directly
        let mut app_term = App::new_terminal();
        assert_eq!(app_term.system_mode, SystemMode::Vga);
        assert!(app_term.update(InputEvent::KeyDown(Key::Char('s'))));
        assert_eq!(app_term.system_mode, SystemMode::Ega);
        assert!(app_term.update(InputEvent::KeyDown(Key::Char('s'))));
        assert_eq!(app_term.system_mode, SystemMode::Cga);
    }

    #[test]
    fn test_terminal_mode_omits_visuals_tab_and_cycles_cleanly() {
        let mut app = App::new_terminal();
        assert_eq!(app.platform, Platform::Terminal);
        assert!(app.status.contains("[1-5] Tabs"));

        // Render in 80 cols and verify no Visuals tab is in the header
        let view = app.render();
        for elem in &view.elements {
            if let Element::Text(t) = elem {
                assert!(!t.text.contains("Visuals"), "Terminal header unexpectedly contains Visuals: {}", t.text);
            }
        }

        // Test keyboard tab selection on terminal: 1=Projects, 2=Resume, 3=About, 4=Contact, 5=Help, 6=ignored
        assert!(app.update(InputEvent::KeyDown(Key::Char('1'))));
        assert_eq!(app.current_tab, Tab::Projects);

        assert!(app.update(InputEvent::KeyDown(Key::Char('2'))));
        assert_eq!(app.current_tab, Tab::Resume);

        assert!(app.update(InputEvent::KeyDown(Key::Char('3'))));
        assert_eq!(app.current_tab, Tab::About);

        assert!(app.update(InputEvent::KeyDown(Key::Char('4'))));
        assert_eq!(app.current_tab, Tab::Contact);

        assert!(app.update(InputEvent::KeyDown(Key::Char('5'))));
        assert_eq!(app.current_tab, Tab::Help);

        // Key '6' is a no-op on Terminal
        assert!(!app.update(InputEvent::KeyDown(Key::Char('6'))));
        assert_eq!(app.current_tab, Tab::Help);

        // Tab cycling on Terminal should skip Visuals:
        // Help -> Projects -> Resume -> About -> Contact -> Help
        assert!(app.update(InputEvent::KeyDown(Key::Tab)));
        assert_eq!(app.current_tab, Tab::Projects);

        assert!(app.update(InputEvent::KeyDown(Key::Tab)));
        assert_eq!(app.current_tab, Tab::Resume);

        assert!(app.update(InputEvent::KeyDown(Key::Tab)));
        assert_eq!(app.current_tab, Tab::About);

        assert!(app.update(InputEvent::KeyDown(Key::Tab)));
        assert_eq!(app.current_tab, Tab::Contact);

        assert!(app.update(InputEvent::KeyDown(Key::Tab)));
        assert_eq!(app.current_tab, Tab::Help);

        assert!(app.update(InputEvent::KeyDown(Key::Tab)));
        assert_eq!(app.current_tab, Tab::Projects);

        // Test all system modes render within bounds for Terminal
        for mode in SystemMode::ALL {
            app.system_mode = mode;
            app.palette_mode = mode;
            for tab in [Tab::Projects, Tab::Resume, Tab::About, Tab::Contact, Tab::Help] {
                app.current_tab = tab;
                let view = app.render();
                let (w, h) = (app.terminal_cols * 8, app.terminal_rows * 16);
                for elem in &view.elements {
                    if let Element::Text(t) = elem {
                        assert!(t.x < w, "Terminal text '{:?}' x={} out of bounds w={}", t.text, t.x, w);
                        assert!(t.y < h, "Terminal text '{:?}' y={} out of bounds h={}", t.text, t.y, h);
                    }
                }
            }
        }
    }

    #[test]
    fn test_web_mode_modals_and_articles() {
        let mut app = App::new_web();
        assert_eq!(app.platform, Platform::Web);
        assert!(app.status.contains("[1-4] Nav"));

        // Render in 80 cols and verify Web header contains [1] Projects, [2] CV, [3] About, [4] Contact, [VIS], [SYS], [?]
        let view = app.render();
        let mut found_projects = false;
        let mut found_cv = false;
        let mut found_vis = false;
        for elem in &view.elements {
            match elem {
                Element::Text(t) => {
                    if t.text.contains("Projects") { found_projects = true; }
                    if t.text.contains("CV") { found_cv = true; }
                }
                Element::Link(l) => {
                    if l.text.contains("[VIS]") { found_vis = true; }
                }
                _ => {}
            }
        }
        assert!(found_projects, "Web header should contain Projects");
        assert!(found_cv, "Web header should contain CV");
        assert!(found_vis, "Web header should contain [VIS]");

        // Key 'v' toggles Visuals modal
        assert!(app.update(InputEvent::KeyDown(Key::Char('v'))));
        assert_eq!(app.active_modal, ActiveModal::Visuals);
        assert!(app.update(InputEvent::KeyDown(Key::Escape)));
        assert_eq!(app.active_modal, ActiveModal::None);

        // Key '?' toggles Help modal
        assert!(app.update(InputEvent::KeyDown(Key::Char('?'))));
        assert_eq!(app.active_modal, ActiveModal::Help);
        assert!(app.update(InputEvent::KeyDown(Key::Escape)));
        assert_eq!(app.active_modal, ActiveModal::None);

        // Key '2' opens CV (article overlay)
        assert!(app.update(InputEvent::KeyDown(Key::Char('2'))));
        assert_eq!(app.current_tab, Tab::Resume);
        // Pressing Escape returns to Projects
        assert!(app.update(InputEvent::KeyDown(Key::Escape)));
        assert_eq!(app.current_tab, Tab::Projects);

        // Tab cycling cycles 1 -> 2 -> 3 -> 4 -> 1
        assert!(app.update(InputEvent::KeyDown(Key::Tab)));
        assert_eq!(app.current_tab, Tab::Resume);
        assert!(app.update(InputEvent::KeyDown(Key::Tab)));
        assert_eq!(app.current_tab, Tab::About);
        assert!(app.update(InputEvent::KeyDown(Key::Tab)));
        assert_eq!(app.current_tab, Tab::Contact);
        assert!(app.update(InputEvent::KeyDown(Key::Tab)));
        assert_eq!(app.current_tab, Tab::Projects);
    }

    #[test]
    fn test_zx_spectrum_navpanel_no_overlapping_chars() {
        let mut app = App::new_web();
        app.system_mode = SystemMode::ZxSpectrum;
        app.palette_mode = SystemMode::ZxSpectrum;

        let view = app.render();
        let mut nav_items: Vec<(u16, u16, String)> = Vec::new();

        for elem in &view.elements {
            match elem {
                Element::Text(t) if t.y == 11 => {
                    let width = t.text.chars().count() as u16 * 8;
                    nav_items.push((t.x, t.x + width, t.text.clone()));
                }
                Element::Link(l) if l.y == 11 => {
                    nav_items.push((l.x, l.x + l.width, l.text.clone()));
                }
                _ => {}
            }
        }

        nav_items.sort_by_key(|item| item.0);
        assert!(nav_items.len() >= 5, "Expected at least 5 nav items, found {}", nav_items.len());

        for i in 0..nav_items.len() - 1 {
            let (_start_a, end_a, ref text_a) = nav_items[i];
            let (start_b, _, ref text_b) = nav_items[i + 1];
            assert!(
                end_a <= start_b,
                "Navpanel items overlap in Web ZX Spectrum: '{:?}' ends at {} but '{:?}' starts at {}",
                text_a, end_a, text_b, start_b
            );
        }

        // Also verify terminal navigation bar tabs at y = 16 do not overlap
        let term_app = App::new_terminal();
        let term_view = term_app.render();
        let mut term_nav_items: Vec<(u16, u16, String)> = Vec::new();
        for elem in &term_view.elements {
            match elem {
                Element::Text(t) if t.y == 16 => {
                    let width = t.text.chars().count() as u16 * 8;
                    term_nav_items.push((t.x, t.x + width, t.text.clone()));
                }
                Element::Link(l) if l.y == 16 => {
                    term_nav_items.push((l.x, l.x + l.width, l.text.clone()));
                }
                _ => {}
            }
        }
        term_nav_items.sort_by_key(|item| item.0);
        assert!(term_nav_items.len() >= 5, "Expected at least 5 term nav items, found {}", term_nav_items.len());
        for i in 0..term_nav_items.len() - 1 {
            let (_start_a, end_a, ref text_a) = term_nav_items[i];
            let (start_b, _, ref text_b) = term_nav_items[i + 1];
            assert!(
                end_a <= start_b,
                "Terminal navpanel items overlap: '{:?}' ends at {} but '{:?}' starts at {}",
                text_a, end_a, text_b, start_b
            );
        }
    }

    #[test]
    fn test_clock_colon_steady_cadence() {
        let mut app = App::new();
        app.set_time(12, 34, 56);
        let clock1 = app.clock_formatted();
        assert_eq!(clock1, "12:34");

        // Keypresses should not cause rapid colon flickering
        app.update(InputEvent::KeyDown(Key::Down));
        let clock2 = app.clock_formatted();
        assert_eq!(clock2, "12:34");

        app.set_time(12, 34, 57);
        let clock3 = app.clock_formatted();
        assert_eq!(clock3, "12 34");
    }

    #[test]
    fn test_all_16_projects_have_detailed_specs_and_subsystems() {
        use crate::data::details::get_project_detail;

        assert_eq!(PROJECTS.len(), 16, "Expected exactly 16 portfolio projects");

        for p in PROJECTS {
            let detail = get_project_detail(p.slug);
            assert!(
                detail.overview.len() >= 100,
                "Project {} has too short overview (len {})",
                p.slug,
                detail.overview.len()
            );
            assert!(
                detail.subsystems.len() >= 4,
                "Project {} has {} subsystems, expected >= 4",
                p.slug,
                detail.subsystems.len()
            );
            assert!(
                detail.specs.len() >= 4,
                "Project {} has {} specs, expected >= 4",
                p.slug,
                detail.specs.len()
            );
            assert!(
                detail.diagram_80.len() >= 4,
                "Project {} has {} lines in diagram_80, expected >= 4",
                p.slug,
                detail.diagram_80.len()
            );
            assert!(
                detail.diagram_40.len() >= 4,
                "Project {} has {} lines in diagram_40, expected >= 4",
                p.slug,
                detail.diagram_40.len()
            );
            assert!(
                detail.diagram_32.len() >= 4,
                "Project {} has {} lines in diagram_32, expected >= 4",
                p.slug,
                detail.diagram_32.len()
            );
            assert!(
                detail.highlights.len() >= 3,
                "Project {} has {} highlights, expected >= 3",
                p.slug,
                detail.highlights.len()
            );
        }
    }

    #[test]
    fn test_project_detail_lines_width_bounds_across_all_resolutions() {
        for p in PROJECTS {
            // 80 columns: text must fit <= 74 chars
            let lines_80 = project_detail_lines(p, 80, 0);
            assert!(lines_80.len() > 15, "Project {} lines_80 too short ({})", p.slug, lines_80.len());
            for (idx, (l, _, _)) in lines_80.iter().enumerate() {
                let char_count = l.chars().count();
                assert!(
                    char_count <= 74,
                    "Project {} 80-col line {} exceeds 74 chars ({}): '{}'",
                    p.slug, idx, char_count, l
                );
            }

            // 40 columns: text must fit <= 38 chars
            let lines_40 = project_detail_lines(p, 40, 0);
            assert!(lines_40.len() > 15, "Project {} lines_40 too short ({})", p.slug, lines_40.len());
            for (idx, (l, _, _)) in lines_40.iter().enumerate() {
                let char_count = l.chars().count();
                assert!(
                    char_count <= 38,
                    "Project {} 40-col line {} exceeds 38 chars ({}): '{}'",
                    p.slug, idx, char_count, l
                );
            }

            // 32 columns: text must fit <= 29 chars
            let lines_32 = project_detail_lines(p, 32, 0);
            assert!(lines_32.len() > 15, "Project {} lines_32 too short ({})", p.slug, lines_32.len());
            for (idx, (l, _, _)) in lines_32.iter().enumerate() {
                let char_count = l.chars().count();
                assert!(
                    char_count <= 29,
                    "Project {} 32-col line {} exceeds 29 chars ({}): '{}'",
                    p.slug, idx, char_count, l
                );
            }
        }
    }

    #[test]
    fn test_detail_view_scrolling_and_navigation() {
        let mut app = App::new();
        app.current_tab = Tab::Projects;
        app.show_detail = true;
        app.selected_project = 0;

        let max_scroll_80 = app.detail_max_scroll();
        assert!(max_scroll_80 > 0, "80-col detail view should be scrollable (max_scroll={})", max_scroll_80);

        // Scroll down with 'j'
        assert_eq!(app.detail_scroll, 0);
        app.update(InputEvent::KeyDown(Key::Char('j')));
        assert_eq!(app.detail_scroll, 1);

        // Scroll up with 'k'
        app.update(InputEvent::KeyDown(Key::Char('k')));
        assert_eq!(app.detail_scroll, 0);

        // Cycle to next project with 'l'
        app.detail_scroll = 5;
        app.update(InputEvent::KeyDown(Key::Char('l')));
        assert_eq!(app.selected_project, 1);
        assert_eq!(app.detail_scroll, 0, "Changing project should reset scroll");

        // Cycle to prev project with 'h'
        app.update(InputEvent::KeyDown(Key::Char('h')));
        assert_eq!(app.selected_project, 0);
        assert_eq!(app.detail_scroll, 0);

        // ESC should return to list
        app.update(InputEvent::KeyDown(Key::Escape));
        assert!(!app.show_detail, "ESC should close detail view");
    }

    #[test]
    fn test_visuals_magnet_controls_and_presets() {
        let mut app = App::new();
        app.current_tab = Tab::Visuals;

        // Default has magnet effect disabled completely (0.0)
        assert_eq!(app.visual_effects.magnet, 0.0);

        // 'a' switches to Clean preset (magnet = 0.0)
        app.update(InputEvent::KeyDown(Key::Char('a')));
        assert_eq!(app.visual_effects.magnet, 0.0);

        // 'w' switches to 80s Trinitron CRT preset (magnet = 0.0)
        app.update(InputEvent::KeyDown(Key::Char('w')));
        assert_eq!(app.visual_effects, VisualEffects::crt_trinitron());
        assert_eq!(app.visual_effects.magnet, 0.0);

        // 'g' toggles magnet effect slider
        app.update(InputEvent::KeyDown(Key::Char('g')));
        assert_eq!(app.selected_fx_slider, 6);
        assert!(app.status.contains("Magnet"));

        // Fine tune slider 6 with '+' and '-'
        let prev = app.visual_effects.magnet;
        app.update(InputEvent::KeyDown(Key::Char('+')));
        assert!((app.visual_effects.magnet - (prev + 0.05)).abs() < 1e-4);

        app.update(InputEvent::KeyDown(Key::Char('-')));
        assert!((app.visual_effects.magnet - prev).abs() < 1e-4);
    }

    #[test]
    fn test_resume_lines_width_bounds_across_all_resolutions() {
        // 80 columns: text must fit <= 74 chars
        assert!(RESUME_LINES_80.len() >= 40);
        for (idx, (l, _, _)) in RESUME_LINES_80.iter().enumerate() {
            let char_count = l.chars().count();
            assert!(
                char_count <= 74,
                "RESUME_LINES_80 line {} exceeds 74 chars ({}): '{}'",
                idx, char_count, l
            );
        }

        // 40 columns: text must fit <= 38 chars
        assert!(RESUME_LINES_40.len() >= 40);
        for (idx, (l, _, _)) in RESUME_LINES_40.iter().enumerate() {
            let char_count = l.chars().count();
            assert!(
                char_count <= 38,
                "RESUME_LINES_40 line {} exceeds 38 chars ({}): '{}'",
                idx, char_count, l
            );
        }

        // 32 columns: text must fit <= 29 chars
        assert!(RESUME_LINES_32.len() >= 40);
        for (idx, (l, _, _)) in RESUME_LINES_32.iter().enumerate() {
            let char_count = l.chars().count();
            assert!(
                char_count <= 29,
                "RESUME_LINES_32 line {} exceeds 29 chars ({}): '{}'",
                idx, char_count, l
            );
        }
    }

    #[test]
    fn test_mouse_pos_tracking_and_view_propagation() {
        let mut app = App::new();
        assert_eq!(app.mouse_pos, None);

        // Move mouse
        assert!(app.update(InputEvent::PointerMove { x: 120, y: 85 }));
        assert_eq!(app.mouse_pos, Some((120, 85)));

        // Render should propagate mouse_pos to View
        let view = app.render();
        assert_eq!(view.mouse_pos, Some((120, 85)));

        // Click also maintains mouse_pos
        app.update(InputEvent::PointerDown { x: 200, y: 150, button: Button::Left });
        assert_eq!(app.mouse_pos, Some((200, 150)));

        // Leave clears mouse_pos
        assert!(app.update(InputEvent::PointerLeave));
        assert_eq!(app.mouse_pos, None);

        let view2 = app.render();
        assert_eq!(view2.mouse_pos, None);
    }

    #[test]
    fn test_mouse_move_does_not_advance_tick_rate() {
        let mut app = App::new();
        let initial_tick = app.tick;

        // Multiple rapid pointer moves (simulating mouse drag/movement across canvas)
        for i in 0..100 {
            app.update(InputEvent::PointerMove { x: i, y: i * 2 });
        }

        // Ticks must remain completely constant and not be accelerated by input events
        assert_eq!(app.tick, initial_tick, "PointerMove events must not advance tick");

        // Key presses and clicks must also not advance tick
        app.update(InputEvent::KeyDown(Key::Down));
        app.update(InputEvent::PointerDown { x: 50, y: 50, button: Button::Left });
        assert_eq!(app.tick, initial_tick, "Input events must not advance tick");

        // Only dedicated tick() advances the animation clock
        app.tick();
        assert_eq!(app.tick, initial_tick + 1, "App::tick() should advance tick by 1");
    }

    #[test]
    fn test_terminal_mode_consecutive_project_rows_no_2_grouping() {
        let app = App::new_terminal();
        assert_eq!(app.platform, Platform::Terminal);

        let view = app.render();
        // Collect all project row texts starting with "[0" or "[1"
        let mut row_ys: Vec<u16> = Vec::new();
        for elem in &view.elements {
            if let Element::Text(t) = elem {
                if t.x == 8 && (t.text.starts_with("> [") || t.text.starts_with("  [")) {
                    row_ys.push(t.y);
                }
            }
        }

        assert!(row_ys.len() >= 6, "Expected at least 6 visible project rows on terminal, found {}", row_ys.len());

        // Each consecutive project row must be spaced by exactly 16 pixels (1 character line = row + 1).
        // If there were 2-grouping, the deltas would alternate (e.g. 16, 32, 16, 32).
        for i in 0..row_ys.len() - 1 {
            let delta = row_ys[i + 1] - row_ys[i];
            assert_eq!(
                delta, 16,
                "Project rows must be on consecutive lines! Row {} (y={}) and Row {} (y={}) delta={}",
                i, row_ys[i], i + 1, row_ys[i + 1], delta
            );
        }
    }

    #[test]
    fn test_terminal_mode_expands_to_custom_dimensions() {
        let mut app = App::new_terminal();
        app.set_terminal_size(140, 45);

        assert_eq!(app.terminal_cols, 140);
        assert_eq!(app.terminal_rows, 45);

        let view = app.render();
        assert_eq!(view.width, 140 * 8);
        assert_eq!(view.height, 45 * 16);

        // Projects max visible should scale with terminal height
        let max_vis = app.projects_max_visible();
        assert_eq!(max_vis, 45 - 5); // 40 visible items
    }

    #[test]
    fn test_vga_80_col_consecutive_project_rows_no_2_grouping() {
        let app = App::new();
        assert_eq!(app.platform, Platform::Web);
        assert_eq!(app.system_mode, SystemMode::Vga);

        let view = app.render();
        let mut row_ys: Vec<u16> = Vec::new();
        for elem in &view.elements {
            if let Element::Text(t) = elem {
                if t.x == 16 && (t.text.starts_with("> [") || t.text.starts_with("  [")) {
                    row_ys.push(t.y);
                }
            }
        }

        assert!(row_ys.len() >= 12, "Expected at least 12 visible project rows on VGA, found {}", row_ys.len());

        for i in 0..row_ys.len() - 1 {
            let delta = row_ys[i + 1] - row_ys[i];
            assert_eq!(
                delta, 16,
                "80-col project rows must be on consecutive lines! Row {} (y={}) and Row {} (y={}) delta={}",
                i, row_ys[i], i + 1, row_ys[i + 1], delta
            );
        }
    }

    #[test]
    fn test_80_col_all_elements_aligned_to_16px_grid() {
        for tab in [Tab::Projects, Tab::Resume, Tab::About, Tab::Visuals, Tab::Contact, Tab::Help] {
            let mut app = App::new();
            app.current_tab = tab;
            let view = app.render();

            for elem in &view.elements {
                match elem {
                    Element::Text(t) => {
                        assert_eq!(
                            t.y % 16, 0,
                            "Text '{}' at y={} in tab {:?} is not aligned to 16px character row grid!",
                            t.text, t.y, tab
                        );
                    }
                    Element::Link(l) => {
                        assert_eq!(
                            l.y % 16, 0,
                            "Link '{}' at y={} in tab {:?} is not aligned to 16px character row grid!",
                            l.text, l.y, tab
                        );
                    }
                    Element::Rect(r) => {
                        // Background rects should start on character rows
                        if r.height >= 16 {
                            assert_eq!(
                                r.y % 16, 0,
                                "Rect at (x={}, y={}, w={}, h={}) in tab {:?} is not aligned to 16px grid!",
                                r.x, r.y, r.width, r.height, tab
                            );
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    #[test]
    fn test_about_tab_includes_profile_portrait_and_sprite() {
        // 1. Verify text lines in all column modes contain architect profile
        assert!(ABOUT_LINES_80.iter().any(|(t, _, _)| t.contains("DMYTRO YEMELIANOV")));
        assert!(ABOUT_LINES_80.iter().any(|(t, _, _)| t.contains("ARCHITECT")));
        assert!(ABOUT_LINES_80.iter().any(|(t, _, _)| t.contains("┌────────────┐")));

        assert!(ABOUT_LINES_40.iter().any(|(t, _, _)| t.contains("DMYTRO YEMELIANOV")));
        assert!(ABOUT_LINES_40.iter().any(|(t, _, _)| t.contains("┌────────┐")));

        assert!(ABOUT_LINES_32.iter().any(|(t, _, _)| t.contains("DMYTRO YEMELIANOV")));
        assert!(ABOUT_LINES_32.iter().any(|(t, _, _)| t.contains("┌────────┐")));

        // 2. Verify Web mode About tab renders high-resolution dithered sprite
        let mut app = App::new_web();
        app.current_tab = Tab::About;
        let view = app.render();

        let mut found_sprite = false;
        for elem in &view.elements {
            if let Element::Sprite(s) = elem {
                assert!(s.width == 96 || s.width == 64);
                assert!(s.height == 96 || s.height == 64);
                assert_eq!(s.data.len(), (s.width as usize) * (s.height as usize));
                found_sprite = true;
            }
        }
        assert!(found_sprite, "About article on Web must render dithered profile sprite");

        // 3. Verify get_profile_sprite returns valid data for all 10 system modes
        for mode in SystemMode::ALL {
            let s96 = crate::data::profile::get_profile_sprite(mode, 96);
            assert_eq!(s96.len(), 96 * 96);
            let s64 = crate::data::profile::get_profile_sprite(mode, 64);
            assert_eq!(s64.len(), 64 * 64);
        }
    }


