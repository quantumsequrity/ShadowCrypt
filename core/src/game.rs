//! Game state and main game logic

use std::collections::HashMap;
use rand::prelude::*;
use rand::rngs::StdRng;
use serde::{Serialize, Deserialize};

use crate::classes::CharacterClass;
use crate::combat::StatusEffect;
use crate::entities::{Enemy, EnemyKind, Player};
use crate::items::{Item, ItemKind, Rarity, EquipSlot};
use crate::magic::Skill;
use crate::world::{Map, Room, Tile, BOSS_LEVELS, MAX_DUNGEON_LEVEL, MAP_WIDTH, MAP_HEIGHT};
use crate::ai::{AIAction, AIDecider};
use crate::quests::{QuestTracker, QuestReward, QuestId};
use crate::achievements::{AchievementTracker, AchievementId};
use crate::npcs::NPCManager;

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
#[derive(Clone, Serialize, Deserialize)]
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
    pub show_achievements: bool,
    pub quest_tracker: QuestTracker,
    pub achievement_tracker: AchievementTracker,
    pub npc_manager: NPCManager,
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

        let mut achievement_tracker = AchievementTracker::new();
        achievement_tracker.reset_run();

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
            show_achievements: false,
            quest_tracker,
            achievement_tracker,
            npc_manager: NPCManager::new(),
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
        use crate::entities::{EntityBehavior, MovementPattern};

        self.enemies.clear();

        let is_boss_level = BOSS_LEVELS.contains(&self.dungeon_level);

        // Collect room centers for patrol routes
        let room_centers: Vec<(usize, usize)> = self.map.rooms.iter()
            .map(|r| r.center())
            .collect();

        let rooms_clone = self.map.rooms.clone();
        for (i, room) in rooms_clone.iter().enumerate() {
            if i == 0 {
                continue;
            }

            // Spawn boss in last room on boss levels
            if is_boss_level && i == self.map.rooms.len() - 1 && !self.boss_defeated {
                if let Some(boss_kind) = EnemyKind::boss_for_level(self.dungeon_level) {
                    let (bx, by) = room.center();
                    let mut boss = Enemy::new_in_room(bx, by, boss_kind, self.dungeon_level, i);
                    // Bosses guard their room
                    boss.ai.behavior = EntityBehavior::Guard;
                    boss.ai.aggro_range = 20;
                    boss.ai.vision_range = 25;
                    self.enemies.push(boss);
                    continue;
                }
            }

            let max_enemies = 2 + (self.dungeon_level as usize / 5);
            let num_enemies = self.rng.gen_range(1..=max_enemies.min(5));

            for _ in 0..num_enemies {
                let (x, y) = room.random_point(&mut self.rng);
                let kind = EnemyKind::for_level(self.dungeon_level, &mut self.rng);
                let mut enemy = Enemy::new_in_room(x, y, kind, self.dungeon_level, i);

                // Set up AI based on enemy type
                self.setup_enemy_ai(&mut enemy, i, &room_centers);

                self.enemies.push(enemy);
            }
        }
    }

    /// Set up AI behavior for a newly spawned enemy
    fn setup_enemy_ai(&mut self, enemy: &mut Enemy, room_idx: usize, room_centers: &[(usize, usize)]) {
        use crate::entities::{EntityBehavior, MovementPattern};

        // Territorial creatures get territory behavior
        if enemy.kind.is_territorial() {
            enemy.ai.behavior = EntityBehavior::Territorial;
            enemy.ai.movement = MovementPattern::new_territory(enemy.x, enemy.y, 10);
            return;
        }

        // Predators hunt
        if enemy.kind.is_predator() {
            enemy.ai.behavior = EntityBehavior::Hunt;
            enemy.ai.aggro_range = 12;
            return;
        }

        // Some enemies patrol between rooms
        let patrol_chance = match enemy.kind {
            EnemyKind::Skeleton | EnemyKind::Zombie | EnemyKind::Ghost
            | EnemyKind::Goblin | EnemyKind::Hobgoblin => 0.6,
            _ => 0.3,
        };

        if self.rng.gen_bool(patrol_chance) && room_centers.len() > 2 {
            // Create patrol route
            let mut waypoints = Vec::new();
            waypoints.push((enemy.x, enemy.y));

            // Add 2-4 nearby room centers to patrol
            let num_waypoints = self.rng.gen_range(2..=4.min(room_centers.len()));
            let mut available_rooms: Vec<usize> = (0..room_centers.len())
                .filter(|&idx| idx != room_idx)
                .collect();
            available_rooms.shuffle(&mut self.rng);

            for &idx in available_rooms.iter().take(num_waypoints) {
                waypoints.push(room_centers[idx]);
            }

            enemy.ai.behavior = EntityBehavior::Patrol;
            enemy.ai.movement = MovementPattern::new_patrol(waypoints);
        } else {
            // Default to wandering within the room area
            enemy.ai.behavior = EntityBehavior::Wander;
            enemy.ai.movement = MovementPattern::Random { radius: 8 };
        }

        // Some enemies sleep (nocturnal)
        if self.rng.gen_bool(0.15) {
            match enemy.kind {
                EnemyKind::Bat | EnemyKind::Ghost | EnemyKind::Vampire
                | EnemyKind::Wraith | EnemyKind::Banshee => {
                    enemy.ai.behavior = EntityBehavior::Sleep;
                }
                _ => {}
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

        // Check for shrine (any type)
        if self.map.tiles[new_y][new_x].is_shrine() {
            self.use_shrine(new_x, new_y);
            return;
        }

        if self.map.is_walkable(new_x, new_y) {
            let old_room = self.get_room_at(self.player.x, self.player.y);
            self.player.x = new_x;
            self.player.y = new_y;
            self.map.compute_fov(self.player.x, self.player.y);

            // Check if entered a new room
            let new_room = self.get_room_at(new_x, new_y);
            if new_room != old_room && new_room.is_some() {
                let quest_msgs = self.quest_tracker.on_room_explored();
                for msg in quest_msgs {
                    self.add_message(msg, 5);
                }
            }

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

    /// Get the room index at a given position, if any
    fn get_room_at(&self, x: usize, y: usize) -> Option<usize> {
        for (i, room) in self.map.rooms.iter().enumerate() {
            if x >= room.x && x < room.x + room.width
                && y >= room.y && y < room.y + room.height
            {
                return Some(i);
            }
        }
        None
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
            let enemy_max_hp = self.enemies[idx].max_hp;

            self.add_message(format!("{} is dead! +{} XP", enemy_name, xp), 5);
            self.player.kills += 1;

            // Track kill for achievements
            let one_hit_kill = damage >= enemy_max_hp;
            self.achievement_tracker.record_kill(enemy_kind, one_hit_kill, self.turn_count);
            self.achievement_tracker.record_damage_dealt(damage as u32, enemy_max_hp as u32, self.turn_count);

            // Check if kill was at 1 HP
            if self.player.hp == 1 {
                self.achievement_tracker.record_kill_at_1hp(self.turn_count);
            }

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
                // Track level for achievements
                self.achievement_tracker.record_level_up(self.player.level, self.turn_count);
            }

            if is_boss {
                self.boss_defeated = true;
                self.add_message("BOSS DEFEATED! The stairs are now accessible!".to_string(), 11);

                // Track boss defeat for achievements
                let boss_turns = self.achievement_tracker.run_stats.turns_in_boss_fight;
                let boss_damage = self.achievement_tracker.run_stats.boss_fight_damage_taken;
                self.achievement_tracker.record_boss_defeat(enemy_kind, boss_turns, boss_damage, self.turn_count);

                // Reset boss fight tracking
                self.achievement_tracker.run_stats.turns_in_boss_fight = 0;
                self.achievement_tracker.run_stats.boss_fight_damage_taken = 0;

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

                    // Track victory for achievements
                    self.achievement_tracker.record_victory(
                        self.player.class,
                        self.turn_count,
                        self.player.level,
                        self.turn_count
                    );
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
            // Track gold for achievements
            self.achievement_tracker.record_gold_collected(self.player.gold, self.turn_count);
        }

        // Track item collection for quests and achievements
        for (kind, rarity) in items_collected {
            let quest_msgs = self.quest_tracker.on_item_collected(kind, rarity);
            for msg in quest_msgs {
                self.add_message(msg, 5);
            }
            // Track item for achievements
            self.achievement_tracker.record_item_found(kind.name(), rarity, self.dungeon_level, self.turn_count);

            // Track weapon equipment for achievements
            if kind.equip_slot() == Some(EquipSlot::Weapon) {
                self.achievement_tracker.record_weapon_equipped(kind.name(), self.turn_count);
            }
        }

        // Track key count for achievements
        self.achievement_tracker.record_keys(self.player.keys, self.turn_count);

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

        // Check combo kills for achievements at end of turn
        self.achievement_tracker.check_combo_kills(self.turn_count);

        // Reset turn-based achievement tracking
        self.achievement_tracker.reset_turn();

        self.enemy_turn();

        // Tick player status effects
        let status_msgs = self.player.tick_status_effects();
        for msg in status_msgs {
            self.add_message(msg, 1);
        }

        // Tick hunger every 20 turns
        if self.turn_count % 20 == 0 {
            let hunger_msgs = self.player.tick_hunger();
            for msg in hunger_msgs {
                self.add_message(msg, 3);
            }
        }

        // Track hunger for achievements
        self.achievement_tracker.record_hunger(self.player.hunger, self.player.max_hunger, self.turn_count);

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

        // Track HP status for achievements
        self.achievement_tracker.record_hp_status(self.player.hp, self.turn_count);

        // Track equipment status for achievements
        self.check_equipment_achievements();

        // Check death
        if self.player.hp <= 0 {
            self.game_over = true;
            self.add_message("You have died! Game Over.".to_string(), 3);
            // Track death for achievements
            self.achievement_tracker.record_death(self.turn_count);
        }

        // Pop achievement notifications
        while let Some(notification) = self.achievement_tracker.pop_notification() {
            self.add_message(notification, 11);
        }
    }

    /// Check equipment-related achievements
    fn check_equipment_achievements(&mut self) {
        let equipped_slots = self.player.equipment.len();
        let legendary_count = self.player.equipment.values()
            .filter(|item| item.rarity >= Rarity::Legendary)
            .count();
        let has_both_rings = self.player.equipment.contains_key(&EquipSlot::Ring1)
            && self.player.equipment.contains_key(&EquipSlot::Ring2);

        self.achievement_tracker.record_fully_equipped(
            equipped_slots,
            legendary_count,
            has_both_rings,
            self.turn_count
        );
    }

    /// Process all entity turns - autonomous world simulation
    fn enemy_turn(&mut self) {
        self.process_autonomous_world();
    }

    /// Process the autonomous world where all entities act independently
    fn process_autonomous_world(&mut self) {
        use crate::entities::{EntityAction, EntityBehavior, EntityFaction, EntityDisposition, MovementPattern};

        let player_invisible = self.player.has_status(StatusEffect::Invisibility);
        let player_pos = (self.player.x, self.player.y);

        // Phase 1: Update all enemy AI states
        for enemy in &mut self.enemies {
            if !enemy.is_alive() {
                continue;
            }

            // Tick enemy status effects
            let damage_events = enemy.tick_status_effects();
            for (_effect, dmg) in damage_events {
                enemy.hp -= dmg;
            }

            // Determine if enemy can see player
            let can_see_player = self.map.visible[enemy.y][enemy.x] && !player_invisible;

            // Update AI state
            enemy.update_ai(can_see_player, Some(player_pos));
            enemy.acted_this_turn = false;
        }

        // Phase 2: Enemy vs Enemy combat (territorial disputes, predator/prey)
        self.process_enemy_interactions();

        // Phase 3: Process each enemy's autonomous action
        let mut player_attacks: Vec<(usize, i32, Option<StatusEffect>)> = Vec::new();
        let mut enemy_moves: Vec<(usize, usize, usize)> = Vec::new();
        let mut sound_events: Vec<(usize, usize, usize)> = Vec::new();

        // Get positions snapshot to avoid borrow issues
        let enemy_positions: Vec<(u64, usize, usize)> = self.enemies.iter()
            .filter(|e| e.is_alive())
            .map(|e| (e.id, e.x, e.y))
            .collect();

        for (idx, enemy) in self.enemies.iter_mut().enumerate() {
            if !enemy.is_alive() || enemy.acted_this_turn {
                continue;
            }

            // Check if stunned or frozen
            if enemy.has_status(StatusEffect::Stun) || enemy.has_status(StatusEffect::Freeze) {
                continue;
            }

            // Get the enemy's decision
            let action = enemy.decide_action(&[], player_pos, &self.map.visible);

            match action {
                EntityAction::Attack(tx, ty) => {
                    // Check if attacking player
                    if tx == self.player.x && ty == self.player.y {
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

                        player_attacks.push((idx, damage, status));
                    }
                    // Note: Enemy vs enemy attacks are handled in process_enemy_interactions
                }

                EntityAction::Move(dx, dy) => {
                    let new_x = (enemy.x as i32 + dx).max(0) as usize;
                    let new_y = (enemy.y as i32 + dy).max(0) as usize;

                    // Check for collision with other enemies
                    let blocked = enemy_positions.iter()
                        .any(|(id, ex, ey)| *id != enemy.id && *ex == new_x && *ey == new_y)
                        || (new_x == self.player.x && new_y == self.player.y);

                    if self.map.is_walkable(new_x, new_y) && !blocked {
                        enemy_moves.push((idx, new_x, new_y));
                    }
                }

                EntityAction::PatrolNext => {
                    enemy.advance_patrol();
                }

                EntityAction::Flee => {
                    // Move away from player
                    let dx = (enemy.x as i32 - player_pos.0 as i32).signum();
                    let dy = (enemy.y as i32 - player_pos.1 as i32).signum();
                    let new_x = (enemy.x as i32 + dx).max(0) as usize;
                    let new_y = (enemy.y as i32 + dy).max(0) as usize;

                    let blocked = enemy_positions.iter()
                        .any(|(id, ex, ey)| *id != enemy.id && *ex == new_x && *ey == new_y)
                        || (new_x == self.player.x && new_y == self.player.y);

                    if self.map.is_walkable(new_x, new_y) && !blocked {
                        enemy_moves.push((idx, new_x, new_y));
                    }
                }

                EntityAction::Rest => {
                    // Enemy rests and heals slightly
                    if enemy.hp < enemy.max_hp {
                        enemy.hp = (enemy.hp + 1).min(enemy.max_hp);
                    }
                }

                EntityAction::MakeNoise(radius) => {
                    // Alert nearby entities (deferred to avoid double mutable borrow)
                    sound_events.push((enemy.x, enemy.y, radius as usize));
                }

                _ => {} // Wait or other actions
            }

            enemy.acted_this_turn = true;
        }

        // Propagate deferred sound events
        for (sx, sy, radius) in sound_events {
            self.propagate_sound(sx, sy, radius);
        }

        // Apply enemy moves
        for (idx, new_x, new_y) in enemy_moves {
            if idx < self.enemies.len() {
                self.enemies[idx].x = new_x;
                self.enemies[idx].y = new_y;
            }
        }

        // Apply player attacks
        for (idx, damage, status) in player_attacks {
            if idx >= self.enemies.len() || !self.enemies[idx].is_alive() {
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

    /// Process enemy vs enemy interactions (territorial combat, predation)
    fn process_enemy_interactions(&mut self) {
        use crate::entities::{EntityFaction, EntityDisposition, EntityBehavior};

        let mut combat_pairs: Vec<(usize, usize, i32)> = Vec::new(); // attacker_idx, defender_idx, damage
        let mut combat_messages: Vec<String> = Vec::new();

        // Find potential combat pairs
        for i in 0..self.enemies.len() {
            if !self.enemies[i].is_alive() {
                continue;
            }

            let attacker_faction = self.enemies[i].kind.faction();
            let attacker_territorial = self.enemies[i].kind.is_territorial();
            let attacker_predator = self.enemies[i].kind.is_predator();

            for j in 0..self.enemies.len() {
                if i == j || !self.enemies[j].is_alive() {
                    continue;
                }

                let defender_faction = self.enemies[j].kind.faction();
                let disposition = attacker_faction.disposition_towards(&defender_faction);

                // Check if they're adjacent
                let dx = (self.enemies[i].x as i32 - self.enemies[j].x as i32).abs();
                let dy = (self.enemies[i].y as i32 - self.enemies[j].y as i32).abs();

                if dx > 1 || dy > 1 {
                    continue;
                }

                // Determine if combat should occur
                let should_fight = match disposition {
                    EntityDisposition::Hostile => true,
                    EntityDisposition::Neutral => {
                        // Territorial creatures attack intruders
                        if attacker_territorial {
                            // Random chance to attack neutral creatures in territory
                            self.rng.gen_bool(0.3)
                        } else {
                            false
                        }
                    }
                    EntityDisposition::Fearful => false,
                    _ => false,
                };

                // Check predator/prey relationship
                let is_prey = if attacker_predator {
                    self.enemies[i].kind.prey_factions().contains(&defender_faction)
                } else {
                    false
                };

                if should_fight || is_prey {
                    // Calculate damage
                    let damage = (self.enemies[i].attack - self.enemies[j].defense).max(1);
                    combat_pairs.push((i, j, damage));
                }
            }
        }

        // Apply combat (limit to prevent infinite loops)
        for (attacker_idx, defender_idx, damage) in combat_pairs.into_iter().take(5) {
            if attacker_idx >= self.enemies.len() || defender_idx >= self.enemies.len() {
                continue;
            }

            if !self.enemies[attacker_idx].is_alive() || !self.enemies[defender_idx].is_alive() {
                continue;
            }

            self.enemies[defender_idx].hp -= damage;

            let attacker_name = self.enemies[attacker_idx].kind.name();
            let defender_name = self.enemies[defender_idx].kind.name();

            // Only show message if in player's view
            if self.map.visible[self.enemies[attacker_idx].y][self.enemies[attacker_idx].x] {
                self.add_message(
                    format!("{} attacks {} for {} damage!", attacker_name, defender_name, damage),
                    6 // Orange color for enemy combat
                );
            }

            // Check if defender died
            if !self.enemies[defender_idx].is_alive() {
                if self.map.visible[self.enemies[defender_idx].y][self.enemies[defender_idx].x] {
                    self.add_message(
                        format!("{} was slain by {}!", defender_name, attacker_name),
                        4 // Red for death
                    );
                }
            }
        }
    }

    /// Propagate sound to alert nearby enemies
    fn propagate_sound(&mut self, x: usize, y: usize, radius: usize) {
        use crate::entities::EntityBehavior;

        for enemy in &mut self.enemies {
            if !enemy.is_alive() {
                continue;
            }

            let dx = (enemy.x as i32 - x as i32).abs() as usize;
            let dy = (enemy.y as i32 - y as i32).abs() as usize;
            let dist = ((dx * dx + dy * dy) as f32).sqrt() as usize;

            if dist <= radius {
                // Add interest point for investigation
                let priority = ((radius - dist) * 10) as u32;
                enemy.ai.add_interest(x, y, priority);

                // Wake sleeping enemies
                if enemy.ai.behavior == EntityBehavior::Sleep && dist <= radius / 2 {
                    enemy.ai.behavior = EntityBehavior::Investigate;
                }
            }
        }
    }

    /// Simulate world time passing (called even when player doesn't act)
    pub fn simulate_world_tick(&mut self) {
        use crate::entities::EntityBehavior;
        use crate::npcs::{NPCAction, NPCBehavior};

        // Update all enemy AIs
        let player_invisible = self.player.has_status(StatusEffect::Invisibility);
        let player_pos = (self.player.x, self.player.y);

        for enemy in &mut self.enemies {
            if !enemy.is_alive() {
                continue;
            }

            let can_see_player = self.map.visible[enemy.y][enemy.x] && !player_invisible;
            enemy.update_ai(can_see_player, Some(player_pos));

            // Decay behavior timer for idle returns
            if enemy.ai.behavior_timer > 20 {
                match enemy.ai.behavior {
                    EntityBehavior::Hunt => {
                        if enemy.ai.turns_since_player > 15 {
                            enemy.ai.behavior = EntityBehavior::Patrol;
                            enemy.ai.behavior_timer = 0;
                        }
                    }
                    EntityBehavior::Investigate => {
                        if enemy.ai.interest_points.is_empty() {
                            enemy.ai.behavior = EntityBehavior::Wander;
                            enemy.ai.behavior_timer = 0;
                        }
                    }
                    _ => {}
                }
            }
        }

        // Process enemy movements (wandering, patrolling)
        self.process_idle_movements();

        // Process NPC autonomous behavior
        self.process_npc_behavior();

        // Process companion autonomous actions
        self.process_companion_behavior();
    }

    /// Process NPC autonomous behavior
    fn process_npc_behavior(&mut self) {
        use crate::npcs::{NPCAction, NPCBehavior};

        // Get enemy positions to check for danger
        let enemy_positions: Vec<(usize, usize)> = self.enemies.iter()
            .filter(|e| e.is_alive())
            .map(|e| (e.x, e.y))
            .collect();

        // Get NPC positions for collision detection
        let npc_positions: Vec<(u32, usize, usize)> = self.npc_manager.npcs.iter()
            .map(|n| (n.id, n.x, n.y))
            .collect();

        // Collect deferred messages to avoid borrowing self during NPC iteration
        let mut deferred_npc_messages: Vec<(String, u8)> = Vec::new();

        // Update each NPC
        for npc in &mut self.npc_manager.npcs {
            // Update NPC state
            npc.update(&mut self.rng, self.dungeon_level);

            // Check for danger
            npc.check_danger(&enemy_positions);

            // Decide action
            let action = npc.decide_action(&mut self.rng);

            match action {
                NPCAction::Move(dx, dy) => {
                    let new_x = (npc.x as i32 + dx).max(0) as usize;
                    let new_y = (npc.y as i32 + dy).max(0) as usize;

                    // Check collisions
                    let blocked_by_npc = npc_positions.iter()
                        .any(|(id, x, y)| *id != npc.id && *x == new_x && *y == new_y);
                    let blocked_by_enemy = enemy_positions.iter()
                        .any(|(x, y)| *x == new_x && *y == new_y);
                    let blocked_by_player = new_x == self.player.x && new_y == self.player.y;

                    // Check if within wander radius
                    let within_radius = {
                        let dx = (new_x as i32 - npc.schedule.home_pos.0 as i32).abs() as usize;
                        let dy = (new_y as i32 - npc.schedule.home_pos.1 as i32).abs() as usize;
                        dx <= npc.schedule.wander_radius && dy <= npc.schedule.wander_radius
                    };

                    if self.map.is_walkable(new_x, new_y)
                        && !blocked_by_npc
                        && !blocked_by_enemy
                        && !blocked_by_player
                        && (within_radius || npc.behavior == NPCBehavior::Fleeing)
                    {
                        npc.x = new_x;
                        npc.y = new_y;
                    }
                }

                NPCAction::Talk => {
                    // Generate ambient dialogue if player is nearby and can see
                    if self.map.visible[npc.y][npc.x] {
                        if let Some(line) = npc.get_ambient_line(&mut self.rng) {
                            let name = npc.display_name();
                            deferred_npc_messages.push((format!("{}: \"{}\"", name, line), 9));
                        }
                    }
                }

                _ => {} // Wait, Rest, etc.
            }

            // Check for NPC-NPC interactions (socializing)
            if npc.behavior != NPCBehavior::Socializing && npc.action_cooldown == 0 {
                for (other_id, ox, oy) in &npc_positions {
                    if *other_id == npc.id {
                        continue;
                    }

                    let dx = (npc.x as i32 - *ox as i32).abs();
                    let dy = (npc.y as i32 - *oy as i32).abs();

                    // Adjacent NPCs may start conversations
                    if dx <= 2 && dy <= 2 && self.rng.gen_bool(0.02) {
                        npc.try_socialize(*other_id);
                        break;
                    }
                }
            }

            // Wandering NPCs return to stationary after a while
            if npc.behavior == NPCBehavior::Fleeing {
                let dx = (npc.x as i32 - npc.schedule.home_pos.0 as i32).abs();
                let dy = (npc.y as i32 - npc.schedule.home_pos.1 as i32).abs();
                if dx <= 2 && dy <= 2 {
                    npc.behavior = NPCBehavior::Wandering;
                }
            }
        }

        // Apply deferred NPC messages
        for (msg, priority) in deferred_npc_messages {
            self.add_message(msg, priority);
        }
    }

    /// Process companion autonomous behavior
    fn process_companion_behavior(&mut self) {
        use crate::companions::{CompanionAction, CompanionAI, CompanionBehavior};

        let player_pos = (self.player.x, self.player.y);

        // Collect deferred messages to avoid borrowing self.messages during iteration
        let mut deferred_messages: Vec<(String, u8)> = Vec::new();

        // Snapshot enemy positions for collision detection
        let enemy_positions: Vec<(usize, usize, bool)> = self.enemies.iter()
            .map(|e| (e.x, e.y, e.is_alive()))
            .collect();

        // Snapshot companion positions for collision detection
        let companion_positions: Vec<(String, usize, usize)> = self.player.companions.iter()
            .map(|c| (c.name.clone(), c.x, c.y))
            .collect();

        let num_companions = self.player.companions.len();

        // Process each companion by index
        for ci in 0..num_companions {
            if !self.player.companions[ci].is_alive() {
                continue;
            }

            // Tick companion status effects
            let tick_damage = self.player.companions[ci].tick();
            if tick_damage > 0 {
                // Show status damage if in view
                let cy = self.player.companions[ci].y;
                let cx = self.player.companions[ci].x;
                if self.map.visible[cy][cx] {
                    deferred_messages.push((
                        format!("{} takes {} status damage!", self.player.companions[ci].name, tick_damage),
                        3
                    ));
                }
            }

            // Decide action based on AI
            let action = CompanionAI::decide(&self.player.companions[ci], player_pos, &self.enemies, &self.map);

            match action {
                CompanionAction::Move(dx, dy) => {
                    let new_x = (self.player.companions[ci].x as i32 + dx).max(0) as usize;
                    let new_y = (self.player.companions[ci].y as i32 + dy).max(0) as usize;

                    // Check collisions using snapshots
                    let blocked = enemy_positions.iter()
                        .any(|(ex, ey, alive)| *alive && *ex == new_x && *ey == new_y)
                        || (new_x == self.player.x && new_y == self.player.y)
                        || companion_positions.iter()
                            .any(|(name, cx, cy)| *cx == new_x && *cy == new_y && *name != self.player.companions[ci].name);

                    if self.map.is_walkable(new_x, new_y) && !blocked {
                        self.player.companions[ci].x = new_x;
                        self.player.companions[ci].y = new_y;
                    }
                }

                CompanionAction::Follow => {
                    // Follow - move towards player
                    let dx = (player_pos.0 as i32 - self.player.companions[ci].x as i32).signum();
                    let dy = (player_pos.1 as i32 - self.player.companions[ci].y as i32).signum();

                    let new_x = (self.player.companions[ci].x as i32 + dx).max(0) as usize;
                    let new_y = (self.player.companions[ci].y as i32 + dy).max(0) as usize;

                    // Check collisions using snapshots
                    let blocked = enemy_positions.iter()
                        .any(|(ex, ey, alive)| *alive && *ex == new_x && *ey == new_y)
                        || (new_x == self.player.x && new_y == self.player.y)
                        || companion_positions.iter()
                            .any(|(name, cx, cy)| *cx == new_x && *cy == new_y && *name != self.player.companions[ci].name);

                    if self.map.is_walkable(new_x, new_y) && !blocked {
                        self.player.companions[ci].x = new_x;
                        self.player.companions[ci].y = new_y;
                    }
                }

                CompanionAction::Attack(enemy_idx) => {
                    if enemy_idx < self.enemies.len() && self.enemies[enemy_idx].is_alive() {
                        let damage = (self.player.companions[ci].attack - self.enemies[enemy_idx].defense).max(1);
                        self.enemies[enemy_idx].hp -= damage;

                        // Track damage and potentially kills
                        self.player.companions[ci].damage_dealt += damage as u32;

                        let comp_y = self.player.companions[ci].y;
                        let comp_x = self.player.companions[ci].x;
                        if self.map.visible[comp_y][comp_x] {
                            deferred_messages.push((
                                format!("{} attacks {} for {} damage!",
                                    self.player.companions[ci].name,
                                    self.enemies[enemy_idx].kind.name(),
                                    damage),
                                5 // Green for ally attacks
                            ));
                        }

                        // Check if enemy died
                        if !self.enemies[enemy_idx].is_alive() {
                            self.player.companions[ci].kills += 1;
                            let xp = self.enemies[enemy_idx].xp_value;
                            if self.player.companions[ci].gain_xp(xp / 2) {
                                if self.map.visible[comp_y][comp_x] {
                                    deferred_messages.push((
                                        format!("{} leveled up to {}!", self.player.companions[ci].name, self.player.companions[ci].level),
                                        11
                                    ));
                                }
                            }
                        }
                    }
                }

                CompanionAction::UseAbility(enemy_idx) => {
                    if self.player.companions[ci].can_use_ability() {
                        let ability = self.player.companions[ci].use_ability();
                        let power = self.player.companions[ci].ability_power();
                        let comp_x = self.player.companions[ci].x;
                        let comp_y = self.player.companions[ci].y;

                        if self.map.visible[comp_y][comp_x] {
                            deferred_messages.push((
                                format!("{} uses {}!", self.player.companions[ci].name, ability.name()),
                                13 // Magenta for abilities
                            ));
                        }

                        // Apply ability effects based on type
                        match ability {
                            crate::companions::CompanionAbility::NatureHeal => {
                                self.player.hp = (self.player.hp + power).min(self.player.total_max_hp());
                                deferred_messages.push((format!("You are healed for {}!", power), 5));
                            }
                            crate::companions::CompanionAbility::FrostNova => {
                                // Freeze nearby enemies
                                for enemy in &mut self.enemies {
                                    let dx = (enemy.x as i32 - comp_x as i32).abs();
                                    let dy = (enemy.y as i32 - comp_y as i32).abs();
                                    if dx <= 2 && dy <= 2 && enemy.is_alive() {
                                        enemy.add_status(StatusEffect::Freeze, 2);
                                    }
                                }
                            }
                            crate::companions::CompanionAbility::FlameAura => {
                                // Burn adjacent enemies
                                for enemy in &mut self.enemies {
                                    let dx = (enemy.x as i32 - comp_x as i32).abs();
                                    let dy = (enemy.y as i32 - comp_y as i32).abs();
                                    if dx <= 1 && dy <= 1 && enemy.is_alive() {
                                        enemy.hp -= power / 2;
                                        enemy.add_status(StatusEffect::Burn, 3);
                                    }
                                }
                            }
                            crate::companions::CompanionAbility::Terrify => {
                                // Stun nearby enemies
                                for enemy in &mut self.enemies {
                                    let dx = (enemy.x as i32 - comp_x as i32).abs();
                                    let dy = (enemy.y as i32 - comp_y as i32).abs();
                                    if dx <= 3 && dy <= 3 && enemy.is_alive() {
                                        enemy.add_status(StatusEffect::Stun, 2);
                                    }
                                }
                            }
                            _ => {
                                // Default: damage to target
                                if enemy_idx < self.enemies.len() && self.enemies[enemy_idx].is_alive() {
                                    self.enemies[enemy_idx].hp -= power;
                                }
                            }
                        }
                    }
                }

                CompanionAction::Heal => {
                    if self.player.companions[ci].can_use_ability() {
                        let _ability = self.player.companions[ci].use_ability();
                        let power = self.player.companions[ci].ability_power();

                        self.player.hp = (self.player.hp + power).min(self.player.total_max_hp());
                        deferred_messages.push((
                            format!("{} heals you for {} HP!", self.player.companions[ci].name, power),
                            5
                        ));
                    }
                }

                CompanionAction::Flee => {
                    // Move away from nearest enemy
                    let comp_x = self.player.companions[ci].x;
                    let comp_y = self.player.companions[ci].y;
                    let flee_dir = self.enemies.iter()
                        .filter(|e| e.is_alive())
                        .min_by_key(|e| {
                            let dx = e.x as i32 - comp_x as i32;
                            let dy = e.y as i32 - comp_y as i32;
                            dx * dx + dy * dy
                        })
                        .map(|nearest| {
                            let dx = (comp_x as i32 - nearest.x as i32).signum();
                            let dy = (comp_y as i32 - nearest.y as i32).signum();
                            (dx, dy)
                        });

                    if let Some((dx, dy)) = flee_dir {
                        let new_x = (comp_x as i32 + dx).max(0) as usize;
                        let new_y = (comp_y as i32 + dy).max(0) as usize;

                        if self.map.is_walkable(new_x, new_y) {
                            self.player.companions[ci].x = new_x;
                            self.player.companions[ci].y = new_y;
                        }
                    }
                }

                CompanionAction::Wait => {} // Do nothing
            }
        }

        // Apply deferred messages
        for (msg, color) in deferred_messages {
            self.add_message(msg, color);
        }

        // Remove dead enemies
        self.enemies.retain(|e| e.is_alive());
    }

    /// Process idle enemy movements (wandering, patrolling)
    fn process_idle_movements(&mut self) {
        use crate::entities::{EntityBehavior, MovementPattern};

        let enemy_positions: Vec<(u64, usize, usize)> = self.enemies.iter()
            .filter(|e| e.is_alive())
            .map(|e| (e.id, e.x, e.y))
            .collect();

        for enemy in &mut self.enemies {
            if !enemy.is_alive() {
                continue;
            }

            // Only process idle/patrol/wander behaviors
            match enemy.ai.behavior {
                EntityBehavior::Wander | EntityBehavior::Patrol | EntityBehavior::Idle => {}
                _ => continue,
            }

            // 30% chance to move each tick for wanderers
            if enemy.ai.behavior == EntityBehavior::Wander && self.rng.gen_bool(0.3) {
                let dx = self.rng.gen_range(-1..=1);
                let dy = self.rng.gen_range(-1..=1);

                if dx != 0 || dy != 0 {
                    let new_x = (enemy.x as i32 + dx).max(0) as usize;
                    let new_y = (enemy.y as i32 + dy).max(0) as usize;

                    let blocked = enemy_positions.iter()
                        .any(|(id, ex, ey)| *id != enemy.id && *ex == new_x && *ey == new_y)
                        || (new_x == self.player.x && new_y == self.player.y);

                    if self.map.is_walkable(new_x, new_y) && !blocked {
                        enemy.x = new_x;
                        enemy.y = new_y;
                    }
                }
            }

            // Process patrol movements
            if enemy.ai.behavior == EntityBehavior::Patrol {
                if let MovementPattern::Patrol { waypoints, current_idx, .. } = &enemy.ai.movement {
                    if !waypoints.is_empty() {
                        let (tx, ty) = waypoints[*current_idx];
                        let dx = (tx as i32 - enemy.x as i32).signum();
                        let dy = (ty as i32 - enemy.y as i32).signum();

                        if dx == 0 && dy == 0 {
                            // Reached waypoint, advance
                            enemy.advance_patrol();
                        } else {
                            let new_x = (enemy.x as i32 + dx).max(0) as usize;
                            let new_y = (enemy.y as i32 + dy).max(0) as usize;

                            let blocked = enemy_positions.iter()
                                .any(|(id, ex, ey)| *id != enemy.id && *ex == new_x && *ey == new_y)
                                || (new_x == self.player.x && new_y == self.player.y);

                            if self.map.is_walkable(new_x, new_y) && !blocked {
                                enemy.x = new_x;
                                enemy.y = new_y;
                            }
                        }
                    }
                }
            }
        }
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
            AIAction::Wait | AIAction::Rest => self.end_turn(),
            AIAction::Flee(dx, dy) => self.move_player(dx, dy),
            AIAction::EquipItem(idx) => self.use_item(idx),
            AIAction::DropItem(_) => self.end_turn(),
            AIAction::CycleSkill => self.cycle_skill(),
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
