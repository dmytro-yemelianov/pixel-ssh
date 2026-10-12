//! Shareable browser state, independent of the DOM and animation clock.
use pixel_ssh_core::{documents::DOCUMENTS, views::screensaver::Screensaver, App, Tab, PROJECTS};
use pixel_ssh_view::{ActiveModal, ResolutionMode, VisualEffects};

pub const OWNED_PARAMS: &[&str] = &[
    "tab",
    "doc",
    "project",
    "detail",
    "row",
    "offset",
    "link",
    "item",
    "scroll",
    "modal",
    "system",
    "mode",
    "resolution",
    "preset",
    "fx",
    "slider",
    "screensaver",
    "saver",
    "magnet",
    "noise",
    "hum",
];

#[derive(Clone, Debug, PartialEq)]
pub struct Route {
    tab: Tab,
    document: Option<usize>,
    project: usize,
    row: usize,
    detail: bool,
    offset: usize,
    link: usize,
    item: usize,
    scroll: usize,
    modal: ActiveModal,
    resolution: ResolutionMode,
    effects: VisualEffects,
    slider: usize,
    saver: Option<Screensaver>,
}

impl Route {
    pub fn capture(app: &App) -> Self {
        let detail = app.current_tab == Tab::Projects && app.show_detail;
        Self {
            tab: app.current_tab,
            document: (app.current_tab == Tab::About)
                .then_some(app.document)
                .flatten(),
            project: if app.current_tab == Tab::Projects && (detail || app.selected_list_item >= 3)
            {
                app.selected_project
            } else {
                0
            },
            row: if app.current_tab == Tab::Projects && !detail {
                app.selected_list_item
            } else {
                0
            },
            detail,
            offset: match app.current_tab {
                Tab::Projects if !detail => app.scroll_offset,
                Tab::Links => app.links_scroll,
                _ => 0,
            },
            link: if app.current_tab == Tab::Links {
                app.selected_link
            } else {
                0
            },
            item: if detail { app.selected_detail_item } else { 0 },
            scroll: match app.current_tab {
                Tab::Projects if detail => app.detail_scroll,
                Tab::Resume => app.resume_scroll,
                Tab::About => app.about_scroll,
                _ => 0,
            },
            modal: app.active_modal,
            resolution: app.resolution,
            effects: app.visual_effects,
            slider: if app.active_modal == ActiveModal::Visuals {
                app.selected_fx_slider
            } else {
                0
            },
            saver: app.screensaver_active.then_some(app.screensaver_mode),
        }
    }

    /// DOM callers use URLSearchParams.get, which handles percent-encoded values.
    pub fn read(get: impl Fn(&str) -> Option<String>, default_resolution: ResolutionMode) -> Self {
        let mut route = Self::capture(&App::new_web());
        let number = |key| get(key).and_then(|v| v.parse::<usize>().ok()).unwrap_or(0);
        route.resolution = ["system", "mode", "resolution"]
            .iter()
            .find_map(|key| {
                get(key).and_then(|v| {
                    ResolutionMode::ALL
                        .into_iter()
                        .find(|r| r.short_name().eq_ignore_ascii_case(&v))
                })
            })
            .unwrap_or(default_resolution);
        route.tab = match get("tab").as_deref() {
            Some("links") => Tab::Links,
            Some("resume") => Tab::Resume,
            Some("about") => Tab::About,
            Some("contact") => Tab::Contact,
            _ => Tab::Projects,
        };
        if let Some(index) = get("doc").and_then(|id| DOCUMENTS.iter().position(|d| d.0 == id)) {
            route.tab = Tab::About;
            route.document = Some(index);
        } else if let Some(index) = get("project").and_then(|id| {
            PROJECTS
                .iter()
                .position(|p| p.slug == id)
                .or_else(|| id.parse::<usize>().ok().filter(|i| *i < PROJECTS.len()))
        }) {
            route.tab = Tab::Projects;
            route.project = index;
            route.row = index + 3;
            route.detail = get("detail").as_deref() == Some("1");
        } else {
            route.row = number("row").min(PROJECTS.len() + 2);
            if route.row >= 3 {
                route.project = route.row - 3;
            }
        }
        route.modal = match get("modal").or_else(|| get("tab")).as_deref() {
            Some("visuals") => ActiveModal::Visuals,
            Some("help") => ActiveModal::Help,
            _ => ActiveModal::None,
        };
        route.offset = number("offset");
        route.link = number("link");
        route.item = number("item");
        route.scroll = number("scroll");
        route.slider = number("slider").min(8);
        route.effects = match get("preset").as_deref() {
            Some("clean") => VisualEffects::clean(),
            Some("crt" | "trinitron") => VisualEffects::crt_trinitron(),
            Some("arcade") => VisualEffects::crt_arcade(),
            Some("bloom") => VisualEffects::phosphor_bloom(),
            Some("glitch") => VisualEffects::retro_glitch(),
            _ => VisualEffects::default(),
        };
        let bounded = |v: &str| {
            v.parse::<f32>()
                .ok()
                .filter(|n| n.is_finite())
                .map(|n| n.clamp(0.0, 1.0))
        };
        if let Some(fx) = get("fx") {
            let values: Option<Vec<_>> = fx.split(',').map(bounded).collect();
            if let Some(v) = values.filter(|v| v.len() == 9) {
                route.effects = VisualEffects {
                    scanlines: v[0],
                    pixel_grid: v[1],
                    chromatic: v[2],
                    afterglow: v[3],
                    curvature: v[4],
                    jitter: v[5],
                    magnet: v[6],
                    noise: v[7],
                    antenna_hum: v[8],
                };
            }
        }
        for (key, value) in [
            ("magnet", &mut route.effects.magnet),
            ("noise", &mut route.effects.noise),
            ("hum", &mut route.effects.antenna_hum),
        ] {
            if let Some(n) = get(key).and_then(|v| bounded(&v)) {
                *value = n;
            }
        }
        if get("screensaver").as_deref() == Some("1") {
            route.saver = Some(
                get("saver")
                    .and_then(|name| {
                        Screensaver::ALL
                            .into_iter()
                            .find(|mode| saver_id(*mode) == name)
                    })
                    .unwrap_or_default(),
            );
        }
        route
    }

    pub fn apply(&self, app: &mut App, web_height: Option<u16>) {
        app.set_resolution(self.resolution);
        app.set_web_height(web_height);
        app.selected_project = self.project;
        let target = if let Some(doc) = self.document {
            format!("/?doc={}", DOCUMENTS[doc].0)
        } else if self.tab == Tab::Projects && (self.detail || self.row >= 3) {
            format!(
                "/?project={}&detail={}",
                PROJECTS[self.project].slug,
                usize::from(self.detail)
            )
        } else {
            format!("/?tab={}", tab_id(self.tab))
        };
        app.open_route(&target);
        app.show_detail = self.detail;
        if self.tab == Tab::Projects {
            app.selected_list_item = if self.detail {
                self.project + 3
            } else {
                self.row
            };
        }
        app.scroll_offset = self.offset.min(PROJECTS.len());
        app.ensure_selected_project_visible();
        app.links_scroll = self.offset;
        app.select_link(self.link);
        app.selected_detail_item = self.item.min(app.detail_item_count().saturating_sub(1));
        app.detail_scroll = self.scroll.min(app.detail_max_scroll());
        app.resume_scroll = self.scroll.min(app.resume_max_scroll());
        app.about_scroll = self.scroll.min(app.about_max_scroll());
        app.active_modal = self.modal;
        app.visual_effects = self.effects;
        app.selected_fx_slider = self.slider;
        app.screensaver_active = self.saver.is_some();
        if let Some(mode) = self.saver {
            app.screensaver_mode = mode;
            app.screensaver_tick = 0;
        }
    }

    pub fn resolution(&self) -> ResolutionMode {
        self.resolution
    }

    /// Cursor, scroll, effects edits and automatic saver rotation replace the current entry.
    pub fn changes_screen(&self, previous: &Self) -> bool {
        self.tab != previous.tab
            || self.document != previous.document
            || self.detail != previous.detail
            || (self.detail && self.project != previous.project)
            || self.modal != previous.modal
            || self.saver.is_some() != previous.saver.is_some()
    }

    pub fn params(&self) -> Vec<(&'static str, String)> {
        let mut params = Vec::new();
        if let Some(doc) = self.document {
            params.push(("doc", DOCUMENTS[doc].0.to_owned()));
        } else if self.tab == Tab::Projects && (self.detail || self.row >= 3) {
            params.push(("project", PROJECTS[self.project].slug.to_owned()));
            if self.detail {
                params.push(("detail", "1".to_owned()));
            }
        } else {
            params.push(("tab", tab_id(self.tab).to_owned()));
            if self.tab == Tab::Projects && self.row > 0 {
                params.push(("row", self.row.to_string()));
            }
        }
        if self.offset > 0 {
            params.push(("offset", self.offset.to_string()));
        }
        if self.tab == Tab::Links && self.link > 0 {
            params.push(("link", self.link.to_string()));
        }
        if self.item > 0 {
            params.push(("item", self.item.to_string()));
        }
        if self.scroll > 0 {
            params.push(("scroll", self.scroll.to_string()));
        }
        if self.modal != ActiveModal::None {
            params.push((
                "modal",
                if self.modal == ActiveModal::Help {
                    "help"
                } else {
                    "visuals"
                }
                .to_owned(),
            ));
        }
        params.push(("system", self.resolution.short_name().to_ascii_lowercase()));
        params.push(("preset", self.effects.preset_name().to_ascii_lowercase()));
        if self.effects.preset_name() == "Custom" {
            let fx = self.effects;
            params.push((
                "fx",
                [
                    fx.scanlines,
                    fx.pixel_grid,
                    fx.chromatic,
                    fx.afterglow,
                    fx.curvature,
                    fx.jitter,
                    fx.magnet,
                    fx.noise,
                    fx.antenna_hum,
                ]
                .map(|n| n.to_string())
                .join(","),
            ));
        }
        if self.slider > 0 {
            params.push(("slider", self.slider.to_string()));
        }
        if let Some(saver) = self.saver {
            params.push(("screensaver", "1".to_owned()));
            params.push(("saver", saver_id(saver).to_owned()));
        }
        params
    }
}

fn tab_id(tab: Tab) -> &'static str {
    match tab {
        Tab::Projects => "projects",
        Tab::Links => "links",
        Tab::Resume => "resume",
        Tab::About => "about",
        Tab::Contact => "contact",
        Tab::Visuals => "visuals",
        Tab::Help => "help",
    }
}
fn saver_id(mode: Screensaver) -> &'static str {
    match mode {
        Screensaver::Starfield => "starfield",
        Screensaver::Rain => "rain",
        Screensaver::Clock => "clock",
        Screensaver::Orbits => "orbits",
        Screensaver::Swarm => "swarm",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pixel_ssh_view::{InputEvent, Key};

    fn round_trip(app: &App) {
        let expected = Route::capture(app);
        let params = expected.params();
        let route = Route::read(
            |key| {
                params
                    .iter()
                    .find(|(k, _)| *k == key)
                    .map(|(_, v)| v.clone())
            },
            ResolutionMode::ZxSpectrum,
        );
        let mut restored = App::new_web();
        route.apply(&mut restored, app.web_height);
        assert_eq!(Route::capture(&restored), expected);
    }

    #[test]
    fn shared_urls_restore_visible_state_in_every_system_and_portrait_layout() {
        for mode in ResolutionMode::ALL {
            for height in [None, Some(720)] {
                let mut app = App::new_web();
                app.set_resolution(mode);
                app.set_web_height(height);
                round_trip(&app);
                app.selected_list_item = 2;
                round_trip(&app);
                app.update(InputEvent::KeyDown(Key::End));
                round_trip(&app);
                app.update(InputEvent::KeyDown(Key::Enter));
                app.update(InputEvent::KeyDown(Key::End));
                app.selected_detail_item = app.detail_item_count() - 1;
                round_trip(&app);
                app.open_route("/?tab=links");
                app.select_link(18);
                round_trip(&app);
                for tab in ["resume", "about", "contact"] {
                    app.open_route(&format!("/?tab={tab}"));
                    app.update(InputEvent::KeyDown(Key::End));
                    round_trip(&app);
                }
                for (doc, _, _) in DOCUMENTS {
                    app.open_route(&format!("/?doc={doc}"));
                    app.update(InputEvent::KeyDown(Key::End));
                    app.active_modal = ActiveModal::Visuals;
                    app.selected_fx_slider = 8;
                    app.visual_effects = VisualEffects::retro_glitch();
                    app.visual_effects.noise = 0.37;
                    round_trip(&app);
                    app.start_screensaver();
                    round_trip(&app);
                }
            }
        }
    }

    #[test]
    fn invalid_query_values_are_bounded_and_do_not_create_non_finite_effects() {
        let params = [
            ("project", "missing"),
            ("row", "9999"),
            ("scroll", "18446744073709551615"),
            ("item", "9999"),
            ("link", "9999"),
            ("slider", "9999"),
            ("fx", "NaN,0,0,0,0,0,0,0,0"),
            ("magnet", "inf"),
            ("noise", "-5"),
            ("hum", "9"),
            ("system", "invalid"),
        ];
        let route = Route::read(
            |key| {
                params
                    .iter()
                    .find(|(k, _)| *k == key)
                    .map(|(_, v)| v.to_string())
            },
            ResolutionMode::C64,
        );
        let mut app = App::new_web();
        route.apply(&mut app, None);
        assert_eq!(app.resolution, ResolutionMode::C64);
        assert_eq!(app.selected_project, PROJECTS.len() - 1);
        assert_eq!(app.selected_fx_slider, 8);
        assert_eq!(app.visual_effects.magnet, 0.0);
        assert_eq!(app.visual_effects.noise, 0.0);
        assert_eq!(app.visual_effects.antenna_hum, 1.0);
        assert!(app.about_scroll <= app.about_max_scroll());
        let _ = app.render();
    }

    #[test]
    fn navigation_pushes_while_cursor_scroll_settings_and_saver_rotation_replace() {
        let mut app = App::new_web();
        let start = Route::capture(&app);
        app.update(InputEvent::KeyDown(Key::End));
        let selected = Route::capture(&app);
        assert!(!selected.changes_screen(&start));
        app.update(InputEvent::KeyDown(Key::Enter));
        let detail = Route::capture(&app);
        assert!(detail.changes_screen(&selected));
        app.update(InputEvent::KeyDown(Key::End));
        assert!(!Route::capture(&app).changes_screen(&detail));
        app.active_modal = ActiveModal::Help;
        let help = Route::capture(&app);
        assert!(help.changes_screen(&detail));
        app.set_resolution(ResolutionMode::C64);
        app.visual_effects = VisualEffects::clean();
        assert!(!Route::capture(&app).changes_screen(&help));
        app.start_screensaver();
        let saver = Route::capture(&app);
        assert!(saver.changes_screen(&help));
        app.advance_ticks(300);
        assert!(!Route::capture(&app).changes_screen(&saver));
    }
}
