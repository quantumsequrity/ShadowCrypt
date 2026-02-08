//! ShadowCrypt Multi-Agent CLI
//!
//! Terminal-based visualization of 75+ agents running in parallel
//! and communicating with each other.

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color as CrosstermColor, Print, ResetColor, SetForegroundColor, SetBackgroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::prelude::*;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

use shadowcrypt_core::prelude::*;
use shadowcrypt_agents::prelude::*;

/// Simple RGB color struct for agent visualization
#[derive(Clone, Copy, Debug)]
struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const CYAN: Color = Color { r: 0, g: 255, b: 255 };
    pub const YELLOW: Color = Color { r: 255, g: 255, b: 0 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255 };
}

const VIEW_WIDTH: usize = 80;
const VIEW_HEIGHT: usize = 35;

/// Convert color to crossterm
fn to_crossterm_color(color: Color) -> CrosstermColor {
    CrosstermColor::Rgb { r: color.r, g: color.g, b: color.b }
}

/// Multi-agent game state
struct MultiAgentGame {
    /// Agent manager
    agents: AgentManager,
    /// Game map
    map: Map,
    /// Player position for camera
    camera_x: usize,
    camera_y: usize,
    /// Messages log
    messages: Vec<(String, Color)>,
    /// Turn count
    turn: u32,
    /// Selected agent for inspection
    selected_agent: Option<AgentId>,
    /// Show agent list panel
    show_agent_list: bool,
    /// Show communication log
    show_comms: bool,
    /// Communication log
    comm_log: Vec<String>,
    /// Paused
    paused: bool,
    /// Speed (ms per turn)
    speed: u64,
    /// RNG
    rng: StdRng,
}

impl MultiAgentGame {
    fn new() -> Self {
        let mut rng = StdRng::from_entropy();
        let mut map = Map::new();
        map.generate(&mut rng, 1);

        let mut agents = AgentManager::with_seed(rng.gen());

        // Spawn 75+ agents across the map
        agents.spawn_default_agents(MAP_WIDTH, MAP_HEIGHT);

        // Set camera to center
        let camera_x = MAP_WIDTH / 2;
        let camera_y = MAP_HEIGHT / 2;

        Self {
            agents,
            map,
            camera_x,
            camera_y,
            messages: vec![
                ("Welcome to ShadowCrypt Multi-Agent Simulation!".to_string(), Color::CYAN),
                ("Press SPACE to pause/resume, +/- to adjust speed".to_string(), Color::YELLOW),
                ("Arrow keys to move camera, TAB for agent list, C for comms".to_string(), Color::YELLOW),
            ],
            turn: 0,
            selected_agent: None,
            show_agent_list: false,
            show_comms: false,
            comm_log: Vec::new(),
            paused: false,
            speed: 200,
            rng,
        }
    }

    fn add_message(&mut self, msg: String, color: Color) {
        self.messages.push((msg, color));
        if self.messages.len() > 5 {
            self.messages.remove(0);
        }
    }

    fn add_comm(&mut self, msg: String) {
        self.comm_log.push(msg);
        if self.comm_log.len() > 20 {
            self.comm_log.remove(0);
        }
    }

    fn process_turn(&mut self) {
        if self.paused {
            return;
        }

        self.turn += 1;
        self.agents.process_turn();

        // Process communications
        let messages = self.agents.message_bus.recent_messages(5);
        for msg in messages.iter() {
            let from_name = self.agents.get(msg.from)
                .map(|a| a.name.clone())
                .unwrap_or_else(|| "Unknown".to_string());
            let to_name = self.agents.get(msg.to)
                .map(|a| a.name.clone())
                .unwrap_or_else(|| "Unknown".to_string());

            self.add_comm(format!("{} -> {}: {}", from_name, to_name, msg.content));
        }

        // Periodic messages
        if self.turn % 50 == 0 {
            let stats = self.agents.stats();
            self.add_message(
                format!("Turn {} - {} agents active ({} NPCs, {} enemies, {} companions)",
                    self.turn, stats.alive, stats.npcs, stats.enemies, stats.companions),
                Color::CYAN,
            );
        }

        // Random agent dialogue
        if self.turn % 10 == 0 {
            let agents: Vec<_> = self.agents.all()
                .filter(|a| a.is_alive() && a.kind.category() == AgentCategory::Npc)
                .collect();

            if !agents.is_empty() {
                let agent = agents[self.rng.gen_range(0..agents.len())];
                let dialogue = NpcBehaviors::random_dialogue(agent.kind, &mut self.rng);
                self.add_message(
                    format!("{}: \"{}\"", agent.name, dialogue),
                    Color::WHITE,
                );
            }
        }
    }

    fn move_camera(&mut self, dx: i32, dy: i32) {
        self.camera_x = ((self.camera_x as i32 + dx * 5).max(0) as usize).min(MAP_WIDTH - VIEW_WIDTH);
        self.camera_y = ((self.camera_y as i32 + dy * 5).max(0) as usize).min(MAP_HEIGHT - VIEW_HEIGHT);
    }

    fn select_agent_at(&mut self, x: usize, y: usize) {
        let map_x = self.camera_x + x;
        let map_y = self.camera_y + y;

        let agents = self.agents.at_position(map_x, map_y);
        if let Some(agent) = agents.first() {
            self.selected_agent = Some(agent.id);
            self.add_message(
                format!("Selected: {} ({}) - HP: {}/{}",
                    agent.name, agent.kind.name(), agent.hp, agent.stats.max_hp),
                Color::YELLOW,
            );
        }
    }
}

fn render(game: &MultiAgentGame) -> std::io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

    // Get terminal size
    let (term_width, term_height) = terminal::size()?;
    let view_width = (term_width as usize).min(VIEW_WIDTH);
    let view_height = (term_height as usize - 12).min(VIEW_HEIGHT);

    // Render header
    execute!(
        stdout,
        SetForegroundColor(CrosstermColor::Cyan),
        Print("=== SHADOWCRYPT MULTI-AGENT SIMULATION ==="),
        ResetColor,
    )?;

    execute!(stdout, MoveTo(50, 0))?;
    let status = if game.paused { "PAUSED" } else { "RUNNING" };
    let status_color = if game.paused { CrosstermColor::Yellow } else { CrosstermColor::Green };
    execute!(
        stdout,
        Print(format!("Turn: {} | Status: ", game.turn)),
        SetForegroundColor(status_color),
        Print(status),
        ResetColor,
        Print(format!(" | Speed: {}ms", game.speed)),
    )?;

    // Render map
    for y in 0..view_height {
        let map_y = game.camera_y + y;
        if map_y >= MAP_HEIGHT {
            break;
        }

        execute!(stdout, MoveTo(0, (y + 1) as u16))?;

        for x in 0..view_width {
            let map_x = game.camera_x + x;
            if map_x >= MAP_WIDTH {
                break;
            }

            // Check for agents at this position
            let agents_here = game.agents.at_position(map_x, map_y);
            if let Some(agent) = agents_here.first() {
                if agent.is_alive() && agent.visible {
                    let (r, g, b) = agent.color();
                    let color = CrosstermColor::Rgb { r, g, b };
                    let glyph = agent.glyph();

                    // Highlight selected agent
                    if game.selected_agent == Some(agent.id) {
                        execute!(
                            stdout,
                            SetBackgroundColor(CrosstermColor::DarkGrey),
                            SetForegroundColor(color),
                            Print(glyph),
                            ResetColor,
                        )?;
                    } else {
                        execute!(
                            stdout,
                            SetForegroundColor(color),
                            Print(glyph),
                            ResetColor,
                        )?;
                    }
                    continue;
                }
            }

            // Render tile
            let tile = game.map.tiles[map_y][map_x];
            let tile_color = match tile {
                Tile::Floor => CrosstermColor::DarkGrey,
                Tile::Wall => CrosstermColor::Grey,
                Tile::StairsDown | Tile::StairsUp => CrosstermColor::Yellow,
                Tile::Door | Tile::OpenDoor => CrosstermColor::DarkYellow,
                Tile::Water => CrosstermColor::Blue,
                Tile::Lava => CrosstermColor::Red,
                _ => CrosstermColor::DarkGrey,
            };

            execute!(
                stdout,
                SetForegroundColor(tile_color),
                Print(tile.glyph()),
                ResetColor,
            )?;
        }
    }

    // Render agent list panel
    if game.show_agent_list {
        let panel_x = view_width as u16 + 2;
        execute!(stdout, MoveTo(panel_x, 1))?;
        execute!(
            stdout,
            SetForegroundColor(CrosstermColor::Cyan),
            Print("=== AGENTS ==="),
            ResetColor,
        )?;

        let stats = game.agents.stats();
        execute!(stdout, MoveTo(panel_x, 2))?;
        execute!(stdout, Print(format!("Total: {}/{}", stats.alive, stats.total)))?;

        let mut row = 3u16;
        for category in [AgentCategory::Npc, AgentCategory::Enemy, AgentCategory::Companion] {
            let agents: Vec<_> = game.agents.by_category(category);
            let living: Vec<_> = agents.iter().filter(|a| a.is_alive()).collect();

            if !living.is_empty() {
                execute!(stdout, MoveTo(panel_x, row))?;
                let cat_name = match category {
                    AgentCategory::Npc => "NPCs",
                    AgentCategory::Enemy => "Enemies",
                    AgentCategory::Companion => "Companions",
                    _ => "Other",
                };
                execute!(
                    stdout,
                    SetForegroundColor(CrosstermColor::Yellow),
                    Print(format!("-- {} ({}) --", cat_name, living.len())),
                    ResetColor,
                )?;
                row += 1;

                for agent in living.iter().take(8) {
                    execute!(stdout, MoveTo(panel_x, row))?;
                    let (r, g, b) = agent.color();
                    execute!(
                        stdout,
                        SetForegroundColor(CrosstermColor::Rgb { r, g, b }),
                        Print(format!("{} {} HP:{}/{}", agent.glyph(), agent.name, agent.hp, agent.stats.max_hp)),
                        ResetColor,
                    )?;
                    row += 1;
                    if row > view_height as u16 {
                        break;
                    }
                }
            }
        }
    }

    // Render communication log
    if game.show_comms {
        let comm_y = view_height as u16 + 2;
        execute!(stdout, MoveTo(0, comm_y))?;
        execute!(
            stdout,
            SetForegroundColor(CrosstermColor::Cyan),
            Print("=== COMMUNICATIONS ==="),
            ResetColor,
        )?;

        for (i, msg) in game.comm_log.iter().rev().take(5).enumerate() {
            execute!(stdout, MoveTo(0, comm_y + 1 + i as u16))?;
            execute!(
                stdout,
                SetForegroundColor(CrosstermColor::DarkGrey),
                Print(msg),
                ResetColor,
            )?;
        }
    }

    // Render messages
    let msg_y = (view_height + 8) as u16;
    for (i, (msg, color)) in game.messages.iter().enumerate() {
        execute!(stdout, MoveTo(0, msg_y + i as u16))?;
        execute!(
            stdout,
            SetForegroundColor(to_crossterm_color(*color)),
            Print(msg),
            ResetColor,
        )?;
    }

    // Render selected agent info
    if let Some(id) = game.selected_agent {
        if let Some(agent) = game.agents.get(id) {
            let info_y = msg_y - 3;
            execute!(stdout, MoveTo(0, info_y))?;
            execute!(
                stdout,
                SetForegroundColor(CrosstermColor::Yellow),
                Print(format!("Selected: {} | ", agent.name)),
                ResetColor,
                Print(format!("Type: {} | State: {:?} | Pos: ({}, {})",
                    agent.kind.name(), agent.state, agent.x, agent.y)),
            )?;

            if let Some(faction_id) = agent.faction {
                if let Some(faction) = game.agents.factions.get(faction_id) {
                    execute!(stdout, MoveTo(0, info_y + 1))?;
                    execute!(
                        stdout,
                        Print(format!("Faction: {} | Goals: {} | Personality: {:?}",
                            faction.name, agent.goals.len(), agent.personality.mood)),
                    )?;
                }
            }
        }
    }

    // Render help
    let help_y = (term_height - 1) as u16;
    execute!(stdout, MoveTo(0, help_y))?;
    execute!(
        stdout,
        SetForegroundColor(CrosstermColor::DarkGrey),
        Print("SPACE:Pause | +/-:Speed | Arrows:Camera | TAB:Agents | C:Comms | Click:Select | Q:Quit"),
        ResetColor,
    )?;

    stdout.flush()?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();

    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, Hide)?;

    let mut game = MultiAgentGame::new();
    let mut last_update = Instant::now();

    // Main loop
    loop {
        render(&game)?;

        // Process turn based on speed
        if last_update.elapsed().as_millis() >= game.speed as u128 {
            game.process_turn();
            last_update = Instant::now();
        }

        // Handle input
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') | KeyCode::Esc => break,

                    // Pause/resume
                    KeyCode::Char(' ') => {
                        game.paused = !game.paused;
                        let msg = if game.paused { "Simulation PAUSED" } else { "Simulation RESUMED" };
                        game.add_message(msg.to_string(), Color::YELLOW);
                    }

                    // Speed control
                    KeyCode::Char('+') | KeyCode::Char('=') => {
                        game.speed = (game.speed.saturating_sub(50)).max(50);
                        game.add_message(format!("Speed: {}ms per turn", game.speed), Color::CYAN);
                    }
                    KeyCode::Char('-') => {
                        game.speed = (game.speed + 50).min(1000);
                        game.add_message(format!("Speed: {}ms per turn", game.speed), Color::CYAN);
                    }

                    // Camera movement
                    KeyCode::Up => game.move_camera(0, -1),
                    KeyCode::Down => game.move_camera(0, 1),
                    KeyCode::Left => game.move_camera(-1, 0),
                    KeyCode::Right => game.move_camera(1, 0),

                    // Panels
                    KeyCode::Tab => game.show_agent_list = !game.show_agent_list,
                    KeyCode::Char('c') => game.show_comms = !game.show_comms,

                    // Manual turn advance when paused
                    KeyCode::Char('n') if game.paused => {
                        game.paused = false;
                        game.process_turn();
                        game.paused = true;
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
    let stats = game.agents.stats();
    println!("\n=== SIMULATION ENDED ===");
    println!("Total turns: {}", game.turn);
    println!("Agents: {} total ({} alive, {} dead)", stats.total, stats.alive, stats.dead);
    println!("  NPCs: {}", stats.npcs);
    println!("  Enemies: {}", stats.enemies);
    println!("  Companions: {}", stats.companions);
    println!("  Environmental: {}", stats.environmental);
    println!("  System: {}", stats.system);

    Ok(())
}
