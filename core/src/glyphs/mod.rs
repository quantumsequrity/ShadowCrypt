//! Map Symbols and Glyphs Module
//!
//! This module provides a comprehensive set of glyphs and symbols for rendering
//! the game world, entities, items, and UI elements. It supports multiple glyph
//! sets for different terminal capabilities.
//!
//! # Glyph Sets
//!
//! - **ASCII**: Basic ASCII characters for maximum compatibility
//! - **Unicode**: Enhanced Unicode symbols for better visuals
//! - **Emoji**: Emoji characters for modern terminal support
//!
//! # Example Usage
//!
//! ```rust
//! use shadowcrypt_core::glyphs::{GlyphSet, UNICODE_GLYPHS, PLAYER, WALL};
//!
//! // Use default Unicode glyphs
//! let glyphs = UNICODE_GLYPHS;
//! println!("Player: {}", glyphs.player);
//!
//! // Or use constants directly
//! println!("Wall: {}", WALL);
//! ```

use serde::{Deserialize, Serialize};

// ============================================================================
// TERRAIN GLYPHS
// ============================================================================

/// Standard floor tile
pub const FLOOR: char = '\u{00B7}'; // ·
/// Alternative floor tile (ASCII compatible)
pub const FLOOR_ALT: char = '.';
/// Solid wall block
pub const WALL: char = '\u{2588}'; // █
/// Horizontal wall segment
pub const WALL_H: char = '\u{2500}'; // ─
/// Vertical wall segment
pub const WALL_V: char = '\u{2502}'; // │
/// Top-left corner
pub const CORNER_TL: char = '\u{250C}'; // ┌
/// Top-right corner
pub const CORNER_TR: char = '\u{2510}'; // ┐
/// Bottom-left corner
pub const CORNER_BL: char = '\u{2514}'; // └
/// Bottom-right corner
pub const CORNER_BR: char = '\u{2518}'; // ┘
/// Closed door
pub const DOOR_CLOSED: char = '+';
/// Open door
pub const DOOR_OPEN: char = '\u{256C}'; // ╬
/// Stairs leading up
pub const STAIRS_UP: char = '<';
/// Stairs leading down
pub const STAIRS_DOWN: char = '>';
/// Water terrain
pub const WATER: char = '\u{2248}'; // ≈
/// Lava terrain
pub const LAVA: char = '~';
/// Hidden trap
pub const TRAP: char = '^';
/// Portal/teleporter
pub const PORTAL: char = 'O';

// Additional terrain glyphs
/// Deep water
pub const WATER_DEEP: char = '\u{224B}'; // ≋
/// Shallow water
pub const WATER_SHALLOW: char = '\u{223C}'; // ∼
/// Grass terrain
pub const GRASS: char = ',';
/// Tree/forest
pub const TREE: char = '\u{2663}'; // ♣
/// Mountain/rock
pub const MOUNTAIN: char = '\u{25B2}'; // ▲
/// Bridge
pub const BRIDGE: char = '=';
/// Pit/hole
pub const PIT: char = '\u{25CB}'; // ○
/// Altar
pub const ALTAR: char = '\u{2021}'; // ‡
/// Fountain
pub const FOUNTAIN: char = '\u{00A7}'; // §
/// Campfire
pub const CAMPFIRE: char = '\u{00A4}'; // ¤
/// Rubble/debris
pub const RUBBLE: char = ';';
/// Ice terrain
pub const ICE: char = '\u{2022}'; // •
/// Sand terrain
pub const SAND: char = ':';
/// Secret door (hidden)
pub const SECRET_DOOR: char = '#';
/// Broken wall
pub const WALL_BROKEN: char = '\u{2591}'; // ░
/// Thick wall
pub const WALL_THICK: char = '\u{2593}'; // ▓

// T-junction wall pieces
/// T-junction pointing down
pub const WALL_T_DOWN: char = '\u{252C}'; // ┬
/// T-junction pointing up
pub const WALL_T_UP: char = '\u{2534}'; // ┴
/// T-junction pointing right
pub const WALL_T_RIGHT: char = '\u{251C}'; // ├
/// T-junction pointing left
pub const WALL_T_LEFT: char = '\u{2524}'; // ┤
/// Cross junction (4-way)
pub const WALL_CROSS: char = '\u{253C}'; // ┼

// ============================================================================
// ENTITY GLYPHS
// ============================================================================

/// Player character
pub const PLAYER: char = '@';
/// Goblin enemy
pub const GOBLIN: char = 'g';
/// Orc enemy
pub const ORC: char = 'o';
/// Skeleton enemy
pub const SKELETON: char = 's';
/// Zombie enemy
pub const ZOMBIE: char = 'z';
/// Dragon enemy
pub const DRAGON: char = 'D';
/// Demon enemy
pub const DEMON: char = '&';
/// Boss enemy
pub const BOSS: char = 'B';
/// Non-player character
pub const NPC: char = 'P';
/// Merchant NPC
pub const MERCHANT: char = '$';
/// Companion/ally
pub const COMPANION: char = 'c';

// Additional entity glyphs
/// Rat enemy
pub const RAT: char = 'r';
/// Bat enemy
pub const BAT: char = 'b';
/// Spider enemy
pub const SPIDER: char = 'x';
/// Snake enemy
pub const SNAKE: char = 'S';
/// Ghost enemy
pub const GHOST: char = 'G';
/// Vampire enemy
pub const VAMPIRE: char = 'V';
/// Werewolf enemy
pub const WEREWOLF: char = 'W';
/// Lich enemy
pub const LICH: char = 'L';
/// Troll enemy
pub const TROLL: char = 'T';
/// Golem enemy
pub const GOLEM: char = 'Y';
/// Elemental enemy
pub const ELEMENTAL: char = 'E';
/// Slime enemy
pub const SLIME: char = 'j';
/// Imp enemy
pub const IMP: char = 'i';
/// Wraith enemy
pub const WRAITH: char = 'w';
/// Mimic enemy (looks like chest)
pub const MIMIC: char = 'M';
/// Hydra enemy
pub const HYDRA: char = 'H';
/// Giant enemy
pub const GIANT: char = 'R';
/// Necromancer enemy
pub const NECROMANCER: char = 'N';
/// Cultist enemy
pub const CULTIST: char = 'C';
/// Quest giver NPC
pub const QUEST_GIVER: char = '!';
/// Guard NPC
pub const GUARD: char = 'Q';
/// Healer NPC
pub const HEALER: char = 'h';
/// Blacksmith NPC
pub const BLACKSMITH: char = 'K';
/// Summoned creature
pub const SUMMON: char = '\u{03B1}'; // α
/// Pet/familiar
pub const PET: char = 'p';
/// Mount
pub const MOUNT: char = '\u{03C9}'; // ω

// ============================================================================
// ITEM GLYPHS
// ============================================================================

/// Gold/currency
pub const GOLD: char = '*';
/// Potion item
pub const POTION: char = '!';
/// Scroll item
pub const SCROLL: char = '?';
/// Weapon item
pub const WEAPON: char = '/';
/// Armor item
pub const ARMOR: char = '[';
/// Ring item
pub const RING: char = '=';
/// Amulet item
pub const AMULET: char = '"';
/// Food item
pub const FOOD: char = '%';
/// Key item
pub const KEY: char = 'k';
/// Chest container
pub const CHEST: char = '\u{25A1}'; // □

// Additional item glyphs
/// Helmet/headgear
pub const HELMET: char = ']';
/// Shield item
pub const SHIELD_ITEM: char = ')';
/// Boots/footwear
pub const BOOTS: char = '\\';
/// Gloves/gauntlets
pub const GLOVES: char = '(';
/// Cloak/cape
pub const CLOAK: char = '`';
/// Belt item
pub const BELT: char = '-';
/// Wand item
pub const WAND: char = '\u{00AC}'; // ¬
/// Staff item
pub const STAFF: char = '|';
/// Bow item
pub const BOW: char = '}';
/// Arrows/ammunition
pub const ARROWS: char = '{';
/// Tome/book
pub const TOME: char = '\u{00B6}'; // ¶
/// Gem/jewel
pub const GEM: char = '\u{25C6}'; // ◆
/// Rune item
pub const RUNE: char = '\u{00A5}'; // ¥
/// Artifact item
pub const ARTIFACT: char = '\u{00A9}'; // ©
/// Orb item
pub const ORB: char = '\u{00B0}'; // °
/// Tool item
pub const TOOL: char = '\u{00A2}'; // ¢
/// Material/ingredient
pub const MATERIAL: char = '\u{00A3}'; // £
/// Quest item
pub const QUEST_ITEM: char = '\u{00A1}'; // ¡
/// Locked chest
pub const CHEST_LOCKED: char = '\u{25A0}'; // ■
/// Open/empty chest
pub const CHEST_OPEN: char = '\u{25A2}'; // ▢
/// Bag/sack
pub const BAG: char = '\u{00AB}'; // «
/// Barrel container
pub const BARREL: char = '\u{00BB}'; // »
/// Pile of items
pub const ITEM_PILE: char = '\u{00B8}'; // ¸
/// Legendary item marker
pub const LEGENDARY: char = '\u{00AE}'; // ®

// ============================================================================
// UI GLYPHS
// ============================================================================

/// Health/heart symbol
pub const HEART: char = '\u{2665}'; // ♥
/// Mana/magic symbol
pub const MANA: char = '\u{25C6}'; // ◆
/// Star symbol
pub const STAR: char = '\u{2605}'; // ★
/// Up arrow
pub const ARROW_UP: char = '\u{2191}'; // ↑
/// Down arrow
pub const ARROW_DOWN: char = '\u{2193}'; // ↓
/// Left arrow
pub const ARROW_LEFT: char = '\u{2190}'; // ←
/// Right arrow
pub const ARROW_RIGHT: char = '\u{2192}'; // →
/// Skull/death symbol
pub const SKULL: char = '\u{2620}'; // ☠
/// Sword symbol
pub const SWORD: char = '\u{2020}'; // †
/// Shield symbol
pub const SHIELD: char = '\u{25CA}'; // ◊

// Additional UI glyphs
/// Empty heart (missing health)
pub const HEART_EMPTY: char = '\u{2661}'; // ♡
/// Half heart
pub const HEART_HALF: char = '\u{2764}'; // ❤
/// Empty mana
pub const MANA_EMPTY: char = '\u{25C7}'; // ◇
/// Empty star
pub const STAR_EMPTY: char = '\u{2606}'; // ☆
/// Checkmark
pub const CHECK: char = '\u{2713}'; // ✓
/// Cross/X mark
pub const CROSS: char = '\u{2717}'; // ✗
/// Bullet point
pub const BULLET: char = '\u{2022}'; // •
/// Diamond
pub const DIAMOND: char = '\u{2666}'; // ♦
/// Club symbol
pub const CLUB: char = '\u{2663}'; // ♣
/// Spade symbol
pub const SPADE: char = '\u{2660}'; // ♠
/// Sun symbol
pub const SUN: char = '\u{263C}'; // ☼
/// Moon symbol
pub const MOON: char = '\u{263E}'; // ☾
/// Lightning bolt
pub const LIGHTNING: char = '\u{2607}'; // ☇
/// Fire symbol
pub const FIRE: char = '\u{2668}'; // ♨
/// Snowflake
pub const SNOWFLAKE: char = '\u{2744}'; // ❄
/// Music note
pub const MUSIC: char = '\u{266A}'; // ♪
/// Infinity symbol
pub const INFINITY: char = '\u{221E}'; // ∞
/// Warning/alert
pub const WARNING: char = '\u{26A0}'; // ⚠
/// Info symbol
pub const INFO: char = '\u{2139}'; // ℹ
/// Crown
pub const CROWN: char = '\u{265B}'; // ♛
/// Hourglass
pub const HOURGLASS: char = '\u{29D6}'; // ⧖
/// Lock symbol
pub const LOCK: char = '\u{1F512}'; // 🔒 (may need fallback)
/// Key symbol (UI)
pub const KEY_UI: char = '\u{26BF}'; // ⚿

// Progress bar elements
/// Progress bar empty
pub const BAR_EMPTY: char = '\u{2591}'; // ░
/// Progress bar half
pub const BAR_HALF: char = '\u{2592}'; // ▒
/// Progress bar full
pub const BAR_FULL: char = '\u{2593}'; // ▓

// Box drawing for UI
/// Box horizontal
pub const BOX_H: char = '\u{2550}'; // ═
/// Box vertical
pub const BOX_V: char = '\u{2551}'; // ║
/// Box corner top-left
pub const BOX_TL: char = '\u{2554}'; // ╔
/// Box corner top-right
pub const BOX_TR: char = '\u{2557}'; // ╗
/// Box corner bottom-left
pub const BOX_BL: char = '\u{255A}'; // ╚
/// Box corner bottom-right
pub const BOX_BR: char = '\u{255D}'; // ╝
/// Box T-junction down
pub const BOX_T_DOWN: char = '\u{2566}'; // ╦
/// Box T-junction up
pub const BOX_T_UP: char = '\u{2569}'; // ╩
/// Box T-junction right
pub const BOX_T_RIGHT: char = '\u{2560}'; // ╠
/// Box T-junction left
pub const BOX_T_LEFT: char = '\u{2563}'; // ╣
/// Box cross
pub const BOX_CROSS: char = '\u{256C}'; // ╬

// ============================================================================
// EFFECT GLYPHS
// ============================================================================

/// Explosion effect
pub const EXPLOSION: char = '\u{2731}'; // ✱
/// Magic sparkle
pub const SPARKLE: char = '\u{2728}'; // ✨
/// Poison effect
pub const POISON: char = '\u{2623}'; // ☣
/// Radiation/corruption
pub const RADIATION: char = '\u{2622}'; // ☢
/// Shield aura
pub const AURA: char = '\u{25CE}'; // ◎
/// Target marker
pub const TARGET: char = '\u{25CE}'; // ◎
/// Projectile
pub const PROJECTILE: char = '\u{2219}'; // ∙
/// Beam effect
pub const BEAM: char = '\u{2261}'; // ≡
/// Wave effect
pub const WAVE: char = '\u{223F}'; // ∿

// ============================================================================
// GLYPH SET STRUCTURE
// ============================================================================

/// Complete set of glyphs for rendering the game
///
/// This struct contains all glyphs organized by category, allowing for
/// easy switching between different visual styles (ASCII, Unicode, Emoji).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlyphSet {
    /// Name of this glyph set
    pub name: String,

    // Terrain
    pub floor: char,
    pub floor_alt: char,
    pub wall: char,
    pub wall_h: char,
    pub wall_v: char,
    pub corner_tl: char,
    pub corner_tr: char,
    pub corner_bl: char,
    pub corner_br: char,
    pub door_closed: char,
    pub door_open: char,
    pub stairs_up: char,
    pub stairs_down: char,
    pub water: char,
    pub lava: char,
    pub trap: char,
    pub portal: char,
    pub grass: char,
    pub tree: char,
    pub mountain: char,
    pub bridge: char,
    pub pit: char,
    pub altar: char,
    pub fountain: char,
    pub campfire: char,
    pub ice: char,
    pub sand: char,
    pub secret_door: char,

    // Entities
    pub player: char,
    pub goblin: char,
    pub orc: char,
    pub skeleton: char,
    pub zombie: char,
    pub dragon: char,
    pub demon: char,
    pub boss: char,
    pub npc: char,
    pub merchant: char,
    pub companion: char,
    pub rat: char,
    pub bat: char,
    pub spider: char,
    pub snake: char,
    pub ghost: char,
    pub vampire: char,
    pub werewolf: char,
    pub lich: char,
    pub troll: char,
    pub golem: char,
    pub slime: char,

    // Items
    pub gold: char,
    pub potion: char,
    pub scroll: char,
    pub weapon: char,
    pub armor: char,
    pub ring: char,
    pub amulet: char,
    pub food: char,
    pub key: char,
    pub chest: char,
    pub chest_locked: char,
    pub chest_open: char,
    pub helmet: char,
    pub shield_item: char,
    pub boots: char,
    pub gloves: char,
    pub cloak: char,
    pub wand: char,
    pub staff: char,
    pub bow: char,
    pub arrows: char,
    pub tome: char,
    pub gem: char,
    pub rune: char,

    // UI
    pub heart: char,
    pub heart_empty: char,
    pub mana: char,
    pub mana_empty: char,
    pub star: char,
    pub star_empty: char,
    pub arrow_up: char,
    pub arrow_down: char,
    pub arrow_left: char,
    pub arrow_right: char,
    pub skull: char,
    pub sword: char,
    pub shield: char,
    pub check: char,
    pub cross: char,
    pub warning: char,
    pub sun: char,
    pub moon: char,
    pub fire: char,
    pub snowflake: char,
    pub crown: char,

    // Box drawing
    pub box_h: char,
    pub box_v: char,
    pub box_tl: char,
    pub box_tr: char,
    pub box_bl: char,
    pub box_br: char,

    // Effects
    pub explosion: char,
    pub sparkle: char,
    pub poison: char,
    pub target: char,
    pub projectile: char,
}

impl Default for GlyphSet {
    fn default() -> Self {
        UNICODE_GLYPHS
    }
}

impl GlyphSet {
    /// Create a new custom glyph set starting from Unicode defaults
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..UNICODE_GLYPHS
        }
    }

    /// Get glyph set by name
    pub fn by_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "ascii" => ASCII_GLYPHS,
            "unicode" => UNICODE_GLYPHS,
            "emoji" => EMOJI_GLYPHS,
            _ => UNICODE_GLYPHS,
        }
    }

    /// Check if this glyph set uses only ASCII characters
    pub fn is_ascii_compatible(&self) -> bool {
        self.floor.is_ascii()
            && self.wall.is_ascii()
            && self.player.is_ascii()
    }

    /// Get the glyph for a terrain type by name
    pub fn terrain(&self, name: &str) -> char {
        match name {
            "floor" => self.floor,
            "floor_alt" => self.floor_alt,
            "wall" => self.wall,
            "wall_h" => self.wall_h,
            "wall_v" => self.wall_v,
            "corner_tl" => self.corner_tl,
            "corner_tr" => self.corner_tr,
            "corner_bl" => self.corner_bl,
            "corner_br" => self.corner_br,
            "door_closed" => self.door_closed,
            "door_open" => self.door_open,
            "stairs_up" => self.stairs_up,
            "stairs_down" => self.stairs_down,
            "water" => self.water,
            "lava" => self.lava,
            "trap" => self.trap,
            "portal" => self.portal,
            "grass" => self.grass,
            "tree" => self.tree,
            "mountain" => self.mountain,
            "bridge" => self.bridge,
            "pit" => self.pit,
            "altar" => self.altar,
            "fountain" => self.fountain,
            "campfire" => self.campfire,
            "ice" => self.ice,
            "sand" => self.sand,
            "secret_door" => self.secret_door,
            _ => self.floor,
        }
    }

    /// Get the glyph for an entity type by name
    pub fn entity(&self, name: &str) -> char {
        match name {
            "player" => self.player,
            "goblin" => self.goblin,
            "orc" => self.orc,
            "skeleton" => self.skeleton,
            "zombie" => self.zombie,
            "dragon" => self.dragon,
            "demon" => self.demon,
            "boss" => self.boss,
            "npc" => self.npc,
            "merchant" => self.merchant,
            "companion" => self.companion,
            "rat" => self.rat,
            "bat" => self.bat,
            "spider" => self.spider,
            "snake" => self.snake,
            "ghost" => self.ghost,
            "vampire" => self.vampire,
            "werewolf" => self.werewolf,
            "lich" => self.lich,
            "troll" => self.troll,
            "golem" => self.golem,
            "slime" => self.slime,
            _ => '?',
        }
    }

    /// Get the glyph for an item type by name
    pub fn item(&self, name: &str) -> char {
        match name {
            "gold" => self.gold,
            "potion" => self.potion,
            "scroll" => self.scroll,
            "weapon" => self.weapon,
            "armor" => self.armor,
            "ring" => self.ring,
            "amulet" => self.amulet,
            "food" => self.food,
            "key" => self.key,
            "chest" => self.chest,
            "chest_locked" => self.chest_locked,
            "chest_open" => self.chest_open,
            "helmet" => self.helmet,
            "shield" => self.shield_item,
            "boots" => self.boots,
            "gloves" => self.gloves,
            "cloak" => self.cloak,
            "wand" => self.wand,
            "staff" => self.staff,
            "bow" => self.bow,
            "arrows" => self.arrows,
            "tome" => self.tome,
            "gem" => self.gem,
            "rune" => self.rune,
            _ => '?',
        }
    }

    /// Get a UI glyph by name
    pub fn ui(&self, name: &str) -> char {
        match name {
            "heart" => self.heart,
            "heart_empty" => self.heart_empty,
            "mana" => self.mana,
            "mana_empty" => self.mana_empty,
            "star" => self.star,
            "star_empty" => self.star_empty,
            "arrow_up" => self.arrow_up,
            "arrow_down" => self.arrow_down,
            "arrow_left" => self.arrow_left,
            "arrow_right" => self.arrow_right,
            "skull" => self.skull,
            "sword" => self.sword,
            "shield" => self.shield,
            "check" => self.check,
            "cross" => self.cross,
            "warning" => self.warning,
            "sun" => self.sun,
            "moon" => self.moon,
            "fire" => self.fire,
            "snowflake" => self.snowflake,
            "crown" => self.crown,
            _ => ' ',
        }
    }
}

// ============================================================================
// PRESET GLYPH SETS
// ============================================================================

/// ASCII-compatible glyph set for maximum terminal compatibility
pub const ASCII_GLYPHS: GlyphSet = GlyphSet {
    name: String::new(), // Will be set at runtime if needed

    // Terrain
    floor: '.',
    floor_alt: '.',
    wall: '#',
    wall_h: '-',
    wall_v: '|',
    corner_tl: '+',
    corner_tr: '+',
    corner_bl: '+',
    corner_br: '+',
    door_closed: '+',
    door_open: '/',
    stairs_up: '<',
    stairs_down: '>',
    water: '~',
    lava: '~',
    trap: '^',
    portal: 'O',
    grass: ',',
    tree: 'T',
    mountain: 'A',
    bridge: '=',
    pit: 'o',
    altar: '_',
    fountain: '{',
    campfire: '*',
    ice: '.',
    sand: ':',
    secret_door: '#',

    // Entities
    player: '@',
    goblin: 'g',
    orc: 'o',
    skeleton: 's',
    zombie: 'z',
    dragon: 'D',
    demon: '&',
    boss: 'B',
    npc: 'P',
    merchant: '$',
    companion: 'c',
    rat: 'r',
    bat: 'b',
    spider: 'x',
    snake: 'S',
    ghost: 'G',
    vampire: 'V',
    werewolf: 'W',
    lich: 'L',
    troll: 'T',
    golem: 'Y',
    slime: 'j',

    // Items
    gold: '*',
    potion: '!',
    scroll: '?',
    weapon: '/',
    armor: '[',
    ring: '=',
    amulet: '"',
    food: '%',
    key: 'k',
    chest: '#',
    chest_locked: '#',
    chest_open: '_',
    helmet: ']',
    shield_item: ')',
    boots: '\\',
    gloves: '(',
    cloak: '`',
    wand: '-',
    staff: '|',
    bow: '}',
    arrows: '{',
    tome: '+',
    gem: '*',
    rune: '*',

    // UI
    heart: '<',
    heart_empty: '>',
    mana: '*',
    mana_empty: '.',
    star: '*',
    star_empty: '.',
    arrow_up: '^',
    arrow_down: 'v',
    arrow_left: '<',
    arrow_right: '>',
    skull: 'X',
    sword: '/',
    shield: 'O',
    check: '+',
    cross: 'x',
    warning: '!',
    sun: 'O',
    moon: 'C',
    fire: '*',
    snowflake: '*',
    crown: '^',

    // Box drawing
    box_h: '-',
    box_v: '|',
    box_tl: '+',
    box_tr: '+',
    box_bl: '+',
    box_br: '+',

    // Effects
    explosion: '*',
    sparkle: '*',
    poison: '+',
    target: 'X',
    projectile: '.',
};

/// Unicode glyph set with enhanced visual symbols
pub const UNICODE_GLYPHS: GlyphSet = GlyphSet {
    name: String::new(),

    // Terrain
    floor: '\u{00B7}',      // ·
    floor_alt: '.',
    wall: '\u{2588}',       // █
    wall_h: '\u{2500}',     // ─
    wall_v: '\u{2502}',     // │
    corner_tl: '\u{250C}',  // ┌
    corner_tr: '\u{2510}',  // ┐
    corner_bl: '\u{2514}',  // └
    corner_br: '\u{2518}',  // ┘
    door_closed: '+',
    door_open: '\u{256C}',  // ╬
    stairs_up: '<',
    stairs_down: '>',
    water: '\u{2248}',      // ≈
    lava: '~',
    trap: '^',
    portal: 'O',
    grass: ',',
    tree: '\u{2663}',       // ♣
    mountain: '\u{25B2}',   // ▲
    bridge: '=',
    pit: '\u{25CB}',        // ○
    altar: '\u{2021}',      // ‡
    fountain: '\u{00A7}',   // §
    campfire: '\u{00A4}',   // ¤
    ice: '\u{2022}',        // •
    sand: ':',
    secret_door: '#',

    // Entities
    player: '@',
    goblin: 'g',
    orc: 'o',
    skeleton: 's',
    zombie: 'z',
    dragon: 'D',
    demon: '&',
    boss: 'B',
    npc: 'P',
    merchant: '$',
    companion: 'c',
    rat: 'r',
    bat: 'b',
    spider: 'x',
    snake: 'S',
    ghost: 'G',
    vampire: 'V',
    werewolf: 'W',
    lich: 'L',
    troll: 'T',
    golem: 'Y',
    slime: 'j',

    // Items
    gold: '*',
    potion: '!',
    scroll: '?',
    weapon: '/',
    armor: '[',
    ring: '=',
    amulet: '"',
    food: '%',
    key: 'k',
    chest: '\u{25A1}',      // □
    chest_locked: '\u{25A0}', // ■
    chest_open: '\u{25A2}', // ▢
    helmet: ']',
    shield_item: ')',
    boots: '\\',
    gloves: '(',
    cloak: '`',
    wand: '\u{00AC}',       // ¬
    staff: '|',
    bow: '}',
    arrows: '{',
    tome: '\u{00B6}',       // ¶
    gem: '\u{25C6}',        // ◆
    rune: '\u{00A5}',       // ¥

    // UI
    heart: '\u{2665}',      // ♥
    heart_empty: '\u{2661}', // ♡
    mana: '\u{25C6}',       // ◆
    mana_empty: '\u{25C7}', // ◇
    star: '\u{2605}',       // ★
    star_empty: '\u{2606}', // ☆
    arrow_up: '\u{2191}',   // ↑
    arrow_down: '\u{2193}', // ↓
    arrow_left: '\u{2190}', // ←
    arrow_right: '\u{2192}', // →
    skull: '\u{2620}',      // ☠
    sword: '\u{2020}',      // †
    shield: '\u{25CA}',     // ◊
    check: '\u{2713}',      // ✓
    cross: '\u{2717}',      // ✗
    warning: '\u{26A0}',    // ⚠
    sun: '\u{263C}',        // ☼
    moon: '\u{263E}',       // ☾
    fire: '\u{2668}',       // ♨
    snowflake: '\u{2744}',  // ❄
    crown: '\u{265B}',      // ♛

    // Box drawing
    box_h: '\u{2550}',      // ═
    box_v: '\u{2551}',      // ║
    box_tl: '\u{2554}',     // ╔
    box_tr: '\u{2557}',     // ╗
    box_bl: '\u{255A}',     // ╚
    box_br: '\u{255D}',     // ╝

    // Effects
    explosion: '\u{2731}',  // ✱
    sparkle: '\u{2728}',    // ✨
    poison: '\u{2623}',     // ☣
    target: '\u{25CE}',     // ◎
    projectile: '\u{2219}', // ∙
};

/// Emoji glyph set for modern terminal support
pub const EMOJI_GLYPHS: GlyphSet = GlyphSet {
    name: String::new(),

    // Terrain - Using emoji where appropriate, falling back to Unicode
    floor: '\u{00B7}',      // · (no good emoji)
    floor_alt: '.',
    wall: '\u{2588}',       // █
    wall_h: '\u{2500}',     // ─
    wall_v: '\u{2502}',     // │
    corner_tl: '\u{250C}',  // ┌
    corner_tr: '\u{2510}',  // ┐
    corner_bl: '\u{2514}',  // └
    corner_br: '\u{2518}',  // ┘
    door_closed: '\u{1F6AA}', // Note: This is a multi-byte emoji, may cause issues
    door_open: '\u{256C}',  // ╬
    stairs_up: '<',
    stairs_down: '>',
    water: '\u{2248}',      // ≈
    lava: '~',
    trap: '^',
    portal: 'O',
    grass: ',',
    tree: '\u{2663}',       // ♣
    mountain: '\u{25B2}',   // ▲
    bridge: '=',
    pit: '\u{25CB}',        // ○
    altar: '\u{2021}',      // ‡
    fountain: '\u{00A7}',   // §
    campfire: '\u{00A4}',   // ¤
    ice: '\u{2022}',        // •
    sand: ':',
    secret_door: '#',

    // Entities
    player: '@',
    goblin: 'g',
    orc: 'o',
    skeleton: 's',
    zombie: 'z',
    dragon: 'D',
    demon: '&',
    boss: 'B',
    npc: 'P',
    merchant: '$',
    companion: 'c',
    rat: 'r',
    bat: 'b',
    spider: 'x',
    snake: 'S',
    ghost: 'G',
    vampire: 'V',
    werewolf: 'W',
    lich: 'L',
    troll: 'T',
    golem: 'Y',
    slime: 'j',

    // Items
    gold: '*',
    potion: '!',
    scroll: '?',
    weapon: '/',
    armor: '[',
    ring: '=',
    amulet: '"',
    food: '%',
    key: 'k',
    chest: '\u{25A1}',      // □
    chest_locked: '\u{25A0}', // ■
    chest_open: '\u{25A2}', // ▢
    helmet: ']',
    shield_item: ')',
    boots: '\\',
    gloves: '(',
    cloak: '`',
    wand: '\u{00AC}',       // ¬
    staff: '|',
    bow: '}',
    arrows: '{',
    tome: '\u{00B6}',       // ¶
    gem: '\u{25C6}',        // ◆
    rune: '\u{00A5}',       // ¥

    // UI - Emoji versions where available
    heart: '\u{2764}',      // ❤ (red heart)
    heart_empty: '\u{2661}', // ♡
    mana: '\u{25C6}',       // ◆
    mana_empty: '\u{25C7}', // ◇
    star: '\u{2B50}',       // ⭐ (may be emoji)
    star_empty: '\u{2606}', // ☆
    arrow_up: '\u{2191}',   // ↑
    arrow_down: '\u{2193}', // ↓
    arrow_left: '\u{2190}', // ←
    arrow_right: '\u{2192}', // →
    skull: '\u{2620}',      // ☠
    sword: '\u{2694}',      // ⚔ (crossed swords)
    shield: '\u{1F6E1}',    // (shield emoji)
    check: '\u{2705}',      // ✅
    cross: '\u{274C}',      // ❌
    warning: '\u{26A0}',    // ⚠
    sun: '\u{2600}',        // ☀
    moon: '\u{1F319}',      // (crescent moon)
    fire: '\u{1F525}',      // (fire emoji)
    snowflake: '\u{2744}',  // ❄
    crown: '\u{1F451}',     // (crown emoji)

    // Box drawing (same as Unicode)
    box_h: '\u{2550}',      // ═
    box_v: '\u{2551}',      // ║
    box_tl: '\u{2554}',     // ╔
    box_tr: '\u{2557}',     // ╗
    box_bl: '\u{255A}',     // ╚
    box_br: '\u{255D}',     // ╝

    // Effects
    explosion: '\u{1F4A5}', // (collision/explosion emoji)
    sparkle: '\u{2728}',    // ✨
    poison: '\u{2623}',     // ☣
    target: '\u{1F3AF}',    // (target emoji)
    projectile: '\u{2219}', // ∙
};

// ============================================================================
// GLYPH REGISTRY
// ============================================================================

/// Enumeration of available glyph set types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GlyphSetType {
    /// Basic ASCII for maximum compatibility
    Ascii,
    /// Enhanced Unicode symbols
    Unicode,
    /// Emoji characters for modern terminals
    Emoji,
    /// Custom user-defined set
    Custom,
}

impl GlyphSetType {
    /// Get the glyph set for this type
    pub fn get_set(&self) -> GlyphSet {
        match self {
            GlyphSetType::Ascii => ASCII_GLYPHS,
            GlyphSetType::Unicode => UNICODE_GLYPHS,
            GlyphSetType::Emoji => EMOJI_GLYPHS,
            GlyphSetType::Custom => UNICODE_GLYPHS, // Default to Unicode for custom
        }
    }

    /// Get all available glyph set types
    pub fn all() -> &'static [GlyphSetType] {
        &[
            GlyphSetType::Ascii,
            GlyphSetType::Unicode,
            GlyphSetType::Emoji,
        ]
    }
}

impl std::fmt::Display for GlyphSetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GlyphSetType::Ascii => write!(f, "ASCII"),
            GlyphSetType::Unicode => write!(f, "Unicode"),
            GlyphSetType::Emoji => write!(f, "Emoji"),
            GlyphSetType::Custom => write!(f, "Custom"),
        }
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Convert a character to its display width (accounting for wide characters)
pub fn char_width(c: char) -> usize {
    // Most characters are width 1, but some Unicode/emoji are width 2
    if c.is_ascii() {
        1
    } else {
        // Simplified: assume most non-ASCII are width 1, emoji are width 2
        match c as u32 {
            // Emoji ranges (very simplified)
            0x1F300..=0x1F9FF => 2,
            // CJK characters
            0x4E00..=0x9FFF => 2,
            0x3000..=0x303F => 2,
            // Default
            _ => 1,
        }
    }
}

/// Check if a character is likely to render correctly in most terminals
pub fn is_safe_glyph(c: char) -> bool {
    c.is_ascii() || matches!(c as u32, 0x00A0..=0x00FF | 0x2500..=0x257F | 0x2580..=0x259F | 0x25A0..=0x25FF | 0x2600..=0x26FF)
}

/// Get a fallback ASCII character for a Unicode glyph
pub fn ascii_fallback(c: char) -> char {
    if c.is_ascii() {
        return c;
    }

    match c {
        '\u{00B7}' | '\u{2022}' => '.', // · •
        '\u{2588}' | '\u{2591}' | '\u{2592}' | '\u{2593}' => '#', // █ ░ ▒ ▓
        '\u{2500}' | '\u{2550}' => '-', // ─ ═
        '\u{2502}' | '\u{2551}' => '|', // │ ║
        '\u{250C}' | '\u{2554}' | '\u{2510}' | '\u{2557}' |
        '\u{2514}' | '\u{255A}' | '\u{2518}' | '\u{255D}' => '+', // corners
        '\u{256C}' => '#', // ╬
        '\u{2248}' => '~', // ≈
        '\u{25A0}' | '\u{25A1}' | '\u{25A2}' => '#', // ■ □ ▢
        '\u{2665}' | '\u{2661}' | '\u{2764}' => '<', // ♥ ♡ ❤
        '\u{25C6}' | '\u{25C7}' => '*', // ◆ ◇
        '\u{2605}' | '\u{2606}' | '\u{2B50}' => '*', // ★ ☆ ⭐
        '\u{2191}' => '^', // ↑
        '\u{2193}' => 'v', // ↓
        '\u{2190}' => '<', // ←
        '\u{2192}' => '>', // →
        '\u{2620}' => 'X', // ☠
        '\u{2020}' | '\u{2694}' => '/', // † ⚔
        '\u{25CA}' | '\u{1F6E1}' => 'O', // ◊
        '\u{2713}' | '\u{2705}' => '+', // ✓ ✅
        '\u{2717}' | '\u{274C}' => 'x', // ✗ ❌
        '\u{26A0}' => '!', // ⚠
        '\u{263C}' | '\u{2600}' => 'O', // ☼ ☀
        '\u{263E}' | '\u{1F319}' => 'C', // ☾
        '\u{2663}' => 'T', // ♣
        '\u{25B2}' => 'A', // ▲
        '\u{25CB}' => 'o', // ○
        '\u{2021}' => '_', // ‡
        '\u{00A7}' => '{', // §
        '\u{00A4}' | '\u{2668}' | '\u{1F525}' => '*', // ¤ ♨
        '\u{00AC}' => '-', // ¬
        '\u{00B6}' => '+', // ¶
        '\u{00A5}' => '*', // ¥
        '\u{2731}' | '\u{1F4A5}' => '*', // ✱
        '\u{2728}' => '*', // ✨
        '\u{2623}' => '+', // ☣
        '\u{25CE}' | '\u{1F3AF}' => 'X', // ◎
        '\u{2219}' => '.', // ∙
        '\u{2744}' => '*', // ❄
        '\u{265B}' | '\u{1F451}' => '^', // ♛
        _ => '?',
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glyph_constants() {
        assert_eq!(PLAYER, '@');
        assert_eq!(WALL, '\u{2588}');
        assert_eq!(FLOOR, '\u{00B7}');
        assert_eq!(STAIRS_UP, '<');
        assert_eq!(STAIRS_DOWN, '>');
    }

    #[test]
    fn test_ascii_glyphs_are_ascii() {
        assert!(ASCII_GLYPHS.is_ascii_compatible());
        assert!(ASCII_GLYPHS.floor.is_ascii());
        assert!(ASCII_GLYPHS.wall.is_ascii());
        assert!(ASCII_GLYPHS.player.is_ascii());
    }

    #[test]
    fn test_glyph_set_by_name() {
        let ascii = GlyphSet::by_name("ascii");
        assert!(ascii.is_ascii_compatible());

        let unicode = GlyphSet::by_name("unicode");
        assert!(!unicode.is_ascii_compatible());

        let unknown = GlyphSet::by_name("unknown");
        assert_eq!(unknown.player, UNICODE_GLYPHS.player);
    }

    #[test]
    fn test_terrain_lookup() {
        let glyphs = UNICODE_GLYPHS;
        assert_eq!(glyphs.terrain("floor"), glyphs.floor);
        assert_eq!(glyphs.terrain("wall"), glyphs.wall);
        assert_eq!(glyphs.terrain("unknown"), glyphs.floor);
    }

    #[test]
    fn test_entity_lookup() {
        let glyphs = UNICODE_GLYPHS;
        assert_eq!(glyphs.entity("player"), '@');
        assert_eq!(glyphs.entity("dragon"), 'D');
        assert_eq!(glyphs.entity("unknown"), '?');
    }

    #[test]
    fn test_item_lookup() {
        let glyphs = UNICODE_GLYPHS;
        assert_eq!(glyphs.item("gold"), '*');
        assert_eq!(glyphs.item("potion"), '!');
        assert_eq!(glyphs.item("unknown"), '?');
    }

    #[test]
    fn test_ui_lookup() {
        let glyphs = UNICODE_GLYPHS;
        assert_eq!(glyphs.ui("heart"), '\u{2665}');
        assert_eq!(glyphs.ui("skull"), '\u{2620}');
        assert_eq!(glyphs.ui("unknown"), ' ');
    }

    #[test]
    fn test_ascii_fallback() {
        assert_eq!(ascii_fallback('\u{2588}'), '#');
        assert_eq!(ascii_fallback('\u{2665}'), '<');
        assert_eq!(ascii_fallback('@'), '@');
        assert_eq!(ascii_fallback('a'), 'a');
    }

    #[test]
    fn test_char_width() {
        assert_eq!(char_width('@'), 1);
        assert_eq!(char_width('.'), 1);
        assert_eq!(char_width('\u{2588}'), 1);
    }

    #[test]
    fn test_is_safe_glyph() {
        assert!(is_safe_glyph('@'));
        assert!(is_safe_glyph('.'));
        assert!(is_safe_glyph('\u{2588}')); // Box drawing
        assert!(is_safe_glyph('\u{2665}')); // Heart
    }

    #[test]
    fn test_glyph_set_type() {
        assert_eq!(GlyphSetType::Ascii.get_set().floor, '.');
        assert_eq!(GlyphSetType::Unicode.get_set().floor, '\u{00B7}');
        assert_eq!(GlyphSetType::all().len(), 3);
    }
}
