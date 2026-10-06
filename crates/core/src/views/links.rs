//! Project launchers and source links, using the native framebuffer UI.

use crate::state::App;
use pixel_ssh_view::{
    horizontal_scroll, Color, Element, LinkElement, Platform, RectElement, TextElement, TextStyle,
    View,
};

pub(crate) const PROJECT_LINKS: &[(&str, &str)] = &[
    ("AutoCADED / open", "https://autocaded.yemelianov.dev/"),
    (
        "AutoCADED / repository",
        "https://github.com/dmytro-yemelianov/autocaded",
    ),
    ("NetHackED / open", "https://nethacked.yemelianov.dev/"),
    (
        "NetHackED / repository",
        "https://github.com/dmytro-yemelianov/NetHackED",
    ),
    (
        "maked / repository",
        "https://github.com/dmytro-yemelianov/maked",
    ),
    (
        "Pixel SSH / repository",
        "https://github.com/dmytro-yemelianov/pixel-ssh",
    ),
    ("Dry / documentation", "https://dry-public-docs.pages.dev/"),
    (
        "Dry / repository",
        "https://github.com/dmytro-yemelianov/dry",
    ),
    ("UPYR / open", "https://upyr.org/"),
    (
        "UPYR / repository",
        "https://github.com/dmytro-yemelianov/upyr",
    ),
    ("VerbaCorpus / open", "https://verbacorpus.org/"),
    (
        "VerbaCorpus / repository",
        "https://github.com/dmytro-yemelianov/verbacorpus",
    ),
    ("Barvinca / open", "https://barvinca.com/"),
    ("VPA / documentation", "https://yemelianov.dev/vpa/"),
    ("Full project catalog", "https://yemelianov.dev/catalog/"),
];

impl App {
    pub(crate) fn navigation_geometry(&self) -> (u16, u16) {
        let (_, height) = self.view_dimensions();
        if self.platform == Platform::Terminal {
            ((self.terminal_rows.saturating_sub(2)) * 16, 16)
        } else if self.resolution.char_grid().0 < 80 {
            (height.saturating_sub(20), 10)
        } else {
            (height.saturating_sub(32), 16)
        }
    }

    pub(crate) fn links_geometry(&self) -> (u16, u16, usize) {
        let lh = if self.platform == Platform::Terminal {
            16
        } else {
            self.resolution.line_height()
        };
        let top = 4 * lh;
        let visible = (self.navigation_geometry().0.saturating_sub(top + lh) / (2 * lh)) as usize;
        (top, lh, visible.clamp(1, PROJECT_LINKS.len()))
    }

    pub(crate) fn select_link(&mut self, selected: usize) {
        self.selected_link = selected.min(PROJECT_LINKS.len() - 1);
        let visible = self.links_geometry().2;
        self.links_scroll = self
            .links_scroll
            .min(PROJECT_LINKS.len().saturating_sub(visible));
        if self.selected_link < self.links_scroll {
            self.links_scroll = self.selected_link;
        } else if self.selected_link >= self.links_scroll + visible {
            self.links_scroll = self.selected_link + 1 - visible;
        }
    }

    pub(crate) fn render_links(&self, view: &mut View) {
        let cols = view.width / 8;
        let (top, lh, visible) = self.links_geometry();
        let offset = self
            .links_scroll
            .min(PROJECT_LINKS.len().saturating_sub(visible));
        view.add(Element::Text(TextElement {
            x: 8,
            y: 2 * lh,
            text: "PROJECT LINKS".to_owned(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));
        view.add(Element::Rect(RectElement {
            x: 0,
            y: 3 * lh,
            width: view.width,
            height: 1,
            color: Color::from_palette(7),
            filled: true,
        }));
        for (row, &(label, url)) in PROJECT_LINKS.iter().skip(offset).take(visible).enumerate() {
            let selected = offset + row == self.selected_link;
            let y = top + row as u16 * 2 * lh;
            if selected {
                view.add(Element::Rect(RectElement {
                    x: 0,
                    y,
                    width: view.width,
                    height: 2 * lh,
                    color: Color::from_palette(13),
                    filled: true,
                }));
            }
            view.add(Element::Link(LinkElement::new(
                8,
                y,
                format!("{} {}", if selected { "►" } else { " " }, label),
                url,
                TextStyle::new(Color::from_palette(if selected { 14 } else { 7 }))
                    .bold()
                    .underline(),
            )));
            view.add(Element::Text(TextElement {
                x: 24,
                y: y + lh,
                text: horizontal_scroll(
                    url.trim_start_matches("https://"),
                    cols.saturating_sub(4) as usize,
                    self.tick,
                ),
                style: TextStyle::new(Color::from_palette(8)),
            }));
        }
        view.add(Element::Text(TextElement {
            x: 8,
            y: self.navigation_geometry().0.saturating_sub(lh),
            text: format!(
                "{}/{}  j/k:Move Enter:Open",
                self.selected_link + 1,
                PROJECT_LINKS.len()
            ),
            style: TextStyle::new(Color::from_palette(8)),
        }));
    }
}
