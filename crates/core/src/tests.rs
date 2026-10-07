//! Unit tests for core application state and view rendering.

use super::*;
use pixel_ssh_view::*;

#[test]
fn links_screen_opens_apps_and_repositories_and_drags_with_the_finger() {
    let mut app = App::new_web();
    app.update(InputEvent::KeyDown(Key::Char('l')));
    assert_eq!(app.current_tab, Tab::Links);
    for url in [
        "https://autocaded.yemelianov.dev/",
        "https://github.com/dmytro-yemelianov/autocaded",
        "https://nethacked.yemelianov.dev/",
        "https://github.com/dmytro-yemelianov/NetHackED",
    ] {
        app.update(InputEvent::KeyDown(Key::Enter));
        assert_eq!(app.take_link_activation().as_deref(), Some(url));
        app.update(InputEvent::KeyDown(Key::Tab));
    }
    app.update(InputEvent::KeyDown(Key::Home));
    app.update(InputEvent::TouchDrag { dy: -1 });
    assert_eq!(app.selected_link, 1);
    assert_eq!(app.take_link_activation(), None);
    app.update(InputEvent::TouchDrag { dy: 1 });
    assert_eq!(app.selected_link, 0);
    app.update(InputEvent::KeyDown(Key::Escape));
    assert_eq!(app.current_tab, Tab::Projects);
}

#[test]
fn links_menu_and_last_link_remain_accessible_in_every_display_size() {
    for resolution in ResolutionMode::ALL {
        for portrait in [false, true] {
            let mut app = App::new_web();
            app.set_resolution(resolution);
            if portrait {
                app.set_web_height(Some(1200));
            }
            let (width, _) = app.view_dimensions();
            let (nav_y, nav_h) = app.navigation_geometry();
            app.update(InputEvent::PointerDown {
                x: width / 6 * 5 + 12,
                y: nav_y + nav_h / 2,
                button: Button::Left,
            });
            assert_eq!(app.current_tab, Tab::Links);
            app.update(InputEvent::KeyDown(Key::End));
            let view = app.render();
            let last = view
                .elements
                .iter()
                .find_map(|element| match element {
                    Element::Link(link)
                        if link.url == "https://yemelianov.dev/?doc=06-roadmap-and-milestones" =>
                    {
                        Some(link)
                    }
                    _ => None,
                })
                .unwrap();
            assert!(last.y + resolution.line_height() < nav_y);
            assert_eq!(
                view.link_at(last.x + 4, last.y + 2)
                    .map(|link| link.url.as_str()),
                Some("https://yemelianov.dev/?doc=06-roadmap-and-milestones")
            );
            app.update(InputEvent::KeyDown(Key::Enter));
            assert_eq!(
                app.take_link_activation().as_deref(),
                Some("https://yemelianov.dev/?doc=06-roadmap-and-milestones")
            );
            app.update(InputEvent::KeyDown(Key::Home));
            assert_eq!(app.links_scroll, 0);
        }
    }
}

#[test]
fn detail_enter_activates_links_without_reopening_or_resetting_the_page() {
    let mut app = App::new_web();
    app.selected_project = PROJECTS.iter().position(|p| p.demo.is_some()).unwrap();
    app.selected_list_item = app.selected_project + 3;
    app.show_detail = true;
    app.detail_scroll = 2;
    let project = &PROJECTS[app.selected_project];
    app.update(InputEvent::KeyDown(Key::Enter));
    assert_eq!(
        app.take_link_activation(),
        Some(format!(
            "https://github.com/dmytro-yemelianov/{}",
            project.slug
        ))
    );
    assert_eq!(app.take_link_activation(), None);
    assert!(app.show_detail);
    assert_eq!(app.detail_scroll, 2);
    app.update(InputEvent::KeyDown(Key::Tab));
    app.update(InputEvent::KeyDown(Key::Enter));
    assert_eq!(app.take_link_activation().as_deref(), project.demo);
    assert_eq!(app.selected_detail_item, 1);
    app.update(InputEvent::KeyDown(Key::Tab));
    app.update(InputEvent::KeyDown(Key::Enter));
    assert!(!app.show_detail);
    assert_eq!(app.take_link_activation(), None);
}

#[test]
fn overlays_capture_links_both_inside_and_outside_their_frame() {
    let mut app = App::new_web();
    app.set_resolution(ResolutionMode::Vga);
    app.current_tab = Tab::Contact;
    assert!(app.render().link_at(32, 100).is_some());
    for modal in [ActiveModal::Help, ActiveModal::Visuals] {
        app.active_modal = modal;
        let view = app.render();
        assert!(
            view.link_at(32, 100).is_none(),
            "covered email link: {modal:?}"
        );
        assert!(
            view.link_at(460, 4).is_none(),
            "header outside modal: {modal:?}"
        );
        let close = view
            .elements
            .iter()
            .find_map(|e| match e {
                Element::Link(l) if l.url.starts_with("#close") => Some(l),
                _ => None,
            })
            .unwrap();
        assert_eq!(
            view.link_at(close.x + 4, close.y + 4).map(|l| &l.url),
            Some(&close.url)
        );
    }
}

#[test]
fn links_use_all_font_rows_and_character_widths() {
    for resolution in ResolutionMode::ALL {
        let (width, height) = resolution.resolution();
        let mut view = View::new(width, height);
        view.resolution = resolution;
        view.add(Element::Link(LinkElement::new(
            8,
            32,
            "ї",
            "https://example.com",
            TextStyle::new(Color::from_palette(7)),
        )));
        let cell_height = resolution.line_height();
        for dy in 0..cell_height {
            assert!(
                view.link_at(12, 32 + dy).is_some(),
                "{resolution:?}, row {dy}"
            );
        }
        assert!(view.link_at(12, 32 + cell_height).is_none());
        assert!(
            view.link_at(16, 32).is_none(),
            "one Cyrillic character occupies one cell"
        );
        view.add(Element::Rect(RectElement {
            x: 8,
            y: 32,
            width: 8,
            height: cell_height,
            color: Color::from_palette(0),
            filled: true,
        }));
        assert!(
            view.link_at(12, 32).is_none(),
            "later paint covers the link"
        );
    }
}

#[test]
fn terminal_detail_scroll_reaches_the_last_rendered_line_at_each_width() {
    for cols in [40, 60, 80, 120] {
        for resolution in [
            ResolutionMode::Vga,
            ResolutionMode::Cga,
            ResolutionMode::Svga,
        ] {
            let mut app = App::new_terminal();
            app.set_resolution(resolution);
            app.set_terminal_size(cols, 25);
            app.show_detail = true;
            for (index, project) in PROJECTS.iter().enumerate() {
                app.selected_project = index;
                app.detail_scroll = app.detail_max_scroll();
                let lines = project_detail_lines(project, cols.min(80) as usize, app.tick);
                let last = &lines.last().unwrap().0;
                assert!(app.render().elements.iter().any(|e| matches!(e, Element::Text(t) if t.y >= 7 * 16 && t.y < 21 * 16 && &t.text == last)), "project {index}, terminal {cols}, {resolution:?}");
            }
        }
    }
}

#[test]
fn test_all_systems_and_tabs_render_within_bounds() {
    for mode in SystemMode::ALL {
        let (w, h) = mode.resolution();
        let (cols, rows) = mode.char_grid();

        assert!(w > 0 && h > 0);
        assert!(cols > 0 && rows > 0);

        let mut app = App::new();
        app.resolution = mode.to_resolution();
        app.system_mode = mode;
        app.palette_mode = mode;

        for tab in [
            Tab::Projects,
            Tab::Links,
            Tab::Resume,
            Tab::About,
            Tab::Visuals,
            Tab::Contact,
            Tab::Help,
        ] {
            app.current_tab = tab;
            app.show_detail = false;

            let view = app.render();
            assert_eq!(view.width, w);
            assert_eq!(view.height, h);
            assert_eq!(view.system_mode, mode);

            for elem in &view.elements {
                match elem {
                    Element::Text(t) => {
                        assert!(
                            t.x < w,
                            "Text '{:?}' x={} out of bounds w={}",
                            t.text,
                            t.x,
                            w
                        );
                        assert!(
                            t.y < h,
                            "Text '{:?}' y={} out of bounds h={}",
                            t.text,
                            t.y,
                            h
                        );
                        let text_len = t.text.chars().count() as u16;
                        assert!(
                            t.x + text_len * 8 <= w + 8, // slight margin for right-aligned items
                            "Text '{:?}' (len {}) at x={} overflows w={} for mode {:?}",
                            t.text,
                            text_len,
                            t.x,
                            w,
                            mode
                        );
                    }
                    Element::Rect(r) => {
                        assert!(r.x <= w, "Rect x={} out of bounds w={}", r.x, w);
                        assert!(r.y <= h, "Rect y={} out of bounds h={}", r.y, h);
                    }
                    Element::Link(l) => {
                        assert!(
                            l.x < w,
                            "Link '{:?}' x={} out of bounds w={}",
                            l.text,
                            l.x,
                            w
                        );
                        assert!(
                            l.y < h,
                            "Link '{:?}' y={} out of bounds h={}",
                            l.text,
                            l.y,
                            h
                        );
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
fn system_key_and_footer_cycle_native_presets() {
    for initial in ResolutionMode::ALL {
        let mut app = App::new_web();
        app.set_resolution(initial);
        let next = initial.next();
        assert!(app.update(InputEvent::KeyDown(Key::Char('s'))));
        assert_eq!(app.resolution, next);
        assert_eq!(app.active_modal, ActiveModal::None);
        assert_eq!(app.system_mode, next.to_system_mode());
        let expected_palette = match next {
            ResolutionMode::Svga | ResolutionMode::Sga | ResolutionMode::Vga => {
                ColorPalette::VgaModern
            }
            ResolutionMode::Ega | ResolutionMode::Cga => ColorPalette::Ega,
            ResolutionMode::C64 => ColorPalette::C64,
            ResolutionMode::Atari => ColorPalette::Atari,
            ResolutionMode::ZxSpectrum => ColorPalette::ZxSpectrum,
        };
        assert_eq!(app.color_palette, expected_palette);
        assert_eq!(app.palette_mode, app.interface_theme.font_mode());

        let (width, height) = app.resolution.resolution();
        let nav_y = if app.resolution.char_grid().0 < 80 {
            height - 20
        } else {
            height - 32
        };
        assert!(app.update(InputEvent::PointerDown {
            x: width * 2 / 5 + 1,
            y: nav_y + 2,
            button: Button::Left,
        }));
        assert_eq!(app.resolution, next.next());
    }
}

#[test]
fn article_system_key_cycles_without_opening_selector() {
    let mut app = App::new_web();
    app.current_tab = Tab::Resume;
    assert!(app.update(InputEvent::KeyDown(Key::Char('s'))));
    assert_eq!(app.resolution, ResolutionMode::Ega);
    assert_eq!(app.active_modal, ActiveModal::None);
    assert_eq!(app.current_tab, Tab::Resume);
}

#[test]
fn test_terminal_mode_omits_visuals_tab_and_cycles_cleanly() {
    let mut app = App::new_terminal();
    assert_eq!(app.platform, Platform::Terminal);
    assert!(app.status.contains("S Cycle system"));

    // Render in 80 cols and verify no Visuals tab is in the header
    let view = app.render();
    for elem in &view.elements {
        if let Element::Text(t) = elem {
            assert!(
                !t.text.contains("Visuals"),
                "Terminal header unexpectedly contains Visuals: {}",
                t.text
            );
        }
    }

    // Digits have no global navigation assignment.
    assert!(!app.update(InputEvent::KeyDown(Key::Char('1'))));
    assert_eq!(app.current_tab, Tab::Projects);
    assert!(!app.update(InputEvent::KeyDown(Key::Char('6'))));
    assert_eq!(app.current_tab, Tab::Projects);

    // Tab cycling on Terminal should skip Visuals.
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

    assert!(app.update(InputEvent::KeyDown(Key::Tab)));
    assert_eq!(app.current_tab, Tab::Resume);

    // Test all system modes render within bounds for Terminal
    for mode in SystemMode::ALL {
        app.system_mode = mode;
        app.palette_mode = mode;
        for tab in [
            Tab::Projects,
            Tab::Links,
            Tab::Resume,
            Tab::About,
            Tab::Contact,
            Tab::Help,
        ] {
            app.current_tab = tab;
            let view = app.render();
            let (w, h) = (app.terminal_cols * 8, app.terminal_rows * 16);
            for elem in &view.elements {
                if let Element::Text(t) = elem {
                    assert!(
                        t.x < w,
                        "Terminal text '{:?}' x={} out of bounds w={}",
                        t.text,
                        t.x,
                        w
                    );
                    assert!(
                        t.y < h,
                        "Terminal text '{:?}' y={} out of bounds h={}",
                        t.text,
                        t.y,
                        h
                    );
                }
            }
        }
    }
}

#[test]
fn test_web_mode_modals_and_articles() {
    let mut app = App::new_web();
    assert_eq!(app.platform, Platform::Web);
    assert!(app.status.contains("P Projects"));

    // Render in 80 cols and verify the Projects entries and six-slot menu.
    let view = app.render();
    let mut found_projects = false;
    let mut found_cv = false;
    let mut found_vis = false;
    for elem in &view.elements {
        match elem {
            Element::Text(t) => {
                if t.text.contains("Projects") || t.text.contains("Prj") {
                    found_projects = true;
                }
                if t.text.contains("CV") {
                    found_cv = true;
                }
                if t.text.contains("Visual") {
                    found_vis = true;
                }
            }
            Element::Link(l) => {
                if l.text.contains("Visual") || l.text.contains("[VIS]") {
                    found_vis = true;
                }
            }
            _ => {}
        }
    }
    assert!(
        found_projects,
        "Web interface should contain Projects navigation"
    );
    assert!(found_cv, "Web interface should contain CV navigation");
    assert!(found_vis, "Web interface should contain Visual navigation");

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

    // CV is the first selectable row on Projects; digits have no global action.
    assert!(!app.update(InputEvent::KeyDown(Key::Char('2'))));
    assert!(app.update(InputEvent::KeyDown(Key::Enter)));
    assert_eq!(app.current_tab, Tab::Resume);
    // Pressing Escape returns to Projects
    assert!(app.update(InputEvent::KeyDown(Key::Escape)));
    assert_eq!(app.current_tab, Tab::Projects);

    // Tab reaches Links; Escape returns to Projects.
    assert!(app.update(InputEvent::KeyDown(Key::Tab)));
    assert_eq!(app.current_tab, Tab::Resume);
    assert!(app.update(InputEvent::KeyDown(Key::Tab)));
    assert_eq!(app.current_tab, Tab::About);
    assert!(app.update(InputEvent::KeyDown(Key::Tab)));
    assert_eq!(app.current_tab, Tab::Contact);
    assert!(app.update(InputEvent::KeyDown(Key::Tab)));
    assert_eq!(app.current_tab, Tab::Links);
    assert!(app.update(InputEvent::KeyDown(Key::Escape)));
    assert_eq!(app.current_tab, Tab::Projects);
}

#[test]
fn test_zx_spectrum_navpanel_no_overlapping_chars() {
    let mut app = App::new_web();
    app.set_resolution(ResolutionMode::ZxSpectrum);

    let view = app.render();
    let mut nav_items: Vec<(u16, u16, String)> = Vec::new();

    let nav_y = 192 - 20; // 172
    for elem in &view.elements {
        match elem {
            Element::Text(t) if t.y == nav_y => {
                let width = t.text.chars().count() as u16 * 8;
                nav_items.push((t.x, t.x + width, t.text.clone()));
            }
            _ => {}
        }
    }

    nav_items.sort_by_key(|item| item.0);
    assert!(
        nav_items.len() >= 5,
        "Expected at least 5 nav items, found {}",
        nav_items.len()
    );

    for i in 0..nav_items.len() - 1 {
        let (_start_a, end_a, ref text_a) = nav_items[i];
        let (start_b, _, ref text_b) = nav_items[i + 1];
        assert!(
            end_a <= start_b,
            "Navpanel items overlap in Web ZX Spectrum: '{text_a:?}' ends at {end_a} but '{text_b:?}' starts at {start_b}"
        );
    }

    // Also verify terminal bottom navigation bar text items do not overlap
    let term_app = App::new_terminal();
    let term_view = term_app.render();
    let mut term_nav_items: Vec<(u16, u16, String)> = Vec::new();
    let term_nav_y = (term_app.terminal_rows.saturating_sub(2)) * 16;
    for elem in &term_view.elements {
        match elem {
            Element::Text(t) if t.y == term_nav_y => {
                let width = t.text.chars().count() as u16 * 8;
                term_nav_items.push((t.x, t.x + width, t.text.clone()));
            }
            _ => {}
        }
    }
    term_nav_items.sort_by_key(|item| item.0);
    assert!(
        term_nav_items.len() >= 5,
        "Expected at least 5 term nav items, found {}",
        term_nav_items.len()
    );
    for i in 0..term_nav_items.len() - 1 {
        let (_start_a, end_a, ref text_a) = term_nav_items[i];
        let (start_b, _, ref text_b) = term_nav_items[i + 1];
        assert!(
            end_a <= start_b,
            "Navpanel items overlap in Terminal: '{text_a:?}' ends at {end_a} but '{text_b:?}' starts at {start_b}"
        );
    }
}

#[test]
fn native_zx_system_uses_its_own_geometry_and_colors() {
    let mut app = App::new_web();
    app.set_resolution(ResolutionMode::ZxSpectrum);
    assert_eq!(app.color_palette, ColorPalette::ZxSpectrum);
    assert_eq!(app.interface_theme, InterfaceTheme::ZxSpectrum);
    let projects = app.render();
    assert_eq!((projects.width, projects.height), (256, 192));
    app.current_tab = Tab::Resume;
    let article = app.render();
    assert_eq!((article.width, article.height), (256, 192));
}

#[test]
fn test_native_zx_content_stays_above_bottom_navigation() {
    let mut app = App::new_web();
    app.set_resolution(ResolutionMode::ZxSpectrum);

    let projects = app.render();
    let nav_y = projects.height - 20;
    let project_count = format!("/{}", PROJECTS.len());
    assert!(projects.elements.iter().any(|elem| matches!(elem,
        Element::Text(t) if t.y == 152 && t.text.contains(&project_count)
    )));
    assert!(!projects.elements.iter().any(|elem| matches!(elem,
        Element::Text(t) if t.y > 152 && t.y < nav_y && t.text.contains(&project_count)
    )));

    app.current_tab = Tab::Resume;
    let article = app.render();
    let bottom_border = article
        .elements
        .iter()
        .filter_map(|elem| match elem {
            Element::Text(t) if t.text.starts_with("╚") => Some(t.y),
            _ => None,
        })
        .max()
        .expect("article bottom border");
    assert!(
        bottom_border + 8 <= nav_y,
        "Article bottom overlaps ZX navigation"
    );
}

#[test]
fn test_zx_projects_use_two_rows_with_reachable_scroll_and_clicks() {
    let mut app = App::new_web();
    app.set_resolution(ResolutionMode::ZxSpectrum);

    assert_eq!(app.projects_max_visible(), 8);
    assert_eq!(app.projects_max_scroll(), PROJECTS.len() - 8);

    let view = app.render();
    for row in 0..4 {
        let title_y = 56 + row as u16 * 16;
        assert!(view.elements.iter().any(|element| matches!(element,
            Element::Text(text) if text.y == title_y && text.x == 2 && text.text.contains(&format!("[{:02}]", row + 1))
        )));
        assert!(view.elements.iter().any(|element| matches!(element,
            Element::Text(text) if text.y == title_y + 8 && text.x == 2 && text.text.starts_with("  <")
        )));
    }

    assert!(app.update(InputEvent::KeyDown(Key::End)));
    assert_eq!(app.selected_project, PROJECTS.len() - 1);
    assert_eq!(app.scroll_offset, app.projects_max_scroll());

    // The first and last title/tag rows in the final window select the first
    // and last of the eight visible projects. The position line is inert.
    assert!(app.update(InputEvent::PointerDown {
        x: 2,
        y: 24,
        button: Button::Left,
    }));
    assert_eq!(app.selected_project, PROJECTS.len() - 8);
    assert!(app.update(InputEvent::PointerDown {
        x: 2,
        y: 24 + 7 * 16 + 8,
        button: Button::Left,
    }));
    assert_eq!(app.selected_project, PROJECTS.len() - 1);
    assert!(!app.update(InputEvent::PointerDown {
        x: 2,
        y: 24 + 8 * 16,
        button: Button::Left,
    }));

    assert!(app.update(InputEvent::KeyDown(Key::Home)));
    assert!(app.update(InputEvent::Wheel { dx: 0, dy: 4 }));
    assert_eq!(app.selected_list_item, 4);
    assert!(app.update(InputEvent::Wheel { dx: 0, dy: -4 }));
    assert_eq!(app.selected_list_item, 0);
    for _ in 0..PROJECTS.len() + 2 {
        assert!(app.update(InputEvent::KeyDown(Key::Down)));
    }
    assert_eq!(app.selected_project, PROJECTS.len() - 1);
    assert_eq!(app.scroll_offset, app.projects_max_scroll());
}

#[test]
fn test_article_scroll_reaches_last_line_at_each_resolution() {
    for resolution in ResolutionMode::ALL {
        let mut app = App::new_web();
        app.set_resolution(resolution);
        let (cols, _) = resolution.char_grid();
        for tab in [Tab::Resume, Tab::About] {
            app.current_tab = tab;
            let lines = match (tab, cols) {
                (Tab::Resume, 100 | 80) => RESUME_LINES_80,
                (Tab::Resume, 40) => RESUME_LINES_40,
                (Tab::Resume, _) => RESUME_LINES_32,
                (_, 100 | 80) => ABOUT_LINES_80,
                (_, 40) => ABOUT_LINES_40,
                _ => ABOUT_LINES_32,
            };
            if tab == Tab::Resume {
                app.resume_scroll = app.resume_max_scroll();
            } else {
                app.about_scroll = app.about_max_scroll();
            }
            let last = lines.last().unwrap().0;
            let view = app.render();
            assert!(
                view.elements.iter().any(|elem| matches!(elem,
                    Element::Text(t) if t.text == last
                )),
                "Last {tab:?} line is unreachable in {resolution:?}"
            );
        }
    }
}

#[test]
fn test_project_row_click_matches_rendered_grid() {
    for resolution in [
        ResolutionMode::Svga,
        ResolutionMode::Sga,
        ResolutionMode::Vga,
        ResolutionMode::Ega,
    ] {
        let mut app = App::new_web();
        app.set_resolution(resolution);
        let y = (3 + 4 + 3 * 2 + 1) * resolution.line_height() + 1;
        assert!(app.update(InputEvent::PointerDown {
            x: 24,
            y,
            button: Button::Left
        }));
        assert_eq!(
            app.selected_project, 3,
            "Wrong project selected in {resolution:?}"
        );
    }
}

#[test]
fn compact_project_rows_fill_available_space_and_remain_clickable() {
    for (resolution, visible, last_tag_y) in [
        (ResolutionMode::C64, 7, 160),
        (ResolutionMode::ZxSpectrum, 6, 144),
    ] {
        let mut app = App::new_web();
        app.set_resolution(resolution);
        assert_eq!(app.projects_visible_at(0), visible);
        let view = app.render();
        assert!(view.elements.iter().any(|element| matches!(element,
            Element::Text(text) if text.y == last_tag_y && text.text.contains('<')
        )));
        assert!(app.update(InputEvent::PointerDown {
            x: 24,
            y: last_tag_y + 1,
            button: Button::Left,
        }));
        assert_eq!(app.selected_project, visible - 1);
    }
}

#[test]
fn tall_portrait_view_uses_extra_rows_without_stretching_columns() {
    for (resolution, height) in [
        (ResolutionMode::C64, 696),
        (ResolutionMode::ZxSpectrum, 552),
    ] {
        let mut app = App::new_web();
        app.set_resolution(resolution);
        app.set_web_height(Some(height));
        let view = app.render();
        assert_eq!(view.width, resolution.resolution().0);
        assert_eq!(view.height, height);
        assert_eq!(app.projects_visible_at(0), PROJECTS.len());

        let pitch = app.compact_project_pitch(0, resolution.char_grid().0);
        let last_y = 56 + (PROJECTS.len() as u16 - 1) * pitch;
        assert!(app.update(InputEvent::PointerDown {
            x: 24,
            y: last_y + 1,
            button: Button::Left,
        }));
        assert_eq!(app.selected_project, PROJECTS.len() - 1);

        app.set_web_height(None);
        assert_eq!(app.view_dimensions(), resolution.resolution());
        assert!(app.scroll_offset > 0);
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
fn test_all_20_projects_have_detailed_specs_and_subsystems() {
    use crate::data::details::get_project_detail;

    assert_eq!(PROJECTS.len(), 20, "Expected exactly 20 portfolio projects");

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
fn test_project_diagrams_have_distinct_layouts_at_each_width() {
    use crate::data::get_project_detail;
    use std::collections::HashSet;

    for select in [
        |d: &crate::data::ProjectDetail| d.diagram_80,
        |d: &crate::data::ProjectDetail| d.diagram_40,
        |d: &crate::data::ProjectDetail| d.diagram_32,
    ] {
        let layouts: HashSet<String> = PROJECTS
            .iter()
            .map(|project| {
                select(&get_project_detail(project.slug))
                    .iter()
                    .map(|line| {
                        line.chars()
                            .map(|ch| if ch.is_alphanumeric() { 'x' } else { ch })
                            .collect::<String>()
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            })
            .collect();

        assert!(
            layouts.len() >= 12,
            "Project diagrams have only {} distinct layouts",
            layouts.len()
        );
    }
}

#[test]
fn test_project_detail_lines_width_bounds_across_all_resolutions() {
    for p in PROJECTS {
        // SVGA uses the full 94-character content width.
        let lines_100 = project_detail_lines(p, 100, 0);
        assert!(lines_100
            .iter()
            .all(|(line, _, _)| line.chars().count() <= 94));
        assert!(lines_100
            .iter()
            .any(|(line, _, _)| line.starts_with('┌') && line.chars().count() == 94));

        // 80 columns: text must fit <= 74 chars
        let lines_80 = project_detail_lines(p, 80, 0);
        assert!(
            lines_80.len() > 15,
            "Project {} lines_80 too short ({})",
            p.slug,
            lines_80.len()
        );
        for (idx, (l, _, _)) in lines_80.iter().enumerate() {
            let char_count = l.chars().count();
            assert!(
                char_count <= 74,
                "Project {} 80-col line {} exceeds 74 chars ({}): '{}'",
                p.slug,
                idx,
                char_count,
                l
            );
        }

        // 40 columns: text must fit <= 38 chars
        let lines_40 = project_detail_lines(p, 40, 0);
        assert!(
            lines_40.len() > 15,
            "Project {} lines_40 too short ({})",
            p.slug,
            lines_40.len()
        );
        for (idx, (l, _, _)) in lines_40.iter().enumerate() {
            let char_count = l.chars().count();
            assert!(
                char_count <= 38,
                "Project {} 40-col line {} exceeds 38 chars ({}): '{}'",
                p.slug,
                idx,
                char_count,
                l
            );
        }

        // 32 columns: text must fit <= 29 chars
        let lines_32 = project_detail_lines(p, 32, 0);
        assert!(
            lines_32.len() > 15,
            "Project {} lines_32 too short ({})",
            p.slug,
            lines_32.len()
        );
        for (idx, (l, _, _)) in lines_32.iter().enumerate() {
            let char_count = l.chars().count();
            assert!(
                char_count <= 29,
                "Project {} 32-col line {} exceeds 29 chars ({}): '{}'",
                p.slug,
                idx,
                char_count,
                l
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
    assert!(
        max_scroll_80 > 0,
        "80-col detail view should be scrollable (max_scroll={max_scroll_80})"
    );

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
            "RESUME_LINES_80 line {idx} exceeds 74 chars ({char_count}): '{l}'"
        );
    }

    // 40 columns: text must fit <= 38 chars
    assert!(RESUME_LINES_40.len() >= 40);
    for (idx, (l, _, _)) in RESUME_LINES_40.iter().enumerate() {
        let char_count = l.chars().count();
        assert!(
            char_count <= 38,
            "RESUME_LINES_40 line {idx} exceeds 38 chars ({char_count}): '{l}'"
        );
    }

    // 32 columns: text must fit <= 29 chars
    assert!(RESUME_LINES_32.len() >= 40);
    for (idx, (l, _, _)) in RESUME_LINES_32.iter().enumerate() {
        let char_count = l.chars().count();
        assert!(
            char_count <= 29,
            "RESUME_LINES_32 line {idx} exceeds 29 chars ({char_count}): '{l}'"
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
    app.update(InputEvent::PointerDown {
        x: 200,
        y: 150,
        button: Button::Left,
    });
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
    assert_eq!(
        app.tick, initial_tick,
        "PointerMove events must not advance tick"
    );

    // Key presses and clicks must also not advance tick
    app.update(InputEvent::KeyDown(Key::Down));
    app.update(InputEvent::PointerDown {
        x: 50,
        y: 50,
        button: Button::Left,
    });
    assert_eq!(app.tick, initial_tick, "Input events must not advance tick");

    // Only dedicated tick() advances the animation clock
    app.tick();
    assert_eq!(
        app.tick,
        initial_tick + 1,
        "App::tick() should advance tick by 1"
    );
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

    assert!(
        row_ys.len() >= 6,
        "Expected at least 6 visible project rows on terminal, found {}",
        row_ys.len()
    );

    // Each consecutive project row must be spaced by exactly 16 pixels (1 character line = row + 1).
    // If there were 2-grouping, the deltas would alternate (e.g. 16, 32, 16, 32).
    for i in 0..row_ys.len() - 1 {
        let delta = row_ys[i + 1] - row_ys[i];
        assert_eq!(
            delta,
            16,
            "Project rows must be on consecutive lines! Row {} (y={}) and Row {} (y={}) delta={}",
            i,
            row_ys[i],
            i + 1,
            row_ys[i + 1],
            delta
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
    assert_eq!(max_vis, 45 - 6); // 39 visible items
}

#[test]
fn test_vga_project_cards_use_two_rows_and_fill_list_area() {
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

    assert!(
        row_ys.len() == 7,
        "Expected seven complete two-row project cards on VGA, found {}",
        row_ys.len()
    );

    for i in 0..row_ys.len() - 1 {
        let delta = row_ys[i + 1] - row_ys[i];
        assert_eq!(
            delta,
            32,
            "80-col project cards must use two lines. Card {} (y={}) and card {} (y={}) delta={}",
            i,
            row_ys[i],
            i + 1,
            row_ys[i + 1],
            delta
        );
    }
}

#[test]
fn wide_project_cards_show_complete_titles_tags_and_footer_labels() {
    for resolution in [
        ResolutionMode::Svga,
        ResolutionMode::Sga,
        ResolutionMode::Vga,
        ResolutionMode::Ega,
    ] {
        let mut app = App::new_web();
        app.set_resolution(resolution);
        let view = app.render();
        let (cols, rows) = resolution.char_grid();
        let lh = resolution.line_height();
        let count = app.projects_visible_at(0);
        for (index, project) in PROJECTS.iter().take(count).enumerate() {
            let title_y = (7 + index as u16 * 2) * lh;
            let tags = project
                .tags
                .iter()
                .map(|tag| format!("<{tag}>"))
                .collect::<Vec<_>>()
                .join(" ");
            assert!(view.elements.iter().any(|element| matches!(element,
                Element::Text(text) if text.x == 16 && text.y == title_y && text.text.contains(project.title)
            )));
            assert!(view.elements.iter().any(|element| matches!(element,
                Element::Text(text) if text.x == 48 && text.y == title_y + lh && text.text == tags
            )));
        }
        assert!((7 + count as u16 * 2) <= rows - 3);

        let slot_width = view.width / 6;
        let nav_y = view.height - 32;
        for (index, label) in ["Projects", "Help", "System", "Visuals", "Quit", "Links"]
            .iter()
            .enumerate()
        {
            let x = index as u16 * slot_width;
            let (hotkey, rest) = label.split_at(1);
            assert!(view.elements.iter().any(|element| matches!(element,
                Element::Text(text) if text.x == x + 3 && text.y == nav_y && text.text == hotkey && text.style.underline
            )));
            assert!(view.elements.iter().any(|element| matches!(element,
                Element::Text(text) if text.x == x + 11 && text.y == nav_y && text.text == rest
            )));
        }
        assert!(cols >= 80);
    }
}

#[test]
fn wide_project_list_can_scroll_to_the_last_card() {
    for resolution in [
        ResolutionMode::Svga,
        ResolutionMode::Sga,
        ResolutionMode::Vga,
        ResolutionMode::Ega,
    ] {
        let mut app = App::new_web();
        app.set_resolution(resolution);
        assert!(app.update(InputEvent::KeyDown(Key::End)));
        assert_eq!(app.selected_project, PROJECTS.len() - 1);
        assert_eq!(app.scroll_offset, app.projects_max_scroll());

        let view = app.render();
        let last_row = PROJECTS.len() - 1 - app.scroll_offset;
        let y = (3 + last_row as u16 * 2) * resolution.line_height();
        let last_card = format!("[{}] {}", PROJECTS.len(), PROJECTS.last().unwrap().title);
        assert!(view.elements.iter().any(|element| matches!(element,
            Element::Text(text) if text.x == 16 && text.y == y && text.text.contains(&last_card)
        )));
        assert!(y + 2 * resolution.line_height() <= view.height - 3 * resolution.line_height());
    }
}

#[test]
fn test_80_col_all_elements_aligned_to_16px_grid() {
    for tab in [
        Tab::Projects,
        Tab::Links,
        Tab::Resume,
        Tab::About,
        Tab::Visuals,
        Tab::Contact,
        Tab::Help,
    ] {
        let mut app = App::new();
        app.current_tab = tab;
        let view = app.render();

        for elem in &view.elements {
            match elem {
                Element::Text(t) => {
                    assert_eq!(
                        t.y % 16,
                        0,
                        "Text '{}' at y={} in tab {:?} is not aligned to 16px character row grid!",
                        t.text,
                        t.y,
                        tab
                    );
                }
                Element::Link(l) => {
                    assert_eq!(
                        l.y % 16,
                        0,
                        "Link '{}' at y={} in tab {:?} is not aligned to 16px character row grid!",
                        l.text,
                        l.y,
                        tab
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
    assert!(ABOUT_LINES_80
        .iter()
        .any(|(t, _, _)| t.contains("DMYTRO YEMELIANOV")));
    assert!(ABOUT_LINES_80
        .iter()
        .any(|(t, _, _)| t.contains("ARCHITECT")));
    assert!(ABOUT_LINES_80
        .iter()
        .any(|(t, _, _)| t.contains("┌────────────┐")));

    assert!(ABOUT_LINES_40
        .iter()
        .any(|(t, _, _)| t.contains("DMYTRO YEMELIANOV")));
    assert!(ABOUT_LINES_40
        .iter()
        .any(|(t, _, _)| t.contains("┌────────┐")));

    assert!(ABOUT_LINES_32
        .iter()
        .any(|(t, _, _)| t.contains("DMYTRO YEMELIANOV")));
    assert!(ABOUT_LINES_32
        .iter()
        .any(|(t, _, _)| t.contains("┌────────┐")));

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
    assert!(
        found_sprite,
        "About article on Web must render dithered profile sprite"
    );

    // 3. Verify get_profile_sprite returns valid data for all 10 system modes
    for mode in SystemMode::ALL {
        let s96 = crate::data::profile::get_profile_sprite(mode, 96);
        assert_eq!(s96.len(), 96 * 96);
        let s64 = crate::data::profile::get_profile_sprite(mode, 64);
        assert_eq!(s64.len(), 64 * 64);
    }
}

#[test]
fn test_modal_and_article_strict_event_capture() {
    let mut app = App::new_web();
    app.current_tab = Tab::Projects;
    app.selected_project = 0;

    // Help captures navigation while open.
    app.active_modal = ActiveModal::Help;
    app.update(InputEvent::KeyDown(Key::Down));
    assert_eq!(app.selected_project, 0);
    app.update(InputEvent::KeyDown(Key::Tab));
    assert_eq!(app.current_tab, Tab::Projects);
    app.update(InputEvent::KeyDown(Key::Escape));
    assert_eq!(app.active_modal, ActiveModal::None);

    // 2. Open Visuals modal
    app.active_modal = ActiveModal::Visuals;
    app.update(InputEvent::KeyDown(Key::Char('2'))); // Trinitron preset
    assert_eq!(app.status, "Preset: 80s Trinitron CRT");
    assert_eq!(app.active_modal, ActiveModal::Visuals);

    // Escape closes Visuals modal
    app.update(InputEvent::KeyDown(Key::Escape));
    assert_eq!(app.active_modal, ActiveModal::None);

    // 3. Mouse Wheel while modal is active must not scroll background
    app.active_modal = ActiveModal::Help;
    app.update(InputEvent::Wheel { dx: 0, dy: 5 });
    assert_eq!(
        app.selected_project, 0,
        "Wheel must not scroll background projects when modal is active"
    );
    app.update(InputEvent::KeyDown(Key::Escape));
    assert_eq!(app.active_modal, ActiveModal::None);

    // 4. Click outside modal dismisses it without clicking background
    app.active_modal = ActiveModal::Help;
    app.update(InputEvent::PointerDown {
        x: 5,
        y: 5,
        button: Button::Left,
    });
    assert_eq!(app.active_modal, ActiveModal::None);
    assert_eq!(app.current_tab, Tab::Projects);
}

#[test]
fn test_volkov_commander_clock_badge_without_forcing_panel_layout() {
    let mut app = App::new();
    app.platform = Platform::Web;
    app.resolution = ResolutionMode::Vga;
    app.color_palette = ColorPalette::Commander;
    app.set_time(14, 30, 0);

    let view = app.render();

    // 1. Volkov Commander clock badge at top right
    let mut found_clock_bg = false;
    let mut found_clock_text = false;
    let clock_x = 640 - 60;

    for elem in &view.elements {
        match elem {
            Element::Rect(r)
                if r.x == clock_x && r.y == 0 && r.width == 60 && r.height == 16 && r.filled =>
            {
                assert_eq!(
                    r.color.palette_index, 13,
                    "Clock badge background must be black (index 13)"
                );
                found_clock_bg = true;
            }
            Element::Text(t) if t.y == 0 && t.text.contains("14:30") => {
                assert_eq!(
                    t.style.fg.palette_index, 6,
                    "Clock text must be Volkov yellow (index 6)"
                );
                found_clock_text = true;
            }
            _ => {}
        }
    }
    assert!(
        found_clock_bg,
        "Volkov Commander clock background badge missing at top right"
    );
    assert!(found_clock_text, "Volkov Commander clock text missing");

    assert_eq!(app.resolution, ResolutionMode::Vga);
    assert_eq!(app.interface_theme, InterfaceTheme::Modern);
}

#[test]
fn system_change_applies_native_font_and_palette() {
    let mut app = App::new_web();
    for resolution in ResolutionMode::ALL {
        app.set_resolution(resolution);
        assert_eq!(app.palette_mode, app.interface_theme.font_mode());
        assert_eq!(app.system_mode, resolution.to_system_mode());
    }
    app.set_resolution(ResolutionMode::Vga);
    assert_eq!(app.color_palette, ColorPalette::VgaModern);
    assert_eq!(app.interface_theme, InterfaceTheme::Modern);
}

#[test]
fn compact_views_keep_text_inside_screen() {
    let mut overflows = Vec::new();
    for resolution in [
        ResolutionMode::Cga,
        ResolutionMode::C64,
        ResolutionMode::Atari,
        ResolutionMode::ZxSpectrum,
    ] {
        for modal in [ActiveModal::None, ActiveModal::Visuals, ActiveModal::Help] {
            let mut app = App::new_web();
            app.set_resolution(resolution);
            app.active_modal = modal;
            for (tab, detail, last_page) in [
                (Tab::Projects, false, false),
                (Tab::Projects, false, true),
                (Tab::Projects, true, false),
                (Tab::Projects, true, true),
                (Tab::Resume, false, false),
                (Tab::Resume, false, true),
                (Tab::About, false, false),
                (Tab::About, false, true),
            ] {
                app.current_tab = tab;
                app.show_detail = detail;
                app.selected_project = PROJECTS.len() - 1;
                app.scroll_offset = if last_page {
                    app.projects_max_scroll()
                } else {
                    0
                };
                app.detail_scroll = if last_page {
                    app.detail_max_scroll()
                } else {
                    0
                };
                app.resume_scroll = if last_page {
                    app.resume_max_scroll()
                } else {
                    0
                };
                app.about_scroll = if last_page { app.about_max_scroll() } else { 0 };
                let view = app.render();
                for element in &view.elements {
                    let (x, y, count, label) = match element {
                        Element::Text(t) => (t.x, t.y, t.text.chars().count(), t.text.as_str()),
                        Element::Link(l) => (l.x, l.y, l.text.chars().count(), l.text.as_str()),
                        _ => continue,
                    };
                    if x as usize + count * 8 > view.width as usize {
                        overflows.push(format!(
                            "{resolution:?} {modal:?} {tab:?}: ({x},{y}) {label}"
                        ));
                    }
                }
            }
        }
    }
    assert!(overflows.is_empty(), "{}", overflows.join("\n"));
}

#[test]
fn compact_visuals_controls_reach_last_property_and_show_preset() {
    for resolution in [ResolutionMode::Cga, ResolutionMode::ZxSpectrum] {
        let mut app = App::new_web();
        app.set_resolution(resolution);
        app.active_modal = ActiveModal::Visuals;
        let geometry = app.standard_dialog_geometry();
        let view = app.render();
        assert!(view.elements.iter().any(|element| matches!(element,
            Element::Text(text) if text.text.contains("PRESET: Default")
        )));
        assert!(app.update(InputEvent::KeyDown(Key::Up)));
        assert_eq!(app.selected_fx_slider, 8);
        assert!(app.update(InputEvent::KeyDown(Key::Right)));
        assert!(app.visual_effects.noise > 0.0);
        assert!(app.update(InputEvent::PointerDown {
            x: geometry.box_x + 9,
            y: geometry.box_y + 12 * geometry.char_h + 1,
            button: Button::Left,
        }));
        assert_eq!(app.visual_effects, VisualEffects::clean());
        assert!(app.update(InputEvent::KeyDown(Key::Char('s'))));
        assert_eq!(app.resolution, resolution.next());
        assert_eq!(app.active_modal, ActiveModal::None);
    }
}

#[test]
fn wide_visuals_pointer_matches_rendered_rows() {
    let mut app = App::new_web();
    app.active_modal = ActiveModal::Visuals;
    let geometry = app.standard_dialog_geometry();
    assert!(app.update(InputEvent::PointerDown {
        x: geometry.box_x + 144 + 56,
        y: geometry.box_y + 11 * geometry.char_h + 1,
        button: Button::Left,
    }));
    assert_eq!(app.selected_fx_slider, 8);
    assert!((app.visual_effects.noise - 0.5).abs() < 0.01);
    assert!(app.update(InputEvent::PointerDown {
        x: geometry.box_x + 180,
        y: geometry.box_y + geometry.char_h + 1,
        button: Button::Left,
    }));
    assert_eq!(app.visual_effects, VisualEffects::crt_trinitron());
}

#[test]
fn wide_project_viewer_fills_available_height_and_tracks_clicks() {
    for resolution in [
        ResolutionMode::Svga,
        ResolutionMode::Sga,
        ResolutionMode::Vga,
        ResolutionMode::Ega,
    ] {
        let mut app = App::new_web();
        app.set_resolution(resolution);
        app.show_detail = true;
        app.selected_project = 0;
        let view = app.render();
        let lh = resolution.line_height();
        let visible = app.detail_wide_visible_rows();
        let nav_y = view.height - 32;
        let return_y = (8 + visible as u16) * lh;
        assert!(
            return_y + 2 * lh <= nav_y,
            "{resolution:?}: return and indicator must stay above navigation"
        );
        assert!(view.elements.iter().any(|element| matches!(element,
            Element::Rect(rect) if rect.x == 0 && rect.y == 7 * lh && rect.width == view.width
                && rect.y + rect.height == nav_y
        )));
        assert!(view.elements.iter().any(|element| matches!(element,
            Element::Link(link) if link.url == "#back" && link.y == return_y
        )));
        let bottom_arrow = (8 + visible as u16 - 1) * lh;
        assert!(app.update(InputEvent::PointerDown {
            x: view.width - 12,
            y: bottom_arrow + 1,
            button: Button::Left,
        }));
        assert_eq!(app.detail_scroll, 2, "{resolution:?}: scrollbar arrow");
        assert!(app.update(InputEvent::PointerDown {
            x: 24,
            y: return_y + 1,
            button: Button::Left,
        }));
        assert!(!app.show_detail, "{resolution:?}: return row");
    }
}

#[test]
fn wide_detail_shows_live_demo_link_and_tab_cycles_three_items() {
    let mut app = App::new();
    app.current_tab = Tab::Projects;
    app.show_detail = true;
    app.selected_project = PROJECTS.iter().position(|p| p.slug == "autocaded").unwrap();
    assert_eq!(app.detail_item_count(), 3);

    let links = |app: &App| -> Vec<(String, String)> {
        app.render()
            .elements
            .iter()
            .filter_map(|e| match e {
                Element::Link(l) => Some((l.text.clone(), l.url.clone())),
                _ => None,
            })
            .collect()
    };
    let shown = links(&app);
    assert!(shown
        .iter()
        .any(|(text, url)| url == "https://autocaded.yemelianov.dev"
            && text.contains("Live demo: autocaded.yemelianov.dev")));
    assert!(shown
        .iter()
        .any(|(_, url)| url == "https://github.com/dmytro-yemelianov/autocaded"));

    // Tab: repository -> live demo -> Return -> repository.
    for expected in [1, 2, 0] {
        app.update(InputEvent::KeyDown(Key::Tab));
        assert_eq!(app.selected_detail_item, expected);
    }
    assert!(
        links(&app)
            .iter()
            .any(|(t, u)| u.ends_with("/autocaded") && t.starts_with('►')),
        "repository focused"
    );
    // Enter on Return (the last item) closes the detail page.
    app.update(InputEvent::KeyDown(Key::Tab));
    app.update(InputEvent::KeyDown(Key::Tab));
    app.update(InputEvent::KeyDown(Key::Enter));
    assert!(!app.show_detail);

    // Projects without a demo keep two items.
    app.show_detail = true;
    app.selected_detail_item = 0;
    app.selected_project = 0;
    assert_eq!(PROJECTS[0].demo, None);
    assert_eq!(app.detail_item_count(), 2);
    assert!(!links(&app).iter().any(|(t, _)| t.contains("Live demo")));
}

#[test]
fn touch_drag_moves_the_row_selector_in_the_finger_direction() {
    for resolution in [
        ResolutionMode::Vga,
        ResolutionMode::Cga,
        ResolutionMode::ZxSpectrum,
    ] {
        let mut app = App::new_web();
        app.set_resolution(resolution);
        assert_eq!(app.scroll_offset, 0);
        assert!(!app.update(InputEvent::TouchDrag { dy: 1 }));
        assert!(!app.update(InputEvent::TouchDrag { dy: 0 }));
        assert!(app.update(InputEvent::TouchDrag { dy: -1 }));
        assert_eq!(app.selected_list_item, 1, "drag down selects Contacts");
        assert_eq!(app.scroll_offset, 0, "{resolution:?}");
        assert!(app.update(InputEvent::TouchDrag { dy: 1 }));
        assert_eq!(app.selected_list_item, 0, "drag up selects CV");
        for _ in 0..100 {
            app.update(InputEvent::TouchDrag { dy: -1 });
        }
        assert_eq!(app.selected_list_item, PROJECTS.len() + 2);
        assert_eq!(app.selected_project, PROJECTS.len() - 1);
        assert_eq!(app.scroll_offset, app.projects_max_scroll());
        assert!(
            !app.update(InputEvent::TouchDrag { dy: -1 }),
            "stops at the end"
        );
        for _ in 0..100 {
            app.update(InputEvent::TouchDrag { dy: 1 });
        }
        assert_eq!(app.selected_list_item, 0);
        assert_eq!(app.scroll_offset, 0);
    }
}

#[test]
fn touch_drag_on_a_detail_page_scrolls_like_the_wheel() {
    let mut a = App::new_web();
    let mut b = App::new_web();
    for app in [&mut a, &mut b] {
        app.current_tab = Tab::Projects;
        app.show_detail = true;
    }
    a.update(InputEvent::TouchDrag { dy: 1 });
    b.update(InputEvent::Wheel { dx: 0, dy: 1 });
    assert_eq!(a.detail_scroll, b.detail_scroll);
    assert_eq!(a.detail_scroll, 1);
    a.update(InputEvent::TouchDrag { dy: -1 });
    b.update(InputEvent::Wheel { dx: 0, dy: -1 });
    assert_eq!(a.detail_scroll, b.detail_scroll);
    assert_eq!(a.detail_scroll, 0);
}

#[test]
fn native_routes_select_projects_and_embedded_chapters_without_html() {
    let mut app = App::new_web();
    assert!(app.open_route("/?project=vpa&detail=1"));
    assert_eq!(PROJECTS[app.selected_project].slug, "vpa");
    assert_eq!(app.selected_list_item, app.selected_project + 3);
    assert!(app.show_detail);
    assert!(app.open_route("/?tab=projects"));
    assert!(!app.show_detail);
    assert!(!app.open_route("/?project=missing"));
    assert!(!app.open_route("/?doc=missing"));
    assert!(!app.open_route("https://yemelianov.dev.evil/?tab=projects"));
    for resolution in ResolutionMode::ALL {
        app.set_resolution(resolution);
        for (i, (id, _, _)) in crate::documents::DOCUMENTS.iter().enumerate() {
            assert!(app.open_route(&format!("https://yemelianov.dev/?doc={id}")));
            assert_eq!(app.document, Some(i));
            assert_eq!(app.current_tab, Tab::About);
            assert!(!app.document_lines.is_empty());
            assert!(app
                .document_lines
                .iter()
                .all(|(s, _, _)| s.chars().count() <= resolution.char_grid().0 as usize - 8));
            app.update(InputEvent::KeyDown(Key::End));
            assert_eq!(app.about_scroll, app.about_max_scroll());
            let view = app.render();
            let tail = &app.document_lines.last().unwrap().0;
            assert!(view
                .elements
                .iter()
                .any(|e| matches!(e, Element::Text(t) if &t.text == tail)));
            assert!(!view
                .elements
                .iter()
                .any(|e| matches!(e, Element::Sprite(_))));
            app.update(InputEvent::KeyDown(Key::Escape));
            assert_eq!(app.current_tab, Tab::Projects);
        }
    }
    app.open_route("/?tab=about");
    assert_eq!(app.document, None);
    assert_eq!(app.about_scroll, 0);
}

#[test]
fn header_link_title_and_clock_do_not_overlap_at_any_width() {
    for resolution in ResolutionMode::ALL {
        for platform in [Platform::Web, Platform::Terminal] {
            let mut app = App::new_web();
            app.platform = platform;
            app.set_resolution(resolution);
            if platform == Platform::Terminal {
                app.set_terminal_size(40, 25);
            }
            app.set_time(23, 59, 0);
            let view = app.render();
            let link = view
                .elements
                .iter()
                .find_map(|e| match e {
                    Element::Link(l) if l.y <= 1 && l.url == "https://yemelianov.dev" => Some(l),
                    _ => None,
                })
                .unwrap();
            let end = link.x + link.text.chars().count() as u16 * 8;
            let clock = view
                .elements
                .iter()
                .find_map(|e| match e {
                    Element::Text(t) if t.y <= 1 && t.text.contains("23:59") => Some(t),
                    _ => None,
                })
                .unwrap();
            assert!(end + 8 <= clock.x, "{resolution:?} {platform:?}");
            for element in &view.elements {
                if let Element::Rect(r) = element {
                    if r.y == 0 && r.color.palette_index == 13 {
                        assert!(end <= r.x);
                    }
                }
                if let Element::Text(t) = element {
                    if t.y <= 1 && t.text.contains("DMYTRO") {
                        assert!(t.x + t.text.chars().count() as u16 * 8 <= link.x);
                    }
                }
            }
            assert_eq!(view.link_at(link.x + 4, link.y + 2).unwrap().url, link.url);
        }
    }
}

#[test]
fn delayed_screensaver_ticks_match_regular_frames() {
    let mut delayed = App::new_web();
    let mut regular = App::new_web();
    delayed.start_screensaver();
    regular.start_screensaver();
    for ticks in [1, 299, 601, 18007] {
        delayed.advance_ticks(ticks);
        for _ in 0..ticks {
            regular.tick();
        }
        assert_eq!(delayed.tick, regular.tick);
        assert_eq!(delayed.screensaver_tick, regular.screensaver_tick);
        assert_eq!(delayed.screensaver_mode, regular.screensaver_mode);
    }
}

#[test]
fn screensaver_rotates_and_wakes_without_changing_the_saved_view_or_effects() {
    use crate::views::screensaver::Screensaver;
    let mut app = App::new_web();
    app.selected_project = 5;
    app.show_detail = true;
    app.visual_effects = VisualEffects::retro_glitch();
    let effects = app.visual_effects;
    app.start_screensaver();
    assert_eq!(app.screensaver_mode, Screensaver::Rain);
    for _ in 0..300 {
        app.tick();
    }
    assert_eq!(app.screensaver_mode, Screensaver::Clock);
    assert_eq!(app.screensaver_tick, 0);
    app.update(InputEvent::KeyDown(Key::Escape));
    assert!(!app.screensaver_active);
    assert_eq!(app.selected_project, 5);
    assert!(app.show_detail);
    assert_eq!(app.visual_effects, effects);
    app.start_screensaver();
    assert_eq!(app.screensaver_mode, Screensaver::Orbits);
    app.update(InputEvent::PointerDown {
        x: 80,
        y: 80,
        button: Button::Left,
    });
    assert!(!app.screensaver_active);
}
