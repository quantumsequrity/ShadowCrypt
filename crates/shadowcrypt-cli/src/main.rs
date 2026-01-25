//! ShadowCrypt CLI - Terminal-based roguelike dungeon crawler
//!
//! This is the terminal/CLI version of the ShadowCrypt roguelike game.

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Color as CrosstermColor, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::prelude::*;
use std::io::{stdout, Write};
use std::time::Duration;

use shadowcrypt_core::prelude::*;
use shadowcrypt_core::ai::{AIAction, AutoPlayAI};
use shadowcrypt_core::ui::{Color, tile_color, enemy_color, rarity_color};

/// Convert our color type to crossterm color
fn to_crossterm_color(color: Color) -> CrosstermColor {
    CrosstermColor::Rgb { r: color.r, g: color.g, b: color.b }
}

/// Renders the game state to the terminal
fn render(state: &GameState) -> std::io::Result<()> {
    let mut stdout = stdout();

    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

    // Get terminal size
    let (term_width, term_height) = terminal::size()?;
    let view_width = (term_width as usize).min(MAP_WIDTH);
    let view_height = (term_height as usize - 8).min(MAP_HEIGHT); // Leave room for UI

    // Calculate viewport centered on player
    let half_width = view_width / 2;
    let half_height = view_height / 2;
    let view_x = state.player.x.saturating_sub(half_width);
    let view_y = state.player.y.saturating_sub(half_height);

    // Render map
    for y in 0..view_height {
        let map_y = view_y + y;
        if map_y >= MAP_HEIGHT {
            break;
        }

        execute!(stdout, MoveTo(0, y as u16))?;

        for x in 0..view_width {
            let map_x = view_x + x;
            if map_x >= MAP_WIDTH {
                break;
            }

            // Check for player
            if map_x == state.player.x && map_y == state.player.y {
                execute!(
                    stdout,
                    SetForegroundColor(CrosstermColor::Yellow),
                    Print('@'),
                    ResetColor
                )?;
                continue;
            }

            // Check for visible enemies
            if state.map.visible[map_y][map_x] {
                if let Some(enemy) = state.enemies.iter().find(|e| e.is_alive() && e.x == map_x && e.y == map_y) {
                    let color = to_crossterm_color(enemy_color(enemy.kind));
                    execute!(
                        stdout,
                        SetForegroundColor(color),
                        Print(enemy.kind.glyph()),
                        ResetColor
                    )?;
                    continue;
                }

                // Check for items
                if let Some(item) = state.items.iter().find(|i| i.x == map_x && i.y == map_y) {
                    let color = to_crossterm_color(rarity_color(item.rarity));
                    execute!(
                        stdout,
                        SetForegroundColor(color),
                        Print(item.kind.glyph()),
                        ResetColor
                    )?;
                    continue;
                }
            }

            // Render tile
            let tile = state.map.tiles[map_y][map_x];
            if state.map.visible[map_y][map_x] {
                let color = to_crossterm_color(tile_color(tile));
                execute!(
                    stdout,
                    SetForegroundColor(color),
                    Print(tile.glyph()),
                    ResetColor
                )?;
            } else if state.map.explored[map_y][map_x] {
                // Darker for explored but not visible
                execute!(
                    stdout,
                    SetForegroundColor(CrosstermColor::DarkGrey),
                    Print(tile.glyph()),
                    ResetColor
                )?;
            } else {
                execute!(stdout, Print(' '))?;
            }
        }
    }

    // Render status bar
    let status_y = view_height as u16 + 1;
    execute!(stdout, MoveTo(0, status_y))?;

    // HP bar
    let hp_ratio = state.player.hp as f32 / state.player.total_max_hp() as f32;
    let hp_color = if hp_ratio > 0.6 {
        CrosstermColor::Green
    } else if hp_ratio > 0.3 {
        CrosstermColor::Yellow
    } else {
        CrosstermColor::Red
    };

    execute!(
        stdout,
        SetForegroundColor(hp_color),
        Print(format!("HP: {}/{} ", state.player.hp, state.player.total_max_hp())),
        ResetColor,
        SetForegroundColor(CrosstermColor::Blue),
        Print(format!("MP: {}/{} ", state.player.mana, state.player.total_max_mana())),
        ResetColor,
        SetForegroundColor(CrosstermColor::Yellow),
        Print(format!("Gold: {} ", state.player.gold)),
        ResetColor,
        Print(format!("Lvl: {} XP: {}/{} ", state.player.level, state.player.xp, state.player.xp_to_level)),
        Print(format!("Floor: {} ", state.dungeon_level)),
    )?;

    // Render skill info
    execute!(stdout, MoveTo(0, status_y + 1))?;
    if let Some(skill) = state.player.current_skill() {
        execute!(
            stdout,
            SetForegroundColor(CrosstermColor::Cyan),
            Print(format!("[Space] {} ({}mp) ", skill.name(), skill.mana_cost())),
            ResetColor,
        )?;
    }

    // Render status effects
    let mut effect_str = String::new();
    for (effect, _duration) in &state.player.status_effects {
        effect_str.push_str(&format!("{} ", effect.name()));
    }
    if !effect_str.is_empty() {
        execute!(
            stdout,
            SetForegroundColor(CrosstermColor::Magenta),
            Print(effect_str),
            ResetColor,
        )?;
    }

    // Render messages
    for (i, (msg, color)) in state.messages.iter().enumerate() {
        execute!(stdout, MoveTo(0, status_y + 3 + i as u16))?;
        let ct_color = to_crossterm_color(*color);
        execute!(
            stdout,
            SetForegroundColor(ct_color),
            Print(msg),
            ResetColor,
        )?;
    }

    // Game over / victory messages
    if state.game_over {
        let center_x = view_width / 2 - 10;
        let center_y = view_height / 2;
        execute!(
            stdout,
            MoveTo(center_x as u16, center_y as u16),
            SetBackgroundColor(CrosstermColor::DarkRed),
            SetForegroundColor(CrosstermColor::White),
            Print("  GAME OVER - Press any key  "),
            ResetColor,
        )?;
    } else if state.victory {
        let center_x = view_width / 2 - 15;
        let center_y = view_height / 2;
        execute!(
            stdout,
            MoveTo(center_x as u16, center_y as u16),
            SetBackgroundColor(CrosstermColor::DarkGreen),
            SetForegroundColor(CrosstermColor::White),
            Print("  VICTORY! You conquered ShadowCrypt!  "),
            ResetColor,
        )?;
    }

    stdout.flush()?;
    Ok(())
}

/// Renders the class selection screen
fn render_class_select() -> std::io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

    execute!(
        stdout,
        SetForegroundColor(CrosstermColor::Cyan),
        Print("=== SHADOWCRYPT ===\n\n"),
        ResetColor,
        Print("Choose your class:\n\n"),
    )?;

    for (i, class) in CharacterClass::all().iter().enumerate() {
        let (hp, atk, def, mana, spd) = class.base_stats();
        execute!(
            stdout,
            SetForegroundColor(CrosstermColor::Yellow),
            Print(format!("[{}] ", i + 1)),
            SetForegroundColor(CrosstermColor::White),
            Print(format!("{:<12}", class.name())),
            SetForegroundColor(CrosstermColor::Grey),
            Print(format!(" HP:{:<3} ATK:{:<2} DEF:{:<2} MP:{:<3} SPD:{:<2}\n", hp, atk, def, mana, spd)),
            SetForegroundColor(CrosstermColor::DarkGrey),
            Print(format!("              {}\n\n", class.special_ability())),
            ResetColor,
        )?;
    }

    execute!(
        stdout,
        Print("\nPress 1-6 to select, Q to quit\n"),
    )?;

    stdout.flush()?;
    Ok(())
}

/// Renders the inventory screen
fn render_inventory(state: &GameState) -> std::io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

    execute!(
        stdout,
        SetForegroundColor(CrosstermColor::Cyan),
        Print("=== INVENTORY ===\n\n"),
        ResetColor,
    )?;

    // Equipment
    execute!(stdout, Print("Equipped:\n"))?;
    for slot in [EquipSlot::Weapon, EquipSlot::Shield, EquipSlot::Helmet,
                 EquipSlot::Armor, EquipSlot::Gloves, EquipSlot::Boots,
                 EquipSlot::Ring1, EquipSlot::Ring2, EquipSlot::Amulet] {
        let slot_name = slot.name();
        if let Some(item) = state.player.equipment.get(&slot) {
            let color = to_crossterm_color(rarity_color(item.rarity));
            execute!(
                stdout,
                Print(format!("  {:<12} ", slot_name)),
                SetForegroundColor(color),
                Print(format!("{}\n", item.display_name())),
                ResetColor,
            )?;
        } else {
            execute!(
                stdout,
                SetForegroundColor(CrosstermColor::DarkGrey),
                Print(format!("  {:<12} (empty)\n", slot_name)),
                ResetColor,
            )?;
        }
    }

    execute!(stdout, Print("\nInventory:\n"))?;
    if state.player.inventory.is_empty() {
        execute!(
            stdout,
            SetForegroundColor(CrosstermColor::DarkGrey),
            Print("  (empty)\n"),
            ResetColor,
        )?;
    } else {
        for (i, item) in state.player.inventory.iter().enumerate().take(10) {
            let key = if i == 9 { '0' } else { char::from_digit((i + 1) as u32, 10).unwrap() };
            let color = to_crossterm_color(rarity_color(item.rarity));
            execute!(
                stdout,
                Print(format!("  [{}] ", key)),
                SetForegroundColor(color),
                Print(format!("{}\n", item.display_name())),
                ResetColor,
            )?;
        }
    }

    execute!(stdout, Print("\nPress a number to use/equip, I or ESC to close\n"))?;
    stdout.flush()?;
    Ok(())
}

/// Renders the help screen
fn render_help() -> std::io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

    execute!(
        stdout,
        SetForegroundColor(CrosstermColor::Cyan),
        Print("=== HELP ===\n\n"),
        ResetColor,
        Print("Movement:\n"),
        Print("  Arrow keys, WASD, or HJKL - Move\n"),
        Print("  YUBN - Diagonal movement\n"),
        Print("\n"),
        Print("Actions:\n"),
        Print("  Space - Use selected skill\n"),
        Print("  Tab   - Cycle skills\n"),
        Print("  >/.   - Descend stairs\n"),
        Print("  <     - Ascend stairs\n"),
        Print("  I     - Open inventory\n"),
        Print("  1-0   - Quick use inventory item\n"),
        Print("  ?     - This help screen\n"),
        Print("  Q/ESC - Quit game\n"),
        Print("\n"),
        Print("Goal:\n"),
        Print("  Descend to level 30 and defeat the Demon King!\n"),
        Print("\nPress any key to close\n"),
    )?;

    stdout.flush()?;
    Ok(())
}

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

    // Class selection
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
    let auto_ai = AutoPlayAI::new();

    // Game loop
    loop {
        if state.show_inventory {
            render_inventory(&state)?;
        } else if state.show_help {
            render_help()?;
        } else {
            render(&state)?;
        }

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

            let action = auto_ai.decide(
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

            match action {
                AIAction::Move(dx, dy) => state.move_player(dx, dy),
                AIAction::UseSkill => state.use_skill(),
                AIAction::UseItem(idx) => state.use_item(idx),
                AIAction::Descend => state.descend(),
                AIAction::Ascend => state.ascend(),
                AIAction::Wait => state.end_turn(),
                AIAction::Attack(_, _) => {} // Handled by move
            }

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
                        KeyCode::Char('?') | KeyCode::Esc | _ => state.show_help = false,
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
