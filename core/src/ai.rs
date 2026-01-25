//! AI system: decision making for auto-play and enemies

use rand::prelude::*;
use serde::{Serialize, Deserialize};

use crate::items::ItemKind;
use crate::world::{Map, Tile, MAP_WIDTH, MAP_HEIGHT, BOSS_LEVELS};
use crate::entities::{Player, Enemy};

/// Actions the AI can take
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum AIAction {
    Move(i32, i32),
    UseSkill,
    UseItem(usize),
    Descend,
    Ascend,
    Wait,
}

/// AI decision making for auto-play mode
pub struct AIDecider;

impl AIDecider {
    /// Decide the best action for the player
    pub fn decide(
        player: &Player,
        enemies: &[Enemy],
        items: &[crate::items::Item],
        map: &Map,
        dungeon_level: u32,
        boss_defeated: bool,
    ) -> AIAction {
        let px = player.x as i32;
        let py = player.y as i32;

        // Priority 1: Use health potion if HP is critical (below 30%)
        if player.hp < player.total_max_hp() * 30 / 100 {
            for (i, item) in player.inventory.iter().enumerate() {
                if matches!(item.kind, ItemKind::HealthPotion | ItemKind::FullRestorePotion) {
                    return AIAction::UseItem(i);
                }
            }
        }

        // Priority 2: Eat food if starving
        if player.hunger < 15 {
            for (i, item) in player.inventory.iter().enumerate() {
                if item.kind.is_food() {
                    return AIAction::UseItem(i);
                }
            }
        }

        // Priority 3: Attack adjacent enemy
        let directions = [
            (0, -1), (0, 1), (-1, 0), (1, 0),
            (-1, -1), (1, -1), (-1, 1), (1, 1),
        ];

        for (dx, dy) in directions.iter() {
            let nx = (px + dx) as usize;
            let ny = (py + dy) as usize;
            if enemies.iter().any(|e| e.x == nx && e.y == ny && e.is_alive()) {
                return AIAction::Move(*dx, *dy);
            }
        }

        // Priority 4: Use skill on nearby enemies if we have mana
        if player.can_use_skill() {
            let skill_range = 5;
            let has_nearby_enemy = enemies.iter().any(|e| {
                let dx = (e.x as i32 - px).abs();
                let dy = (e.y as i32 - py).abs();
                e.is_alive() && dx <= skill_range && dy <= skill_range && map.visible[e.y][e.x]
            });
            if has_nearby_enemy {
                return AIAction::UseSkill;
            }
        }

        // Priority 5: Move towards visible enemy
        if let Some(target) = enemies.iter()
            .filter(|e| e.is_alive() && map.visible[e.y][e.x])
            .min_by_key(|e| {
                let dx = e.x as i32 - px;
                let dy = e.y as i32 - py;
                dx * dx + dy * dy
            })
        {
            let dx = (target.x as i32 - px).signum();
            let dy = (target.y as i32 - py).signum();
            let nx = (px + dx) as usize;
            let ny = (py + dy) as usize;
            if map.is_walkable(nx, ny) {
                return AIAction::Move(dx, dy);
            }
            // Try horizontal or vertical if diagonal blocked
            if dx != 0 && map.is_walkable((px + dx) as usize, py as usize) {
                return AIAction::Move(dx, 0);
            }
            if dy != 0 && map.is_walkable(px as usize, (py + dy) as usize) {
                return AIAction::Move(0, dy);
            }
        }

        // Priority 6: Descend stairs if on them and boss defeated (or no boss)
        let current_tile = map.tiles[player.y][player.x];
        if current_tile == Tile::StairsDown && (boss_defeated || !BOSS_LEVELS.contains(&dungeon_level)) {
            return AIAction::Descend;
        }

        // Priority 7: Move towards stairs if visible and no enemies around
        let no_visible_enemies = !enemies.iter().any(|e| e.is_alive() && map.visible[e.y][e.x]);
        if no_visible_enemies {
            // Find stairs
            for y in 0..MAP_HEIGHT {
                for x in 0..MAP_WIDTH {
                    if map.tiles[y][x] == Tile::StairsDown && map.explored[y][x] {
                        let dx = (x as i32 - px).signum();
                        let dy = (y as i32 - py).signum();
                        if dx != 0 || dy != 0 {
                            let nx = (px + dx) as usize;
                            let ny = (py + dy) as usize;
                            if map.is_walkable(nx, ny) {
                                return AIAction::Move(dx, dy);
                            }
                            if dx != 0 && map.is_walkable((px + dx) as usize, py as usize) {
                                return AIAction::Move(dx, 0);
                            }
                            if dy != 0 && map.is_walkable(px as usize, (py + dy) as usize) {
                                return AIAction::Move(0, dy);
                            }
                        }
                    }
                }
            }
        }

        // Priority 8: Explore unexplored areas
        let mut best_unexplored: Option<(usize, usize, i32)> = None;
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                if map.visible[y][x] && !map.explored[y][x] && map.is_walkable(x, y) {
                    let dist = (x as i32 - px).abs() + (y as i32 - py).abs();
                    if best_unexplored.is_none() || dist < best_unexplored.unwrap().2 {
                        best_unexplored = Some((x, y, dist));
                    }
                }
            }
        }

        if let Some((tx, ty, _)) = best_unexplored {
            let dx = (tx as i32 - px).signum();
            let dy = (ty as i32 - py).signum();
            let nx = (px + dx) as usize;
            let ny = (py + dy) as usize;
            if map.is_walkable(nx, ny) {
                return AIAction::Move(dx, dy);
            }
        }

        // Priority 9: Random walk to explore
        let mut rng = thread_rng();
        let mut shuffled = directions.to_vec();
        shuffled.shuffle(&mut rng);

        for (dx, dy) in shuffled {
            let nx = (px + dx) as usize;
            let ny = (py + dy) as usize;
            if map.is_walkable(nx, ny) {
                return AIAction::Move(dx, dy);
            }
        }

        AIAction::Wait
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classes::CharacterClass;

    #[test]
    fn test_ai_returns_valid_action() {
        let player = Player::new(50, 22, CharacterClass::Warrior);
        let mut map = Map::new();
        let mut rng = rand::thread_rng();
        map.generate(&mut rng, 1);

        let action = AIDecider::decide(&player, &[], &[], &map, 1, false);

        // Should return some valid action
        match action {
            AIAction::Move(_, _) | AIAction::Wait | AIAction::UseSkill |
            AIAction::UseItem(_) | AIAction::Descend | AIAction::Ascend => {}
        }
    }
}
