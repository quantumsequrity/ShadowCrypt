//! Game state and player management for the ShadowCrypt roguelike
//!
//! This module contains the main game state, player structure,
//! and core game logic.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use rand::prelude::*;

use crate::classes::CharacterClass;
use crate::combat::{StatusEffect, Enemy, EnemyKind};
use crate::items::{Item, ItemKind, Rarity, EquipSlot};
use crate::magic::Skill;
use crate::world::{Map, Tile};
use crate::constants::{MAP_WIDTH, MAP_HEIGHT, BOSS_LEVELS};
use crate::ui::Color;

/// The player character
#[derive(Clone, Debug)]
pub struct Player {
    /// X coordinate on the map
    pub x: usize,
    /// Y coordinate on the map
    pub y: usize,
    /// Character class
    pub class: CharacterClass,
    /// Current hit points
    pub hp: i32,
    /// Maximum hit points (base)
    pub max_hp: i32,
    /// Current mana
    pub mana: i32,
    /// Maximum mana (base)
    pub max_mana: i32,
    /// Base attack power
    pub base_attack: i32,
    /// Base defense
    pub base_defense: i32,
    /// Speed (unused currently)
    pub speed: i32,
    /// Gold collected
    pub gold: u32,
    /// Character level
    pub level: u32,
    /// Current experience points
    pub xp: u32,
    /// XP needed for next level
    pub xp_to_level: u32,
    /// Current hunger
    pub hunger: i32,
    /// Maximum hunger
    pub max_hunger: i32,
    /// Number of keys held
    pub keys: u32,
    /// Number of enemies killed
    pub kills: u32,
    /// Active status effects and their remaining duration
    pub status_effects: HashMap<StatusEffect, u32>,
    /// Equipped items by slot
    pub equipment: HashMap<EquipSlot, Item>,
    /// Inventory items
    pub inventory: Vec<Item>,
    /// Available skills
    pub skills: Vec<Skill>,
    /// Currently selected skill index
    pub active_skill: usize,
    /// Summoned minions (for Necromancer)
    pub minions: Vec<Enemy>,
}

impl Player {
    /// Creates a new player of the given class at the specified position
    pub fn new(x: usize, y: usize, class: CharacterClass) -> Self {
        let (hp, attack, defense, mana, speed) = class.base_stats();
        Self {
            x,
            y,
            class,
            hp,
            max_hp: hp,
            mana,
            max_mana: mana,
            base_attack: attack,
            base_defense: defense,
            speed,
            gold: 0,
            level: 1,
            xp: 0,
            xp_to_level: 100,
            hunger: 100,
            max_hunger: 100,
            keys: 0,
            kills: 0,
            status_effects: HashMap::new(),
            equipment: HashMap::new(),
            inventory: Vec::new(),
            skills: Skill::for_class(class),
            active_skill: 0,
            minions: Vec::new(),
        }
    }

    /// Returns total attack power including equipment
    pub fn total_attack(&self) -> i32 {
        let mut total = self.base_attack;
        for item in self.equipment.values() {
            let (atk, _, _, _) = item.stats();
            total += atk;
        }
        if self.has_status(StatusEffect::Strength) {
            total = (total as f32 * 1.5) as i32;
        }
        if self.has_status(StatusEffect::Weakness) {
            total = (total as f32 * 0.5) as i32;
        }
        total
    }

    /// Returns total defense including equipment
    pub fn total_defense(&self) -> i32 {
        let mut total = self.base_defense;
        for item in self.equipment.values() {
            let (_, def, _, _) = item.stats();
            total += def;
        }
        if self.has_status(StatusEffect::Shield) {
            total += 10;
        }
        total
    }

    /// Returns total max HP including equipment
    pub fn total_max_hp(&self) -> i32 {
        let mut total = self.max_hp;
        for item in self.equipment.values() {
            let (_, _, hp, _) = item.stats();
            total += hp;
        }
        total
    }

    /// Returns total max mana including equipment
    pub fn total_max_mana(&self) -> i32 {
        let mut total = self.max_mana;
        for item in self.equipment.values() {
            let (_, _, _, mana) = item.stats();
            total += mana;
        }
        total
    }

    /// Gains XP and returns true if leveled up
    pub fn gain_xp(&mut self, amount: u32) -> bool {
        self.xp += amount;
        if self.xp >= self.xp_to_level {
            self.xp -= self.xp_to_level;
            self.level += 1;
            self.xp_to_level = (self.xp_to_level as f32 * 1.4) as u32;
            self.max_hp += 8 + (self.level as i32 / 3);
            self.hp = self.total_max_hp();
            self.max_mana += 5;
            self.mana = self.total_max_mana();
            self.base_attack += 2;
            self.base_defense += 1;
            return true;
        }
        false
    }

    /// Heals the player
    pub fn heal(&mut self, amount: i32) {
        self.hp = (self.hp + amount).min(self.total_max_hp());
    }

    /// Restores mana
    pub fn restore_mana(&mut self, amount: i32) {
        self.mana = (self.mana + amount).min(self.total_max_mana());
    }

    /// Eats food to restore hunger
    pub fn eat(&mut self, food_value: i32) {
        self.hunger = (self.hunger + food_value).min(self.max_hunger);
    }

    /// Adds a status effect
    pub fn add_status(&mut self, effect: StatusEffect, duration: u32) {
        self.status_effects.insert(effect, duration);
    }

    /// Checks if player has a status effect
    pub fn has_status(&self, effect: StatusEffect) -> bool {
        self.status_effects.contains_key(&effect)
    }

    /// Removes a status effect
    pub fn remove_status(&mut self, effect: StatusEffect) {
        self.status_effects.remove(&effect);
    }

    /// Processes status effects each turn, returns messages
    pub fn tick_status_effects(&mut self) -> Vec<String> {
        let mut messages = Vec::new();
        let mut to_remove = Vec::new();
        let mut damage = 0;
        let mut heal = 0;

        for (effect, duration) in self.status_effects.iter_mut() {
            match effect {
                StatusEffect::Poison => {
                    damage += 2;
                    messages.push("You take poison damage!".to_string());
                }
                StatusEffect::Burn => {
                    damage += 3;
                    messages.push("You are burning!".to_string());
                }
                StatusEffect::Bleed => {
                    damage += 1;
                    messages.push("You are bleeding!".to_string());
                }
                StatusEffect::Regeneration => {
                    heal += 3;
                }
                _ => {}
            }
            *duration = duration.saturating_sub(1);
            if *duration == 0 {
                to_remove.push(*effect);
                messages.push(format!("{} wore off.", effect.name()));
            }
        }

        self.hp -= damage;
        self.heal(heal);

        for effect in to_remove {
            self.status_effects.remove(&effect);
        }

        messages
    }

    /// Processes hunger each turn
    pub fn tick_hunger(&mut self) -> Option<String> {
        self.hunger -= 1;
        if self.hunger <= 0 {
            self.hp -= 1;
            Some("You are starving!".to_string())
        } else if self.hunger < 20 {
            Some("You are very hungry!".to_string())
        } else {
            None
        }
    }

    /// Equips an item, returning the previously equipped item if any
    pub fn equip(&mut self, item: Item) -> Option<Item> {
        if let Some(slot) = item.kind.equip_slot() {
            // Handle rings specially - can wear two
            let actual_slot = if slot == EquipSlot::Ring1 {
                if self.equipment.contains_key(&EquipSlot::Ring1) && !self.equipment.contains_key(&EquipSlot::Ring2) {
                    EquipSlot::Ring2
                } else {
                    EquipSlot::Ring1
                }
            } else {
                slot
            };
            let old = self.equipment.remove(&actual_slot);
            self.equipment.insert(actual_slot, item);
            old
        } else {
            None
        }
    }

    /// Checks if player can use their current skill
    pub fn can_use_skill(&self) -> bool {
        if self.skills.is_empty() {
            return false;
        }
        let skill = self.skills[self.active_skill];
        self.mana >= skill.mana_cost()
    }

    /// Returns the currently selected skill
    pub fn current_skill(&self) -> Option<Skill> {
        if self.skills.is_empty() {
            None
        } else {
            Some(self.skills[self.active_skill])
        }
    }

    /// Cycles to the next skill
    pub fn cycle_skill(&mut self) {
        if !self.skills.is_empty() {
            self.active_skill = (self.active_skill + 1) % self.skills.len();
        }
    }

    /// Finds the index of the first health potion in inventory
    pub fn find_health_potion(&self) -> Option<usize> {
        self.inventory.iter().position(|i| i.kind == ItemKind::HealthPotion)
    }

    /// Finds the index of the first mana potion in inventory
    pub fn find_mana_potion(&self) -> Option<usize> {
        self.inventory.iter().position(|i| i.kind == ItemKind::ManaPotion)
    }
}

/// The main game state
#[derive(Clone)]
pub struct GameState {
    /// The dungeon map
    pub map: Map,
    /// The player
    pub player: Player,
    /// All enemies on the current level
    pub enemies: Vec<Enemy>,
    /// All items on the current level
    pub items: Vec<Item>,
    /// Message log with colors
    pub messages: Vec<(String, Color)>,
    /// Current dungeon level
    pub dungeon_level: u32,
    /// Total turns taken
    pub turn_count: u32,
    /// Whether the game has ended (death)
    pub game_over: bool,
    /// Whether the player won
    pub victory: bool,
    /// Whether the boss on this level has been defeated
    pub boss_defeated: bool,
    /// Whether to show the inventory screen
    pub show_inventory: bool,
    /// Whether to show the help screen
    pub show_help: bool,
    /// Random number generator
    pub rng: StdRng,
}

impl GameState {
    /// Creates a new game with the given class
    pub fn new(class: CharacterClass) -> Self {
        let mut rng = StdRng::from_entropy();
        let mut map = Map::new();
        map.generate(&mut rng, 1);

        let (px, py) = map.rooms[0].center();
        let player = Player::new(px, py, class);

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
            rng,
        };

        state.add_message(
            format!("Welcome, {}! Descend to level 30 to defeat the Demon King!", class.name()),
            Color::CYAN,
        );
        state.add_message(
            format!("Press ? for help. Your skill: {}", class.special_ability()),
            Color::YELLOW,
        );
        state.spawn_enemies();
        state.spawn_items();
        state.map.compute_fov(state.player.x, state.player.y);

        state
    }

    /// Creates a new game with a seeded RNG for testing
    pub fn new_seeded(class: CharacterClass, seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut map = Map::new();
        map.generate(&mut rng, 1);

        let (px, py) = map.rooms[0].center();
        let player = Player::new(px, py, class);

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
            rng,
        };

        state.spawn_enemies();
        state.spawn_items();
        state.map.compute_fov(state.player.x, state.player.y);

        state
    }

    /// Adds a message to the log
    pub fn add_message(&mut self, msg: String, color: Color) {
        self.messages.push((msg, color));
        if self.messages.len() > 6 {
            self.messages.remove(0);
        }
    }

    /// Spawns enemies for the current level
    pub fn spawn_enemies(&mut self) {
        self.enemies.clear();

        let is_boss_level = BOSS_LEVELS.contains(&self.dungeon_level);

        for (i, room) in self.map.rooms.iter().enumerate() {
            if i == 0 {
                continue;
            }

            // Boss room
            if is_boss_level && i == self.map.rooms.len() - 1 {
                if let Some(boss_kind) = EnemyKind::boss_for_level(self.dungeon_level) {
                    let (bx, by) = room.center();
                    self.enemies.push(Enemy::new(bx, by, boss_kind, self.dungeon_level));
                }
                continue;
            }

            // Regular enemies
            let num_enemies = self.rng.gen_range(1..=3);
            for _ in 0..num_enemies {
                let (ex, ey) = room.random_point(&mut self.rng);
                if self.map.is_walkable(ex, ey) {
                    let kind = EnemyKind::for_level(self.dungeon_level, &mut self.rng);
                    self.enemies.push(Enemy::new(ex, ey, kind, self.dungeon_level));
                }
            }
        }
    }

    /// Spawns items for the current level
    pub fn spawn_items(&mut self) {
        self.items.clear();

        for (i, room) in self.map.rooms.iter().enumerate() {
            if i == 0 {
                continue;
            }

            // Spawn gold
            if self.rng.gen_bool(0.7) {
                let (x, y) = room.random_point(&mut self.rng);
                if self.map.is_walkable(x, y) {
                    self.items.push(Item::new(x, y, ItemKind::Gold, Rarity::Common));
                }
            }

            // Spawn health potion
            if self.rng.gen_bool(0.3) {
                let (x, y) = room.random_point(&mut self.rng);
                if self.map.is_walkable(x, y) {
                    self.items.push(Item::new(x, y, ItemKind::HealthPotion, Rarity::Common));
                }
            }

            // Spawn random equipment
            if self.rng.gen_bool(0.15) {
                let (x, y) = room.random_point(&mut self.rng);
                if self.map.is_walkable(x, y) {
                    let kind = self.random_equipment();
                    let rarity = self.random_rarity();
                    self.items.push(Item::new(x, y, kind, rarity));
                }
            }

            // Spawn food
            if self.rng.gen_bool(0.2) {
                let (x, y) = room.random_point(&mut self.rng);
                if self.map.is_walkable(x, y) {
                    let food = [ItemKind::Bread, ItemKind::Meat, ItemKind::Apple, ItemKind::Cheese];
                    let kind = food[self.rng.gen_range(0..food.len())];
                    self.items.push(Item::new(x, y, kind, Rarity::Common));
                }
            }
        }
    }

    /// Returns a random equipment type
    fn random_equipment(&mut self) -> ItemKind {
        let weapons = [
            ItemKind::Dagger, ItemKind::ShortSword, ItemKind::LongSword,
            ItemKind::Axe, ItemKind::Mace, ItemKind::Staff, ItemKind::Bow,
        ];
        let armor = [
            ItemKind::LeatherArmor, ItemKind::ChainMail, ItemKind::LeatherCap,
            ItemKind::LeatherBoots, ItemKind::LeatherGloves,
        ];
        let all: Vec<ItemKind> = weapons.iter().chain(armor.iter()).copied().collect();
        all[self.rng.gen_range(0..all.len())]
    }

    /// Returns a random rarity based on dungeon level
    fn random_rarity(&mut self) -> Rarity {
        let roll: f32 = self.rng.gen();
        let level_bonus = self.dungeon_level as f32 * 0.01;

        if roll < 0.01 + level_bonus * 0.5 {
            Rarity::Mythic
        } else if roll < 0.05 + level_bonus {
            Rarity::Legendary
        } else if roll < 0.15 + level_bonus * 1.5 {
            Rarity::Epic
        } else if roll < 0.30 + level_bonus * 2.0 {
            Rarity::Rare
        } else if roll < 0.55 + level_bonus * 2.5 {
            Rarity::Uncommon
        } else {
            Rarity::Common
        }
    }

    /// Moves the player in a direction
    pub fn move_player(&mut self, dx: i32, dy: i32) {
        if self.game_over || self.victory {
            return;
        }

        let new_x = (self.player.x as i32 + dx) as usize;
        let new_y = (self.player.y as i32 + dy) as usize;

        // Check bounds
        if new_x >= MAP_WIDTH || new_y >= MAP_HEIGHT {
            return;
        }

        // Check for enemy at destination
        if let Some(enemy_idx) = self.enemies.iter().position(|e| e.is_alive() && e.x == new_x && e.y == new_y) {
            self.attack_enemy(enemy_idx);
            self.end_turn();
            return;
        }

        // Check for door
        if self.map.tiles[new_y][new_x] == Tile::Door {
            self.map.tiles[new_y][new_x] = Tile::OpenDoor;
            self.add_message("You open the door.".to_string(), Color::WHITE);
            self.end_turn();
            return;
        }

        // Check walkability
        if !self.map.is_walkable(new_x, new_y) {
            return;
        }

        // Move player
        self.player.x = new_x;
        self.player.y = new_y;

        // Check for items
        self.pickup_items();

        // Check for traps
        if self.map.tiles[new_y][new_x] == Tile::Trap {
            let damage = self.rng.gen_range(5..15);
            self.player.hp -= damage;
            self.add_message(format!("You triggered a trap! {} damage!", damage), Color::RED);
            self.map.tiles[new_y][new_x] = Tile::DisarmedTrap;
        }

        // Check for lava
        if self.map.tiles[new_y][new_x] == Tile::Lava {
            let damage = self.rng.gen_range(10..20);
            self.player.hp -= damage;
            self.add_message(format!("You're burning in lava! {} damage!", damage), Color::RED);
        }

        self.end_turn();
    }

    /// Attacks an enemy
    fn attack_enemy(&mut self, enemy_idx: usize) {
        let damage = self.player.total_attack();
        let actual_damage = self.enemies[enemy_idx].take_damage(damage);

        let enemy_name = self.enemies[enemy_idx].kind.name();
        self.add_message(
            format!("You hit the {} for {} damage!", enemy_name, actual_damage),
            Color::YELLOW,
        );

        if !self.enemies[enemy_idx].is_alive() {
            let xp = self.enemies[enemy_idx].xp_value;
            self.add_message(format!("The {} dies! +{} XP", enemy_name, xp), Color::GREEN);
            self.player.kills += 1;

            if self.player.gain_xp(xp) {
                self.add_message(format!("Level up! You are now level {}!", self.player.level), Color::CYAN);
            }

            // Check if boss was killed
            if self.enemies[enemy_idx].kind.is_boss() {
                self.boss_defeated = true;
                self.add_message("The boss has been defeated!".to_string(), Color::MAGENTA);

                if self.dungeon_level == 30 {
                    self.victory = true;
                    self.add_message("CONGRATULATIONS! You have conquered ShadowCrypt!".to_string(), Color::YELLOW);
                }
            }
        }
    }

    /// Picks up items at the player's position
    fn pickup_items(&mut self) {
        let mut picked_up = Vec::new();

        for (i, item) in self.items.iter().enumerate() {
            if item.x == self.player.x && item.y == self.player.y {
                picked_up.push(i);
            }
        }

        // Process in reverse to avoid index shifting
        for &i in picked_up.iter().rev() {
            let item = self.items.remove(i);
            match item.kind {
                ItemKind::Gold => {
                    let amount = self.rng.gen_range(10..50) * self.dungeon_level;
                    self.player.gold += amount;
                    self.add_message(format!("You picked up {} gold!", amount), Color::YELLOW);
                }
                ItemKind::Key => {
                    self.player.keys += 1;
                    self.add_message("You picked up a key!".to_string(), Color::YELLOW);
                }
                _ => {
                    let name = item.display_name();
                    self.player.inventory.push(item);
                    self.add_message(format!("You picked up {}!", name), Color::BLUE);
                }
            }
        }
    }

    /// Ends the turn and processes enemy actions
    pub fn end_turn(&mut self) {
        self.turn_count += 1;

        // Process player status effects
        let status_messages = self.player.tick_status_effects();
        for msg in status_messages {
            self.add_message(msg, Color::RED);
        }

        // Process hunger
        if let Some(msg) = self.player.tick_hunger() {
            self.add_message(msg, Color::DARK_YELLOW);
        }

        // Check for death
        if self.player.hp <= 0 {
            self.game_over = true;
            self.add_message("You have died!".to_string(), Color::RED);
            return;
        }

        // Process enemy turns
        self.process_enemy_turns();

        // Update FOV
        self.map.compute_fov(self.player.x, self.player.y);
    }

    /// Processes all enemy turns
    fn process_enemy_turns(&mut self) {
        let player_x = self.player.x;
        let player_y = self.player.y;

        for i in 0..self.enemies.len() {
            if !self.enemies[i].is_alive() {
                continue;
            }

            let enemy = &self.enemies[i];
            let visible = self.map.visible[enemy.y][enemy.x];

            // Simple AI: if visible, remember player position
            if visible {
                self.enemies[i].last_seen_player = Some((player_x, player_y));
            }

            // Calculate movement
            if let Some((dx, dy)) = crate::ai::enemy_decide(
                &self.enemies[i],
                player_x,
                player_y,
                visible,
                &self.map,
                &self.enemies,
            ) {
                let new_x = (self.enemies[i].x as i32 + dx) as usize;
                let new_y = (self.enemies[i].y as i32 + dy) as usize;

                // Attack player if adjacent
                if new_x == player_x && new_y == player_y {
                    let damage = self.enemies[i].attack;
                    let actual = (damage - self.player.total_defense()).max(1);
                    self.player.hp -= actual;
                    let name = self.enemies[i].kind.name();
                    self.add_message(format!("The {} hits you for {} damage!", name, actual), Color::RED);

                    // Apply status effects
                    if self.enemies[i].kind.can_poison() && self.rng.gen_bool(0.3) {
                        self.player.add_status(StatusEffect::Poison, 5);
                        self.add_message("You have been poisoned!".to_string(), Color::GREEN);
                    }
                    if self.enemies[i].kind.can_burn() && self.rng.gen_bool(0.3) {
                        self.player.add_status(StatusEffect::Burn, 3);
                        self.add_message("You are on fire!".to_string(), Color::RED);
                    }
                    if self.enemies[i].kind.can_freeze() && self.rng.gen_bool(0.2) {
                        self.player.add_status(StatusEffect::Freeze, 2);
                        self.add_message("You are freezing!".to_string(), Color::CYAN);
                    }
                } else if self.map.is_walkable(new_x, new_y) {
                    // Check if another enemy is there
                    let blocked = self.enemies.iter().enumerate()
                        .any(|(j, e)| j != i && e.is_alive() && e.x == new_x && e.y == new_y);
                    if !blocked {
                        self.enemies[i].x = new_x;
                        self.enemies[i].y = new_y;
                    }
                }
            }

            // Check for player death
            if self.player.hp <= 0 {
                self.game_over = true;
                self.add_message("You have died!".to_string(), Color::RED);
                return;
            }
        }
    }

    /// Descends to the next level
    pub fn descend(&mut self) {
        let tile = self.map.tiles[self.player.y][self.player.x];
        if tile != Tile::StairsDown && tile != Tile::BossGate {
            self.add_message("There are no stairs here.".to_string(), Color::WHITE);
            return;
        }

        if tile == Tile::BossGate && !self.boss_defeated {
            self.add_message("You must defeat the boss first!".to_string(), Color::RED);
            return;
        }

        self.dungeon_level += 1;
        self.boss_defeated = false;

        self.map.generate(&mut self.rng, self.dungeon_level);
        let (px, py) = self.map.rooms[0].center();
        self.player.x = px;
        self.player.y = py;

        self.spawn_enemies();
        self.spawn_items();
        self.map.compute_fov(self.player.x, self.player.y);

        self.add_message(
            format!("You descend to level {}. {}", self.dungeon_level, self.map.theme.name()),
            Color::CYAN,
        );
    }

    /// Ascends to the previous level
    pub fn ascend(&mut self) {
        if self.map.tiles[self.player.y][self.player.x] != Tile::StairsUp {
            self.add_message("There are no stairs up here.".to_string(), Color::WHITE);
            return;
        }

        if self.dungeon_level == 1 {
            self.add_message("You cannot leave the dungeon!".to_string(), Color::RED);
            return;
        }

        self.dungeon_level -= 1;
        self.boss_defeated = false;

        self.map.generate(&mut self.rng, self.dungeon_level);
        let last_room = self.map.rooms.last().unwrap();
        let (px, py) = last_room.center();
        self.player.x = px;
        self.player.y = py;

        self.spawn_enemies();
        self.spawn_items();
        self.map.compute_fov(self.player.x, self.player.y);

        self.add_message(
            format!("You ascend to level {}.", self.dungeon_level),
            Color::CYAN,
        );
    }

    /// Uses an item from inventory
    pub fn use_item(&mut self, index: usize) {
        if index >= self.player.inventory.len() {
            return;
        }

        let item = &self.player.inventory[index];

        // Handle consumables
        if item.kind.is_consumable() {
            let kind = item.kind;
            let name = item.display_name();

            match kind {
                ItemKind::HealthPotion => {
                    self.player.heal(30);
                    self.add_message(format!("You drink the {}. Health restored!", name), Color::GREEN);
                }
                ItemKind::ManaPotion => {
                    self.player.restore_mana(30);
                    self.add_message(format!("You drink the {}. Mana restored!", name), Color::BLUE);
                }
                ItemKind::StrengthPotion => {
                    self.player.add_status(StatusEffect::Strength, 20);
                    self.add_message(format!("You drink the {}. You feel stronger!", name), Color::YELLOW);
                }
                _ if kind.is_food() => {
                    let value = kind.food_value();
                    self.player.eat(value);
                    self.add_message(format!("You eat the {}. Delicious!", name), Color::GREEN);
                }
                _ => {
                    self.add_message(format!("You use the {}.", name), Color::WHITE);
                }
            }

            self.player.inventory.remove(index);
            self.end_turn();
        } else if item.kind.equip_slot().is_some() {
            // Equip the item
            let item = self.player.inventory.remove(index);
            let name = item.display_name();
            if let Some(old) = self.player.equip(item) {
                self.player.inventory.push(old);
            }
            self.add_message(format!("You equip the {}.", name), Color::BLUE);
        }
    }

    /// Uses the current skill
    pub fn use_skill(&mut self) {
        if !self.player.can_use_skill() {
            self.add_message("Not enough mana!".to_string(), Color::RED);
            return;
        }

        let skill = self.player.current_skill().unwrap();
        let cost = skill.mana_cost();
        self.player.mana -= cost;

        self.add_message(format!("You use {}!", skill.name()), Color::CYAN);

        // Simple skill effects
        match skill {
            Skill::Berserk => {
                self.player.add_status(StatusEffect::Strength, 10);
            }
            Skill::HolyLight => {
                self.player.heal(20);
            }
            Skill::Fireball | Skill::IceSpear | Skill::Lightning => {
                // Damage nearest visible enemy
                if let Some(idx) = self.enemies.iter()
                    .position(|e| e.is_alive() && self.map.visible[e.y][e.x])
                {
                    let damage = 20 + self.player.level as i32 * 2;
                    self.enemies[idx].take_damage(damage);
                    let name = self.enemies[idx].kind.name();
                    self.add_message(format!("The {} takes {} damage!", name, damage), Color::YELLOW);
                }
            }
            _ => {}
        }

        self.end_turn();
    }

    /// Cycles to the next skill
    pub fn cycle_skill(&mut self) {
        self.player.cycle_skill();
        if let Some(skill) = self.player.current_skill() {
            self.add_message(format!("Selected skill: {} ({})", skill.name(), skill.mana_cost()), Color::CYAN);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = Player::new(10, 10, CharacterClass::Warrior);
        assert_eq!(player.class, CharacterClass::Warrior);
        assert_eq!(player.hp, 50);
        assert_eq!(player.level, 1);
    }

    #[test]
    fn test_player_leveling() {
        let mut player = Player::new(10, 10, CharacterClass::Warrior);
        let initial_hp = player.max_hp;
        assert!(player.gain_xp(100));
        assert_eq!(player.level, 2);
        assert!(player.max_hp > initial_hp);
    }

    #[test]
    fn test_game_creation() {
        let state = GameState::new_seeded(CharacterClass::Mage, 12345);
        assert_eq!(state.dungeon_level, 1);
        assert!(!state.enemies.is_empty());
    }
}
