//! Projects tab view implementation.

use pixel_ssh_view::{
    horizontal_scroll, Color, Element, LinkElement, Platform, RectElement,
    TextElement, TextStyle, View,
};
use crate::data::{project_detail_lines, PROJECTS};
use crate::state::App;

impl App {
    pub(crate) fn render_projects(&self, view: &mut View) {
        if self.platform == Platform::Terminal {
            self.render_projects_terminal(view);
            return;
        }
        let (cols, _) = self.palette_mode.char_grid();
        match cols {
            100 | 80 => self.render_projects_80(view),
            40 => self.render_projects_40(view),
            _ => self.render_projects_32(view),
        }
    }

    pub(crate) fn render_projects_80(&self, view: &mut View) {
        let lh = self.palette_mode.line_height();
        if self.show_detail {
            let p = &PROJECTS[self.selected_project];
            let lines = project_detail_lines(p, 80, self.tick);
            let max_scroll = self.detail_max_scroll();

            // Fixed header at top (Row 3: y = 3 * lh)
            let title_header = format!("PROJECT // {}  [h/l: Prev/Next]", p.title);
            let title_disp = horizontal_scroll(&title_header, 74, self.tick);
            let title_color = match (self.tick / 8) % 4 { 0 => 12, 1 => 14, 2 => 15, _ => 12 };

            view.add(Element::Text(TextElement {
                x: 16,
                y: 3 * lh,
                text: title_disp,
                style: TextStyle::new(Color::from_palette(title_color)).bold(),
            }));

            let status_badge = if p.wip {
                "STATUS: In Active Development"
            } else {
                "STATUS: Production-Ready / Stable Open-Source"
            };
            view.add(Element::Text(TextElement {
                x: 16,
                y: 4 * lh,
                text: status_badge.to_string(),
                style: TextStyle::new(if p.wip { Color::from_palette(10) } else { Color::from_palette(8) }),
            }));

            view.add(Element::Text(TextElement {
                x: 16,
                y: 5 * lh,
                text: format!("TAGS: {}", p.tags.join(", ")),
                style: TextStyle::new(Color::from_palette(11)),
            }));

            // Clickable repository link (interactive item 0, Row 6: y = 6 * lh)
            let gh_url = format!("https://github.com/dmytro-yemelianov/{}", p.slug);
            let repo_active = self.selected_detail_item == 0;
            let repo_style = if repo_active {
                TextStyle::new(Color::from_palette(14)).bold().underline()
            } else {
                TextStyle::new(Color::from_palette(7))
            };
            view.add(Element::Link(LinkElement::new(
                16,
                6 * lh,
                format!("{} [ Open Repository: {} ]", if repo_active { "►" } else { " " }, gh_url),
                gh_url,
                repo_style,
            )));

            // Divider line (Row 7: y = 7 * lh)
            view.add(Element::Rect(RectElement {
                x: 0,
                y: 7 * lh,
                width: view.width,
                height: 1,
                color: Color::from_palette(7),
                filled: true,
            }));

            // Scrollable content area: rows from 8 to visible_rows
            let visible_rows = ((view.height.saturating_sub(12 * lh)) / lh).min(13) as usize;
            for (i, (line_text, pal_idx, bold)) in lines.iter().enumerate() {
                let row_idx = (i as i32) - (self.detail_scroll as i32);
                if row_idx >= 0 && (row_idx as usize) < visible_rows {
                    let y = (8 + row_idx as u16) * lh;
                    let mut style = TextStyle::new(Color::from_palette(*pal_idx));
                    if *bold {
                        style = style.bold();
                    }
                    view.add(Element::Text(TextElement {
                        x: 16,
                        y,
                        text: line_text.clone(),
                        style,
                    }));
                }
            }

            // Scrollbar at right edge
            let scroll_x = view.width.saturating_sub(16);
            let track_rows = visible_rows.saturating_sub(2);
            let track_h = (track_rows as u16) * lh;
            view.add(Element::Text(TextElement {
                x: scroll_x,
                y: 8 * lh,
                text: "▲".to_string(),
                style: TextStyle::new(Color::from_palette(7)).bold(),
            }));

            view.add(Element::Rect(RectElement {
                x: scroll_x + 3,
                y: 9 * lh,
                width: 2,
                height: track_h,
                color: Color::from_palette(7),
                filled: true,
            }));

            let thumb_ratio = if max_scroll > 0 { self.detail_scroll as f32 / max_scroll as f32 } else { 0.0 };
            let thumb_pos = (thumb_ratio * (track_h.saturating_sub(lh) as f32)) as u16;
            let thumb_y = 9 * lh + thumb_pos;
            view.add(Element::Rect(RectElement {
                x: scroll_x,
                y: thumb_y,
                width: 8,
                height: lh,
                color: Color::from_palette(6),
                filled: true,
            }));

            view.add(Element::Text(TextElement {
                x: scroll_x,
                y: (8 + visible_rows as u16 - 1) * lh,
                text: "▼".to_string(),
                style: TextStyle::new(Color::from_palette(7)).bold(),
            }));

            // Clickable Return button (interactive item 1, Row 8 + visible_rows)
            let return_row = 8 + visible_rows as u16;
            let back_active = self.selected_detail_item == 1;
            let back_style = if back_active {
                TextStyle::new(Color::from_palette(0)).with_bg(Color::from_palette(7)).bold()
            } else {
                TextStyle::new(Color::from_palette(9)).bold()
            };
            view.add(Element::Link(LinkElement::new(
                16,
                return_row * lh,
                format!("{} < [ESC] or [q] Return to Project List | [h/l] Prev/Next Project >", if back_active { "►" } else { " " }),
                "#back",
                back_style,
            )));

            // Line indicator at return_row + 1
            view.add(Element::Text(TextElement {
                x: 16,
                y: (return_row + 1) * lh,
                text: format!(
                    "Line {}-{} of {} [k/j Scroll] | [Tab] Focus Link | [Enter] Select",
                    self.detail_scroll + 1,
                    (self.detail_scroll + visible_rows).min(lines.len()),
                    lines.len()
                ),
                style: TextStyle::new(Color::from_palette(4)),
            }));
            return;
        }

        // Single Panel Volkov Commander layout
        let cols = (view.width / 8) as usize;
        let title_panel = " D:\\PORTFOLIO\\PROJECTS ";
        let left_pad = cols.saturating_sub(2 + title_panel.len()) / 2;
        let right_pad = cols.saturating_sub(2 + title_panel.len() + left_pad);
        let top_border = format!("╔{}{}{}╗", "═".repeat(left_pad), title_panel, "═".repeat(right_pad));
        view.add(Element::Text(TextElement {
            x: 0,
            y: 2 * lh,
            text: top_border,
            style: TextStyle::new(Color::from_palette(3)).bold(),
        }));

        let tag_col_w = cols.saturating_sub(49);
        let header_str = format!("║  #   Project Title            │ {:<tag_col_w$}│ Action       ║", "Tags / Domain Subsystems ");
        view.add(Element::Text(TextElement {
            x: 0,
            y: 3 * lh,
            text: header_str,
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        let sep_str = format!("╟──────────────────────────────┼─{}┼──────────────╢", "─".repeat(tag_col_w.saturating_sub(1)));
        view.add(Element::Text(TextElement {
            x: 0,
            y: 4 * lh,
            text: sep_str,
            style: TextStyle::new(Color::from_palette(3)),
        }));

        let max_visible = self.projects_max_visible();
        let start = self.scroll_offset;
        let end = (start + max_visible).min(PROJECTS.len());

        for (i, p) in PROJECTS[start..end].iter().enumerate() {
            let idx = start + i;
            let is_sel = idx == self.selected_project;
            let y = (5 + i as u16) * lh;

            if is_sel {
                view.add(Element::Rect(RectElement {
                    x: 8,
                    y,
                    width: view.width.saturating_sub(16),
                    height: lh,
                    color: Color::from_palette(2),
                    filled: true,
                }));
            }

            let marker = if is_sel { ">" } else { " " };
            let fg = if is_sel {
                Color::from_palette(6)
            } else if p.wip {
                Color::from_palette(10)
            } else {
                Color::from_palette(5)
            };

            let num = format!("{:02}", idx + 1);
            let title = horizontal_scroll(p.title, 22, self.tick);

            // Left panel border
            view.add(Element::Text(TextElement {
                x: 0,
                y,
                text: "║".to_string(),
                style: TextStyle::new(Color::from_palette(3)),
            }));

            // Item number and title (at x=16 for test compatibility)
            view.add(Element::Text(TextElement {
                x: 16,
                y,
                text: format!("{} [{}] {:<22}", marker, num, title),
                style: TextStyle::new(fg).bold(),
            }));

            // Mid separator
            view.add(Element::Text(TextElement {
                x: 248,
                y,
                text: "│".to_string(),
                style: TextStyle::new(Color::from_palette(3)),
            }));

            // Tags formatted with horizontal auto-scroll when selected or long
            let tag_str = p.tags.iter().map(|t| format!("<{}>", t)).collect::<Vec<_>>().join(" ");
            let tag_w = tag_col_w.saturating_sub(2);
            let tag_display = if is_sel || tag_str.chars().count() > tag_w {
                horizontal_scroll(&tag_str, tag_w, self.tick)
            } else {
                tag_str
            };

            view.add(Element::Text(TextElement {
                x: 264,
                y,
                text: tag_display,
                style: TextStyle::new(if is_sel { Color::from_palette(14) } else { Color::from_palette(4) }),
            }));

            // Action separator
            let act_sep_x = (cols.saturating_sub(16) * 8) as u16;
            view.add(Element::Text(TextElement {
                x: act_sep_x,
                y,
                text: "│".to_string(),
                style: TextStyle::new(Color::from_palette(3)),
            }));

            // Action text
            let action_x = act_sep_x + 16;
            view.add(Element::Text(TextElement {
                x: action_x,
                y,
                text: if is_sel { "[Enter] ->" } else { "  Details " }.to_string(),
                style: TextStyle::new(if is_sel { Color::from_palette(7) } else { Color::from_palette(3) }).bold(),
            }));

            // Right panel border
            view.add(Element::Text(TextElement {
                x: view.width.saturating_sub(8),
                y,
                text: "║".to_string(),
                style: TextStyle::new(Color::from_palette(3)),
            }));
        }

        // Panel Bottom Border
        let bot_label = " 16 Projects • 160 KB Free ";
        let left_bot = cols.saturating_sub(2 + bot_label.len()) / 2;
        let right_bot = cols.saturating_sub(2 + bot_label.len() + left_bot);
        let bot_border = format!("╚{}{}{}╝", "═".repeat(left_bot), bot_label, "═".repeat(right_bot));
        let bot_y = (5 + max_visible as u16) * lh;
        if bot_y + lh <= view.height {
            view.add(Element::Text(TextElement {
                x: 0,
                y: bot_y,
                text: bot_border,
                style: TextStyle::new(Color::from_palette(3)).bold(),
            }));
        }

        // DOS prompt path at bottom
        let prompt_y = bot_y + lh;
        if prompt_y + lh <= view.height {
            let selected_slug = PROJECTS.get(self.selected_project).map(|p| p.slug).unwrap_or("portfolio");
            let prompt_path = format!("C:\\DMYTRO\\PROJECTS\\{}>", selected_slug.to_uppercase());
            let cursor_char = if (self.tick / 4) % 2 == 0 { "█" } else { " " };
            view.add(Element::Text(TextElement {
                x: 8,
                y: prompt_y,
                text: format!("{}{}", prompt_path, cursor_char),
                style: TextStyle::new(Color::from_palette(14)).bold(),
            }));
        }

        // Navigation guidance
        let hint_y = prompt_y + lh;
        if hint_y + lh <= view.height {
            view.add(Element::Text(TextElement {
                x: 8,
                y: hint_y,
                text: "Use [↑/↓] or [k/j] to select • [Enter] Details • [1-8] or Click buttons below".to_string(),
                style: TextStyle::new(Color::from_palette(4)),
            }));
        }
    }

    pub(crate) fn render_projects_40(&self, view: &mut View) {
        if self.show_detail {
            let p = &PROJECTS[self.selected_project];
            let lines = project_detail_lines(p, 40, self.tick);
            let max_scroll = self.detail_max_scroll();

            for (i, (line_text, pal_idx, bold)) in lines.iter().enumerate() {
                let line_y = 24 + ((i as i32) - (self.detail_scroll as i32)) * 8;
                if line_y >= 24 && line_y <= 172 {
                    let mut style = TextStyle::new(Color::from_palette(*pal_idx));
                    if *bold {
                        style = style.bold();
                    }
                    view.add(Element::Text(TextElement {
                        x: 4,
                        y: line_y as u16,
                        text: line_text.clone(),
                        style,
                    }));
                }
            }

            // Scrollbar at x = 310
            view.add(Element::Text(TextElement {
                x: 310,
                y: 24,
                text: "▲".to_string(),
                style: TextStyle::new(Color::from_palette(7)).bold(),
            }));

            view.add(Element::Rect(RectElement {
                x: 313,
                y: 34,
                width: 2,
                height: 134,
                color: Color::from_palette(3),
                filled: true,
            }));

            let track_h = 134.0 - 16.0;
            let thumb_ratio = if max_scroll > 0 { self.detail_scroll as f32 / max_scroll as f32 } else { 0.0 };
            let thumb_y = 34 + (thumb_ratio * track_h) as u16;
            view.add(Element::Rect(RectElement {
                x: 310,
                y: thumb_y,
                width: 8,
                height: 16,
                color: Color::from_palette(6),
                filled: true,
            }));

            view.add(Element::Text(TextElement {
                x: 310,
                y: 168,
                text: "▼".to_string(),
                style: TextStyle::new(Color::from_palette(7)).bold(),
            }));

            view.add(Element::Text(TextElement {
                x: 4,
                y: 177,
                text: format!(
                    "[h/l]Prj {}-{} of {} [k/j] | [ESC]Back",
                    self.detail_scroll + 1,
                    (self.detail_scroll + 16).min(lines.len()),
                    lines.len()
                ),
                style: TextStyle::new(Color::from_palette(4)),
            }));
            return;
        }

        let max_visible = 12;
        let start = self.scroll_offset;
        let end = (start + max_visible).min(PROJECTS.len());

        for (i, p) in PROJECTS[start..end].iter().enumerate() {
            let idx = start + i;
            let is_sel = idx == self.selected_project;
            let y = 24 + (i as u16) * 12;

            if is_sel {
                view.add(Element::Rect(RectElement {
                    x: 2,
                    y: y - 1,
                    width: 316,
                    height: 11,
                    color: Color::from_palette(2),
                    filled: true,
                }));
            }

            let marker = if is_sel { ">" } else { " " };
            let fg = if is_sel {
                Color::from_palette(6)
            } else if p.wip {
                Color::from_palette(10)
            } else {
                Color::from_palette(5)
            };

            let num = format!("{:02}", idx + 1);
            let title = horizontal_scroll(p.title, 14, self.tick);

            let tag_str = p.tags.iter().map(|t| format!("<{}>", t)).collect::<Vec<_>>().join(" ");
            let tag_display = if is_sel {
                horizontal_scroll(&tag_str, 17, self.tick)
            } else if tag_str.chars().count() > 17 {
                horizontal_scroll(&tag_str, 17, self.tick)
            } else {
                tag_str
            };

            view.add(Element::Text(TextElement {
                x: 4,
                y,
                text: format!("{} [{}] {:<14} {}", marker, num, title, tag_display),
                style: TextStyle::new(fg).bold(),
            }));
        }

        if PROJECTS.len() > max_visible {
            view.add(Element::Text(TextElement {
                x: 4,
                y: 176,
                text: format!("{}-{} of {}. [j/k]Nav [Enter]Info", start + 1, end, PROJECTS.len()),
                style: TextStyle::new(Color::from_palette(4)),
            }));
        }
    }

    pub(crate) fn render_projects_32(&self, view: &mut View) {
        if self.show_detail {
            let p = &PROJECTS[self.selected_project];
            let lines = project_detail_lines(p, 32, self.tick);
            let max_scroll = self.detail_max_scroll();

            for (i, (line_text, pal_idx, bold)) in lines.iter().enumerate() {
                let line_y = 24 + ((i as i32) - (self.detail_scroll as i32)) * 8;
                if line_y >= 24 && line_y <= 162 {
                    let mut style = TextStyle::new(Color::from_palette(*pal_idx));
                    if *bold {
                        style = style.bold();
                    }
                    view.add(Element::Text(TextElement {
                        x: 2,
                        y: line_y as u16,
                        text: line_text.clone(),
                        style,
                    }));
                }
            }

            // Scrollbar at x = 246
            view.add(Element::Text(TextElement {
                x: 246,
                y: 24,
                text: "▲".to_string(),
                style: TextStyle::new(Color::from_palette(7)).bold(),
            }));

            view.add(Element::Rect(RectElement {
                x: 249,
                y: 34,
                width: 2,
                height: 124,
                color: Color::from_palette(3),
                filled: true,
            }));

            let track_h = 124.0 - 14.0;
            let thumb_ratio = if max_scroll > 0 { self.detail_scroll as f32 / max_scroll as f32 } else { 0.0 };
            let thumb_y = 34 + (thumb_ratio * track_h) as u16;
            view.add(Element::Rect(RectElement {
                x: 246,
                y: thumb_y,
                width: 8,
                height: 14,
                color: Color::from_palette(6),
                filled: true,
            }));

            view.add(Element::Text(TextElement {
                x: 246,
                y: 158,
                text: "▼".to_string(),
                style: TextStyle::new(Color::from_palette(7)).bold(),
            }));

            view.add(Element::Text(TextElement {
                x: 2,
                y: 169,
                text: format!(
                    "[h/l]Prj {}-{} of {} | [ESC]",
                    self.detail_scroll + 1,
                    (self.detail_scroll + 15).min(lines.len()),
                    lines.len()
                ),
                style: TextStyle::new(Color::from_palette(4)),
            }));
            return;
        }

        let max_visible = 12;
        let start = self.scroll_offset;
        let end = (start + max_visible).min(PROJECTS.len());

        for (i, p) in PROJECTS[start..end].iter().enumerate() {
            let idx = start + i;
            let is_sel = idx == self.selected_project;
            let y = 24 + (i as u16) * 11;

            if is_sel {
                view.add(Element::Rect(RectElement {
                    x: 1,
                    y: y - 1,
                    width: 254,
                    height: 11,
                    color: Color::from_palette(2),
                    filled: true,
                }));
            }

            let marker = if is_sel { ">" } else { " " };
            let fg = if is_sel {
                Color::from_palette(6)
            } else if p.wip {
                Color::from_palette(10)
            } else {
                Color::from_palette(5)
            };

            let num = format!("{:02}", idx + 1);
            let title = horizontal_scroll(p.title, 12, self.tick);

            let tag_str = p.tags.iter().map(|t| format!("<{}>", t)).collect::<Vec<_>>().join(" ");
            let tag_display = if is_sel {
                horizontal_scroll(&tag_str, 12, self.tick)
            } else if tag_str.chars().count() > 12 {
                horizontal_scroll(&tag_str, 12, self.tick)
            } else {
                tag_str
            };

            view.add(Element::Text(TextElement {
                x: 2,
                y,
                text: format!("{}{:<2} {:<12} {}", marker, num, title, tag_display),
                style: TextStyle::new(fg).bold(),
            }));
        }

        if PROJECTS.len() > max_visible {
            view.add(Element::Text(TextElement {
                x: 2,
                y: 168,
                text: format!("{}-{} of {}. [j/k]Nav [Enter]", start + 1, end, PROJECTS.len()),
                style: TextStyle::new(Color::from_palette(4)),
            }));
        }
    }

    pub(crate) fn render_projects_terminal(&self, view: &mut View) {
        let cols = self.terminal_cols;
        let rows = self.terminal_rows;
        let width = cols * 8;

        if self.show_detail {
            let p = &PROJECTS[self.selected_project];
            let lines = project_detail_lines(p, (cols as usize).min(80), self.tick);

            // Row 3: Title Header (y = 48)
            let title_header = format!("PROJECT // {}  [h/l: Prev/Next]", p.title);
            let max_title = (cols as usize).saturating_sub(6).max(30);
            let title_disp = horizontal_scroll(&title_header, max_title, self.tick);
            let title_color = match (self.tick / 8) % 4 { 0 => 12, 1 => 14, 2 => 15, _ => 12 };
            view.add(Element::Text(TextElement {
                x: 8,
                y: 48,
                text: title_disp,
                style: TextStyle::new(Color::from_palette(title_color)).bold(),
            }));

            // Row 4: Status badge & tags (y = 64)
            let status_badge = if p.wip {
                "STATUS: In Active Development"
            } else {
                "STATUS: Production-Ready / Stable Open-Source"
            };
            let tags_str = format!("TAGS: {}", p.tags.join(", "));
            let header_row_2 = format!("{} | {}", status_badge, tags_str);
            view.add(Element::Text(TextElement {
                x: 8,
                y: 64,
                text: header_row_2,
                style: TextStyle::new(if p.wip { Color::from_palette(10) } else { Color::from_palette(8) }),
            }));

            // Row 5: Open Repository link (y = 80)
            let gh_url = format!("https://github.com/dmytro-yemelianov/{}", p.slug);
            let repo_active = self.selected_detail_item == 0;
            let repo_style = if repo_active {
                TextStyle::new(Color::from_palette(14)).bold().underline()
            } else {
                TextStyle::new(Color::from_palette(7))
            };
            view.add(Element::Link(LinkElement::new(
                8,
                80,
                format!("{} [ Open Repository: {} ]", if repo_active { "►" } else { " " }, gh_url),
                gh_url,
                repo_style,
            )));

            // Row 6: Divider line (y = 96)
            view.add(Element::Rect(RectElement {
                x: 0,
                y: 96,
                width,
                height: 1,
                color: Color::from_palette(7),
                filled: true,
            }));

            // Content area: rows 7 to rows - 4
            let visible_rows = (rows as usize).saturating_sub(11).max(6);
            for row_idx in 0..visible_rows {
                let line_idx = self.detail_scroll + row_idx;
                if line_idx < lines.len() {
                    let (line_text, pal_idx, bold) = &lines[line_idx];
                    let y = (7 + row_idx as u16) * 16;
                    let mut style = TextStyle::new(Color::from_palette(*pal_idx));
                    if *bold {
                        style = style.bold();
                    }
                    view.add(Element::Text(TextElement {
                        x: 8,
                        y,
                        text: line_text.clone(),
                        style,
                    }));
                }
            }

            // Row rows - 3: Return button
            let back_active = self.selected_detail_item == 1;
            let back_style = if back_active {
                TextStyle::new(Color::from_palette(0)).with_bg(Color::from_palette(7)).bold()
            } else {
                TextStyle::new(Color::from_palette(9)).bold()
            };
            let back_y = (rows.saturating_sub(3)) * 16;
            view.add(Element::Link(LinkElement::new(
                8,
                back_y,
                format!("{} < [ESC] or [q] Return to Project List | [h/l] Prev/Next Project >", if back_active { "►" } else { " " }),
                "#back",
                back_style,
            )));

            // Row rows - 2: Line indicator
            let ind_y = (rows.saturating_sub(2)) * 16;
            view.add(Element::Text(TextElement {
                x: 8,
                y: ind_y,
                text: format!(
                    "Line {}-{} of {} [k/j Scroll] | [Tab] Focus Link | [Enter] Select",
                    self.detail_scroll + 1,
                    (self.detail_scroll + visible_rows).min(lines.len()),
                    lines.len()
                ),
                style: TextStyle::new(Color::from_palette(4)),
            }));
            return;
        }

        // Project List on Terminal
        let max_visible = self.projects_max_visible();
        let start = self.scroll_offset;
        let end = (start + max_visible).min(PROJECTS.len());

        for (i, p) in PROJECTS[start..end].iter().enumerate() {
            let idx = start + i;
            let is_sel = idx == self.selected_project;
            let y = (3 + i as u16) * 16;

            if is_sel {
                view.add(Element::Rect(RectElement {
                    x: 0,
                    y,
                    width,
                    height: 16,
                    color: Color::from_palette(2),
                    filled: true,
                }));
            }

            let marker = if is_sel { ">" } else { " " };
            let fg = if is_sel {
                Color::from_palette(6)
            } else if p.wip {
                Color::from_palette(10)
            } else {
                Color::from_palette(5)
            };

            let num = format!("{:02}", idx + 1);
            let title_w = if cols >= 110 { 28 } else { 22 };
            let title = horizontal_scroll(p.title, title_w, self.tick);

            view.add(Element::Text(TextElement {
                x: 8,
                y,
                text: format!("{} [{}] {:<width$}", marker, num, title, width = title_w),
                style: TextStyle::new(fg).bold(),
            }));

            // Tags formatted to fill available width cleanly
            let tag_col = (title_w + 10) as u16;
            let tag_x = tag_col * 8;
            let tag_str = p.tags.iter().map(|t| format!("<{}>", t)).collect::<Vec<_>>().join(" ");
            let avail_chars = (cols as usize).saturating_sub(tag_col as usize + 16);
            let tag_display = if is_sel || tag_str.chars().count() > avail_chars {
                horizontal_scroll(&tag_str, avail_chars, self.tick)
            } else {
                tag_str
            };

            view.add(Element::Text(TextElement {
                x: tag_x,
                y,
                text: tag_display,
                style: TextStyle::new(Color::from_palette(4)),
            }));

            // Detail arrow at far right edge
            let det_x = width.saturating_sub(112);
            view.add(Element::Text(TextElement {
                x: det_x,
                y,
                text: if is_sel { "[Enter] ->" } else { "  Details " }.to_string(),
                style: TextStyle::new(if is_sel { Color::from_palette(7) } else { Color::from_palette(4) }),
            }));
        }

        // Scroll guidance note at row rows - 3
        let note_y = (rows.saturating_sub(3)) * 16;
        view.add(Element::Text(TextElement {
            x: 8,
            y: note_y,
            text: format!(
                "Showing {}-{} of {} projects. Use [Up/Down] or [k/j] to scroll | [Enter] Project Detail",
                start + 1,
                end,
                PROJECTS.len()
            ),
            style: TextStyle::new(Color::from_palette(4)),
        }));
    }
}
