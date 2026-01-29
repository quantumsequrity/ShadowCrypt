//! ASCII Art Module for ShadowCrypt
//!
//! This module contains all ASCII art assets for the game including:
//! - Title screen
//! - Class selection art
//! - Boss portraits
//! - Game over and victory screens
//! - Loading animations
//! - Item icons

use crossterm::style::Color;

// ============================================================================
// TITLE SCREEN
// ============================================================================

/// The main title screen ASCII art
pub const TITLE_ART: &str = r#"
   _____ _               _                _____                  _
  / ____| |             | |              / ____|                | |
 | (___ | |__   __ _  __| | _____      _| |     _ __ _   _ _ __ | |_
  \___ \| '_ \ / _` |/ _` |/ _ \ \ /\ / / |    | '__| | | | '_ \| __|
  ____) | | | | (_| | (_| | (_) \ V  V /| |____| |  | |_| | |_) | |_
 |_____/|_| |_|\__,_|\__,_|\___/ \_/\_/  \_____|_|   \__, | .__/ \__|
                                                      __/ | |
                                                     |___/|_|
"#;

/// Subtitle for the title screen
pub const TITLE_SUBTITLE: &str = r#"
                    -=- A Roguelike Adventure -=-

           [ Press ENTER to Begin Your Journey ]
           [      Press Q to Quit the Game     ]
"#;

/// Decorative border for title screen
pub const TITLE_BORDER_TOP: &str = r#"
    *  .  *       .   *   .    *    .   *   .      *
  .    *    .  *    .   *  .    *    .  *    .   *   .
 ========================================================
"#;

pub const TITLE_BORDER_BOTTOM: &str = r#"
 ========================================================
  .    *    .  *    .   *  .    *    .  *    .   *   .
    *  .  *       .   *   .    *    .   *   .      *
"#;

// ============================================================================
// CLASS SELECTION ART
// ============================================================================

/// Warrior class ASCII art - Knight with sword and shield
pub const WARRIOR_ART: &str = r#"
           /\
          /  \
         /    \
        /______\
          |  |
      ___/|  |\___
     /   ||  ||   \
    /    ||  ||    \
   /_____|    |_____\
         |    |
         |    |
      [  |    |  ]
      [__|    |__]
        /|    |\
       / |    | \
      /__|    |__\
         |====|
        /|    |\
       / |    | \
      /  |    |  \
     [___|    |___]
         |    |
        /      \
       /________\
"#;

/// Warrior with sword and shield (compact version)
pub const WARRIOR_ART_COMPACT: &str = r#"
       ,^.
      / _ \
     | / \ |
    /| === |\
   | |     | |
   |_|     |_|___
     |     |  ___)
     |     | |
    /|     |\|
   | |     | |
   |_|_____|_|
"#;

/// Mage class ASCII art - Wizard with staff
pub const MAGE_ART: &str = r#"
                *
               /|\
              /*|*\
             / *|* \
            /  *|*  \
           /___*|*___\
               |||
       ___    / | \    ___
      /   \  /  |  \  /   \
     |  *  ||   |   ||  *  |
      \___/ |   |   | \___/
            |   |   |
       ~~~~~/   |   \~~~~~
      /   */    |    \*   \
     / * /      |      \ * \
    (__*/       |       \*__)
               /|\
              / | \
             /  |  \
            /___|___\
                |
               /|\
              / | \
             [__|__]
"#;

/// Mage compact version
pub const MAGE_ART_COMPACT: &str = r#"
       *
      /|\
     /*|*\
       |
    ~~/|~~
   /  / \  \
  |  |   |  |
   \_|   |_/
     |   |
    [|___|]
"#;

/// Rogue class ASCII art - Assassin with daggers
pub const ROGUE_ART: &str = r#"
            ___
         .-'   '-.
        /  ^   ^  \
       |  (o) (o)  |
        \    <    /
         '-.__.-'
            |
       \    |    /
        \   |   /
    _.-- |  |  | --._
   /   .'|     |'.   \
  (   /  |     |  \   )
   '-|   |     |   |-'
     |   |     |   |
     /\  |     |  /\
    /  \ |     | /  \
   <\   \|     |/   />
    \\   |     |   //
     \\__|     |__//
       /|       |\
      / |       | \
     <__|       |__>
"#;

/// Rogue compact version
pub const ROGUE_ART_COMPACT: &str = r#"
     .---.
    / o o \
    \  <  /
   \_'---'_/
 _/|  |  |\_
<  |  |  |  >
 \_|__|__|_/
"#;

/// Cleric class ASCII art - Holy warrior with divine symbol
pub const CLERIC_ART: &str = r#"
           .*****.
          *       *
         *    +    *
        *     |     *
         *    |    *
          *   |   *
           '**|**'
              |
           ___|___
          |       |
         _|_     _|_
        |   |   |   |
        |   |___|   |
        |     |     |
        |    /|\    |
       /    / | \    \
      /    /  |  \    \
     |    |   |   |    |
     |____|   |   |____|
          |   |   |
          |___|___|
            / | \
           /  |  \
          [___|___]
"#;

/// Cleric compact version
pub const CLERIC_ART_COMPACT: &str = r#"
    .+.
   * + *
    '|'
   __|__
  |     |
  |  |  |
   \/_\/
    |__|
"#;

/// Ranger class ASCII art - Archer with bow
pub const RANGER_ART: &str = r#"
              ___
           .-'   '-.
          /  ^   ^  \
         |    (_)    |
          \   ___   /
           '-|   |-'
         ____|   |____
        /    |   |    \
       |  /| |   | |\  |
       | / | |   | | \ |
       |/  \ |   | /  \|
           _\|   |/_
       _.-'  |   |  '-._
      (     /     \     )
       '--./   ^   \.--'
          |   /|\   |
         /   / | \   \
        /   /  |  \   \
       /___/   |   \___\
              /|\
             / | \
            /  |  \
           /___|___\
"#;

/// Ranger compact version
pub const RANGER_ART_COMPACT: &str = r#"
    .---.
   / o_o \
   \  ^  /
  __|   |__
 )  \   /  (
 `-. | | .-'
   _|| ||_
  (__| |__)
"#;

/// Monk class ASCII art - Martial artist in fighting pose
pub const MONK_ART: &str = r#"
              ___
           .-'   '-.
          /  -   -  \
         |    (_)    |
          \  '---'  /
           '-.___.--'
              |||
          ____|_|____
         /     |     \
        /   _--|--_   \
       /  _/   |   \_  \
      |  /     |     \  |
      | |      |      | |
       \|      |      |/
       /\______|______/\
      /                 \
     |   /__       __\   |
     |  |   |     |   |  |
      \_|   |     |   |_/
        |___|     |___|
        /   \     /   \
       /     \   /     \
      [_______] [_______]
"#;

/// Monk compact version
pub const MONK_ART_COMPACT: &str = r#"
    .---.
   /  _  \
   \_/ \_/
  __|   |__
 /  \   /  \
 \__|   |__/
   /|   |\
  (_|   |_)
"#;

// ============================================================================
// BOSS ART
// ============================================================================

/// Dragon boss ASCII art
pub const DRAGON_ART: &str = r#"
                 __                  __
                ( o>              <( o)
               /.||\\            //||.\
              / / || \\        // || \ \
             /  | || | \\    // | ||  |  \
           _/   | || |   \\//   | ||  |   \_
          /     | || |   //\\   | ||  |     \
         /      | || |  //  \\  | ||  |      \
        /       |_||_| //    \\ |_||__|       \
       /         \__///__    __\\\__/          \
      <____        \\||//    \\||//        ____>
           --------//__\\----//__\\--------
                  <_\  /_>  <_\  /_>
                   '\\||//    \\||//'
                    \\||/      \||//
                     \|/        \|/
                      V   @@@    V
                         @@@@@
                    ~ ~ ~@@@@@~ ~ ~
                    ~  ~  @@@  ~  ~
                       ~ ~ @ ~ ~
"#;

/// Dragon compact portrait
pub const DRAGON_ART_COMPACT: &str = r#"
   /\   /\
  <  '-'  >
   \ 0 0 /
    \   /
   _|^^^|_
  / ~~~~~ \
  \_/\_/\_/
"#;

/// Demon Lord boss ASCII art
pub const DEMON_LORD_ART: &str = r#"
           |\                    /|
           | \                  / |
          /|  \      __      /  |\
         / |   \   .'  '.   /   | \
        |  |    \ /  ()  \ /    |  |
        |   \    |   <>   |    /   |
        |    \   |   /\   |   /    |
         \    '--'  |  |  '--'    /
          \     .--'    '--.     /
           \   /   \  /   \   /
            \ |  O  ||  O  | /
             \|     ||     |/
              |  \__/\__/  |
              |     \/     |
              |    /__\    |
             /|   /    \   |\
            / |  | [==] |  | \
           |  | /        \ |  |
           |__|/    /\    \|__|
              |    /  \    |
              |___/    \___|
             /    \    /    \
            /______\  /______\
"#;

/// Demon Lord compact portrait
pub const DEMON_LORD_ART_COMPACT: &str = r#"
  |\    /|
  | \()/ |
   \ <> /
   |_/\_|
  /| [] |\
 |_|    |_|
"#;

/// Lich boss ASCII art
pub const LICH_ART: &str = r#"
              .---.
             /     \
            | () () |
            |   ^   |
            |  ===  |
             \_____/
           .'|     |'.
          /  |     |  \
         |   |     |   |
        /    |     |    \
       |  .--|     |--.  |
       | /   |_____|   \ |
       |/    |     |    \|
            /|     |\
           / |     | \
          /  |_____|  \
         /   |     |   \
        /   /|     |\   \
       /___/ |     | \___\
            /       \
           /_________\
"#;

/// Lich compact portrait
pub const LICH_ART_COMPACT: &str = r#"
   .---.
  | x x |
  |  ^  |
  |_===_|
  /|   |\
 |_|   |_|
"#;

/// Giant boss ASCII art
pub const GIANT_ART: &str = r#"
                  ____
               .-'    '-.
              /  ^    ^  \
             |    (____)   |
             |     ____    |
              \   '----'  /
               '-._    _.-'
             _____|    |_____
            /     |    |     \
           /      |    |      \
          |    ___|    |___    |
          |   |   |    |   |   |
          |   |   |    |   |   |
          |   |   |____|   |   |
          |   |            |   |
          |   |            |   |
         /    |            |    \
        /     |            |     \
       /      |            |      \
      |       |            |       |
      |_______|            |_______|
      |       |            |       |
      |       |            |       |
      |_______|            |_______|
       /      \            /      \
      /________\          /________\
"#;

/// Giant compact portrait
pub const GIANT_ART_COMPACT: &str = r#"
   .---.
  / o o \
  | __  |
  \_||_/
  _|  |_
 | |  | |
 |_|  |_|
"#;

// ============================================================================
// GAME OVER SCREEN
// ============================================================================

/// Game Over ASCII art
pub const GAME_OVER_ART: &str = r#"
   ▄████  ▄▄▄       ███▄ ▄███▓▓█████     ▒█████   ██▒   █▓▓█████  ██▀███
  ██▒ ▀█▒▒████▄    ▓██▒▀█▀ ██▒▓█   ▀    ▒██▒  ██▒▓██░   █▒▓█   ▀ ▓██ ▒ ██▒
 ▒██░▄▄▄░▒██  ▀█▄  ▓██    ▓██░▒███      ▒██░  ██▒ ▓██  █▒░▒███   ▓██ ░▄█ ▒
 ░▓█  ██▓░██▄▄▄▄██ ▒██    ▒██ ▒▓█  ▄    ▒██   ██░  ▒██ █░░▒▓█  ▄ ▒██▀▀█▄
 ░▒▓███▀▒ ▓█   ▓██▒▒██▒   ░██▒░▒████▒   ░ ████▓▒░   ▒▀█░  ░▒████▒░██▓ ▒██▒
  ░▒   ▒  ▒▒   ▓▒█░░ ▒░   ░  ░░░ ▒░ ░   ░ ▒░▒░▒░    ░ ▐░  ░░ ▒░ ░░ ▒▓ ░▒▓░
   ░   ░   ▒   ▒▒ ░░  ░      ░ ░ ░  ░     ░ ▒ ▒░    ░ ░░   ░ ░  ░  ░▒ ░ ▒░
"#;

/// Alternative simpler Game Over art
pub const GAME_OVER_ART_SIMPLE: &str = r#"
  ____    _    __  __ _____    _____     _______ ____
 / ___|  / \  |  \/  | ____|  / _ \ \   / / ____|  _ \
| |  _  / _ \ | |\/| |  _|   | | | \ \ / /|  _| | |_) |
| |_| |/ ___ \| |  | | |___  | |_| |\ V / | |___|  _ <
 \____/_/   \_\_|  |_|_____|  \___/  \_/  |_____|_| \_\
"#;

/// Skull decoration for game over
pub const SKULL_ART: &str = r#"
        ___
     .-'   '-.
    /         \
   |  O     O  |
   |     ^     |
   |   '---'   |
    \  _____  /
     '-------'
       | | |
      /| | |\
     /_|_|_|_\
"#;

/// RIP tombstone
pub const TOMBSTONE_ART: &str = r#"
       .---.
      /     \
     |  R.I.P |
     |       |
     | HERE  |
     | LIES  |
     |  A    |
     | HERO  |
     |       |
    _|_______|_
   |___________|
"#;

// ============================================================================
// VICTORY SCREEN
// ============================================================================

/// Victory ASCII art
pub const VICTORY_ART: &str = r#"
 __      __ _____   _____  _______   ____   _______     __
 \ \    / /|_   _| / ____||__   __| / __ \ |  __ \ \   / /
  \ \  / /   | |  | |        | |   | |  | || |__) \ \_/ /
   \ \/ /    | |  | |        | |   | |  | ||  _  / \   /
    \  /    _| |_ | |____    | |   | |__| || | \ \  | |
     \/    |_____| \_____|   |_|    \____/ |_|  \_\ |_|
"#;

/// Trophy art for victory
pub const TROPHY_ART: &str = r#"
           ___________
          '._==_==_=_.'
          .-\:      /-.
         | (|:.     |) |
          '-|:.     |-'
            \::.    /
             '::. .'
               ) (
             _.' '._
            '-------'
"#;

/// Crown art for victory
pub const CROWN_ART: &str = r#"
       *   *   *   *   *
      /|\ /|\ /|\ /|\ /|\
     / | X | X | X | X | \
    /  |/ \|/ \|/ \|/ \|  \
   /   '---'---'---'---'   \
  /                         \
 /___________________________\
           |     |
           |_____|
"#;

/// Celebratory fireworks
pub const FIREWORKS_ART: &str = r#"
     *  .  *      *        *   .    *
  .    * .   *  .    *  .   *    . *   .
    *     .     *     .   *   .      *
  .   *     '     .  *    .  *    .   *
     .  *    *   . *    *    .  *
   *    .  *   .    *  . *     *    .
      *      *   .   *     *  .    *
   .     *  .  *   .   *  .     *    .
"#;

// ============================================================================
// LOADING ANIMATION FRAMES
// ============================================================================

/// Spinning sword animation frames
pub const LOADING_SWORD_FRAMES: [&str; 8] = [
    r#"
    |
    |
   /|\
    |
    "
"#,
    r#"
     /
    /
   /
  /
 /
"#,
    r#"
_____
"#,
    r#"
\
 \
  \
   \
    \
"#,
    r#"
    |
    |
   \|/
    |
    "
"#,
    r#"
    /
   /
  /
 /
/
"#,
    r#"
_____
"#,
    r#"
 \
  \
   \
    \
     \
"#,
];

/// Pulsing orb animation frames
pub const LOADING_ORB_FRAMES: [&str; 8] = [
    r#"
   .
  (.)
   '
"#,
    r#"
  .-.
 ( . )
  '-'
"#,
    r#"
  .---.
 (  *  )
  '---'
"#,
    r#"
  .-----.
 (   *   )
  '-----'
"#,
    r#"
  .-------.
 (    *    )
  '-------'
"#,
    r#"
  .-----.
 (   *   )
  '-----'
"#,
    r#"
  .---.
 (  *  )
  '---'
"#,
    r#"
  .-.
 ( . )
  '-'
"#,
];

/// Simple spinner animation
pub const LOADING_SPINNER_FRAMES: [&str; 4] = ["|", "/", "-", "\\"];

/// Dots loading animation
pub const LOADING_DOTS_FRAMES: [&str; 4] = [".   ", "..  ", "... ", "...."];

// ============================================================================
// ITEM ICONS (5x5 or smaller)
// ============================================================================

/// Sword icon (5 lines)
pub const ICON_SWORD: &str = r#"
    /\
   /  \
   |  |
   |__|
  /|  |\
"#;

/// Shield icon (5 lines)
pub const ICON_SHIELD: &str = r#"
 .-----.
 | .-. |
 | |+| |
 | '-' |
  '---'
"#;

/// Potion icon (5 lines)
pub const ICON_POTION: &str = r#"
   __
  |==|
 /    \
 |~~~~|
 '----'
"#;

/// Scroll icon (5 lines)
pub const ICON_SCROLL: &str = r#"
 .===.
 |   |
 | ~ |
 |   |
 '==='
"#;

/// Ring icon (3 lines)
pub const ICON_RING: &str = r#"
  o
 (*)
  o
"#;

/// Key icon (3 lines)
pub const ICON_KEY: &str = r#"
 o--.
 |  |
 '--'
"#;

/// Gold/coin icon (3 lines)
pub const ICON_GOLD: &str = r#"
 .--.
 |$$|
 '--'
"#;

/// Helmet icon (5 lines)
pub const ICON_HELMET: &str = r#"
  ___
 /   \
 |===|
 | ^ |
 '---'
"#;

/// Bow icon (5 lines)
pub const ICON_BOW: &str = r#"
   )
  /|
 / |
  \|
   )
"#;

/// Staff icon (5 lines)
pub const ICON_STAFF: &str = r#"
  *
 /|\
  |
  |
  |
"#;

// ============================================================================
// DUNGEON/ENVIRONMENT ART
// ============================================================================

/// Dungeon entrance art
pub const DUNGEON_ENTRANCE: &str = r#"
        _____________________
       /                     \
      /   _______________     \
     |   |               |     |
     |   |    DUNGEON    |     |
     |   |   =========   |     |
     |   |               |     |
     |   |       _       |     |
    _|___|______| |______|_____|_
   |_____________________________|
        |__|         |__|
"#;

/// Stairs down art
pub const STAIRS_DOWN: &str = r#"
    _____
   |     |
   |  ___|
   | |___
   |_____|
     >>>
"#;

/// Stairs up art
pub const STAIRS_UP: &str = r#"
     <<<
    _____
   |___  |
   |___| |
   |     |
   |_____|
"#;

/// Treasure chest art
pub const TREASURE_CHEST: &str = r#"
   _______
  |   $   |
  |_______|
  |[=====]|
  |_______|
"#;

// ============================================================================
// COLOR CONFIGURATIONS
// ============================================================================

/// Color scheme for title screen
pub struct TitleColors {
    pub main_text: Color,
    pub subtitle: Color,
    pub border: Color,
    pub stars: Color,
}

impl Default for TitleColors {
    fn default() -> Self {
        Self {
            main_text: Color::Cyan,
            subtitle: Color::Yellow,
            border: Color::DarkGrey,
            stars: Color::White,
        }
    }
}

/// Color scheme for class art
pub struct ClassColors {
    pub warrior: Color,
    pub mage: Color,
    pub rogue: Color,
    pub cleric: Color,
    pub ranger: Color,
    pub monk: Color,
}

impl Default for ClassColors {
    fn default() -> Self {
        Self {
            warrior: Color::Red,
            mage: Color::Blue,
            rogue: Color::DarkGrey,
            cleric: Color::Yellow,
            ranger: Color::Green,
            monk: Color::Magenta,
        }
    }
}

/// Color scheme for boss art
pub struct BossColors {
    pub dragon: Color,
    pub demon_lord: Color,
    pub lich: Color,
    pub giant: Color,
}

impl Default for BossColors {
    fn default() -> Self {
        Self {
            dragon: Color::Red,
            demon_lord: Color::DarkRed,
            lich: Color::Magenta,
            giant: Color::DarkYellow,
        }
    }
}

/// Color scheme for game over
pub struct GameOverColors {
    pub text: Color,
    pub skull: Color,
    pub tombstone: Color,
}

impl Default for GameOverColors {
    fn default() -> Self {
        Self {
            text: Color::DarkRed,
            skull: Color::White,
            tombstone: Color::Grey,
        }
    }
}

/// Color scheme for victory
pub struct VictoryColors {
    pub text: Color,
    pub trophy: Color,
    pub crown: Color,
    pub fireworks: Color,
}

impl Default for VictoryColors {
    fn default() -> Self {
        Self {
            text: Color::Yellow,
            trophy: Color::Yellow,
            crown: Color::Yellow,
            fireworks: Color::Cyan,
        }
    }
}

/// Color scheme for items
pub struct ItemColors {
    pub sword: Color,
    pub shield: Color,
    pub potion: Color,
    pub scroll: Color,
    pub ring: Color,
    pub gold: Color,
    pub key: Color,
}

impl Default for ItemColors {
    fn default() -> Self {
        Self {
            sword: Color::White,
            shield: Color::Blue,
            potion: Color::Red,
            scroll: Color::Yellow,
            ring: Color::Magenta,
            gold: Color::Yellow,
            key: Color::DarkYellow,
        }
    }
}

// ============================================================================
// DISPLAY HELPER STRUCTURES
// ============================================================================

/// Represents a piece of ASCII art with its metadata
#[derive(Clone)]
pub struct AsciiArt {
    pub content: &'static str,
    pub width: usize,
    pub height: usize,
    pub default_color: Color,
}

impl AsciiArt {
    /// Create a new AsciiArt from content
    pub fn new(content: &'static str, default_color: Color) -> Self {
        let lines: Vec<&str> = content.lines().collect();
        let height = lines.len();
        let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

        Self {
            content,
            width,
            height,
            default_color,
        }
    }

    /// Get the art as lines
    pub fn lines(&self) -> Vec<&str> {
        self.content.lines().collect()
    }

    /// Get centered position for given terminal width
    pub fn center_x(&self, terminal_width: u16) -> u16 {
        if self.width as u16 >= terminal_width {
            0
        } else {
            (terminal_width - self.width as u16) / 2
        }
    }
}

// ============================================================================
// DISPLAY FUNCTIONS
// ============================================================================

/// Get the title screen art
pub fn get_title_art() -> AsciiArt {
    AsciiArt::new(TITLE_ART, Color::Cyan)
}

/// Get class art by class name
pub fn get_class_art(class_name: &str, compact: bool) -> AsciiArt {
    let colors = ClassColors::default();

    match class_name.to_lowercase().as_str() {
        "warrior" => {
            let art = if compact { WARRIOR_ART_COMPACT } else { WARRIOR_ART };
            AsciiArt::new(art, colors.warrior)
        }
        "mage" | "wizard" => {
            let art = if compact { MAGE_ART_COMPACT } else { MAGE_ART };
            AsciiArt::new(art, colors.mage)
        }
        "rogue" | "thief" | "assassin" => {
            let art = if compact { ROGUE_ART_COMPACT } else { ROGUE_ART };
            AsciiArt::new(art, colors.rogue)
        }
        "cleric" | "priest" | "paladin" => {
            let art = if compact { CLERIC_ART_COMPACT } else { CLERIC_ART };
            AsciiArt::new(art, colors.cleric)
        }
        "ranger" | "archer" | "hunter" => {
            let art = if compact { RANGER_ART_COMPACT } else { RANGER_ART };
            AsciiArt::new(art, colors.ranger)
        }
        "monk" | "martial artist" => {
            let art = if compact { MONK_ART_COMPACT } else { MONK_ART };
            AsciiArt::new(art, colors.monk)
        }
        _ => {
            // Default to warrior
            let art = if compact { WARRIOR_ART_COMPACT } else { WARRIOR_ART };
            AsciiArt::new(art, colors.warrior)
        }
    }
}

/// Get boss art by boss name
pub fn get_boss_art(boss_name: &str, compact: bool) -> AsciiArt {
    let colors = BossColors::default();

    match boss_name.to_lowercase().as_str() {
        "dragon" | "wyrm" | "drake" => {
            let art = if compact { DRAGON_ART_COMPACT } else { DRAGON_ART };
            AsciiArt::new(art, colors.dragon)
        }
        "demon" | "demon lord" | "devil" => {
            let art = if compact { DEMON_LORD_ART_COMPACT } else { DEMON_LORD_ART };
            AsciiArt::new(art, colors.demon_lord)
        }
        "lich" | "necromancer" | "undead lord" => {
            let art = if compact { LICH_ART_COMPACT } else { LICH_ART };
            AsciiArt::new(art, colors.lich)
        }
        "giant" | "titan" | "colossus" => {
            let art = if compact { GIANT_ART_COMPACT } else { GIANT_ART };
            AsciiArt::new(art, colors.giant)
        }
        _ => {
            // Default to dragon
            let art = if compact { DRAGON_ART_COMPACT } else { DRAGON_ART };
            AsciiArt::new(art, colors.dragon)
        }
    }
}

/// Get game over art
pub fn get_game_over_art(simple: bool) -> AsciiArt {
    let colors = GameOverColors::default();
    let art = if simple { GAME_OVER_ART_SIMPLE } else { GAME_OVER_ART };
    AsciiArt::new(art, colors.text)
}

/// Get victory art
pub fn get_victory_art() -> AsciiArt {
    AsciiArt::new(VICTORY_ART, VictoryColors::default().text)
}

/// Get item icon by item type
pub fn get_item_icon(item_type: &str) -> AsciiArt {
    let colors = ItemColors::default();

    match item_type.to_lowercase().as_str() {
        "sword" | "weapon" | "blade" => AsciiArt::new(ICON_SWORD, colors.sword),
        "shield" | "armor" | "defence" => AsciiArt::new(ICON_SHIELD, colors.shield),
        "potion" | "elixir" | "drink" => AsciiArt::new(ICON_POTION, colors.potion),
        "scroll" | "spell" | "magic" => AsciiArt::new(ICON_SCROLL, colors.scroll),
        "ring" | "accessory" => AsciiArt::new(ICON_RING, colors.ring),
        "key" | "lock" => AsciiArt::new(ICON_KEY, colors.key),
        "gold" | "coin" | "money" => AsciiArt::new(ICON_GOLD, colors.gold),
        "helmet" | "helm" | "head" => AsciiArt::new(ICON_HELMET, colors.shield),
        "bow" | "ranged" => AsciiArt::new(ICON_BOW, colors.sword),
        "staff" | "wand" | "rod" => AsciiArt::new(ICON_STAFF, colors.scroll),
        _ => AsciiArt::new(ICON_GOLD, colors.gold),
    }
}

/// Get loading animation frame
pub fn get_loading_frame(animation_type: LoadingAnimation, frame: usize) -> &'static str {
    match animation_type {
        LoadingAnimation::Sword => {
            LOADING_SWORD_FRAMES[frame % LOADING_SWORD_FRAMES.len()]
        }
        LoadingAnimation::Orb => {
            LOADING_ORB_FRAMES[frame % LOADING_ORB_FRAMES.len()]
        }
        LoadingAnimation::Spinner => {
            LOADING_SPINNER_FRAMES[frame % LOADING_SPINNER_FRAMES.len()]
        }
        LoadingAnimation::Dots => {
            LOADING_DOTS_FRAMES[frame % LOADING_DOTS_FRAMES.len()]
        }
    }
}

/// Get the frame count for a loading animation
pub fn get_loading_frame_count(animation_type: LoadingAnimation) -> usize {
    match animation_type {
        LoadingAnimation::Sword => LOADING_SWORD_FRAMES.len(),
        LoadingAnimation::Orb => LOADING_ORB_FRAMES.len(),
        LoadingAnimation::Spinner => LOADING_SPINNER_FRAMES.len(),
        LoadingAnimation::Dots => LOADING_DOTS_FRAMES.len(),
    }
}

/// Types of loading animations available
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoadingAnimation {
    Sword,
    Orb,
    Spinner,
    Dots,
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/// Add a colored border around ASCII art
pub fn add_border(art: &str, border_char: char) -> String {
    let lines: Vec<&str> = art.lines().collect();
    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut result = String::new();

    // Top border
    result.push(border_char);
    for _ in 0..max_width + 2 {
        result.push(border_char);
    }
    result.push(border_char);
    result.push('\n');

    // Content with side borders
    for line in &lines {
        result.push(border_char);
        result.push(' ');
        result.push_str(line);
        for _ in 0..(max_width - line.len()) {
            result.push(' ');
        }
        result.push(' ');
        result.push(border_char);
        result.push('\n');
    }

    // Bottom border
    result.push(border_char);
    for _ in 0..max_width + 2 {
        result.push(border_char);
    }
    result.push(border_char);
    result.push('\n');

    result
}

/// Center text within a given width
pub fn center_text(text: &str, width: usize) -> String {
    if text.len() >= width {
        return text.to_string();
    }

    let padding = (width - text.len()) / 2;
    let mut result = String::new();
    for _ in 0..padding {
        result.push(' ');
    }
    result.push_str(text);
    result
}

/// Combine multiple ASCII art pieces horizontally
pub fn combine_horizontal(arts: &[&str], spacing: usize) -> String {
    let art_lines: Vec<Vec<&str>> = arts.iter()
        .map(|a| a.lines().collect())
        .collect();

    let max_height = art_lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let widths: Vec<usize> = art_lines.iter()
        .map(|lines| lines.iter().map(|l| l.len()).max().unwrap_or(0))
        .collect();

    let mut result = String::new();

    for row in 0..max_height {
        for (i, lines) in art_lines.iter().enumerate() {
            let line = if row < lines.len() { lines[row] } else { "" };
            result.push_str(line);

            // Pad to width
            for _ in 0..(widths[i] - line.len()) {
                result.push(' ');
            }

            // Add spacing between arts (except after last)
            if i < art_lines.len() - 1 {
                for _ in 0..spacing {
                    result.push(' ');
                }
            }
        }
        result.push('\n');
    }

    result
}

/// Create a simple animated text effect (returns frames)
pub fn create_typewriter_frames(text: &str) -> Vec<String> {
    let mut frames = Vec::new();
    let mut current = String::new();

    for ch in text.chars() {
        current.push(ch);
        frames.push(current.clone());
    }

    frames
}

/// Get decorative divider line
pub fn get_divider(width: usize, style: DividerStyle) -> String {
    match style {
        DividerStyle::Single => "-".repeat(width),
        DividerStyle::Double => "=".repeat(width),
        DividerStyle::Fancy => {
            let mut result = String::new();
            for i in 0..width {
                if i % 2 == 0 {
                    result.push('-');
                } else {
                    result.push('~');
                }
            }
            result
        }
        DividerStyle::Stars => {
            let mut result = String::new();
            for i in 0..width {
                if i % 3 == 0 {
                    result.push('*');
                } else {
                    result.push(' ');
                }
            }
            result
        }
    }
}

/// Divider line styles
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DividerStyle {
    Single,
    Double,
    Fancy,
    Stars,
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_art_creation() {
        let art = get_title_art();
        assert!(art.height > 0);
        assert!(art.width > 0);
    }

    #[test]
    fn test_class_art_retrieval() {
        let classes = ["warrior", "mage", "rogue", "cleric", "ranger", "monk"];
        for class in &classes {
            let art = get_class_art(class, false);
            assert!(art.height > 0, "Class {} should have art", class);
        }
    }

    #[test]
    fn test_boss_art_retrieval() {
        let bosses = ["dragon", "demon", "lich", "giant"];
        for boss in &bosses {
            let art = get_boss_art(boss, false);
            assert!(art.height > 0, "Boss {} should have art", boss);
        }
    }

    #[test]
    fn test_loading_frames() {
        let frame = get_loading_frame(LoadingAnimation::Spinner, 0);
        assert!(!frame.is_empty());

        let count = get_loading_frame_count(LoadingAnimation::Spinner);
        assert_eq!(count, 4);
    }

    #[test]
    fn test_center_text() {
        let centered = center_text("test", 10);
        assert_eq!(centered.len(), 7); // 3 spaces + 4 chars
    }

    #[test]
    fn test_divider() {
        let div = get_divider(10, DividerStyle::Single);
        assert_eq!(div.len(), 10);
        assert!(div.chars().all(|c| c == '-'));
    }
}
