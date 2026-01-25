//! ShadowCrypt CLI - Terminal frontend for the roguelike game
//!
//! This CLI uses crossterm for terminal rendering and input handling,
//! importing all game logic from shadowcrypt-core.

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::prelude::*;
use std::io::{stdout, Write};
use std::time::Duration;

use shadowcrypt_core::prelude::*;

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
    }
}

/// Get the terminal glyph for a tile
fn tile_glyph(tile: &Tile) -> char {
    match tile {
        Tile::Wall => '#',
        Tile::Floor => '.',
        Tile::Corridor => '.',
        Tile::Door => '+',
        Tile::OpenDoor => '/',
        Tile::LockedDoor => '%',
        Tile::StairsDown => '>',
        Tile::StairsUp => '<',
        Tile::Chest => '$',
        Tile::OpenChest => '_',
        Tile::Trap => '^',
        Tile::DisarmedTrap => '~',
        Tile::Shrine => '*',
        Tile::UsedShrine => '.',
        Tile::Water => '~',
        Tile::Lava => '~',
        Tile::Sand => '.',
        Tile::Grass => '"',
        Tile::Ice => '=',
    }
}

/// Get the terminal color for a tile
fn tile_color(tile: &Tile) -> Color {
    match tile {
        Tile::Wall => Color::Grey,
        Tile::Floor | Tile::Corridor => Color::White,
        Tile::Door | Tile::OpenDoor => Color::Yellow,
        Tile::LockedDoor => Color::Red,
        Tile::StairsDown | Tile::StairsUp => Color::Cyan,
        Tile::Chest => Color::Yellow,
        Tile::OpenChest => Color::DarkYellow,
        Tile::Trap => Color::Red,
        Tile::DisarmedTrap => Color::DarkGrey,
        Tile::Shrine => Color::Magenta,
        Tile::UsedShrine => Color::DarkMagenta,
        Tile::Water => Color::Blue,
        Tile::Lava => Color::Red,
        Tile::Sand => Color::Yellow,
        Tile::Grass => Color::Green,
        Tile::Ice => Color::Cyan,
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
        EnemyKind::BossLichKing => 'L',
        EnemyKind::BossForestGuardian => 'F',
        EnemyKind::BossFrostWyrm => 'W',
        EnemyKind::BossDemonKing => 'D',
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
        EnemyKind::BossLichKing => Color::Magenta,
        EnemyKind::BossForestGuardian => Color::Green,
        EnemyKind::BossFrostWyrm => Color::Cyan,
        EnemyKind::BossDemonKing => Color::Red,
    }
}

/// Get the terminal glyph for an item
fn item_glyph(kind: &ItemKind) -> char {
    match kind {
        // Potions
        ItemKind::HealthPotion | ItemKind::ManaPotion | ItemKind::StrengthPotion
        | ItemKind::SpeedPotion | ItemKind::InvisibilityPotion | ItemKind::GreaterHealthPotion
        | ItemKind::GreaterManaPotion | ItemKind::ElixirOfLife | ItemKind::AntidotePotion
        | ItemKind::FireResistPotion | ItemKind::ColdResistPotion => '!',

        // Scrolls
        ItemKind::ScrollOfFire | ItemKind::ScrollOfIce | ItemKind::ScrollOfLightning
        | ItemKind::ScrollOfTeleport | ItemKind::ScrollOfIdentify | ItemKind::ScrollOfEnchant
        | ItemKind::ScrollOfMapping | ItemKind::ScrollOfSummoning => '?',

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
    }
}

/// Get the terminal color for a rarity
fn rarity_color(rarity: &Rarity) -> Color {
    match rarity {
        Rarity::Common => Color::White,
        Rarity::Uncommon => Color::Green,
        Rarity::Rare => Color::Blue,
        Rarity::Epic => Color::Magenta,
        Rarity::Legendary => Color::Yellow,
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
// RENDERING FUNCTIONS
// ============================================================================

fn render(state: &GameState) -> std::io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, MoveTo(0, 0))?;

    // Render map
    for y in 0..MAP_HEIGHT.min(43) {
        execute!(stdout, MoveTo(0, y as u16))?;

        for x in 0..MAP_WIDTH.min(100) {
            // Player
            if state.player.x == x && state.player.y == y {
                execute!(
                    stdout,
                    SetForegroundColor(Color::Yellow),
                    Print('@'),
                    ResetColor
                )?;
            }
            // Enemies
            else if let Some(enemy) = state.enemies.iter()
                .find(|e| e.x == x && e.y == y && e.is_alive() && state.map.visible[y][x])
            {
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

    // Status effects (line 44)
    execute!(
        stdout,
        MoveTo(0, stats_y + 1),
        Clear(ClearType::CurrentLine)
    )?;

    if !state.player.status_effects.is_empty() {
        write!(stdout, "Status: ")?;
        for (effect, duration) in &state.player.status_effects {
            execute!(stdout, SetForegroundColor(status_color(effect)))?;
            write!(stdout, "{}({}) ", effect.name(), duration)?;
        }
        execute!(stdout, ResetColor)?;
    }

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

    // Controls hint
    execute!(
        stdout,
        MoveTo(0, stats_y + 8),
        Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::DarkGrey),
        Print("[WASD:Move] [Space:Skill] [Tab:CycleSkill] [I:Inventory] [>:Descend] [<:Ascend] [?:Help] [Q:Quit]"),
        ResetColor
    )?;

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

fn render_inventory(state: &GameState) -> std::io::Result<()> {
    let mut stdout = stdout();

    let start_x = 10;
    let start_y = 5;
    let width = 60;
    let height = 30;

    // Draw border
    for y in start_y..(start_y + height) {
        execute!(stdout, MoveTo(start_x, y))?;
        if y == start_y || y == start_y + height - 1 {
            write!(stdout, "{}", "=".repeat(width))?;
        } else {
            write!(stdout, "|{}|", " ".repeat(width - 2))?;
        }
    }

    // Title
    execute!(stdout, MoveTo(start_x + 2, start_y + 1))?;
    execute!(stdout, SetForegroundColor(Color::Yellow))?;
    write!(stdout, "=== INVENTORY ({}/{}) ===", state.player.inventory.len(), 20)?;
    execute!(stdout, ResetColor)?;

    // Equipped items
    execute!(stdout, MoveTo(start_x + 2, start_y + 3))?;
    execute!(stdout, SetForegroundColor(Color::Cyan))?;
    write!(stdout, "-- Equipped --")?;
    execute!(stdout, ResetColor)?;

    let slots = [
        (EquipSlot::Weapon, "Weapon"),
        (EquipSlot::Armor, "Armor"),
        (EquipSlot::Shield, "Shield"),
        (EquipSlot::Helmet, "Helmet"),
        (EquipSlot::Gloves, "Gloves"),
        (EquipSlot::Boots, "Boots"),
        (EquipSlot::Ring, "Ring"),
        (EquipSlot::Amulet, "Amulet"),
    ];

    for (i, (slot, name)) in slots.iter().enumerate() {
        execute!(stdout, MoveTo(start_x + 2, start_y + 4 + i as u16))?;
        if let Some(item) = state.player.equipment.get(slot) {
            execute!(stdout, SetForegroundColor(rarity_color(&item.rarity)))?;
            write!(stdout, "{}: {}{}", name, item.rarity.prefix(), item.kind.name())?;
        } else {
            execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
            write!(stdout, "{}: (empty)", name)?;
        }
        execute!(stdout, ResetColor)?;
    }

    // Inventory items
    execute!(stdout, MoveTo(start_x + 2, start_y + 14))?;
    execute!(stdout, SetForegroundColor(Color::Cyan))?;
    write!(stdout, "-- Items (1-9,0 to use/equip) --")?;
    execute!(stdout, ResetColor)?;

    for (i, item) in state.player.inventory.iter().take(10).enumerate() {
        execute!(stdout, MoveTo(start_x + 2, start_y + 15 + i as u16))?;
        execute!(stdout, SetForegroundColor(rarity_color(&item.rarity)))?;
        let key = if i == 9 { 0 } else { i + 1 };
        write!(stdout, "{}: {}{}", key, item.rarity.prefix(), item.kind.name())?;
        execute!(stdout, ResetColor)?;
    }

    // Instructions
    execute!(stdout, MoveTo(start_x + 2, start_y + 27))?;
    execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
    write!(stdout, "Press I or ESC to close")?;
    execute!(stdout, ResetColor)?;

    Ok(())
}

fn render_help(_state: &GameState) -> std::io::Result<()> {
    let mut stdout = stdout();

    let start_x = 5;
    let start_y = 3;
    let width = 70;
    let height = 38;

    // Draw border
    for y in start_y..(start_y + height) {
        execute!(stdout, MoveTo(start_x, y))?;
        if y == start_y || y == start_y + height - 1 {
            write!(stdout, "{}", "=".repeat(width))?;
        } else {
            write!(stdout, "|{}|", " ".repeat(width - 2))?;
        }
    }

    // Title
    execute!(stdout, MoveTo(start_x + 2, start_y + 1))?;
    execute!(stdout, SetForegroundColor(Color::Yellow))?;
    write!(stdout, "=== SHADOWCRYPT HELP ===")?;
    execute!(stdout, ResetColor)?;

    let help_text = [
        ("", ""),
        ("MOVEMENT:", ""),
        ("  WASD / Arrow Keys / HJKL", "Move in 4 directions"),
        ("  YUBN", "Move diagonally"),
        ("", ""),
        ("ACTIONS:", ""),
        ("  Space", "Use current skill"),
        ("  Tab", "Cycle through skills"),
        ("  I", "Open/close inventory"),
        ("  1-9, 0", "Use item from inventory"),
        ("  > or .", "Descend stairs"),
        ("  < or ,", "Ascend stairs"),
        ("  ?", "Toggle this help screen"),
        ("  Q or ESC", "Quit game"),
        ("", ""),
        ("SYMBOLS:", ""),
        ("  @", "You (the player)"),
        ("  #", "Wall"),
        ("  .", "Floor"),
        ("  +", "Closed door"),
        ("  /", "Open door"),
        ("  >", "Stairs down"),
        ("  <", "Stairs up"),
        ("  $", "Chest"),
        ("  ^", "Trap"),
        ("  *", "Shrine"),
        ("", ""),
        ("GOAL: Descend to level 30 and defeat the Demon King!", ""),
    ];

    for (i, (key, desc)) in help_text.iter().enumerate() {
        execute!(stdout, MoveTo(start_x + 2, start_y + 2 + i as u16))?;
        if !key.is_empty() {
            execute!(stdout, SetForegroundColor(Color::Cyan))?;
            write!(stdout, "{:<30}", key)?;
        }
        execute!(stdout, SetForegroundColor(Color::White))?;
        write!(stdout, "{}", desc)?;
        execute!(stdout, ResetColor)?;
    }

    // Instructions
    execute!(stdout, MoveTo(start_x + 2, start_y + 35))?;
    execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
    write!(stdout, "Press ? or ESC to close")?;
    execute!(stdout, ResetColor)?;

    Ok(())
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
        let classes = CharacterClass::all();
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

    // Game loop
    loop {
        render(&state)?;

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

            let action = AIDecider::decide(&state);
            state.execute_ai_action(action);
            std::thread::sleep(Duration::from_millis(auto_speed));
            continue;
        }

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                // Inventory mode
                if state.show_inventory {
                    match code {
                        KeyCode::Char('i') | KeyCode::Esc => state.show_inventory = false,
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

                // Normal mode
                match code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => break,

                    // Movement
                    KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('k') => state.move_player(0, -1),
                    KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j') => state.move_player(0, 1),
                    KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('h') => state.move_player(-1, 0),
                    KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('l') => state.move_player(1, 0),

                    // Diagonal
                    KeyCode::Char('y') => state.move_player(-1, -1),
                    KeyCode::Char('u') => state.move_player(1, -1),
                    KeyCode::Char('b') => state.move_player(-1, 1),
                    KeyCode::Char('n') => state.move_player(1, 1),

                    // Skills
                    KeyCode::Char(' ') => state.use_skill(),
                    KeyCode::Tab => state.cycle_skill(),

                    // Stairs
                    KeyCode::Char('>') | KeyCode::Char('.') => state.descend(),
                    KeyCode::Char('<') | KeyCode::Char(',') => state.ascend(),

                    // Inventory
                    KeyCode::Char('i') => state.show_inventory = true,
                    KeyCode::Char(c) if c.is_ascii_digit() => {
                        let idx = if c == '0' { 9 } else { (c as u8 - b'1') as usize };
                        state.use_item(idx);
                    }

                    // Help
                    KeyCode::Char('?') => state.show_help = true,

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
