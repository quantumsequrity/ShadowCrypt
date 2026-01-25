//! ShadowCrypt Multi-Agent GUI
//!
//! Graphical visualization of 75+ agents running in parallel
//! and communicating with each other using egui.

use eframe::egui;
use rand::prelude::*;
use std::time::Instant;

use shadowcrypt_core::prelude::*;
use shadowcrypt_core::ui::Color;
use shadowcrypt_agents::prelude::*;

/// Convert our color type to egui color
fn to_egui_color(color: Color) -> egui::Color32 {
    egui::Color32::from_rgb(color.r, color.g, color.b)
}

/// Application state
enum AppState {
    Running,
    Paused,
}

/// Selected panel in the UI
#[derive(Clone, Copy, PartialEq)]
enum Panel {
    Agents,
    Factions,
    Communications,
    Statistics,
}

/// Main application
struct MultiAgentApp {
    /// Agent manager
    agents: AgentManager,
    /// Game map for visualization
    map: Map,
    /// App state
    app_state: AppState,
    /// Turn count
    turn: u32,
    /// Last update time
    last_update: Instant,
    /// Simulation speed (ms per turn)
    speed: u64,
    /// Tile size for rendering
    tile_size: f32,
    /// Camera position
    camera_x: f32,
    camera_y: f32,
    /// Selected agent
    selected_agent: Option<AgentId>,
    /// Active panel
    active_panel: Panel,
    /// Communication log
    comm_log: Vec<String>,
    /// Event log
    event_log: Vec<(String, egui::Color32)>,
    /// Show grid
    show_grid: bool,
    /// Show agent paths
    show_paths: bool,
    /// Show faction territories
    show_territories: bool,
    /// RNG
    rng: StdRng,
}

impl Default for MultiAgentApp {
    fn default() -> Self {
        let mut rng = StdRng::from_entropy();
        let mut map = Map::new();
        map.generate(&mut rng, 1);

        let mut agents = AgentManager::with_seed(rng.gen());
        agents.spawn_default_agents(MAP_WIDTH, MAP_HEIGHT);

        Self {
            agents,
            map,
            app_state: AppState::Running,
            turn: 0,
            last_update: Instant::now(),
            speed: 200,
            tile_size: 12.0,
            camera_x: (MAP_WIDTH / 2) as f32,
            camera_y: (MAP_HEIGHT / 2) as f32,
            selected_agent: None,
            active_panel: Panel::Agents,
            comm_log: Vec::new(),
            event_log: vec![
                ("Welcome to ShadowCrypt Multi-Agent Simulation!".to_string(), egui::Color32::from_rgb(0, 200, 200)),
                ("75+ agents are running in parallel".to_string(), egui::Color32::YELLOW),
            ],
            show_grid: false,
            show_paths: false,
            show_territories: false,
            rng,
        }
    }
}

impl MultiAgentApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn add_event(&mut self, msg: String, color: egui::Color32) {
        self.event_log.push((msg, color));
        if self.event_log.len() > 10 {
            self.event_log.remove(0);
        }
    }

    fn add_comm(&mut self, msg: String) {
        self.comm_log.push(msg);
        if self.comm_log.len() > 50 {
            self.comm_log.remove(0);
        }
    }

    fn process_turn(&mut self) {
        self.turn += 1;
        self.agents.process_turn();

        // Process communications
        let messages = self.agents.message_bus.messages.clone();
        for msg in messages.iter().take(5) {
            let from_name = self.agents.get(msg.from)
                .map(|a| a.name.clone())
                .unwrap_or_else(|| "Unknown".to_string());
            let to_name = self.agents.get(msg.to)
                .map(|a| a.name.clone())
                .unwrap_or_else(|| "Unknown".to_string());

            self.add_comm(format!("[{}] {} -> {}: {}", self.turn, from_name, to_name, msg.content));
        }

        // Periodic events
        if self.turn % 50 == 0 {
            let stats = self.agents.stats();
            self.add_event(
                format!("Turn {} - {} agents active", self.turn, stats.alive),
                egui::Color32::from_rgb(0, 200, 200),
            );
        }

        // Random dialogue
        if self.turn % 10 == 0 {
            let agents: Vec<_> = self.agents.all()
                .filter(|a| a.is_alive() && a.kind.category() == AgentCategory::Npc)
                .collect();

            if !agents.is_empty() {
                let agent = agents[self.rng.gen_range(0..agents.len())];
                let dialogue = NpcBehaviors::random_dialogue(agent.kind, &mut self.rng);
                self.add_event(
                    format!("{}: \"{}\"", agent.name, dialogue),
                    egui::Color32::WHITE,
                );
            }
        }
    }

    fn render_toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Play/Pause button
            let btn_text = match self.app_state {
                AppState::Running => "⏸ Pause",
                AppState::Paused => "▶ Play",
            };
            if ui.button(btn_text).clicked() {
                self.app_state = match self.app_state {
                    AppState::Running => AppState::Paused,
                    AppState::Paused => AppState::Running,
                };
            }

            // Speed control
            ui.separator();
            ui.label("Speed:");
            if ui.button("-").clicked() {
                self.speed = (self.speed + 50).min(1000);
            }
            ui.label(format!("{}ms", self.speed));
            if ui.button("+").clicked() {
                self.speed = self.speed.saturating_sub(50).max(50);
            }

            // Step button (when paused)
            if matches!(self.app_state, AppState::Paused) {
                ui.separator();
                if ui.button("Step").clicked() {
                    self.process_turn();
                }
            }

            ui.separator();
            ui.label(format!("Turn: {} | Agents: {}", self.turn, self.agents.living_count()));

            // Zoom control
            ui.separator();
            ui.label("Zoom:");
            if ui.button("-").clicked() {
                self.tile_size = (self.tile_size - 2.0).max(6.0);
            }
            ui.label(format!("{:.0}", self.tile_size));
            if ui.button("+").clicked() {
                self.tile_size = (self.tile_size + 2.0).min(24.0);
            }

            // View options
            ui.separator();
            ui.checkbox(&mut self.show_grid, "Grid");
            ui.checkbox(&mut self.show_territories, "Territories");
        });
    }

    fn render_map(&mut self, ui: &mut egui::Ui) {
        let available_size = ui.available_size();
        let view_width = (available_size.x / self.tile_size) as usize;
        let view_height = (available_size.y / self.tile_size) as usize;

        // Calculate viewport
        let half_width = view_width as f32 / 2.0;
        let half_height = view_height as f32 / 2.0;
        let view_x = (self.camera_x - half_width).max(0.0) as usize;
        let view_y = (self.camera_y - half_height).max(0.0) as usize;

        let (response, painter) = ui.allocate_painter(available_size, egui::Sense::click_and_drag());
        let rect = response.rect;

        // Handle dragging to pan
        if response.dragged() {
            let delta = response.drag_delta();
            self.camera_x -= delta.x / self.tile_size;
            self.camera_y -= delta.y / self.tile_size;
            self.camera_x = self.camera_x.clamp(0.0, MAP_WIDTH as f32);
            self.camera_y = self.camera_y.clamp(0.0, MAP_HEIGHT as f32);
        }

        // Handle clicks for selection
        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                let rel_pos = pos - rect.min;
                let map_x = view_x + (rel_pos.x / self.tile_size) as usize;
                let map_y = view_y + (rel_pos.y / self.tile_size) as usize;

                let agents = self.agents.at_position(map_x, map_y);
                if let Some(agent) = agents.first() {
                    self.selected_agent = Some(agent.id);
                    self.add_event(
                        format!("Selected: {} ({})", agent.name, agent.kind.name()),
                        egui::Color32::YELLOW,
                    );
                } else {
                    self.selected_agent = None;
                }
            }
        }

        // Draw background
        painter.rect_filled(rect, 0.0, egui::Color32::from_gray(20));

        // Draw grid if enabled
        if self.show_grid {
            for x in 0..=view_width {
                let screen_x = rect.min.x + x as f32 * self.tile_size;
                painter.line_segment(
                    [egui::pos2(screen_x, rect.min.y), egui::pos2(screen_x, rect.max.y)],
                    egui::Stroke::new(0.5, egui::Color32::from_gray(40)),
                );
            }
            for y in 0..=view_height {
                let screen_y = rect.min.y + y as f32 * self.tile_size;
                painter.line_segment(
                    [egui::pos2(rect.min.x, screen_y), egui::pos2(rect.max.x, screen_y)],
                    egui::Stroke::new(0.5, egui::Color32::from_gray(40)),
                );
            }
        }

        // Draw tiles and agents
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

                // Check for agents
                let agents_here = self.agents.at_position(map_x, map_y);
                if let Some(agent) = agents_here.first() {
                    if agent.is_alive() && agent.visible {
                        let (r, g, b) = agent.color();
                        let color = egui::Color32::from_rgb(r, g, b);

                        // Highlight selected
                        if self.selected_agent == Some(agent.id) {
                            painter.rect_filled(tile_rect, 2.0, egui::Color32::from_gray(60));
                        }

                        painter.text(
                            tile_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            agent.glyph(),
                            egui::FontId::monospace(self.tile_size * 0.8),
                            color,
                        );
                        continue;
                    }
                }

                // Draw tile
                let tile = self.map.tiles[map_y][map_x];
                let tile_color = match tile {
                    Tile::Floor => egui::Color32::from_gray(35),
                    Tile::Wall => egui::Color32::from_gray(80),
                    Tile::StairsDown | Tile::StairsUp => egui::Color32::YELLOW,
                    Tile::Door | Tile::OpenDoor => egui::Color32::from_rgb(139, 90, 43),
                    Tile::Water => egui::Color32::from_rgb(30, 144, 255),
                    Tile::Lava => egui::Color32::from_rgb(255, 69, 0),
                    _ => egui::Color32::from_gray(30),
                };

                painter.text(
                    tile_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    tile.glyph(),
                    egui::FontId::monospace(self.tile_size * 0.7),
                    tile_color,
                );
            }
        }
    }

    fn render_side_panel(&mut self, ui: &mut egui::Ui) {
        // Panel selector
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.active_panel, Panel::Agents, "Agents");
            ui.selectable_value(&mut self.active_panel, Panel::Factions, "Factions");
            ui.selectable_value(&mut self.active_panel, Panel::Communications, "Comms");
            ui.selectable_value(&mut self.active_panel, Panel::Statistics, "Stats");
        });

        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            match self.active_panel {
                Panel::Agents => self.render_agents_panel(ui),
                Panel::Factions => self.render_factions_panel(ui),
                Panel::Communications => self.render_comms_panel(ui),
                Panel::Statistics => self.render_stats_panel(ui),
            }
        });
    }

    fn render_agents_panel(&mut self, ui: &mut egui::Ui) {
        // Agent categories
        for category in [AgentCategory::Npc, AgentCategory::Enemy, AgentCategory::Companion, AgentCategory::Environmental, AgentCategory::System] {
            let agents: Vec<_> = self.agents.by_category(category);
            let living: Vec<_> = agents.iter().filter(|a| a.is_alive()).collect();

            let cat_name = match category {
                AgentCategory::Npc => "NPCs",
                AgentCategory::Enemy => "Enemies",
                AgentCategory::Companion => "Companions",
                AgentCategory::Environmental => "Environmental",
                AgentCategory::System => "System",
            };

            egui::CollapsingHeader::new(format!("{} ({})", cat_name, living.len()))
                .default_open(category == AgentCategory::Npc)
                .show(ui, |ui| {
                    for agent in living.iter().take(20) {
                        let (r, g, b) = agent.color();
                        let color = egui::Color32::from_rgb(r, g, b);

                        let selected = self.selected_agent == Some(agent.id);
                        let text = format!("{} {} HP:{}/{} {:?}",
                            agent.glyph(), agent.name, agent.hp, agent.stats.max_hp, agent.state);

                        if ui.selectable_label(selected, egui::RichText::new(text).color(color)).clicked() {
                            self.selected_agent = Some(agent.id);
                            self.camera_x = agent.x as f32;
                            self.camera_y = agent.y as f32;
                        }
                    }
                });
        }

        // Selected agent details
        if let Some(id) = self.selected_agent {
            if let Some(agent) = self.agents.get(id) {
                ui.separator();
                ui.heading("Selected Agent");
                ui.label(format!("Name: {}", agent.name));
                ui.label(format!("Type: {}", agent.kind.name()));
                ui.label(format!("Position: ({}, {})", agent.x, agent.y));
                ui.label(format!("HP: {}/{}", agent.hp, agent.stats.max_hp));
                ui.label(format!("State: {:?}", agent.state));
                ui.label(format!("Mood: {:?}", agent.personality.mood));
                ui.label(format!("Goals: {}", agent.goals.len()));

                if let Some(faction_id) = agent.faction {
                    if let Some(faction) = self.agents.factions.get(faction_id) {
                        ui.label(format!("Faction: {}", faction.name));
                    }
                }

                if ui.button("Deselect").clicked() {
                    self.selected_agent = None;
                }
            }
        }
    }

    fn render_factions_panel(&mut self, ui: &mut egui::Ui) {
        for faction in self.agents.factions.all() {
            egui::CollapsingHeader::new(format!("{} ({} members)", faction.name, faction.member_count()))
                .show(ui, |ui| {
                    ui.label(format!("Type: {:?}", faction.faction_type));
                    ui.label(format!("Player Rep: {}", faction.player_reputation));

                    if !faction.traits.is_empty() {
                        ui.label(format!("Traits: {:?}", faction.traits));
                    }

                    // Show some members
                    let members: Vec<_> = faction.members.iter()
                        .filter_map(|id| self.agents.get(*id))
                        .take(5)
                        .collect();

                    if !members.is_empty() {
                        ui.label("Members:");
                        for member in members {
                            ui.label(format!("  {} - {}", member.glyph(), member.name));
                        }
                    }
                });
        }
    }

    fn render_comms_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Communication Log");
        for msg in self.comm_log.iter().rev() {
            ui.label(egui::RichText::new(msg).color(egui::Color32::GRAY).small());
        }
    }

    fn render_stats_panel(&mut self, ui: &mut egui::Ui) {
        let stats = self.agents.stats();

        ui.heading("Simulation Statistics");
        ui.separator();

        ui.label(format!("Total Turns: {}", self.turn));
        ui.label(format!("Total Agents: {}", stats.total));
        ui.label(format!("Living Agents: {}", stats.alive));
        ui.label(format!("Dead Agents: {}", stats.dead));

        ui.separator();
        ui.label("By Category:");
        ui.label(format!("  NPCs: {}", stats.npcs));
        ui.label(format!("  Enemies: {}", stats.enemies));
        ui.label(format!("  Companions: {}", stats.companions));
        ui.label(format!("  Environmental: {}", stats.environmental));
        ui.label(format!("  System: {}", stats.system));

        ui.separator();
        ui.label(format!("Total Factions: {}", self.agents.factions.all().count()));
        ui.label(format!("Communications: {}", self.comm_log.len()));
    }

    fn render_event_log(&mut self, ui: &mut egui::Ui) {
        for (msg, color) in &self.event_log {
            ui.label(egui::RichText::new(msg).color(*color));
        }
    }
}

impl eframe::App for MultiAgentApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Process turns based on speed
        if matches!(self.app_state, AppState::Running) {
            if self.last_update.elapsed().as_millis() >= self.speed as u128 {
                self.process_turn();
                self.last_update = Instant::now();
            }
            ctx.request_repaint();
        }

        // Top panel - toolbar
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            self.render_toolbar(ui);
        });

        // Bottom panel - event log
        egui::TopBottomPanel::bottom("events").show(ctx, |ui| {
            self.render_event_log(ui);
        });

        // Right panel - agent info
        egui::SidePanel::right("info")
            .min_width(250.0)
            .show(ctx, |ui| {
                self.render_side_panel(ui);
            });

        // Central panel - map
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_map(ui);
        });

        // Handle keyboard
        ctx.input(|i| {
            // Pause/Resume
            if i.key_pressed(egui::Key::Space) {
                self.app_state = match self.app_state {
                    AppState::Running => AppState::Paused,
                    AppState::Paused => AppState::Running,
                };
            }

            // Camera movement
            if i.key_down(egui::Key::ArrowUp) || i.key_down(egui::Key::W) {
                self.camera_y -= 1.0;
            }
            if i.key_down(egui::Key::ArrowDown) || i.key_down(egui::Key::S) {
                self.camera_y += 1.0;
            }
            if i.key_down(egui::Key::ArrowLeft) || i.key_down(egui::Key::A) {
                self.camera_x -= 1.0;
            }
            if i.key_down(egui::Key::ArrowRight) || i.key_down(egui::Key::D) {
                self.camera_x += 1.0;
            }

            // Clamp camera
            self.camera_x = self.camera_x.clamp(0.0, MAP_WIDTH as f32);
            self.camera_y = self.camera_y.clamp(0.0, MAP_HEIGHT as f32);
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 900.0])
            .with_title("ShadowCrypt Multi-Agent Simulation"),
        ..Default::default()
    };

    eframe::run_native(
        "ShadowCrypt Multi-Agent Simulation",
        options,
        Box::new(|cc| Ok(Box::new(MultiAgentApp::new(cc)))),
    )
}
