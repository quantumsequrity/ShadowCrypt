//! ShadowCrypt GUI - Graphical roguelike dungeon crawler using egui
//!
//! This is the GUI version of the ShadowCrypt roguelike game using egui/eframe.

use eframe::egui;
use rand::prelude::*;

use shadowcrypt_core::prelude::*;
use shadowcrypt_core::ai::{AIAction, AutoPlayAI};
use shadowcrypt_core::ui::{Color, tile_color, enemy_color, rarity_color};

/// Convert our color type to egui color
fn to_egui_color(color: Color) -> egui::Color32 {
    egui::Color32::from_rgb(color.r, color.g, color.b)
}

/// Application state
enum AppState {
    ClassSelect,
    Playing,
    GameOver,
    Victory,
}

/// Main application
struct ShadowCryptApp {
    state: Option<GameState>,
    app_state: AppState,
    auto_play: bool,
    auto_ai: AutoPlayAI,
    last_update: std::time::Instant,
    tile_size: f32,
}

impl Default for ShadowCryptApp {
    fn default() -> Self {
        Self {
            state: None,
            app_state: AppState::ClassSelect,
            auto_play: false,
            auto_ai: AutoPlayAI::new(),
            last_update: std::time::Instant::now(),
            tile_size: 16.0,
        }
    }
}

impl ShadowCryptApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn start_game(&mut self, class: CharacterClass) {
        self.state = Some(GameState::new(class));
        self.app_state = AppState::Playing;
    }

    fn render_class_select(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);
                ui.heading(egui::RichText::new("SHADOWCRYPT").size(48.0).color(egui::Color32::from_rgb(0, 200, 200)));
                ui.add_space(20.0);
                ui.label("Choose your class:");
                ui.add_space(20.0);

                for class in CharacterClass::all() {
                    let (hp, atk, def, mana, _spd) = class.base_stats();
                    let button_text = format!(
                        "{:<12} HP:{:<3} ATK:{:<2} DEF:{:<2} MP:{:<3}",
                        class.name(), hp, atk, def, mana
                    );

                    if ui.button(egui::RichText::new(button_text).monospace().size(16.0)).clicked() {
                        self.start_game(*class);
                    }
                    ui.label(egui::RichText::new(format!("  {}", class.special_ability())).size(12.0).color(egui::Color32::GRAY));
                    ui.add_space(10.0);
                }

                ui.add_space(20.0);
                ui.checkbox(&mut self.auto_play, "Auto-play mode");
            });
        });
    }

    fn render_game(&mut self, ctx: &egui::Context) {
        let state = match &self.state {
            Some(s) => s,
            None => return,
        };

        // Handle auto-play
        if self.auto_play && !state.game_over && !state.victory {
            let elapsed = self.last_update.elapsed();
            if elapsed.as_millis() > 100 {
                self.last_update = std::time::Instant::now();

                let state = self.state.as_ref().unwrap();
                let action = self.auto_ai.decide(
                    state.player.x,
                    state.player.y,
                    state.player.hp,
                    state.player.total_max_hp(),
                    state.player.mana,
                    state.player.can_use_skill(),
                    &state.map,
                    &state.enemies,
                    state.player.find_health_potion(),
                    state.player.find_mana_potion(),
                    state.dungeon_level,
                    state.boss_defeated,
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
            }
            ctx.request_repaint();
        }

        let state = self.state.as_ref().unwrap();

        // Check game state
        if state.game_over {
            self.app_state = AppState::GameOver;
        } else if state.victory {
            self.app_state = AppState::Victory;
        }

        // Top panel for stats
        egui::TopBottomPanel::top("stats").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let hp_color = if state.player.hp as f32 / state.player.total_max_hp() as f32 > 0.3 {
                    egui::Color32::GREEN
                } else {
                    egui::Color32::RED
                };

                ui.label(egui::RichText::new(format!("HP: {}/{}", state.player.hp, state.player.total_max_hp())).color(hp_color));
                ui.separator();
                ui.label(egui::RichText::new(format!("MP: {}/{}", state.player.mana, state.player.total_max_mana())).color(egui::Color32::from_rgb(100, 100, 255)));
                ui.separator();
                ui.label(egui::RichText::new(format!("Gold: {}", state.player.gold)).color(egui::Color32::YELLOW));
                ui.separator();
                ui.label(format!("Level: {}", state.player.level));
                ui.separator();
                ui.label(format!("XP: {}/{}", state.player.xp, state.player.xp_to_level));
                ui.separator();
                ui.label(format!("Floor: {} - {}", state.dungeon_level, state.map.theme.name()));

                if let Some(skill) = state.player.current_skill() {
                    ui.separator();
                    ui.label(egui::RichText::new(format!("[Space] {} ({}mp)", skill.name(), skill.mana_cost())).color(egui::Color32::from_rgb(0, 200, 200)));
                }
            });
        });

        // Bottom panel for messages
        egui::TopBottomPanel::bottom("messages").show(ctx, |ui| {
            ui.vertical(|ui| {
                for (msg, color) in &state.messages {
                    ui.label(egui::RichText::new(msg).color(to_egui_color(*color)));
                }
            });
        });

        // Side panel for inventory
        egui::SidePanel::right("inventory").min_width(200.0).show(ctx, |ui| {
            ui.heading("Inventory");
            ui.separator();

            if state.player.inventory.is_empty() {
                ui.label(egui::RichText::new("(empty)").color(egui::Color32::GRAY));
            } else {
                for (i, item) in state.player.inventory.iter().enumerate().take(10) {
                    let key = if i == 9 { '0' } else { char::from_digit((i + 1) as u32, 10).unwrap() };
                    let color = to_egui_color(rarity_color(item.rarity));
                    if ui.button(egui::RichText::new(format!("[{}] {}", key, item.display_name())).color(color)).clicked() {
                        if let Some(state) = &mut self.state {
                            state.use_item(i);
                        }
                    }
                }
            }

            ui.separator();
            ui.heading("Equipment");
            for slot in [EquipSlot::Weapon, EquipSlot::Armor, EquipSlot::Shield] {
                if let Some(item) = state.player.equipment.get(&slot) {
                    let color = to_egui_color(rarity_color(item.rarity));
                    ui.label(egui::RichText::new(format!("{}: {}", slot.name(), item.display_name())).color(color));
                }
            }
        });

        // Central panel for the game map
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();
            let view_width = (available_size.x / self.tile_size) as usize;
            let view_height = (available_size.y / self.tile_size) as usize;

            // Calculate viewport centered on player
            let half_width = view_width / 2;
            let half_height = view_height / 2;
            let view_x = state.player.x.saturating_sub(half_width);
            let view_y = state.player.y.saturating_sub(half_height);

            let (response, painter) = ui.allocate_painter(available_size, egui::Sense::click());
            let rect = response.rect;

            // Draw map
            for y in 0..view_height.min(MAP_HEIGHT) {
                let map_y = view_y + y;
                if map_y >= MAP_HEIGHT {
                    break;
                }

                for x in 0..view_width.min(MAP_WIDTH) {
                    let map_x = view_x + x;
                    if map_x >= MAP_WIDTH {
                        break;
                    }

                    let screen_x = rect.min.x + x as f32 * self.tile_size;
                    let screen_y = rect.min.y + y as f32 * self.tile_size;
                    let tile_rect = egui::Rect::from_min_size(
                        egui::pos2(screen_x, screen_y),
                        egui::vec2(self.tile_size, self.tile_size),
                    );

                    let (glyph, color) = if map_x == state.player.x && map_y == state.player.y {
                        ('@', egui::Color32::YELLOW)
                    } else if state.map.visible[map_y][map_x] {
                        if let Some(enemy) = state.enemies.iter().find(|e| e.is_alive() && e.x == map_x && e.y == map_y) {
                            (enemy.kind.glyph(), to_egui_color(enemy_color(enemy.kind)))
                        } else if let Some(item) = state.items.iter().find(|i| i.x == map_x && i.y == map_y) {
                            (item.kind.glyph(), to_egui_color(rarity_color(item.rarity)))
                        } else {
                            let tile = state.map.tiles[map_y][map_x];
                            (tile.glyph(), to_egui_color(tile_color(tile)))
                        }
                    } else if state.map.explored[map_y][map_x] {
                        let tile = state.map.tiles[map_y][map_x];
                        (tile.glyph(), egui::Color32::from_gray(40))
                    } else {
                        (' ', egui::Color32::BLACK)
                    };

                    painter.text(
                        tile_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        glyph,
                        egui::FontId::monospace(self.tile_size * 0.8),
                        color,
                    );
                }
            }
        });

        // Handle keyboard input
        ctx.input(|i| {
            if let Some(state) = &mut self.state {
                // Movement
                if i.key_pressed(egui::Key::ArrowUp) || i.key_pressed(egui::Key::W) {
                    state.move_player(0, -1);
                }
                if i.key_pressed(egui::Key::ArrowDown) || i.key_pressed(egui::Key::S) {
                    state.move_player(0, 1);
                }
                if i.key_pressed(egui::Key::ArrowLeft) || i.key_pressed(egui::Key::A) {
                    state.move_player(-1, 0);
                }
                if i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::D) {
                    state.move_player(1, 0);
                }

                // Skills
                if i.key_pressed(egui::Key::Space) {
                    state.use_skill();
                }
                if i.key_pressed(egui::Key::Tab) {
                    state.cycle_skill();
                }

                // Stairs
                if i.key_pressed(egui::Key::Period) {
                    state.descend();
                }
                if i.key_pressed(egui::Key::Comma) {
                    state.ascend();
                }

                // Quick use items
                for (idx, key) in [
                    egui::Key::Num1, egui::Key::Num2, egui::Key::Num3, egui::Key::Num4, egui::Key::Num5,
                    egui::Key::Num6, egui::Key::Num7, egui::Key::Num8, egui::Key::Num9, egui::Key::Num0,
                ].iter().enumerate() {
                    if i.key_pressed(*key) {
                        let item_idx = if idx == 9 { 9 } else { idx };
                        state.use_item(item_idx);
                    }
                }
            }
        });
    }

    fn render_game_over(&mut self, ctx: &egui::Context) {
        let state = self.state.as_ref().unwrap();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                ui.heading(egui::RichText::new("GAME OVER").size(48.0).color(egui::Color32::RED));
                ui.add_space(30.0);
                ui.label(format!("Class: {}", state.player.class.name()));
                ui.label(format!("Died on floor {} after {} turns", state.dungeon_level, state.turn_count));
                ui.label(format!("Level: {} | Gold: {} | Kills: {}", state.player.level, state.player.gold, state.player.kills));
                ui.add_space(30.0);
                if ui.button("Play Again").clicked() {
                    self.state = None;
                    self.app_state = AppState::ClassSelect;
                }
            });
        });
    }

    fn render_victory(&mut self, ctx: &egui::Context) {
        let state = self.state.as_ref().unwrap();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                ui.heading(egui::RichText::new("VICTORY!").size(48.0).color(egui::Color32::GOLD));
                ui.add_space(10.0);
                ui.heading(egui::RichText::new("You have conquered ShadowCrypt!").color(egui::Color32::from_rgb(0, 200, 200)));
                ui.add_space(30.0);
                ui.label(format!("Class: {}", state.player.class.name()));
                ui.label(format!("Final Level: {}", state.player.level));
                ui.label(format!("Gold Collected: {}", state.player.gold));
                ui.label(format!("Enemies Slain: {}", state.player.kills));
                ui.label(format!("Turns Taken: {}", state.turn_count));
                ui.add_space(30.0);
                if ui.button("Play Again").clicked() {
                    self.state = None;
                    self.app_state = AppState::ClassSelect;
                }
            });
        });
    }
}

impl eframe::App for ShadowCryptApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.app_state {
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
            .with_inner_size([1024.0, 768.0])
            .with_title("ShadowCrypt"),
        ..Default::default()
    };

    eframe::run_native(
        "ShadowCrypt",
        options,
        Box::new(|cc| Ok(Box::new(ShadowCryptApp::new(cc)))),
    )
}
