//! Embedded documentation, rendered by the same native reader as About and CV.
use crate::{App, Tab, PROJECTS};
use pixel_ssh_view::{word_wrap, ActiveModal};

pub const DOCUMENTS: &[(&str, &str, &str)] = &[
    (
        "01-vision-and-paradigms",
        "VPA: Vision",
        include_str!("data/documents/01-vision-and-paradigms.md"),
    ),
    (
        "02-language-and-verification",
        "VPA: Language",
        include_str!("data/documents/02-language-and-verification.md"),
    ),
    (
        "03-device-interaction",
        "VPA: Devices",
        include_str!("data/documents/03-device-interaction.md"),
    ),
    (
        "04-compilation-and-lowering",
        "VPA: Compilation",
        include_str!("data/documents/04-compilation-and-lowering.md"),
    ),
    (
        "05-agent-harness-and-llm",
        "VPA: AI Harness",
        include_str!("data/documents/05-agent-harness-and-llm.md"),
    ),
    (
        "06-roadmap-and-milestones",
        "VPA: Roadmap",
        include_str!("data/documents/06-roadmap-and-milestones.md"),
    ),
];

impl App {
    /// Opens a native view without navigating away from the WASM application.
    pub fn open_route(&mut self, url: &str) -> bool {
        let route = url.strip_prefix("https://yemelianov.dev").unwrap_or(url);
        let Some(query) = route.strip_prefix("/?") else {
            return false;
        };
        let params: Vec<_> = query.split('&').filter_map(|p| p.split_once('=')).collect();
        let param = |key: &str| params.iter().find(|(k, _)| *k == key).map(|(_, v)| *v);
        if let Some(id) = param("doc") {
            let Some(index) = DOCUMENTS.iter().position(|d| d.0 == id) else {
                return false;
            };
            self.document = Some(index);
            self.current_tab = Tab::About;
            self.about_scroll = 0;
            self.reflow_document();
        } else if let Some(slug) = param("project") {
            let index = PROJECTS
                .iter()
                .position(|p| p.slug == slug)
                .or_else(|| slug.parse::<usize>().ok().filter(|i| *i < PROJECTS.len()));
            let Some(index) = index else {
                return false;
            };
            self.document = None;
            self.current_tab = Tab::Projects;
            self.selected_project = index;
            self.selected_list_item = index + 3;
            self.show_detail = param("detail") == Some("1");
            self.detail_scroll = 0;
            self.selected_detail_item = 0;
            self.ensure_selected_project_visible();
        } else if let Some(tab) = param("tab") {
            let tab = match tab {
                "projects" => Tab::Projects,
                "links" => Tab::Links,
                "resume" => Tab::Resume,
                "about" => Tab::About,
                "contact" => Tab::Contact,
                _ => return false,
            };
            self.document = None;
            self.current_tab = tab;
            self.show_detail = false;
            if tab == Tab::Links {
                self.select_link(self.selected_link);
            }
        } else {
            return false;
        }
        self.active_modal = ActiveModal::None;
        self.screensaver_active = false;
        self.status = "j/k:Scroll  Esc:Projects  L:Links".to_owned();
        true
    }

    pub(crate) fn reflow_document(&mut self) {
        self.document_lines.clear();
        let Some(index) = self.document else {
            return;
        };
        let cols = if self.platform == pixel_ssh_view::Platform::Terminal {
            self.terminal_cols
        } else {
            self.resolution.char_grid().0
        };
        let width = cols.saturating_sub(8) as usize;
        for line in DOCUMENTS[index].2.lines() {
            if line.starts_with("```") {
                continue;
            }
            let heading = line.starts_with('#');
            let text = line
                .trim_start_matches('#')
                .trim()
                .replace("**", "")
                .replace('`', "");
            for row in word_wrap(&text, width) {
                if row.is_empty() {
                    self.document_lines.push((row, 7, false));
                } else {
                    for chunk in row.chars().collect::<Vec<_>>().chunks(width) {
                        self.document_lines.push((
                            chunk.iter().collect(),
                            if heading { 6 } else { 7 },
                            heading,
                        ));
                    }
                }
            }
        }
    }
}
