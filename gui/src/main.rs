//! ShadowCrypt GUI - Enhanced Dark Fantasy Edition
//!
//! A visually stunning roguelike GUI featuring:
//! - Dark fantasy theme with consistent color palette
//! - Advanced lighting system with fog of war
//! - Particle effects for combat and magic
//! - Professional UI layout with panels
//! - Interactive elements with tooltips and context menus
//! - Multiple windows: Inventory, Character, Skills, Settings

mod particles;

use eframe::egui::{self, Color32, FontId, Pos2, Rect, RichText, Rounding, Sense, Stroke, Vec2};
use shadowcrypt_core::ai::{AIAction, AutoPlayAI};
use shadowcrypt_core::classes::CharacterClass;
use shadowcrypt_core::combat::StatusEffect;
use shadowcrypt_core::items::{EquipSlot, ItemKind, Rarity};
use shadowcrypt_core::magic::Skill;
use shadowcrypt_core::prelude::*;
// Color struct and helper functions (defined locally since core has no public ui module)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

fn rarity_color(rarity: Rarity) -> Color {
    match rarity {
        Rarity::Common => Color::new(180, 180, 180),
        Rarity::Uncommon => Color::new(80, 200, 80),
        Rarity::Rare => Color::new(80, 140, 255),
        Rarity::Epic => Color::new(180, 80, 255),
        Rarity::Legendary => Color::new(255, 165, 0),
        Rarity::Mythic => Color::new(255, 50, 50),
    }
}

fn enemy_color(kind: EnemyKind) -> Color {
    match kind {
        EnemyKind::Goblin | EnemyKind::Kobold | EnemyKind::Hobgoblin => Color::new(100, 180, 100),
        EnemyKind::Skeleton | EnemyKind::Zombie | EnemyKind::Mummy => Color::new(200, 200, 200),
        EnemyKind::Ghost | EnemyKind::Wraith | EnemyKind::Banshee => Color::new(150, 200, 255),
        EnemyKind::Troll | EnemyKind::CaveOgre | EnemyKind::ForestTroll => Color::new(100, 140, 80),
        EnemyKind::FireElemental | EnemyKind::FireDrake | EnemyKind::Hellhound => Color::new(255, 140, 50),
        EnemyKind::IceElemental | EnemyKind::IceWraith | EnemyKind::FrostGiant => Color::new(100, 180, 255),
        _ => Color::new(200, 60, 60),
    }
}

fn tile_color(tile: Tile) -> Color {
    match tile {
        Tile::Floor => Color::new(80, 80, 90),
        Tile::Wall => Color::new(120, 110, 100),
        Tile::StairsDown => Color::new(80, 200, 80),
        Tile::StairsUp => Color::new(80, 140, 200),
        Tile::Door | Tile::OpenDoor => Color::new(160, 120, 60),
        Tile::Water => Color::new(40, 100, 200),
        Tile::Lava => Color::new(255, 100, 30),
        Tile::Chest | Tile::OpenChest => Color::new(200, 180, 50),
        Tile::Shrine | Tile::UsedShrine => Color::new(160, 120, 200),
        Tile::Trap | Tile::DisarmedTrap => Color::new(200, 60, 60),
        Tile::BossGate => Color::new(255, 50, 50),
        _ => Color::new(60, 60, 70),
    }
}

fn status_effect_color(effect: StatusEffect) -> Color {
    match effect {
        StatusEffect::Poison => Color::new(80, 200, 80),
        StatusEffect::Burn => Color::new(255, 140, 50),
        StatusEffect::Freeze => Color::new(100, 200, 255),
        StatusEffect::Bleed => Color::new(200, 40, 40),
        StatusEffect::Stun => Color::new(255, 255, 100),
        StatusEffect::Shield => Color::new(100, 200, 255),
        StatusEffect::Regeneration => Color::new(80, 255, 80),
        StatusEffect::Strength => Color::new(255, 100, 100),
        StatusEffect::Invisibility => Color::new(150, 150, 200),
        _ => Color::new(200, 200, 200),
    }
}
use std::collections::{HashMap, VecDeque};
use particles::{process_particle_event, ParticleEvent, ParticleSystem, ParticleType};

// ============================================================================
// THEME AND COLOR PALETTE
// ============================================================================

/// Dark fantasy color palette for consistent theming
pub struct ThemeColors;

impl ThemeColors {
    // Primary colors
    pub const BACKGROUND_DARKEST: Color32 = Color32::from_rgb(8, 8, 12);
    pub const BACKGROUND_DARK: Color32 = Color32::from_rgb(12, 12, 18);
    pub const BACKGROUND_MEDIUM: Color32 = Color32::from_rgb(18, 18, 26);
    pub const BACKGROUND_LIGHT: Color32 = Color32::from_rgb(25, 25, 35);
    pub const BACKGROUND_PANEL: Color32 = Color32::from_rgb(15, 15, 22);

    // Accent colors
    pub const ACCENT_GOLD: Color32 = Color32::from_rgb(255, 200, 80);
    pub const ACCENT_CYAN: Color32 = Color32::from_rgb(80, 200, 220);
    pub const ACCENT_PURPLE: Color32 = Color32::from_rgb(160, 100, 220);
    pub const ACCENT_RED: Color32 = Color32::from_rgb(220, 60, 60);
    pub const ACCENT_GREEN: Color32 = Color32::from_rgb(80, 200, 100);
    pub const ACCENT_BLUE: Color32 = Color32::from_rgb(80, 140, 220);

    // Text colors
    pub const TEXT_BRIGHT: Color32 = Color32::from_rgb(240, 240, 250);
    pub const TEXT_NORMAL: Color32 = Color32::from_rgb(200, 200, 220);
    pub const TEXT_DIM: Color32 = Color32::from_rgb(140, 140, 160);
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(90, 90, 110);

    // Border colors
    pub const BORDER_DARK: Color32 = Color32::from_rgb(40, 40, 55);
    pub const BORDER_LIGHT: Color32 = Color32::from_rgb(60, 60, 80);
    pub const BORDER_HIGHLIGHT: Color32 = Color32::from_rgb(100, 100, 130);

    // HP/MP bar colors
    pub const HP_FULL: Color32 = Color32::from_rgb(60, 180, 80);
    pub const HP_MID: Color32 = Color32::from_rgb(220, 180, 50);
    pub const HP_LOW: Color32 = Color32::from_rgb(200, 50, 50);
    pub const HP_CRITICAL: Color32 = Color32::from_rgb(255, 40, 40);
    pub const MP_FULL: Color32 = Color32::from_rgb(60, 120, 220);
    pub const XP_BAR: Color32 = Color32::from_rgb(180, 140, 255);

    // Rarity colors
    pub const RARITY_COMMON: Color32 = Color32::from_rgb(180, 180, 180);
    pub const RARITY_UNCOMMON: Color32 = Color32::from_rgb(80, 200, 80);
    pub const RARITY_RARE: Color32 = Color32::from_rgb(80, 140, 255);
    pub const RARITY_EPIC: Color32 = Color32::from_rgb(180, 80, 255);
    pub const RARITY_LEGENDARY: Color32 = Color32::from_rgb(255, 165, 0);
}

// ============================================================================
// UI HELPER FUNCTIONS
// ============================================================================

/// Draw a styled panel frame
fn draw_panel_frame(painter: &egui::Painter, rect: Rect, title: Option<&str>) {
    // Background with gradient effect (simulated)
    painter.rect_filled(rect, Rounding::same(4), ThemeColors::BACKGROUND_PANEL);

    // Inner shadow at top
    let shadow_rect = Rect::from_min_max(rect.min, Pos2::new(rect.max.x, rect.min.y + 3.0));
    painter.rect_filled(shadow_rect, Rounding::same(4), Color32::from_rgba_unmultiplied(0, 0, 0, 40));

    // Border with subtle highlight
    painter.rect_stroke(rect, Rounding::same(4), Stroke::new(1.0, ThemeColors::BORDER_DARK));
    let inner_rect = rect.shrink(1.0);
    painter.rect_stroke(
        Rect::from_min_max(inner_rect.min, Pos2::new(inner_rect.max.x, inner_rect.min.y + 1.0)),
        Rounding::ZERO,
        Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 10)),
    );

    // Title bar if provided
    if let Some(title) = title {
        let title_rect = Rect::from_min_size(rect.min, Vec2::new(rect.width(), 24.0));
        painter.rect_filled(title_rect, Rounding::same(4), ThemeColors::BACKGROUND_LIGHT);
        painter.text(
            Pos2::new(rect.min.x + 10.0, rect.min.y + 12.0),
            egui::Align2::LEFT_CENTER,
            title,
            FontId::proportional(13.0),
            ThemeColors::ACCENT_GOLD,
        );
    }
}

/// Draw a gradient bar (HP, MP, XP)
fn draw_gradient_bar(
    painter: &egui::Painter,
    rect: Rect,
    fill_ratio: f32,
    color_left: Color32,
    color_right: Color32,
    show_segments: bool,
) {
    // Background
    painter.rect_filled(rect, Rounding::same(3), Color32::from_rgb(20, 20, 30));

    // Fill
    if fill_ratio > 0.0 {
        let fill_width = rect.width() * fill_ratio.clamp(0.0, 1.0);
        let fill_rect = Rect::from_min_size(rect.min, Vec2::new(fill_width, rect.height()));

        // Draw gradient by splitting into segments
        let segments = 10;
        let segment_width = fill_width / segments as f32;
        for i in 0..segments {
            let t = i as f32 / segments as f32;
            let seg_color = Color32::from_rgb(
                (color_left.r() as f32 * (1.0 - t) + color_right.r() as f32 * t) as u8,
                (color_left.g() as f32 * (1.0 - t) + color_right.g() as f32 * t) as u8,
                (color_left.b() as f32 * (1.0 - t) + color_right.b() as f32 * t) as u8,
            );
            let seg_rect = Rect::from_min_size(
                Pos2::new(rect.min.x + i as f32 * segment_width, rect.min.y),
                Vec2::new(segment_width + 1.0, rect.height()),
            );
            painter.rect_filled(seg_rect.intersect(fill_rect), Rounding::ZERO, seg_color);
        }

        // Shine effect
        let shine_rect = Rect::from_min_size(fill_rect.min, Vec2::new(fill_rect.width(), 2.0));
        painter.rect_filled(shine_rect, Rounding::ZERO, Color32::from_rgba_unmultiplied(255, 255, 255, 30));
    }

    // Segment lines
    if show_segments {
        let segment_count = 10;
        for i in 1..segment_count {
            let x = rect.min.x + (rect.width() / segment_count as f32) * i as f32;
            painter.line_segment(
                [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 0, 0, 60)),
            );
        }
    }

    // Border
    painter.rect_stroke(rect, Rounding::same(3), Stroke::new(1.0, ThemeColors::BORDER_DARK));
}

/// Draw character portrait placeholder
fn draw_character_portrait(painter: &egui::Painter, rect: Rect, class: CharacterClass) {
    // Frame
    painter.rect_filled(rect, Rounding::same(4), ThemeColors::BACKGROUND_DARK);
    painter.rect_stroke(rect, Rounding::same(4), Stroke::new(2.0, ThemeColors::ACCENT_GOLD));

    // Class icon/symbol
    let symbol = match class {
        CharacterClass::Warrior => "W",
        CharacterClass::Mage => "M",
        CharacterClass::Rogue => "R",
        CharacterClass::Paladin => "P",
        CharacterClass::Ranger => "A",
        CharacterClass::Necromancer => "N",
    };

    let color = match class {
        CharacterClass::Warrior => Color32::from_rgb(200, 80, 80),
        CharacterClass::Mage => Color32::from_rgb(80, 120, 220),
        CharacterClass::Rogue => Color32::from_rgb(100, 180, 100),
        CharacterClass::Paladin => Color32::from_rgb(255, 220, 100),
        CharacterClass::Ranger => Color32::from_rgb(120, 180, 80),
        CharacterClass::Necromancer => Color32::from_rgb(150, 80, 180),
    };

    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        symbol,
        FontId::proportional(rect.width() * 0.6),
        color,
    );

    // Class name below
    painter.text(
        Pos2::new(rect.center().x, rect.max.y + 8.0),
        egui::Align2::CENTER_TOP,
        class.name(),
        FontId::proportional(11.0),
        ThemeColors::TEXT_NORMAL,
    );
}

/// Draw an equipment slot
fn draw_equipment_slot(
    painter: &egui::Painter,
    rect: Rect,
    slot: EquipSlot,
    item: Option<&shadowcrypt_core::items::Item>,
    is_hovered: bool,
) {
    // Slot background
    let bg_color = if is_hovered {
        ThemeColors::BACKGROUND_LIGHT
    } else {
        ThemeColors::BACKGROUND_DARK
    };
    painter.rect_filled(rect, Rounding::same(3), bg_color);

    // Slot border
    let border_color = if item.is_some() {
        if is_hovered { ThemeColors::ACCENT_GOLD } else { ThemeColors::BORDER_LIGHT }
    } else {
        ThemeColors::BORDER_DARK
    };
    painter.rect_stroke(rect, Rounding::same(3), Stroke::new(1.0, border_color));

    if let Some(item) = item {
        // Item rarity glow
        let rarity_col = to_egui_color(rarity_color(item.rarity));
        painter.rect_filled(
            rect.shrink(2.0),
            Rounding::same(2),
            Color32::from_rgba_unmultiplied(rarity_col.r(), rarity_col.g(), rarity_col.b(), 30),
        );

        // Item icon
        let icon = match item.kind {
            ItemKind::Weapon(_) => "W",
            ItemKind::Armor => "A",
            ItemKind::Shield => "S",
            ItemKind::Helmet => "H",
            ItemKind::Boots => "B",
            ItemKind::Gloves => "G",
            _ => "?",
        };
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            icon,
            FontId::proportional(rect.width() * 0.5),
            rarity_col,
        );
    } else {
        // Empty slot indicator
        let slot_char = match slot {
            EquipSlot::Weapon => "W",
            EquipSlot::Armor => "A",
            EquipSlot::Shield => "S",
            EquipSlot::Helmet => "H",
            EquipSlot::Boots => "B",
            EquipSlot::Gloves => "G",
            EquipSlot::Ring => "R",
            EquipSlot::Amulet => "N",
        };
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            slot_char,
            FontId::proportional(rect.width() * 0.4),
            ThemeColors::TEXT_MUTED,
        );
    }
}

// ============================================================================
// Lighting System
// ============================================================================

#[derive(Clone, Copy)]
pub struct LightingConfig {
    pub torch_radius: f32,
    pub ambient_light: f32,
    pub falloff_exponent: f32,
    pub fog_density: f32,
    pub fog_color: Color,
    pub torch_flicker: bool,
    pub torch_color: Color,
    pub memory_darkness: f32,
}

impl Default for LightingConfig {
    fn default() -> Self {
        Self {
            torch_radius: 8.0,
            ambient_light: 0.02,
            falloff_exponent: 1.8,
            fog_density: 0.15,
            fog_color: Color::new(20, 20, 40),
            torch_flicker: true,
            torch_color: Color::new(255, 220, 180),
            memory_darkness: 0.75,
        }
    }
}

impl LightingConfig {
    pub fn for_theme(theme: DungeonTheme, dungeon_level: u32) -> Self {
        let base = Self::default();
        let depth_factor = (dungeon_level as f32 / 30.0).min(1.0);

        match theme {
            DungeonTheme::DarkDungeon => Self {
                torch_radius: 8.0 - depth_factor * 1.5,
                ambient_light: 0.03 - depth_factor * 0.02,
                fog_density: 0.12 + depth_factor * 0.08,
                fog_color: Color::new(15, 15, 25),
                torch_color: Color::new(255, 200, 150),
                ..base
            },
            DungeonTheme::TwistedCaves => Self {
                torch_radius: 7.0,
                ambient_light: 0.01,
                fog_density: 0.20,
                fog_color: Color::new(20, 25, 20),
                torch_color: Color::new(255, 230, 180),
                ..base
            },
            DungeonTheme::HauntedCrypt => Self {
                torch_radius: 6.5,
                ambient_light: 0.02,
                fog_density: 0.25,
                fog_color: Color::new(30, 25, 35),
                torch_color: Color::new(200, 180, 255),
                ..base
            },
            DungeonTheme::CursedForest => Self {
                torch_radius: 9.0,
                ambient_light: 0.08,
                fog_density: 0.18,
                fog_color: Color::new(20, 35, 25),
                torch_color: Color::new(255, 240, 200),
                ..base
            },
            DungeonTheme::FrozenCaverns => Self {
                torch_radius: 10.0,
                ambient_light: 0.06,
                fog_density: 0.22,
                fog_color: Color::new(40, 50, 60),
                torch_color: Color::new(200, 220, 255),
                ..base
            },
            DungeonTheme::VolcanicDepths => Self {
                torch_radius: 7.5,
                ambient_light: 0.10,
                fog_density: 0.30,
                fog_color: Color::new(50, 25, 15),
                torch_color: Color::new(255, 150, 100),
                ..base
            },
            DungeonTheme::AncientRuins => Self {
                torch_radius: 8.0,
                ambient_light: 0.04,
                fog_density: 0.15,
                fog_color: Color::new(40, 35, 25),
                torch_color: Color::new(255, 220, 160),
                ..base
            },
            DungeonTheme::DemonRealm => Self {
                torch_radius: 6.0,
                ambient_light: 0.05,
                fog_density: 0.35,
                fog_color: Color::new(40, 15, 20),
                torch_color: Color::new(255, 100, 80),
                falloff_exponent: 2.2,
                ..base
            },
            _ => base,
        }
    }
}

#[derive(Clone, Copy)]
pub struct LightSource {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub intensity: f32,
    pub color: Color,
}

pub struct LightingSystem {
    config: LightingConfig,
    time: f32,
    flicker_offset: f32,
}

impl LightingSystem {
    pub fn new() -> Self {
        Self {
            config: LightingConfig::default(),
            time: 0.0,
            flicker_offset: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.time += delta_time;
        if self.config.torch_flicker {
            self.flicker_offset =
                (self.time * 8.0).sin() * 0.05
                    + (self.time * 13.0).sin() * 0.03
                    + (self.time * 21.0).sin() * 0.02;
        } else {
            self.flicker_offset = 0.0;
        }
    }

    pub fn set_theme(&mut self, theme: DungeonTheme, dungeon_level: u32) {
        self.config = LightingConfig::for_theme(theme, dungeon_level);
    }

    pub fn calculate_light_at(
        &self,
        x: usize,
        y: usize,
        player_x: usize,
        player_y: usize,
        map: &Map,
        additional_lights: &[LightSource],
    ) -> f32 {
        let mut light = self.config.ambient_light;
        let dx = x as f32 - player_x as f32;
        let dy = y as f32 - player_y as f32;
        let distance = (dx * dx + dy * dy).sqrt();
        let effective_radius = self.config.torch_radius * (1.0 + self.flicker_offset);

        if distance < effective_radius {
            let normalized_dist = distance / effective_radius;
            let torch_light = (1.0 - normalized_dist.powf(self.config.falloff_exponent)).max(0.0);
            light += torch_light * (1.0 + self.flicker_offset * 0.5);
        }

        for source in additional_lights {
            let sdx = x as f32 - source.x;
            let sdy = y as f32 - source.y;
            let sdist = (sdx * sdx + sdy * sdy).sqrt();
            if sdist < source.radius {
                let normalized = sdist / source.radius;
                let source_light = (1.0 - normalized.powf(1.5)) * source.intensity;
                light += source_light;
            }
        }

        if let Some(tile) = map.get_tile(x, y) {
            match tile {
                Tile::Lava => light += 0.4,
                Tile::Shrine => light += 0.3,
                Tile::BossGate => light += 0.2,
                _ => {}
            }
        }

        light.clamp(0.0, 1.0)
    }

    pub fn calculate_fog_at(&self, x: usize, y: usize, player_x: usize, player_y: usize) -> f32 {
        let dx = x as f32 - player_x as f32;
        let dy = y as f32 - player_y as f32;
        let distance = (dx * dx + dy * dy).sqrt();
        let fog_start = self.config.torch_radius * 0.5;
        let fog_end = self.config.torch_radius * 1.5;

        if distance <= fog_start {
            0.0
        } else if distance >= fog_end {
            self.config.fog_density
        } else {
            let t = (distance - fog_start) / (fog_end - fog_start);
            t * self.config.fog_density
        }
    }

    pub fn apply_lighting(&self, base_color: Color, light_level: f32, fog_amount: f32) -> Color {
        let mut r = base_color.r as f32 * light_level;
        let mut g = base_color.g as f32 * light_level;
        let mut b = base_color.b as f32 * light_level;

        let tint_strength = light_level * 0.3;
        r = r * (1.0 - tint_strength) + self.config.torch_color.r as f32 * tint_strength * (r / 255.0);
        g = g * (1.0 - tint_strength) + self.config.torch_color.g as f32 * tint_strength * (g / 255.0);
        b = b * (1.0 - tint_strength) + self.config.torch_color.b as f32 * tint_strength * (b / 255.0);

        r = r * (1.0 - fog_amount) + self.config.fog_color.r as f32 * fog_amount;
        g = g * (1.0 - fog_amount) + self.config.fog_color.g as f32 * fog_amount;
        b = b * (1.0 - fog_amount) + self.config.fog_color.b as f32 * fog_amount;

        Color::new(
            (r as u8).min(255),
            (g as u8).min(255),
            (b as u8).min(255),
        )
    }

    pub fn get_memory_color(&self, base_color: Color) -> Color {
        let darkness = self.config.memory_darkness;
        Color::new(
            ((base_color.r as f32 * (1.0 - darkness)) as u8).max(15),
            ((base_color.g as f32 * (1.0 - darkness)) as u8).max(15),
            ((base_color.b as f32 * (1.0 - darkness)) as u8).max(15),
        )
    }

    pub fn collect_light_sources(
        &self,
        map: &Map,
        enemies: &[Enemy],
        view_x: usize,
        view_y: usize,
        view_width: usize,
        view_height: usize,
    ) -> Vec<LightSource> {
        let mut sources = Vec::new();

        for y in view_y..(view_y + view_height).min(MAP_HEIGHT) {
            for x in view_x..(view_x + view_width).min(MAP_WIDTH) {
                if let Some(tile) = map.get_tile(x, y) {
                    match tile {
                        Tile::Lava => sources.push(LightSource {
                            x: x as f32 + 0.5, y: y as f32 + 0.5, radius: 3.5, intensity: 0.6,
                            color: Color::new(255, 100, 50),
                        }),
                        Tile::Shrine => sources.push(LightSource {
                            x: x as f32 + 0.5, y: y as f32 + 0.5, radius: 4.0, intensity: 0.5,
                            color: Color::new(200, 150, 255),
                        }),
                        Tile::BossGate => sources.push(LightSource {
                            x: x as f32 + 0.5, y: y as f32 + 0.5, radius: 5.0, intensity: 0.4,
                            color: Color::new(255, 50, 50),
                        }),
                        _ => {}
                    }
                }
            }
        }

        for enemy in enemies.iter().filter(|e| e.is_alive()) {
            if enemy.x >= view_x && enemy.x < view_x + view_width &&
               enemy.y >= view_y && enemy.y < view_y + view_height {
                let light = match enemy.kind {
                    EnemyKind::FireElemental | EnemyKind::FireDrake => Some((4.0, 0.5, Color::new(255, 150, 50))),
                    EnemyKind::LavaGolem | EnemyKind::MagmaSlime => Some((3.5, 0.4, Color::new(255, 100, 30))),
                    EnemyKind::Hellhound | EnemyKind::InfernalImp => Some((2.5, 0.3, Color::new(255, 120, 50))),
                    EnemyKind::CinderWraith => Some((3.0, 0.35, Color::new(255, 80, 30))),
                    EnemyKind::Salamander => Some((3.0, 0.4, Color::new(255, 130, 60))),
                    EnemyKind::InfernalLord | EnemyKind::Balrog => Some((5.0, 0.6, Color::new(255, 80, 40))),
                    EnemyKind::Ghost | EnemyKind::Wraith | EnemyKind::Banshee => Some((2.0, 0.2, Color::new(150, 200, 255))),
                    EnemyKind::IceWraith => Some((2.5, 0.25, Color::new(100, 200, 255))),
                    _ => None,
                };
                if let Some((radius, intensity, color)) = light {
                    sources.push(LightSource { x: enemy.x as f32 + 0.5, y: enemy.y as f32 + 0.5, radius, intensity, color });
                }
            }
        }

        sources
    }

    pub fn config(&self) -> &LightingConfig { &self.config }
}

impl Default for LightingSystem {
    fn default() -> Self { Self::new() }
}

// ============================================================================
// SETTINGS
// ============================================================================

#[derive(Clone, PartialEq)]
pub struct DisplaySettings {
    pub tile_size: f32,
    pub zoom_level: f32,
    pub show_grid: bool,
    pub smooth_scrolling: bool,
    pub show_fps: bool,
    pub torch_flicker: bool,
    pub ui_scale: f32,
    pub enable_vignette: bool,
    pub vignette_intensity: f32,
    pub enable_particles: bool,
}

impl Default for DisplaySettings {
    fn default() -> Self {
        Self {
            tile_size: 20.0,
            zoom_level: 1.0,
            show_grid: false,
            smooth_scrolling: true,
            show_fps: false,
            torch_flicker: true,
            ui_scale: 1.0,
            enable_vignette: true,
            vignette_intensity: 0.3,
            enable_particles: true,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct GameplaySettings {
    pub auto_pickup_gold: bool,
    pub auto_pickup_items: bool,
    pub confirm_stairs: bool,
    pub show_damage_numbers: bool,
    pub click_to_move: bool,
    pub message_log_size: usize,
    pub auto_play_speed: u32,
}

impl Default for GameplaySettings {
    fn default() -> Self {
        Self {
            auto_pickup_gold: true,
            auto_pickup_items: false,
            confirm_stairs: true,
            show_damage_numbers: true,
            click_to_move: true,
            message_log_size: 8,
            auto_play_speed: 100,
        }
    }
}

#[derive(Clone)]
pub struct Settings {
    pub display: DisplaySettings,
    pub gameplay: GameplaySettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self { display: DisplaySettings::default(), gameplay: GameplaySettings::default() }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SettingsTab { Display, Gameplay, Controls }

// ============================================================================
// MINIMAP
// ============================================================================

#[derive(Clone, Copy, PartialEq)]
pub enum MinimapCorner { TopLeft, TopRight, BottomLeft, BottomRight }

pub struct Minimap {
    visible: bool,
    zoom: f32,
    corner: MinimapCorner,
    show_enemies: bool,
    show_items: bool,
}

impl Default for Minimap {
    fn default() -> Self {
        Self { visible: true, zoom: 2.0, corner: MinimapCorner::TopRight, show_enemies: true, show_items: true }
    }
}

impl Minimap {
    pub fn toggle(&mut self) { self.visible = !self.visible; }
    pub fn zoom_in(&mut self) { self.zoom = (self.zoom + 0.5).min(5.0); }
    pub fn zoom_out(&mut self) { self.zoom = (self.zoom - 0.5).max(1.0); }

    pub fn render(&self, painter: &egui::Painter, rect: Rect, state: &GameState, _fog_color: Color) {
        if !self.visible { return; }

        let size = Vec2::new(MAP_WIDTH as f32 * self.zoom, MAP_HEIGHT as f32 * self.zoom);
        let margin = 10.0;
        let pos = match self.corner {
            MinimapCorner::TopRight => Pos2::new(rect.max.x - size.x - margin, rect.min.y + margin),
            MinimapCorner::TopLeft => Pos2::new(rect.min.x + margin, rect.min.y + margin),
            MinimapCorner::BottomRight => Pos2::new(rect.max.x - size.x - margin, rect.max.y - size.y - margin),
            MinimapCorner::BottomLeft => Pos2::new(rect.min.x + margin, rect.max.y - size.y - margin),
        };
        let minimap_rect = Rect::from_min_size(pos, size);

        // Background
        painter.rect_filled(minimap_rect, Rounding::same(4), Color32::from_rgba_unmultiplied(5, 5, 10, 220));
        painter.rect_stroke(minimap_rect, Rounding::same(4), Stroke::new(1.0, ThemeColors::BORDER_LIGHT));

        // Draw tiles
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                if !state.map.explored[y][x] { continue; }
                let tile_rect = Rect::from_min_size(
                    Pos2::new(pos.x + x as f32 * self.zoom, pos.y + y as f32 * self.zoom),
                    Vec2::new(self.zoom, self.zoom),
                );
                let tile = state.map.tiles[y][x];
                let color = match tile {
                    Tile::Floor => if state.map.visible[y][x] { Color32::from_rgb(50, 50, 60) } else { Color32::from_rgb(30, 30, 35) },
                    Tile::Wall => Color32::from_rgb(70, 65, 60),
                    Tile::StairsDown => Color32::from_rgb(80, 180, 80),
                    Tile::StairsUp => Color32::from_rgb(80, 140, 180),
                    Tile::Lava => Color32::from_rgb(180, 60, 20),
                    Tile::Water => Color32::from_rgb(40, 80, 140),
                    Tile::Shrine => Color32::from_rgb(160, 120, 200),
                    _ => Color32::from_rgb(40, 40, 50),
                };
                painter.rect_filled(tile_rect, 0.0, color);
            }
        }

        // Items
        if self.show_items {
            for item in &state.items {
                if state.map.explored[item.y][item.x] {
                    let dot_pos = Pos2::new(pos.x + item.x as f32 * self.zoom + self.zoom * 0.5,
                                            pos.y + item.y as f32 * self.zoom + self.zoom * 0.5);
                    painter.circle_filled(dot_pos, (self.zoom * 0.4).max(1.5), to_egui_color(rarity_color(item.rarity)));
                }
            }
        }

        // Enemies
        if self.show_enemies {
            for enemy in state.enemies.iter().filter(|e| e.is_alive() && state.map.visible[e.y][e.x]) {
                let dot_pos = Pos2::new(pos.x + enemy.x as f32 * self.zoom + self.zoom * 0.5,
                                        pos.y + enemy.y as f32 * self.zoom + self.zoom * 0.5);
                let color = if enemy.is_boss { Color32::from_rgb(255, 50, 200) } else { Color32::from_rgb(200, 60, 60) };
                painter.circle_filled(dot_pos, (self.zoom * 0.5).max(2.0), color);
            }
        }

        // Player
        let player_pos = Pos2::new(pos.x + state.player.x as f32 * self.zoom + self.zoom * 0.5,
                                   pos.y + state.player.y as f32 * self.zoom + self.zoom * 0.5);
        painter.circle_filled(player_pos, (self.zoom * 0.6).max(3.0), Color32::from_rgb(255, 255, 100));

        // Label
        painter.text(
            Pos2::new(minimap_rect.center().x, minimap_rect.max.y + 5.0),
            egui::Align2::CENTER_TOP,
            format!("[M] Minimap  {:.0}x", self.zoom),
            FontId::proportional(9.0),
            ThemeColors::TEXT_MUTED,
        );
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

fn to_egui_color(color: Color) -> Color32 {
    Color32::from_rgb(color.r, color.g, color.b)
}

fn to_egui_color_alpha(color: Color, alpha: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(color.r, color.g, color.b, alpha)
}

fn get_skill_icon(skill: Skill) -> &'static str {
    match skill {
        Skill::Berserk => "B", Skill::Cleave => "C", Skill::ShieldBash => "S", Skill::Whirlwind => "W",
        Skill::Fireball => "F", Skill::IceSpear => "I", Skill::Lightning => "L", Skill::Teleport => "T",
        Skill::Backstab => "B", Skill::ShadowStep => "S", Skill::PoisonBlade => "P", Skill::Vanish => "V",
        Skill::HolyLight => "H", Skill::DivineShield => "D", Skill::Smite => "S", Skill::Consecrate => "C",
        Skill::MultiShot => "M", Skill::PoisonArrow => "P", Skill::TrapSet => "T", Skill::EagleEye => "E",
        Skill::RaiseDead => "R", Skill::LifeDrain => "L", Skill::Curse => "C", Skill::DarkPact => "D",
    }
}

fn get_skill_color(skill: Skill) -> Color32 {
    match skill {
        Skill::Berserk | Skill::Cleave | Skill::Whirlwind => Color32::from_rgb(255, 100, 100),
        Skill::ShieldBash => Color32::from_rgb(180, 180, 200),
        Skill::Fireball => Color32::from_rgb(255, 140, 50),
        Skill::IceSpear => Color32::from_rgb(100, 200, 255),
        Skill::Lightning => Color32::from_rgb(255, 255, 100),
        Skill::Teleport | Skill::ShadowStep => Color32::from_rgb(180, 100, 255),
        Skill::Backstab => Color32::from_rgb(150, 100, 100),
        Skill::PoisonBlade | Skill::PoisonArrow => Color32::from_rgb(100, 200, 100),
        Skill::Vanish => Color32::from_rgb(100, 100, 140),
        Skill::HolyLight | Skill::DivineShield | Skill::Smite | Skill::Consecrate => Color32::from_rgb(255, 255, 180),
        Skill::MultiShot | Skill::TrapSet | Skill::EagleEye => Color32::from_rgb(150, 200, 100),
        Skill::RaiseDead | Skill::LifeDrain | Skill::Curse | Skill::DarkPact => Color32::from_rgb(150, 80, 180),
    }
}

fn get_skill_description(skill: Skill) -> &'static str {
    match skill {
        Skill::Berserk => "Empower attacks for 10 turns",
        Skill::Cleave => "Hit all adjacent enemies",
        Skill::ShieldBash => "Stun and damage nearby enemy",
        Skill::Whirlwind => "Devastating spin attack",
        Skill::Fireball => "AOE fire damage + burn",
        Skill::IceSpear => "AOE ice damage + freeze",
        Skill::Lightning => "Chain lightning to 3 enemies",
        Skill::Teleport => "Random teleport",
        Skill::Backstab => "Triple damage attack",
        Skill::ShadowStep => "Teleport behind enemy",
        Skill::PoisonBlade => "Poison melee attack",
        Skill::Vanish => "Invisibility for 10 turns",
        Skill::HolyLight => "Heal self, damage undead",
        Skill::DivineShield => "Block next attack",
        Skill::Smite => "Holy damage (2x vs undead)",
        Skill::Consecrate => "Create healing shrine",
        Skill::MultiShot => "Hit up to 3 enemies",
        Skill::PoisonArrow => "Ranged poison attack",
        Skill::TrapSet => "Place trap at position",
        Skill::EagleEye => "Reveal entire floor",
        Skill::RaiseDead => "Summon skeleton minion",
        Skill::LifeDrain => "Steal HP from enemy",
        Skill::Curse => "Weaken all visible enemies",
        Skill::DarkPact => "Sacrifice HP for power",
    }
}

fn get_max_cooldown(skill: Skill) -> u32 {
    match skill {
        Skill::Whirlwind | Skill::Consecrate | Skill::DarkPact => 8,
        Skill::Teleport | Skill::Vanish | Skill::RaiseDead => 6,
        Skill::Fireball | Skill::Lightning | Skill::IceSpear => 4,
        Skill::HolyLight | Skill::DivineShield | Skill::LifeDrain => 5,
        Skill::MultiShot | Skill::EagleEye => 3,
        _ => 2,
    }
}

// ============================================================================
// WINDOWS AND POPUPS
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OpenWindow {
    None,
    Inventory,
    Character,
    Skills,
    Settings,
    Help,
}

// ============================================================================
// APPLICATION STATE
// ============================================================================

enum AppState {
    Loading { progress: f32, message: String },
    ClassSelect,
    Playing,
    GameOver,
    Victory,
}

struct ShadowCryptApp {
    state: Option<GameState>,
    app_state: AppState,
    auto_play: bool,
    auto_ai: AutoPlayAI,
    last_update: std::time::Instant,
    lighting: LightingSystem,
    settings: Settings,
    minimap: Minimap,
    skill_cooldowns: HashMap<Skill, u32>,
    last_turn_count: u32,
    hover_skill: Option<usize>,
    hover_item: Option<usize>,
    hover_enemy: Option<(usize, usize)>,
    open_window: OpenWindow,
    settings_tab: SettingsTab,
    click_target: Option<(usize, usize)>,
    particles: ParticleSystem,
    previous_player_hp: i32,
    previous_player_level: u32,
    context_menu: Option<(Pos2, usize)>, // Position and item index
    tooltip_text: Option<(Pos2, String)>,
    frame_times: VecDeque<f32>,
    loading_start: std::time::Instant,
}

impl Default for ShadowCryptApp {
    fn default() -> Self {
        Self {
            state: None,
            app_state: AppState::Loading { progress: 0.0, message: "Initializing...".to_string() },
            auto_play: false,
            auto_ai: AutoPlayAI::new(),
            last_update: std::time::Instant::now(),
            lighting: LightingSystem::new(),
            settings: Settings::default(),
            minimap: Minimap::default(),
            skill_cooldowns: HashMap::new(),
            last_turn_count: 0,
            hover_skill: None,
            hover_item: None,
            hover_enemy: None,
            open_window: OpenWindow::None,
            settings_tab: SettingsTab::Display,
            click_target: None,
            particles: ParticleSystem::new(2000),
            previous_player_hp: 0,
            previous_player_level: 1,
            context_menu: None,
            tooltip_text: None,
            frame_times: VecDeque::with_capacity(60),
            loading_start: std::time::Instant::now(),
        }
    }
}

impl ShadowCryptApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn start_game(&mut self, class: CharacterClass) {
        let state = GameState::new(class);
        self.lighting.set_theme(state.map.theme, state.dungeon_level);
        self.previous_player_hp = state.player.hp;
        self.previous_player_level = state.player.level;
        self.state = Some(state);
        self.app_state = AppState::Playing;
        self.skill_cooldowns.clear();
        self.last_turn_count = 0;
        self.particles.clear();
    }

    fn activate_skill_by_index(&mut self, skill_idx: usize) {
        if let Some(state) = &mut self.state {
            if skill_idx < state.player.skills.len() {
                let skill = state.player.skills[skill_idx];
                let cooldown = *self.skill_cooldowns.get(&skill).unwrap_or(&0);
                if cooldown == 0 && state.player.mana >= skill.mana_cost() {
                    state.player.active_skill = skill_idx;
                    state.use_skill();
                    let max_cd = get_max_cooldown(skill);
                    self.skill_cooldowns.insert(skill, max_cd);
                }
            }
        }
    }

    fn render_loading(&mut self, ctx: &egui::Context) {
        let elapsed = self.loading_start.elapsed().as_secs_f32();
        let progress = (elapsed / 1.5).min(1.0);
        let message = if progress < 0.3 { "Loading assets..." }
        else if progress < 0.6 { "Preparing dungeon..." }
        else if progress < 0.9 { "Summoning monsters..." }
        else { "Ready!" };

        if progress >= 1.0 {
            self.app_state = AppState::ClassSelect;
            return;
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(ThemeColors::BACKGROUND_DARKEST))
            .show(ctx, |ui| {
                let size = ui.available_size();
                let center = Pos2::new(size.x / 2.0, size.y / 2.0);
                let painter = ui.painter();

                // Title
                painter.text(
                    Pos2::new(center.x, center.y - 80.0),
                    egui::Align2::CENTER_CENTER,
                    "SHADOWCRYPT",
                    FontId::proportional(56.0),
                    ThemeColors::ACCENT_CYAN,
                );

                // Loading bar background
                let bar_width = 400.0;
                let bar_height = 12.0;
                let bar_rect = Rect::from_center_size(center, Vec2::new(bar_width, bar_height));
                painter.rect_filled(bar_rect, Rounding::same(6), ThemeColors::BACKGROUND_DARK);

                // Loading bar fill
                let fill_rect = Rect::from_min_size(
                    bar_rect.min,
                    Vec2::new(bar_width * progress, bar_height),
                );
                painter.rect_filled(fill_rect, Rounding::same(6), ThemeColors::ACCENT_PURPLE);

                // Loading text
                painter.text(
                    Pos2::new(center.x, center.y + 30.0),
                    egui::Align2::CENTER_CENTER,
                    message,
                    FontId::proportional(16.0),
                    ThemeColors::TEXT_DIM,
                );

                // Pulsing dots animation
                let dot_count = 3;
                let dot_spacing = 20.0;
                let start_x = center.x - (dot_count as f32 - 1.0) * dot_spacing / 2.0;
                for i in 0..dot_count {
                    let phase = elapsed * 3.0 + i as f32 * 0.5;
                    let alpha = ((phase.sin() + 1.0) * 0.5 * 255.0) as u8;
                    painter.circle_filled(
                        Pos2::new(start_x + i as f32 * dot_spacing, center.y + 60.0),
                        4.0,
                        Color32::from_rgba_unmultiplied(160, 100, 220, alpha),
                    );
                }
            });

        ctx.request_repaint();
    }

    fn render_class_select(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(ThemeColors::BACKGROUND_DARKEST))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(40.0);

                    // Title with shadow effect
                    ui.heading(RichText::new("SHADOWCRYPT")
                        .size(64.0)
                        .color(ThemeColors::ACCENT_CYAN));
                    ui.label(RichText::new("A Dark Fantasy Roguelike")
                        .size(18.0)
                        .color(ThemeColors::TEXT_DIM));

                    ui.add_space(40.0);

                    // Class selection frame
                    egui::Frame::none()
                        .fill(ThemeColors::BACKGROUND_PANEL)
                        .rounding(Rounding::same(8))
                        .stroke(Stroke::new(1.0, ThemeColors::BORDER_DARK))
                        .inner_margin(20.0)
                        .show(ui, |ui| {
                            ui.label(RichText::new("Choose Your Path")
                                .size(20.0)
                                .color(ThemeColors::ACCENT_GOLD));
                            ui.add_space(15.0);

                            for class in CharacterClass::all() {
                                let (hp, atk, def, mana, _spd) = class.base_stats();

                                let class_color = match class {
                                    CharacterClass::Warrior => Color32::from_rgb(200, 80, 80),
                                    CharacterClass::Mage => Color32::from_rgb(80, 120, 220),
                                    CharacterClass::Rogue => Color32::from_rgb(100, 180, 100),
                                    CharacterClass::Paladin => Color32::from_rgb(255, 220, 100),
                                    CharacterClass::Ranger => Color32::from_rgb(120, 180, 80),
                                    CharacterClass::Necromancer => Color32::from_rgb(150, 80, 180),
                                };

                                ui.horizontal(|ui| {
                                    // Class button
                                    let button = egui::Button::new(
                                        RichText::new(format!("{:<12}", class.name()))
                                            .size(18.0)
                                            .color(class_color)
                                    )
                                    .min_size(Vec2::new(140.0, 36.0))
                                    .fill(ThemeColors::BACKGROUND_DARK)
                                    .stroke(Stroke::new(1.0, class_color.gamma_multiply(0.5)));

                                    if ui.add(button).clicked() {
                                        self.start_game(*class);
                                    }

                                    ui.add_space(10.0);

                                    // Stats display
                                    ui.label(RichText::new(format!("HP:{}", hp))
                                        .size(13.0).color(ThemeColors::HP_FULL).monospace());
                                    ui.label(RichText::new(format!("ATK:{}", atk))
                                        .size(13.0).color(ThemeColors::ACCENT_RED).monospace());
                                    ui.label(RichText::new(format!("DEF:{}", def))
                                        .size(13.0).color(ThemeColors::TEXT_NORMAL).monospace());
                                    ui.label(RichText::new(format!("MP:{}", mana))
                                        .size(13.0).color(ThemeColors::MP_FULL).monospace());
                                });

                                ui.label(RichText::new(format!("  {}", class.special_ability()))
                                    .size(11.0)
                                    .color(ThemeColors::TEXT_MUTED));
                                ui.add_space(8.0);
                            }
                        });

                    ui.add_space(20.0);

                    // Auto-play toggle
                    ui.checkbox(
                        &mut self.auto_play,
                        RichText::new("Auto-play mode (watch AI play)")
                            .color(ThemeColors::TEXT_DIM),
                    );

                    ui.add_space(30.0);

                    // Controls hint
                    egui::Frame::none()
                        .fill(ThemeColors::BACKGROUND_DARK)
                        .rounding(Rounding::same(4))
                        .inner_margin(12.0)
                        .show(ui, |ui| {
                            ui.label(RichText::new("Controls")
                                .size(14.0)
                                .color(ThemeColors::ACCENT_GOLD));
                            ui.label(RichText::new("WASD / Arrows - Move  |  Space - Use Skill  |  Tab - Cycle Skills")
                                .size(11.0).color(ThemeColors::TEXT_MUTED));
                            ui.label(RichText::new(". - Descend  |  1-9 - Use Items  |  M - Minimap  |  I - Inventory")
                                .size(11.0).color(ThemeColors::TEXT_MUTED));
                        });
                });
            });
    }

    fn render_game(&mut self, ctx: &egui::Context) {
        let delta_time = self.last_update.elapsed().as_secs_f32();
        self.last_update = std::time::Instant::now();

        // Track frame time for FPS
        self.frame_times.push_back(delta_time);
        if self.frame_times.len() > 60 {
            self.frame_times.pop_front();
        }

        self.lighting.update(delta_time);

        let state = match &self.state {
            Some(s) => s,
            None => return,
        };

        // Particle updates
        if self.settings.display.enable_particles {
            if let Some(state) = &self.state {
                let screen_rect = ctx.screen_rect();
                self.particles.update(delta_time, state.map.theme, screen_rect);

                // Detect player damage for particles
                if state.player.hp < self.previous_player_hp {
                    let damage = self.previous_player_hp - state.player.hp;
                    let player_screen_pos = Pos2::new(screen_rect.center().x, screen_rect.center().y);
                    process_particle_event(&mut self.particles, ParticleEvent::PlayerHit {
                        position: player_screen_pos,
                        damage,
                    });
                }
                self.previous_player_hp = state.player.hp;

                // Level up particles
                if state.player.level > self.previous_player_level {
                    let player_screen_pos = Pos2::new(screen_rect.center().x, screen_rect.center().y);
                    process_particle_event(&mut self.particles, ParticleEvent::LevelUp {
                        position: player_screen_pos,
                    });
                }
                self.previous_player_level = state.player.level;
            }
        }

        // Handle auto-play
        if self.auto_play && !state.game_over && !state.victory {
            let elapsed = self.last_update.elapsed();
            if elapsed.as_millis() > self.settings.gameplay.auto_play_speed as u128 {
                let state = self.state.as_ref().unwrap();
                let action = self.auto_ai.decide(
                    state.player.x, state.player.y, state.player.hp, state.player.total_max_hp(),
                    state.player.mana, state.player.can_use_skill(), &state.map, &state.enemies,
                    state.player.find_health_potion(), state.player.find_mana_potion(),
                    state.dungeon_level, state.boss_defeated,
                );

                let state = self.state.as_mut().unwrap();
                match action {
                    AIAction::Move(dx, dy) => state.move_player(dx, dy),
                    AIAction::UseSkill => state.use_skill(),
                    AIAction::UseItem(idx) => state.use_item(idx),
                    AIAction::Descend => state.descend(),
                    AIAction::Ascend => state.ascend(),
                    AIAction::Wait => state.end_turn(),
                    AIAction::Attack(_, _) => {}
                }
                self.lighting.set_theme(state.map.theme, state.dungeon_level);
            }
        }

        ctx.request_repaint();

        let state = self.state.as_ref().unwrap();

        // Tick cooldowns
        if state.turn_count > self.last_turn_count {
            let turns_passed = state.turn_count - self.last_turn_count;
            for (_skill, cd) in self.skill_cooldowns.iter_mut() {
                *cd = cd.saturating_sub(turns_passed);
            }
            self.skill_cooldowns.retain(|_, cd| *cd > 0);
            self.last_turn_count = state.turn_count;
        }

        // Check game state
        if state.game_over {
            self.app_state = AppState::GameOver;
        } else if state.victory {
            self.app_state = AppState::Victory;
        }

        // ========== TOP PANEL ==========
        egui::TopBottomPanel::top("top_bar")
            .frame(egui::Frame::default()
                .fill(ThemeColors::BACKGROUND_DARK)
                .stroke(Stroke::new(1.0, ThemeColors::BORDER_DARK)))
            .exact_height(32.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.add_space(10.0);

                    // Floor info with theme color
                    let theme_color = match state.map.theme {
                        DungeonTheme::DarkDungeon => ThemeColors::TEXT_NORMAL,
                        DungeonTheme::TwistedCaves => Color32::from_rgb(140, 160, 140),
                        DungeonTheme::HauntedCrypt => Color32::from_rgb(180, 150, 200),
                        DungeonTheme::CursedForest => Color32::from_rgb(100, 180, 100),
                        DungeonTheme::FrozenCaverns => Color32::from_rgb(150, 200, 230),
                        DungeonTheme::VolcanicDepths => Color32::from_rgb(230, 140, 100),
                        DungeonTheme::AncientRuins => Color32::from_rgb(200, 180, 140),
                        DungeonTheme::DemonRealm => Color32::from_rgb(230, 100, 100),
                        _ => ThemeColors::TEXT_NORMAL,
                    };

                    ui.label(RichText::new(format!("Floor {} - {}", state.dungeon_level, state.map.theme.name()))
                        .size(14.0).color(theme_color));

                    ui.separator();

                    ui.label(RichText::new(format!("Turn {}", state.turn_count))
                        .size(12.0).color(ThemeColors::TEXT_DIM));

                    ui.separator();

                    ui.label(RichText::new(format!("Gold: {}", state.player.gold))
                        .size(13.0).color(ThemeColors::ACCENT_GOLD));

                    // Right-aligned controls
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(10.0);

                        if self.settings.display.show_fps {
                            let avg_frame_time: f32 = self.frame_times.iter().sum::<f32>() / self.frame_times.len().max(1) as f32;
                            let fps = if avg_frame_time > 0.0 { 1.0 / avg_frame_time } else { 0.0 };
                            ui.label(RichText::new(format!("FPS: {:.0}", fps))
                                .size(11.0).color(ThemeColors::TEXT_MUTED));
                            ui.separator();
                        }

                        if ui.small_button(RichText::new("Settings").size(11.0)).clicked() {
                            self.open_window = if self.open_window == OpenWindow::Settings {
                                OpenWindow::None
                            } else {
                                OpenWindow::Settings
                            };
                        }

                        if ui.small_button(RichText::new("?").size(11.0)).clicked() {
                            self.open_window = if self.open_window == OpenWindow::Help {
                                OpenWindow::None
                            } else {
                                OpenWindow::Help
                            };
                        }
                    });
                });
            });

        // ========== RIGHT PANEL - CHARACTER STATS ==========
        egui::SidePanel::right("stats_panel")
            .frame(egui::Frame::default()
                .fill(ThemeColors::BACKGROUND_PANEL)
                .stroke(Stroke::new(1.0, ThemeColors::BORDER_DARK)))
            .min_width(240.0)
            .max_width(280.0)
            .show(ctx, |ui| {
                ui.add_space(8.0);

                // Character Portrait
                let portrait_size = 70.0;
                let (portrait_response, portrait_painter) = ui.allocate_painter(
                    Vec2::new(ui.available_width(), portrait_size + 20.0),
                    Sense::hover(),
                );
                let portrait_rect = Rect::from_center_size(
                    Pos2::new(portrait_response.rect.center().x, portrait_response.rect.min.y + portrait_size / 2.0 + 5.0),
                    Vec2::new(portrait_size, portrait_size),
                );
                draw_character_portrait(&portrait_painter, portrait_rect, state.player.class);

                ui.add_space(10.0);

                // Level and Name
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new(format!("Level {}", state.player.level))
                        .size(16.0).color(ThemeColors::ACCENT_GOLD));
                });

                ui.add_space(10.0);

                // HP Bar
                ui.horizontal(|ui| {
                    ui.label(RichText::new("HP").size(12.0).color(ThemeColors::TEXT_DIM));
                    ui.add_space(5.0);
                    ui.label(RichText::new(format!("{}/{}", state.player.hp, state.player.total_max_hp()))
                        .size(12.0).color(ThemeColors::HP_FULL));
                });

                let hp_ratio = state.player.hp as f32 / state.player.total_max_hp() as f32;
                let hp_color = if hp_ratio > 0.6 { ThemeColors::HP_FULL }
                    else if hp_ratio > 0.3 { ThemeColors::HP_MID }
                    else { ThemeColors::HP_LOW };
                let hp_bar_rect = ui.allocate_space(Vec2::new(ui.available_width() - 10.0, 14.0)).1;
                draw_gradient_bar(&ui.painter(), hp_bar_rect, hp_ratio, hp_color, hp_color.gamma_multiply(0.7), true);

                ui.add_space(8.0);

                // MP Bar
                ui.horizontal(|ui| {
                    ui.label(RichText::new("MP").size(12.0).color(ThemeColors::TEXT_DIM));
                    ui.add_space(5.0);
                    ui.label(RichText::new(format!("{}/{}", state.player.mana, state.player.total_max_mana()))
                        .size(12.0).color(ThemeColors::MP_FULL));
                });

                let mp_ratio = state.player.mana as f32 / state.player.total_max_mana().max(1) as f32;
                let mp_bar_rect = ui.allocate_space(Vec2::new(ui.available_width() - 10.0, 14.0)).1;
                draw_gradient_bar(&ui.painter(), mp_bar_rect, mp_ratio, ThemeColors::MP_FULL, Color32::from_rgb(40, 80, 180), true);

                ui.add_space(8.0);

                // XP Bar
                ui.horizontal(|ui| {
                    ui.label(RichText::new("XP").size(12.0).color(ThemeColors::TEXT_DIM));
                    ui.add_space(5.0);
                    ui.label(RichText::new(format!("{}/{}", state.player.xp, state.player.xp_to_level))
                        .size(12.0).color(ThemeColors::XP_BAR));
                });

                let xp_ratio = state.player.xp as f32 / state.player.xp_to_level.max(1) as f32;
                let xp_bar_rect = ui.allocate_space(Vec2::new(ui.available_width() - 10.0, 10.0)).1;
                draw_gradient_bar(&ui.painter(), xp_bar_rect, xp_ratio, ThemeColors::XP_BAR, Color32::from_rgb(140, 100, 200), false);

                ui.add_space(15.0);
                ui.separator();
                ui.add_space(10.0);

                // Stats
                ui.label(RichText::new("Combat Stats").size(13.0).color(ThemeColors::ACCENT_GOLD));
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.label(RichText::new(format!("ATK: {}", state.player.total_attack())).size(12.0).color(ThemeColors::ACCENT_RED));
                    ui.add_space(20.0);
                    ui.label(RichText::new(format!("DEF: {}", state.player.total_defense())).size(12.0).color(ThemeColors::ACCENT_BLUE));
                });
                ui.label(RichText::new(format!("Kills: {}", state.player.kills)).size(11.0).color(ThemeColors::TEXT_DIM));

                ui.add_space(15.0);
                ui.separator();
                ui.add_space(10.0);

                // Equipment Grid
                ui.label(RichText::new("Equipment").size(13.0).color(ThemeColors::ACCENT_GOLD));
                ui.add_space(8.0);

                let slot_size = 36.0;
                let slots_per_row = 3;
                let spacing = 6.0;

                for (i, slot) in [EquipSlot::Weapon, EquipSlot::Armor, EquipSlot::Shield,
                                  EquipSlot::Helmet, EquipSlot::Boots, EquipSlot::Gloves].iter().enumerate() {
                    if i % slots_per_row == 0 {
                        ui.horizontal(|ui| {
                            ui.add_space((ui.available_width() - (slot_size * slots_per_row as f32 + spacing * (slots_per_row - 1) as f32)) / 2.0);
                            for j in 0..slots_per_row {
                                let idx = i + j;
                                if idx >= 6 { break; }
                                let slot = [EquipSlot::Weapon, EquipSlot::Armor, EquipSlot::Shield,
                                            EquipSlot::Helmet, EquipSlot::Boots, EquipSlot::Gloves][idx];
                                let item = state.player.equipment.get(&slot);
                                let (response, painter) = ui.allocate_painter(Vec2::new(slot_size, slot_size), Sense::hover());
                                draw_equipment_slot(&painter, response.rect, slot, item, response.hovered());

                                if response.hovered() {
                                    if let Some(item) = item {
                                        egui::show_tooltip(ui.ctx(), ui.layer_id(), egui::Id::new("equip_tooltip"), |ui| {
                                            ui.label(RichText::new(&item.name).color(to_egui_color(rarity_color(item.rarity))));
                                            ui.label(RichText::new(format!("{}", item.kind)).size(11.0).color(ThemeColors::TEXT_DIM));
                                        });
                                    }
                                }

                                if j < slots_per_row - 1 { ui.add_space(spacing); }
                            }
                        });
                        if i + slots_per_row < 6 { ui.add_space(spacing); }
                    }
                }

                ui.add_space(15.0);
                ui.separator();
                ui.add_space(10.0);

                // Status Effects
                if !state.player.status_effects.is_empty() {
                    ui.label(RichText::new("Status Effects").size(13.0).color(ThemeColors::ACCENT_GOLD));
                    ui.add_space(5.0);

                    for (effect, duration) in &state.player.status_effects {
                        let effect_color = status_effect_color(*effect);
                        ui.label(RichText::new(format!("{} ({})", effect.name(), duration))
                            .size(11.0).color(to_egui_color(effect_color)));
                    }
                }
            });

        // ========== BOTTOM PANEL - ACTION BAR ==========
        egui::TopBottomPanel::bottom("action_bar")
            .frame(egui::Frame::default()
                .fill(ThemeColors::BACKGROUND_DARK)
                .stroke(Stroke::new(1.0, ThemeColors::BORDER_DARK)))
            .exact_height(120.0)
            .show(ctx, |ui| {
                ui.add_space(8.0);

                // Skills section
                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    ui.label(RichText::new("Skills").size(12.0).color(ThemeColors::ACCENT_GOLD));
                    ui.add_space(15.0);

                    let skill_size = 50.0;

                    for (idx, skill) in state.player.skills.iter().enumerate() {
                        let skill = *skill;
                        let is_active = idx == state.player.active_skill;
                        let cooldown = *self.skill_cooldowns.get(&skill).unwrap_or(&0);
                        let has_mana = state.player.mana >= skill.mana_cost();
                        let can_use = cooldown == 0 && has_mana;

                        let (response, painter) = ui.allocate_painter(
                            Vec2::new(skill_size, skill_size + 16.0),
                            Sense::click(),
                        );
                        let icon_rect = Rect::from_min_size(response.rect.min, Vec2::new(skill_size, skill_size));

                        // Background
                        let bg_color = if can_use {
                            get_skill_color(skill).gamma_multiply(0.3)
                        } else {
                            ThemeColors::BACKGROUND_DARK
                        };
                        painter.rect_filled(icon_rect, Rounding::same(6), bg_color);

                        // Border
                        let border_color = if is_active {
                            ThemeColors::ACCENT_GOLD
                        } else if response.hovered() && can_use {
                            ThemeColors::BORDER_HIGHLIGHT
                        } else {
                            ThemeColors::BORDER_DARK
                        };
                        painter.rect_stroke(icon_rect, Rounding::same(6),
                            Stroke::new(if is_active { 2.0 } else { 1.0 }, border_color));

                        // Skill letter
                        painter.text(
                            icon_rect.center() - Vec2::new(0.0, 3.0),
                            egui::Align2::CENTER_CENTER,
                            get_skill_icon(skill),
                            FontId::proportional(22.0),
                            if can_use { get_skill_color(skill) } else { ThemeColors::TEXT_MUTED },
                        );

                        // Hotkey number
                        painter.text(
                            icon_rect.left_top() + Vec2::new(5.0, 4.0),
                            egui::Align2::LEFT_TOP,
                            format!("{}", idx + 1),
                            FontId::proportional(10.0),
                            ThemeColors::TEXT_DIM,
                        );

                        // Cooldown overlay
                        if cooldown > 0 {
                            let max_cd = get_max_cooldown(skill) as f32;
                            let cd_ratio = cooldown as f32 / max_cd;
                            let overlay_rect = Rect::from_min_max(
                                Pos2::new(icon_rect.min.x, icon_rect.max.y - icon_rect.height() * cd_ratio),
                                icon_rect.max,
                            );
                            painter.rect_filled(overlay_rect, Rounding::same(6),
                                Color32::from_rgba_unmultiplied(0, 0, 0, 180));
                            painter.text(icon_rect.center(), egui::Align2::CENTER_CENTER,
                                format!("{}", cooldown), FontId::proportional(18.0), Color32::WHITE);
                        }

                        // Mana cost
                        painter.text(
                            Pos2::new(icon_rect.center().x, icon_rect.max.y + 8.0),
                            egui::Align2::CENTER_CENTER,
                            format!("{} MP", skill.mana_cost()),
                            FontId::proportional(9.0),
                            if has_mana { ThemeColors::MP_FULL } else { ThemeColors::ACCENT_RED },
                        );

                        // Handle click
                        if response.clicked() && can_use {
                            let skill = state.player.skills[idx];
                            if let Some(game_state) = &mut self.state {
                                game_state.player.active_skill = idx;
                                game_state.use_skill();
                                self.skill_cooldowns.insert(skill, get_max_cooldown(skill));
                            }
                        }

                        // Tooltip
                        if response.hovered() {
                            egui::show_tooltip(ui.ctx(), ui.layer_id(), egui::Id::new(format!("skill_{}", idx)), |ui| {
                                ui.label(RichText::new(skill.name()).color(get_skill_color(skill)).strong());
                                ui.label(RichText::new(get_skill_description(skill)).size(11.0).color(ThemeColors::TEXT_DIM));
                                ui.label(RichText::new(format!("Cost: {} MP", skill.mana_cost())).size(10.0).color(ThemeColors::MP_FULL));
                            });
                        }

                        ui.add_space(8.0);
                    }

                    ui.add_space(30.0);
                    ui.separator();
                    ui.add_space(15.0);

                    // Quick inventory slots
                    ui.label(RichText::new("Items").size(12.0).color(ThemeColors::ACCENT_GOLD));
                    ui.add_space(10.0);

                    for i in 0..5.min(state.player.inventory.len()) {
                        let item = &state.player.inventory[i];
                        let item_color = to_egui_color(rarity_color(item.rarity));

                        let button = egui::Button::new(
                            RichText::new(format!("{}", i + 1)).size(14.0).color(item_color)
                        )
                        .min_size(Vec2::new(32.0, 32.0))
                        .fill(ThemeColors::BACKGROUND_DARK)
                        .stroke(Stroke::new(1.0, item_color.gamma_multiply(0.5)));

                        let response = ui.add(button);

                        if response.clicked() {
                            if let Some(state) = &mut self.state {
                                state.use_item(i);
                            }
                        }

                        if response.hovered() {
                            egui::show_tooltip(ui.ctx(), ui.layer_id(), egui::Id::new(format!("item_{}", i)), |ui| {
                                ui.label(RichText::new(item.display_name()).color(item_color));
                            });
                        }
                    }
                });

                ui.add_space(5.0);
                ui.separator();

                // Message log
                egui::ScrollArea::horizontal()
                    .max_height(35.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            for (msg, color) in state.messages.iter().rev().take(3) {
                                ui.label(RichText::new(msg).size(11.0).color(to_egui_color(*color)));
                                ui.add_space(15.0);
                            }
                        });
                    });
            });

        // ========== CENTRAL PANEL - GAME MAP ==========
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(ThemeColors::BACKGROUND_DARKEST))
            .show(ctx, |ui| {
                let available_size = ui.available_size();
                let tile_size = self.settings.display.tile_size * self.settings.display.zoom_level;
                let view_width = (available_size.x / tile_size) as usize;
                let view_height = (available_size.y / tile_size) as usize;

                let half_width = view_width / 2;
                let half_height = view_height / 2;
                let view_x = state.player.x.saturating_sub(half_width);
                let view_y = state.player.y.saturating_sub(half_height);

                let (response, painter) = ui.allocate_painter(available_size, Sense::click());
                let rect = response.rect;

                // Collect light sources
                let light_sources = self.lighting.collect_light_sources(
                    &state.map, &state.enemies, view_x, view_y, view_width, view_height
                );

                // Handle click-to-move
                if self.settings.gameplay.click_to_move && response.clicked() {
                    if let Some(pos) = response.interact_pointer_pos() {
                        let click_x = ((pos.x - rect.min.x) / tile_size) as usize + view_x;
                        let click_y = ((pos.y - rect.min.y) / tile_size) as usize + view_y;
                        self.click_target = Some((click_x, click_y));
                    }
                }

                // Draw map with enhanced rendering
                for y in 0..view_height.min(MAP_HEIGHT) {
                    let map_y = view_y + y;
                    if map_y >= MAP_HEIGHT { break; }

                    for x in 0..view_width.min(MAP_WIDTH) {
                        let map_x = view_x + x;
                        if map_x >= MAP_WIDTH { break; }

                        let screen_x = rect.min.x + x as f32 * tile_size;
                        let screen_y = rect.min.y + y as f32 * tile_size;
                        let tile_rect = Rect::from_min_size(
                            Pos2::new(screen_x, screen_y),
                            Vec2::new(tile_size, tile_size),
                        );

                        let is_player_pos = map_x == state.player.x && map_y == state.player.y;
                        let is_visible = state.map.visible[map_y][map_x];
                        let is_explored = state.map.explored[map_y][map_x];

                        if is_player_pos {
                            // Player with glow
                            painter.circle_filled(
                                tile_rect.center(),
                                tile_size * 0.6,
                                Color32::from_rgba_unmultiplied(255, 255, 100, 40),
                            );
                            painter.text(
                                tile_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                '@',
                                FontId::monospace(tile_size * 0.8),
                                ThemeColors::ACCENT_GOLD,
                            );
                        } else if is_visible {
                            let light_level = self.lighting.calculate_light_at(
                                map_x, map_y, state.player.x, state.player.y,
                                &state.map, &light_sources
                            );
                            let fog_amount = self.lighting.calculate_fog_at(
                                map_x, map_y, state.player.x, state.player.y
                            );

                            // Determine what to draw
                            let (glyph, base_color, is_enemy, is_item) = if let Some(enemy) = state.enemies.iter()
                                .find(|e| e.is_alive() && e.x == map_x && e.y == map_y)
                            {
                                (enemy.kind.glyph(), enemy_color(enemy.kind), true, false)
                            } else if let Some(item) = state.items.iter()
                                .find(|i| i.x == map_x && i.y == map_y)
                            {
                                (item.kind.glyph(), rarity_color(item.rarity), false, true)
                            } else {
                                let tile = state.map.tiles[map_y][map_x];
                                (tile.glyph(), tile_color(tile), false, false)
                            };

                            let lit_color = self.lighting.apply_lighting(base_color, light_level, fog_amount);

                            // Draw tile background for special tiles
                            let tile = state.map.tiles[map_y][map_x];
                            match tile {
                                Tile::Lava => {
                                    painter.rect_filled(tile_rect, 0.0, Color32::from_rgba_unmultiplied(150, 40, 20, 100));
                                }
                                Tile::Water => {
                                    painter.rect_filled(tile_rect, 0.0, Color32::from_rgba_unmultiplied(30, 60, 120, 80));
                                }
                                Tile::Shrine => {
                                    painter.rect_filled(tile_rect, 0.0, Color32::from_rgba_unmultiplied(120, 80, 160, 60));
                                }
                                _ => {}
                            }

                            // Draw the glyph
                            painter.text(
                                tile_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                glyph,
                                FontId::monospace(tile_size * 0.8),
                                to_egui_color(lit_color),
                            );

                            // Item glow effect
                            if is_item {
                                painter.circle_stroke(
                                    tile_rect.center(),
                                    tile_size * 0.45,
                                    Stroke::new(1.0, Color32::from_rgba_unmultiplied(
                                        base_color.r, base_color.g, base_color.b, 100
                                    )),
                                );
                            }

                            // Enemy highlight
                            if is_enemy {
                                painter.rect_stroke(
                                    tile_rect.shrink(2.0),
                                    Rounding::same(2),
                                    Stroke::new(1.0, Color32::from_rgba_unmultiplied(200, 60, 60, 100)),
                                );
                            }

                            // Fog overlay
                            if fog_amount > 0.15 {
                                let fog_alpha = ((fog_amount - 0.15) * 150.0) as u8;
                                painter.rect_filled(
                                    tile_rect, 0.0,
                                    to_egui_color_alpha(self.lighting.config().fog_color, fog_alpha),
                                );
                            }
                        } else if is_explored {
                            let tile = state.map.tiles[map_y][map_x];
                            let base_color = tile_color(tile);
                            let memory_color = self.lighting.get_memory_color(base_color);
                            painter.text(
                                tile_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                tile.glyph(),
                                FontId::monospace(tile_size * 0.8),
                                to_egui_color(memory_color),
                            );
                        }
                    }
                }

                // Render particles
                if self.settings.display.enable_particles {
                    self.particles.render(&painter);
                }

                // Vignette effect
                if self.settings.display.enable_vignette {
                    let vignette_strength = self.settings.display.vignette_intensity;
                    let fog_color = self.lighting.config().fog_color;

                    // Top and bottom vignette
                    for i in 0..4 {
                        let alpha = (vignette_strength * 255.0 * (1.0 - i as f32 / 4.0)) as u8;

                        // Top
                        painter.rect_filled(
                            Rect::from_min_size(
                                rect.min + Vec2::new(0.0, i as f32 * tile_size * 1.5),
                                Vec2::new(rect.width(), tile_size * 1.5),
                            ),
                            0.0,
                            to_egui_color_alpha(fog_color, alpha),
                        );

                        // Bottom
                        painter.rect_filled(
                            Rect::from_min_size(
                                Pos2::new(rect.min.x, rect.max.y - (i + 1) as f32 * tile_size * 1.5),
                                Vec2::new(rect.width(), tile_size * 1.5),
                            ),
                            0.0,
                            to_egui_color_alpha(fog_color, alpha),
                        );
                    }
                }

                // Draw minimap
                self.minimap.render(&painter, rect, state, self.lighting.config().fog_color);
            });

        // ========== POPUP WINDOWS ==========
        self.render_windows(ctx);

        // ========== INPUT HANDLING ==========
        self.handle_input(ctx);
    }

    fn render_windows(&mut self, ctx: &egui::Context) {
        // Settings Window
        if self.open_window == OpenWindow::Settings {
            egui::Window::new("Settings")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .frame(egui::Frame::default()
                    .fill(ThemeColors::BACKGROUND_PANEL)
                    .stroke(Stroke::new(1.0, ThemeColors::BORDER_LIGHT))
                    .rounding(Rounding::same(8)))
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.selectable_label(self.settings_tab == SettingsTab::Display, "Display").clicked() {
                            self.settings_tab = SettingsTab::Display;
                        }
                        if ui.selectable_label(self.settings_tab == SettingsTab::Gameplay, "Gameplay").clicked() {
                            self.settings_tab = SettingsTab::Gameplay;
                        }
                        if ui.selectable_label(self.settings_tab == SettingsTab::Controls, "Controls").clicked() {
                            self.settings_tab = SettingsTab::Controls;
                        }
                    });

                    ui.separator();

                    match self.settings_tab {
                        SettingsTab::Display => {
                            ui.add(egui::Slider::new(&mut self.settings.display.tile_size, 14.0..=28.0).text("Tile Size"));
                            ui.add(egui::Slider::new(&mut self.settings.display.zoom_level, 0.5..=2.0).text("Zoom"));
                            ui.checkbox(&mut self.settings.display.show_fps, "Show FPS");
                            ui.checkbox(&mut self.settings.display.torch_flicker, "Torch Flicker");
                            ui.checkbox(&mut self.settings.display.enable_vignette, "Vignette Effect");
                            ui.checkbox(&mut self.settings.display.enable_particles, "Particle Effects");
                            if self.settings.display.enable_vignette {
                                ui.add(egui::Slider::new(&mut self.settings.display.vignette_intensity, 0.1..=0.6).text("Vignette"));
                            }
                        }
                        SettingsTab::Gameplay => {
                            ui.checkbox(&mut self.settings.gameplay.auto_pickup_gold, "Auto-pickup Gold");
                            ui.checkbox(&mut self.settings.gameplay.confirm_stairs, "Confirm Stairs");
                            ui.checkbox(&mut self.settings.gameplay.show_damage_numbers, "Show Damage Numbers");
                            ui.checkbox(&mut self.settings.gameplay.click_to_move, "Click to Move");
                            ui.add(egui::Slider::new(&mut self.settings.gameplay.auto_play_speed, 50..=500).text("Auto-play Speed (ms)"));
                        }
                        SettingsTab::Controls => {
                            ui.label(RichText::new("Movement").color(ThemeColors::ACCENT_GOLD));
                            ui.label("WASD / Arrow Keys - Move");
                            ui.label("Y U B N - Diagonal movement");
                            ui.add_space(10.0);
                            ui.label(RichText::new("Actions").color(ThemeColors::ACCENT_GOLD));
                            ui.label("Space - Use current skill");
                            ui.label("Tab - Cycle skills");
                            ui.label("1-9 - Use inventory items");
                            ui.label(". (Period) - Descend stairs");
                            ui.add_space(10.0);
                            ui.label(RichText::new("Interface").color(ThemeColors::ACCENT_GOLD));
                            ui.label("M - Toggle minimap");
                            ui.label("I - Inventory window");
                            ui.label("C - Character window");
                            ui.label("Esc - Close windows / Settings");
                        }
                    }

                    ui.add_space(10.0);
                    if ui.button("Close").clicked() {
                        self.open_window = OpenWindow::None;
                    }
                });
        }

        // Help Window
        if self.open_window == OpenWindow::Help {
            egui::Window::new("Help")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .frame(egui::Frame::default()
                    .fill(ThemeColors::BACKGROUND_PANEL)
                    .stroke(Stroke::new(1.0, ThemeColors::BORDER_LIGHT))
                    .rounding(Rounding::same(8)))
                .show(ctx, |ui| {
                    ui.label(RichText::new("Welcome to ShadowCrypt!").size(16.0).color(ThemeColors::ACCENT_GOLD));
                    ui.add_space(10.0);
                    ui.label("Descend through the dungeon, defeat enemies, and find the boss on floor 30.");
                    ui.add_space(10.0);
                    ui.label(RichText::new("Tips:").color(ThemeColors::ACCENT_CYAN));
                    ui.label("- Use skills wisely - they have cooldowns");
                    ui.label("- Pick up items and equipment");
                    ui.label("- Watch your HP and MP bars");
                    ui.label("- Different dungeon themes have different enemies");
                    ui.add_space(10.0);
                    if ui.button("Got it!").clicked() {
                        self.open_window = OpenWindow::None;
                    }
                });
        }

        // Inventory Window
        if self.open_window == OpenWindow::Inventory {
            egui::Window::new("Inventory")
                .collapsible(false)
                .resizable(true)
                .default_size([350.0, 400.0])
                .frame(egui::Frame::default()
                    .fill(ThemeColors::BACKGROUND_PANEL)
                    .stroke(Stroke::new(1.0, ThemeColors::BORDER_LIGHT))
                    .rounding(Rounding::same(8)))
                .show(ctx, |ui| {
                    if let Some(state) = &self.state {
                        ui.label(RichText::new(format!("Items: {}/20", state.player.inventory.len()))
                            .color(ThemeColors::TEXT_DIM));
                        ui.separator();

                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for (i, item) in state.player.inventory.iter().enumerate() {
                                let item_color = to_egui_color(rarity_color(item.rarity));
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new(format!("[{}]", if i < 9 { (i + 1).to_string() } else { "0".to_string() }))
                                        .size(11.0).color(ThemeColors::TEXT_MUTED));

                                    if ui.button(RichText::new(item.display_name()).color(item_color)).clicked() {
                                        if let Some(state) = &mut self.state {
                                            state.use_item(i);
                                        }
                                    }
                                });
                            }
                        });
                    }

                    ui.add_space(10.0);
                    if ui.button("Close").clicked() {
                        self.open_window = OpenWindow::None;
                    }
                });
        }

        // Character Window
        if self.open_window == OpenWindow::Character {
            egui::Window::new("Character")
                .collapsible(false)
                .resizable(false)
                .default_size([300.0, 400.0])
                .frame(egui::Frame::default()
                    .fill(ThemeColors::BACKGROUND_PANEL)
                    .stroke(Stroke::new(1.0, ThemeColors::BORDER_LIGHT))
                    .rounding(Rounding::same(8)))
                .show(ctx, |ui| {
                    if let Some(state) = &self.state {
                        ui.label(RichText::new(state.player.class.name()).size(18.0).color(ThemeColors::ACCENT_GOLD));
                        ui.label(RichText::new(format!("Level {}", state.player.level)).color(ThemeColors::TEXT_NORMAL));
                        ui.separator();

                        ui.label(RichText::new("Stats").color(ThemeColors::ACCENT_CYAN));
                        ui.label(format!("HP: {} / {}", state.player.hp, state.player.total_max_hp()));
                        ui.label(format!("MP: {} / {}", state.player.mana, state.player.total_max_mana()));
                        ui.label(format!("Attack: {}", state.player.total_attack()));
                        ui.label(format!("Defense: {}", state.player.total_defense()));
                        ui.add_space(10.0);

                        ui.label(RichText::new("Progress").color(ThemeColors::ACCENT_CYAN));
                        ui.label(format!("XP: {} / {}", state.player.xp, state.player.xp_to_level));
                        ui.label(format!("Gold: {}", state.player.gold));
                        ui.label(format!("Kills: {}", state.player.kills));
                        ui.label(format!("Current Floor: {}", state.dungeon_level));
                    }

                    ui.add_space(10.0);
                    if ui.button("Close").clicked() {
                        self.open_window = OpenWindow::None;
                    }
                });
        }
    }

    fn handle_input(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            // Close windows with Escape
            if i.key_pressed(egui::Key::Escape) {
                if self.open_window != OpenWindow::None {
                    self.open_window = OpenWindow::None;
                } else {
                    self.open_window = OpenWindow::Settings;
                }
            }

            // Window toggles
            if i.key_pressed(egui::Key::I) {
                self.open_window = if self.open_window == OpenWindow::Inventory {
                    OpenWindow::None
                } else {
                    OpenWindow::Inventory
                };
            }
            if i.key_pressed(egui::Key::C) {
                self.open_window = if self.open_window == OpenWindow::Character {
                    OpenWindow::None
                } else {
                    OpenWindow::Character
                };
            }

            // Minimap controls
            if i.key_pressed(egui::Key::M) {
                self.minimap.toggle();
            }
            if i.key_pressed(egui::Key::PlusEquals) {
                self.minimap.zoom_in();
            }
            if i.key_pressed(egui::Key::Minus) {
                self.minimap.zoom_out();
            }

            if let Some(state) = &mut self.state {
                // Movement
                if i.key_pressed(egui::Key::ArrowUp) || i.key_pressed(egui::Key::W) {
                    state.move_player(0, -1);
                    self.lighting.set_theme(state.map.theme, state.dungeon_level);
                }
                if i.key_pressed(egui::Key::ArrowDown) || i.key_pressed(egui::Key::S) {
                    state.move_player(0, 1);
                    self.lighting.set_theme(state.map.theme, state.dungeon_level);
                }
                if i.key_pressed(egui::Key::ArrowLeft) || i.key_pressed(egui::Key::A) {
                    state.move_player(-1, 0);
                    self.lighting.set_theme(state.map.theme, state.dungeon_level);
                }
                if i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::D) {
                    state.move_player(1, 0);
                    self.lighting.set_theme(state.map.theme, state.dungeon_level);
                }

                // Diagonal
                if i.key_pressed(egui::Key::Y) { state.move_player(-1, -1); self.lighting.set_theme(state.map.theme, state.dungeon_level); }
                if i.key_pressed(egui::Key::U) { state.move_player(1, -1); self.lighting.set_theme(state.map.theme, state.dungeon_level); }
                if i.key_pressed(egui::Key::B) { state.move_player(-1, 1); self.lighting.set_theme(state.map.theme, state.dungeon_level); }
                if i.key_pressed(egui::Key::N) { state.move_player(1, 1); self.lighting.set_theme(state.map.theme, state.dungeon_level); }

                // Skills
                if i.key_pressed(egui::Key::Space) {
                    let skill_idx = state.player.active_skill;
                    if skill_idx < state.player.skills.len() {
                        let skill = state.player.skills[skill_idx];
                        let cooldown = *self.skill_cooldowns.get(&skill).unwrap_or(&0);
                        if cooldown == 0 && state.player.mana >= skill.mana_cost() {
                            state.use_skill();
                            self.skill_cooldowns.insert(skill, get_max_cooldown(skill));
                        }
                    }
                }
                if i.key_pressed(egui::Key::Tab) { state.cycle_skill(); }

                if i.key_pressed(egui::Key::F1) { self.activate_skill_by_index(0); }
                if i.key_pressed(egui::Key::F2) { self.activate_skill_by_index(1); }
                if i.key_pressed(egui::Key::F3) { self.activate_skill_by_index(2); }
                if i.key_pressed(egui::Key::F4) { self.activate_skill_by_index(3); }

                // Stairs
                if i.key_pressed(egui::Key::Period) {
                    state.descend();
                    self.lighting.set_theme(state.map.theme, state.dungeon_level);
                }
                if i.key_pressed(egui::Key::Comma) {
                    state.ascend();
                    self.lighting.set_theme(state.map.theme, state.dungeon_level);
                }

                // Wait
                if i.key_pressed(egui::Key::Num5) || i.key_pressed(egui::Key::Z) {
                    state.end_turn();
                }

                // Items
                for (idx, key) in [
                    egui::Key::Num1, egui::Key::Num2, egui::Key::Num3, egui::Key::Num4,
                    egui::Key::Num6, egui::Key::Num7, egui::Key::Num8, egui::Key::Num9, egui::Key::Num0,
                ].iter().enumerate() {
                    if i.key_pressed(*key) {
                        let item_idx = if idx >= 4 { idx + 1 } else { idx };
                        let item_idx = if *key == egui::Key::Num0 { 9 } else { item_idx };
                        state.use_item(item_idx);
                    }
                }

                // Auto-play toggle
                if i.key_pressed(egui::Key::P) {
                    self.auto_play = !self.auto_play;
                }
            }
        });
    }

    fn render_game_over(&mut self, ctx: &egui::Context) {
        let state = self.state.as_ref().unwrap();

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(Color32::from_rgb(15, 8, 8)))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(80.0);

                    // Death skull effect
                    ui.label(RichText::new("GAME OVER")
                        .size(64.0)
                        .color(ThemeColors::ACCENT_RED));

                    ui.add_space(10.0);

                    ui.label(RichText::new("The darkness has claimed another soul...")
                        .size(16.0)
                        .color(Color32::from_rgb(150, 80, 80)));

                    ui.add_space(40.0);

                    // Stats frame
                    egui::Frame::none()
                        .fill(ThemeColors::BACKGROUND_PANEL)
                        .rounding(Rounding::same(8))
                        .stroke(Stroke::new(1.0, ThemeColors::BORDER_DARK))
                        .inner_margin(20.0)
                        .show(ui, |ui| {
                            ui.label(RichText::new("Final Record").size(18.0).color(ThemeColors::ACCENT_GOLD));
                            ui.separator();
                            ui.label(RichText::new(format!("Class: {}", state.player.class.name()))
                                .color(ThemeColors::TEXT_NORMAL));
                            ui.label(RichText::new(format!("Level: {}", state.player.level))
                                .color(ThemeColors::TEXT_NORMAL));
                            ui.label(RichText::new(format!("Floor Reached: {}", state.dungeon_level))
                                .color(ThemeColors::TEXT_NORMAL));
                            ui.label(RichText::new(format!("Gold Collected: {}", state.player.gold))
                                .color(ThemeColors::ACCENT_GOLD));
                            ui.label(RichText::new(format!("Enemies Slain: {}", state.player.kills))
                                .color(ThemeColors::ACCENT_RED));
                            ui.label(RichText::new(format!("Turns Survived: {}", state.turn_count))
                                .color(ThemeColors::TEXT_DIM));
                        });

                    ui.add_space(30.0);

                    if ui.add(egui::Button::new(RichText::new("Try Again").size(18.0))
                        .min_size(Vec2::new(150.0, 40.0))).clicked() {
                        self.state = None;
                        self.app_state = AppState::ClassSelect;
                    }
                });
            });
    }

    fn render_victory(&mut self, ctx: &egui::Context) {
        let state = self.state.as_ref().unwrap();

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(Color32::from_rgb(12, 15, 20)))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(60.0);

                    ui.label(RichText::new("VICTORY!")
                        .size(72.0)
                        .color(ThemeColors::ACCENT_GOLD));

                    ui.add_space(10.0);

                    ui.label(RichText::new("The Demon King has been vanquished!")
                        .size(20.0)
                        .color(ThemeColors::ACCENT_CYAN));

                    ui.label(RichText::new("Light returns to the realm of ShadowCrypt")
                        .size(14.0)
                        .color(ThemeColors::TEXT_DIM));

                    ui.add_space(40.0);

                    // Victory stats
                    egui::Frame::none()
                        .fill(ThemeColors::BACKGROUND_PANEL)
                        .rounding(Rounding::same(8))
                        .stroke(Stroke::new(2.0, ThemeColors::ACCENT_GOLD))
                        .inner_margin(25.0)
                        .show(ui, |ui| {
                            ui.label(RichText::new("Champion's Record").size(20.0).color(ThemeColors::ACCENT_GOLD));
                            ui.separator();
                            ui.add_space(10.0);

                            ui.label(RichText::new(format!("Hero Class: {}", state.player.class.name()))
                                .size(14.0).color(ThemeColors::TEXT_BRIGHT));
                            ui.label(RichText::new(format!("Final Level: {}", state.player.level))
                                .size(14.0).color(ThemeColors::ACCENT_GREEN));
                            ui.label(RichText::new(format!("Gold Amassed: {}", state.player.gold))
                                .size(14.0).color(ThemeColors::ACCENT_GOLD));
                            ui.label(RichText::new(format!("Foes Defeated: {}", state.player.kills))
                                .size(14.0).color(ThemeColors::ACCENT_RED));
                            ui.label(RichText::new(format!("Journey Duration: {} turns", state.turn_count))
                                .size(14.0).color(ThemeColors::TEXT_DIM));
                        });

                    ui.add_space(40.0);

                    if ui.add(egui::Button::new(RichText::new("Begin New Legend").size(18.0).color(ThemeColors::ACCENT_GOLD))
                        .min_size(Vec2::new(180.0, 45.0))
                        .fill(ThemeColors::BACKGROUND_DARK)
                        .stroke(Stroke::new(2.0, ThemeColors::ACCENT_GOLD))).clicked() {
                        self.state = None;
                        self.app_state = AppState::ClassSelect;
                    }
                });
            });
    }
}

impl eframe::App for ShadowCryptApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set dark visuals with custom styling
        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = ThemeColors::BACKGROUND_PANEL;
        visuals.panel_fill = ThemeColors::BACKGROUND_DARK;
        visuals.widgets.noninteractive.bg_fill = ThemeColors::BACKGROUND_MEDIUM;
        visuals.widgets.inactive.bg_fill = ThemeColors::BACKGROUND_LIGHT;
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(45, 45, 60);
        visuals.widgets.active.bg_fill = Color32::from_rgb(55, 55, 75);
        visuals.selection.bg_fill = ThemeColors::ACCENT_PURPLE.gamma_multiply(0.5);
        ctx.set_visuals(visuals);

        match &self.app_state {
            AppState::Loading { .. } => self.render_loading(ctx),
            AppState::ClassSelect => self.render_class_select(ctx),
            AppState::Playing => self.render_game(ctx),
            AppState::GameOver => self.render_game_over(ctx),
            AppState::Victory => self.render_victory(ctx),
        }
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([1024.0, 768.0])
            .with_title("ShadowCrypt - Dark Fantasy Roguelike"),
        ..Default::default()
    };

    eframe::run_native(
        "ShadowCrypt",
        options,
        Box::new(|cc| Ok(Box::new(ShadowCryptApp::new(cc)))),
    )
}
