//! Pixel-first portfolio application core.
//!
//! Provides platform-independent application state, interactive view models,
//! retro system mode specifications (ZX Spectrum, C64, Atari 800, VGA, Amber, Green),
//! project architectures, resume content, and session orchestration.

pub mod data;
pub mod events;
pub mod state;
pub mod views;

pub use data::{
    project_detail_lines, Project, ABOUT_LINES_32, ABOUT_LINES_40, ABOUT_LINES_80,
    ABOUT_MAX_SCROLL, PROJECTS, RESUME_LINES_32, RESUME_LINES_40, RESUME_LINES_80,
    RESUME_MAX_SCROLL,
};
pub use state::{App, Session, Tab};

#[cfg(test)]
mod tests;
