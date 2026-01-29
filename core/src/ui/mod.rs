//! UI Layout System for ShadowCrypt
//!
//! This module provides a comprehensive UI layout system including:
//! - Panel system with borders, scrolling, and content management
//! - Multiple layout types (Classic, Wide, Compact, Full, Custom)
//! - UI components for game views (Map, Stats, Inventory, etc.)
//! - Responsive design adapting to terminal/window size
//! - Border styles (Single, Double, Rounded, Heavy, None)
//! - Interactive features (tooltips, highlighting, selection)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Constants
// ============================================================================

/// Minimum terminal width required for the UI
pub const MIN_TERMINAL_WIDTH: u16 = 80;

/// Minimum terminal height required for the UI
pub const MIN_TERMINAL_HEIGHT: u16 = 24;

/// Default map panel width ratio
pub const DEFAULT_MAP_WIDTH_RATIO: f32 = 0.65;

/// Default stats panel width ratio
pub const DEFAULT_STATS_WIDTH_RATIO: f32 = 0.35;

/// Default message log height
pub const DEFAULT_MESSAGE_LOG_HEIGHT: u16 = 6;

/// Default hotbar height
pub const DEFAULT_HOTBAR_HEIGHT: u16 = 3;

/// Maximum scroll buffer size for message log
pub const MAX_MESSAGE_BUFFER: usize = 500;

/// Mini-map size (square)
pub const MINI_MAP_SIZE: u16 = 15;

// ============================================================================
// Border Styles
// ============================================================================

/// Border style for panels
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Default)]
pub enum BorderStyle {
    /// No border
    None,
    /// Single line border (Box drawing: ─│┌┐└┘)
    #[default]
    Single,
    /// Double line border (Box drawing: ═║╔╗╚╝)
    Double,
    /// Rounded corners (Box drawing: ─│╭╮╰╯)
    Rounded,
    /// Heavy/thick border (Box drawing: ━┃┏┓┗┛)
    Heavy,
    /// ASCII border (+-|)
    Ascii,
    /// Dashed border
    Dashed,
}

impl BorderStyle {
    /// Returns the border characters for this style
    /// Order: (horizontal, vertical, top_left, top_right, bottom_left, bottom_right)
    pub fn chars(&self) -> BorderChars {
        match self {
            Self::None => BorderChars::none(),
            Self::Single => BorderChars {
                horizontal: '─',
                vertical: '│',
                top_left: '┌',
                top_right: '┐',
                bottom_left: '└',
                bottom_right: '┘',
                t_down: '┬',
                t_up: '┴',
                t_right: '├',
                t_left: '┤',
                cross: '┼',
            },
            Self::Double => BorderChars {
                horizontal: '═',
                vertical: '║',
                top_left: '╔',
                top_right: '╗',
                bottom_left: '╚',
                bottom_right: '╝',
                t_down: '╦',
                t_up: '╩',
                t_right: '╠',
                t_left: '╣',
                cross: '╬',
            },
            Self::Rounded => BorderChars {
                horizontal: '─',
                vertical: '│',
                top_left: '╭',
                top_right: '╮',
                bottom_left: '╰',
                bottom_right: '╯',
                t_down: '┬',
                t_up: '┴',
                t_right: '├',
                t_left: '┤',
                cross: '┼',
            },
            Self::Heavy => BorderChars {
                horizontal: '━',
                vertical: '┃',
                top_left: '┏',
                top_right: '┓',
                bottom_left: '┗',
                bottom_right: '┛',
                t_down: '┳',
                t_up: '┻',
                t_right: '┣',
                t_left: '┫',
                cross: '╋',
            },
            Self::Ascii => BorderChars {
                horizontal: '-',
                vertical: '|',
                top_left: '+',
                top_right: '+',
                bottom_left: '+',
                bottom_right: '+',
                t_down: '+',
                t_up: '+',
                t_right: '+',
                t_left: '+',
                cross: '+',
            },
            Self::Dashed => BorderChars {
                horizontal: '╌',
                vertical: '╎',
                top_left: '┌',
                top_right: '┐',
                bottom_left: '└',
                bottom_right: '┘',
                t_down: '┬',
                t_up: '┴',
                t_right: '├',
                t_left: '┤',
                cross: '┼',
            },
        }
    }

    /// Returns display name for this border style
    pub fn name(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Single => "Single Line",
            Self::Double => "Double Line",
            Self::Rounded => "Rounded",
            Self::Heavy => "Heavy",
            Self::Ascii => "ASCII",
            Self::Dashed => "Dashed",
        }
    }

    /// Returns all available border styles
    pub fn all() -> &'static [BorderStyle] {
        &[
            Self::None,
            Self::Single,
            Self::Double,
            Self::Rounded,
            Self::Heavy,
            Self::Ascii,
            Self::Dashed,
        ]
    }
}

/// Border characters for drawing panels
#[derive(Clone, Copy, Debug)]
pub struct BorderChars {
    pub horizontal: char,
    pub vertical: char,
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,
    pub t_down: char,
    pub t_up: char,
    pub t_right: char,
    pub t_left: char,
    pub cross: char,
}

impl BorderChars {
    /// Create empty border chars (for no border)
    pub fn none() -> Self {
        Self {
            horizontal: ' ',
            vertical: ' ',
            top_left: ' ',
            top_right: ' ',
            bottom_left: ' ',
            bottom_right: ' ',
            t_down: ' ',
            t_up: ' ',
            t_right: ' ',
            t_left: ' ',
            cross: ' ',
        }
    }
}

// ============================================================================
// Color and Styling
// ============================================================================

/// Text color for UI elements
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Default)]
pub enum TextColor {
    #[default]
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    /// Custom RGB color (r, g, b)
    Rgb(u8, u8, u8),
}

impl TextColor {
    /// Convert to ANSI color code index (0-15 for standard colors)
    pub fn to_ansi_index(&self) -> Option<u8> {
        match self {
            Self::Default => None,
            Self::Black => Some(0),
            Self::Red => Some(1),
            Self::Green => Some(2),
            Self::Yellow => Some(3),
            Self::Blue => Some(4),
            Self::Magenta => Some(5),
            Self::Cyan => Some(6),
            Self::White => Some(7),
            Self::BrightBlack => Some(8),
            Self::BrightRed => Some(9),
            Self::BrightGreen => Some(10),
            Self::BrightYellow => Some(11),
            Self::BrightBlue => Some(12),
            Self::BrightMagenta => Some(13),
            Self::BrightCyan => Some(14),
            Self::BrightWhite => Some(15),
            Self::Rgb(_, _, _) => None,
        }
    }
}

/// Text style attributes
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Default)]
pub struct TextStyle {
    pub fg_color: TextColor,
    pub bg_color: TextColor,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub blink: bool,
    pub reverse: bool,
}

impl TextStyle {
    /// Create a new default text style
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder: set foreground color
    pub fn fg(mut self, color: TextColor) -> Self {
        self.fg_color = color;
        self
    }

    /// Builder: set background color
    pub fn bg(mut self, color: TextColor) -> Self {
        self.bg_color = color;
        self
    }

    /// Builder: set bold
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Builder: set italic
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Builder: set underline
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    /// Predefined style for highlighted text
    pub fn highlight() -> Self {
        Self::new().fg(TextColor::Black).bg(TextColor::Yellow)
    }

    /// Predefined style for selected items
    pub fn selected() -> Self {
        Self::new().fg(TextColor::Black).bg(TextColor::Cyan).bold()
    }

    /// Predefined style for error text
    pub fn error() -> Self {
        Self::new().fg(TextColor::BrightRed).bold()
    }

    /// Predefined style for success text
    pub fn success() -> Self {
        Self::new().fg(TextColor::BrightGreen)
    }

    /// Predefined style for warning text
    pub fn warning() -> Self {
        Self::new().fg(TextColor::Yellow)
    }

    /// Predefined style for info text
    pub fn info() -> Self {
        Self::new().fg(TextColor::Cyan)
    }

    /// Predefined style for muted/dim text
    pub fn muted() -> Self {
        Self::new().fg(TextColor::BrightBlack)
    }
}

// ============================================================================
// Styled Text
// ============================================================================

/// A segment of styled text
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StyledText {
    pub text: String,
    pub style: TextStyle,
}

impl StyledText {
    /// Create new styled text
    pub fn new(text: impl Into<String>, style: TextStyle) -> Self {
        Self {
            text: text.into(),
            style,
        }
    }

    /// Create plain text with default style
    pub fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: TextStyle::default(),
        }
    }

    /// Get the display width of this text (accounting for unicode)
    pub fn width(&self) -> usize {
        self.text.chars().count()
    }
}

/// A line that can contain multiple styled segments
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct StyledLine {
    pub segments: Vec<StyledText>,
}

impl StyledLine {
    /// Create a new empty styled line
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a line from plain text
    pub fn plain(text: impl Into<String>) -> Self {
        Self {
            segments: vec![StyledText::plain(text)],
        }
    }

    /// Add a styled segment
    pub fn add(&mut self, text: impl Into<String>, style: TextStyle) -> &mut Self {
        self.segments.push(StyledText::new(text, style));
        self
    }

    /// Add plain text
    pub fn add_plain(&mut self, text: impl Into<String>) -> &mut Self {
        self.segments.push(StyledText::plain(text));
        self
    }

    /// Get the total display width of this line
    pub fn width(&self) -> usize {
        self.segments.iter().map(|s| s.width()).sum()
    }

    /// Convert to plain string (no styling)
    pub fn to_plain_string(&self) -> String {
        self.segments.iter().map(|s| s.text.as_str()).collect()
    }

    /// Check if the line is empty
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty() || self.segments.iter().all(|s| s.text.is_empty())
    }
}

impl From<String> for StyledLine {
    fn from(s: String) -> Self {
        Self::plain(s)
    }
}

impl From<&str> for StyledLine {
    fn from(s: &str) -> Self {
        Self::plain(s)
    }
}

// ============================================================================
// Panel System
// ============================================================================

/// A UI panel with position, size, content, and styling
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Panel {
    /// X position (column) of the panel
    pub x: u16,
    /// Y position (row) of the panel
    pub y: u16,
    /// Width of the panel
    pub width: u16,
    /// Height of the panel
    pub height: u16,
    /// Optional panel title
    pub title: Option<String>,
    /// Border style for the panel
    pub border_style: BorderStyle,
    /// Content lines (plain strings for backward compatibility)
    pub content: Vec<String>,
    /// Styled content lines
    pub styled_content: Vec<StyledLine>,
    /// Current scroll offset
    pub scroll_offset: usize,
    /// Whether the panel is focused
    pub focused: bool,
    /// Whether the panel is visible
    pub visible: bool,
    /// Panel identifier
    pub id: PanelId,
    /// Z-index for layering
    pub z_index: u8,
    /// Padding inside the panel
    pub padding: Padding,
    /// Title style
    pub title_style: TextStyle,
    /// Border color
    pub border_color: TextColor,
    /// Background color
    pub background_color: TextColor,
    /// Whether content can be scrolled
    pub scrollable: bool,
    /// Selected item index (for lists)
    pub selected_index: Option<usize>,
    /// Highlighted indices
    pub highlighted_indices: Vec<usize>,
    /// Tooltip text (shown on hover/focus)
    pub tooltip: Option<String>,
    /// Minimum width constraint
    pub min_width: Option<u16>,
    /// Minimum height constraint
    pub min_height: Option<u16>,
    /// Maximum width constraint
    pub max_width: Option<u16>,
    /// Maximum height constraint
    pub max_height: Option<u16>,
}

/// Padding values for panels
#[derive(Clone, Copy, Serialize, Deserialize, Debug, Default)]
pub struct Padding {
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
    pub left: u16,
}

impl Padding {
    /// Create uniform padding
    pub fn uniform(value: u16) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    /// Create padding with horizontal and vertical values
    pub fn symmetric(horizontal: u16, vertical: u16) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    /// Create padding with specific values
    pub fn new(top: u16, right: u16, bottom: u16, left: u16) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    /// Total horizontal padding
    pub fn horizontal(&self) -> u16 {
        self.left + self.right
    }

    /// Total vertical padding
    pub fn vertical(&self) -> u16 {
        self.top + self.bottom
    }
}

/// Panel identifiers for UI components
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum PanelId {
    /// Main game map view
    Map,
    /// Player statistics (HP, MP, XP, etc.)
    Stats,
    /// Inventory list
    Inventory,
    /// Message log
    MessageLog,
    /// Corner mini-map
    MiniMap,
    /// Quick action hotbar
    Hotbar,
    /// Active buffs/debuffs display
    BuffBar,
    /// Selected target information
    TargetInfo,
    /// Equipment panel
    Equipment,
    /// Skills panel
    Skills,
    /// Quest log
    QuestLog,
    /// Dialog window
    Dialog,
    /// Help/keybindings
    Help,
    /// Menu panel
    Menu,
    /// Custom panel with identifier
    Custom(u32),
}

impl PanelId {
    /// Returns the default title for this panel type
    pub fn default_title(&self) -> &'static str {
        match self {
            Self::Map => "Dungeon",
            Self::Stats => "Stats",
            Self::Inventory => "Inventory",
            Self::MessageLog => "Messages",
            Self::MiniMap => "Map",
            Self::Hotbar => "Actions",
            Self::BuffBar => "Effects",
            Self::TargetInfo => "Target",
            Self::Equipment => "Equipment",
            Self::Skills => "Skills",
            Self::QuestLog => "Quests",
            Self::Dialog => "Dialog",
            Self::Help => "Help",
            Self::Menu => "Menu",
            Self::Custom(_) => "Panel",
        }
    }
}

impl Panel {
    /// Create a new panel with given position and size
    pub fn new(id: PanelId, x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
            title: Some(id.default_title().to_string()),
            border_style: BorderStyle::Single,
            content: Vec::new(),
            styled_content: Vec::new(),
            scroll_offset: 0,
            focused: false,
            visible: true,
            id,
            z_index: 0,
            padding: Padding::default(),
            title_style: TextStyle::new().bold(),
            border_color: TextColor::Default,
            background_color: TextColor::Default,
            scrollable: true,
            selected_index: None,
            highlighted_indices: Vec::new(),
            tooltip: None,
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
        }
    }

    /// Create a panel without a border
    pub fn borderless(id: PanelId, x: u16, y: u16, width: u16, height: u16) -> Self {
        let mut panel = Self::new(id, x, y, width, height);
        panel.border_style = BorderStyle::None;
        panel.title = None;
        panel
    }

    /// Builder: set title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Builder: remove title
    pub fn without_title(mut self) -> Self {
        self.title = None;
        self
    }

    /// Builder: set border style
    pub fn with_border(mut self, style: BorderStyle) -> Self {
        self.border_style = style;
        self
    }

    /// Builder: set padding
    pub fn with_padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    /// Builder: set z-index
    pub fn with_z_index(mut self, z: u8) -> Self {
        self.z_index = z;
        self
    }

    /// Builder: set scrollable
    pub fn scrollable(mut self, can_scroll: bool) -> Self {
        self.scrollable = can_scroll;
        self
    }

    /// Builder: set visibility
    pub fn visible(mut self, is_visible: bool) -> Self {
        self.visible = is_visible;
        self
    }

    /// Builder: set minimum size
    pub fn with_min_size(mut self, width: u16, height: u16) -> Self {
        self.min_width = Some(width);
        self.min_height = Some(height);
        self
    }

    /// Builder: set maximum size
    pub fn with_max_size(mut self, width: u16, height: u16) -> Self {
        self.max_width = Some(width);
        self.max_height = Some(height);
        self
    }

    /// Builder: set tooltip
    pub fn with_tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Get the inner width (content area)
    pub fn inner_width(&self) -> u16 {
        let border = if self.border_style == BorderStyle::None {
            0
        } else {
            2
        };
        self.width
            .saturating_sub(border)
            .saturating_sub(self.padding.horizontal())
    }

    /// Get the inner height (content area)
    pub fn inner_height(&self) -> u16 {
        let border = if self.border_style == BorderStyle::None {
            0
        } else {
            2
        };
        self.height
            .saturating_sub(border)
            .saturating_sub(self.padding.vertical())
    }

    /// Get the inner X position (content start)
    pub fn inner_x(&self) -> u16 {
        let border = if self.border_style == BorderStyle::None {
            0
        } else {
            1
        };
        self.x + border + self.padding.left
    }

    /// Get the inner Y position (content start)
    pub fn inner_y(&self) -> u16 {
        let border = if self.border_style == BorderStyle::None {
            0
        } else {
            1
        };
        self.y + border + self.padding.top
    }

    /// Set plain text content
    pub fn set_content(&mut self, content: Vec<String>) {
        self.content = content;
        self.styled_content.clear();
    }

    /// Set styled content
    pub fn set_styled_content(&mut self, content: Vec<StyledLine>) {
        self.styled_content = content;
        self.content.clear();
    }

    /// Add a plain text line
    pub fn add_line(&mut self, line: impl Into<String>) {
        self.content.push(line.into());
    }

    /// Add a styled line
    pub fn add_styled_line(&mut self, line: StyledLine) {
        self.styled_content.push(line);
    }

    /// Clear all content
    pub fn clear_content(&mut self) {
        self.content.clear();
        self.styled_content.clear();
        self.scroll_offset = 0;
    }

    /// Get the total number of content lines
    pub fn content_line_count(&self) -> usize {
        if !self.styled_content.is_empty() {
            self.styled_content.len()
        } else {
            self.content.len()
        }
    }

    /// Get visible content lines based on scroll offset and panel height
    pub fn visible_content(&self) -> Vec<&str> {
        let visible_lines = self.inner_height() as usize;
        self.content
            .iter()
            .skip(self.scroll_offset)
            .take(visible_lines)
            .map(|s| s.as_str())
            .collect()
    }

    /// Get visible styled content
    pub fn visible_styled_content(&self) -> Vec<&StyledLine> {
        let visible_lines = self.inner_height() as usize;
        self.styled_content
            .iter()
            .skip(self.scroll_offset)
            .take(visible_lines)
            .collect()
    }

    /// Scroll up by the specified number of lines
    pub fn scroll_up(&mut self, lines: usize) {
        if self.scrollable {
            self.scroll_offset = self.scroll_offset.saturating_sub(lines);
        }
    }

    /// Scroll down by the specified number of lines
    pub fn scroll_down(&mut self, lines: usize) {
        if self.scrollable {
            let max_offset = self
                .content_line_count()
                .saturating_sub(self.inner_height() as usize);
            self.scroll_offset = (self.scroll_offset + lines).min(max_offset);
        }
    }

    /// Scroll to the top
    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = 0;
    }

    /// Scroll to the bottom
    pub fn scroll_to_bottom(&mut self) {
        if self.scrollable {
            let max_offset = self
                .content_line_count()
                .saturating_sub(self.inner_height() as usize);
            self.scroll_offset = max_offset;
        }
    }

    /// Check if content can scroll up
    pub fn can_scroll_up(&self) -> bool {
        self.scrollable && self.scroll_offset > 0
    }

    /// Check if content can scroll down
    pub fn can_scroll_down(&self) -> bool {
        self.scrollable
            && self.scroll_offset + (self.inner_height() as usize) < self.content_line_count()
    }

    /// Select the next item (for list panels)
    pub fn select_next(&mut self) {
        if let Some(idx) = self.selected_index {
            let max = self.content_line_count().saturating_sub(1);
            self.selected_index = Some((idx + 1).min(max));
            self.ensure_selection_visible();
        } else if self.content_line_count() > 0 {
            self.selected_index = Some(0);
        }
    }

    /// Select the previous item (for list panels)
    pub fn select_previous(&mut self) {
        if let Some(idx) = self.selected_index {
            self.selected_index = Some(idx.saturating_sub(1));
            self.ensure_selection_visible();
        } else if self.content_line_count() > 0 {
            self.selected_index = Some(0);
        }
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selected_index = None;
    }

    /// Ensure the selected item is visible
    pub fn ensure_selection_visible(&mut self) {
        if let Some(idx) = self.selected_index {
            let visible_lines = self.inner_height() as usize;

            if idx < self.scroll_offset {
                self.scroll_offset = idx;
            } else if idx >= self.scroll_offset + visible_lines {
                self.scroll_offset = idx.saturating_sub(visible_lines - 1);
            }
        }
    }

    /// Add a highlighted index
    pub fn highlight(&mut self, index: usize) {
        if !self.highlighted_indices.contains(&index) {
            self.highlighted_indices.push(index);
        }
    }

    /// Remove a highlighted index
    pub fn unhighlight(&mut self, index: usize) {
        self.highlighted_indices.retain(|&i| i != index);
    }

    /// Clear all highlights
    pub fn clear_highlights(&mut self) {
        self.highlighted_indices.clear();
    }

    /// Check if an index is highlighted
    pub fn is_highlighted(&self, index: usize) -> bool {
        self.highlighted_indices.contains(&index)
    }

    /// Check if a point is inside this panel
    pub fn contains_point(&self, px: u16, py: u16) -> bool {
        px >= self.x && px < self.x + self.width && py >= self.y && py < self.y + self.height
    }

    /// Get the bounds as (x, y, width, height)
    pub fn bounds(&self) -> (u16, u16, u16, u16) {
        (self.x, self.y, self.width, self.height)
    }

    /// Move the panel to a new position
    pub fn move_to(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }

    /// Resize the panel
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = self.apply_width_constraints(width);
        self.height = self.apply_height_constraints(height);
    }

    /// Apply width constraints
    fn apply_width_constraints(&self, width: u16) -> u16 {
        let mut w = width;
        if let Some(min) = self.min_width {
            w = w.max(min);
        }
        if let Some(max) = self.max_width {
            w = w.min(max);
        }
        w
    }

    /// Apply height constraints
    fn apply_height_constraints(&self, height: u16) -> u16 {
        let mut h = height;
        if let Some(min) = self.min_height {
            h = h.max(min);
        }
        if let Some(max) = self.max_height {
            h = h.min(max);
        }
        h
    }

    /// Calculate scroll bar position (returns (position, size) as fractions 0.0-1.0)
    pub fn scroll_bar_info(&self) -> Option<(f32, f32)> {
        if !self.scrollable || self.content_line_count() == 0 {
            return None;
        }

        let visible = self.inner_height() as usize;
        let total = self.content_line_count();

        if total <= visible {
            return None;
        }

        let size = (visible as f32 / total as f32).min(1.0);
        let max_offset = total - visible;
        let position = self.scroll_offset as f32 / max_offset as f32;

        Some((position, size))
    }
}

impl Default for Panel {
    fn default() -> Self {
        Self::new(PanelId::Custom(0), 0, 0, 40, 10)
    }
}

// ============================================================================
// Layout Types
// ============================================================================

/// Layout type for the UI
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Default)]
pub enum LayoutType {
    /// Classic layout: map on left, stats on right
    #[default]
    Classic,
    /// Wide layout: map on top, stats on bottom
    Wide,
    /// Compact layout: minimal UI, maximized map
    Compact,
    /// Full layout: all panels visible
    Full,
    /// Custom user-defined layout
    Custom,
}

impl LayoutType {
    /// Returns the display name for this layout
    pub fn name(&self) -> &'static str {
        match self {
            Self::Classic => "Classic",
            Self::Wide => "Wide",
            Self::Compact => "Compact",
            Self::Full => "Full",
            Self::Custom => "Custom",
        }
    }

    /// Returns a description of this layout
    pub fn description(&self) -> &'static str {
        match self {
            Self::Classic => "Map on left, stats and inventory on right",
            Self::Wide => "Map on top, stats and info on bottom",
            Self::Compact => "Minimal UI, maximum map visibility",
            Self::Full => "All panels visible with detailed information",
            Self::Custom => "User-defined panel arrangement",
        }
    }

    /// Returns all available layout types
    pub fn all() -> &'static [LayoutType] {
        &[
            Self::Classic,
            Self::Wide,
            Self::Compact,
            Self::Full,
            Self::Custom,
        ]
    }
}

// ============================================================================
// UI Components
// ============================================================================

/// Progress bar display for stats
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ProgressBar {
    pub current: i32,
    pub maximum: i32,
    pub width: u16,
    pub filled_char: char,
    pub empty_char: char,
    pub filled_color: TextColor,
    pub empty_color: TextColor,
    pub show_text: bool,
    pub label: Option<String>,
}

impl ProgressBar {
    /// Create a new progress bar
    pub fn new(current: i32, maximum: i32, width: u16) -> Self {
        Self {
            current,
            maximum,
            width,
            filled_char: '█',
            empty_char: '░',
            filled_color: TextColor::Green,
            empty_color: TextColor::BrightBlack,
            show_text: true,
            label: None,
        }
    }

    /// Create an HP bar
    pub fn hp(current: i32, maximum: i32, width: u16) -> Self {
        let ratio = current as f32 / maximum.max(1) as f32;
        let color = if ratio > 0.5 {
            TextColor::Green
        } else if ratio > 0.25 {
            TextColor::Yellow
        } else {
            TextColor::Red
        };

        Self {
            current,
            maximum,
            width,
            filled_char: '█',
            empty_char: '░',
            filled_color: color,
            empty_color: TextColor::BrightBlack,
            show_text: true,
            label: Some("HP".to_string()),
        }
    }

    /// Create an MP bar
    pub fn mp(current: i32, maximum: i32, width: u16) -> Self {
        Self {
            current,
            maximum,
            width,
            filled_char: '█',
            empty_char: '░',
            filled_color: TextColor::Blue,
            empty_color: TextColor::BrightBlack,
            show_text: true,
            label: Some("MP".to_string()),
        }
    }

    /// Create an XP bar
    pub fn xp(current: i32, maximum: i32, width: u16) -> Self {
        Self {
            current,
            maximum,
            width,
            filled_char: '█',
            empty_char: '░',
            filled_color: TextColor::Yellow,
            empty_color: TextColor::BrightBlack,
            show_text: true,
            label: Some("XP".to_string()),
        }
    }

    /// Set custom characters
    pub fn with_chars(mut self, filled: char, empty: char) -> Self {
        self.filled_char = filled;
        self.empty_char = empty;
        self
    }

    /// Set colors
    pub fn with_colors(mut self, filled: TextColor, empty: TextColor) -> Self {
        self.filled_color = filled;
        self.empty_color = empty;
        self
    }

    /// Set label
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Get the fill ratio (0.0 to 1.0)
    pub fn ratio(&self) -> f32 {
        if self.maximum <= 0 {
            0.0
        } else {
            (self.current as f32 / self.maximum as f32).clamp(0.0, 1.0)
        }
    }

    /// Render to a string
    pub fn render(&self) -> String {
        let ratio = self.ratio();
        let inner_width = if self.show_text {
            self.width.saturating_sub(10) as usize
        } else {
            self.width as usize
        };

        let filled_count = (inner_width as f32 * ratio).round() as usize;
        let empty_count = inner_width.saturating_sub(filled_count);

        let bar: String = std::iter::repeat(self.filled_char)
            .take(filled_count)
            .chain(std::iter::repeat(self.empty_char).take(empty_count))
            .collect();

        if self.show_text {
            if let Some(ref label) = self.label {
                format!("{}: {} {}/{}", label, bar, self.current, self.maximum)
            } else {
                format!("{} {}/{}", bar, self.current, self.maximum)
            }
        } else {
            bar
        }
    }
}

/// Buff/debuff display info
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BuffDisplay {
    pub name: String,
    pub icon: char,
    pub color: TextColor,
    pub duration: Option<u32>,
    pub stacks: u32,
    pub is_debuff: bool,
    pub tooltip: String,
}

impl BuffDisplay {
    /// Create a new buff display
    pub fn buff(name: impl Into<String>, icon: char, duration: Option<u32>) -> Self {
        Self {
            name: name.into(),
            icon,
            color: TextColor::Green,
            duration,
            stacks: 1,
            is_debuff: false,
            tooltip: String::new(),
        }
    }

    /// Create a new debuff display
    pub fn debuff(name: impl Into<String>, icon: char, duration: Option<u32>) -> Self {
        Self {
            name: name.into(),
            icon,
            color: TextColor::Red,
            duration,
            stacks: 1,
            is_debuff: true,
            tooltip: String::new(),
        }
    }

    /// Set stacks
    pub fn with_stacks(mut self, stacks: u32) -> Self {
        self.stacks = stacks;
        self
    }

    /// Set tooltip
    pub fn with_tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = tooltip.into();
        self
    }

    /// Render to a compact string
    pub fn render_compact(&self) -> String {
        let duration_str = self
            .duration
            .map(|d| format!(":{}", d))
            .unwrap_or_default();
        let stacks_str = if self.stacks > 1 {
            format!("x{}", self.stacks)
        } else {
            String::new()
        };
        format!("{}{}{}", self.icon, duration_str, stacks_str)
    }
}

/// Hotbar slot information
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct HotbarSlot {
    pub index: u8,
    pub key: char,
    pub content: HotbarContent,
    pub cooldown: Option<u32>,
    pub usable: bool,
}

/// Content type for hotbar slots
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum HotbarContent {
    Empty,
    Skill { name: String, icon: char },
    Item { name: String, icon: char, count: u32 },
    Consumable { name: String, icon: char, count: u32 },
}

impl HotbarSlot {
    /// Create an empty slot
    pub fn empty(index: u8, key: char) -> Self {
        Self {
            index,
            key,
            content: HotbarContent::Empty,
            cooldown: None,
            usable: false,
        }
    }

    /// Create a skill slot
    pub fn skill(index: u8, key: char, name: impl Into<String>, icon: char) -> Self {
        Self {
            index,
            key,
            content: HotbarContent::Skill {
                name: name.into(),
                icon,
            },
            cooldown: None,
            usable: true,
        }
    }

    /// Create an item slot
    pub fn item(index: u8, key: char, name: impl Into<String>, icon: char, count: u32) -> Self {
        Self {
            index,
            key,
            content: HotbarContent::Item {
                name: name.into(),
                icon,
                count,
            },
            cooldown: None,
            usable: count > 0,
        }
    }

    /// Check if the slot is empty
    pub fn is_empty(&self) -> bool {
        matches!(self.content, HotbarContent::Empty)
    }

    /// Check if on cooldown
    pub fn is_on_cooldown(&self) -> bool {
        self.cooldown.map(|c| c > 0).unwrap_or(false)
    }

    /// Render the slot to a string
    pub fn render(&self) -> String {
        match &self.content {
            HotbarContent::Empty => format!("[{}]   ", self.key),
            HotbarContent::Skill { icon, .. } => {
                if let Some(cd) = self.cooldown {
                    format!("[{}] {}:{}", self.key, icon, cd)
                } else {
                    format!("[{}] {}", self.key, icon)
                }
            }
            HotbarContent::Item { icon, count, .. }
            | HotbarContent::Consumable { icon, count, .. } => {
                format!("[{}] {}x{}", self.key, icon, count)
            }
        }
    }
}

/// Target information display
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct TargetInfo {
    pub name: String,
    pub level: u32,
    pub hp_current: i32,
    pub hp_max: i32,
    pub status_effects: Vec<String>,
    pub creature_type: String,
    pub is_boss: bool,
    pub is_elite: bool,
    pub distance: u32,
}

impl TargetInfo {
    /// Create new target info
    pub fn new(name: impl Into<String>, level: u32, hp: i32, hp_max: i32) -> Self {
        Self {
            name: name.into(),
            level,
            hp_current: hp,
            hp_max,
            status_effects: Vec::new(),
            creature_type: "Unknown".to_string(),
            is_boss: false,
            is_elite: false,
            distance: 0,
        }
    }

    /// Get HP as a percentage
    pub fn hp_percentage(&self) -> f32 {
        if self.hp_max <= 0 {
            0.0
        } else {
            (self.hp_current as f32 / self.hp_max as f32 * 100.0).clamp(0.0, 100.0)
        }
    }

    /// Get a difficulty indicator color
    pub fn difficulty_color(&self) -> TextColor {
        if self.is_boss {
            TextColor::Magenta
        } else if self.is_elite {
            TextColor::Yellow
        } else {
            TextColor::Default
        }
    }
}

// ============================================================================
// Message Log
// ============================================================================

/// A message in the message log
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LogMessage {
    pub text: String,
    pub style: TextStyle,
    pub timestamp: u32,
    pub category: MessageCategory,
    pub count: u32,
}

/// Categories for log messages
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Default)]
pub enum MessageCategory {
    #[default]
    General,
    Combat,
    Loot,
    System,
    Quest,
    Dialog,
    Error,
    Warning,
    Achievement,
}

impl MessageCategory {
    /// Get the default style for this category
    pub fn default_style(&self) -> TextStyle {
        match self {
            Self::General => TextStyle::new(),
            Self::Combat => TextStyle::new().fg(TextColor::Red),
            Self::Loot => TextStyle::new().fg(TextColor::Yellow),
            Self::System => TextStyle::new().fg(TextColor::Cyan),
            Self::Quest => TextStyle::new().fg(TextColor::Magenta),
            Self::Dialog => TextStyle::new().fg(TextColor::White),
            Self::Error => TextStyle::error(),
            Self::Warning => TextStyle::warning(),
            Self::Achievement => TextStyle::new().fg(TextColor::BrightYellow).bold(),
        }
    }
}

impl LogMessage {
    /// Create a new log message
    pub fn new(text: impl Into<String>, category: MessageCategory, timestamp: u32) -> Self {
        Self {
            text: text.into(),
            style: category.default_style(),
            timestamp,
            category,
            count: 1,
        }
    }

    /// Create with custom style
    pub fn with_style(
        text: impl Into<String>,
        style: TextStyle,
        category: MessageCategory,
        timestamp: u32,
    ) -> Self {
        Self {
            text: text.into(),
            style,
            timestamp,
            category,
            count: 1,
        }
    }
}

/// Message log with scrolling and filtering
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct MessageLog {
    messages: Vec<LogMessage>,
    max_messages: usize,
    scroll_offset: usize,
    visible_lines: usize,
    filter: Option<MessageCategory>,
    combine_duplicates: bool,
}

impl MessageLog {
    /// Create a new message log
    pub fn new(max_messages: usize, visible_lines: usize) -> Self {
        Self {
            messages: Vec::with_capacity(max_messages),
            max_messages,
            scroll_offset: 0,
            visible_lines,
            filter: None,
            combine_duplicates: true,
        }
    }

    /// Add a message to the log
    pub fn add(&mut self, message: LogMessage) {
        // Check for duplicate combining
        if self.combine_duplicates {
            if let Some(last) = self.messages.last_mut() {
                if last.text == message.text && last.category == message.category {
                    last.count += 1;
                    last.timestamp = message.timestamp;
                    return;
                }
            }
        }

        self.messages.push(message);

        // Trim old messages
        if self.messages.len() > self.max_messages {
            self.messages.remove(0);
            if self.scroll_offset > 0 {
                self.scroll_offset = self.scroll_offset.saturating_sub(1);
            }
        }

        // Auto-scroll to bottom
        self.scroll_to_bottom();
    }

    /// Add a simple text message
    pub fn add_text(&mut self, text: impl Into<String>, category: MessageCategory, timestamp: u32) {
        self.add(LogMessage::new(text, category, timestamp));
    }

    /// Get visible messages
    pub fn visible_messages(&self) -> Vec<&LogMessage> {
        let filtered: Vec<&LogMessage> = self
            .messages
            .iter()
            .filter(|m| self.filter.is_none() || Some(m.category) == self.filter)
            .collect();

        filtered
            .iter()
            .skip(self.scroll_offset)
            .take(self.visible_lines)
            .copied()
            .collect()
    }

    /// Scroll up
    pub fn scroll_up(&mut self, lines: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(lines);
    }

    /// Scroll down
    pub fn scroll_down(&mut self, lines: usize) {
        let max = self.messages.len().saturating_sub(self.visible_lines);
        self.scroll_offset = (self.scroll_offset + lines).min(max);
    }

    /// Scroll to top
    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = 0;
    }

    /// Scroll to bottom
    pub fn scroll_to_bottom(&mut self) {
        let max = self.messages.len().saturating_sub(self.visible_lines);
        self.scroll_offset = max;
    }

    /// Set message filter
    pub fn set_filter(&mut self, category: Option<MessageCategory>) {
        self.filter = category;
        self.scroll_offset = 0;
    }

    /// Clear all messages
    pub fn clear(&mut self) {
        self.messages.clear();
        self.scroll_offset = 0;
    }

    /// Get message count
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Get all messages (for saving)
    pub fn all_messages(&self) -> &[LogMessage] {
        &self.messages
    }
}

// ============================================================================
// Tooltip System
// ============================================================================

/// Tooltip display
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Tooltip {
    pub content: Vec<StyledLine>,
    pub x: u16,
    pub y: u16,
    pub visible: bool,
    pub anchor: TooltipAnchor,
    pub max_width: u16,
}

/// Tooltip positioning anchor
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Default)]
pub enum TooltipAnchor {
    #[default]
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

impl Tooltip {
    /// Create a new tooltip
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
            x: 0,
            y: 0,
            visible: false,
            anchor: TooltipAnchor::TopLeft,
            max_width: 40,
        }
    }

    /// Set content from plain text lines
    pub fn set_content(&mut self, lines: Vec<String>) {
        self.content = lines.into_iter().map(StyledLine::plain).collect();
    }

    /// Set styled content
    pub fn set_styled_content(&mut self, lines: Vec<StyledLine>) {
        self.content = lines;
    }

    /// Show at position
    pub fn show_at(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
        self.visible = true;
    }

    /// Hide the tooltip
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Calculate display position based on anchor and screen bounds
    pub fn calculate_position(&self, screen_width: u16, screen_height: u16) -> (u16, u16) {
        let width = self.content.iter().map(|l| l.width()).max().unwrap_or(0) as u16;
        let height = self.content.len() as u16;

        let (mut x, mut y) = match self.anchor {
            TooltipAnchor::TopLeft => (self.x, self.y),
            TooltipAnchor::TopRight => (self.x.saturating_sub(width), self.y),
            TooltipAnchor::BottomLeft => (self.x, self.y.saturating_sub(height)),
            TooltipAnchor::BottomRight => {
                (self.x.saturating_sub(width), self.y.saturating_sub(height))
            }
            TooltipAnchor::Center => {
                (self.x.saturating_sub(width / 2), self.y.saturating_sub(height / 2))
            }
        };

        // Ensure tooltip stays on screen
        if x + width > screen_width {
            x = screen_width.saturating_sub(width);
        }
        if y + height > screen_height {
            y = screen_height.saturating_sub(height);
        }

        (x, y)
    }
}

// ============================================================================
// UI Layout
// ============================================================================

/// Size requirements for the UI
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct SizeRequirements {
    pub min_width: u16,
    pub min_height: u16,
    pub preferred_width: u16,
    pub preferred_height: u16,
}

impl Default for SizeRequirements {
    fn default() -> Self {
        Self {
            min_width: MIN_TERMINAL_WIDTH,
            min_height: MIN_TERMINAL_HEIGHT,
            preferred_width: 120,
            preferred_height: 40,
        }
    }
}

/// Main UI layout manager
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UILayout {
    /// Current layout type
    pub layout_type: LayoutType,
    /// Terminal/window width
    pub width: u16,
    /// Terminal/window height
    pub height: u16,
    /// All panels
    pub panels: HashMap<PanelId, Panel>,
    /// Current panel focus
    pub focused_panel: Option<PanelId>,
    /// Message log
    pub message_log: MessageLog,
    /// Active tooltip
    pub tooltip: Tooltip,
    /// Global border style
    pub global_border_style: BorderStyle,
    /// Whether UI needs redraw
    pub dirty: bool,
    /// Size requirements
    pub size_requirements: SizeRequirements,
    /// Custom layout panel arrangements (for Custom layout type)
    pub custom_layout: Option<CustomLayout>,
    /// UI scale factor (1.0 = normal)
    pub scale: f32,
    /// Panel spacing
    pub panel_spacing: u16,
    /// Show debug info
    pub show_debug: bool,
}

/// Custom layout definition
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct CustomLayout {
    pub panel_positions: HashMap<PanelId, PanelPosition>,
    pub name: String,
}

/// Position and size for custom layouts
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct PanelPosition {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub visible: bool,
}

impl UILayout {
    /// Create a new UI layout with default settings
    pub fn new(width: u16, height: u16) -> Self {
        let mut layout = Self {
            layout_type: LayoutType::Classic,
            width,
            height,
            panels: HashMap::new(),
            focused_panel: None,
            message_log: MessageLog::new(MAX_MESSAGE_BUFFER, DEFAULT_MESSAGE_LOG_HEIGHT as usize),
            tooltip: Tooltip::new(),
            global_border_style: BorderStyle::Single,
            dirty: true,
            size_requirements: SizeRequirements::default(),
            custom_layout: None,
            scale: 1.0,
            panel_spacing: 0,
            show_debug: false,
        };

        layout.create_default_panels();
        layout.apply_layout();
        layout
    }

    /// Create default panels for the game
    fn create_default_panels(&mut self) {
        // Map panel (main game view)
        self.panels.insert(
            PanelId::Map,
            Panel::new(PanelId::Map, 0, 0, 60, 20)
                .with_title("Dungeon")
                .with_border(BorderStyle::Double),
        );

        // Stats panel (HP/MP/XP)
        self.panels.insert(
            PanelId::Stats,
            Panel::new(PanelId::Stats, 60, 0, 20, 10)
                .with_title("Stats")
                .scrollable(false),
        );

        // Inventory panel
        self.panels.insert(
            PanelId::Inventory,
            Panel::new(PanelId::Inventory, 60, 10, 20, 10)
                .with_title("Inventory"),
        );

        // Message log
        self.panels.insert(
            PanelId::MessageLog,
            Panel::new(PanelId::MessageLog, 0, 20, 60, 6)
                .with_title("Messages"),
        );

        // Mini-map
        self.panels.insert(
            PanelId::MiniMap,
            Panel::new(PanelId::MiniMap, 0, 0, MINI_MAP_SIZE, MINI_MAP_SIZE)
                .with_title("Map")
                .with_border(BorderStyle::Rounded)
                .with_z_index(10),
        );

        // Hotbar
        self.panels.insert(
            PanelId::Hotbar,
            Panel::new(PanelId::Hotbar, 0, 0, 80, 3)
                .with_title("Actions")
                .scrollable(false),
        );

        // Buff bar
        self.panels.insert(
            PanelId::BuffBar,
            Panel::new(PanelId::BuffBar, 0, 0, 30, 3)
                .with_title("Effects")
                .with_border(BorderStyle::Rounded)
                .scrollable(false),
        );

        // Target info
        self.panels.insert(
            PanelId::TargetInfo,
            Panel::new(PanelId::TargetInfo, 0, 0, 25, 8)
                .with_title("Target")
                .visible(false),
        );

        // Equipment panel
        self.panels.insert(
            PanelId::Equipment,
            Panel::new(PanelId::Equipment, 0, 0, 25, 12)
                .with_title("Equipment")
                .visible(false),
        );

        // Skills panel
        self.panels.insert(
            PanelId::Skills,
            Panel::new(PanelId::Skills, 0, 0, 30, 15)
                .with_title("Skills")
                .visible(false),
        );

        // Quest log
        self.panels.insert(
            PanelId::QuestLog,
            Panel::new(PanelId::QuestLog, 0, 0, 40, 20)
                .with_title("Quest Log")
                .visible(false),
        );

        // Help panel
        self.panels.insert(
            PanelId::Help,
            Panel::new(PanelId::Help, 0, 0, 50, 25)
                .with_title("Help")
                .with_border(BorderStyle::Double)
                .with_z_index(100)
                .visible(false),
        );

        // Menu panel
        self.panels.insert(
            PanelId::Menu,
            Panel::new(PanelId::Menu, 0, 0, 30, 15)
                .with_title("Menu")
                .with_border(BorderStyle::Double)
                .with_z_index(100)
                .visible(false),
        );
    }

    /// Apply current layout type
    pub fn apply_layout(&mut self) {
        match self.layout_type {
            LayoutType::Classic => self.apply_classic_layout(),
            LayoutType::Wide => self.apply_wide_layout(),
            LayoutType::Compact => self.apply_compact_layout(),
            LayoutType::Full => self.apply_full_layout(),
            LayoutType::Custom => self.apply_custom_layout(),
        }
        self.dirty = true;
    }

    /// Apply classic layout (map left, stats right)
    fn apply_classic_layout(&mut self) {
        let map_width = (self.width as f32 * DEFAULT_MAP_WIDTH_RATIO) as u16;
        let stats_width = self.width - map_width;
        let message_height = DEFAULT_MESSAGE_LOG_HEIGHT;
        let map_height = self.height - message_height;

        // Map panel
        if let Some(panel) = self.panels.get_mut(&PanelId::Map) {
            panel.x = 0;
            panel.y = 0;
            panel.width = map_width;
            panel.height = map_height;
            panel.visible = true;
        }

        // Stats panel
        if let Some(panel) = self.panels.get_mut(&PanelId::Stats) {
            panel.x = map_width;
            panel.y = 0;
            panel.width = stats_width;
            panel.height = map_height / 3;
            panel.visible = true;
        }

        // Inventory panel
        if let Some(panel) = self.panels.get_mut(&PanelId::Inventory) {
            let stats_height = map_height / 3;
            panel.x = map_width;
            panel.y = stats_height;
            panel.width = stats_width;
            panel.height = map_height - stats_height;
            panel.visible = true;
        }

        // Message log
        if let Some(panel) = self.panels.get_mut(&PanelId::MessageLog) {
            panel.x = 0;
            panel.y = map_height;
            panel.width = self.width;
            panel.height = message_height;
            panel.visible = true;
        }

        // Mini-map (top-right corner of map panel)
        if let Some(panel) = self.panels.get_mut(&PanelId::MiniMap) {
            panel.x = map_width.saturating_sub(MINI_MAP_SIZE + 1);
            panel.y = 1;
            panel.visible = false; // Hidden in classic by default
        }

        // Buff bar (above hotbar area)
        if let Some(panel) = self.panels.get_mut(&PanelId::BuffBar) {
            panel.visible = false; // Part of stats in classic
        }

        // Hotbar
        if let Some(panel) = self.panels.get_mut(&PanelId::Hotbar) {
            panel.visible = false; // Integrated into message area in classic
        }

        // Target info
        if let Some(panel) = self.panels.get_mut(&PanelId::TargetInfo) {
            panel.x = map_width.saturating_sub(26);
            panel.y = map_height.saturating_sub(9);
        }
    }

    /// Apply wide layout (map top, stats bottom)
    fn apply_wide_layout(&mut self) {
        let map_height = (self.height as f32 * 0.6) as u16;
        let bottom_height = self.height - map_height;
        let stats_width = self.width / 4;

        // Map panel (full width)
        if let Some(panel) = self.panels.get_mut(&PanelId::Map) {
            panel.x = 0;
            panel.y = 0;
            panel.width = self.width;
            panel.height = map_height;
            panel.visible = true;
        }

        // Stats panel (bottom left)
        if let Some(panel) = self.panels.get_mut(&PanelId::Stats) {
            panel.x = 0;
            panel.y = map_height;
            panel.width = stats_width;
            panel.height = bottom_height;
            panel.visible = true;
        }

        // Inventory panel (bottom middle-left)
        if let Some(panel) = self.panels.get_mut(&PanelId::Inventory) {
            panel.x = stats_width;
            panel.y = map_height;
            panel.width = stats_width;
            panel.height = bottom_height;
            panel.visible = true;
        }

        // Message log (bottom right)
        if let Some(panel) = self.panels.get_mut(&PanelId::MessageLog) {
            panel.x = stats_width * 2;
            panel.y = map_height;
            panel.width = self.width - (stats_width * 2);
            panel.height = bottom_height;
            panel.visible = true;
        }

        // Mini-map (top-right corner)
        if let Some(panel) = self.panels.get_mut(&PanelId::MiniMap) {
            panel.x = self.width.saturating_sub(MINI_MAP_SIZE + 1);
            panel.y = 1;
            panel.visible = true;
        }

        // Buff bar
        if let Some(panel) = self.panels.get_mut(&PanelId::BuffBar) {
            panel.x = 0;
            panel.y = map_height.saturating_sub(4);
            panel.width = 40;
            panel.visible = true;
        }
    }

    /// Apply compact layout (minimal UI)
    fn apply_compact_layout(&mut self) {
        let hotbar_height = DEFAULT_HOTBAR_HEIGHT;
        let map_height = self.height - hotbar_height;

        // Map panel (almost full screen)
        if let Some(panel) = self.panels.get_mut(&PanelId::Map) {
            panel.x = 0;
            panel.y = 0;
            panel.width = self.width;
            panel.height = map_height;
            panel.border_style = BorderStyle::None;
            panel.visible = true;
        }

        // Hide most panels
        for id in &[
            PanelId::Stats,
            PanelId::Inventory,
            PanelId::Equipment,
            PanelId::Skills,
            PanelId::QuestLog,
        ] {
            if let Some(panel) = self.panels.get_mut(id) {
                panel.visible = false;
            }
        }

        // Hotbar at bottom
        if let Some(panel) = self.panels.get_mut(&PanelId::Hotbar) {
            panel.x = 0;
            panel.y = map_height;
            panel.width = self.width;
            panel.height = hotbar_height;
            panel.border_style = BorderStyle::Single;
            panel.visible = true;
        }

        // Mini-map (smaller, corner)
        if let Some(panel) = self.panels.get_mut(&PanelId::MiniMap) {
            panel.x = self.width.saturating_sub(12);
            panel.y = 0;
            panel.width = 12;
            panel.height = 8;
            panel.visible = true;
        }

        // Buff bar (top overlay)
        if let Some(panel) = self.panels.get_mut(&PanelId::BuffBar) {
            panel.x = 0;
            panel.y = 0;
            panel.width = 30;
            panel.height = 1;
            panel.border_style = BorderStyle::None;
            panel.visible = true;
        }

        // Message log (overlay at bottom of map)
        if let Some(panel) = self.panels.get_mut(&PanelId::MessageLog) {
            panel.x = 0;
            panel.y = map_height.saturating_sub(4);
            panel.width = self.width / 2;
            panel.height = 4;
            panel.border_style = BorderStyle::None;
            panel.visible = true;
        }
    }

    /// Apply full layout (all panels visible)
    fn apply_full_layout(&mut self) {
        let left_width = (self.width as f32 * 0.55) as u16;
        let right_width = self.width - left_width;
        let message_height = 8;
        let hotbar_height = 3;
        let map_height = self.height - message_height - hotbar_height;

        // Map panel
        if let Some(panel) = self.panels.get_mut(&PanelId::Map) {
            panel.x = 0;
            panel.y = 0;
            panel.width = left_width;
            panel.height = map_height;
            panel.border_style = BorderStyle::Double;
            panel.visible = true;
        }

        // Stats panel
        let stats_height = 8;
        if let Some(panel) = self.panels.get_mut(&PanelId::Stats) {
            panel.x = left_width;
            panel.y = 0;
            panel.width = right_width;
            panel.height = stats_height;
            panel.visible = true;
        }

        // Buff bar (below stats)
        if let Some(panel) = self.panels.get_mut(&PanelId::BuffBar) {
            panel.x = left_width;
            panel.y = stats_height;
            panel.width = right_width;
            panel.height = 3;
            panel.visible = true;
        }

        // Inventory panel
        let inv_y = stats_height + 3;
        let inv_height = (map_height - inv_y) / 2;
        if let Some(panel) = self.panels.get_mut(&PanelId::Inventory) {
            panel.x = left_width;
            panel.y = inv_y;
            panel.width = right_width;
            panel.height = inv_height;
            panel.visible = true;
        }

        // Equipment panel
        if let Some(panel) = self.panels.get_mut(&PanelId::Equipment) {
            panel.x = left_width;
            panel.y = inv_y + inv_height;
            panel.width = right_width / 2;
            panel.height = map_height - inv_y - inv_height;
            panel.visible = true;
        }

        // Target info
        if let Some(panel) = self.panels.get_mut(&PanelId::TargetInfo) {
            panel.x = left_width + right_width / 2;
            panel.y = inv_y + inv_height;
            panel.width = right_width / 2;
            panel.height = map_height - inv_y - inv_height;
            panel.visible = true;
        }

        // Mini-map (overlay on map)
        if let Some(panel) = self.panels.get_mut(&PanelId::MiniMap) {
            panel.x = left_width.saturating_sub(MINI_MAP_SIZE + 1);
            panel.y = 1;
            panel.visible = true;
        }

        // Hotbar
        if let Some(panel) = self.panels.get_mut(&PanelId::Hotbar) {
            panel.x = 0;
            panel.y = map_height;
            panel.width = self.width;
            panel.height = hotbar_height;
            panel.visible = true;
        }

        // Message log
        if let Some(panel) = self.panels.get_mut(&PanelId::MessageLog) {
            panel.x = 0;
            panel.y = map_height + hotbar_height;
            panel.width = self.width;
            panel.height = message_height;
            panel.visible = true;
        }
    }

    /// Apply custom layout
    fn apply_custom_layout(&mut self) {
        if let Some(custom) = &self.custom_layout {
            for (id, pos) in &custom.panel_positions {
                if let Some(panel) = self.panels.get_mut(id) {
                    panel.x = pos.x;
                    panel.y = pos.y;
                    panel.width = pos.width;
                    panel.height = pos.height;
                    panel.visible = pos.visible;
                }
            }
        }
    }

    /// Resize the UI to new dimensions
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width.max(self.size_requirements.min_width);
        self.height = height.max(self.size_requirements.min_height);
        self.apply_layout();
    }

    /// Check if current size meets minimum requirements
    pub fn meets_minimum_size(&self) -> bool {
        self.width >= self.size_requirements.min_width
            && self.height >= self.size_requirements.min_height
    }

    /// Get a panel by ID
    pub fn get_panel(&self, id: PanelId) -> Option<&Panel> {
        self.panels.get(&id)
    }

    /// Get a mutable panel by ID
    pub fn get_panel_mut(&mut self, id: PanelId) -> Option<&mut Panel> {
        self.dirty = true;
        self.panels.get_mut(&id)
    }

    /// Set focus to a panel
    pub fn set_focus(&mut self, id: PanelId) {
        // Remove focus from current panel
        if let Some(old_id) = self.focused_panel {
            if let Some(panel) = self.panels.get_mut(&old_id) {
                panel.focused = false;
            }
        }

        // Set focus to new panel
        if let Some(panel) = self.panels.get_mut(&id) {
            panel.focused = true;
            self.focused_panel = Some(id);
        }
        self.dirty = true;
    }

    /// Clear focus
    pub fn clear_focus(&mut self) {
        if let Some(id) = self.focused_panel {
            if let Some(panel) = self.panels.get_mut(&id) {
                panel.focused = false;
            }
        }
        self.focused_panel = None;
        self.dirty = true;
    }

    /// Cycle focus to next visible panel
    pub fn focus_next(&mut self) {
        let visible_ids: Vec<PanelId> = self
            .panels
            .iter()
            .filter(|(_, p)| p.visible)
            .map(|(id, _)| *id)
            .collect();

        if visible_ids.is_empty() {
            return;
        }

        let next_id = if let Some(current) = self.focused_panel {
            let current_pos = visible_ids.iter().position(|&id| id == current);
            match current_pos {
                Some(pos) => visible_ids[(pos + 1) % visible_ids.len()],
                None => visible_ids[0],
            }
        } else {
            visible_ids[0]
        };

        self.set_focus(next_id);
    }

    /// Set the layout type
    pub fn set_layout(&mut self, layout: LayoutType) {
        self.layout_type = layout;
        self.apply_layout();
    }

    /// Toggle a panel's visibility
    pub fn toggle_panel(&mut self, id: PanelId) {
        if let Some(panel) = self.panels.get_mut(&id) {
            panel.visible = !panel.visible;
            self.dirty = true;
        }
    }

    /// Show a panel
    pub fn show_panel(&mut self, id: PanelId) {
        if let Some(panel) = self.panels.get_mut(&id) {
            panel.visible = true;
            self.dirty = true;
        }
    }

    /// Hide a panel
    pub fn hide_panel(&mut self, id: PanelId) {
        if let Some(panel) = self.panels.get_mut(&id) {
            panel.visible = false;
            self.dirty = true;
        }
    }

    /// Add a message to the log
    pub fn add_message(&mut self, text: impl Into<String>, category: MessageCategory) {
        // Use a simple timestamp based on message count
        let timestamp = self.message_log.len() as u32;
        self.message_log.add_text(text, category, timestamp);
        self.dirty = true;
    }

    /// Show tooltip at position
    pub fn show_tooltip(&mut self, content: Vec<String>, x: u16, y: u16) {
        self.tooltip.set_content(content);
        self.tooltip.show_at(x, y);
        self.dirty = true;
    }

    /// Hide tooltip
    pub fn hide_tooltip(&mut self) {
        self.tooltip.hide();
        self.dirty = true;
    }

    /// Get all visible panels sorted by z-index
    pub fn visible_panels_sorted(&self) -> Vec<&Panel> {
        let mut panels: Vec<&Panel> = self.panels.values().filter(|p| p.visible).collect();
        panels.sort_by_key(|p| p.z_index);
        panels
    }

    /// Find panel at screen coordinates
    pub fn panel_at(&self, x: u16, y: u16) -> Option<PanelId> {
        // Check in reverse z-order (highest z-index first)
        let mut panels: Vec<(&PanelId, &Panel)> = self
            .panels
            .iter()
            .filter(|(_, p)| p.visible)
            .collect();
        panels.sort_by_key(|(_, p)| std::cmp::Reverse(p.z_index));

        for (id, panel) in panels {
            if panel.contains_point(x, y) {
                return Some(*id);
            }
        }
        None
    }

    /// Set global border style
    pub fn set_global_border_style(&mut self, style: BorderStyle) {
        self.global_border_style = style;
        for panel in self.panels.values_mut() {
            if panel.border_style != BorderStyle::None {
                panel.border_style = style;
            }
        }
        self.dirty = true;
    }

    /// Save custom layout
    pub fn save_custom_layout(&mut self, name: impl Into<String>) {
        let mut positions = HashMap::new();
        for (id, panel) in &self.panels {
            positions.insert(
                *id,
                PanelPosition {
                    x: panel.x,
                    y: panel.y,
                    width: panel.width,
                    height: panel.height,
                    visible: panel.visible,
                },
            );
        }
        self.custom_layout = Some(CustomLayout {
            panel_positions: positions,
            name: name.into(),
        });
    }

    /// Mark UI as needing redraw
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Clear dirty flag
    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }

    /// Check if UI needs redraw
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}

impl Default for UILayout {
    fn default() -> Self {
        Self::new(MIN_TERMINAL_WIDTH, MIN_TERMINAL_HEIGHT)
    }
}

// ============================================================================
// UI Builder Helpers
// ============================================================================

/// Helper for building stat displays
pub struct StatsBuilder {
    lines: Vec<StyledLine>,
}

impl StatsBuilder {
    /// Create a new stats builder
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    /// Add a stat line
    pub fn stat(mut self, label: &str, value: impl std::fmt::Display) -> Self {
        let mut line = StyledLine::new();
        line.add(format!("{}: ", label), TextStyle::new().bold());
        line.add_plain(format!("{}", value));
        self.lines.push(line);
        self
    }

    /// Add a colored stat line
    pub fn stat_colored(
        mut self,
        label: &str,
        value: impl std::fmt::Display,
        color: TextColor,
    ) -> Self {
        let mut line = StyledLine::new();
        line.add(format!("{}: ", label), TextStyle::new().bold());
        line.add(format!("{}", value), TextStyle::new().fg(color));
        self.lines.push(line);
        self
    }

    /// Add a progress bar
    pub fn progress_bar(mut self, bar: &ProgressBar) -> Self {
        self.lines.push(StyledLine::plain(bar.render()));
        self
    }

    /// Add an empty line
    pub fn empty_line(mut self) -> Self {
        self.lines.push(StyledLine::new());
        self
    }

    /// Add a separator line
    pub fn separator(mut self, width: usize) -> Self {
        self.lines
            .push(StyledLine::plain("─".repeat(width)));
        self
    }

    /// Build into styled lines
    pub fn build(self) -> Vec<StyledLine> {
        self.lines
    }
}

impl Default for StatsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panel_creation() {
        let panel = Panel::new(PanelId::Map, 0, 0, 80, 24);
        assert_eq!(panel.id, PanelId::Map);
        assert_eq!(panel.width, 80);
        assert_eq!(panel.height, 24);
        assert!(panel.visible);
    }

    #[test]
    fn test_panel_inner_dimensions() {
        let panel = Panel::new(PanelId::Stats, 0, 0, 20, 10)
            .with_padding(Padding::uniform(1));

        // With border (2 chars) and padding (2 chars): 20 - 2 - 2 = 16
        assert_eq!(panel.inner_width(), 16);
        assert_eq!(panel.inner_height(), 6);
    }

    #[test]
    fn test_panel_scrolling() {
        let mut panel = Panel::new(PanelId::MessageLog, 0, 0, 40, 5);
        panel.set_content((0..20).map(|i| format!("Line {}", i)).collect());

        assert!(panel.can_scroll_down());
        assert!(!panel.can_scroll_up());

        panel.scroll_down(5);
        assert!(panel.can_scroll_up());
        assert!(panel.can_scroll_down());

        panel.scroll_to_bottom();
        assert!(!panel.can_scroll_down());
    }

    #[test]
    fn test_panel_selection() {
        let mut panel = Panel::new(PanelId::Inventory, 0, 0, 20, 10);
        panel.set_content(vec!["Item 1".into(), "Item 2".into(), "Item 3".into()]);

        panel.select_next();
        assert_eq!(panel.selected_index, Some(0));

        panel.select_next();
        assert_eq!(panel.selected_index, Some(1));

        panel.select_previous();
        assert_eq!(panel.selected_index, Some(0));
    }

    #[test]
    fn test_border_styles() {
        let single = BorderStyle::Single.chars();
        assert_eq!(single.horizontal, '─');
        assert_eq!(single.vertical, '│');

        let double = BorderStyle::Double.chars();
        assert_eq!(double.horizontal, '═');
        assert_eq!(double.vertical, '║');

        let rounded = BorderStyle::Rounded.chars();
        assert_eq!(rounded.top_left, '╭');
    }

    #[test]
    fn test_ui_layout_creation() {
        let layout = UILayout::new(120, 40);
        assert!(layout.panels.contains_key(&PanelId::Map));
        assert!(layout.panels.contains_key(&PanelId::Stats));
        assert!(layout.panels.contains_key(&PanelId::Inventory));
    }

    #[test]
    fn test_layout_types() {
        let mut layout = UILayout::new(120, 40);

        layout.set_layout(LayoutType::Classic);
        assert_eq!(layout.layout_type, LayoutType::Classic);

        layout.set_layout(LayoutType::Wide);
        assert_eq!(layout.layout_type, LayoutType::Wide);
    }

    #[test]
    fn test_progress_bar() {
        let bar = ProgressBar::hp(50, 100, 20);
        assert_eq!(bar.ratio(), 0.5);

        let bar_empty = ProgressBar::new(0, 100, 10);
        assert_eq!(bar_empty.ratio(), 0.0);

        let bar_full = ProgressBar::new(100, 100, 10);
        assert_eq!(bar_full.ratio(), 1.0);
    }

    #[test]
    fn test_message_log() {
        let mut log = MessageLog::new(100, 5);

        log.add_text("Test message 1", MessageCategory::General, 0);
        log.add_text("Test message 2", MessageCategory::Combat, 1);

        assert_eq!(log.len(), 2);
        assert!(!log.is_empty());
    }

    #[test]
    fn test_styled_text() {
        let mut line = StyledLine::new();
        line.add("Bold ", TextStyle::new().bold());
        line.add_plain("normal");

        assert_eq!(line.width(), 11);
        assert_eq!(line.to_plain_string(), "Bold normal");
    }

    #[test]
    fn test_tooltip() {
        let mut tooltip = Tooltip::new();
        tooltip.set_content(vec!["Line 1".into(), "Line 2".into()]);
        tooltip.show_at(10, 10);

        assert!(tooltip.visible);
        assert_eq!(tooltip.x, 10);
        assert_eq!(tooltip.y, 10);
    }

    #[test]
    fn test_panel_focus() {
        let mut layout = UILayout::new(120, 40);

        layout.set_focus(PanelId::Inventory);
        assert_eq!(layout.focused_panel, Some(PanelId::Inventory));
        assert!(layout.panels.get(&PanelId::Inventory).unwrap().focused);

        layout.clear_focus();
        assert!(layout.focused_panel.is_none());
    }

    #[test]
    fn test_panel_visibility() {
        let mut layout = UILayout::new(120, 40);

        layout.hide_panel(PanelId::MiniMap);
        assert!(!layout.panels.get(&PanelId::MiniMap).unwrap().visible);

        layout.show_panel(PanelId::MiniMap);
        assert!(layout.panels.get(&PanelId::MiniMap).unwrap().visible);

        layout.toggle_panel(PanelId::MiniMap);
        assert!(!layout.panels.get(&PanelId::MiniMap).unwrap().visible);
    }

    #[test]
    fn test_stats_builder() {
        let lines = StatsBuilder::new()
            .stat("HP", "100/100")
            .stat_colored("MP", "50/50", TextColor::Blue)
            .empty_line()
            .separator(10)
            .build();

        assert_eq!(lines.len(), 4);
    }

    #[test]
    fn test_resize() {
        let mut layout = UILayout::new(120, 40);
        layout.resize(80, 24);

        assert_eq!(layout.width, 80);
        assert_eq!(layout.height, 24);
    }

    #[test]
    fn test_hotbar_slot() {
        let empty = HotbarSlot::empty(0, '1');
        assert!(empty.is_empty());

        let skill = HotbarSlot::skill(1, '2', "Fireball", 'F');
        assert!(!skill.is_empty());
        assert!(skill.usable);
    }

    #[test]
    fn test_buff_display() {
        let buff = BuffDisplay::buff("Shield", 'S', Some(10))
            .with_stacks(3)
            .with_tooltip("Increases defense");

        assert!(!buff.is_debuff);
        assert_eq!(buff.stacks, 3);

        let debuff = BuffDisplay::debuff("Poison", 'P', Some(5));
        assert!(debuff.is_debuff);
    }

    #[test]
    fn test_target_info() {
        let target = TargetInfo::new("Goblin", 5, 30, 50);
        assert_eq!(target.hp_percentage(), 60.0);
    }
}
