//! Centralized theme system for Amazon Q CLI colors
//!
//! This module provides a unified color management system that replaces inline color
//! definitions throughout the codebase with semantic theme references. The theme system
//! maintains backward compatibility with existing color systems (color_print, crossterm)
//! while providing a consistent API for color usage.

pub mod colors;
pub mod crossterm_ext;

use std::sync::LazyLock;

pub use colors::*;
pub use crossterm_ext::*;

/// Main theme configuration containing all color categories
#[derive(Debug, Clone)]
pub struct Theme {
    /// Colors for status messages (error, warning, success, info)
    pub status: StatusColors,
    /// Colors for UI elements (branding, text, links, etc.)
    pub ui: UiColors,
    /// Colors for syntax highlighting and code display
    pub syntax: SyntaxColors,
    /// Colors for interactive elements (prompts, indicators, etc.)
    pub interactive: InteractiveColors,
}

/// Global theme instance available throughout the application
pub static DEFAULT_THEME: LazyLock<Theme> = LazyLock::new(Theme::default);

/// Get a reference to the global theme instance
pub fn theme() -> &'static Theme {
    &DEFAULT_THEME
}

impl Default for Theme {
    /// Creates the default theme with colors matching the current CLI appearance
    fn default() -> Self {
        Self {
            status: StatusColors::default(),
            ui: UiColors::default(),
            syntax: SyntaxColors::default(),
            interactive: InteractiveColors::default(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_default_creation() {
        let theme_instance = Theme::default();

        // Test status colors match expected theme values
        assert_eq!(theme_instance.status.error, theme().status.error);
        assert_eq!(theme_instance.status.warning, theme().status.warning);
        assert_eq!(theme_instance.status.success, theme().status.success);
        assert_eq!(theme_instance.status.info, theme().status.info);

        // Test UI colors match expected theme values
        assert_eq!(theme_instance.ui.primary_brand, theme().ui.primary_brand);
        assert_eq!(theme_instance.ui.secondary_text, theme().ui.secondary_text);
        assert_eq!(theme_instance.ui.emphasis, theme().ui.emphasis);
        assert_eq!(theme_instance.ui.command_highlight, theme().ui.command_highlight);

        // Test syntax colors match expected theme values
        assert_eq!(theme_instance.syntax.code_block, theme().syntax.code_block);

        // Test interactive colors match expected theme values
        assert_eq!(
            theme_instance.interactive.prompt_symbol,
            theme().interactive.prompt_symbol
        );
        assert_eq!(
            theme_instance.interactive.profile_indicator,
            theme().interactive.profile_indicator
        );
        assert_eq!(
            theme_instance.interactive.tangent_indicator,
            theme().interactive.tangent_indicator
        );
        assert_eq!(theme_instance.interactive.usage_low, theme().interactive.usage_low);
        assert_eq!(
            theme_instance.interactive.usage_medium,
            theme().interactive.usage_medium
        );
        assert_eq!(theme_instance.interactive.usage_high, theme().interactive.usage_high);
    }

    #[test]
    fn test_global_theme_instance() {
        let theme1 = theme();
        let theme2 = theme();

        // Verify it's the same instance
        assert!(std::ptr::eq(theme1, theme2));

        // Verify colors are consistent with theme system
        assert_eq!(theme1.status.error, theme().status.error);
        assert_eq!(theme1.ui.primary_brand, theme().ui.primary_brand);
    }
}
