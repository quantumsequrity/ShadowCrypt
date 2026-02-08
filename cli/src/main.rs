//! ShadowCrypt CLI - Terminal frontend for the roguelike game
//!
//! This CLI uses crossterm for terminal rendering and input handling,
//! importing all game logic from shadowcrypt-core.

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor, Attribute, SetAttribute},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::prelude::*;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};
use std::collections::VecDeque;

use shadowcrypt_core::prelude::*;

// ============================================================================
// BOX DRAWING CHARACTERS FOR UI
// ============================================================================

mod box_chars {
    pub const TOP_LEFT: char = '╔';
    pub const TOP_RIGHT: char = '╗';
    pub const BOTTOM_LEFT: char = '╚';
    pub const BOTTOM_RIGHT: char = '╝';
    pub const HORIZONTAL: char = '═';
    pub const VERTICAL: char = '║';
    pub const T_LEFT: char = '╠';
    pub const T_RIGHT: char = '╣';
    pub const T_TOP: char = '╦';
    pub const T_BOTTOM: char = '╩';
    pub const CROSS: char = '╬';

    // Single line variants
    pub const S_TOP_LEFT: char = '┌';
    pub const S_TOP_RIGHT: char = '┐';
    pub const S_BOTTOM_LEFT: char = '└';
    pub const S_BOTTOM_RIGHT: char = '┘';
    pub const S_HORIZONTAL: char = '─';
    pub const S_VERTICAL: char = '│';
    pub const S_T_LEFT: char = '├';
    pub const S_T_RIGHT: char = '┤';

    // Progress bar characters
    pub const BAR_FULL: char = '█';
    pub const BAR_HIGH: char = '▓';
    pub const BAR_MED: char = '▒';
    pub const BAR_LOW: char = '░';
    pub const BAR_EMPTY: char = '·';
}

// ============================================================================
// ENHANCED SYMBOLS FOR ENTITIES
// ============================================================================

mod symbols {
    // Player symbols
    pub const PLAYER: char = '@';
    pub const PLAYER_DEAD: char = '%';

    // Enemy symbols by category
    pub const BEAST_SMALL: char = 'r';      // rats, bats
    pub const BEAST_MEDIUM: char = 'w';     // wolves
    pub const BEAST_LARGE: char = 'B';      // bears
    pub const INSECT: char = 's';           // spiders, wasps
    pub const HUMANOID_WEAK: char = 'g';    // goblins, kobolds
    pub const HUMANOID_MED: char = 'o';     // orcs
    pub const HUMANOID_STRONG: char = 'O';  // ogres
    pub const UNDEAD_WEAK: char = 'z';      // zombies, skeletons
    pub const UNDEAD_STRONG: char = 'V';    // vampires, liches
    pub const GHOST: char = 'G';            // ghosts, wraiths
    pub const DEMON: char = 'D';            // demons
    pub const ELEMENTAL: char = 'E';        // elementals
    pub const BOSS: char = 'X';             // bosses

    // Item symbols
    pub const POTION: char = '!';
    pub const SCROLL: char = '?';
    pub const WEAPON: char = ')';
    pub const ARMOR: char = '[';
    pub const SHIELD: char = ']';
    pub const HELMET: char = '^';
    pub const RING: char = '=';
    pub const AMULET: char = '"';
    pub const GOLD: char = '$';
    pub const KEY: char = 'k';
    pub const FOOD: char = '%';
    pub const MISC: char = '&';

    // Terrain symbols
    pub const WALL: char = '█';
    pub const WALL_ALT: char = '▓';
    pub const FLOOR: char = '·';
    pub const CORRIDOR: char = '░';
    pub const DOOR_CLOSED: char = '+';
    pub const DOOR_OPEN: char = '/';
    pub const DOOR_LOCKED: char = '╬';
    pub const STAIRS_DOWN: char = '▼';
    pub const STAIRS_UP: char = '▲';
    pub const CHEST: char = '■';
    pub const CHEST_OPEN: char = '□';
    pub const TRAP: char = '▽';
    pub const TRAP_DISABLED: char = '○';
    pub const SHRINE: char = '♦';
    pub const WATER: char = '≈';
    pub const LAVA: char = '~';

    // NPC symbols
    pub const NPC_MERCHANT: char = 'M';
    pub const NPC_QUEST: char = 'Q';
    pub const NPC_TRAINER: char = 'T';

    // Companion symbols
    pub const COMPANION: char = 'c';
}

// ============================================================================
// COLOR THEME DEFINITIONS
// ============================================================================

mod theme {
    use crossterm::style::Color;

    // Player colors
    pub const PLAYER: Color = Color::Rgb { r: 0, g: 255, b: 100 };       // Bright green
    pub const PLAYER_DANGER: Color = Color::Rgb { r: 255, g: 100, b: 0 }; // Orange when low HP

    // Enemy colors by strength
    pub const ENEMY_WEAK: Color = Color::Rgb { r: 180, g: 80, b: 80 };
    pub const ENEMY_NORMAL: Color = Color::Rgb { r: 220, g: 50, b: 50 };
    pub const ENEMY_STRONG: Color = Color::Rgb { r: 255, g: 0, b: 0 };
    pub const ENEMY_BOSS: Color = Color::Rgb { r: 255, g: 50, b: 150 };

    // Item colors by rarity
    pub const ITEM_COMMON: Color = Color::Rgb { r: 200, g: 200, b: 200 };
    pub const ITEM_UNCOMMON: Color = Color::Rgb { r: 50, g: 255, b: 50 };
    pub const ITEM_RARE: Color = Color::Rgb { r: 80, g: 150, b: 255 };
    pub const ITEM_EPIC: Color = Color::Rgb { r: 180, g: 80, b: 255 };
    pub const ITEM_LEGENDARY: Color = Color::Rgb { r: 255, g: 200, b: 50 };

    // Environment colors
    pub const WALL: Color = Color::Rgb { r: 80, g: 80, b: 90 };
    pub const WALL_VISIBLE: Color = Color::Rgb { r: 120, g: 120, b: 140 };
    pub const FLOOR: Color = Color::Rgb { r: 60, g: 60, b: 70 };
    pub const FLOOR_VISIBLE: Color = Color::Rgb { r: 100, g: 100, b: 110 };
    pub const STAIRS: Color = Color::Rgb { r: 0, g: 220, b: 220 };
    pub const DOOR: Color = Color::Rgb { r: 180, g: 140, b: 60 };

    // NPC colors
    pub const NPC: Color = Color::Rgb { r: 100, g: 150, b: 255 };
    pub const COMPANION: Color = Color::Rgb { r: 150, g: 255, b: 150 };

    // UI colors
    pub const UI_BORDER: Color = Color::Rgb { r: 100, g: 100, b: 120 };
    pub const UI_TITLE: Color = Color::Rgb { r: 255, g: 220, b: 100 };
    pub const UI_TEXT: Color = Color::Rgb { r: 200, g: 200, b: 200 };
    pub const UI_HIGHLIGHT: Color = Color::Rgb { r: 255, g: 255, b: 100 };

    // Status bar colors
    pub const HP_HIGH: Color = Color::Rgb { r: 50, g: 255, b: 50 };
    pub const HP_MED: Color = Color::Rgb { r: 255, g: 255, b: 50 };
    pub const HP_LOW: Color = Color::Rgb { r: 255, g: 50, b: 50 };
    pub const MP_FULL: Color = Color::Rgb { r: 80, g: 150, b: 255 };
    pub const MP_LOW: Color = Color::Rgb { r: 50, g: 80, b: 180 };
    pub const XP_BAR: Color = Color::Rgb { r: 180, g: 80, b: 255 };
}

// ============================================================================
// HELP SYSTEM
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HelpPage {
    Overview,
    Controls,
    Items,
    Enemies,
    Skills,
    Mechanics,
}

impl HelpPage {
    pub fn title(&self) -> &'static str {
        match self {
            HelpPage::Overview => "Game Overview",
            HelpPage::Controls => "Controls",
            HelpPage::Items => "Items & Equipment",
            HelpPage::Enemies => "Enemies",
            HelpPage::Skills => "Skills & Abilities",
            HelpPage::Mechanics => "Game Mechanics",
        }
    }

    pub fn page_number(&self) -> u8 {
        match self {
            HelpPage::Overview => 1,
            HelpPage::Controls => 2,
            HelpPage::Items => 3,
            HelpPage::Enemies => 4,
            HelpPage::Skills => 5,
            HelpPage::Mechanics => 6,
        }
    }

    pub fn next(&self) -> HelpPage {
        match self {
            HelpPage::Overview => HelpPage::Controls,
            HelpPage::Controls => HelpPage::Items,
            HelpPage::Items => HelpPage::Enemies,
            HelpPage::Enemies => HelpPage::Skills,
            HelpPage::Skills => HelpPage::Mechanics,
            HelpPage::Mechanics => HelpPage::Overview,
        }
    }

    pub fn prev(&self) -> HelpPage {
        match self {
            HelpPage::Overview => HelpPage::Mechanics,
            HelpPage::Controls => HelpPage::Overview,
            HelpPage::Items => HelpPage::Controls,
            HelpPage::Enemies => HelpPage::Items,
            HelpPage::Skills => HelpPage::Enemies,
            HelpPage::Mechanics => HelpPage::Skills,
        }
    }

    pub fn content(&self) -> Vec<(&'static str, &'static str)> {
        match self {
            HelpPage::Overview => vec![
                ("Welcome to ShadowCrypt!", ""),
                ("", ""),
                ("A challenging roguelike dungeon crawler where you", ""),
                ("explore 30 floors of increasingly dangerous depths.", ""),
                ("", ""),
                ("OBJECTIVES:", ""),
                ("  - Survive and grow stronger", ""),
                ("  - Find better equipment", ""),
                ("  - Defeat powerful bosses", ""),
                ("  - Reach the bottom of the dungeon", ""),
                ("", ""),
                ("TIPS:", ""),
                ("  - Don't rush, plan your moves", ""),
                ("  - Manage your resources carefully", ""),
                ("  - Know when to retreat", ""),
                ("  - Use skills strategically", ""),
            ],
            HelpPage::Controls => vec![
                ("MOVEMENT:", ""),
                ("  Arrow Keys / WASD", " - Move in 4 directions"),
                ("  HJKL (Vi keys)", " - Move (Roguelike style)"),
                ("  YUBN", " - Diagonal movement"),
                ("  Shift+Move", " - Auto-run until interrupted"),
                ("  5 or .", " - Wait a turn"),
                ("", ""),
                ("ACTIONS:", ""),
                ("  Space", " - Use current skill"),
                ("  Tab", " - Cycle targets"),
                ("  F", " - Attack target"),
                ("  G", " - Pick up items"),
                ("  > / <", " - Use stairs"),
                ("", ""),
                ("MENUS:", ""),
                ("  I", " - Open inventory"),
                ("  ?", " - This help screen"),
                ("  Q / Esc", " - Quit game"),
            ],
            HelpPage::Items => vec![
                ("EQUIPMENT SLOTS:", ""),
                ("  Weapon, Armor, Shield, Helmet", ""),
                ("  Gloves, Boots, Ring, Amulet", ""),
                ("", ""),
                ("RARITY TIERS:", ""),
                ("  Common", " - Basic items"),
                ("  Uncommon", " - Slightly enhanced"),
                ("  Rare", " - Notable power"),
                ("  Epic", " - Very powerful"),
                ("  Legendary", " - Ultimate gear"),
                ("", ""),
                ("CONSUMABLES:", ""),
                ("  ! Potions", " - Healing, mana, buffs"),
                ("  ? Scrolls", " - Magic effects"),
                ("  % Food", " - Restore hunger"),
            ],
            HelpPage::Enemies => vec![
                ("ENEMY TYPES:", ""),
                ("  r - Rats, small beasts", ""),
                ("  g - Goblins, kobolds", ""),
                ("  o - Orcs, trolls", ""),
                ("  z - Undead (zombies, skeletons)", ""),
                ("  V - Vampires, powerful undead", ""),
                ("  D - Demons", ""),
                ("  E - Elementals", ""),
                ("  X - BOSS (very dangerous!)", ""),
                ("", ""),
                ("BEHAVIOR:", ""),
                ("  Enemies hunt you when visible", ""),
                ("  Some can see in the dark", ""),
                ("  Bosses appear every 5 floors", ""),
            ],
            HelpPage::Skills => vec![
                ("SKILL SYSTEM:", ""),
                ("  Each class has unique skills", ""),
                ("  Press Space to use current skill", ""),
                ("  Shift+Tab to cycle skills", ""),
                ("", ""),
                ("SKILL COSTS:", ""),
                ("  Most skills cost mana", ""),
                ("  Some have cooldowns", ""),
                ("  Plan usage carefully", ""),
                ("", ""),
                ("CLASS ABILITIES:", ""),
                ("  Warrior - Shield & power attacks", ""),
                ("  Mage - Elemental magic", ""),
                ("  Rogue - Stealth & criticals", ""),
                ("  Paladin - Holy damage & heals", ""),
                ("  Ranger - Ranged & traps", ""),
                ("  Necro - Summons & drains", ""),
            ],
            HelpPage::Mechanics => vec![
                ("COMBAT:", ""),
                ("  Damage = ATK - enemy DEF", ""),
                ("  Critical hits deal 2x damage", ""),
                ("  Status effects stack", ""),
                ("", ""),
                ("SURVIVAL:", ""),
                ("  HP regenerates slowly", ""),
                ("  Hunger decreases over time", ""),
                ("  0 hunger = HP loss", ""),
                ("", ""),
                ("EXPLORATION:", ""),
                ("  Shrines give bonuses", ""),
                ("  Chests contain loot", ""),
                ("  Traps can be avoided", ""),
                ("  Stairs connect floors", ""),
            ],
        }
    }
}

pub struct HelpState {
    pub page: HelpPage,
    pub scroll_offset: usize,
}

impl HelpState {
    pub fn new() -> Self {
        Self {
            page: HelpPage::Overview,
            scroll_offset: 0,
        }
    }

    pub fn next_page(&mut self) {
        self.page = self.page.next();
        self.scroll_offset = 0;
    }

    pub fn prev_page(&mut self) {
        self.page = self.page.prev();
        self.scroll_offset = 0;
    }

    pub fn go_to_page(&mut self, page_num: u8) {
        self.page = match page_num {
            1 => HelpPage::Overview,
            2 => HelpPage::Controls,
            3 => HelpPage::Items,
            4 => HelpPage::Enemies,
            5 => HelpPage::Skills,
            6 => HelpPage::Mechanics,
            _ => self.page,
        };
        self.scroll_offset = 0;
    }
}

// ============================================================================
// TUTORIAL SYSTEM
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TutorialStep {
    Welcome,
    Movement,
    Combat,
    Items,
    Skills,
    Exploration,
    Survival,
    Complete,
}

impl TutorialStep {
    pub fn title(&self) -> &'static str {
        match self {
            TutorialStep::Welcome => "Welcome to ShadowCrypt",
            TutorialStep::Movement => "Movement Basics",
            TutorialStep::Combat => "Combat",
            TutorialStep::Items => "Items & Inventory",
            TutorialStep::Skills => "Using Skills",
            TutorialStep::Exploration => "Dungeon Exploration",
            TutorialStep::Survival => "Survival Tips",
            TutorialStep::Complete => "Tutorial Complete",
        }
    }

    pub fn step_number(&self) -> u8 {
        match self {
            TutorialStep::Welcome => 1,
            TutorialStep::Movement => 2,
            TutorialStep::Combat => 3,
            TutorialStep::Items => 4,
            TutorialStep::Skills => 5,
            TutorialStep::Exploration => 6,
            TutorialStep::Survival => 7,
            TutorialStep::Complete => 8,
        }
    }

    pub fn lines(&self) -> Vec<&'static str> {
        match self {
            TutorialStep::Welcome => vec![
                "Welcome, adventurer!",
                "",
                "You stand at the entrance of ShadowCrypt,",
                "a dungeon filled with monsters and treasure.",
                "",
                "Your goal: Descend 30 floors and survive!",
                "",
                "This tutorial will teach you the basics.",
            ],
            TutorialStep::Movement => vec![
                "MOVEMENT CONTROLS:",
                "",
                "  Arrow Keys - Move in 4 directions",
                "  WASD - Alternative movement",
                "  YUBN - Diagonal movement",
                "",
                "  Hold Shift + direction to auto-run",
                "  Press . or 5 to wait a turn",
            ],
            TutorialStep::Combat => vec![
                "COMBAT BASICS:",
                "",
                "  Walk into enemies to attack them",
                "  Press Tab to cycle through targets",
                "  Press F to attack your current target",
                "",
                "  Watch your HP! Red = danger!",
                "  Retreat if overwhelmed.",
            ],
            TutorialStep::Items => vec![
                "ITEMS & EQUIPMENT:",
                "",
                "  Press G to pick up items",
                "  Press I to open inventory",
                "  Number keys 1-9 to quick-use items",
                "",
                "  Better equipment = stronger character",
                "  Watch for color-coded rarity!",
            ],
            TutorialStep::Skills => vec![
                "USING SKILLS:",
                "",
                "  Press Space to use your current skill",
                "  Shift+Tab to cycle between skills",
                "",
                "  Skills cost mana (blue bar)",
                "  Each class has unique abilities",
            ],
            TutorialStep::Exploration => vec![
                "EXPLORING THE DUNGEON:",
                "",
                "  > = Stairs down (next floor)",
                "  < = Stairs up (previous floor)",
                "  $ = Chests with loot",
                "  * = Shrines give bonuses",
                "",
                "  The deeper you go, the harder it gets!",
            ],
            TutorialStep::Survival => vec![
                "SURVIVAL TIPS:",
                "",
                "  - Manage your hunger (eat food!)",
                "  - Save potions for emergencies",
                "  - Don't fight every enemy",
                "  - Explore carefully",
                "",
                "  Good luck, adventurer!",
            ],
            TutorialStep::Complete => vec![
                "TUTORIAL COMPLETE!",
                "",
                "You now know the basics of ShadowCrypt.",
                "",
                "Press ? anytime to see the help menu.",
                "",
                "May fortune favor the bold!",
            ],
        }
    }

    pub fn hint(&self) -> &'static str {
        match self {
            TutorialStep::Complete => "[Press any key to begin your adventure]",
            _ => "[Press Space to continue, Esc to skip tutorial]",
        }
    }

    pub fn next(&self) -> TutorialStep {
        match self {
            TutorialStep::Welcome => TutorialStep::Movement,
            TutorialStep::Movement => TutorialStep::Combat,
            TutorialStep::Combat => TutorialStep::Items,
            TutorialStep::Items => TutorialStep::Skills,
            TutorialStep::Skills => TutorialStep::Exploration,
            TutorialStep::Exploration => TutorialStep::Survival,
            TutorialStep::Survival => TutorialStep::Complete,
            TutorialStep::Complete => TutorialStep::Complete,
        }
    }
}

pub struct TutorialState {
    pub step: TutorialStep,
    pub active: bool,
    pub completed: bool,
}

impl TutorialState {
    pub fn new() -> Self {
        Self {
            step: TutorialStep::Welcome,
            active: false,
            completed: false,
        }
    }

    pub fn advance(&mut self) {
        if self.step == TutorialStep::Complete {
            self.active = false;
            self.completed = true;
        } else {
            self.step = self.step.next();
        }
    }

    pub fn skip(&mut self) {
        self.active = false;
        self.completed = true;
    }
}

// ============================================================================
// ASCII ART SCREENS
// ============================================================================

pub const TITLE_ART: &str = r#"
    ███████╗██╗  ██╗ █████╗ ██████╗  ██████╗ ██╗    ██╗ ██████╗██████╗ ██╗   ██╗██████╗ ████████╗
    ██╔════╝██║  ██║██╔══██╗██╔══██╗██╔═══██╗██║    ██║██╔════╝██╔══██╗╚██╗ ██╔╝██╔══██╗╚══██╔══╝
    ███████╗███████║███████║██║  ██║██║   ██║██║ █╗ ██║██║     ██████╔╝ ╚████╔╝ ██████╔╝   ██║
    ╚════██║██╔══██║██╔══██║██║  ██║██║   ██║██║███╗██║██║     ██╔══██╗  ╚██╔╝  ██╔═══╝    ██║
    ███████║██║  ██║██║  ██║██████╔╝╚██████╔╝╚███╔███╔╝╚██████╗██║  ██║   ██║   ██║        ██║
    ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═════╝  ╚═════╝  ╚══╝╚══╝  ╚═════╝╚═╝  ╚═╝   ╚═╝   ╚═╝        ╚═╝
"#;

pub const DEATH_ART: &str = r#"
                              ___________
                             /           \
                            /   R.I.P.    \
                           /               \
                          |   Here lies    |
                          |   a brave      |
                          |   adventurer   |
                          |                |
                     _____|________________|_____
                    /                            \
                   /        ╔═══════════╗         \
                  |         ║  YOU DIED ║          |
                   \        ╚═══════════╝         /
                    \____________________________/
"#;

pub const VICTORY_ART: &str = r#"
                                    ╔════════════════════════════════╗
                                    ║                                ║
        ██╗   ██╗██╗ ██████╗████████║  ★  ★  ★  CHAMPION  ★  ★  ★   ║
        ██║   ██║██║██╔════╝╚══██╔══║                                ║
        ██║   ██║██║██║        ██║  ║  You have conquered the       ║
        ╚██╗ ██╔╝██║██║        ██║  ║  depths of ShadowCrypt!       ║
         ╚████╔╝ ██║╚██████╗   ██║  ║                                ║
          ╚═══╝  ╚═╝ ╚═════╝   ╚═╝  ║  Your legend will be          ║
                ██████╗ ██████╗ ██╗ ║  remembered forever!           ║
                ██╔══██╗╚════██╗██║ ║                                ║
                ██████╔╝ █████╔╝██║ ╚════════════════════════════════╝
                ██╔══██╗ ╚═══██╗╚═╝
                ██║  ██║██████╔╝██╗
                ╚═╝  ╚═╝╚═════╝ ╚═╝
"#;

pub const BOSS_ART: &str = r#"
    ╔══════════════════════════════════════════════════════════════════╗
    ║                                                                  ║
    ║     ██████╗  ██████╗ ███████╗███████╗    ██╗██╗██╗               ║
    ║     ██╔══██╗██╔═══██╗██╔════╝██╔════╝    ██║██║██║               ║
    ║     ██████╔╝██║   ██║███████╗███████╗    ██║██║██║               ║
    ║     ██╔══██╗██║   ██║╚════██║╚════██║    ╚═╝╚═╝╚═╝               ║
    ║     ██████╔╝╚██████╔╝███████║███████║    ██╗██╗██╗               ║
    ║     ╚═════╝  ╚═════╝ ╚══════╝╚══════╝    ╚═╝╚═╝╚═╝               ║
    ║                                                                  ║
    ║              A POWERFUL ENEMY APPROACHES...                      ║
    ║                                                                  ║
    ╚══════════════════════════════════════════════════════════════════╝
"#;

pub const LEVEL_UP_ART: &str = r#"
    ╔════════════════════════════════════════╗
    ║    ★ ★ ★  L E V E L   U P !  ★ ★ ★    ║
    ╠════════════════════════════════════════╣
    ║                                        ║
    ║     Your power has increased!          ║
    ║                                        ║
    ╚════════════════════════════════════════╝
"#;

// ============================================================================
// COMBAT ANIMATION SYSTEM
// ============================================================================

/// Types of combat visual effects
#[derive(Clone, Debug)]
enum CombatEffectType {
    /// Player deals damage to enemy
    PlayerHit { damage: i32, is_critical: bool },
    /// Enemy deals damage to player
    EnemyHit { damage: i32 },
    /// Attack missed
    Miss,
    /// Attack was blocked/dodged
    Block,
    /// Healing effect
    Heal { amount: i32 },
    /// Skill activation
    SkillUse { skill_name: String },
    /// Enemy death
    Death { enemy_name: String },
    /// Player level up
    LevelUp,
    /// Status effect applied
    StatusApplied { effect_name: String },
    /// Projectile/ranged attack
    Projectile { from_x: usize, from_y: usize, to_x: usize, to_y: usize },
}

/// A combat effect with position and timing
#[derive(Clone, Debug)]
struct CombatEffect {
    x: usize,
    y: usize,
    effect_type: CombatEffectType,
    created_at: Instant,
    duration_ms: u64,
}

impl CombatEffect {
    fn new(x: usize, y: usize, effect_type: CombatEffectType, duration_ms: u64) -> Self {
        Self {
            x,
            y,
            effect_type,
            created_at: Instant::now(),
            duration_ms,
        }
    }

    fn is_expired(&self) -> bool {
        self.created_at.elapsed().as_millis() as u64 > self.duration_ms
    }

    fn progress(&self) -> f32 {
        let elapsed = self.created_at.elapsed().as_millis() as f32;
        (elapsed / self.duration_ms as f32).min(1.0)
    }
}

/// Manages all active combat animations
struct CombatAnimator {
    effects: VecDeque<CombatEffect>,
    screen_shake: Option<(Instant, u64, i8)>, // start time, duration, intensity
    border_flash: Option<(Instant, u64, Color)>, // start time, duration, color
    last_combat_message: Option<String>,
    combo_count: u32,
    combo_timer: Option<Instant>,
}

impl CombatAnimator {
    fn new() -> Self {
        Self {
            effects: VecDeque::new(),
            screen_shake: None,
            border_flash: None,
            last_combat_message: None,
            combo_count: 0,
            combo_timer: None,
        }
    }

    /// Add a new combat effect
    fn add_effect(&mut self, x: usize, y: usize, effect_type: CombatEffectType) {
        let duration = match &effect_type {
            CombatEffectType::PlayerHit { is_critical, .. } => {
                if *is_critical { 600 } else { 400 }
            }
            CombatEffectType::EnemyHit { .. } => 350,
            CombatEffectType::Miss => 300,
            CombatEffectType::Block => 350,
            CombatEffectType::Heal { .. } => 500,
            CombatEffectType::SkillUse { .. } => 450,
            CombatEffectType::Death { .. } => 700,
            CombatEffectType::LevelUp => 1000,
            CombatEffectType::StatusApplied { .. } => 400,
            CombatEffectType::Projectile { .. } => 200,
        };

        self.effects.push_back(CombatEffect::new(x, y, effect_type, duration));

        // Limit effects to prevent memory issues
        while self.effects.len() > 20 {
            self.effects.pop_front();
        }
    }

    /// Trigger screen shake effect
    fn shake_screen(&mut self, duration_ms: u64, intensity: i8) {
        self.screen_shake = Some((Instant::now(), duration_ms, intensity));
    }

    /// Trigger border flash effect
    fn flash_border(&mut self, duration_ms: u64, color: Color) {
        self.border_flash = Some((Instant::now(), duration_ms, color));
    }

    /// Update combo counter
    fn register_hit(&mut self) {
        let now = Instant::now();
        if let Some(timer) = self.combo_timer {
            if timer.elapsed().as_millis() < 1500 {
                self.combo_count += 1;
            } else {
                self.combo_count = 1;
            }
        } else {
            self.combo_count = 1;
        }
        self.combo_timer = Some(now);
    }

    /// Get current combo count (0 if expired)
    fn get_combo(&self) -> u32 {
        if let Some(timer) = self.combo_timer {
            if timer.elapsed().as_millis() < 1500 {
                return self.combo_count;
            }
        }
        0
    }

    /// Clean up expired effects
    fn update(&mut self) {
        self.effects.retain(|e| !e.is_expired());

        if let Some((start, duration, _)) = self.screen_shake {
            if start.elapsed().as_millis() as u64 > duration {
                self.screen_shake = None;
            }
        }

        if let Some((start, duration, _)) = self.border_flash {
            if start.elapsed().as_millis() as u64 > duration {
                self.border_flash = None;
            }
        }
    }

    /// Get screen shake offset
    fn get_shake_offset(&self) -> (i8, i8) {
        if let Some((start, duration, intensity)) = self.screen_shake {
            let elapsed = start.elapsed().as_millis() as f32;
            let progress = elapsed / duration as f32;
            if progress < 1.0 {
                let decay = 1.0 - progress;
                let shake_x = ((elapsed * 0.1).sin() * intensity as f32 * decay) as i8;
                let shake_y = ((elapsed * 0.15).cos() * intensity as f32 * decay * 0.5) as i8;
                return (shake_x, shake_y);
            }
        }
        (0, 0)
    }

    /// Check if border should flash
    fn get_border_flash(&self) -> Option<Color> {
        if let Some((start, duration, color)) = self.border_flash {
            let elapsed = start.elapsed().as_millis() as u64;
            if elapsed < duration {
                // Pulse effect
                let phase = (elapsed as f32 / 50.0).sin();
                if phase > 0.0 {
                    return Some(color);
                }
            }
        }
        None
    }
}

/// Get the hit indicator character based on damage and timing
fn get_hit_indicator(effect: &CombatEffect) -> (char, Color) {
    let progress = effect.progress();

    match &effect.effect_type {
        CombatEffectType::PlayerHit { is_critical, .. } => {
            if *is_critical {
                let chars = ['*', 'X', '*', '+'];
                let idx = ((progress * 4.0) as usize).min(3);
                (chars[idx], Color::Yellow)
            } else {
                let chars = ['/', '\\', '|', '-'];
                let idx = ((progress * 4.0) as usize).min(3);
                (chars[idx], Color::White)
            }
        }
        CombatEffectType::EnemyHit { .. } => {
            let chars = ['!', '*', '!', '.'];
            let idx = ((progress * 4.0) as usize).min(3);
            (chars[idx], Color::Red)
        }
        CombatEffectType::Miss => ('o', Color::DarkGrey),
        CombatEffectType::Block => ('#', Color::Cyan),
        CombatEffectType::Death { .. } => {
            let chars = ['%', '&', '#', '.'];
            let idx = ((progress * 4.0) as usize).min(3);
            (chars[idx], Color::DarkRed)
        }
        _ => (' ', Color::White),
    }
}

/// Format damage number with styling
fn format_damage_number(damage: i32, is_critical: bool, progress: f32) -> (String, Color, i16) {
    let y_offset = (progress * 3.0) as i16; // Float upward

    if is_critical {
        (format!("*{}*", damage), Color::Yellow, -y_offset)
    } else {
        (format!("-{}", damage), Color::Red, -y_offset)
    }
}

/// Format healing number
fn format_heal_number(amount: i32, progress: f32) -> (String, Color, i16) {
    let y_offset = (progress * 2.0) as i16;
    (format!("+{}", amount), Color::Green, -y_offset)
}

// ============================================================================
// EQUIPMENT COMPARISON SYSTEM
// ============================================================================

/// Represents the stat difference between two items
struct StatComparison {
    attack_diff: i32,
    defense_diff: i32,
    hp_diff: i32,
    mana_diff: i32,
}

impl StatComparison {
    /// Compare a new item to a currently equipped item (or no item)
    fn compare(new_item: &Item, equipped: Option<&Item>) -> Self {
        let (new_atk, new_def, new_hp, new_mana) = new_item.stats();
        let (old_atk, old_def, old_hp, old_mana) = equipped
            .map(|i| i.stats())
            .unwrap_or((0, 0, 0, 0));

        StatComparison {
            attack_diff: new_atk - old_atk,
            defense_diff: new_def - old_def,
            hp_diff: new_hp - old_hp,
            mana_diff: new_mana - old_mana,
        }
    }

    /// Check if there are any stat differences worth displaying
    fn has_differences(&self) -> bool {
        self.attack_diff != 0 || self.defense_diff != 0 ||
        self.hp_diff != 0 || self.mana_diff != 0
    }
}

/// Format a single stat difference with color coding
fn format_stat_diff(diff: i32, name: &str) -> Option<(String, Color)> {
    if diff == 0 {
        return None;
    }
    let color = if diff > 0 { Color::Green } else { Color::Red };
    let sign = if diff > 0 { "+" } else { "" };
    Some((format!("{}{}{}", sign, diff, name), color))
}

/// Render stat comparison inline after an item name
fn render_stat_comparison_inline(stdout: &mut std::io::Stdout, comparison: &StatComparison) -> std::io::Result<()> {
    let mut parts: Vec<(String, Color)> = Vec::new();

    if let Some(part) = format_stat_diff(comparison.attack_diff, "ATK") {
        parts.push(part);
    }
    if let Some(part) = format_stat_diff(comparison.defense_diff, "DEF") {
        parts.push(part);
    }
    if let Some(part) = format_stat_diff(comparison.hp_diff, "HP") {
        parts.push(part);
    }
    if let Some(part) = format_stat_diff(comparison.mana_diff, "MP") {
        parts.push(part);
    }

    if !parts.is_empty() {
        execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
        write!(stdout, " (")?;
        for (i, (text, color)) in parts.iter().enumerate() {
            if i > 0 {
                execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
                write!(stdout, " ")?;
            }
            execute!(stdout, SetForegroundColor(*color))?;
            write!(stdout, "{}", text)?;
        }
        execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
        write!(stdout, ")")?;
        execute!(stdout, ResetColor)?;
    }

    Ok(())
}

/// Get the equipped item for comparison based on an item's equip slot
fn get_equipped_for_comparison<'a>(player: &'a Player, item: &Item) -> Option<&'a Item> {
    if let Some(slot) = item.kind.equip_slot() {
        // Handle ring slots - compare against Ring1 first, then Ring2
        match slot {
            EquipSlot::Ring1 => {
                player.equipment.get(&EquipSlot::Ring1)
                    .or_else(|| player.equipment.get(&EquipSlot::Ring2))
            }
            _ => player.equipment.get(&slot)
        }
    } else {
        None
    }
}

// ============================================================================
// MOVEMENT SYSTEM - Direction handling and movement state
// ============================================================================

/// Represents a movement direction with dx, dy offsets
#[derive(Clone, Copy, Debug, PartialEq)]
struct Direction {
    dx: i32,
    dy: i32,
}

impl Direction {
    const NORTH: Direction = Direction { dx: 0, dy: -1 };
    const SOUTH: Direction = Direction { dx: 0, dy: 1 };
    const EAST: Direction = Direction { dx: 1, dy: 0 };
    const WEST: Direction = Direction { dx: -1, dy: 0 };
    const NORTHEAST: Direction = Direction { dx: 1, dy: -1 };
    const NORTHWEST: Direction = Direction { dx: -1, dy: -1 };
    const SOUTHEAST: Direction = Direction { dx: 1, dy: 1 };
    const SOUTHWEST: Direction = Direction { dx: -1, dy: 1 };
    const NONE: Direction = Direction { dx: 0, dy: 0 };

    /// Get a descriptive name for the direction
    fn name(&self) -> &'static str {
        match (self.dx, self.dy) {
            (0, -1) => "North",
            (0, 1) => "South",
            (1, 0) => "East",
            (-1, 0) => "West",
            (1, -1) => "NE",
            (-1, -1) => "NW",
            (1, 1) => "SE",
            (-1, 1) => "SW",
            _ => "",
        }
    }

    /// Get an arrow symbol for the direction
    fn arrow(&self) -> char {
        match (self.dx, self.dy) {
            (0, -1) => '^',
            (0, 1) => 'v',
            (1, 0) => '>',
            (-1, 0) => '<',
            (1, -1) => '/',   // NE: going up-right
            (-1, -1) => '\\', // NW: going up-left
            (1, 1) => '\\',   // SE: going down-right
            (-1, 1) => '/',   // SW: going down-left
            _ => '*',
        }
    }

    /// Check if this is a diagonal direction
    fn is_diagonal(&self) -> bool {
        self.dx != 0 && self.dy != 0
    }

    /// Check if this is a valid movement direction (not stationary)
    fn is_valid(&self) -> bool {
        self.dx != 0 || self.dy != 0
    }
}

/// Movement state for tracking running and last direction
struct MovementState {
    last_direction: Option<Direction>,
    last_move_time: Instant,
    is_running: bool,
    run_direction: Option<Direction>,
    move_count: u32,
}

impl MovementState {
    fn new() -> Self {
        Self {
            last_direction: None,
            last_move_time: Instant::now(),
            is_running: false,
            run_direction: None,
            move_count: 0,
        }
    }

    /// Record a movement
    fn record_move(&mut self, dir: Direction) {
        self.last_direction = Some(dir);
        self.last_move_time = Instant::now();
        self.move_count += 1;
    }

    /// Start running in a direction
    fn start_run(&mut self, dir: Direction) {
        self.is_running = true;
        self.run_direction = Some(dir);
    }

    /// Stop running
    fn stop_run(&mut self) {
        self.is_running = false;
        self.run_direction = None;
    }

    /// Check if we should continue running (no enemies visible, not blocked)
    fn should_continue_run(&self, state: &GameState) -> bool {
        if !self.is_running {
            return false;
        }

        // Stop if any enemy is visible
        for enemy in &state.enemies {
            if enemy.is_alive() && state.map.visible[enemy.y][enemy.x] {
                return false;
            }
        }

        // Check if we can continue in the run direction
        if let Some(dir) = self.run_direction {
            let new_x = (state.player.x as i32 + dir.dx).max(0) as usize;
            let new_y = (state.player.y as i32 + dir.dy).max(0) as usize;

            // Stop at interesting tiles
            let tile = state.map.tiles[new_y][new_x];
            if matches!(tile, Tile::Door | Tile::Chest | Tile::Shrine | Tile::StairsDown | Tile::StairsUp | Tile::Trap) {
                return false;
            }

            // Stop if blocked
            if !state.map.is_walkable(new_x, new_y) {
                return false;
            }

            // Stop if there's an item here
            if state.items.iter().any(|i| i.x == new_x && i.y == new_y) {
                return false;
            }

            return true;
        }

        false
    }

    /// Get movement indicator string for display
    fn get_direction_indicator(&self) -> Option<(char, &'static str)> {
        // Only show indicator if recent movement
        if self.last_move_time.elapsed().as_millis() < 800 {
            if let Some(dir) = self.last_direction {
                return Some((dir.arrow(), dir.name()));
            }
        }
        None
    }
}

// ============================================================================
// TARGETING SYSTEM
// ============================================================================

/// Information about a targetable enemy
#[derive(Clone, Debug)]
struct TargetInfo {
    /// Index in the enemies vector
    enemy_index: usize,
    /// Enemy position
    x: usize,
    y: usize,
    /// Distance from player (squared, for sorting)
    distance_sq: i32,
    /// Enemy name
    name: String,
    /// Current HP
    hp: i32,
    /// Max HP
    max_hp: i32,
    /// Is boss
    is_boss: bool,
    /// Attack stat
    attack: i32,
    /// Defense stat
    defense: i32,
    /// Active status effects
    status_effects: Vec<(String, u32)>,
}

/// State for the targeting system
struct TargetingState {
    /// Whether targeting mode is active
    active: bool,
    /// Index of currently selected target in visible_targets
    current_index: usize,
    /// List of visible targetable enemies
    visible_targets: Vec<TargetInfo>,
    /// Time when targeting mode was activated (for blinking effect)
    activated_at: Instant,
    /// Whether target is locked (persists between turns)
    locked: bool,
}

impl TargetingState {
    fn new() -> Self {
        Self {
            active: false,
            current_index: 0,
            visible_targets: Vec::new(),
            activated_at: Instant::now(),
            locked: false,
        }
    }

    /// Update the list of visible targets from the game state
    fn update_targets(&mut self, state: &GameState) {
        let px = state.player.x as i32;
        let py = state.player.y as i32;

        // Remember current target position if locked
        let locked_pos = if self.locked {
            self.current_target().map(|t| (t.x, t.y))
        } else {
            None
        };

        self.visible_targets.clear();

        for (idx, enemy) in state.enemies.iter().enumerate() {
            // Only include alive enemies that are visible
            if enemy.is_alive() && state.map.visible[enemy.y][enemy.x] {
                let dx = enemy.x as i32 - px;
                let dy = enemy.y as i32 - py;
                let distance_sq = dx * dx + dy * dy;

                let status_effects: Vec<(String, u32)> = enemy
                    .status_effects
                    .iter()
                    .map(|(e, d)| (e.name().to_string(), *d))
                    .collect();

                self.visible_targets.push(TargetInfo {
                    enemy_index: idx,
                    x: enemy.x,
                    y: enemy.y,
                    distance_sq,
                    name: enemy.kind.name().to_string(),
                    hp: enemy.hp,
                    max_hp: enemy.max_hp,
                    is_boss: enemy.kind.is_boss(),
                    attack: enemy.attack,
                    defense: enemy.defense,
                    status_effects,
                });
            }
        }

        // Sort by distance (closest first)
        self.visible_targets.sort_by_key(|t| t.distance_sq);

        // Try to maintain locked target
        if let Some((lx, ly)) = locked_pos {
            if let Some(new_idx) = self.visible_targets.iter().position(|t| t.x == lx && t.y == ly) {
                self.current_index = new_idx;
            } else {
                // Target no longer visible, unlock
                self.locked = false;
                self.current_index = 0;
            }
        }

        // Validate current index
        if self.current_index >= self.visible_targets.len() {
            self.current_index = 0;
        }

        // If no targets available, deactivate targeting
        if self.visible_targets.is_empty() {
            self.active = false;
            self.locked = false;
        }
    }

    /// Activate targeting mode
    fn activate(&mut self) {
        if !self.visible_targets.is_empty() {
            self.active = true;
            self.activated_at = Instant::now();
            if !self.locked {
                self.current_index = 0;
            }
        }
    }

    /// Deactivate targeting mode
    fn deactivate(&mut self) {
        self.active = false;
        self.locked = false;
    }

    /// Toggle targeting mode
    fn toggle(&mut self) {
        if self.active {
            self.deactivate();
        } else {
            self.activate();
        }
    }

    /// Cycle to the next target
    fn next_target(&mut self) {
        if !self.visible_targets.is_empty() {
            self.current_index = (self.current_index + 1) % self.visible_targets.len();
            if !self.active {
                self.activate();
            }
        }
    }

    /// Cycle to the previous target
    fn prev_target(&mut self) {
        if !self.visible_targets.is_empty() {
            if self.current_index == 0 {
                self.current_index = self.visible_targets.len() - 1;
            } else {
                self.current_index -= 1;
            }
            if !self.active {
                self.activate();
            }
        }
    }

    /// Lock/unlock the current target
    fn toggle_lock(&mut self) {
        if self.active && !self.visible_targets.is_empty() {
            self.locked = !self.locked;
        }
    }

    /// Get the currently selected target
    fn current_target(&self) -> Option<&TargetInfo> {
        if self.active && !self.visible_targets.is_empty() {
            Some(&self.visible_targets[self.current_index])
        } else {
            None
        }
    }

    /// Get the enemy index of the current target (for attacking)
    fn current_enemy_index(&self) -> Option<usize> {
        self.current_target().map(|t| t.enemy_index)
    }

    /// Check if a specific position is the current target
    fn is_targeted(&self, x: usize, y: usize) -> bool {
        if let Some(target) = self.current_target() {
            target.x == x && target.y == y
        } else {
            false
        }
    }

    /// Get blink state for target indicator (for animation)
    fn should_show_indicator(&self) -> bool {
        let elapsed = self.activated_at.elapsed().as_millis();
        (elapsed / 200) % 2 == 0
    }

    /// Calculate direction from player to target
    fn direction_to_target(&self, player_x: usize, player_y: usize) -> Option<(i32, i32)> {
        self.current_target().map(|t| {
            let dx = t.x as i32 - player_x as i32;
            let dy = t.y as i32 - player_y as i32;
            (dx.signum(), dy.signum())
        })
    }

    /// Get number of visible targets
    fn target_count(&self) -> usize {
        self.visible_targets.len()
    }

    /// Check if an enemy at this position is adjacent to the player
    fn is_adjacent(&self, player_x: usize, player_y: usize) -> bool {
        if let Some(target) = self.current_target() {
            let dx = (target.x as i32 - player_x as i32).abs();
            let dy = (target.y as i32 - player_y as i32).abs();
            dx <= 1 && dy <= 1
        } else {
            false
        }
    }
}

// ============================================================================
// ENHANCED MESSAGE LOG SYSTEM
// ============================================================================

/// Categories for game messages to enable filtering
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MessageCategory {
    Combat,   // Attack, damage, kills
    Loot,     // Item pickups, gold, equipment
    System,   // Game events, level changes, status
    All,      // Special filter to show all messages
}

impl MessageCategory {
    pub fn name(&self) -> &'static str {
        match self {
            MessageCategory::Combat => "Combat",
            MessageCategory::Loot => "Loot",
            MessageCategory::System => "System",
            MessageCategory::All => "All",
        }
    }

    pub fn label_color(&self) -> Color {
        match self {
            MessageCategory::Combat => Color::Red,
            MessageCategory::Loot => Color::Yellow,
            MessageCategory::System => Color::Cyan,
            MessageCategory::All => Color::White,
        }
    }

    pub fn tag(&self) -> &'static str {
        match self {
            MessageCategory::Combat => "[CMB]",
            MessageCategory::Loot => "[LT]",
            MessageCategory::System => "[SYS]",
            MessageCategory::All => "",
        }
    }
}

/// An enhanced message with category and timestamp
#[derive(Clone)]
pub struct EnhancedMessage {
    pub text: String,
    pub color_index: u8,
    pub category: MessageCategory,
    pub turn_number: u32,
    pub elapsed_secs: f32,
}

impl EnhancedMessage {
    pub fn new(text: String, color_index: u8, category: MessageCategory, turn: u32, elapsed: f32) -> Self {
        Self { text, color_index, category, turn_number: turn, elapsed_secs: elapsed }
    }

    pub fn formatted_time(&self) -> String {
        let total_secs = self.elapsed_secs as u32;
        format!("{:02}:{:02}", total_secs / 60, total_secs % 60)
    }
}

/// Enhanced message log with scrolling, history, and filtering
pub struct MessageLog {
    messages: Vec<EnhancedMessage>,
    max_history: usize,
    scroll_offset: usize,
    visible_count: usize,
    active_filter: MessageCategory,
    show_timestamps: bool,
    show_categories: bool,
    game_start: Instant,
    last_processed_count: usize,
    expanded_view: bool,
}

impl MessageLog {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            max_history: 500,
            scroll_offset: 0,
            visible_count: 6,
            active_filter: MessageCategory::All,
            show_timestamps: true,
            show_categories: true,
            game_start: Instant::now(),
            last_processed_count: 0,
            expanded_view: false,
        }
    }

    fn categorize_message(text: &str, color_index: u8) -> MessageCategory {
        let t = text.to_lowercase();
        if t.contains("hit") || t.contains("damage") || t.contains("attack") || t.contains("dead")
            || t.contains("kill") || t.contains("miss") || t.contains("strike") || t.contains("slain")
            || t.contains("poison") || t.contains("burn") || t.contains("freeze") || t.contains("stun")
            || t.contains("enemy") || color_index == 2 || color_index == 3 {
            return MessageCategory::Combat;
        }
        if t.contains("picked up") || t.contains("gold") || t.contains("found") || t.contains("equipped")
            || t.contains("chest") || t.contains("item") || t.contains("key") || t.contains("potion")
            || t.contains("xp") || t.contains("level up") {
            return MessageCategory::Loot;
        }
        MessageCategory::System
    }

    pub fn sync_from_game(&mut self, game_messages: &[GameMessage], turn_count: u32) {
        let elapsed = self.game_start.elapsed().as_secs_f32();
        for msg in game_messages.iter().skip(self.last_processed_count) {
            let category = Self::categorize_message(&msg.text, msg.color_index);
            self.messages.push(EnhancedMessage::new(msg.text.clone(), msg.color_index, category, turn_count, elapsed));
        }
        self.last_processed_count = game_messages.len();
        while self.messages.len() > self.max_history { self.messages.remove(0); }
        if self.scroll_offset == 0 { self.scroll_to_bottom(); }
    }

    pub fn filtered_messages(&self) -> Vec<&EnhancedMessage> {
        if self.active_filter == MessageCategory::All {
            self.messages.iter().collect()
        } else {
            self.messages.iter().filter(|m| m.category == self.active_filter).collect()
        }
    }

    pub fn visible_messages(&self) -> Vec<&EnhancedMessage> {
        let filtered = self.filtered_messages();
        let total = filtered.len();
        let display_count = if self.expanded_view { 20 } else { self.visible_count };
        if total == 0 { return Vec::new(); }
        let start = total.saturating_sub(display_count + self.scroll_offset);
        let end = total.saturating_sub(self.scroll_offset);
        filtered[start..end].to_vec()
    }

    pub fn scroll_up(&mut self) {
        let fc = self.filtered_messages().len();
        let dc = if self.expanded_view { 20 } else { self.visible_count };
        if self.scroll_offset + dc < fc { self.scroll_offset += 1; }
    }

    pub fn scroll_down(&mut self) {
        if self.scroll_offset > 0 { self.scroll_offset -= 1; }
    }

    pub fn scroll_to_bottom(&mut self) { self.scroll_offset = 0; }

    pub fn scroll_to_top(&mut self) {
        let fc = self.filtered_messages().len();
        let dc = if self.expanded_view { 20 } else { self.visible_count };
        if fc > dc { self.scroll_offset = fc - dc; }
    }

    pub fn page_up(&mut self) {
        let fc = self.filtered_messages().len();
        let dc = if self.expanded_view { 20 } else { self.visible_count };
        let max_off = fc.saturating_sub(dc);
        self.scroll_offset = (self.scroll_offset + dc).min(max_off);
    }

    pub fn page_down(&mut self) {
        let dc = if self.expanded_view { 20 } else { self.visible_count };
        self.scroll_offset = self.scroll_offset.saturating_sub(dc);
    }

    pub fn cycle_filter(&mut self) {
        self.active_filter = match self.active_filter {
            MessageCategory::All => MessageCategory::Combat,
            MessageCategory::Combat => MessageCategory::Loot,
            MessageCategory::Loot => MessageCategory::System,
            MessageCategory::System => MessageCategory::All,
        };
        self.scroll_offset = 0;
    }

    pub fn toggle_timestamps(&mut self) { self.show_timestamps = !self.show_timestamps; }
    pub fn toggle_categories(&mut self) { self.show_categories = !self.show_categories; }
    pub fn toggle_expanded(&mut self) { self.expanded_view = !self.expanded_view; self.scroll_offset = 0; }

    pub fn can_scroll_up(&self) -> bool {
        let fc = self.filtered_messages().len();
        let dc = if self.expanded_view { 20 } else { self.visible_count };
        self.scroll_offset + dc < fc
    }

    pub fn can_scroll_down(&self) -> bool { self.scroll_offset > 0 }

    pub fn scroll_info(&self) -> String {
        let f = self.filtered_messages().len();
        let dc = if self.expanded_view { 20 } else { self.visible_count };
        if f <= dc { format!("{}/{}", f, self.messages.len()) }
        else {
            let top = f.saturating_sub(self.scroll_offset + dc) + 1;
            let bot = f.saturating_sub(self.scroll_offset);
            format!("{}-{}/{}", top, bot, f)
        }
    }

    pub fn is_expanded(&self) -> bool { self.expanded_view }
    pub fn get_filter(&self) -> MessageCategory { self.active_filter }
}

/// Parse key input to get direction
fn key_to_direction(code: KeyCode) -> Option<Direction> {
    match code {
        // Arrow keys
        KeyCode::Up => Some(Direction::NORTH),
        KeyCode::Down => Some(Direction::SOUTH),
        KeyCode::Left => Some(Direction::WEST),
        KeyCode::Right => Some(Direction::EAST),

        // WASD keys
        KeyCode::Char('w') | KeyCode::Char('W') => Some(Direction::NORTH),
        KeyCode::Char('s') | KeyCode::Char('S') => Some(Direction::SOUTH),
        KeyCode::Char('a') | KeyCode::Char('A') => Some(Direction::WEST),
        KeyCode::Char('d') | KeyCode::Char('D') => Some(Direction::EAST),

        // Vi keys (hjkl)
        KeyCode::Char('h') | KeyCode::Char('H') => Some(Direction::WEST),
        KeyCode::Char('j') | KeyCode::Char('J') => Some(Direction::SOUTH),
        KeyCode::Char('k') | KeyCode::Char('K') => Some(Direction::NORTH),
        KeyCode::Char('l') | KeyCode::Char('L') => Some(Direction::EAST),

        // Diagonal vi keys (yubn)
        KeyCode::Char('y') | KeyCode::Char('Y') => Some(Direction::NORTHWEST),
        KeyCode::Char('u') | KeyCode::Char('U') => Some(Direction::NORTHEAST),
        KeyCode::Char('b') | KeyCode::Char('B') => Some(Direction::SOUTHWEST),
        KeyCode::Char('n') | KeyCode::Char('N') => Some(Direction::SOUTHEAST),

        // Numpad movement (common roguelike standard)
        // 7 8 9
        // 4 . 6
        // 1 2 3
        KeyCode::Home => Some(Direction::NORTHWEST),     // Numpad 7
        KeyCode::End => Some(Direction::SOUTHWEST),      // Numpad 1
        KeyCode::PageUp => Some(Direction::NORTHEAST),   // Numpad 9
        KeyCode::PageDown => Some(Direction::SOUTHEAST), // Numpad 3

        _ => None,
    }
}

// ============================================================================
// RENDERING HELPERS - Convert core types to terminal display
// ============================================================================

/// Get the terminal color for a status effect
fn status_color(effect: &StatusEffect) -> Color {
    match effect {
        StatusEffect::Poison => Color::Green,
        StatusEffect::Burn => Color::Red,
        StatusEffect::Freeze => Color::Cyan,
        StatusEffect::Bleed => Color::DarkRed,
        StatusEffect::Stun => Color::Yellow,
        StatusEffect::Blind => Color::DarkGrey,
        StatusEffect::Haste => Color::Blue,
        StatusEffect::Shield => Color::White,
        StatusEffect::Regeneration => Color::Magenta,
        StatusEffect::Strength => Color::Yellow,
        StatusEffect::Weakness => Color::DarkMagenta,
        StatusEffect::Invisibility => Color::Grey,
        StatusEffect::Confusion => Color::DarkYellow,
        _ => Color::White,
    }
}

/// Get the icon/symbol for a status effect
fn status_icon(effect: &StatusEffect) -> &'static str {
    match effect {
        StatusEffect::Poison => "[P]",       // Poison vial
        StatusEffect::Burn => "[~]",         // Flames
        StatusEffect::Freeze => "[*]",       // Snowflake/ice crystal
        StatusEffect::Bleed => "[%]",        // Blood drops
        StatusEffect::Stun => "[!]",         // Stars/daze
        StatusEffect::Blind => "[.]",        // Closed eyes
        StatusEffect::Haste => "[>]",        // Speed arrows
        StatusEffect::Shield => "[#]",       // Shield/barrier
        StatusEffect::Regeneration => "[+]", // Healing plus
        StatusEffect::Strength => "[^]",     // Power up arrow
        StatusEffect::Weakness => "[v]",     // Power down arrow
        StatusEffect::Invisibility => "[?]", // Fading/ghost
        StatusEffect::Confusion => "[&]",    // Spiral/swirl
        _ => "[.]",
    }
}

/// Get the background color for status effect highlighting
fn status_bg_color(effect: &StatusEffect) -> Option<Color> {
    if effect.is_harmful() {
        Some(Color::Rgb { r: 60, g: 0, b: 0 })  // Dark red background for harmful
    } else if effect.is_beneficial() {
        Some(Color::Rgb { r: 0, g: 40, b: 0 })  // Dark green background for beneficial
    } else {
        None
    }
}

/// Get the urgency indicator based on remaining duration
fn duration_urgency(duration: u32) -> (&'static str, Color) {
    match duration {
        1 => ("!", Color::Red),           // About to expire - critical
        2 => (":", Color::Yellow),        // Low duration - warning
        3..=5 => (".", Color::White),     // Medium duration
        _ => (" ", Color::Green),         // High duration - stable
    }
}

/// Get a visual timer bar for the duration
fn duration_bar(duration: u32, max_expected: u32) -> String {
    let bar_width = 3;
    let filled = ((duration as f32 / max_expected as f32) * bar_width as f32).ceil() as usize;
    let filled = filled.min(bar_width);
    let empty = bar_width.saturating_sub(filled);
    format!("{}{}", "|".repeat(filled), ".".repeat(empty))
}

/// Determine if an effect should pulse/blink (for low duration warning)
fn should_pulse_effect(duration: u32, turn_count: u64) -> bool {
    duration <= 2 && (turn_count % 2 == 0)
}

/// Get abbreviated name for status effect display
fn status_abbrev(effect: &StatusEffect) -> &'static str {
    match effect {
        StatusEffect::Poison => "PSN",
        StatusEffect::Burn => "BRN",
        StatusEffect::Freeze => "FRZ",
        StatusEffect::Bleed => "BLD",
        StatusEffect::Stun => "STN",
        StatusEffect::Blind => "BLN",
        StatusEffect::Haste => "HST",
        StatusEffect::Shield => "SHD",
        StatusEffect::Regeneration => "RGN",
        StatusEffect::Strength => "STR",
        StatusEffect::Weakness => "WEK",
        StatusEffect::Invisibility => "INV",
        StatusEffect::Confusion => "CNF",
        _ => "???",
    }
}

/// Render a single status effect with all visual enhancements
fn render_status_effect(
    stdout: &mut std::io::Stdout,
    effect: &StatusEffect,
    duration: u32,
    turn_count: u64,
) -> std::io::Result<()> {
    let color = status_color(effect);
    let icon = status_icon(effect);
    let abbrev = status_abbrev(effect);
    let (urgency_char, urgency_color) = duration_urgency(duration);
    let bar = duration_bar(duration, 10);
    let pulse = should_pulse_effect(duration, turn_count);

    // Apply background color for effect type
    if let Some(bg) = status_bg_color(effect) {
        execute!(stdout, SetBackgroundColor(bg))?;
    }

    // Icon with effect color
    execute!(stdout, SetForegroundColor(color))?;
    if pulse {
        execute!(stdout, SetAttribute(Attribute::Bold))?;
    }
    write!(stdout, "{}", icon)?;

    // Abbreviated name
    write!(stdout, "{}", abbrev)?;

    // Duration timer with urgency color
    execute!(stdout, SetForegroundColor(urgency_color))?;
    write!(stdout, "{}", urgency_char)?;
    write!(stdout, "{}", duration)?;

    // Duration bar
    execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
    write!(stdout, "{}", bar)?;

    // Reset styling
    execute!(stdout, ResetColor, SetAttribute(Attribute::Reset))?;
    write!(stdout, " ")?;

    Ok(())
}

/// Render the complete status effects bar with enhanced visuals
fn render_status_effects_bar(
    stdout: &mut std::io::Stdout,
    status_effects: &std::collections::HashMap<StatusEffect, u32>,
    turn_count: u64,
) -> std::io::Result<()> {
    if status_effects.is_empty() {
        execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
        write!(stdout, "(no active effects)")?;
        execute!(stdout, ResetColor)?;
        return Ok(());
    }

    // Separate harmful and beneficial effects
    let mut harmful: Vec<_> = status_effects.iter()
        .filter(|(e, _)| e.is_harmful())
        .collect();
    let mut beneficial: Vec<_> = status_effects.iter()
        .filter(|(e, _)| e.is_beneficial())
        .collect();

    // Sort by duration (lowest first for urgency)
    harmful.sort_by_key(|(_, d)| *d);
    beneficial.sort_by_key(|(_, d)| *d);

    // Render harmful effects first (more urgent)
    if !harmful.is_empty() {
        execute!(stdout, SetForegroundColor(Color::Red))?;
        write!(stdout, "[-]")?;
        execute!(stdout, ResetColor)?;
        for (effect, duration) in &harmful {
            render_status_effect(stdout, effect, **duration, turn_count)?;
        }
    }

    // Render beneficial effects
    if !beneficial.is_empty() {
        if !harmful.is_empty() {
            write!(stdout, " ")?;
        }
        execute!(stdout, SetForegroundColor(Color::Green))?;
        write!(stdout, "[+]")?;
        execute!(stdout, ResetColor)?;
        for (effect, duration) in &beneficial {
            render_status_effect(stdout, effect, **duration, turn_count)?;
        }
    }

    Ok(())
}

/// Get the terminal glyph for a tile (enhanced with better symbols)
fn tile_glyph(tile: &Tile) -> char {
    match tile {
        Tile::Wall => symbols::WALL,
        Tile::Floor => symbols::FLOOR,
        Tile::Door => symbols::DOOR_CLOSED,
        Tile::OpenDoor => symbols::DOOR_OPEN,
        Tile::StairsDown => symbols::STAIRS_DOWN,
        Tile::StairsUp => symbols::STAIRS_UP,
        Tile::Chest => symbols::CHEST,
        Tile::OpenChest => symbols::CHEST_OPEN,
        Tile::Trap => symbols::TRAP,
        Tile::DisarmedTrap => symbols::TRAP_DISABLED,
        Tile::Shrine => symbols::SHRINE,
        Tile::UsedShrine => symbols::FLOOR,
        Tile::Water => symbols::WATER,
        Tile::Lava => symbols::LAVA,
        Tile::Sand => '.',
        Tile::Grass => '"',
        Tile::Ice => '=',
        Tile::Pillar => 'O',
        Tile::BossGate => '#',
        _ => symbols::FLOOR,
    }
}

/// Get the terminal color for a tile (enhanced with theme colors)
fn tile_color(tile: &Tile) -> Color {
    match tile {
        Tile::Wall => theme::WALL_VISIBLE,
        Tile::Floor => theme::FLOOR_VISIBLE,
        Tile::Door | Tile::OpenDoor => theme::DOOR,
        Tile::StairsDown | Tile::StairsUp => theme::STAIRS,
        Tile::Chest => Color::Rgb { r: 255, g: 215, b: 0 },
        Tile::OpenChest => Color::Rgb { r: 139, g: 119, b: 42 },
        Tile::Trap => Color::Rgb { r: 255, g: 80, b: 80 },
        Tile::DisarmedTrap => Color::Rgb { r: 80, g: 80, b: 80 },
        Tile::Shrine => Color::Rgb { r: 200, g: 100, b: 255 },
        Tile::UsedShrine => Color::Rgb { r: 100, g: 50, b: 130 },
        Tile::Water => Color::Rgb { r: 50, g: 100, b: 200 },
        Tile::Lava => Color::Rgb { r: 255, g: 100, b: 0 },
        Tile::Sand => Color::Rgb { r: 210, g: 180, b: 100 },
        Tile::Grass => Color::Rgb { r: 50, g: 180, b: 50 },
        Tile::Ice => Color::Rgb { r: 150, g: 220, b: 255 },
        _ => theme::FLOOR_VISIBLE,
    }
}

/// Get dimmed tile color for explored but not visible tiles
fn tile_color_dim(tile: &Tile) -> Color {
    match tile {
        Tile::Wall => theme::WALL,
        Tile::Floor => theme::FLOOR,
        Tile::StairsDown | Tile::StairsUp => Color::Rgb { r: 0, g: 100, b: 100 },
        _ => Color::Rgb { r: 50, g: 50, b: 60 },
    }
}

/// Get the terminal glyph for an enemy
fn enemy_glyph(kind: &EnemyKind) -> char {
    match kind {
        // Tier 1
        EnemyKind::Rat | EnemyKind::GiantRat => 'r',
        EnemyKind::Bat => 'b',
        EnemyKind::Spider | EnemyKind::GiantSpider => 's',
        EnemyKind::Goblin | EnemyKind::Hobgoblin => 'g',
        EnemyKind::Skeleton => 'z',
        EnemyKind::Kobold => 'k',
        EnemyKind::CaveCrawler => 'c',

        // Tier 2
        EnemyKind::Orc => 'o',
        EnemyKind::Troll | EnemyKind::ForestTroll => 'T',
        EnemyKind::CaveOgre => 'O',
        EnemyKind::Slime | EnemyKind::MagmaSlime => 'j',
        EnemyKind::CaveBear => 'B',
        EnemyKind::Mushroom => 'm',
        EnemyKind::RockElemental => 'E',

        // Tier 3: Crypt
        EnemyKind::Zombie => 'Z',
        EnemyKind::Ghost => 'G',
        EnemyKind::Wraith | EnemyKind::IceWraith | EnemyKind::CinderWraith => 'W',
        EnemyKind::Vampire => 'V',
        EnemyKind::Mummy | EnemyKind::MummyLord => 'M',
        EnemyKind::Ghoul => 'u',
        EnemyKind::Banshee => 'h',
        EnemyKind::DeathKnight | EnemyKind::FrozenKnight => 'K',
        EnemyKind::BoneGolem => 'G',

        // Tier 4: Forest
        EnemyKind::Wolf | EnemyKind::DireWolf | EnemyKind::FrostWolf => 'w',
        EnemyKind::TreeEnt => 't',
        EnemyKind::Druid => 'd',
        EnemyKind::WildBoar => 'p',
        EnemyKind::GiantWasp => 'i',
        EnemyKind::VenomousVine => 'v',
        EnemyKind::ForestSpirit => 'f',

        // Tier 5: Ice
        EnemyKind::IceElemental | EnemyKind::FireElemental => 'E',
        EnemyKind::FrostGiant => 'H',
        EnemyKind::YetiWarrior => 'Y',
        EnemyKind::IceSpider => 'S',
        EnemyKind::Wendigo => 'W',

        // Tier 6: Volcanic
        EnemyKind::LavaGolem => 'G',
        EnemyKind::Hellhound => 'h',
        EnemyKind::FireDrake => 'D',
        EnemyKind::Salamander => 'l',
        EnemyKind::InfernalImp => 'i',

        // Tier 7: Ancient Ruins
        EnemyKind::Golem | EnemyKind::AncientGuardian => 'G',
        EnemyKind::Sphinx => 'X',
        EnemyKind::Lich => 'L',
        EnemyKind::Gargoyle => 'y',
        EnemyKind::CursedStatue => 'C',
        EnemyKind::ShadowAssassin => 'a',

        // Tier 8: Demon Realm
        EnemyKind::Demon | EnemyKind::DemonLord | EnemyKind::ShadowDemon => 'd',
        EnemyKind::Succubus => 'S',
        EnemyKind::Balrog => 'B',
        EnemyKind::PitFiend => 'P',
        EnemyKind::AbyssalHorror => 'A',
        EnemyKind::DoomGuard => 'D',

        // Bosses
        EnemyKind::BossGoblinKing => 'G',
        EnemyKind::BossOrcWarlord => 'O',
        EnemyKind::BossVampireLord => 'L',
        EnemyKind::BossForestGuardian => 'F',
        EnemyKind::BossIceDragon => 'W',
        EnemyKind::BossDemonKing => 'D',

        // Mini-Bosses
        EnemyKind::GoblinChampion => 'G',
        EnemyKind::OrcBerserker => 'O',
        EnemyKind::VampireElite => 'V',
        EnemyKind::AncientWyrm => 'W',
        EnemyKind::FrostLord => 'F',
        EnemyKind::InfernalLord => 'I',
    }
}

/// Get the terminal color for an enemy
fn enemy_color(kind: &EnemyKind) -> Color {
    match kind {
        // Tier 1
        EnemyKind::Rat | EnemyKind::GiantRat => Color::DarkYellow,
        EnemyKind::Bat => Color::DarkGrey,
        EnemyKind::Spider | EnemyKind::GiantSpider => Color::DarkGrey,
        EnemyKind::Goblin | EnemyKind::Hobgoblin => Color::Green,
        EnemyKind::Skeleton => Color::White,
        EnemyKind::Kobold => Color::Yellow,
        EnemyKind::CaveCrawler => Color::Grey,

        // Tier 2
        EnemyKind::Orc => Color::Green,
        EnemyKind::Troll | EnemyKind::ForestTroll => Color::DarkGreen,
        EnemyKind::CaveOgre => Color::Yellow,
        EnemyKind::Slime => Color::Green,
        EnemyKind::CaveBear => Color::DarkYellow,
        EnemyKind::Mushroom => Color::Magenta,
        EnemyKind::RockElemental => Color::Grey,

        // Tier 3: Crypt
        EnemyKind::Zombie => Color::DarkGreen,
        EnemyKind::Ghost => Color::White,
        EnemyKind::Wraith => Color::DarkGrey,
        EnemyKind::Vampire => Color::DarkRed,
        EnemyKind::Mummy => Color::Yellow,
        EnemyKind::Ghoul => Color::DarkGreen,
        EnemyKind::Banshee => Color::White,
        EnemyKind::DeathKnight => Color::DarkGrey,
        EnemyKind::BoneGolem => Color::White,

        // Tier 4: Forest
        EnemyKind::Wolf | EnemyKind::DireWolf => Color::Grey,
        EnemyKind::TreeEnt => Color::DarkGreen,
        EnemyKind::Druid => Color::Green,
        EnemyKind::WildBoar => Color::DarkYellow,
        EnemyKind::GiantWasp => Color::Yellow,
        EnemyKind::VenomousVine => Color::Green,
        EnemyKind::ForestSpirit => Color::Cyan,

        // Tier 5: Ice
        EnemyKind::IceElemental => Color::Cyan,
        EnemyKind::FrostGiant => Color::Blue,
        EnemyKind::YetiWarrior => Color::White,
        EnemyKind::IceWraith => Color::Cyan,
        EnemyKind::FrostWolf => Color::Cyan,
        EnemyKind::IceSpider => Color::Blue,
        EnemyKind::FrozenKnight => Color::Cyan,
        EnemyKind::Wendigo => Color::White,

        // Tier 6: Volcanic
        EnemyKind::FireElemental => Color::Red,
        EnemyKind::LavaGolem => Color::Red,
        EnemyKind::Hellhound => Color::DarkRed,
        EnemyKind::FireDrake => Color::Red,
        EnemyKind::MagmaSlime => Color::Red,
        EnemyKind::Salamander => Color::Red,
        EnemyKind::CinderWraith => Color::DarkRed,
        EnemyKind::InfernalImp => Color::Red,

        // Tier 7: Ancient Ruins
        EnemyKind::Golem => Color::Grey,
        EnemyKind::AncientGuardian => Color::Yellow,
        EnemyKind::Sphinx => Color::Yellow,
        EnemyKind::Lich => Color::Magenta,
        EnemyKind::Gargoyle => Color::Grey,
        EnemyKind::MummyLord => Color::Yellow,
        EnemyKind::CursedStatue => Color::DarkGrey,
        EnemyKind::ShadowAssassin => Color::DarkGrey,

        // Tier 8: Demon Realm
        EnemyKind::Demon => Color::Red,
        EnemyKind::DemonLord => Color::DarkRed,
        EnemyKind::Succubus => Color::Magenta,
        EnemyKind::Balrog => Color::Red,
        EnemyKind::PitFiend => Color::DarkRed,
        EnemyKind::ShadowDemon => Color::DarkGrey,
        EnemyKind::AbyssalHorror => Color::DarkMagenta,
        EnemyKind::DoomGuard => Color::DarkRed,

        // Bosses
        EnemyKind::BossGoblinKing => Color::Green,
        EnemyKind::BossOrcWarlord => Color::Green,
        EnemyKind::BossVampireLord => Color::Magenta,
        EnemyKind::BossForestGuardian => Color::Green,
        EnemyKind::BossIceDragon => Color::Cyan,
        EnemyKind::BossDemonKing => Color::Red,

        // Mini-Bosses
        EnemyKind::GoblinChampion => Color::Green,
        EnemyKind::OrcBerserker => Color::Green,
        EnemyKind::VampireElite => Color::DarkRed,
        EnemyKind::AncientWyrm => Color::Yellow,
        EnemyKind::FrostLord => Color::Cyan,
        EnemyKind::InfernalLord => Color::Red,
    }
}

/// Get the terminal glyph for an item
fn item_glyph(kind: &ItemKind) -> char {
    match kind {
        // Potions
        ItemKind::HealthPotion | ItemKind::ManaPotion | ItemKind::StrengthPotion
        | ItemKind::SpeedPotion | ItemKind::InvisibilityPotion | ItemKind::FullRestorePotion
        | ItemKind::UltimatePowerPotion | ItemKind::PoisonResistPotion
        | ItemKind::FireResistPotion | ItemKind::IceResistPotion => '!',

        // Scrolls
        ItemKind::ScrollFireball | ItemKind::ScrollIceStorm | ItemKind::ScrollLightning
        | ItemKind::ScrollTeleport | ItemKind::ScrollIdentify | ItemKind::ScrollEnchant
        | ItemKind::ScrollMapping | ItemKind::ScrollSummon => '?',

        // Weapons
        ItemKind::Dagger | ItemKind::ShortSword | ItemKind::LongSword | ItemKind::Greatsword
        | ItemKind::Axe | ItemKind::BattleAxe | ItemKind::Mace | ItemKind::WarHammer
        | ItemKind::Spear | ItemKind::Halberd | ItemKind::Staff | ItemKind::Wand
        | ItemKind::Bow | ItemKind::Crossbow | ItemKind::Scythe | ItemKind::Katana
        | ItemKind::Rapier | ItemKind::Flail | ItemKind::Morningstar | ItemKind::Trident
        | ItemKind::FlameSword | ItemKind::FrostBlade | ItemKind::ThunderAxe | ItemKind::VoidStaff
        | ItemKind::DemonSlayer => ')',

        // Armor
        ItemKind::LeatherArmor | ItemKind::ChainMail | ItemKind::ScaleMail | ItemKind::PlateMail
        | ItemKind::MageRobes | ItemKind::AssassinGarb | ItemKind::DragonArmor | ItemKind::HolyArmor
        | ItemKind::DemonArmor | ItemKind::CrystalArmor | ItemKind::ShadowCloak | ItemKind::TitanPlate => '[',

        // Shields
        ItemKind::Buckler | ItemKind::WoodenShield | ItemKind::IronShield | ItemKind::TowerShield
        | ItemKind::MagicShield | ItemKind::DragonShield | ItemKind::SpikedShield | ItemKind::MirrorShield
        | ItemKind::PhoenixShield | ItemKind::AbyssalShield => ']',

        // Helmets
        ItemKind::LeatherCap | ItemKind::IronHelm | ItemKind::SteelHelm | ItemKind::CrownOfKings
        | ItemKind::WizardHat | ItemKind::DemonSkull | ItemKind::DragonHelm | ItemKind::CrystalCrown
        | ItemKind::HoodOfShadows | ItemKind::HelmOfValor => '^',

        // Gloves
        ItemKind::LeatherGloves | ItemKind::IronGauntlets | ItemKind::GlovesOfPower | ItemKind::ThievesGloves
        | ItemKind::DragonGauntlets | ItemKind::FrostGauntlets | ItemKind::FlameGauntlets | ItemKind::GauntletsOfMight => '(',

        // Boots
        ItemKind::LeatherBoots | ItemKind::IronBoots | ItemKind::BootsOfSpeed | ItemKind::BootsOfLeaping
        | ItemKind::WingedBoots | ItemKind::ShadowBoots | ItemKind::LavaWalkers | ItemKind::BootsOfTheWind => '_',

        // Rings
        ItemKind::RingOfStrength | ItemKind::RingOfProtection | ItemKind::RingOfSpeed
        | ItemKind::RingOfRegeneration | ItemKind::RingOfMana | ItemKind::RingOfFireball
        | ItemKind::RingOfInvisibility | ItemKind::RingOfTheVampire | ItemKind::RingOfLuck
        | ItemKind::RingOfDeath | ItemKind::RingOfFrost | ItemKind::RingOfFlame | ItemKind::RingOfThunder
        | ItemKind::RingOfShadows | ItemKind::RingOfTheAncients => '=',

        // Amulets
        ItemKind::AmuletOfHealth | ItemKind::AmuletOfMana | ItemKind::AmuletOfProtection
        | ItemKind::AmuletOfPower | ItemKind::AmuletOfWisdom | ItemKind::AmuletOfLife
        | ItemKind::AmuletOfDeath | ItemKind::AmuletOfTheGods | ItemKind::AmuletOfDragons
        | ItemKind::AmuletOfChaos | ItemKind::AmuletOfOrder | ItemKind::AmuletOfBalance => '"',

        // Food
        ItemKind::Bread | ItemKind::Apple | ItemKind::Meat | ItemKind::Cheese | ItemKind::Feast
        | ItemKind::DragonFruit | ItemKind::AncientWine | ItemKind::GoldenApple => '%',

        // Misc
        ItemKind::Gold => '$',
        ItemKind::Key => 'k',
        ItemKind::Bomb => '*',
        ItemKind::Torch => '|',
        ItemKind::Compass => 'c',
        ItemKind::TeleportCrystal => 'o',
        ItemKind::SoulGem => 'o',
        ItemKind::AncientRelic => '&',
        ItemKind::DragonScale => '~',
        ItemKind::DemonHeart => 'H',
        // Catch-all for remaining item types
        _ => '*',
    }
}

/// Get the terminal color for a rarity (enhanced with theme colors)
fn rarity_color(rarity: &Rarity) -> Color {
    match rarity {
        Rarity::Common => theme::ITEM_COMMON,
        Rarity::Uncommon => theme::ITEM_UNCOMMON,
        Rarity::Rare => theme::ITEM_RARE,
        Rarity::Epic => theme::ITEM_EPIC,
        Rarity::Legendary => theme::ITEM_LEGENDARY,
        Rarity::Mythic => Color::Rgb { r: 255, g: 50, b: 50 },
    }
}

/// Get display name for a rarity
fn rarity_name(rarity: &Rarity) -> &'static str {
    match rarity {
        Rarity::Common => "Common",
        Rarity::Uncommon => "Uncommon",
        Rarity::Rare => "Rare",
        Rarity::Epic => "Epic",
        Rarity::Legendary => "Legendary",
        Rarity::Mythic => "Mythic",
    }
}

/// Helper to get attack bonus from an ItemKind
fn item_attack_bonus(kind: &ItemKind) -> i32 {
    kind.base_stats().0
}

/// Helper to get defense bonus from an ItemKind
fn item_defense_bonus(kind: &ItemKind) -> i32 {
    kind.base_stats().1
}

/// Helper to get HP bonus from an ItemKind
fn item_hp_bonus(kind: &ItemKind) -> i32 {
    kind.base_stats().2
}

/// Helper to get mana bonus from an ItemKind
fn item_mana_bonus(kind: &ItemKind) -> i32 {
    kind.base_stats().3
}

/// Helper to get heal amount from a consumable ItemKind
fn item_heal_amount(kind: &ItemKind) -> i32 {
    match kind {
        ItemKind::HealthPotion => 30,
        ItemKind::FullRestorePotion => 100,
        _ => 0,
    }
}

/// Helper to get mana restore amount from a consumable ItemKind
fn item_mana_restore(kind: &ItemKind) -> i32 {
    match kind {
        ItemKind::ManaPotion => 30,
        _ => 0,
    }
}

/// Convert color index to terminal color
fn color_from_index(index: u8) -> Color {
    match index {
        1 => Color::Red,
        2 => Color::Green,
        3 => Color::Yellow,
        4 => Color::Blue,
        5 => Color::Magenta,
        6 => Color::Cyan,
        7 => Color::White,
        8 => Color::DarkGrey,
        9 => Color::Yellow,  // Welcome message color
        10 => Color::Green,  // XP message color
        11 => Color::Cyan,   // Help message color
        _ => Color::White,
    }
}

// ============================================================================
// ============================================================================
// MINIMAP CONSTANTS AND RENDERING
// ============================================================================

/// Minimap dimensions
const MINIMAP_WIDTH: usize = 20;
const MINIMAP_HEIGHT: usize = 11;
/// Minimap position (top-right corner)
const MINIMAP_X: u16 = 78;
const MINIMAP_Y: u16 = 1;

/// Render a minimap in the top-right corner showing explored areas,
/// player position, stairs, and enemies
fn render_minimap(state: &GameState) -> std::io::Result<()> {
    let mut stdout = stdout();

    // Calculate scale factors
    let scale_x = MAP_WIDTH as f32 / (MINIMAP_WIDTH - 2) as f32;
    let scale_y = MAP_HEIGHT as f32 / (MINIMAP_HEIGHT - 2) as f32;

    // Draw minimap title
    execute!(stdout, MoveTo(MINIMAP_X, MINIMAP_Y - 1))?;
    execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
    write!(stdout, "[MAP]")?;

    // Top border
    execute!(stdout, MoveTo(MINIMAP_X, MINIMAP_Y))?;
    execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
    write!(stdout, "+")?;
    for _ in 0..(MINIMAP_WIDTH - 2) {
        write!(stdout, "-")?;
    }
    write!(stdout, "+")?;

    // Build minimap data - we'll sample the map at scaled positions
    // For each minimap cell, we check a region of the game map
    for my in 0..(MINIMAP_HEIGHT - 2) {
        execute!(stdout, MoveTo(MINIMAP_X, MINIMAP_Y + 1 + my as u16))?;
        execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
        write!(stdout, "|")?;

        for mx in 0..(MINIMAP_WIDTH - 2) {
            // Calculate the region of the game map this minimap cell represents
            let map_x_start = (mx as f32 * scale_x) as usize;
            let map_y_start = (my as f32 * scale_y) as usize;
            let map_x_end = ((mx + 1) as f32 * scale_x) as usize;
            let map_y_end = ((my + 1) as f32 * scale_y) as usize;

            // Check if player is in this region
            let player_here = state.player.x >= map_x_start
                && state.player.x < map_x_end
                && state.player.y >= map_y_start
                && state.player.y < map_y_end;

            if player_here {
                execute!(stdout, SetForegroundColor(Color::Yellow))?;
                write!(stdout, "@")?;
                continue;
            }

            // Check for enemies in this region (only visible ones)
            let enemy_here = state.enemies.iter().any(|e| {
                e.is_alive()
                    && e.x >= map_x_start
                    && e.x < map_x_end
                    && e.y >= map_y_start
                    && e.y < map_y_end
                    && state.map.visible[e.y][e.x]
            });

            if enemy_here {
                execute!(stdout, SetForegroundColor(Color::Red))?;
                write!(stdout, "!")?;
                continue;
            }

            // Check for stairs in this region (explored areas only)
            let mut stairs_char: Option<char> = None;
            for y in map_y_start..map_y_end.min(MAP_HEIGHT) {
                for x in map_x_start..map_x_end.min(MAP_WIDTH) {
                    if state.map.explored[y][x] {
                        match state.map.tiles[y][x] {
                            Tile::StairsDown => { stairs_char = Some('>'); break; }
                            Tile::StairsUp => { stairs_char = Some('<'); break; }
                            _ => {}
                        }
                    }
                }
                if stairs_char.is_some() { break; }
            }

            if let Some(c) = stairs_char {
                execute!(stdout, SetForegroundColor(Color::Cyan))?;
                write!(stdout, "{}", c)?;
                continue;
            }

            // Check if any part of this region is explored
            let mut has_explored = false;
            let mut has_visible = false;
            let mut has_floor = false;

            for y in map_y_start..map_y_end.min(MAP_HEIGHT) {
                for x in map_x_start..map_x_end.min(MAP_WIDTH) {
                    if state.map.visible[y][x] {
                        has_visible = true;
                        has_explored = true;
                        if state.map.tiles[y][x].walkable() {
                            has_floor = true;
                        }
                    } else if state.map.explored[y][x] {
                        has_explored = true;
                        if state.map.tiles[y][x].walkable() {
                            has_floor = true;
                        }
                    }
                }
            }

            if has_visible && has_floor {
                execute!(stdout, SetForegroundColor(Color::White))?;
                write!(stdout, ".")?;
            } else if has_explored && has_floor {
                execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
                write!(stdout, ".")?;
            } else if has_explored {
                execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
                write!(stdout, "#")?;
            } else {
                write!(stdout, " ")?;
            }
        }

        execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
        write!(stdout, "|")?;
    }

    // Bottom border
    execute!(stdout, MoveTo(MINIMAP_X, MINIMAP_Y + MINIMAP_HEIGHT as u16 - 1))?;
    execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
    write!(stdout, "+")?;
    for _ in 0..(MINIMAP_WIDTH - 2) {
        write!(stdout, "-")?;
    }
    write!(stdout, "+")?;

    execute!(stdout, ResetColor)?;
    Ok(())
}

// RENDERING FUNCTIONS
// ============================================================================

fn render(state: &GameState) -> std::io::Result<()> {
    render_full(state, None, None, None)
}

fn render_with_movement(state: &GameState, movement: Option<&MovementState>) -> std::io::Result<()> {
    render_full(state, movement, None, None)
}

fn render_with_targeting(state: &GameState, movement: Option<&MovementState>, targeting: Option<&TargetingState>) -> std::io::Result<()> {
    render_full(state, movement, targeting, None)
}

fn render_with_message_log(state: &GameState, movement: Option<&MovementState>, targeting: Option<&TargetingState>, msg_log: Option<&MessageLog>) -> std::io::Result<()> {
    render_full(state, movement, targeting, msg_log)
}

/// Render the enhanced message log with timestamps, categories, and scroll indicators
fn render_message_log(stdout: &mut std::io::Stdout, msg_log: &MessageLog, stats_y: u16) -> std::io::Result<()> {
    // Message log header with filter and scroll info
    execute!(stdout, MoveTo(0, stats_y + 2), Clear(ClearType::CurrentLine))?;
    execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
    write!(stdout, "--- Messages ")?;

    // Filter indicator
    execute!(stdout, SetForegroundColor(msg_log.get_filter().label_color()))?;
    write!(stdout, "[{}]", msg_log.get_filter().name())?;
    execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
    write!(stdout, " {} ", msg_log.scroll_info())?;

    // Scroll arrows
    if msg_log.can_scroll_up() {
        execute!(stdout, SetForegroundColor(Color::Cyan))?;
        write!(stdout, "[PgUp]")?;
    }
    if msg_log.can_scroll_down() {
        execute!(stdout, SetForegroundColor(Color::Cyan))?;
        write!(stdout, "[PgDn]")?;
    }
    execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
    write!(stdout, " [m:Filter] ---")?;
    execute!(stdout, ResetColor)?;

    // Render visible messages
    let visible = msg_log.visible_messages();
    for (i, msg) in visible.iter().enumerate() {
        execute!(stdout, MoveTo(0, stats_y + 3 + i as u16), Clear(ClearType::CurrentLine))?;

        // Timestamp
        if msg_log.show_timestamps {
            execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
            write!(stdout, "{} ", msg.formatted_time())?;
        }

        // Category tag
        if msg_log.show_categories {
            execute!(stdout, SetForegroundColor(msg.category.label_color()))?;
            write!(stdout, "{} ", msg.category.tag())?;
        }

        // Message text
        execute!(stdout, SetForegroundColor(color_from_index(msg.color_index)))?;
        // Truncate long messages
        let max_len = if msg_log.show_timestamps && msg_log.show_categories { 85 }
                      else if msg_log.show_timestamps || msg_log.show_categories { 90 }
                      else { 100 };
        let text = if msg.text.len() > max_len { format!("{}...", &msg.text[..max_len-3]) } else { msg.text.clone() };
        write!(stdout, "{}", text)?;
        execute!(stdout, ResetColor)?;
    }

    // Clear remaining lines
    for i in visible.len()..6 {
        execute!(stdout, MoveTo(0, stats_y + 3 + i as u16), Clear(ClearType::CurrentLine))?;
    }

    Ok(())
}

fn render_full(state: &GameState, movement: Option<&MovementState>, targeting: Option<&TargetingState>, msg_log: Option<&MessageLog>) -> std::io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, MoveTo(0, 0))?;

    // Check if we should show target indicator (blinking)
    let show_target_indicator = targeting
        .map(|t| t.active && t.should_show_indicator())
        .unwrap_or(false);

    // Render map
    for y in 0..MAP_HEIGHT.min(43) {
        execute!(stdout, MoveTo(0, y as u16))?;

        for x in 0..MAP_WIDTH.min(100) {
            // Player - with direction indicator for recent movement
            if state.player.x == x && state.player.y == y {
                // Check if we should show a directional indicator
                let show_direction = movement
                    .and_then(|m| m.get_direction_indicator())
                    .filter(|_| movement.map_or(false, |m| m.last_move_time.elapsed().as_millis() < 150));

                if let Some((arrow, _name)) = show_direction {
                    // Brief directional flash after movement
                    execute!(
                        stdout,
                        SetForegroundColor(Color::Cyan),
                        Print(arrow),
                        ResetColor
                    )?;
                } else {
                    // Normal player display
                    let player_color = if movement.map_or(false, |m| m.is_running) {
                        Color::Green // Running indicator
                    } else if targeting.map(|t| t.active).unwrap_or(false) {
                        Color::Magenta // Targeting mode indicator
                    } else {
                        Color::Yellow
                    };
                    execute!(
                        stdout,
                        SetForegroundColor(player_color),
                        Print('@'),
                        ResetColor
                    )?;
                }
            }
            // Enemies - with targeting highlight
            else if let Some(enemy) = state.enemies.iter()
                .find(|e| e.x == x && e.y == y && e.is_alive() && state.map.visible[y][x])
            {
                let is_targeted = targeting.map(|t| t.is_targeted(x, y)).unwrap_or(false);

                if is_targeted && show_target_indicator {
                    // Highlighted target - show with brackets and background
                    let is_locked = targeting.map(|t| t.locked).unwrap_or(false);
                    let bg_color = if is_locked {
                        Color::DarkRed
                    } else {
                        Color::DarkMagenta
                    };
                    execute!(
                        stdout,
                        SetBackgroundColor(bg_color),
                        SetForegroundColor(Color::White),
                        SetAttribute(Attribute::Bold),
                        Print(enemy_glyph(&enemy.kind)),
                        SetAttribute(Attribute::Reset),
                        ResetColor
                    )?;
                } else {
                    // Normal enemy display
                    let color = if enemy.kind.is_boss() {
                        Color::Red
                    } else {
                        enemy_color(&enemy.kind)
                    };
                    execute!(
                        stdout,
                        SetForegroundColor(color),
                        Print(enemy_glyph(&enemy.kind)),
                        ResetColor
                    )?;
                }
            }
            // Items
            else if let Some(item) = state.items.iter()
                .find(|i| i.x == x && i.y == y && state.map.visible[y][x])
            {
                execute!(
                    stdout,
                    SetForegroundColor(rarity_color(&item.rarity)),
                    Print(item_glyph(&item.kind)),
                    ResetColor
                )?;
            }
            // Visible tiles
            else if state.map.visible[y][x] {
                let tile = state.map.tiles[y][x];
                let bg = match tile {
                    Tile::Lava => Some(Color::DarkRed),
                    Tile::Water => Some(Color::DarkBlue),
                    Tile::Sand => Some(Color::DarkYellow),
                    _ => None,
                };
                if let Some(bg_color) = bg {
                    execute!(stdout, SetBackgroundColor(bg_color))?;
                }
                execute!(
                    stdout,
                    SetForegroundColor(tile_color(&tile)),
                    Print(tile_glyph(&tile)),
                    ResetColor
                )?;
            }
            // Explored tiles
            else if state.map.explored[y][x] {
                let tile = state.map.tiles[y][x];
                execute!(
                    stdout,
                    SetForegroundColor(Color::DarkGrey),
                    Print(tile_glyph(&tile)),
                    ResetColor
                )?;
            }
            // Unexplored
            else {
                execute!(stdout, Print(' '))?;
            }
        }
    }

    // Render minimap in top-right corner
    render_minimap(state)?;

    // Stats bar (line 43)
    let stats_y = 43u16;
    execute!(
        stdout,
        MoveTo(0, stats_y),
        Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::White)
    )?;

    // HP bar
    let hp_pct = (state.player.hp as f32 / state.player.total_max_hp() as f32 * 10.0) as usize;
    let hp_color = if hp_pct <= 2 { Color::Red } else if hp_pct <= 5 { Color::Yellow } else { Color::Green };
    execute!(stdout, SetForegroundColor(hp_color))?;
    write!(stdout, "HP:{}/{}", state.player.hp, state.player.total_max_hp())?;

    // Mana bar
    execute!(stdout, SetForegroundColor(Color::Blue))?;
    write!(stdout, " MP:{}/{}", state.player.mana, state.player.total_max_mana())?;

    // Stats
    execute!(stdout, SetForegroundColor(Color::White))?;
    write!(
        stdout,
        " ATK:{} DEF:{} LV:{} XP:{}/{} Gold:{} Keys:{} ",
        state.player.total_attack(),
        state.player.total_defense(),
        state.player.level,
        state.player.xp,
        state.player.xp_to_level,
        state.player.gold,
        state.player.keys
    )?;

    // Hunger
    let hunger_color = if state.player.hunger < 20 { Color::Red } else if state.player.hunger < 50 { Color::Yellow } else { Color::Green };
    execute!(stdout, SetForegroundColor(hunger_color))?;
    write!(stdout, "Food:{}", state.player.hunger)?;

    // Dungeon level
    execute!(stdout, SetForegroundColor(Color::Cyan))?;
    write!(stdout, " Floor:{}/30", state.dungeon_level)?;

    // Current skill
    if let Some(skill) = state.player.current_skill() {
        execute!(stdout, SetForegroundColor(Color::Magenta))?;
        write!(stdout, " [{}]", skill.name())?;
    }

    execute!(stdout, ResetColor)?;

    // Status effects and movement indicator (line 44)
    execute!(
        stdout,
        MoveTo(0, stats_y + 1),
        Clear(ClearType::CurrentLine)
    )?;

    // Show running indicator if applicable
    if let Some(mv) = movement {
        if mv.is_running {
            execute!(stdout, SetForegroundColor(Color::Green))?;
            if let Some(dir) = mv.run_direction {
                write!(stdout, "[RUNNING {}] ", dir.name())?;
            } else {
                write!(stdout, "[RUNNING] ")?;
            }
            execute!(stdout, ResetColor)?;
        } else if let Some((arrow, name)) = mv.get_direction_indicator() {
            execute!(stdout, SetForegroundColor(Color::Cyan))?;
            write!(stdout, "[{} {}] ", arrow, name)?;
            execute!(stdout, ResetColor)?;
        }
    }

    // Enhanced status effects display
    write!(stdout, "Effects: ")?;
    render_status_effects_bar(&mut stdout, &state.player.status_effects, state.turn_count as u64)?;

    // Messages (lines 45-50)
    for (i, msg) in state.messages.iter().enumerate() {
        execute!(
            stdout,
            MoveTo(0, stats_y + 2 + i as u16),
            Clear(ClearType::CurrentLine),
            SetForegroundColor(color_from_index(msg.color_index)),
            Print(&msg.text),
            ResetColor
        )?;
    }

    // Clear remaining message lines
    for i in state.messages.len()..6 {
        execute!(
            stdout,
            MoveTo(0, stats_y + 2 + i as u16),
            Clear(ClearType::CurrentLine)
        )?;
    }

    // Controls hint - different when targeting
    execute!(
        stdout,
        MoveTo(0, stats_y + 8),
        Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::DarkGrey)
    )?;

    if targeting.map(|t| t.active).unwrap_or(false) {
        write!(stdout, "[Tab/Shift+Tab:Cycle] [t:Lock] [f:Attack] [Space:Skill] [Esc:Cancel] ")?;
        let count = targeting.map(|t| t.target_count()).unwrap_or(0);
        let idx = targeting.map(|t| t.current_index + 1).unwrap_or(0);
        execute!(stdout, SetForegroundColor(Color::Cyan))?;
        write!(stdout, "Target {}/{}", idx, count)?;
    } else {
        write!(stdout, "[WASD/Arrows:Move] [Tab:Target] [Space:Skill] [I:Inv] [?:Help]")?;
    }
    execute!(stdout, ResetColor)?;

    // Target info panel (on right side of screen when targeting is active)
    if let Some(ts) = targeting {
        if ts.active {
            render_target_info(&mut stdout, ts)?;
        }
    }

    // Inventory screen
    if state.show_inventory {
        render_inventory(state)?;
    }

    // Help screen
    if state.show_help {
        render_help(state)?;
    }

    stdout.flush()?;
    Ok(())
}

/// Render the target information panel
fn render_target_info(stdout: &mut std::io::Stdout, targeting: &TargetingState) -> std::io::Result<()> {
    if let Some(target) = targeting.current_target() {
        let panel_x = 70u16; // Right side of screen
        let panel_y = 2u16;
        let panel_width = 28;

        // Panel border and background
        execute!(stdout, SetForegroundColor(Color::Cyan))?;

        // Top border
        execute!(stdout, MoveTo(panel_x, panel_y))?;
        write!(stdout, "+{}+", "-".repeat(panel_width - 2))?;

        // Title
        execute!(stdout, MoveTo(panel_x, panel_y + 1))?;
        write!(stdout, "|")?;
        execute!(stdout, SetForegroundColor(Color::Yellow), SetAttribute(Attribute::Bold))?;
        let title = if targeting.locked { " [LOCKED] TARGET " } else { " TARGET INFO " };
        write!(stdout, "{:^width$}", title, width = panel_width - 2)?;
        execute!(stdout, SetAttribute(Attribute::Reset), SetForegroundColor(Color::Cyan))?;
        write!(stdout, "|")?;

        // Separator
        execute!(stdout, MoveTo(panel_x, panel_y + 2))?;
        write!(stdout, "+{}+", "-".repeat(panel_width - 2))?;

        // Enemy name
        execute!(stdout, MoveTo(panel_x, panel_y + 3))?;
        write!(stdout, "|")?;
        let name_color = if target.is_boss { Color::Red } else { Color::White };
        execute!(stdout, SetForegroundColor(name_color), SetAttribute(Attribute::Bold))?;
        let display_name = if target.is_boss {
            format!("** {} **", target.name)
        } else {
            target.name.clone()
        };
        write!(stdout, " {:<width$}", display_name, width = panel_width - 3)?;
        execute!(stdout, SetAttribute(Attribute::Reset), SetForegroundColor(Color::Cyan))?;
        write!(stdout, "|")?;

        // HP bar
        execute!(stdout, MoveTo(panel_x, panel_y + 4))?;
        write!(stdout, "|")?;
        let hp_pct = (target.hp as f32 / target.max_hp as f32 * 100.0) as i32;
        let hp_color = if hp_pct <= 25 {
            Color::Red
        } else if hp_pct <= 50 {
            Color::Yellow
        } else {
            Color::Green
        };
        execute!(stdout, SetForegroundColor(hp_color))?;

        // Visual HP bar
        let bar_width = 12;
        let filled = ((target.hp as f32 / target.max_hp as f32) * bar_width as f32) as usize;
        let empty = bar_width - filled;
        write!(stdout, " HP: [")?;
        for _ in 0..filled {
            write!(stdout, "#")?;
        }
        execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
        for _ in 0..empty {
            write!(stdout, "-")?;
        }
        execute!(stdout, SetForegroundColor(hp_color))?;
        write!(stdout, "]")?;

        // HP numbers
        execute!(stdout, SetForegroundColor(Color::White))?;
        write!(stdout, " {:>3}", target.hp)?;
        execute!(stdout, SetForegroundColor(Color::Cyan))?;
        write!(stdout, " |")?;

        // Stats line
        execute!(stdout, MoveTo(panel_x, panel_y + 5))?;
        write!(stdout, "|")?;
        execute!(stdout, SetForegroundColor(Color::Red))?;
        write!(stdout, " ATK:{:<3}", target.attack)?;
        execute!(stdout, SetForegroundColor(Color::Blue))?;
        write!(stdout, " DEF:{:<3}", target.defense)?;

        // Distance
        let dist = (target.distance_sq as f32).sqrt() as i32;
        execute!(stdout, SetForegroundColor(Color::Grey))?;
        write!(stdout, " Dist:{:<2}", dist)?;
        execute!(stdout, SetForegroundColor(Color::Cyan))?;
        write!(stdout, "  |")?;

        // Status effects
        execute!(stdout, MoveTo(panel_x, panel_y + 6))?;
        write!(stdout, "|")?;
        if !target.status_effects.is_empty() {
            execute!(stdout, SetForegroundColor(Color::Magenta))?;
            write!(stdout, " ")?;
            for (effect, dur) in target.status_effects.iter().take(3) {
                let abbrev = match effect.as_str() {
                    "Poison" => "PSN",
                    "Burn" => "BRN",
                    "Freeze" => "FRZ",
                    "Stun" => "STN",
                    "Bleed" => "BLD",
                    _ => &effect[..3.min(effect.len())],
                };
                write!(stdout, "{}:{} ", abbrev, dur)?;
            }
            // Pad remaining space
            let effects_len = target.status_effects.iter()
                .take(3)
                .map(|(e, d)| {
                    let abbrev_len = match e.as_str() {
                        "Poison" | "Freeze" => 3,
                        "Burn" | "Stun" | "Bleed" => 3,
                        _ => 3.min(e.len()),
                    };
                    abbrev_len + 1 + d.to_string().len() + 1
                })
                .sum::<usize>();
            let remaining = (panel_width - 3).saturating_sub(effects_len + 1);
            write!(stdout, "{:width$}", "", width = remaining)?;
        } else {
            execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
            write!(stdout, " (no effects){:width$}", "", width = panel_width - 15)?;
        }
        execute!(stdout, SetForegroundColor(Color::Cyan))?;
        write!(stdout, "|")?;

        // Bottom border
        execute!(stdout, MoveTo(panel_x, panel_y + 7))?;
        write!(stdout, "+{}+", "-".repeat(panel_width - 2))?;

        // Action hints
        execute!(stdout, MoveTo(panel_x, panel_y + 8))?;
        execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
        let dist = (target.distance_sq as f32).sqrt();
        if dist < 1.5 {
            write!(stdout, " [f] Melee attack")?;
        } else {
            write!(stdout, " Move closer to attack")?;
        }

        execute!(stdout, ResetColor)?;
    }

    Ok(())
}

// ============================================================================
// ENHANCED INVENTORY UI
// ============================================================================

/// Sorting options for inventory items
#[derive(Clone, Copy, PartialEq)]
enum InventorySortMode {
    Default,
    ByType,
    ByRarity,
    ByName,
}

impl InventorySortMode {
    fn name(&self) -> &'static str {
        match self {
            Self::Default => "Default",
            Self::ByType => "Type",
            Self::ByRarity => "Rarity",
            Self::ByName => "Name",
        }
    }

    fn next(&self) -> Self {
        match self {
            Self::Default => Self::ByType,
            Self::ByType => Self::ByRarity,
            Self::ByRarity => Self::ByName,
            Self::ByName => Self::Default,
        }
    }
}

/// State for the enhanced inventory UI
struct InventoryUIState {
    selected_index: usize,
    sort_mode: InventorySortMode,
    scroll_offset: usize,
}

impl InventoryUIState {
    fn new() -> Self {
        Self {
            selected_index: 0,
            sort_mode: InventorySortMode::Default,
            scroll_offset: 0,
        }
    }

    fn move_selection(&mut self, delta: i32, max_items: usize) {
        if max_items == 0 {
            self.selected_index = 0;
            return;
        }
        let new_index = (self.selected_index as i32 + delta).rem_euclid(max_items as i32) as usize;
        self.selected_index = new_index;
        let visible_items = 8;
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        } else if self.selected_index >= self.scroll_offset + visible_items {
            self.scroll_offset = self.selected_index - visible_items + 1;
        }
    }

    fn cycle_sort(&mut self) {
        self.sort_mode = self.sort_mode.next();
        self.selected_index = 0;
        self.scroll_offset = 0;
    }
}

thread_local! {
    static INVENTORY_UI: std::cell::RefCell<InventoryUIState> = std::cell::RefCell::new(InventoryUIState::new());
}

fn item_type_category(kind: &ItemKind) -> u8 {
    if let Some(slot) = kind.equip_slot() {
        match slot {
            EquipSlot::Weapon => 0,
            EquipSlot::Armor => 1,
            EquipSlot::Shield => 2,
            EquipSlot::Helmet => 3,
            EquipSlot::Gloves => 4,
            EquipSlot::Boots => 5,
            EquipSlot::Ring1 | EquipSlot::Ring2 => 6,
            EquipSlot::Amulet => 7,
        }
    } else if kind.is_consumable() {
        if kind.is_food() { 9 } else { 8 }
    } else {
        10
    }
}

fn get_sorted_indices(inventory: &[Item], sort_mode: InventorySortMode) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..inventory.len()).collect();
    match sort_mode {
        InventorySortMode::Default => {}
        InventorySortMode::ByType => {
            indices.sort_by(|&a, &b| {
                let cat_a = item_type_category(&inventory[a].kind);
                let cat_b = item_type_category(&inventory[b].kind);
                cat_a.cmp(&cat_b).then_with(|| inventory[a].kind.name().cmp(inventory[b].kind.name()))
            });
        }
        InventorySortMode::ByRarity => {
            indices.sort_by(|&a, &b| {
                inventory[b].rarity.partial_cmp(&inventory[a].rarity)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| inventory[a].kind.name().cmp(inventory[b].kind.name()))
            });
        }
        InventorySortMode::ByName => {
            indices.sort_by(|&a, &b| {
                inventory[a].kind.name().cmp(inventory[b].kind.name())
            });
        }
    }
    indices
}

fn item_type_indicator(kind: &ItemKind) -> &'static str {
    if let Some(slot) = kind.equip_slot() {
        match slot {
            EquipSlot::Weapon => "[WPN]",
            EquipSlot::Armor => "[ARM]",
            EquipSlot::Shield => "[SHL]",
            EquipSlot::Helmet => "[HLM]",
            EquipSlot::Gloves => "[GLV]",
            EquipSlot::Boots => "[BTS]",
            EquipSlot::Ring1 | EquipSlot::Ring2 => "[RNG]",
            EquipSlot::Amulet => "[AMU]",
        }
    } else if kind.is_food() {
        "[FOOD]"
    } else if kind.is_consumable() {
        "[USE]"
    } else {
        "[MISC]"
    }
}

/// Handle inventory input - returns true if inventory should close
pub fn handle_inventory_input(code: KeyCode, state: &mut GameState) -> bool {
    INVENTORY_UI.with(|ui| {
        let mut ui = ui.borrow_mut();
        let inv_len = state.player.inventory.len();

        match code {
            KeyCode::Char('i') | KeyCode::Char('I') | KeyCode::Esc => return true,
            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => ui.move_selection(-1, inv_len),
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => ui.move_selection(1, inv_len),
            KeyCode::Char('s') | KeyCode::Char('S') => ui.cycle_sort(),
            KeyCode::Enter | KeyCode::Char(' ') => {
                if inv_len > 0 {
                    let sorted = get_sorted_indices(&state.player.inventory, ui.sort_mode);
                    if ui.selected_index < sorted.len() {
                        let real_idx = sorted[ui.selected_index];
                        state.use_item(real_idx);
                    }
                }
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                if inv_len > 0 {
                    let sorted = get_sorted_indices(&state.player.inventory, ui.sort_mode);
                    if ui.selected_index < sorted.len() {
                        let real_idx = sorted[ui.selected_index];
                        if real_idx < state.player.inventory.len() {
                            let dropped = state.player.inventory.remove(real_idx);
                            state.add_message(format!("Dropped {}.", dropped.kind.name()), 3);
                        }
                        if ui.selected_index >= state.player.inventory.len() && ui.selected_index > 0 {
                            ui.selected_index -= 1;
                        }
                    }
                }
            }
            KeyCode::Char(c) if c.is_ascii_digit() => {
                let idx = if c == '0' { 9 } else { (c as u8 - b'1') as usize };
                if idx < inv_len {
                    let sorted = get_sorted_indices(&state.player.inventory, ui.sort_mode);
                    if idx < sorted.len() {
                        let real_idx = sorted[idx];
                        state.use_item(real_idx);
                    }
                }
            }
            _ => {}
        }
        false
    })
}

fn render_inventory(state: &GameState) -> std::io::Result<()> {
    let mut stdout = stdout();

    INVENTORY_UI.with(|ui| -> std::io::Result<()> {
        let ui = ui.borrow();

        let start_x: u16 = 5;
        let start_y: u16 = 2;
        let main_width: usize = 42;
        let detail_width: usize = 32;
        let height: u16 = 38;

        // Draw main panel border
        for y in start_y..(start_y + height) {
            execute!(stdout, MoveTo(start_x, y))?;
            if y == start_y || y == start_y + height - 1 {
                execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
                write!(stdout, "{}", "=".repeat(main_width))?;
            } else {
                execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
                write!(stdout, "|{}|", " ".repeat(main_width - 2))?;
            }
        }

        // Title
        execute!(stdout, MoveTo(start_x + 2, start_y + 1))?;
        execute!(stdout, SetForegroundColor(Color::Yellow), SetAttribute(Attribute::Bold))?;
        write!(stdout, "INVENTORY")?;
        execute!(stdout, SetAttribute(Attribute::Reset), SetForegroundColor(Color::White))?;
        write!(stdout, " [{}/20]", state.player.inventory.len())?;

        // Sort indicator
        execute!(stdout, MoveTo(start_x + 22, start_y + 1))?;
        execute!(stdout, SetForegroundColor(Color::Cyan))?;
        write!(stdout, "Sort:{}", ui.sort_mode.name())?;

        // Equipment section
        execute!(stdout, MoveTo(start_x + 2, start_y + 3))?;
        execute!(stdout, SetForegroundColor(Color::Cyan), SetAttribute(Attribute::Bold))?;
        write!(stdout, "-- EQUIPPED --")?;
        execute!(stdout, SetAttribute(Attribute::Reset))?;

        let slots = [
            (EquipSlot::Weapon, "WPN", "ATK"),
            (EquipSlot::Armor,  "ARM", "DEF"),
            (EquipSlot::Shield, "SHL", "DEF"),
            (EquipSlot::Helmet, "HLM", "DEF"),
            (EquipSlot::Gloves, "GLV", "ATK"),
            (EquipSlot::Boots,  "BTS", "SPD"),
            (EquipSlot::Ring1,  "RNG", ""),
            (EquipSlot::Amulet, "AMU", ""),
        ];

        for (i, (slot, name, stat_type)) in slots.iter().enumerate() {
            execute!(stdout, MoveTo(start_x + 2, start_y + 4 + i as u16))?;
            if let Some(item) = state.player.equipment.get(slot) {
                execute!(stdout, SetForegroundColor(Color::Grey))?;
                write!(stdout, "{}:", name)?;
                execute!(stdout, SetForegroundColor(rarity_color(&item.rarity)))?;
                let item_name = format!("{}{}", item.rarity.prefix(), item.kind.name());
                let trunc = if item_name.len() > 22 { format!("{}...", &item_name[..19]) } else { item_name };
                write!(stdout, "{:<22}", trunc)?;
                if !stat_type.is_empty() {
                    let stat_val = match *stat_type {
                        "ATK" => item_attack_bonus(&item.kind),
                        "DEF" => item_defense_bonus(&item.kind),
                        _ => 0,
                    };
                    if stat_val > 0 {
                        execute!(stdout, SetForegroundColor(Color::Green))?;
                        write!(stdout, "+{}", stat_val)?;
                    }
                }
            } else {
                execute!(stdout, SetForegroundColor(Color::Grey))?;
                write!(stdout, "{}:", name)?;
                execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
                write!(stdout, "(empty)")?;
            }
            execute!(stdout, ResetColor)?;
        }

        // Player stats
        execute!(stdout, MoveTo(start_x + 2, start_y + 13))?;
        execute!(stdout, SetForegroundColor(Color::White))?;
        write!(stdout, "Stats: ")?;
        execute!(stdout, SetForegroundColor(Color::Yellow))?;
        write!(stdout, "ATK:{} ", state.player.total_attack())?;
        execute!(stdout, SetForegroundColor(Color::Cyan))?;
        write!(stdout, "DEF:{} ", state.player.total_defense())?;
        execute!(stdout, SetForegroundColor(Color::Green))?;
        write!(stdout, "HP:{}", state.player.total_max_hp())?;

        // Items section
        execute!(stdout, MoveTo(start_x + 2, start_y + 15))?;
        execute!(stdout, SetForegroundColor(Color::Cyan), SetAttribute(Attribute::Bold))?;
        write!(stdout, "-- ITEMS --")?;
        execute!(stdout, SetAttribute(Attribute::Reset))?;

        let sorted_indices = get_sorted_indices(&state.player.inventory, ui.sort_mode);
        let visible_items = 8;
        let scroll = ui.scroll_offset;

        for (display_idx, &real_idx) in sorted_indices.iter().skip(scroll).take(visible_items).enumerate() {
            let item = &state.player.inventory[real_idx];
            let line_y = start_y + 16 + display_idx as u16;
            let list_idx = scroll + display_idx;

            execute!(stdout, MoveTo(start_x + 2, line_y))?;

            if list_idx == ui.selected_index {
                execute!(stdout, SetForegroundColor(Color::White), SetAttribute(Attribute::Bold))?;
                write!(stdout, ">")?;
                execute!(stdout, SetAttribute(Attribute::Reset))?;
            } else {
                write!(stdout, " ")?;
            }

            let key = if display_idx == 9 { '0' } else { (b'1' + display_idx as u8) as char };
            execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
            write!(stdout, "[{}]", key)?;
            execute!(stdout, SetForegroundColor(Color::Grey))?;
            write!(stdout, "{}", item_type_indicator(&item.kind))?;
            execute!(stdout, SetForegroundColor(rarity_color(&item.rarity)))?;
            let item_name = format!("{}{}", item.rarity.prefix(), item.kind.name());
            let truncated = if item_name.len() > 18 { format!("{}...", &item_name[..15]) } else { item_name };
            write!(stdout, "{}", truncated)?;
            execute!(stdout, ResetColor)?;
        }

        // Scroll indicators
        if scroll > 0 {
            execute!(stdout, MoveTo(start_x + 38, start_y + 16), SetForegroundColor(Color::Yellow))?;
            write!(stdout, "^")?;
        }
        if scroll + visible_items < sorted_indices.len() {
            execute!(stdout, MoveTo(start_x + 38, start_y + 23), SetForegroundColor(Color::Yellow))?;
            write!(stdout, "v")?;
        }

        // Detail panel
        let detail_x = start_x + main_width as u16 + 1;

        for y in start_y..(start_y + height) {
            execute!(stdout, MoveTo(detail_x, y))?;
            if y == start_y || y == start_y + height - 1 {
                execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
                write!(stdout, "{}", "=".repeat(detail_width))?;
            } else {
                execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
                write!(stdout, "|{}|", " ".repeat(detail_width - 2))?;
            }
        }

        execute!(stdout, MoveTo(detail_x + 2, start_y + 1))?;
        execute!(stdout, SetForegroundColor(Color::Yellow), SetAttribute(Attribute::Bold))?;
        write!(stdout, "ITEM DETAILS")?;
        execute!(stdout, SetAttribute(Attribute::Reset))?;

        if !sorted_indices.is_empty() && ui.selected_index < sorted_indices.len() {
            let real_idx = sorted_indices[ui.selected_index];
            let item = &state.player.inventory[real_idx];

            execute!(stdout, MoveTo(detail_x + 2, start_y + 3))?;
            execute!(stdout, SetForegroundColor(rarity_color(&item.rarity)), SetAttribute(Attribute::Bold))?;
            let full_name = format!("{}{}", item.rarity.prefix(), item.kind.name());
            let name_trunc = if full_name.len() > detail_width - 4 { format!("{}...", &full_name[..(detail_width - 7)]) } else { full_name };
            write!(stdout, "{}", name_trunc)?;
            execute!(stdout, SetAttribute(Attribute::Reset))?;

            execute!(stdout, MoveTo(detail_x + 2, start_y + 4), SetForegroundColor(Color::Grey))?;
            write!(stdout, "Rarity: ")?;
            execute!(stdout, SetForegroundColor(rarity_color(&item.rarity)))?;
            write!(stdout, "{}", rarity_name(&item.rarity))?;

            execute!(stdout, MoveTo(detail_x + 2, start_y + 5), SetForegroundColor(Color::Grey))?;
            write!(stdout, "Type: ")?;
            execute!(stdout, SetForegroundColor(Color::White))?;
            write!(stdout, "{}", item_type_indicator(&item.kind).trim_matches(|c| c == '[' || c == ']'))?;

            execute!(stdout, MoveTo(detail_x + 2, start_y + 7), SetForegroundColor(Color::Cyan))?;
            write!(stdout, "-- Stats --")?;

            let mut stat_line = start_y + 8;
            let atk = item_attack_bonus(&item.kind);
            let def = item_defense_bonus(&item.kind);
            let hp = item_hp_bonus(&item.kind);
            let mp = item_mana_bonus(&item.kind);

            if atk > 0 {
                execute!(stdout, MoveTo(detail_x + 2, stat_line), SetForegroundColor(Color::Yellow))?;
                write!(stdout, "Attack: +{}", atk)?;
                stat_line += 1;
            }
            if def > 0 {
                execute!(stdout, MoveTo(detail_x + 2, stat_line), SetForegroundColor(Color::Cyan))?;
                write!(stdout, "Defense: +{}", def)?;
                stat_line += 1;
            }
            if hp > 0 {
                execute!(stdout, MoveTo(detail_x + 2, stat_line), SetForegroundColor(Color::Green))?;
                write!(stdout, "Max HP: +{}", hp)?;
                stat_line += 1;
            }
            if mp > 0 {
                execute!(stdout, MoveTo(detail_x + 2, stat_line), SetForegroundColor(Color::Blue))?;
                write!(stdout, "Max Mana: +{}", mp)?;
                stat_line += 1;
            }

            if item.kind.is_consumable() {
                let heal = item_heal_amount(&item.kind);
                let mana = item_mana_restore(&item.kind);
                let food = item.kind.food_value();

                if heal > 0 {
                    execute!(stdout, MoveTo(detail_x + 2, stat_line), SetForegroundColor(Color::Green))?;
                    write!(stdout, "Heals: {} HP", heal)?;
                    stat_line += 1;
                }
                if mana > 0 {
                    execute!(stdout, MoveTo(detail_x + 2, stat_line), SetForegroundColor(Color::Blue))?;
                    write!(stdout, "Restores: {} MP", mana)?;
                    stat_line += 1;
                }
                if food > 0 {
                    execute!(stdout, MoveTo(detail_x + 2, stat_line), SetForegroundColor(Color::Yellow))?;
                    write!(stdout, "Food: +{}", food)?;
                }
            }

            // Comparison
            if let Some(slot) = item.kind.equip_slot() {
                execute!(stdout, MoveTo(detail_x + 2, start_y + 16), SetForegroundColor(Color::Magenta))?;
                write!(stdout, "-- Comparison --")?;

                if let Some(equipped) = state.player.equipment.get(&slot) {
                    execute!(stdout, MoveTo(detail_x + 2, start_y + 17), SetForegroundColor(Color::Grey))?;
                    write!(stdout, "Equipped: ")?;
                    execute!(stdout, SetForegroundColor(rarity_color(&equipped.rarity)))?;
                    let eq_name = equipped.kind.name();
                    let eq_trunc = if eq_name.len() > 15 { format!("{}...", &eq_name[..12]) } else { eq_name.to_string() };
                    write!(stdout, "{}", eq_trunc)?;

                    let curr_atk = item_attack_bonus(&equipped.kind);
                    let new_atk = item_attack_bonus(&item.kind);
                    if curr_atk != 0 || new_atk != 0 {
                        execute!(stdout, MoveTo(detail_x + 2, start_y + 18), SetForegroundColor(Color::Grey))?;
                        write!(stdout, "ATK: {}->{} ", curr_atk, new_atk)?;
                        let diff = new_atk - curr_atk;
                        if diff > 0 {
                            execute!(stdout, SetForegroundColor(Color::Green))?;
                            write!(stdout, "(+{})", diff)?;
                        } else if diff < 0 {
                            execute!(stdout, SetForegroundColor(Color::Red))?;
                            write!(stdout, "({})", diff)?;
                        }
                    }

                    let curr_def = item_defense_bonus(&equipped.kind);
                    let new_def = item_defense_bonus(&item.kind);
                    if curr_def != 0 || new_def != 0 {
                        execute!(stdout, MoveTo(detail_x + 2, start_y + 19), SetForegroundColor(Color::Grey))?;
                        write!(stdout, "DEF: {}->{} ", curr_def, new_def)?;
                        let diff = new_def - curr_def;
                        if diff > 0 {
                            execute!(stdout, SetForegroundColor(Color::Green))?;
                            write!(stdout, "(+{})", diff)?;
                        } else if diff < 0 {
                            execute!(stdout, SetForegroundColor(Color::Red))?;
                            write!(stdout, "({})", diff)?;
                        }
                    }

                    let curr_hp = item_hp_bonus(&equipped.kind);
                    let new_hp = item_hp_bonus(&item.kind);
                    if curr_hp != 0 || new_hp != 0 {
                        execute!(stdout, MoveTo(detail_x + 2, start_y + 20), SetForegroundColor(Color::Grey))?;
                        write!(stdout, "HP: {}->{} ", curr_hp, new_hp)?;
                        let diff = new_hp - curr_hp;
                        if diff > 0 {
                            execute!(stdout, SetForegroundColor(Color::Green))?;
                            write!(stdout, "(+{})", diff)?;
                        } else if diff < 0 {
                            execute!(stdout, SetForegroundColor(Color::Red))?;
                            write!(stdout, "({})", diff)?;
                        }
                    }
                } else {
                    execute!(stdout, MoveTo(detail_x + 2, start_y + 17), SetForegroundColor(Color::DarkGrey))?;
                    write!(stdout, "Slot is empty")?;
                    execute!(stdout, MoveTo(detail_x + 2, start_y + 18), SetForegroundColor(Color::Green))?;
                    if atk > 0 { write!(stdout, "ATK:+{} ", atk)?; }
                    if def > 0 { write!(stdout, "DEF:+{} ", def)?; }
                }
            }
        } else {
            execute!(stdout, MoveTo(detail_x + 2, start_y + 3), SetForegroundColor(Color::DarkGrey))?;
            write!(stdout, "No items in inventory")?;
        }

        // Controls help
        execute!(stdout, MoveTo(start_x + 2, start_y + height - 3), SetForegroundColor(Color::DarkGrey))?;
        write!(stdout, "[I/ESC]Close [D]Drop [S]Sort [Enter]Use")?;

        execute!(stdout, ResetColor)?;
        Ok(())
    })
}


fn render_help_page(help_state: &HelpState) -> std::io::Result<()> {
    let mut stdout = stdout();

    let start_x = 5;
    let start_y = 2;
    let width = 70;
    let height = 40;

    // Draw border
    for y in start_y..(start_y + height) {
        execute!(stdout, MoveTo(start_x, y))?;
        if y == start_y || y == start_y + height - 1 {
            execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
            write!(stdout, "{}", "=".repeat(width))?;
        } else {
            execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
            write!(stdout, "|")?;
            execute!(stdout, ResetColor)?;
            write!(stdout, "{}", " ".repeat(width - 2))?;
            execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
            write!(stdout, "|")?;
        }
        execute!(stdout, ResetColor)?;
    }

    // Title bar with page indicator
    execute!(stdout, MoveTo(start_x + 2, start_y + 1))?;
    execute!(stdout, SetForegroundColor(Color::Yellow), SetAttribute(Attribute::Bold))?;
    write!(stdout, "=== SHADOWCRYPT HELP: {} ===", help_state.page.title())?;
    execute!(stdout, SetAttribute(Attribute::Reset), ResetColor)?;

    // Page indicator
    execute!(stdout, MoveTo(start_x + width as u16 - 15, start_y + 1))?;
    execute!(stdout, SetForegroundColor(Color::Cyan))?;
    write!(stdout, "Page {}/6", help_state.page.page_number())?;
    execute!(stdout, ResetColor)?;

    // Get content for current page
    let content = help_state.page.content();

    for (i, (key, desc)) in content.iter().enumerate() {
        execute!(stdout, MoveTo(start_x + 2, start_y + 3 + i as u16))?;
        if key.starts_with("  ") {
            // Indented item
            execute!(stdout, SetForegroundColor(Color::Cyan))?;
            write!(stdout, "{:<24}", key)?;
            execute!(stdout, SetForegroundColor(Color::White))?;
            write!(stdout, "{}", desc)?;
        } else if !key.is_empty() {
            // Section header
            execute!(stdout, SetForegroundColor(Color::Yellow))?;
            write!(stdout, "{}", key)?;
            execute!(stdout, SetForegroundColor(Color::White))?;
            write!(stdout, "{}", desc)?;
        }
        execute!(stdout, ResetColor)?;
    }

    // Page tabs at bottom
    execute!(stdout, MoveTo(start_x + 2, start_y + height as u16 - 4))?;
    execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
    write!(stdout, "{}", "-".repeat(width - 4))?;

    execute!(stdout, MoveTo(start_x + 2, start_y + height as u16 - 3))?;
    let pages = [
        (HelpPage::Overview, "1:Overview"),
        (HelpPage::Controls, "2:Controls"),
        (HelpPage::Items, "3:Items"),
        (HelpPage::Enemies, "4:Enemies"),
        (HelpPage::Skills, "5:Skills"),
        (HelpPage::Mechanics, "6:Mechanics"),
    ];
    for (page, label) in pages.iter() {
        if *page == help_state.page {
            execute!(stdout, SetForegroundColor(Color::Yellow), SetAttribute(Attribute::Bold))?;
            write!(stdout, "[{}] ", label)?;
            execute!(stdout, SetAttribute(Attribute::Reset))?;
        } else {
            execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
            write!(stdout, " {}  ", label)?;
        }
    }
    execute!(stdout, ResetColor)?;

    // Navigation instructions
    execute!(stdout, MoveTo(start_x + 2, start_y + height as u16 - 2))?;
    execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
    write!(stdout, "[Left/H: Prev] [Right/L: Next] [1-6: Jump] [?/ESC: Close]")?;
    execute!(stdout, ResetColor)?;

    Ok(())
}

/// Render tutorial overlay
fn render_tutorial(tutorial: &TutorialState) -> std::io::Result<()> {
    let mut stdout = stdout();

    let start_x = 10;
    let start_y = 8;
    let width = 60;
    let height = 20;

    // Draw border with highlight
    for y in start_y..(start_y + height) {
        execute!(stdout, MoveTo(start_x, y))?;
        if y == start_y || y == start_y + height - 1 {
            execute!(stdout, SetForegroundColor(Color::Yellow))?;
            write!(stdout, "{}", "=".repeat(width))?;
        } else {
            execute!(stdout, SetForegroundColor(Color::Yellow))?;
            write!(stdout, "|")?;
            execute!(stdout, SetBackgroundColor(Color::Rgb { r: 20, g: 20, b: 40 }))?;
            write!(stdout, "{}", " ".repeat(width - 2))?;
            execute!(stdout, ResetColor)?;
            execute!(stdout, SetForegroundColor(Color::Yellow))?;
            write!(stdout, "|")?;
        }
        execute!(stdout, ResetColor)?;
    }

    // Title
    execute!(stdout, MoveTo(start_x + 2, start_y + 1))?;
    execute!(stdout, SetForegroundColor(Color::Cyan), SetAttribute(Attribute::Bold))?;
    write!(stdout, "TUTORIAL: {}", tutorial.step.title())?;
    execute!(stdout, SetAttribute(Attribute::Reset), ResetColor)?;

    // Step indicator
    execute!(stdout, MoveTo(start_x + width as u16 - 12, start_y + 1))?;
    execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
    write!(stdout, "Step {}/8", tutorial.step.step_number())?;
    execute!(stdout, ResetColor)?;

    // Content lines
    let lines = tutorial.step.lines();
    for (i, line) in lines.iter().enumerate() {
        execute!(stdout, MoveTo(start_x + 3, start_y + 3 + i as u16))?;

        // Color code certain elements
        if line.starts_with("  ") && line.contains(" - ") {
            // Key binding line
            let parts: Vec<&str> = line.splitn(2, " - ").collect();
            execute!(stdout, SetForegroundColor(Color::Cyan))?;
            write!(stdout, "{}", parts[0])?;
            if parts.len() > 1 {
                execute!(stdout, SetForegroundColor(Color::White))?;
                write!(stdout, " - {}", parts[1])?;
            }
        } else if line.contains("=") && !line.starts_with("  ") {
            // Item/rarity line
            execute!(stdout, SetForegroundColor(Color::Green))?;
            write!(stdout, "{}", line)?;
        } else if line.starts_with("  ") {
            // Indented instruction
            execute!(stdout, SetForegroundColor(Color::Yellow))?;
            write!(stdout, "{}", line)?;
        } else {
            execute!(stdout, SetForegroundColor(Color::White))?;
            write!(stdout, "{}", line)?;
        }
        execute!(stdout, ResetColor)?;
    }

    // Progress bar
    execute!(stdout, MoveTo(start_x + 2, start_y + height as u16 - 3))?;
    execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
    write!(stdout, "Progress: ")?;
    let step_num = tutorial.step.step_number() as usize;
    for i in 1..=8 {
        if i < step_num {
            execute!(stdout, SetForegroundColor(Color::Green))?;
            write!(stdout, "[#]")?;
        } else if i == step_num {
            execute!(stdout, SetForegroundColor(Color::Yellow))?;
            write!(stdout, "[>]")?;
        } else {
            execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
            write!(stdout, "[ ]")?;
        }
    }
    execute!(stdout, ResetColor)?;

    // Navigation hint
    execute!(stdout, MoveTo(start_x + 2, start_y + height as u16 - 2))?;
    execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
    write!(stdout, "{}", tutorial.step.hint())?;
    execute!(stdout, ResetColor)?;

    Ok(())
}

/// Legacy render_help wrapper for compatibility
fn render_help(_state: &GameState) -> std::io::Result<()> {
    // This is called when state.show_help is true but we don't have HelpState
    // Create a default help state and render
    let help_state = HelpState::new();
    render_help_page(&help_state)
}

fn render_class_select() -> std::io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

    execute!(stdout, SetForegroundColor(Color::Yellow))?;
    write!(stdout, "=== SHADOWCRYPT ===\n\n")?;
    execute!(stdout, SetForegroundColor(Color::White))?;
    write!(stdout, "Choose your class:\n\n")?;

    let classes = [
        ("1", "Warrior", "High HP, high attack, shield abilities"),
        ("2", "Mage", "Low HP, powerful spells, mana regeneration"),
        ("3", "Rogue", "Fast, critical hits, stealth abilities"),
        ("4", "Paladin", "Balanced, healing, holy damage"),
        ("5", "Ranger", "Ranged attacks, traps, animal companion"),
        ("6", "Necromancer", "Summons, life drain, dark magic"),
    ];

    for (key, name, desc) in classes.iter() {
        execute!(stdout, SetForegroundColor(Color::Cyan))?;
        write!(stdout, "  [{}] ", key)?;
        execute!(stdout, SetForegroundColor(Color::Yellow))?;
        write!(stdout, "{:<12}", name)?;
        execute!(stdout, SetForegroundColor(Color::Grey))?;
        write!(stdout, " - {}\n", desc)?;
    }

    execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
    write!(stdout, "\n  [Q] Quit\n")?;
    execute!(stdout, ResetColor)?;

    stdout.flush()?;
    Ok(())
}

// ============================================================================
// MAIN
// ============================================================================

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let auto_play = args.iter().any(|a| a == "--auto" || a == "-a");
    let auto_speed: u64 = args.iter()
        .position(|a| a == "--speed" || a == "-s")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);

    let mut stdout = stdout();

    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, Hide)?;

    // Class selection (auto-pick random in auto mode)
    let selected_class = if auto_play {
        let classes: Vec<_> = CharacterClass::all().collect();
        classes[thread_rng().gen_range(0..classes.len())]
    } else {
        loop {
            render_class_select()?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                    match code {
                        KeyCode::Char('1') => break CharacterClass::Warrior,
                        KeyCode::Char('2') => break CharacterClass::Mage,
                        KeyCode::Char('3') => break CharacterClass::Rogue,
                        KeyCode::Char('4') => break CharacterClass::Paladin,
                        KeyCode::Char('5') => break CharacterClass::Ranger,
                        KeyCode::Char('6') => break CharacterClass::Necromancer,
                        KeyCode::Char('q') | KeyCode::Esc => {
                            execute!(stdout, Show, LeaveAlternateScreen)?;
                            terminal::disable_raw_mode()?;
                            return Ok(());
                        }
                        _ => {}
                    }
                }
            }
        }
    };

    let mut state = GameState::new(selected_class);
    let mut movement_state = MovementState::new();
    let mut targeting_state = TargetingState::new();
    let mut last_render = Instant::now();

    // Game loop - optimized for responsiveness
    loop {
        // Update targeting list based on visible enemies
        targeting_state.update_targets(&state);

        // Render with movement and targeting state
        // Only re-render if enough time has passed or state changed
        let should_render = last_render.elapsed().as_millis() > 16; // ~60fps cap
        if should_render {
            render_with_targeting(&state, Some(&movement_state), Some(&targeting_state))?;
            last_render = Instant::now();
        }

        if state.game_over || state.victory {
            if auto_play {
                std::thread::sleep(Duration::from_millis(2000));
                break;
            }
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(_) = event::read()? {
                    break;
                }
            }
            continue;
        }

        // Auto-play mode
        if auto_play {
            if event::poll(Duration::from_millis(10))? {
                if let Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) = event::read()? {
                    break;
                }
            }

            let action = state.ai_decide();
            state.ai_execute(action);
            std::thread::sleep(Duration::from_millis(auto_speed));
            continue;
        }

        // Handle running mode - auto-continue movement
        if movement_state.should_continue_run(&state) {
            if let Some(dir) = movement_state.run_direction {
                state.move_player(dir.dx, dir.dy);
                movement_state.record_move(dir);
                // Brief delay for running animation
                std::thread::sleep(Duration::from_millis(40));
                continue;
            }
        } else {
            movement_state.stop_run();
        }

        // Reduced poll timeout for snappier input response
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                let shift_held = modifiers.contains(KeyModifiers::SHIFT);

                // Inventory mode
                if state.show_inventory {
                    match code {
                        KeyCode::Char('i') | KeyCode::Char('I') | KeyCode::Esc => state.show_inventory = false,
                        KeyCode::Char(c) if c.is_ascii_digit() => {
                            let idx = if c == '0' { 9 } else { (c as u8 - b'1') as usize };
                            state.use_item(idx);
                        }
                        _ => {}
                    }
                    continue;
                }

                // Help mode
                if state.show_help {
                    match code {
                        KeyCode::Char('?') | KeyCode::Esc => state.show_help = false,
                        _ => {}
                    }
                    continue;
                }

                // Handle targeting mode inputs first
                if targeting_state.active {
                    match code {
                        // Cancel targeting
                        KeyCode::Esc => {
                            targeting_state.deactivate();
                            continue;
                        }
                        // Cycle targets
                        KeyCode::Tab if !shift_held => {
                            targeting_state.next_target();
                            continue;
                        }
                        KeyCode::Tab if shift_held => {
                            targeting_state.prev_target();
                            continue;
                        }
                        KeyCode::BackTab => {
                            targeting_state.prev_target();
                            continue;
                        }
                        // Lock target
                        KeyCode::Char('t') | KeyCode::Char('T') => {
                            targeting_state.toggle_lock();
                            if targeting_state.locked {
                                state.add_message("Target locked!".to_string(), 11);
                            } else {
                                state.add_message("Target unlocked.".to_string(), 8);
                            }
                            continue;
                        }
                        // Attack target
                        KeyCode::Char('f') | KeyCode::Char('F') => {
                            if let Some(target) = targeting_state.current_target() {
                                // Check if target is adjacent
                                let dx = target.x as i32 - state.player.x as i32;
                                let dy = target.y as i32 - state.player.y as i32;
                                let dist = ((dx * dx + dy * dy) as f32).sqrt();

                                if dist < 1.5 {
                                    // Adjacent - attack directly
                                    if let Some(idx) = targeting_state.current_enemy_index() {
                                        state.attack_enemy(idx);
                                        movement_state.stop_run();
                                    }
                                } else {
                                    // Not adjacent - move towards target
                                    state.move_player(dx.signum(), dy.signum());
                                    movement_state.record_move(Direction {
                                        dx: dx.signum(),
                                        dy: dy.signum(),
                                    });
                                }
                            }
                            continue;
                        }
                        // Use skill on target
                        KeyCode::Char(' ') => {
                            // Skills will use the targeted enemy
                            state.use_skill();
                            movement_state.stop_run();
                            continue;
                        }
                        _ => {
                            // Allow other keys to fall through (movement, etc.)
                        }
                    }
                }

                // Check for movement keys
                if let Some(dir) = key_to_direction(code) {
                    // Deactivate targeting on movement (unless locked)
                    if !targeting_state.locked {
                        targeting_state.deactivate();
                    }

                    if shift_held {
                        // Start running in this direction
                        movement_state.start_run(dir);
                        state.move_player(dir.dx, dir.dy);
                        movement_state.record_move(dir);
                    } else {
                        // Normal single-step movement
                        movement_state.stop_run();
                        state.move_player(dir.dx, dir.dy);
                        movement_state.record_move(dir);
                    }
                    continue;
                }

                // Normal mode - non-movement actions
                match code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => break,
                    KeyCode::Esc if !targeting_state.active => break,
                    KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => break,

                    // Wait/Rest action (5 or . without shift)
                    KeyCode::Char('5') => {
                        state.end_turn();
                        movement_state.stop_run();
                        if !targeting_state.locked {
                            targeting_state.deactivate();
                        }
                    }
                    KeyCode::Char('.') if !shift_held => {
                        state.end_turn();
                        movement_state.stop_run();
                        if !targeting_state.locked {
                            targeting_state.deactivate();
                        }
                    }

                    // Targeting - Tab cycles targets when not in targeting mode
                    KeyCode::Tab if !shift_held => {
                        targeting_state.next_target();
                    }
                    KeyCode::Tab if shift_held && !targeting_state.active => {
                        // Shift+Tab cycles skills when not targeting
                        state.cycle_skill();
                    }
                    KeyCode::BackTab if !targeting_state.active => {
                        state.cycle_skill();
                    }

                    // Skills
                    KeyCode::Char(' ') => {
                        state.use_skill();
                        movement_state.stop_run();
                    }

                    // Stairs (> requires shift)
                    KeyCode::Char('>') => {
                        state.descend();
                        movement_state.stop_run();
                        targeting_state.deactivate();
                    }
                    KeyCode::Char('<') => {
                        state.ascend();
                        movement_state.stop_run();
                        targeting_state.deactivate();
                    }

                    // Inventory
                    KeyCode::Char('i') | KeyCode::Char('I') => {
                        state.show_inventory = true;
                        targeting_state.deactivate();
                    }
                    KeyCode::Char(c) if c.is_ascii_digit() => {
                        let idx = if c == '0' { 9 } else { (c as u8 - b'1') as usize };
                        state.use_item(idx);
                        movement_state.stop_run();
                    }

                    // Help
                    KeyCode::Char('?') => {
                        state.show_help = true;
                        targeting_state.deactivate();
                    }

                    // Grab/pickup explicitly (g key - common roguelike binding)
                    KeyCode::Char('g') | KeyCode::Char('G') => {
                        state.pickup_items();
                    }

                    // Target lock (outside of targeting mode - activates and locks nearest)
                    KeyCode::Char('t') | KeyCode::Char('T') => {
                        if !targeting_state.active {
                            targeting_state.activate();
                        }
                        targeting_state.toggle_lock();
                        if targeting_state.locked {
                            state.add_message("Target locked!".to_string(), 11);
                        }
                    }

                    // Fire/attack toward target (auto-target nearest if none selected)
                    KeyCode::Char('f') | KeyCode::Char('F') => {
                        if !targeting_state.active {
                            targeting_state.activate();
                        }
                        if let Some(target) = targeting_state.current_target() {
                            let dx = target.x as i32 - state.player.x as i32;
                            let dy = target.y as i32 - state.player.y as i32;
                            let dist = ((dx * dx + dy * dy) as f32).sqrt();

                            if dist < 1.5 {
                                if let Some(idx) = targeting_state.current_enemy_index() {
                                    state.attack_enemy(idx);
                                    movement_state.stop_run();
                                }
                            } else {
                                state.add_message(format!("Target too far! Move closer to {}.", target.name), 3);
                            }
                        } else {
                            state.add_message("No valid target!".to_string(), 3);
                        }
                    }

                    _ => {}
                }
            }
        }
    }

    // Cleanup
    execute!(stdout, Show, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    // Final stats
    println!();
    if state.victory {
        println!("*** CONGRATULATIONS! YOU HAVE CONQUERED SHADOWCRYPT! ***");
        println!();
        println!("Class: {}", state.player.class.name());
        println!("Final Level: {}", state.player.level);
        println!("Gold Collected: {}", state.player.gold);
        println!("Enemies Slain: {}", state.player.kills);
        println!("Turns Taken: {}", state.turn_count);
        println!("Floors Explored: {}", state.dungeon_level);
    } else if state.game_over {
        println!("*** GAME OVER ***");
        println!();
        println!("Class: {}", state.player.class.name());
        println!("Died on floor {} after {} turns.", state.dungeon_level, state.turn_count);
        println!("Level: {} | Gold: {} | Kills: {}", state.player.level, state.player.gold, state.player.kills);
    } else {
        println!("Thanks for playing ShadowCrypt!");
    }

    Ok(())
}
