//! Game state and main game logic

use std::collections::HashMap;
use rand::prelude::*;
use rand::rngs::StdRng;
use serde::{Serialize, Deserialize};

use crate::classes::CharacterClass;
use crate::combat::StatusEffect;
use crate::entities::{Enemy, EnemyKind, Player};
use crate::items::{Item, ItemKind, Rarity};
use crate::magic::Skill;
use crate::world::{Map, Room, Tile, BOSS_LEVELS, MAX_DUNGEON_LEVEL, MAP_WIDTH, MAP_HEIGHT};
use crate::ai::{AIAction, AIDecider};
use crate::quests::{QuestTracker, QuestReward, QuestId};

/// A message with an associated color index
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameMessage {
    pub text: String,
    pub color_index: u8,
}

impl GameMessage {
    pub fn new(text: String, color_index: u8) -> Self {
        Self { text, color_index }
    }
}

/// The main game state
#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub map: Map,
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub items: Vec<Item>,
    pub messages: Vec<GameMessage>,
    pub dungeon_level: u32,
    pub turn_count: u32,
    pub game_over: bool,
    pub victory: bool,
    pub boss_defeated: bool,
    pub show_inventory: bool,
    pub show_help: bool,
    pub show_quests: bool,
    pub quest_tracker: QuestTracker,
    #[serde(skip, default = "StdRng::from_entropy")]
    pub rng: StdRng,
}

impl GameState {
    /// Create a new game with the given character class
    pub fn new(class: CharacterClass) -> Self {
        let mut rng = StdRng::from_entropy();
        let mut map = Map::new();
        map.generate(&mut rng, 1);

        let (px, py) = map.rooms[0].center();
        let player = Player::new(px, py, class);

        let mut quest_tracker = QuestTracker::new();
        quest_tracker.auto_start_starter_quests(1, 1);

        let mut state = Self {
            map,
            player,
            enemies: Vec::new(),
            items: Vec::new(),
            messages: Vec::new(),
            dungeon_level: 1,
            turn_count: 0,
            game_over: false,
            victory: false,
            boss_defeated: false,
            show_inventory: false,
            show_help: false,
            show_quests: false,
            quest_tracker,
            rng,
        };

        state.add_message(format!("Welcome, {}! Descend to level 30 to defeat the Demon King!", class.name()), 9);
        state.add_message(format!("Press ? for help. Your skill: {}", class.special_ability()), 11);
        state.add_message("Press Q to view your quests.".to_string(), 5);
        state.spawn_enemies();
        state.spawn_items();
        state.map.compute_fov(state.player.x, state.player.y);

        state
    }

    /// Add a message to the message log
    pub fn add_message(&mut self, msg: String, color_index: u8) {
        self.messages.push(GameMessage::new(msg, color_index));
        if self.messages.len() > 6 {
            self.messages.remove(0);
        }
    }

    /// Spawn enemies on the current level
    pub fn spawn_enemies(&mut self) {
        self.enemies.clear();

        let is_boss_level = BOSS_LEVELS.contains(&self.dungeon_level);

        for (i, room) in self.map.rooms.iter().enumerate() {
            if i == 0 {
                continue;
            }

            // Spawn boss in last room on boss levels
            if is_boss_level && i == self.map.rooms.len() - 1 && !self.boss_defeated {
                if let Some(boss_kind) = EnemyKind::boss_for_level(self.dungeon_level) {
                    let (bx, by) = room.center();
                    self.enemies.push(Enemy::new(bx, by, boss_kind, self.dungeon_level));
                    continue;
                }
            }

            let max_enemies = 2 + (self.dungeon_level as usize / 5);
            let num_enemies = self.rng.gen_range(1..=max_enemies.min(5));

            for _ in 0..num_enemies {
                let (x, y) = room.random_point(&mut self.rng);
                let kind = EnemyKind::for_level(self.dungeon_level, &mut self.rng);
                self.enemies.push(Enemy::new(x, y, kind, self.dungeon_level));
            }
        }
    }

    /// Spawn items on the current level
    pub fn spawn_items(&mut self) {
        self.items.clear();

        // Copy room data to avoid borrow issues
        let rooms: Vec<Room> = self.map.rooms.clone();

        for (i, room) in rooms.iter().enumerate() {
            if i == 0 { continue; }

            let num_items = self.rng.gen_range(0..=3);
            for _ in 0..num_items {
                let x = self.rng.gen_range(room.x + 1..room.x + room.width - 1);
                let y = self.rng.gen_range(room.y + 1..room.y + room.height - 1);
                let (kind, rarity) = self.random_item();
                self.items.push(Item::new(x, y, kind, rarity));
            }

            // Gold
            if self.rng.gen_bool(0.4) {
                let x = self.rng.gen_range(room.x + 1..room.x + room.width - 1);
                let y = self.rng.gen_range(room.y + 1..room.y + room.height - 1);
                self.items.push(Item::new(x, y, ItemKind::Gold, Rarity::Common));
            }

            // Food
            if self.rng.gen_bool(0.15) {
                let x = self.rng.gen_range(room.x + 1..room.x + room.width - 1);
                let y = self.rng.gen_range(room.y + 1..room.y + room.height - 1);
                let food = match self.rng.gen_range(0..5) {
                    0 => ItemKind::Apple,
                    1 => ItemKind::Bread,
                    2 => ItemKind::Cheese,
                    3 => ItemKind::Meat,
                    _ => ItemKind::Bread,
                };
                self.items.push(Item::new(x, y, food, Rarity::Common));
            }
        }
    }

    /// Generate a random item based on dungeon level
    pub fn random_item(&mut self) -> (ItemKind, Rarity) {
        let floor_bonus = self.dungeon_level as i32;
        let rarity = match self.rng.gen_range(0..100) + floor_bonus {
            0..=45 => Rarity::Common,
            46..=70 => Rarity::Uncommon,
            71..=88 => Rarity::Rare,
            89..=96 => Rarity::Epic,
            97..=105 => Rarity::Legendary,
            _ => Rarity::Mythic,
        };

        let kind = match self.rng.gen_range(0..100) {
            0..=24 => self.random_potion(rarity),
            25..=39 => self.random_scroll(rarity),
            40..=57 => self.random_weapon(rarity),
            58..=73 => self.random_armor(rarity),
            74..=83 => self.random_gear(rarity),
            84..=95 => self.random_jewelry(rarity),
            _ => self.random_misc(rarity),
        };

        (kind, rarity)
    }

    fn random_potion(&mut self, rarity: Rarity) -> ItemKind {
        let tier = match rarity {
            Rarity::Common | Rarity::Uncommon => self.rng.gen_range(0..8),
            Rarity::Rare => self.rng.gen_range(0..14),
            _ => self.rng.gen_range(0..20),
        };
        match tier {
            0 => ItemKind::HealthPotion,
            1 => ItemKind::ManaPotion,
            2 => ItemKind::StrengthPotion,
            3 => ItemKind::DefensePotion,
            4 => ItemKind::SpeedPotion,
            5 => ItemKind::RegenerationPotion,
            6 => ItemKind::InvisibilityPotion,
            7 => ItemKind::FireResistPotion,
            8 => ItemKind::IceResistPotion,
            9 => ItemKind::PoisonResistPotion,
            10 => ItemKind::BerserkPotion,
            11 => ItemKind::GiantPotion,
            12 => ItemKind::LevitationPotion,
            13 => ItemKind::XPPotion,
            14 => ItemKind::FullRestorePotion,
            15 => ItemKind::LuckPotion,
            16 => ItemKind::CriticalPotion,
            17 => ItemKind::VisionPotion,
            18 => ItemKind::CureAllPotion,
            _ => ItemKind::UltimatePowerPotion,
        }
    }

    fn random_scroll(&mut self, rarity: Rarity) -> ItemKind {
        let tier = match rarity {
            Rarity::Common | Rarity::Uncommon => self.rng.gen_range(0..8),
            Rarity::Rare | Rarity::Epic => self.rng.gen_range(0..14),
            _ => self.rng.gen_range(0..18),
        };
        match tier {
            0 => ItemKind::ScrollTeleport,
            1 => ItemKind::ScrollFireball,
            2 => ItemKind::ScrollIceStorm,
            3 => ItemKind::ScrollLightning,
            4 => ItemKind::ScrollMapping,
            5 => ItemKind::ScrollIdentify,
            6 => ItemKind::ScrollEnchant,
            7 => ItemKind::ScrollSummon,
            8 => ItemKind::ScrollBanish,
            9 => ItemKind::ScrollTimeStop,
            10 => ItemKind::ScrollMassHeal,
            11 => ItemKind::ScrollDeath,
            12 => ItemKind::ScrollEarthquake,
            13 => ItemKind::ScrollMeteor,
            14 => ItemKind::ScrollBlizzard,
            15 => ItemKind::ScrollChainLightning,
            16 => ItemKind::ScrollDivineWrath,
            _ => ItemKind::ScrollDarkness,
        }
    }

    fn random_weapon(&mut self, rarity: Rarity) -> ItemKind {
        let tier = match rarity {
            Rarity::Common => self.rng.gen_range(0..8),
            Rarity::Uncommon => self.rng.gen_range(0..15),
            Rarity::Rare => self.rng.gen_range(0..20),
            _ => self.rng.gen_range(0..25),
        };
        match tier {
            0 => ItemKind::Dagger,
            1 => ItemKind::ShortSword,
            2 => ItemKind::LongSword,
            3 => ItemKind::Axe,
            4 => ItemKind::Mace,
            5 => ItemKind::Spear,
            6 => ItemKind::Staff,
            7 => ItemKind::Wand,
            8 => ItemKind::Greatsword,
            9 => ItemKind::BattleAxe,
            10 => ItemKind::WarHammer,
            11 => ItemKind::Halberd,
            12 => ItemKind::Bow,
            13 => ItemKind::Crossbow,
            14 => ItemKind::Scythe,
            15 => ItemKind::Katana,
            16 => ItemKind::Rapier,
            17 => ItemKind::Flail,
            18 => ItemKind::Morningstar,
            19 => ItemKind::Trident,
            20 => ItemKind::FlameSword,
            21 => ItemKind::FrostBlade,
            22 => ItemKind::ThunderAxe,
            23 => ItemKind::VoidStaff,
            _ => ItemKind::DemonSlayer,
        }
    }

    fn random_armor(&mut self, rarity: Rarity) -> ItemKind {
        let tier = match rarity {
            Rarity::Common => self.rng.gen_range(0..8),
            Rarity::Uncommon => self.rng.gen_range(0..14),
            Rarity::Rare => self.rng.gen_range(0..22),
            _ => self.rng.gen_range(0..32),
        };
        match tier {
            0 => ItemKind::LeatherArmor,
            1 => ItemKind::ChainMail,
            2 => ItemKind::Buckler,
            3 => ItemKind::WoodenShield,
            4 => ItemKind::LeatherCap,
            5 => ItemKind::LeatherGloves,
            6 => ItemKind::LeatherBoots,
            7 => ItemKind::MageRobes,
            8 => ItemKind::ScaleMail,
            9 => ItemKind::PlateMail,
            10 => ItemKind::IronShield,
            11 => ItemKind::IronHelm,
            12 => ItemKind::IronGauntlets,
            13 => ItemKind::IronBoots,
            14 => ItemKind::TowerShield,
            15 => ItemKind::MagicShield,
            16 => ItemKind::SteelHelm,
            17 => ItemKind::BootsOfSpeed,
            18 => ItemKind::AssassinGarb,
            19 => ItemKind::WizardHat,
            20 => ItemKind::DragonArmor,
            21 => ItemKind::DragonShield,
            22 => ItemKind::SpikedShield,
            23 => ItemKind::MirrorShield,
            24 => ItemKind::PhoenixShield,
            25 => ItemKind::AbyssalShield,
            26 => ItemKind::HolyArmor,
            27 => ItemKind::DemonArmor,
            28 => ItemKind::CrystalArmor,
            29 => ItemKind::ShadowCloak,
            30 => ItemKind::TitanPlate,
            _ => ItemKind::DragonArmor,
        }
    }

    fn random_gear(&mut self, rarity: Rarity) -> ItemKind {
        let tier = match rarity {
            Rarity::Common | Rarity::Uncommon => self.rng.gen_range(0..8),
            Rarity::Rare => self.rng.gen_range(0..16),
            _ => self.rng.gen_range(0..26),
        };
        match tier {
            0 => ItemKind::LeatherCap,
            1 => ItemKind::LeatherGloves,
            2 => ItemKind::LeatherBoots,
            3 => ItemKind::IronHelm,
            4 => ItemKind::IronGauntlets,
            5 => ItemKind::IronBoots,
            6 => ItemKind::BootsOfSpeed,
            7 => ItemKind::SteelHelm,
            8 => ItemKind::CrownOfKings,
            9 => ItemKind::WizardHat,
            10 => ItemKind::DemonSkull,
            11 => ItemKind::DragonHelm,
            12 => ItemKind::CrystalCrown,
            13 => ItemKind::HoodOfShadows,
            14 => ItemKind::HelmOfValor,
            15 => ItemKind::GlovesOfPower,
            16 => ItemKind::ThievesGloves,
            17 => ItemKind::DragonGauntlets,
            18 => ItemKind::FrostGauntlets,
            19 => ItemKind::FlameGauntlets,
            20 => ItemKind::GauntletsOfMight,
            21 => ItemKind::BootsOfLeaping,
            22 => ItemKind::WingedBoots,
            23 => ItemKind::ShadowBoots,
            24 => ItemKind::LavaWalkers,
            _ => ItemKind::BootsOfTheWind,
        }
    }

    fn random_jewelry(&mut self, rarity: Rarity) -> ItemKind {
        let tier = match rarity {
            Rarity::Common | Rarity::Uncommon => self.rng.gen_range(0..10),
            Rarity::Rare | Rarity::Epic => self.rng.gen_range(0..20),
            _ => self.rng.gen_range(0..28),
        };
        match tier {
            0 => ItemKind::RingOfStrength,
            1 => ItemKind::RingOfProtection,
            2 => ItemKind::RingOfSpeed,
            3 => ItemKind::RingOfRegeneration,
            4 => ItemKind::RingOfMana,
            5 => ItemKind::AmuletOfHealth,
            6 => ItemKind::AmuletOfMana,
            7 => ItemKind::AmuletOfProtection,
            8 => ItemKind::AmuletOfPower,
            9 => ItemKind::AmuletOfWisdom,
            10 => ItemKind::RingOfFireball,
            11 => ItemKind::RingOfInvisibility,
            12 => ItemKind::RingOfTheVampire,
            13 => ItemKind::RingOfLuck,
            14 => ItemKind::AmuletOfLife,
            15 => ItemKind::RingOfDeath,
            16 => ItemKind::RingOfFrost,
            17 => ItemKind::RingOfFlame,
            18 => ItemKind::RingOfThunder,
            19 => ItemKind::RingOfShadows,
            20 => ItemKind::RingOfTheAncients,
            21 => ItemKind::AmuletOfDeath,
            22 => ItemKind::AmuletOfTheGods,
            23 => ItemKind::AmuletOfDragons,
            24 => ItemKind::AmuletOfChaos,
            25 => ItemKind::AmuletOfOrder,
            26 => ItemKind::AmuletOfBalance,
            _ => ItemKind::AmuletOfLife,
        }
    }

    fn random_misc(&mut self, rarity: Rarity) -> ItemKind {
        let tier = match rarity {
            Rarity::Common | Rarity::Uncommon => self.rng.gen_range(0..6),
            Rarity::Rare => self.rng.gen_range(0..12),
            _ => self.rng.gen_range(0..18),
        };
        match tier {
            0 => ItemKind::Bread,
            1 => ItemKind::Apple,
            2 => ItemKind::Meat,
            3 => ItemKind::Key,
            4 => ItemKind::Bomb,
            5 => ItemKind::Torch,
            6 => ItemKind::Cheese,
            7 => ItemKind::Meat,
            8 => ItemKind::Feast,
            9 => ItemKind::DragonFruit,
            10 => ItemKind::AncientWine,
            11 => ItemKind::GoldenApple,
            12 => ItemKind::Compass,
            13 => ItemKind::TeleportCrystal,
            14 => ItemKind::SoulGem,
            15 => ItemKind::AncientRelic,
            16 => ItemKind::DragonScale,
            _ => ItemKind::DemonHeart,
        }
    }

    /// Move the player
    pub fn move_player(&mut self, dx: i32, dy: i32) {
        if self.player.has_status(StatusEffect::Stun) {
            self.add_message("You are stunned!".to_string(), 11);
            self.end_turn();
            return;
        }

        let (dx, dy) = if self.player.has_status(StatusEffect::Confusion) && self.rng.gen_bool(0.3) {
            let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)];
            dirs[self.rng.gen_range(0..8)]
        } else {
            (dx, dy)
        };

        let new_x = (self.player.x as i32 + dx).max(0) as usize;
        let new_y = (self.player.y as i32 + dy).max(0) as usize;

        // Check for enemy
        if let Some(idx) = self.enemies.iter().position(|e| e.x == new_x && e.y == new_y && e.is_alive()) {
            self.attack_enemy(idx);
            return;
        }

        // Check for door
        if self.map.tiles[new_y][new_x] == Tile::Door {
            self.map.tiles[new_y][new_x] = Tile::OpenDoor;
            self.add_message("You open the door.".to_string(), 1);
            self.end_turn();
            return;
        }

        // Check for chest
        if self.map.tiles[new_y][new_x] == Tile::Chest {
            self.open_chest(new_x, new_y);
            return;
        }

        // Check for shrine
        if self.map.tiles[new_y][new_x] == Tile::Shrine {
            self.use_shrine(new_x, new_y);
            return;
        }

        if self.map.is_walkable(new_x, new_y) {
            self.player.x = new_x;
            self.player.y = new_y;
            self.map.compute_fov(self.player.x, self.player.y);

            // Check for trap
            if self.map.tiles[new_y][new_x] == Tile::Trap {
                self.trigger_trap();
            }

            // Check for lava
            if self.map.tiles[new_y][new_x] == Tile::Lava {
                let damage = 5 + self.dungeon_level as i32;
                self.player.hp -= damage;
                self.add_message(format!("The lava burns you for {} damage!", damage), 3);
                self.player.add_status(StatusEffect::Burn, 3);
            }

            self.pickup_items();
            self.end_turn();
        }
    }

    /// Attack an enemy
    pub fn attack_enemy(&mut self, idx: usize) {
        let player_attack = self.player.total_attack();
        let damage = self.enemies[idx].take_damage(player_attack);

        let enemy_name = self.enemies[idx].kind.name();
        self.add_message(format!("You hit {} for {} damage!", enemy_name, damage), 2);

        // Track damage for quests
        let quest_msgs = self.quest_tracker.on_damage_dealt(damage as u32);
        for msg in quest_msgs {
            self.add_message(msg, 5);
        }

        // Check for vampire ring life steal
        if self.player.equipment.values().any(|i| i.kind == ItemKind::RingOfTheVampire) {
            let heal = damage / 4;
            if heal > 0 {
                self.player.heal(heal);
                self.add_message(format!("Life steal: +{} HP", heal), 13);
            }
        }

        if !self.enemies[idx].is_alive() {
            let xp = self.enemies[idx].xp_value;
            let is_boss = self.enemies[idx].kind.is_boss();
            let enemy_kind = self.enemies[idx].kind;

            self.add_message(format!("{} is dead! +{} XP", enemy_name, xp), 5);
            self.player.kills += 1;

            // Track kill for quests
            let quest_msgs = self.quest_tracker.on_enemy_killed(enemy_kind);
            for msg in quest_msgs {
                self.add_message(msg, 5);
            }

            let old_level = self.player.level;
            if self.player.gain_xp(xp) {
                self.add_message(format!("LEVEL UP! You are now level {}!", self.player.level), 11);
                // Track player level for quests
                let quest_msgs = self.quest_tracker.on_player_level_changed(self.player.level);
                for msg in quest_msgs {
                    self.add_message(msg, 5);
                }
            }

            if is_boss {
                self.boss_defeated = true;
                self.add_message("BOSS DEFEATED! The stairs are now accessible!".to_string(), 11);

                // Boss drops legendary loot
                let loot_kinds = [
                    ItemKind::DragonArmor, ItemKind::DragonShield, ItemKind::Scythe,
                    ItemKind::CrownOfKings, ItemKind::AmuletOfTheGods, ItemKind::DragonGauntlets,
                ];
                let loot_kind = loot_kinds[self.rng.gen_range(0..loot_kinds.len())];
                self.items.push(Item::new(
                    self.enemies[idx].x,
                    self.enemies[idx].y,
                    loot_kind,
                    Rarity::Legendary,
                ));

                if self.dungeon_level == 30 {
                    self.victory = true;
                    self.add_message("YOU HAVE DEFEATED THE DEMON KING! VICTORY!".to_string(), 11);
                }
            }

            // Check for completable quests
            self.check_completable_quests();
        }

        self.end_turn();
    }

    /// Trigger a trap
    fn trigger_trap(&mut self) {
        let trap_type = self.rng.gen_range(0..5);
        match trap_type {
            0 => {
                let damage = 5 + self.dungeon_level as i32 / 2;
                self.player.hp -= damage;
                self.add_message(format!("Spike trap! {} damage!", damage), 3);
            }
            1 => {
                self.player.add_status(StatusEffect::Poison, 5);
                self.add_message("Poison dart trap! You are poisoned!".to_string(), 5);
            }
            2 => {
                self.teleport_player_random();
                self.add_message("Teleport trap! You are transported!".to_string(), 7);
            }
            3 => {
                self.player.add_status(StatusEffect::Blind, 10);
                self.add_message("Flash trap! You are blinded!".to_string(), 11);
            }
            _ => {
                // Spawn enemies
                let (px, py) = (self.player.x, self.player.y);
                for _ in 0..2 {
                    let dx = self.rng.gen_range(-2..=2);
                    let dy = self.rng.gen_range(-2..=2);
                    let nx = (px as i32 + dx).max(0) as usize;
                    let ny = (py as i32 + dy).max(0) as usize;
                    if self.map.is_walkable(nx, ny) {
                        let kind = EnemyKind::for_level(self.dungeon_level, &mut self.rng);
                        self.enemies.push(Enemy::new(nx, ny, kind, self.dungeon_level));
                    }
                }
                self.add_message("Alarm trap! Enemies appear!".to_string(), 3);
            }
        }
        self.map.tiles[self.player.y][self.player.x] = Tile::DisarmedTrap;
    }

    /// Open a chest
    fn open_chest(&mut self, x: usize, y: usize) {
        self.map.tiles[y][x] = Tile::OpenChest;

        let num_items = self.rng.gen_range(1..=3);
        for _ in 0..num_items {
            let (kind, mut rarity) = self.random_item();
            // Chest items tend to be better
            if self.rng.gen_bool(0.3) {
                rarity = match rarity {
                    Rarity::Common => Rarity::Uncommon,
                    Rarity::Uncommon => Rarity::Rare,
                    Rarity::Rare => Rarity::Epic,
                    r => r,
                };
            }
            self.items.push(Item::new(x, y, kind, rarity));
        }

        // Gold in chests
        let gold_amount = self.rng.gen_range(10..=50) * self.dungeon_level;
        self.player.gold += gold_amount;

        // Track chest opening for quests
        let quest_msgs = self.quest_tracker.on_chest_opened();
        for msg in quest_msgs {
            self.add_message(msg, 5);
        }

        // Track gold for quests
        let quest_msgs = self.quest_tracker.on_gold_changed(self.player.gold);
        for msg in quest_msgs {
            self.add_message(msg, 5);
        }

        self.check_completable_quests();
        self.add_message(format!("You open the chest! Found {} gold and {} items!", gold_amount, num_items), 11);
        self.end_turn();
    }

    /// Use a shrine
    fn use_shrine(&mut self, x: usize, y: usize) {
        self.map.tiles[y][x] = Tile::UsedShrine;

        // Track shrine usage for quests
        let quest_msgs = self.quest_tracker.on_shrine_used();
        for msg in quest_msgs {
            self.add_message(msg, 5);
        }

        let effect = self.rng.gen_range(0..6);
        match effect {
            0 => {
                self.player.hp = self.player.total_max_hp();
                self.player.mana = self.player.total_max_mana();
                self.add_message("Shrine of Restoration! Fully healed!".to_string(), 13);
            }
            1 => {
                self.player.max_hp += 10;
                self.player.hp += 10;
                self.add_message("Shrine of Vitality! +10 Max HP!".to_string(), 3);
            }
            2 => {
                self.player.base_attack += 3;
                self.add_message("Shrine of Power! +3 Attack!".to_string(), 11);
            }
            3 => {
                self.player.base_defense += 2;
                self.add_message("Shrine of Protection! +2 Defense!".to_string(), 9);
            }
            4 => {
                self.player.max_mana += 15;
                self.player.mana += 15;
                self.add_message("Shrine of Wisdom! +15 Max Mana!".to_string(), 7);
            }
            _ => {
                let xp = 50 * self.dungeon_level;
                let old_level = self.player.level;
                if self.player.gain_xp(xp) {
                    self.add_message(format!("Shrine of Experience! +{} XP! LEVEL UP!", xp), 11);
                    // Track player level for quests
                    let quest_msgs = self.quest_tracker.on_player_level_changed(self.player.level);
                    for msg in quest_msgs {
                        self.add_message(msg, 5);
                    }
                } else {
                    self.add_message(format!("Shrine of Experience! +{} XP!", xp), 5);
                }
            }
        }
        self.check_completable_quests();
        self.end_turn();
    }

    /// Pick up items at the player's position
    pub fn pickup_items(&mut self) {
        let px = self.player.x;
        let py = self.player.y;

        let mut picked_up: Vec<(usize, ItemKind, Rarity)> = Vec::new();

        for (idx, item) in self.items.iter().enumerate() {
            if item.x == px && item.y == py {
                picked_up.push((idx, item.kind, item.rarity));
            }
        }

        let mut gold_collected = false;
        let mut items_collected: Vec<(ItemKind, Rarity)> = Vec::new();

        for (_, kind, rarity) in picked_up.iter().rev() {
            match kind {
                ItemKind::Gold => {
                    let amount = self.rng.gen_range(5..=25) * (1 + self.dungeon_level / 3);
                    self.player.gold += amount;
                    self.add_message(format!("Picked up {} gold!", amount), 11);
                    gold_collected = true;
                }
                ItemKind::Key => {
                    self.player.keys += 1;
                    self.add_message("Picked up a key!".to_string(), 11);
                }
                _ => {
                    let display_name = format!("{}{}", rarity.prefix(), kind.name());
                    if kind.equip_slot().is_some() || kind.is_consumable() {
                        if self.player.inventory.len() < 20 {
                            self.player.inventory.push(Item::new(0, 0, *kind, *rarity));
                            self.add_message(format!("Picked up {}!", display_name), rarity.color_index());
                            items_collected.push((*kind, *rarity));
                        } else {
                            self.add_message("Inventory full!".to_string(), 3);
                        }
                    }
                }
            }
        }

        // Track gold for quests
        if gold_collected {
            let quest_msgs = self.quest_tracker.on_gold_changed(self.player.gold);
            for msg in quest_msgs {
                self.add_message(msg, 5);
            }
        }

        // Track item collection for quests
        for (kind, rarity) in items_collected {
            let quest_msgs = self.quest_tracker.on_item_collected(kind, rarity);
            for msg in quest_msgs {
                self.add_message(msg, 5);
            }
        }

        // Remove picked up items
        let to_remove: Vec<usize> = self.items.iter().enumerate()
            .filter(|(_, item)| item.x == px && item.y == py)
            .map(|(idx, _)| idx)
            .collect();
        for idx in to_remove.into_iter().rev() {
            self.items.remove(idx);
        }

        self.check_completable_quests();
    }

    /// End the current turn
    pub fn end_turn(&mut self) {
        self.turn_count += 1;
        self.enemy_turn();

        // Tick player status effects
        let status_msgs = self.player.tick_status_effects();
        for msg in status_msgs {
            self.add_message(msg, 1);
        }

        // Tick hunger every 20 turns
        if self.turn_count % 20 == 0 {
            if let Some(msg) = self.player.tick_hunger() {
                self.add_message(msg, 3);
            }
        }

        // Regeneration from ring
        if self.player.equipment.values().any(|i| i.kind == ItemKind::RingOfRegeneration) {
            if self.turn_count % 5 == 0 {
                self.player.heal(1);
            }
        }

        // Track turns for quests (every 10 turns to reduce message spam)
        if self.turn_count % 10 == 0 {
            let quest_msgs = self.quest_tracker.on_turn(self.turn_count);
            for msg in quest_msgs {
                self.add_message(msg, 5);
            }
        }

        // Check death
        if self.player.hp <= 0 {
            self.game_over = true;
            self.add_message("You have died! Game Over.".to_string(), 3);
        }
    }

    /// Process enemy turns
    fn enemy_turn(&mut self) {
        let mut attacks: Vec<(usize, i32, Option<StatusEffect>)> = Vec::new();
        let mut moves: Vec<(usize, usize, usize)> = Vec::new();

        let player_invisible = self.player.has_status(StatusEffect::Invisibility);
        let enemy_positions: Vec<(usize, usize)> = self.enemies.iter().map(|e| (e.x, e.y)).collect();

        for (idx, enemy) in self.enemies.iter_mut().enumerate() {
            if !enemy.is_alive() {
                continue;
            }

            // Tick enemy status effects
            let damage_events = enemy.tick_status_effects();
            for (_effect, dmg) in damage_events {
                enemy.hp -= dmg;
                if !enemy.is_alive() {
                    continue;
                }
            }

            if enemy.has_status(StatusEffect::Stun) || enemy.has_status(StatusEffect::Freeze) {
                continue;
            }

            let can_see_player = self.map.visible[enemy.y][enemy.x] && !player_invisible;

            if can_see_player {
                enemy.last_seen_player = Some((self.player.x, self.player.y));
            }

            let target = if can_see_player {
                Some((self.player.x, self.player.y))
            } else {
                enemy.last_seen_player
            };

            if let Some((tx, ty)) = target {
                let dx = tx as i32 - enemy.x as i32;
                let dy = ty as i32 - enemy.y as i32;
                let dist = ((dx * dx + dy * dy) as f32).sqrt();

                if dist < 1.5 && can_see_player {
                    // Attack
                    let mut damage = (enemy.attack - self.player.total_defense()).max(1);

                    // Boss attacks hit harder
                    if enemy.kind.is_boss() {
                        damage = (damage as f32 * 1.5) as i32;
                    }

                    let status = if enemy.kind.can_poison() && self.rng.gen_bool(0.3) {
                        Some(StatusEffect::Poison)
                    } else if enemy.kind.can_burn() && self.rng.gen_bool(0.3) {
                        Some(StatusEffect::Burn)
                    } else if enemy.kind.can_freeze() && self.rng.gen_bool(0.2) {
                        Some(StatusEffect::Freeze)
                    } else if enemy.kind.can_bleed() && self.rng.gen_bool(0.25) {
                        Some(StatusEffect::Bleed)
                    } else {
                        None
                    };

                    attacks.push((idx, damage, status));
                } else if dist < 15.0 {
                    // Move towards target
                    let move_x = dx.signum();
                    let move_y = dy.signum();
                    let new_x = (enemy.x as i32 + move_x).max(0) as usize;
                    let new_y = (enemy.y as i32 + move_y).max(0) as usize;

                    let blocked = enemy_positions.iter().any(|&(ex, ey)| ex == new_x && ey == new_y)
                        || (new_x == self.player.x && new_y == self.player.y);

                    if self.map.is_walkable(new_x, new_y) && !blocked {
                        moves.push((idx, new_x, new_y));
                    }
                }
            }
        }

        // Apply moves
        for (idx, new_x, new_y) in moves {
            self.enemies[idx].x = new_x;
            self.enemies[idx].y = new_y;
        }

        // Apply attacks
        for (idx, damage, status) in attacks {
            if !self.enemies[idx].is_alive() {
                continue;
            }

            // Check for divine shield
            if self.player.has_status(StatusEffect::Shield) {
                self.add_message("Your shield absorbs the attack!".to_string(), 9);
                self.player.remove_status(StatusEffect::Shield);
                continue;
            }

            self.player.hp -= damage;
            let enemy_name = self.enemies[idx].kind.name();
            self.add_message(format!("{} hits you for {} damage!", enemy_name, damage), 3);

            if let Some(effect) = status {
                self.player.add_status(effect, 5);
                self.add_message(format!("You are {}!", effect.name().to_lowercase()), effect.color_index());
            }
        }

        // Remove dead enemies
        self.enemies.retain(|e| e.is_alive());
    }

    /// Use the active skill
    pub fn use_skill(&mut self) {
        if !self.player.can_use_skill() {
            self.add_message("Not enough mana!".to_string(), 3);
            return;
        }

        let skill = self.player.skills[self.player.active_skill];
        self.player.mana -= skill.mana_cost();

        // Track skill usage for quests
        let quest_msgs = self.quest_tracker.on_skill_used();
        for msg in quest_msgs {
            self.add_message(msg, 5);
        }

        match skill {
            Skill::Berserk => {
                self.player.add_status(StatusEffect::Strength, 10);
                self.add_message("BERSERK! Your attacks are empowered!".to_string(), 3);
            }
            Skill::Cleave | Skill::Whirlwind => {
                let (px, py) = (self.player.x as i32, self.player.y as i32);
                let atk = self.player.total_attack();
                let mut hits: Vec<(String, i32)> = Vec::new();
                for enemy in &mut self.enemies {
                    let dx = (enemy.x as i32 - px).abs();
                    let dy = (enemy.y as i32 - py).abs();
                    if dx <= 1 && dy <= 1 && enemy.is_alive() {
                        let damage = enemy.take_damage(atk);
                        hits.push((enemy.kind.name().to_string(), damage));
                    }
                }
                for (name, damage) in &hits {
                    self.add_message(format!("Hit {} for {}!", name, damage), 2);
                }
                self.add_message(format!("Hit {} enemies!", hits.len()), 11);
            }
            Skill::ShieldBash => {
                let (px, py) = (self.player.x as i32, self.player.y as i32);
                let def = self.player.total_defense();
                let mut msg: Option<(String, i32)> = None;
                for enemy in &mut self.enemies {
                    let dx = (enemy.x as i32 - px).abs();
                    let dy = (enemy.y as i32 - py).abs();
                    if dx <= 1 && dy <= 1 && enemy.is_alive() {
                        enemy.add_status(StatusEffect::Stun, 3);
                        let damage = enemy.take_damage(def);
                        msg = Some((enemy.kind.name().to_string(), damage));
                        break;
                    }
                }
                if let Some((name, damage)) = msg {
                    self.add_message(format!("Shield bash! {} stunned for {} damage!", name, damage), 9);
                }
            }
            Skill::Fireball => {
                self.cast_aoe_spell(StatusEffect::Burn, 20, 3, "Fireball", 3);
            }
            Skill::IceSpear => {
                self.cast_aoe_spell(StatusEffect::Freeze, 15, 2, "Ice Spear", 9);
            }
            Skill::Lightning => {
                let mut targets: Vec<usize> = self.enemies.iter().enumerate()
                    .filter(|(_, e)| e.is_alive() && self.map.visible[e.y][e.x])
                    .map(|(i, _)| i)
                    .collect();
                targets.shuffle(&mut self.rng);
                for idx in targets.into_iter().take(3) {
                    let damage = self.enemies[idx].take_damage(25);
                    self.add_message(format!("Lightning strikes {} for {}!", self.enemies[idx].kind.name(), damage), 11);
                }
            }
            Skill::Teleport => {
                self.teleport_player_random();
                self.add_message("You teleport!".to_string(), 7);
            }
            Skill::Backstab => {
                let px = self.player.x as i32;
                let py = self.player.y as i32;
                let atk = self.player.total_attack() * 3;
                let target_idx = self.enemies.iter().enumerate()
                    .filter(|(_, e)| e.is_alive())
                    .min_by_key(|(_, e)| {
                        let dx = e.x as i32 - px;
                        let dy = e.y as i32 - py;
                        dx * dx + dy * dy
                    })
                    .map(|(i, _)| i);
                if let Some(idx) = target_idx {
                    let damage = self.enemies[idx].take_damage(atk);
                    let name = self.enemies[idx].kind.name().to_string();
                    self.add_message(format!("Backstab! {} takes {} damage!", name, damage), 2);
                }
            }
            Skill::Vanish => {
                self.player.add_status(StatusEffect::Invisibility, 10);
                self.add_message("You vanish into the shadows!".to_string(), 1);
            }
            Skill::PoisonBlade | Skill::PoisonArrow => {
                let px = self.player.x as i32;
                let py = self.player.y as i32;
                let atk = self.player.total_attack();
                let target_idx = self.enemies.iter().enumerate()
                    .filter(|(_, e)| e.is_alive())
                    .min_by_key(|(_, e)| {
                        let dx = e.x as i32 - px;
                        let dy = e.y as i32 - py;
                        dx * dx + dy * dy
                    })
                    .map(|(i, _)| i);
                if let Some(idx) = target_idx {
                    self.enemies[idx].add_status(StatusEffect::Poison, 10);
                    let damage = self.enemies[idx].take_damage(atk);
                    let name = self.enemies[idx].kind.name().to_string();
                    self.add_message(format!("Poison attack! {} takes {} damage and is poisoned!", name, damage), 5);
                }
            }
            Skill::ShadowStep => {
                if let Some(enemy) = self.enemies.iter().filter(|e| e.is_alive() && self.map.visible[e.y][e.x]).next() {
                    let dx = enemy.x as i32 - self.player.x as i32;
                    let dy = enemy.y as i32 - self.player.y as i32;
                    let new_x = (enemy.x as i32 - dx.signum()).max(0) as usize;
                    let new_y = (enemy.y as i32 - dy.signum()).max(0) as usize;
                    if self.map.is_walkable(new_x, new_y) {
                        self.player.x = new_x;
                        self.player.y = new_y;
                        self.map.compute_fov(self.player.x, self.player.y);
                        self.add_message("You shadow step!".to_string(), 1);
                    }
                }
            }
            Skill::HolyLight => {
                self.player.heal(20 + self.player.level as i32 * 2);
                let mut hits: Vec<(String, i32)> = Vec::new();
                for enemy in &mut self.enemies {
                    if enemy.kind.is_undead() && self.map.visible[enemy.y][enemy.x] {
                        let damage = enemy.take_damage(30);
                        hits.push((enemy.kind.name().to_string(), damage));
                    }
                }
                for (name, damage) in hits {
                    self.add_message(format!("Holy light burns {} for {}!", name, damage), 11);
                }
                self.add_message("Holy light heals you!".to_string(), 11);
            }
            Skill::DivineShield => {
                self.player.add_status(StatusEffect::Shield, 5);
                self.add_message("Divine shield protects you!".to_string(), 2);
            }
            Skill::Smite => {
                let atk = self.player.total_attack() * 2;
                let target_idx = self.enemies.iter().enumerate()
                    .filter(|(_, e)| e.is_alive() && self.map.visible[e.y][e.x])
                    .next()
                    .map(|(i, _)| i);
                if let Some(idx) = target_idx {
                    let is_undead = self.enemies[idx].kind.is_undead();
                    let damage = if is_undead { atk * 2 } else { atk };
                    let actual = self.enemies[idx].take_damage(damage);
                    let name = self.enemies[idx].kind.name().to_string();
                    self.add_message(format!("Smite! {} takes {} holy damage!", name, actual), 11);
                }
            }
            Skill::Consecrate => {
                self.map.tiles[self.player.y][self.player.x] = Tile::Shrine;
                self.add_message("You consecrate the ground!".to_string(), 13);
            }
            Skill::MultiShot => {
                let atk = self.player.total_attack();
                let targets: Vec<usize> = self.enemies.iter().enumerate()
                    .filter(|(_, e)| e.is_alive() && self.map.visible[e.y][e.x])
                    .map(|(i, _)| i)
                    .take(3)
                    .collect();
                let mut hits: Vec<(String, i32)> = Vec::new();
                for idx in targets {
                    let damage = self.enemies[idx].take_damage(atk);
                    hits.push((self.enemies[idx].kind.name().to_string(), damage));
                }
                for (name, damage) in hits {
                    self.add_message(format!("Arrow hits {} for {}!", name, damage), 2);
                }
            }
            Skill::TrapSet => {
                self.map.tiles[self.player.y][self.player.x] = Tile::Trap;
                self.add_message("You set a trap!".to_string(), 3);
            }
            Skill::EagleEye => {
                self.map.reveal_all();
                self.add_message("You can see the entire floor!".to_string(), 2);
            }
            Skill::RaiseDead => {
                let (px, py) = (self.player.x, self.player.y);
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        let nx = (px as i32 + dx).max(0) as usize;
                        let ny = (py as i32 + dy).max(0) as usize;
                        if self.map.is_walkable(nx, ny) && !(nx == px && ny == py) {
                            let mut minion = Enemy::new(nx, ny, EnemyKind::Skeleton, self.player.level);
                            minion.attack = self.player.total_attack() / 2;
                            self.player.minions.push(minion);
                            self.add_message("You raise a skeleton!".to_string(), 1);
                            break;
                        }
                    }
                }
            }
            Skill::LifeDrain => {
                let target_idx = self.enemies.iter().enumerate()
                    .filter(|(_, e)| e.is_alive() && self.map.visible[e.y][e.x])
                    .next()
                    .map(|(i, _)| i);
                if let Some(idx) = target_idx {
                    let damage = self.enemies[idx].take_damage(15);
                    let name = self.enemies[idx].kind.name().to_string();
                    self.player.heal(damage);
                    self.add_message(format!("Life drain! {} loses {}, you gain {} HP!", name, damage, damage), 13);
                }
            }
            Skill::Curse => {
                for enemy in &mut self.enemies {
                    if self.map.visible[enemy.y][enemy.x] {
                        enemy.add_status(StatusEffect::Weakness, 10);
                    }
                }
                self.add_message("You curse all visible enemies!".to_string(), 14);
            }
            Skill::DarkPact => {
                let sacrifice = self.player.hp / 4;
                self.player.hp -= sacrifice;
                self.player.mana = self.player.total_max_mana();
                self.player.add_status(StatusEffect::Strength, 15);
                self.add_message(format!("Dark pact! Sacrificed {} HP for full mana and power!", sacrifice), 4);
            }
        }

        self.end_turn();
    }

    fn cast_aoe_spell(&mut self, effect: StatusEffect, base_damage: i32, radius: i32, name: &str, color_index: u8) {
        let (px, py) = (self.player.x as i32, self.player.y as i32);
        let mut hit_count = 0;

        for enemy in &mut self.enemies {
            let dx = enemy.x as i32 - px;
            let dy = enemy.y as i32 - py;
            let dist = ((dx * dx + dy * dy) as f32).sqrt() as i32;

            if dist <= radius && enemy.is_alive() {
                let _damage = enemy.take_damage(base_damage);
                enemy.add_status(effect, 5);
                hit_count += 1;
            }
        }

        self.add_message(format!("{}! Hit {} enemies!", name, hit_count), color_index);
    }

    /// Teleport player to a random location
    pub fn teleport_player_random(&mut self) {
        if let Some(room) = self.map.rooms.get(self.rng.gen_range(0..self.map.rooms.len())) {
            let (x, y) = room.random_point(&mut self.rng);
            if self.map.is_walkable(x, y) {
                self.player.x = x;
                self.player.y = y;
                self.map.compute_fov(self.player.x, self.player.y);
            }
        }
    }

    /// Use an item from inventory
    pub fn use_item(&mut self, idx: usize) {
        if idx >= self.player.inventory.len() {
            return;
        }

        let item = self.player.inventory[idx].clone();

        if item.kind.is_food() {
            self.player.eat(item.kind.food_value());
            self.add_message(format!("You eat the {}. Hunger restored!", item.kind.name()), 5);
            self.player.inventory.remove(idx);
            self.end_turn();
            return;
        }

        if item.kind.equip_slot().is_some() {
            if let Some(old) = self.player.equip(item.clone()) {
                self.player.inventory[idx] = old;
                self.add_message(format!("Equipped {}!", item.display_name()), item.rarity.color_index());
            } else {
                self.player.inventory.remove(idx);
                self.add_message(format!("Equipped {}!", item.display_name()), item.rarity.color_index());
            }
            return;
        }

        match item.kind {
            ItemKind::HealthPotion => {
                let heal = 30 + self.player.level as i32 * 5;
                self.player.heal(heal);
                self.add_message(format!("Healed {} HP!", heal), 3);
            }
            ItemKind::ManaPotion => {
                let restore = 25 + self.player.level as i32 * 3;
                self.player.restore_mana(restore);
                self.add_message(format!("Restored {} mana!", restore), 7);
            }
            ItemKind::FullRestorePotion => {
                self.player.hp = self.player.total_max_hp();
                self.player.mana = self.player.total_max_mana();
                self.add_message("Fully restored!".to_string(), 13);
            }
            ItemKind::StrengthPotion => {
                self.player.add_status(StatusEffect::Strength, 20);
                self.add_message("You feel stronger!".to_string(), 11);
            }
            ItemKind::DefensePotion => {
                self.player.add_status(StatusEffect::Shield, 20);
                self.add_message("You feel protected!".to_string(), 9);
            }
            ItemKind::SpeedPotion => {
                self.player.add_status(StatusEffect::Haste, 20);
                self.add_message("You feel faster!".to_string(), 13);
            }
            ItemKind::InvisibilityPotion => {
                self.player.add_status(StatusEffect::Invisibility, 15);
                self.add_message("You turn invisible!".to_string(), 1);
            }
            ItemKind::RegenerationPotion => {
                self.player.add_status(StatusEffect::Regeneration, 30);
                self.add_message("You begin regenerating!".to_string(), 13);
            }
            ItemKind::PoisonResistPotion => {
                self.player.remove_status(StatusEffect::Poison);
                self.add_message("Poison cured!".to_string(), 5);
            }
            ItemKind::ScrollTeleport => {
                self.teleport_player_random();
                self.add_message("You teleport!".to_string(), 7);
            }
            ItemKind::ScrollFireball => {
                self.cast_aoe_spell(StatusEffect::Burn, 30, 4, "Fireball scroll", 3);
            }
            ItemKind::ScrollIceStorm => {
                self.cast_aoe_spell(StatusEffect::Freeze, 25, 5, "Ice storm scroll", 9);
            }
            ItemKind::ScrollLightning => {
                for enemy in &mut self.enemies {
                    if self.map.visible[enemy.y][enemy.x] {
                        enemy.take_damage(40);
                    }
                }
                self.add_message("Lightning strikes all visible enemies!".to_string(), 11);
            }
            ItemKind::ScrollMapping => {
                self.map.reveal_all();
                self.add_message("The map is revealed!".to_string(), 2);
            }
            ItemKind::ScrollMassHeal => {
                self.player.hp = self.player.total_max_hp();
                self.add_message("Mass heal! Fully restored!".to_string(), 3);
            }
            ItemKind::Bomb => {
                self.cast_aoe_spell(StatusEffect::Burn, 50, 3, "Bomb explodes", 3);
            }
            ItemKind::XPPotion => {
                let xp = 100 * self.dungeon_level;
                if self.player.gain_xp(xp) {
                    self.add_message(format!("+{} XP! Level up!", xp), 11);
                } else {
                    self.add_message(format!("+{} XP!", xp), 9);
                }
            }
            _ => {
                self.add_message("Can't use that item.".to_string(), 1);
                return;
            }
        }

        self.player.inventory.remove(idx);
        self.end_turn();
    }

    /// Descend to the next level
    pub fn descend(&mut self) {
        let tile = self.map.tiles[self.player.y][self.player.x];

        if tile == Tile::BossGate {
            if !self.boss_defeated {
                self.add_message("Defeat the boss to proceed!".to_string(), 3);
                return;
            }
        } else if tile != Tile::StairsDown {
            self.add_message("No stairs here.".to_string(), 1);
            return;
        }

        self.dungeon_level += 1;
        self.boss_defeated = false;

        if self.dungeon_level > MAX_DUNGEON_LEVEL {
            self.victory = true;
            self.add_message("You have conquered the dungeon! VICTORY!".to_string(), 11);
            return;
        }

        self.map.generate(&mut self.rng, self.dungeon_level);
        let (px, py) = self.map.rooms[0].center();
        self.player.x = px;
        self.player.y = py;
        self.spawn_enemies();
        self.spawn_items();
        self.map.compute_fov(self.player.x, self.player.y);

        // Track dungeon level for quests
        let quest_msgs = self.quest_tracker.on_dungeon_level_changed(self.dungeon_level);
        for msg in quest_msgs {
            self.add_message(msg, 5);
        }

        // Auto-start new quests that become available at this level
        self.quest_tracker.auto_start_starter_quests(self.dungeon_level, self.player.level);
        self.check_completable_quests();

        let theme = crate::world::DungeonTheme::from_level(self.dungeon_level);
        self.add_message(format!("Descended to {} - Level {}!", theme.name(), self.dungeon_level), 9);

        if BOSS_LEVELS.contains(&self.dungeon_level) {
            self.add_message("A powerful boss awaits on this floor!".to_string(), 3);
        }
    }

    /// Ascend to the previous level
    pub fn ascend(&mut self) {
        if self.map.tiles[self.player.y][self.player.x] != Tile::StairsUp {
            self.add_message("No stairs here.".to_string(), 1);
            return;
        }

        if self.dungeon_level == 1 {
            self.add_message("You can't leave! Defeat the Demon King on level 30!".to_string(), 3);
            return;
        }

        self.dungeon_level -= 1;
        self.map.generate(&mut self.rng, self.dungeon_level);

        if let Some(last_room) = self.map.rooms.last() {
            let (px, py) = last_room.center();
            self.player.x = px;
            self.player.y = py;
        }

        self.spawn_enemies();
        self.spawn_items();
        self.map.compute_fov(self.player.x, self.player.y);
        self.add_message(format!("Returned to level {}.", self.dungeon_level), 9);
    }

    /// Cycle to the next skill
    pub fn cycle_skill(&mut self) {
        if !self.player.skills.is_empty() {
            self.player.active_skill = (self.player.active_skill + 1) % self.player.skills.len();
            let skill = self.player.skills[self.player.active_skill];
            self.add_message(format!("Selected skill: {} ({} mana)", skill.name(), skill.mana_cost()), 7);
        }
    }

    /// Get an AI decision for auto-play
    pub fn ai_decide(&self) -> AIAction {
        AIDecider::decide(
            &self.player,
            &self.enemies,
            &self.items,
            &self.map,
            self.dungeon_level,
            self.boss_defeated,
        )
    }

    /// Execute an AI action
    pub fn ai_execute(&mut self, action: AIAction) {
        match action {
            AIAction::Move(dx, dy) => self.move_player(dx, dy),
            AIAction::UseSkill => self.use_skill(),
            AIAction::UseItem(idx) => self.use_item(idx),
            AIAction::Descend => self.descend(),
            AIAction::Ascend => self.ascend(),
            AIAction::Wait => self.end_turn(),
        }
    }

    // === Quest System Methods ===

    /// Toggle quest display
    pub fn toggle_quests(&mut self) {
        self.show_quests = !self.show_quests;
    }

    /// Check for and notify about completable quests
    fn check_completable_quests(&mut self) {
        let completable = self.quest_tracker.get_completable_quests();
        for quest_id in completable {
            if let Some(quest) = self.quest_tracker.get_quest(quest_id) {
                self.add_message(format!("Quest ready to complete: {}!", quest.name), 11);
            }
        }
    }

    /// Start a quest by ID
    pub fn start_quest(&mut self, quest_id: QuestId) -> bool {
        if let Some(quest) = self.quest_tracker.get_quest(quest_id).cloned() {
            if self.quest_tracker.is_quest_available(&quest, self.dungeon_level, self.player.level) {
                if let Some(msg) = self.quest_tracker.start_quest(quest_id) {
                    self.add_message(msg, 5);
                    return true;
                }
            }
        }
        false
    }

    /// Complete a quest and apply rewards
    pub fn complete_quest(&mut self, quest_id: QuestId) -> bool {
        if let Some(rewards) = self.quest_tracker.complete_quest(quest_id) {
            if let Some(quest) = self.quest_tracker.get_quest(quest_id) {
                self.add_message(format!("Quest completed: {}!", quest.name), 11);
            }

            for reward in rewards {
                self.apply_quest_reward(&reward);
            }
            return true;
        }
        false
    }

    /// Apply a quest reward to the player
    fn apply_quest_reward(&mut self, reward: &QuestReward) {
        match reward {
            QuestReward::Experience(xp) => {
                let old_level = self.player.level;
                if self.player.gain_xp(*xp) {
                    self.add_message(format!("Reward: +{} XP! LEVEL UP to {}!", xp, self.player.level), 11);
                    let quest_msgs = self.quest_tracker.on_player_level_changed(self.player.level);
                    for msg in quest_msgs {
                        self.add_message(msg, 5);
                    }
                } else {
                    self.add_message(format!("Reward: +{} XP", xp), 9);
                }
            }
            QuestReward::Gold(amount) => {
                self.player.gold += amount;
                self.add_message(format!("Reward: +{} Gold", amount), 11);
                let quest_msgs = self.quest_tracker.on_gold_changed(self.player.gold);
                for msg in quest_msgs {
                    self.add_message(msg, 5);
                }
            }
            QuestReward::Item(kind, rarity) => {
                if self.player.inventory.len() < 20 {
                    self.player.inventory.push(Item::new(0, 0, *kind, *rarity));
                    self.add_message(format!("Reward: {}{}", rarity.prefix(), kind.name()), rarity.color_index());
                } else {
                    // Drop item at player's feet if inventory is full
                    self.items.push(Item::new(self.player.x, self.player.y, *kind, *rarity));
                    self.add_message(format!("Reward dropped: {}{} (inventory full)", rarity.prefix(), kind.name()), rarity.color_index());
                }
            }
            QuestReward::RandomItems(count, min_rarity) => {
                for _ in 0..*count {
                    let (kind, mut rarity) = self.random_item();
                    // Ensure minimum rarity
                    if rarity < *min_rarity {
                        rarity = *min_rarity;
                    }
                    if self.player.inventory.len() < 20 {
                        self.player.inventory.push(Item::new(0, 0, kind, rarity));
                        self.add_message(format!("Reward: {}{}", rarity.prefix(), kind.name()), rarity.color_index());
                    } else {
                        self.items.push(Item::new(self.player.x, self.player.y, kind, rarity));
                        self.add_message(format!("Reward dropped: {}{}", rarity.prefix(), kind.name()), rarity.color_index());
                    }
                }
            }
            QuestReward::MaxHpBonus(bonus) => {
                self.player.max_hp += bonus;
                self.player.hp += bonus;
                self.add_message(format!("Reward: +{} Max HP", bonus), 3);
            }
            QuestReward::MaxManaBonus(bonus) => {
                self.player.max_mana += bonus;
                self.player.mana += bonus;
                self.add_message(format!("Reward: +{} Max Mana", bonus), 7);
            }
            QuestReward::AttackBonus(bonus) => {
                self.player.base_attack += bonus;
                self.add_message(format!("Reward: +{} Attack", bonus), 11);
            }
            QuestReward::DefenseBonus(bonus) => {
                self.player.base_defense += bonus;
                self.add_message(format!("Reward: +{} Defense", bonus), 9);
            }
            QuestReward::SkillPoint(points) => {
                self.add_message(format!("Reward: +{} Skill Point(s)", points), 13);
                // Skill points can be implemented in a future skill tree system
            }
            QuestReward::Unlock(feature) => {
                self.add_message(format!("Unlocked: {}", feature), 13);
                // Unlocks can be implemented for special features
            }
        }
    }

    /// Get available quests that can be started
    pub fn get_available_quests(&self) -> Vec<&crate::quests::Quest> {
        self.quest_tracker.get_available_quests(self.dungeon_level, self.player.level)
    }

    /// Get active quest IDs
    pub fn get_active_quest_ids(&self) -> Vec<QuestId> {
        self.quest_tracker.get_active_quest_ids()
    }

    /// Get completable quest IDs
    pub fn get_completable_quest_ids(&self) -> Vec<QuestId> {
        self.quest_tracker.get_completable_quests()
    }

    /// Auto-complete all ready quests
    pub fn auto_complete_quests(&mut self) {
        let completable: Vec<QuestId> = self.quest_tracker.get_completable_quests();
        for quest_id in completable {
            self.complete_quest(quest_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_creation() {
        let game = GameState::new(CharacterClass::Warrior);
        assert!(!game.game_over);
        assert!(!game.victory);
        assert_eq!(game.dungeon_level, 1);
    }

    #[test]
    fn test_player_movement() {
        let mut game = GameState::new(CharacterClass::Warrior);
        let initial_x = game.player.x;
        let initial_y = game.player.y;

        // Try to move - may or may not succeed depending on map layout
        game.move_player(1, 0);

        // Player should have moved or stayed in place
        assert!(game.player.x == initial_x || game.player.x == initial_x + 1);
    }
}
