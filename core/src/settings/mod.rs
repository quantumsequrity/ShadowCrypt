//! Settings and Options System
//!
//! Comprehensive settings system for ShadowCrypt including gameplay, display,
//! controls, audio, accessibility, and profile settings with persistence support.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::classes::CharacterClass;

// ============================================================================
// Constants
// ============================================================================

/// Default settings file name
pub const SETTINGS_FILE_NAME: &str = ".shadowcrypt_settings.json";

/// Current settings version for migration support
pub const SETTINGS_VERSION: u32 = 1;

/// Default auto-save interval in turns
pub const DEFAULT_AUTOSAVE_INTERVAL: u32 = 50;

/// Default combat speed multiplier
pub const DEFAULT_COMBAT_SPEED: f32 = 1.0;

/// Default animation speed multiplier
pub const DEFAULT_ANIMATION_SPEED: f32 = 1.0;

/// Default mouse sensitivity
pub const DEFAULT_MOUSE_SENSITIVITY: f32 = 1.0;

/// Default master volume
pub const DEFAULT_MASTER_VOLUME: f32 = 0.8;

/// Default screen width
pub const DEFAULT_SCREEN_WIDTH: u32 = 1280;

/// Default screen height
pub const DEFAULT_SCREEN_HEIGHT: u32 = 720;

/// Default font size
pub const DEFAULT_FONT_SIZE: u32 = 14;

// ============================================================================
// Difficulty Settings
// ============================================================================

/// Game difficulty levels
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Difficulty {
    /// Relaxed experience for casual play
    Easy,
    /// Balanced challenge for most players
    #[default]
    Normal,
    /// Increased difficulty with tougher enemies
    Hard,
    /// Punishing difficulty with aggressive AI
    Nightmare,
    /// Ultimate challenge - one mistake can be fatal
    Hell,
}

impl Difficulty {
    /// Returns all difficulty levels
    pub fn all() -> &'static [Difficulty] {
        &[
            Self::Easy,
            Self::Normal,
            Self::Hard,
            Self::Nightmare,
            Self::Hell,
        ]
    }

    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Easy => "Easy",
            Self::Normal => "Normal",
            Self::Hard => "Hard",
            Self::Nightmare => "Nightmare",
            Self::Hell => "Hell",
        }
    }

    /// Returns a description of the difficulty
    pub fn description(&self) -> &'static str {
        match self {
            Self::Easy => "Relaxed experience with forgiving combat and plentiful resources.",
            Self::Normal => "Balanced challenge suitable for most players.",
            Self::Hard => "Tougher enemies, scarcer resources, and reduced healing.",
            Self::Nightmare => "Aggressive AI, deadly traps, and punishing mechanics.",
            Self::Hell => "The ultimate challenge - every decision matters.",
        }
    }

    /// Returns the damage multiplier for enemies
    pub fn enemy_damage_multiplier(&self) -> f32 {
        match self {
            Self::Easy => 0.7,
            Self::Normal => 1.0,
            Self::Hard => 1.3,
            Self::Nightmare => 1.6,
            Self::Hell => 2.0,
        }
    }

    /// Returns the health multiplier for enemies
    pub fn enemy_health_multiplier(&self) -> f32 {
        match self {
            Self::Easy => 0.8,
            Self::Normal => 1.0,
            Self::Hard => 1.25,
            Self::Nightmare => 1.5,
            Self::Hell => 2.0,
        }
    }

    /// Returns the XP multiplier
    pub fn xp_multiplier(&self) -> f32 {
        match self {
            Self::Easy => 0.8,
            Self::Normal => 1.0,
            Self::Hard => 1.2,
            Self::Nightmare => 1.4,
            Self::Hell => 1.5,
        }
    }

    /// Returns the loot drop rate multiplier
    pub fn loot_multiplier(&self) -> f32 {
        match self {
            Self::Easy => 1.3,
            Self::Normal => 1.0,
            Self::Hard => 0.9,
            Self::Nightmare => 0.8,
            Self::Hell => 0.7,
        }
    }

    /// Returns the healing effectiveness multiplier
    pub fn healing_multiplier(&self) -> f32 {
        match self {
            Self::Easy => 1.2,
            Self::Normal => 1.0,
            Self::Hard => 0.9,
            Self::Nightmare => 0.75,
            Self::Hell => 0.5,
        }
    }

    /// Returns whether permadeath is forced at this difficulty
    pub fn forces_permadeath(&self) -> bool {
        matches!(self, Self::Hell)
    }
}

// ============================================================================
// Auto-Loot Options
// ============================================================================

/// Auto-loot configuration
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoLootOptions {
    /// Master toggle for auto-loot
    pub enabled: bool,
    /// Auto-pickup gold
    pub pickup_gold: bool,
    /// Auto-pickup potions
    pub pickup_potions: bool,
    /// Auto-pickup equipment
    pub pickup_equipment: bool,
    /// Auto-pickup materials
    pub pickup_materials: bool,
    /// Auto-pickup keys and quest items
    pub pickup_keys: bool,
    /// Auto-pickup scrolls
    pub pickup_scrolls: bool,
    /// Minimum rarity to auto-pickup equipment
    pub min_equipment_rarity: ItemRarity,
    /// Auto-equip if better than current
    pub auto_equip_better: bool,
    /// Auto-sell junk items at vendors
    pub auto_sell_junk: bool,
}

impl Default for AutoLootOptions {
    fn default() -> Self {
        Self {
            enabled: true,
            pickup_gold: true,
            pickup_potions: true,
            pickup_equipment: true,
            pickup_materials: true,
            pickup_keys: true,
            pickup_scrolls: true,
            min_equipment_rarity: ItemRarity::Common,
            auto_equip_better: false,
            auto_sell_junk: false,
        }
    }
}

/// Item rarity for auto-loot filtering
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub enum ItemRarity {
    /// Gray items
    Junk,
    /// White items
    #[default]
    Common,
    /// Green items
    Uncommon,
    /// Blue items
    Rare,
    /// Purple items
    Epic,
    /// Orange items
    Legendary,
    /// Red items
    Mythic,
}

impl ItemRarity {
    pub fn all() -> &'static [ItemRarity] {
        &[
            Self::Junk,
            Self::Common,
            Self::Uncommon,
            Self::Rare,
            Self::Epic,
            Self::Legendary,
            Self::Mythic,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Junk => "Junk",
            Self::Common => "Common",
            Self::Uncommon => "Uncommon",
            Self::Rare => "Rare",
            Self::Epic => "Epic",
            Self::Legendary => "Legendary",
            Self::Mythic => "Mythic",
        }
    }
}

// ============================================================================
// Tutorial Settings
// ============================================================================

/// Tutorial toggles for different game systems
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TutorialSettings {
    /// Master toggle for all tutorials
    pub enabled: bool,
    /// Show movement tutorial
    pub movement: bool,
    /// Show combat tutorial
    pub combat: bool,
    /// Show inventory tutorial
    pub inventory: bool,
    /// Show skills tutorial
    pub skills: bool,
    /// Show crafting tutorial
    pub crafting: bool,
    /// Show magic tutorial
    pub magic: bool,
    /// Show quest tutorial
    pub quests: bool,
    /// Show dungeon navigation tutorial
    pub dungeon: bool,
    /// Show companion tutorial
    pub companions: bool,
    /// Show guild tutorial
    pub guilds: bool,
    /// Show trading tutorial
    pub trading: bool,
    /// Show tips during loading screens
    pub loading_tips: bool,
    /// Show contextual hints
    pub contextual_hints: bool,
}

impl Default for TutorialSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            movement: true,
            combat: true,
            inventory: true,
            skills: true,
            crafting: true,
            magic: true,
            quests: true,
            dungeon: true,
            companions: true,
            guilds: true,
            trading: true,
            loading_tips: true,
            contextual_hints: true,
        }
    }
}

impl TutorialSettings {
    /// Disable all tutorials
    pub fn disable_all(&mut self) {
        self.enabled = false;
        self.movement = false;
        self.combat = false;
        self.inventory = false;
        self.skills = false;
        self.crafting = false;
        self.magic = false;
        self.quests = false;
        self.dungeon = false;
        self.companions = false;
        self.guilds = false;
        self.trading = false;
        self.loading_tips = false;
        self.contextual_hints = false;
    }

    /// Enable all tutorials
    pub fn enable_all(&mut self) {
        *self = Self::default();
    }

    /// Mark a specific tutorial as completed
    pub fn complete_tutorial(&mut self, tutorial: TutorialType) {
        match tutorial {
            TutorialType::Movement => self.movement = false,
            TutorialType::Combat => self.combat = false,
            TutorialType::Inventory => self.inventory = false,
            TutorialType::Skills => self.skills = false,
            TutorialType::Crafting => self.crafting = false,
            TutorialType::Magic => self.magic = false,
            TutorialType::Quests => self.quests = false,
            TutorialType::Dungeon => self.dungeon = false,
            TutorialType::Companions => self.companions = false,
            TutorialType::Guilds => self.guilds = false,
            TutorialType::Trading => self.trading = false,
        }
    }
}

/// Tutorial types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TutorialType {
    Movement,
    Combat,
    Inventory,
    Skills,
    Crafting,
    Magic,
    Quests,
    Dungeon,
    Companions,
    Guilds,
    Trading,
}

// ============================================================================
// Gameplay Settings
// ============================================================================

/// Combat speed presets
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum CombatSpeed {
    /// 0.5x speed - slow and deliberate
    Slow,
    /// 1.0x speed - normal pace
    #[default]
    Normal,
    /// 1.5x speed - faster action
    Fast,
    /// 2.0x speed - rapid combat
    VeryFast,
    /// Instant combat resolution
    Instant,
}

impl CombatSpeed {
    pub fn all() -> &'static [CombatSpeed] {
        &[
            Self::Slow,
            Self::Normal,
            Self::Fast,
            Self::VeryFast,
            Self::Instant,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Slow => "Slow",
            Self::Normal => "Normal",
            Self::Fast => "Fast",
            Self::VeryFast => "Very Fast",
            Self::Instant => "Instant",
        }
    }

    pub fn multiplier(&self) -> f32 {
        match self {
            Self::Slow => 0.5,
            Self::Normal => 1.0,
            Self::Fast => 1.5,
            Self::VeryFast => 2.0,
            Self::Instant => 100.0,
        }
    }
}

/// Core gameplay settings
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GameplaySettings {
    /// Game difficulty level
    pub difficulty: Difficulty,
    /// Auto-save interval in turns (0 = disabled)
    pub autosave_interval: u32,
    /// Combat speed setting
    pub combat_speed: CombatSpeed,
    /// Custom combat speed multiplier (used when combat_speed allows)
    pub combat_speed_custom: f32,
    /// Auto-loot configuration
    pub auto_loot: AutoLootOptions,
    /// Tutorial settings
    pub tutorials: TutorialSettings,
    /// Permadeath mode - character is deleted on death
    pub permadeath: bool,
    /// Ironman mode - only one save slot, no save scumming
    pub ironman: bool,
    /// Confirm before resting
    pub confirm_rest: bool,
    /// Confirm before using consumables
    pub confirm_consumables: bool,
    /// Confirm before entering boss rooms
    pub confirm_boss_entry: bool,
    /// Auto-pause on low health (percent threshold, 0 = disabled)
    pub auto_pause_low_health: u32,
    /// Auto-pause when spotted by enemy
    pub auto_pause_enemy_spotted: bool,
    /// Show damage numbers
    pub show_damage_numbers: bool,
    /// Show floating combat text
    pub show_combat_text: bool,
    /// Enable gore effects
    pub gore_enabled: bool,
    /// Camera shake intensity (0.0 - 1.0)
    pub camera_shake: f32,
    /// Auto-explore enabled
    pub auto_explore: bool,
    /// Auto-rest when idle and safe
    pub auto_rest: bool,
    /// Skip already-seen dialogue
    pub skip_seen_dialogue: bool,
}

impl Default for GameplaySettings {
    fn default() -> Self {
        Self {
            difficulty: Difficulty::default(),
            autosave_interval: DEFAULT_AUTOSAVE_INTERVAL,
            combat_speed: CombatSpeed::default(),
            combat_speed_custom: DEFAULT_COMBAT_SPEED,
            auto_loot: AutoLootOptions::default(),
            tutorials: TutorialSettings::default(),
            permadeath: false,
            ironman: false,
            confirm_rest: true,
            confirm_consumables: false,
            confirm_boss_entry: true,
            auto_pause_low_health: 20,
            auto_pause_enemy_spotted: false,
            show_damage_numbers: true,
            show_combat_text: true,
            gore_enabled: true,
            camera_shake: 0.5,
            auto_explore: true,
            auto_rest: false,
            skip_seen_dialogue: false,
        }
    }
}

// ============================================================================
// Display Settings
// ============================================================================

/// Screen mode
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ScreenMode {
    /// Windowed mode
    #[default]
    Windowed,
    /// Borderless windowed (fake fullscreen)
    BorderlessWindowed,
    /// Exclusive fullscreen
    Fullscreen,
}

impl ScreenMode {
    pub fn all() -> &'static [ScreenMode] {
        &[Self::Windowed, Self::BorderlessWindowed, Self::Fullscreen]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Windowed => "Windowed",
            Self::BorderlessWindowed => "Borderless Windowed",
            Self::Fullscreen => "Fullscreen",
        }
    }
}

/// Screen resolution preset
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl Resolution {
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Common resolution presets
    pub fn presets() -> &'static [Resolution] {
        &[
            Resolution::new(800, 600),
            Resolution::new(1024, 768),
            Resolution::new(1280, 720),
            Resolution::new(1280, 800),
            Resolution::new(1366, 768),
            Resolution::new(1440, 900),
            Resolution::new(1600, 900),
            Resolution::new(1920, 1080),
            Resolution::new(2560, 1440),
            Resolution::new(3840, 2160),
        ]
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}

impl Default for Resolution {
    fn default() -> Self {
        Self::new(DEFAULT_SCREEN_WIDTH, DEFAULT_SCREEN_HEIGHT)
    }
}

/// Color scheme/theme
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ColorScheme {
    /// Classic dark dungeon theme
    #[default]
    DarkDungeon,
    /// Light theme for bright environments
    LightCastle,
    /// Green terminal aesthetic
    RetroTerminal,
    /// Purple and pink neon
    Cyberpunk,
    /// Warm sepia tones
    Parchment,
    /// Cool blue theme
    IceKingdom,
    /// Warm orange/red theme
    InfernalDepths,
    /// Forest green theme
    ElvenWoods,
    /// Gray stone theme
    DwarvenHalls,
    /// Pure black and white
    HighContrast,
    /// User-defined custom theme
    Custom,
}

impl ColorScheme {
    pub fn all() -> &'static [ColorScheme] {
        &[
            Self::DarkDungeon,
            Self::LightCastle,
            Self::RetroTerminal,
            Self::Cyberpunk,
            Self::Parchment,
            Self::IceKingdom,
            Self::InfernalDepths,
            Self::ElvenWoods,
            Self::DwarvenHalls,
            Self::HighContrast,
            Self::Custom,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::DarkDungeon => "Dark Dungeon",
            Self::LightCastle => "Light Castle",
            Self::RetroTerminal => "Retro Terminal",
            Self::Cyberpunk => "Cyberpunk",
            Self::Parchment => "Parchment",
            Self::IceKingdom => "Ice Kingdom",
            Self::InfernalDepths => "Infernal Depths",
            Self::ElvenWoods => "Elven Woods",
            Self::DwarvenHalls => "Dwarven Halls",
            Self::HighContrast => "High Contrast",
            Self::Custom => "Custom",
        }
    }

    /// Returns the background color (RGBA hex)
    pub fn background_color(&self) -> u32 {
        match self {
            Self::DarkDungeon => 0x1a1a2eFF,
            Self::LightCastle => 0xf5f5f5FF,
            Self::RetroTerminal => 0x0d0d0dFF,
            Self::Cyberpunk => 0x0f0f23FF,
            Self::Parchment => 0xf4ecd8FF,
            Self::IceKingdom => 0x1a2a3aFF,
            Self::InfernalDepths => 0x2a1a1aFF,
            Self::ElvenWoods => 0x1a2a1aFF,
            Self::DwarvenHalls => 0x2a2a2aFF,
            Self::HighContrast => 0x000000FF,
            Self::Custom => 0x1a1a2eFF,
        }
    }

    /// Returns the primary text color (RGBA hex)
    pub fn text_color(&self) -> u32 {
        match self {
            Self::DarkDungeon => 0xe0e0e0FF,
            Self::LightCastle => 0x1a1a1aFF,
            Self::RetroTerminal => 0x00ff00FF,
            Self::Cyberpunk => 0xff00ffFF,
            Self::Parchment => 0x3a3a3aFF,
            Self::IceKingdom => 0xc0e0ffFF,
            Self::InfernalDepths => 0xffc0c0FF,
            Self::ElvenWoods => 0xc0ffc0FF,
            Self::DwarvenHalls => 0xd0d0d0FF,
            Self::HighContrast => 0xffffffFF,
            Self::Custom => 0xe0e0e0FF,
        }
    }
}

/// Custom color theme
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomTheme {
    pub background: u32,
    pub foreground: u32,
    pub primary: u32,
    pub secondary: u32,
    pub accent: u32,
    pub success: u32,
    pub warning: u32,
    pub error: u32,
    pub health_bar: u32,
    pub mana_bar: u32,
    pub xp_bar: u32,
}

impl Default for CustomTheme {
    fn default() -> Self {
        Self {
            background: 0x1a1a2eFF,
            foreground: 0xe0e0e0FF,
            primary: 0x4a90d9FF,
            secondary: 0x6c757dFF,
            accent: 0xffc107FF,
            success: 0x28a745FF,
            warning: 0xffc107FF,
            error: 0xdc3545FF,
            health_bar: 0xdc3545FF,
            mana_bar: 0x007bffFF,
            xp_bar: 0xffc107FF,
        }
    }
}

/// Colorblind mode options
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ColorblindMode {
    /// No color adjustment
    #[default]
    None,
    /// Protanopia (red-blind)
    Protanopia,
    /// Deuteranopia (green-blind)
    Deuteranopia,
    /// Tritanopia (blue-blind)
    Tritanopia,
    /// Achromatopsia (complete color blindness)
    Achromatopsia,
}

impl ColorblindMode {
    pub fn all() -> &'static [ColorblindMode] {
        &[
            Self::None,
            Self::Protanopia,
            Self::Deuteranopia,
            Self::Tritanopia,
            Self::Achromatopsia,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Protanopia => "Protanopia (Red-Blind)",
            Self::Deuteranopia => "Deuteranopia (Green-Blind)",
            Self::Tritanopia => "Tritanopia (Blue-Blind)",
            Self::Achromatopsia => "Achromatopsia (Monochrome)",
        }
    }
}

/// Animation speed preset
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum AnimationSpeed {
    /// Very slow animations
    Slow,
    /// Normal animation speed
    #[default]
    Normal,
    /// Faster animations
    Fast,
    /// Skip most animations
    VeryFast,
    /// No animations (instant)
    None,
}

impl AnimationSpeed {
    pub fn all() -> &'static [AnimationSpeed] {
        &[
            Self::Slow,
            Self::Normal,
            Self::Fast,
            Self::VeryFast,
            Self::None,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Slow => "Slow",
            Self::Normal => "Normal",
            Self::Fast => "Fast",
            Self::VeryFast => "Very Fast",
            Self::None => "None",
        }
    }

    pub fn multiplier(&self) -> f32 {
        match self {
            Self::Slow => 0.5,
            Self::Normal => 1.0,
            Self::Fast => 2.0,
            Self::VeryFast => 4.0,
            Self::None => 100.0,
        }
    }
}

/// UI element visibility options
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UIVisibility {
    /// Show minimap
    pub minimap: bool,
    /// Show health bar
    pub health_bar: bool,
    /// Show mana bar
    pub mana_bar: bool,
    /// Show experience bar
    pub xp_bar: bool,
    /// Show hotbar/quickslots
    pub hotbar: bool,
    /// Show buff/debuff icons
    pub buffs: bool,
    /// Show enemy health bars
    pub enemy_health: bool,
    /// Show damage numbers
    pub damage_numbers: bool,
    /// Show item labels on ground
    pub item_labels: bool,
    /// Show NPC names
    pub npc_names: bool,
    /// Show grid overlay
    pub grid_overlay: bool,
    /// Show coordinates
    pub coordinates: bool,
    /// Show FPS counter
    pub fps_counter: bool,
    /// Show clock/game time
    pub game_clock: bool,
    /// Show quest tracker
    pub quest_tracker: bool,
    /// Show chat/message log
    pub message_log: bool,
    /// Show inventory weight
    pub inventory_weight: bool,
    /// Show gold count
    pub gold_display: bool,
}

impl Default for UIVisibility {
    fn default() -> Self {
        Self {
            minimap: true,
            health_bar: true,
            mana_bar: true,
            xp_bar: true,
            hotbar: true,
            buffs: true,
            enemy_health: true,
            damage_numbers: true,
            item_labels: true,
            npc_names: true,
            grid_overlay: false,
            coordinates: false,
            fps_counter: false,
            game_clock: true,
            quest_tracker: true,
            message_log: true,
            inventory_weight: true,
            gold_display: true,
        }
    }
}

/// Display settings
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DisplaySettings {
    /// Screen resolution
    pub resolution: Resolution,
    /// Screen mode (windowed, fullscreen, etc.)
    pub screen_mode: ScreenMode,
    /// VSync enabled
    pub vsync: bool,
    /// Frame rate limit (0 = unlimited)
    pub fps_limit: u32,
    /// Font size for UI text
    pub font_size: u32,
    /// Font size for messages/chat
    pub message_font_size: u32,
    /// Color scheme
    pub color_scheme: ColorScheme,
    /// Custom theme colors (used when color_scheme is Custom)
    pub custom_theme: CustomTheme,
    /// Colorblind mode
    pub colorblind_mode: ColorblindMode,
    /// Animation speed
    pub animation_speed: AnimationSpeed,
    /// UI element visibility
    pub ui_visibility: UIVisibility,
    /// UI scale (0.5 - 2.0)
    pub ui_scale: f32,
    /// Minimap scale (0.5 - 2.0)
    pub minimap_scale: f32,
    /// Minimap transparency (0.0 - 1.0)
    pub minimap_opacity: f32,
    /// Tile size in pixels
    pub tile_size: u32,
    /// Enable particle effects
    pub particles_enabled: bool,
    /// Particle density (0.0 - 1.0)
    pub particle_density: f32,
    /// Screen brightness (0.5 - 1.5)
    pub brightness: f32,
    /// Screen contrast (0.5 - 1.5)
    pub contrast: f32,
    /// Screen gamma (0.5 - 2.0)
    pub gamma: f32,
    /// Show tooltips
    pub tooltips_enabled: bool,
    /// Tooltip delay in milliseconds
    pub tooltip_delay_ms: u32,
}

impl Default for DisplaySettings {
    fn default() -> Self {
        Self {
            resolution: Resolution::default(),
            screen_mode: ScreenMode::default(),
            vsync: true,
            fps_limit: 60,
            font_size: DEFAULT_FONT_SIZE,
            message_font_size: 12,
            color_scheme: ColorScheme::default(),
            custom_theme: CustomTheme::default(),
            colorblind_mode: ColorblindMode::default(),
            animation_speed: AnimationSpeed::default(),
            ui_visibility: UIVisibility::default(),
            ui_scale: 1.0,
            minimap_scale: 1.0,
            minimap_opacity: 0.8,
            tile_size: 32,
            particles_enabled: true,
            particle_density: 1.0,
            brightness: 1.0,
            contrast: 1.0,
            gamma: 1.0,
            tooltips_enabled: true,
            tooltip_delay_ms: 500,
        }
    }
}

// ============================================================================
// Control Settings
// ============================================================================

/// Game actions that can be bound to keys
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameAction {
    // Movement
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    MoveUpLeft,
    MoveUpRight,
    MoveDownLeft,
    MoveDownRight,
    Wait,

    // Combat
    Attack,
    UseSkill1,
    UseSkill2,
    UseSkill3,
    UseSkill4,
    UseSkill5,
    UseSkill6,
    UseSkill7,
    UseSkill8,
    UseSkill9,
    UseSkill10,
    CycleTargetNext,
    CycleTargetPrev,
    ConfirmTarget,
    CancelTarget,

    // Items
    UseQuickItem1,
    UseQuickItem2,
    UseQuickItem3,
    UseQuickItem4,
    UseQuickItem5,
    UseQuickItem6,
    PickupItem,
    DropItem,
    UseItem,

    // Menus
    OpenInventory,
    OpenCharacter,
    OpenSkills,
    OpenMap,
    OpenQuests,
    OpenSettings,
    OpenHelp,
    OpenChat,

    // System
    Pause,
    QuickSave,
    QuickLoad,
    Screenshot,
    ToggleFullscreen,

    // Interaction
    Interact,
    Talk,
    Rest,
    AutoExplore,
    Search,

    // Camera/View
    ZoomIn,
    ZoomOut,
    CenterCamera,
    RotateCameraLeft,
    RotateCameraRight,
}

impl GameAction {
    pub fn all() -> Vec<GameAction> {
        vec![
            Self::MoveUp, Self::MoveDown, Self::MoveLeft, Self::MoveRight,
            Self::MoveUpLeft, Self::MoveUpRight, Self::MoveDownLeft, Self::MoveDownRight,
            Self::Wait, Self::Attack,
            Self::UseSkill1, Self::UseSkill2, Self::UseSkill3, Self::UseSkill4, Self::UseSkill5,
            Self::UseSkill6, Self::UseSkill7, Self::UseSkill8, Self::UseSkill9, Self::UseSkill10,
            Self::CycleTargetNext, Self::CycleTargetPrev, Self::ConfirmTarget, Self::CancelTarget,
            Self::UseQuickItem1, Self::UseQuickItem2, Self::UseQuickItem3,
            Self::UseQuickItem4, Self::UseQuickItem5, Self::UseQuickItem6,
            Self::PickupItem, Self::DropItem, Self::UseItem,
            Self::OpenInventory, Self::OpenCharacter, Self::OpenSkills, Self::OpenMap,
            Self::OpenQuests, Self::OpenSettings, Self::OpenHelp, Self::OpenChat,
            Self::Pause, Self::QuickSave, Self::QuickLoad, Self::Screenshot, Self::ToggleFullscreen,
            Self::Interact, Self::Talk, Self::Rest, Self::AutoExplore, Self::Search,
            Self::ZoomIn, Self::ZoomOut, Self::CenterCamera, Self::RotateCameraLeft, Self::RotateCameraRight,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::MoveUp => "Move Up",
            Self::MoveDown => "Move Down",
            Self::MoveLeft => "Move Left",
            Self::MoveRight => "Move Right",
            Self::MoveUpLeft => "Move Up-Left",
            Self::MoveUpRight => "Move Up-Right",
            Self::MoveDownLeft => "Move Down-Left",
            Self::MoveDownRight => "Move Down-Right",
            Self::Wait => "Wait",
            Self::Attack => "Attack",
            Self::UseSkill1 => "Use Skill 1",
            Self::UseSkill2 => "Use Skill 2",
            Self::UseSkill3 => "Use Skill 3",
            Self::UseSkill4 => "Use Skill 4",
            Self::UseSkill5 => "Use Skill 5",
            Self::UseSkill6 => "Use Skill 6",
            Self::UseSkill7 => "Use Skill 7",
            Self::UseSkill8 => "Use Skill 8",
            Self::UseSkill9 => "Use Skill 9",
            Self::UseSkill10 => "Use Skill 10",
            Self::CycleTargetNext => "Next Target",
            Self::CycleTargetPrev => "Previous Target",
            Self::ConfirmTarget => "Confirm Target",
            Self::CancelTarget => "Cancel Target",
            Self::UseQuickItem1 => "Quick Item 1",
            Self::UseQuickItem2 => "Quick Item 2",
            Self::UseQuickItem3 => "Quick Item 3",
            Self::UseQuickItem4 => "Quick Item 4",
            Self::UseQuickItem5 => "Quick Item 5",
            Self::UseQuickItem6 => "Quick Item 6",
            Self::PickupItem => "Pickup Item",
            Self::DropItem => "Drop Item",
            Self::UseItem => "Use Item",
            Self::OpenInventory => "Inventory",
            Self::OpenCharacter => "Character Sheet",
            Self::OpenSkills => "Skills",
            Self::OpenMap => "Map",
            Self::OpenQuests => "Quest Log",
            Self::OpenSettings => "Settings",
            Self::OpenHelp => "Help",
            Self::OpenChat => "Chat/Log",
            Self::Pause => "Pause",
            Self::QuickSave => "Quick Save",
            Self::QuickLoad => "Quick Load",
            Self::Screenshot => "Screenshot",
            Self::ToggleFullscreen => "Toggle Fullscreen",
            Self::Interact => "Interact",
            Self::Talk => "Talk",
            Self::Rest => "Rest",
            Self::AutoExplore => "Auto-Explore",
            Self::Search => "Search",
            Self::ZoomIn => "Zoom In",
            Self::ZoomOut => "Zoom Out",
            Self::CenterCamera => "Center Camera",
            Self::RotateCameraLeft => "Rotate Camera Left",
            Self::RotateCameraRight => "Rotate Camera Right",
        }
    }

    pub fn category(&self) -> &'static str {
        match self {
            Self::MoveUp | Self::MoveDown | Self::MoveLeft | Self::MoveRight |
            Self::MoveUpLeft | Self::MoveUpRight | Self::MoveDownLeft | Self::MoveDownRight |
            Self::Wait => "Movement",

            Self::Attack | Self::UseSkill1 | Self::UseSkill2 | Self::UseSkill3 |
            Self::UseSkill4 | Self::UseSkill5 | Self::UseSkill6 | Self::UseSkill7 |
            Self::UseSkill8 | Self::UseSkill9 | Self::UseSkill10 |
            Self::CycleTargetNext | Self::CycleTargetPrev | Self::ConfirmTarget |
            Self::CancelTarget => "Combat",

            Self::UseQuickItem1 | Self::UseQuickItem2 | Self::UseQuickItem3 |
            Self::UseQuickItem4 | Self::UseQuickItem5 | Self::UseQuickItem6 |
            Self::PickupItem | Self::DropItem | Self::UseItem => "Items",

            Self::OpenInventory | Self::OpenCharacter | Self::OpenSkills |
            Self::OpenMap | Self::OpenQuests | Self::OpenSettings |
            Self::OpenHelp | Self::OpenChat => "Menus",

            Self::Pause | Self::QuickSave | Self::QuickLoad |
            Self::Screenshot | Self::ToggleFullscreen => "System",

            Self::Interact | Self::Talk | Self::Rest | Self::AutoExplore |
            Self::Search => "Interaction",

            Self::ZoomIn | Self::ZoomOut | Self::CenterCamera |
            Self::RotateCameraLeft | Self::RotateCameraRight => "Camera",
        }
    }
}

/// Keyboard key representation
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyBinding {
    /// Primary key code
    pub key: String,
    /// Modifier keys (Ctrl, Shift, Alt)
    pub modifiers: Vec<KeyModifier>,
}

impl KeyBinding {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            modifiers: vec![],
        }
    }

    pub fn with_ctrl(mut self) -> Self {
        self.modifiers.push(KeyModifier::Ctrl);
        self
    }

    pub fn with_shift(mut self) -> Self {
        self.modifiers.push(KeyModifier::Shift);
        self
    }

    pub fn with_alt(mut self) -> Self {
        self.modifiers.push(KeyModifier::Alt);
        self
    }

    pub fn display(&self) -> String {
        let mut parts = Vec::new();
        for modifier in &self.modifiers {
            parts.push(modifier.name());
        }
        parts.push(&self.key);
        parts.join("+")
    }
}

/// Modifier keys
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyModifier {
    Ctrl,
    Shift,
    Alt,
    Meta,
}

impl KeyModifier {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Ctrl => "Ctrl",
            Self::Shift => "Shift",
            Self::Alt => "Alt",
            Self::Meta => "Meta",
        }
    }
}

/// Mouse button
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Button4,
    Button5,
}

impl MouseButton {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Left => "Left Click",
            Self::Right => "Right Click",
            Self::Middle => "Middle Click",
            Self::Button4 => "Mouse Button 4",
            Self::Button5 => "Mouse Button 5",
        }
    }
}

/// Controller button
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControllerButton {
    A,
    B,
    X,
    Y,
    LeftBumper,
    RightBumper,
    LeftTrigger,
    RightTrigger,
    LeftStick,
    RightStick,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    Start,
    Select,
    Guide,
}

impl ControllerButton {
    pub fn all() -> &'static [ControllerButton] {
        &[
            Self::A, Self::B, Self::X, Self::Y,
            Self::LeftBumper, Self::RightBumper,
            Self::LeftTrigger, Self::RightTrigger,
            Self::LeftStick, Self::RightStick,
            Self::DPadUp, Self::DPadDown, Self::DPadLeft, Self::DPadRight,
            Self::Start, Self::Select, Self::Guide,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::A => "A",
            Self::B => "B",
            Self::X => "X",
            Self::Y => "Y",
            Self::LeftBumper => "LB",
            Self::RightBumper => "RB",
            Self::LeftTrigger => "LT",
            Self::RightTrigger => "RT",
            Self::LeftStick => "L3",
            Self::RightStick => "R3",
            Self::DPadUp => "D-Pad Up",
            Self::DPadDown => "D-Pad Down",
            Self::DPadLeft => "D-Pad Left",
            Self::DPadRight => "D-Pad Right",
            Self::Start => "Start",
            Self::Select => "Select",
            Self::Guide => "Guide",
        }
    }
}

/// Touch gesture
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TouchGesture {
    Tap,
    DoubleTap,
    LongPress,
    SwipeUp,
    SwipeDown,
    SwipeLeft,
    SwipeRight,
    PinchIn,
    PinchOut,
    TwoFingerTap,
    ThreeFingerTap,
}

impl TouchGesture {
    pub fn all() -> &'static [TouchGesture] {
        &[
            Self::Tap, Self::DoubleTap, Self::LongPress,
            Self::SwipeUp, Self::SwipeDown, Self::SwipeLeft, Self::SwipeRight,
            Self::PinchIn, Self::PinchOut,
            Self::TwoFingerTap, Self::ThreeFingerTap,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Tap => "Tap",
            Self::DoubleTap => "Double Tap",
            Self::LongPress => "Long Press",
            Self::SwipeUp => "Swipe Up",
            Self::SwipeDown => "Swipe Down",
            Self::SwipeLeft => "Swipe Left",
            Self::SwipeRight => "Swipe Right",
            Self::PinchIn => "Pinch In",
            Self::PinchOut => "Pinch Out",
            Self::TwoFingerTap => "Two-Finger Tap",
            Self::ThreeFingerTap => "Three-Finger Tap",
        }
    }
}

/// Control settings
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ControlSettings {
    /// Keyboard bindings
    pub keybindings: HashMap<GameAction, Vec<KeyBinding>>,
    /// Mouse button bindings
    pub mouse_bindings: HashMap<GameAction, MouseButton>,
    /// Controller button bindings
    pub controller_bindings: HashMap<GameAction, ControllerButton>,
    /// Touch gesture bindings
    pub touch_bindings: HashMap<GameAction, TouchGesture>,
    /// Mouse sensitivity (0.1 - 3.0)
    pub mouse_sensitivity: f32,
    /// Invert mouse Y axis
    pub invert_mouse_y: bool,
    /// Controller enabled
    pub controller_enabled: bool,
    /// Controller vibration enabled
    pub controller_vibration: bool,
    /// Controller vibration intensity (0.0 - 1.0)
    pub vibration_intensity: f32,
    /// Controller deadzone (0.0 - 0.5)
    pub controller_deadzone: f32,
    /// Touch controls enabled (for mobile)
    pub touch_enabled: bool,
    /// Touch control opacity (0.0 - 1.0)
    pub touch_opacity: f32,
    /// Touch button size
    pub touch_button_size: TouchButtonSize,
    /// Show virtual joystick
    pub virtual_joystick: bool,
    /// Double-click speed in milliseconds
    pub double_click_time_ms: u32,
    /// Hold-to-repeat delay in milliseconds
    pub key_repeat_delay_ms: u32,
    /// Hold-to-repeat interval in milliseconds
    pub key_repeat_interval_ms: u32,
    /// Click-to-move enabled
    pub click_to_move: bool,
    /// Edge scrolling enabled
    pub edge_scrolling: bool,
    /// Edge scroll speed
    pub edge_scroll_speed: f32,
}

/// Touch button size preset
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum TouchButtonSize {
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
}

impl TouchButtonSize {
    pub fn all() -> &'static [TouchButtonSize] {
        &[Self::Small, Self::Medium, Self::Large, Self::ExtraLarge]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Small => "Small",
            Self::Medium => "Medium",
            Self::Large => "Large",
            Self::ExtraLarge => "Extra Large",
        }
    }

    pub fn pixels(&self) -> u32 {
        match self {
            Self::Small => 48,
            Self::Medium => 64,
            Self::Large => 80,
            Self::ExtraLarge => 96,
        }
    }
}

impl Default for ControlSettings {
    fn default() -> Self {
        let mut keybindings = HashMap::new();

        // Movement
        keybindings.insert(GameAction::MoveUp, vec![KeyBinding::new("W"), KeyBinding::new("Up"), KeyBinding::new("Numpad8")]);
        keybindings.insert(GameAction::MoveDown, vec![KeyBinding::new("S"), KeyBinding::new("Down"), KeyBinding::new("Numpad2")]);
        keybindings.insert(GameAction::MoveLeft, vec![KeyBinding::new("A"), KeyBinding::new("Left"), KeyBinding::new("Numpad4")]);
        keybindings.insert(GameAction::MoveRight, vec![KeyBinding::new("D"), KeyBinding::new("Right"), KeyBinding::new("Numpad6")]);
        keybindings.insert(GameAction::MoveUpLeft, vec![KeyBinding::new("Q"), KeyBinding::new("Numpad7")]);
        keybindings.insert(GameAction::MoveUpRight, vec![KeyBinding::new("E"), KeyBinding::new("Numpad9")]);
        keybindings.insert(GameAction::MoveDownLeft, vec![KeyBinding::new("Z"), KeyBinding::new("Numpad1")]);
        keybindings.insert(GameAction::MoveDownRight, vec![KeyBinding::new("C"), KeyBinding::new("Numpad3")]);
        keybindings.insert(GameAction::Wait, vec![KeyBinding::new("Space"), KeyBinding::new("Numpad5")]);

        // Combat
        keybindings.insert(GameAction::Attack, vec![KeyBinding::new("F")]);
        keybindings.insert(GameAction::UseSkill1, vec![KeyBinding::new("1")]);
        keybindings.insert(GameAction::UseSkill2, vec![KeyBinding::new("2")]);
        keybindings.insert(GameAction::UseSkill3, vec![KeyBinding::new("3")]);
        keybindings.insert(GameAction::UseSkill4, vec![KeyBinding::new("4")]);
        keybindings.insert(GameAction::UseSkill5, vec![KeyBinding::new("5")]);
        keybindings.insert(GameAction::UseSkill6, vec![KeyBinding::new("6")]);
        keybindings.insert(GameAction::UseSkill7, vec![KeyBinding::new("7")]);
        keybindings.insert(GameAction::UseSkill8, vec![KeyBinding::new("8")]);
        keybindings.insert(GameAction::UseSkill9, vec![KeyBinding::new("9")]);
        keybindings.insert(GameAction::UseSkill10, vec![KeyBinding::new("0")]);
        keybindings.insert(GameAction::CycleTargetNext, vec![KeyBinding::new("Tab")]);
        keybindings.insert(GameAction::CycleTargetPrev, vec![KeyBinding::new("Tab").with_shift()]);
        keybindings.insert(GameAction::ConfirmTarget, vec![KeyBinding::new("Enter")]);
        keybindings.insert(GameAction::CancelTarget, vec![KeyBinding::new("Escape")]);

        // Items
        keybindings.insert(GameAction::UseQuickItem1, vec![KeyBinding::new("F1")]);
        keybindings.insert(GameAction::UseQuickItem2, vec![KeyBinding::new("F2")]);
        keybindings.insert(GameAction::UseQuickItem3, vec![KeyBinding::new("F3")]);
        keybindings.insert(GameAction::UseQuickItem4, vec![KeyBinding::new("F4")]);
        keybindings.insert(GameAction::UseQuickItem5, vec![KeyBinding::new("F5")]);
        keybindings.insert(GameAction::UseQuickItem6, vec![KeyBinding::new("F6")]);
        keybindings.insert(GameAction::PickupItem, vec![KeyBinding::new("G")]);
        keybindings.insert(GameAction::DropItem, vec![KeyBinding::new("G").with_shift()]);
        keybindings.insert(GameAction::UseItem, vec![KeyBinding::new("U")]);

        // Menus
        keybindings.insert(GameAction::OpenInventory, vec![KeyBinding::new("I")]);
        keybindings.insert(GameAction::OpenCharacter, vec![KeyBinding::new("C").with_shift()]);
        keybindings.insert(GameAction::OpenSkills, vec![KeyBinding::new("K")]);
        keybindings.insert(GameAction::OpenMap, vec![KeyBinding::new("M")]);
        keybindings.insert(GameAction::OpenQuests, vec![KeyBinding::new("J")]);
        keybindings.insert(GameAction::OpenSettings, vec![KeyBinding::new("Escape")]);
        keybindings.insert(GameAction::OpenHelp, vec![KeyBinding::new("H"), KeyBinding::new("F1").with_shift()]);
        keybindings.insert(GameAction::OpenChat, vec![KeyBinding::new("Enter").with_shift()]);

        // System
        keybindings.insert(GameAction::Pause, vec![KeyBinding::new("P")]);
        keybindings.insert(GameAction::QuickSave, vec![KeyBinding::new("F5").with_ctrl()]);
        keybindings.insert(GameAction::QuickLoad, vec![KeyBinding::new("F9").with_ctrl()]);
        keybindings.insert(GameAction::Screenshot, vec![KeyBinding::new("F12")]);
        keybindings.insert(GameAction::ToggleFullscreen, vec![KeyBinding::new("Enter").with_alt()]);

        // Interaction
        keybindings.insert(GameAction::Interact, vec![KeyBinding::new("E")]);
        keybindings.insert(GameAction::Talk, vec![KeyBinding::new("T")]);
        keybindings.insert(GameAction::Rest, vec![KeyBinding::new("R")]);
        keybindings.insert(GameAction::AutoExplore, vec![KeyBinding::new("X")]);
        keybindings.insert(GameAction::Search, vec![KeyBinding::new("S").with_shift()]);

        // Camera
        keybindings.insert(GameAction::ZoomIn, vec![KeyBinding::new("+"), KeyBinding::new("=")]);
        keybindings.insert(GameAction::ZoomOut, vec![KeyBinding::new("-")]);
        keybindings.insert(GameAction::CenterCamera, vec![KeyBinding::new("Home")]);
        keybindings.insert(GameAction::RotateCameraLeft, vec![KeyBinding::new("[")]);
        keybindings.insert(GameAction::RotateCameraRight, vec![KeyBinding::new("]")]);

        // Controller bindings
        let mut controller_bindings = HashMap::new();
        controller_bindings.insert(GameAction::Attack, ControllerButton::A);
        controller_bindings.insert(GameAction::Interact, ControllerButton::B);
        controller_bindings.insert(GameAction::OpenInventory, ControllerButton::Y);
        controller_bindings.insert(GameAction::OpenMap, ControllerButton::X);
        controller_bindings.insert(GameAction::CycleTargetNext, ControllerButton::RightBumper);
        controller_bindings.insert(GameAction::CycleTargetPrev, ControllerButton::LeftBumper);
        controller_bindings.insert(GameAction::Pause, ControllerButton::Start);
        controller_bindings.insert(GameAction::OpenSettings, ControllerButton::Select);

        // Touch bindings
        let mut touch_bindings = HashMap::new();
        touch_bindings.insert(GameAction::Attack, TouchGesture::Tap);
        touch_bindings.insert(GameAction::Interact, TouchGesture::DoubleTap);
        touch_bindings.insert(GameAction::OpenInventory, TouchGesture::TwoFingerTap);
        touch_bindings.insert(GameAction::Rest, TouchGesture::LongPress);
        touch_bindings.insert(GameAction::ZoomIn, TouchGesture::PinchOut);
        touch_bindings.insert(GameAction::ZoomOut, TouchGesture::PinchIn);

        Self {
            keybindings,
            mouse_bindings: HashMap::new(),
            controller_bindings,
            touch_bindings,
            mouse_sensitivity: DEFAULT_MOUSE_SENSITIVITY,
            invert_mouse_y: false,
            controller_enabled: true,
            controller_vibration: true,
            vibration_intensity: 0.7,
            controller_deadzone: 0.15,
            touch_enabled: false,
            touch_opacity: 0.7,
            touch_button_size: TouchButtonSize::default(),
            virtual_joystick: true,
            double_click_time_ms: 400,
            key_repeat_delay_ms: 500,
            key_repeat_interval_ms: 50,
            click_to_move: true,
            edge_scrolling: false,
            edge_scroll_speed: 1.0,
        }
    }
}

impl ControlSettings {
    /// Get keybindings for an action
    pub fn get_keybindings(&self, action: GameAction) -> Option<&Vec<KeyBinding>> {
        self.keybindings.get(&action)
    }

    /// Set keybindings for an action
    pub fn set_keybindings(&mut self, action: GameAction, bindings: Vec<KeyBinding>) {
        self.keybindings.insert(action, bindings);
    }

    /// Add a keybinding for an action
    pub fn add_keybinding(&mut self, action: GameAction, binding: KeyBinding) {
        self.keybindings.entry(action).or_default().push(binding);
    }

    /// Remove a keybinding from an action
    pub fn remove_keybinding(&mut self, action: GameAction, binding: &KeyBinding) {
        if let Some(bindings) = self.keybindings.get_mut(&action) {
            bindings.retain(|b| b != binding);
        }
    }

    /// Clear all keybindings for an action
    pub fn clear_keybindings(&mut self, action: GameAction) {
        self.keybindings.remove(&action);
    }

    /// Find action by key binding
    pub fn find_action_by_key(&self, key: &str, modifiers: &[KeyModifier]) -> Option<GameAction> {
        for (action, bindings) in &self.keybindings {
            for binding in bindings {
                if binding.key == key && binding.modifiers == modifiers {
                    return Some(*action);
                }
            }
        }
        None
    }

    /// Check for keybinding conflicts
    pub fn find_conflicts(&self) -> Vec<(GameAction, GameAction, KeyBinding)> {
        let mut conflicts = Vec::new();
        let actions: Vec<_> = self.keybindings.iter().collect();

        for i in 0..actions.len() {
            for j in (i + 1)..actions.len() {
                let (action1, bindings1) = actions[i];
                let (action2, bindings2) = actions[j];

                for b1 in bindings1 {
                    for b2 in bindings2 {
                        if b1 == b2 {
                            conflicts.push((*action1, *action2, b1.clone()));
                        }
                    }
                }
            }
        }

        conflicts
    }
}

// ============================================================================
// Audio Settings (Stubs for future)
// ============================================================================

/// Audio settings
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioSettings {
    /// Master volume (0.0 - 1.0)
    pub master_volume: f32,
    /// Music volume (0.0 - 1.0)
    pub music_volume: f32,
    /// Sound effects volume (0.0 - 1.0)
    pub sfx_volume: f32,
    /// Voice/dialogue volume (0.0 - 1.0)
    pub voice_volume: f32,
    /// Ambient sounds volume (0.0 - 1.0)
    pub ambient_volume: f32,
    /// UI sounds volume (0.0 - 1.0)
    pub ui_volume: f32,
    /// Master mute
    pub mute_all: bool,
    /// Mute music
    pub mute_music: bool,
    /// Mute sound effects
    pub mute_sfx: bool,
    /// Mute voice
    pub mute_voice: bool,
    /// Mute ambient
    pub mute_ambient: bool,
    /// Mute UI sounds
    pub mute_ui: bool,
    /// Mute when game is in background
    pub mute_on_focus_loss: bool,
    /// Enable positional/3D audio
    pub positional_audio: bool,
    /// Audio output device (empty = default)
    pub output_device: String,
    /// Enable dynamic music (changes based on combat)
    pub dynamic_music: bool,
    /// Music crossfade duration in milliseconds
    pub crossfade_duration_ms: u32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: DEFAULT_MASTER_VOLUME,
            music_volume: 0.7,
            sfx_volume: 0.8,
            voice_volume: 0.9,
            ambient_volume: 0.5,
            ui_volume: 0.6,
            mute_all: false,
            mute_music: false,
            mute_sfx: false,
            mute_voice: false,
            mute_ambient: false,
            mute_ui: false,
            mute_on_focus_loss: true,
            positional_audio: true,
            output_device: String::new(),
            dynamic_music: true,
            crossfade_duration_ms: 2000,
        }
    }
}

impl AudioSettings {
    /// Get effective volume for a channel (considering master and mutes)
    pub fn effective_master_volume(&self) -> f32 {
        if self.mute_all { 0.0 } else { self.master_volume }
    }

    pub fn effective_music_volume(&self) -> f32 {
        if self.mute_all || self.mute_music { 0.0 } else { self.master_volume * self.music_volume }
    }

    pub fn effective_sfx_volume(&self) -> f32 {
        if self.mute_all || self.mute_sfx { 0.0 } else { self.master_volume * self.sfx_volume }
    }

    pub fn effective_voice_volume(&self) -> f32 {
        if self.mute_all || self.mute_voice { 0.0 } else { self.master_volume * self.voice_volume }
    }

    pub fn effective_ambient_volume(&self) -> f32 {
        if self.mute_all || self.mute_ambient { 0.0 } else { self.master_volume * self.ambient_volume }
    }

    pub fn effective_ui_volume(&self) -> f32 {
        if self.mute_all || self.mute_ui { 0.0 } else { self.master_volume * self.ui_volume }
    }

    /// Toggle master mute
    pub fn toggle_mute(&mut self) {
        self.mute_all = !self.mute_all;
    }
}

// ============================================================================
// Accessibility Settings
// ============================================================================

/// Accessibility settings
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessibilitySettings {
    /// Enable screen reader support
    pub screen_reader_enabled: bool,
    /// Screen reader verbosity
    pub screen_reader_verbosity: ScreenReaderVerbosity,
    /// High contrast mode
    pub high_contrast: bool,
    /// Large text mode
    pub large_text: bool,
    /// Large text scale multiplier
    pub text_scale: f32,
    /// Reduced motion (minimize animations)
    pub reduced_motion: bool,
    /// Disable screen shake
    pub disable_screen_shake: bool,
    /// Disable flashing effects
    pub disable_flashing: bool,
    /// One-handed mode
    pub one_handed_mode: bool,
    /// One-handed mode hand preference
    pub preferred_hand: HandPreference,
    /// Auto-aim assistance
    pub auto_aim: bool,
    /// Auto-aim strength (0.0 - 1.0)
    pub auto_aim_strength: f32,
    /// Extended timers for timed events
    pub extended_timers: bool,
    /// Timer extension multiplier
    pub timer_multiplier: f32,
    /// Subtitle size
    pub subtitle_size: SubtitleSize,
    /// Show speaker names in subtitles
    pub subtitle_speaker_names: bool,
    /// Subtitle background opacity
    pub subtitle_background_opacity: f32,
    /// Enable text-to-speech for game messages
    pub text_to_speech: bool,
    /// Text-to-speech speed
    pub tts_speed: f32,
    /// Dyslexia-friendly font
    pub dyslexia_font: bool,
    /// Button hold time for inputs (ms)
    pub button_hold_time_ms: u32,
    /// Sticky keys enabled
    pub sticky_keys: bool,
    /// Toggle mode for run/crouch instead of hold
    pub toggle_run: bool,
}

/// Screen reader verbosity level
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ScreenReaderVerbosity {
    /// Minimal announcements
    Low,
    /// Standard announcements
    #[default]
    Medium,
    /// Detailed announcements
    High,
    /// Maximum detail
    Full,
}

impl ScreenReaderVerbosity {
    pub fn all() -> &'static [ScreenReaderVerbosity] {
        &[Self::Low, Self::Medium, Self::High, Self::Full]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::Full => "Full",
        }
    }
}

/// Hand preference for one-handed mode
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum HandPreference {
    #[default]
    Left,
    Right,
}

impl HandPreference {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Left => "Left Hand",
            Self::Right => "Right Hand",
        }
    }
}

/// Subtitle size
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SubtitleSize {
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
}

impl SubtitleSize {
    pub fn all() -> &'static [SubtitleSize] {
        &[Self::Small, Self::Medium, Self::Large, Self::ExtraLarge]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Small => "Small",
            Self::Medium => "Medium",
            Self::Large => "Large",
            Self::ExtraLarge => "Extra Large",
        }
    }

    pub fn font_size(&self) -> u32 {
        match self {
            Self::Small => 12,
            Self::Medium => 16,
            Self::Large => 20,
            Self::ExtraLarge => 28,
        }
    }
}

impl Default for AccessibilitySettings {
    fn default() -> Self {
        Self {
            screen_reader_enabled: false,
            screen_reader_verbosity: ScreenReaderVerbosity::default(),
            high_contrast: false,
            large_text: false,
            text_scale: 1.0,
            reduced_motion: false,
            disable_screen_shake: false,
            disable_flashing: false,
            one_handed_mode: false,
            preferred_hand: HandPreference::default(),
            auto_aim: false,
            auto_aim_strength: 0.5,
            extended_timers: false,
            timer_multiplier: 1.5,
            subtitle_size: SubtitleSize::default(),
            subtitle_speaker_names: true,
            subtitle_background_opacity: 0.7,
            text_to_speech: false,
            tts_speed: 1.0,
            dyslexia_font: false,
            button_hold_time_ms: 0,
            sticky_keys: false,
            toggle_run: false,
        }
    }
}

// ============================================================================
// Profile Settings
// ============================================================================

/// Preferred playstyle
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Playstyle {
    /// Balanced approach
    #[default]
    Balanced,
    /// Aggressive combat focus
    Aggressive,
    /// Defensive/cautious
    Defensive,
    /// Stealth and avoidance
    Stealthy,
    /// Magic and spellcasting
    Magical,
    /// Exploration focus
    Explorer,
    /// Completionist - do everything
    Completionist,
    /// Speedrunner - go fast
    Speedrunner,
}

impl Playstyle {
    pub fn all() -> &'static [Playstyle] {
        &[
            Self::Balanced,
            Self::Aggressive,
            Self::Defensive,
            Self::Stealthy,
            Self::Magical,
            Self::Explorer,
            Self::Completionist,
            Self::Speedrunner,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Balanced => "Balanced",
            Self::Aggressive => "Aggressive",
            Self::Defensive => "Defensive",
            Self::Stealthy => "Stealthy",
            Self::Magical => "Magical",
            Self::Explorer => "Explorer",
            Self::Completionist => "Completionist",
            Self::Speedrunner => "Speedrunner",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Balanced => "A well-rounded approach to dungeon delving.",
            Self::Aggressive => "Rush in and overwhelm enemies with force.",
            Self::Defensive => "Careful, methodical progression with safety first.",
            Self::Stealthy => "Avoid confrontation when possible, strike from shadows.",
            Self::Magical => "Rely on spells and magical abilities.",
            Self::Explorer => "Explore every corner, find every secret.",
            Self::Completionist => "Complete all quests, collect all items.",
            Self::Speedrunner => "Finish as fast as possible.",
        }
    }
}

/// Profile settings
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileSettings {
    /// Player display name
    pub player_name: String,
    /// Default character class for new games
    pub default_class: CharacterClass,
    /// Preferred playstyle
    pub playstyle: Playstyle,
    /// Preferred starting items (hints to item generation)
    pub preferred_starting_items: Vec<String>,
    /// Profile icon/avatar ID
    pub avatar_id: u32,
    /// Profile banner ID
    pub banner_id: u32,
    /// Profile title (earned through achievements)
    pub title: Option<String>,
    /// Show online status
    pub show_online_status: bool,
    /// Allow leaderboard submissions
    pub submit_to_leaderboard: bool,
    /// Enable cloud saves
    pub cloud_save_enabled: bool,
    /// Statistics tracking enabled
    pub track_statistics: bool,
    /// Achievement popup enabled
    pub achievement_popups: bool,
}

impl Default for ProfileSettings {
    fn default() -> Self {
        Self {
            player_name: "Adventurer".to_string(),
            default_class: CharacterClass::Warrior,
            playstyle: Playstyle::default(),
            preferred_starting_items: vec![],
            avatar_id: 0,
            banner_id: 0,
            title: None,
            show_online_status: true,
            submit_to_leaderboard: true,
            cloud_save_enabled: true,
            track_statistics: true,
            achievement_popups: true,
        }
    }
}

// ============================================================================
// Main Settings Structure
// ============================================================================

/// Complete game settings
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GameSettings {
    /// Settings version for migration
    pub version: u32,
    /// Gameplay settings
    pub gameplay: GameplaySettings,
    /// Display settings
    pub display: DisplaySettings,
    /// Control settings
    pub controls: ControlSettings,
    /// Audio settings
    pub audio: AudioSettings,
    /// Accessibility settings
    pub accessibility: AccessibilitySettings,
    /// Profile settings
    pub profile: ProfileSettings,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            version: SETTINGS_VERSION,
            gameplay: GameplaySettings::default(),
            display: DisplaySettings::default(),
            controls: ControlSettings::default(),
            audio: AudioSettings::default(),
            accessibility: AccessibilitySettings::default(),
            profile: ProfileSettings::default(),
        }
    }
}

// ============================================================================
// Settings Persistence
// ============================================================================

/// Settings error type
#[derive(Debug)]
pub enum SettingsError {
    IoError(std::io::Error),
    SerializeError(String),
    DeserializeError(String),
    ValidationError(String),
    MigrationError(String),
}

impl std::fmt::Display for SettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::SerializeError(e) => write!(f, "Serialization error: {}", e),
            Self::DeserializeError(e) => write!(f, "Deserialization error: {}", e),
            Self::ValidationError(e) => write!(f, "Validation error: {}", e),
            Self::MigrationError(e) => write!(f, "Migration error: {}", e),
        }
    }
}

impl std::error::Error for SettingsError {}

impl From<std::io::Error> for SettingsError {
    fn from(err: std::io::Error) -> Self {
        SettingsError::IoError(err)
    }
}

/// Settings system for managing game settings with persistence
#[derive(Clone, Debug)]
pub struct SettingsSystem {
    /// Current settings
    pub settings: GameSettings,
    /// Path to settings file
    settings_path: PathBuf,
    /// Whether settings have been modified since last save
    dirty: bool,
    /// Backup of settings before changes (for revert)
    backup: Option<GameSettings>,
}

impl Default for SettingsSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsSystem {
    /// Create a new settings system with default settings
    pub fn new() -> Self {
        Self {
            settings: GameSettings::default(),
            settings_path: Self::default_settings_path(),
            dirty: false,
            backup: None,
        }
    }

    /// Create a settings system with a custom path
    pub fn with_path(path: PathBuf) -> Self {
        Self {
            settings: GameSettings::default(),
            settings_path: path,
            dirty: false,
            backup: None,
        }
    }

    /// Get the default settings file path
    pub fn default_settings_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(SETTINGS_FILE_NAME)
    }

    /// Check if settings file exists
    pub fn settings_exist(&self) -> bool {
        self.settings_path.exists()
    }

    /// Save settings to file
    pub fn save(&mut self) -> Result<(), SettingsError> {
        self.validate()?;

        let json = serde_json::to_string_pretty(&self.settings)
            .map_err(|e| SettingsError::SerializeError(e.to_string()))?;

        // Ensure parent directory exists
        if let Some(parent) = self.settings_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&self.settings_path, json)?;
        self.dirty = false;

        Ok(())
    }

    /// Load settings from file
    pub fn load(&mut self) -> Result<(), SettingsError> {
        if !self.settings_exist() {
            return Ok(());
        }

        let json = fs::read_to_string(&self.settings_path)?;
        let loaded: GameSettings = serde_json::from_str(&json)
            .map_err(|e| SettingsError::DeserializeError(e.to_string()))?;

        // Migrate if necessary
        let migrated = self.migrate(loaded)?;
        self.settings = migrated;
        self.dirty = false;

        Ok(())
    }

    /// Load settings or create defaults
    pub fn load_or_create(&mut self) -> Result<(), SettingsError> {
        if self.settings_exist() {
            self.load()
        } else {
            self.settings = GameSettings::default();
            self.save()
        }
    }

    /// Reset to default settings
    pub fn reset_to_defaults(&mut self) {
        self.settings = GameSettings::default();
        self.dirty = true;
    }

    /// Reset a specific category to defaults
    pub fn reset_category(&mut self, category: SettingsCategory) {
        match category {
            SettingsCategory::Gameplay => self.settings.gameplay = GameplaySettings::default(),
            SettingsCategory::Display => self.settings.display = DisplaySettings::default(),
            SettingsCategory::Controls => self.settings.controls = ControlSettings::default(),
            SettingsCategory::Audio => self.settings.audio = AudioSettings::default(),
            SettingsCategory::Accessibility => self.settings.accessibility = AccessibilitySettings::default(),
            SettingsCategory::Profile => self.settings.profile = ProfileSettings::default(),
        }
        self.dirty = true;
    }

    /// Create a backup of current settings
    pub fn create_backup(&mut self) {
        self.backup = Some(self.settings.clone());
    }

    /// Revert to backup
    pub fn revert_to_backup(&mut self) -> bool {
        if let Some(backup) = self.backup.take() {
            self.settings = backup;
            self.dirty = true;
            true
        } else {
            false
        }
    }

    /// Check if settings have been modified
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Mark settings as modified
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Validate settings
    pub fn validate(&self) -> Result<(), SettingsError> {
        // Validate gameplay settings
        if self.settings.gameplay.camera_shake < 0.0 || self.settings.gameplay.camera_shake > 1.0 {
            return Err(SettingsError::ValidationError(
                "Camera shake must be between 0.0 and 1.0".to_string()
            ));
        }

        // Validate display settings
        if self.settings.display.ui_scale < 0.5 || self.settings.display.ui_scale > 2.0 {
            return Err(SettingsError::ValidationError(
                "UI scale must be between 0.5 and 2.0".to_string()
            ));
        }

        if self.settings.display.brightness < 0.5 || self.settings.display.brightness > 1.5 {
            return Err(SettingsError::ValidationError(
                "Brightness must be between 0.5 and 1.5".to_string()
            ));
        }

        // Validate control settings
        if self.settings.controls.mouse_sensitivity < 0.1 || self.settings.controls.mouse_sensitivity > 3.0 {
            return Err(SettingsError::ValidationError(
                "Mouse sensitivity must be between 0.1 and 3.0".to_string()
            ));
        }

        if self.settings.controls.controller_deadzone < 0.0 || self.settings.controls.controller_deadzone > 0.5 {
            return Err(SettingsError::ValidationError(
                "Controller deadzone must be between 0.0 and 0.5".to_string()
            ));
        }

        // Validate audio settings
        if self.settings.audio.master_volume < 0.0 || self.settings.audio.master_volume > 1.0 {
            return Err(SettingsError::ValidationError(
                "Master volume must be between 0.0 and 1.0".to_string()
            ));
        }

        // Validate accessibility settings
        if self.settings.accessibility.text_scale < 0.5 || self.settings.accessibility.text_scale > 3.0 {
            return Err(SettingsError::ValidationError(
                "Text scale must be between 0.5 and 3.0".to_string()
            ));
        }

        Ok(())
    }

    /// Migrate settings from older versions
    fn migrate(&self, mut settings: GameSettings) -> Result<GameSettings, SettingsError> {
        if settings.version == SETTINGS_VERSION {
            return Ok(settings);
        }

        // Future migrations would go here
        // For now, just update the version
        settings.version = SETTINGS_VERSION;

        Ok(settings)
    }

    /// Export settings to a portable format
    pub fn export(&self) -> Result<String, SettingsError> {
        serde_json::to_string_pretty(&self.settings)
            .map_err(|e| SettingsError::SerializeError(e.to_string()))
    }

    /// Export settings to a file
    pub fn export_to_file(&self, path: &PathBuf) -> Result<(), SettingsError> {
        let json = self.export()?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Import settings from a string
    pub fn import(&mut self, data: &str) -> Result<(), SettingsError> {
        let imported: GameSettings = serde_json::from_str(data)
            .map_err(|e| SettingsError::DeserializeError(e.to_string()))?;

        self.settings = self.migrate(imported)?;
        self.dirty = true;

        Ok(())
    }

    /// Import settings from a file
    pub fn import_from_file(&mut self, path: &PathBuf) -> Result<(), SettingsError> {
        let json = fs::read_to_string(path)?;
        self.import(&json)
    }

    /// Get a summary of current settings
    pub fn summary(&self) -> SettingsSummary {
        SettingsSummary {
            difficulty: self.settings.gameplay.difficulty,
            permadeath: self.settings.gameplay.permadeath,
            ironman: self.settings.gameplay.ironman,
            resolution: self.settings.display.resolution,
            screen_mode: self.settings.display.screen_mode,
            color_scheme: self.settings.display.color_scheme,
            master_volume: self.settings.audio.master_volume,
            accessibility_features: self.count_accessibility_features(),
        }
    }

    fn count_accessibility_features(&self) -> u32 {
        let a = &self.settings.accessibility;
        let mut count = 0;
        if a.screen_reader_enabled { count += 1; }
        if a.high_contrast { count += 1; }
        if a.large_text { count += 1; }
        if a.reduced_motion { count += 1; }
        if a.one_handed_mode { count += 1; }
        if a.auto_aim { count += 1; }
        if a.extended_timers { count += 1; }
        if a.text_to_speech { count += 1; }
        if a.dyslexia_font { count += 1; }
        count
    }

    // Convenience accessors

    /// Get gameplay settings
    pub fn gameplay(&self) -> &GameplaySettings {
        &self.settings.gameplay
    }

    /// Get mutable gameplay settings
    pub fn gameplay_mut(&mut self) -> &mut GameplaySettings {
        self.dirty = true;
        &mut self.settings.gameplay
    }

    /// Get display settings
    pub fn display(&self) -> &DisplaySettings {
        &self.settings.display
    }

    /// Get mutable display settings
    pub fn display_mut(&mut self) -> &mut DisplaySettings {
        self.dirty = true;
        &mut self.settings.display
    }

    /// Get control settings
    pub fn controls(&self) -> &ControlSettings {
        &self.settings.controls
    }

    /// Get mutable control settings
    pub fn controls_mut(&mut self) -> &mut ControlSettings {
        self.dirty = true;
        &mut self.settings.controls
    }

    /// Get audio settings
    pub fn audio(&self) -> &AudioSettings {
        &self.settings.audio
    }

    /// Get mutable audio settings
    pub fn audio_mut(&mut self) -> &mut AudioSettings {
        self.dirty = true;
        &mut self.settings.audio
    }

    /// Get accessibility settings
    pub fn accessibility(&self) -> &AccessibilitySettings {
        &self.settings.accessibility
    }

    /// Get mutable accessibility settings
    pub fn accessibility_mut(&mut self) -> &mut AccessibilitySettings {
        self.dirty = true;
        &mut self.settings.accessibility
    }

    /// Get profile settings
    pub fn profile(&self) -> &ProfileSettings {
        &self.settings.profile
    }

    /// Get mutable profile settings
    pub fn profile_mut(&mut self) -> &mut ProfileSettings {
        self.dirty = true;
        &mut self.settings.profile
    }

    /// Apply accessibility overrides to display settings
    pub fn apply_accessibility_overrides(&mut self) {
        let a = &self.settings.accessibility;

        if a.high_contrast {
            self.settings.display.color_scheme = ColorScheme::HighContrast;
        }

        if a.large_text {
            self.settings.display.font_size = (DEFAULT_FONT_SIZE as f32 * a.text_scale) as u32;
            self.settings.display.message_font_size = (12.0 * a.text_scale) as u32;
        }

        if a.reduced_motion {
            self.settings.display.animation_speed = AnimationSpeed::None;
            self.settings.display.particles_enabled = false;
        }

        if a.disable_screen_shake {
            self.settings.gameplay.camera_shake = 0.0;
        }
    }
}

/// Settings categories for selective reset
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SettingsCategory {
    Gameplay,
    Display,
    Controls,
    Audio,
    Accessibility,
    Profile,
}

impl SettingsCategory {
    pub fn all() -> &'static [SettingsCategory] {
        &[
            Self::Gameplay,
            Self::Display,
            Self::Controls,
            Self::Audio,
            Self::Accessibility,
            Self::Profile,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Gameplay => "Gameplay",
            Self::Display => "Display",
            Self::Controls => "Controls",
            Self::Audio => "Audio",
            Self::Accessibility => "Accessibility",
            Self::Profile => "Profile",
        }
    }
}

/// Summary of current settings
#[derive(Clone, Debug)]
pub struct SettingsSummary {
    pub difficulty: Difficulty,
    pub permadeath: bool,
    pub ironman: bool,
    pub resolution: Resolution,
    pub screen_mode: ScreenMode,
    pub color_scheme: ColorScheme,
    pub master_volume: f32,
    pub accessibility_features: u32,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;

    #[test]
    fn test_default_settings() {
        let settings = GameSettings::default();
        assert_eq!(settings.version, SETTINGS_VERSION);
        assert_eq!(settings.gameplay.difficulty, Difficulty::Normal);
        assert!(!settings.gameplay.permadeath);
    }

    #[test]
    fn test_difficulty_multipliers() {
        assert!(Difficulty::Easy.enemy_damage_multiplier() < 1.0);
        assert_eq!(Difficulty::Normal.enemy_damage_multiplier(), 1.0);
        assert!(Difficulty::Hell.enemy_damage_multiplier() > 1.0);
    }

    #[test]
    fn test_keybinding_display() {
        let binding = KeyBinding::new("S").with_ctrl().with_shift();
        assert!(binding.display().contains("Ctrl"));
        assert!(binding.display().contains("Shift"));
        assert!(binding.display().contains("S"));
    }

    #[test]
    fn test_audio_effective_volume() {
        let mut audio = AudioSettings::default();
        assert!(audio.effective_music_volume() > 0.0);

        audio.mute_all = true;
        assert_eq!(audio.effective_music_volume(), 0.0);

        audio.mute_all = false;
        audio.mute_music = true;
        assert_eq!(audio.effective_music_volume(), 0.0);
    }

    #[test]
    fn test_settings_save_load() {
        let path = temp_dir().join("test_shadowcrypt_settings.json");

        let mut system = SettingsSystem::with_path(path.clone());
        system.settings.gameplay.difficulty = Difficulty::Hard;
        system.settings.profile.player_name = "TestPlayer".to_string();
        system.save().expect("Failed to save");

        let mut loaded = SettingsSystem::with_path(path.clone());
        loaded.load().expect("Failed to load");

        assert_eq!(loaded.settings.gameplay.difficulty, Difficulty::Hard);
        assert_eq!(loaded.settings.profile.player_name, "TestPlayer");

        // Cleanup
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_settings_export_import() {
        let mut system = SettingsSystem::new();
        system.settings.gameplay.difficulty = Difficulty::Nightmare;

        let exported = system.export().expect("Failed to export");

        let mut new_system = SettingsSystem::new();
        new_system.import(&exported).expect("Failed to import");

        assert_eq!(new_system.settings.gameplay.difficulty, Difficulty::Nightmare);
    }

    #[test]
    fn test_settings_validation() {
        let mut system = SettingsSystem::new();
        assert!(system.validate().is_ok());

        system.settings.display.ui_scale = 5.0; // Invalid
        assert!(system.validate().is_err());
    }

    #[test]
    fn test_backup_revert() {
        let mut system = SettingsSystem::new();
        system.settings.gameplay.difficulty = Difficulty::Easy;

        system.create_backup();
        system.settings.gameplay.difficulty = Difficulty::Hell;

        assert_eq!(system.settings.gameplay.difficulty, Difficulty::Hell);

        system.revert_to_backup();
        assert_eq!(system.settings.gameplay.difficulty, Difficulty::Easy);
    }

    #[test]
    fn test_reset_category() {
        let mut system = SettingsSystem::new();
        system.settings.audio.master_volume = 0.1;

        system.reset_category(SettingsCategory::Audio);

        assert_eq!(system.settings.audio.master_volume, DEFAULT_MASTER_VOLUME);
    }

    #[test]
    fn test_control_conflicts() {
        let mut controls = ControlSettings::default();

        // Add a conflicting binding
        controls.keybindings.insert(
            GameAction::Attack,
            vec![KeyBinding::new("I")]
        );

        let conflicts = controls.find_conflicts();
        // Should find conflict with OpenInventory which uses "I"
        assert!(!conflicts.is_empty());
    }

    #[test]
    fn test_tutorial_settings() {
        let mut tutorials = TutorialSettings::default();
        assert!(tutorials.enabled);

        tutorials.disable_all();
        assert!(!tutorials.enabled);
        assert!(!tutorials.combat);

        tutorials.enable_all();
        assert!(tutorials.enabled);
        assert!(tutorials.combat);
    }

    #[test]
    fn test_auto_loot_options() {
        let auto_loot = AutoLootOptions::default();
        assert!(auto_loot.enabled);
        assert!(auto_loot.pickup_gold);
        assert_eq!(auto_loot.min_equipment_rarity, ItemRarity::Common);
    }

    #[test]
    fn test_color_scheme() {
        for scheme in ColorScheme::all() {
            assert!(!scheme.name().is_empty());
            // Verify colors are valid (non-zero with alpha)
            assert!(scheme.background_color() > 0);
            assert!(scheme.text_color() > 0);
        }
    }

    #[test]
    fn test_resolution_presets() {
        let presets = Resolution::presets();
        assert!(!presets.is_empty());

        for res in presets {
            assert!(res.width > 0);
            assert!(res.height > 0);
            assert!(res.aspect_ratio() > 0.0);
        }
    }

    #[test]
    fn test_game_action_categories() {
        for action in GameAction::all() {
            assert!(!action.name().is_empty());
            assert!(!action.category().is_empty());
        }
    }
}
