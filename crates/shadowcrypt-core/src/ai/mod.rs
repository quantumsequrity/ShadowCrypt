//! AI system for the ShadowCrypt roguelike
//!
//! This module handles enemy AI behavior, pathfinding, and
//! the auto-play AI for demonstration purposes.

use rand::prelude::*;
use crate::constants::{MAP_WIDTH, MAP_HEIGHT, BOSS_LEVELS};
use crate::world::{Map, Tile};
use crate::combat::Enemy;

/// Actions that can be taken by the AI
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AIAction {
    /// Move in a direction (dx, dy)
    Move(i32, i32),
    /// Use the current skill
    UseSkill,
    /// Use an inventory item at index
    UseItem(usize),
    /// Descend stairs
    Descend,
    /// Ascend stairs
    Ascend,
    /// Wait in place
    Wait,
    /// Attack a specific target
    Attack(usize, usize),
}

/// AI behavior types for enemies
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AIBehavior {
    /// Moves toward the player when in sight
    Aggressive,
    /// Stays in place until the player is close
    Passive,
    /// Runs away when HP is low
    Cowardly,
    /// Patrols a route
    Patrol,
    /// Moves randomly
    Random,
    /// Supports other enemies (heals, buffs)
    Support,
}

/// Calculates the Manhattan distance between two points
pub fn manhattan_distance(x1: usize, y1: usize, x2: usize, y2: usize) -> i32 {
    (x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()
}

/// Calculates the Euclidean distance squared between two points
pub fn distance_squared(x1: usize, y1: usize, x2: usize, y2: usize) -> i32 {
    let dx = x1 as i32 - x2 as i32;
    let dy = y1 as i32 - y2 as i32;
    dx * dx + dy * dy
}

/// Simple pathfinding to move toward a target
pub fn move_toward(
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize,
    map: &Map,
    enemies: &[Enemy],
) -> Option<(i32, i32)> {
    let dx = (to_x as i32 - from_x as i32).signum();
    let dy = (to_y as i32 - from_y as i32).signum();

    // Check if we're already at target
    if dx == 0 && dy == 0 {
        return None;
    }

    // Check if any enemy is blocking the target position
    let is_blocked = |x: usize, y: usize| -> bool {
        enemies.iter().any(|e| e.is_alive() && e.x == x && e.y == y)
    };

    // Try diagonal first
    if dx != 0 && dy != 0 {
        let nx = (from_x as i32 + dx) as usize;
        let ny = (from_y as i32 + dy) as usize;
        if map.is_walkable(nx, ny) && !is_blocked(nx, ny) {
            return Some((dx, dy));
        }
    }

    // Try horizontal
    if dx != 0 {
        let nx = (from_x as i32 + dx) as usize;
        if map.is_walkable(nx, from_y) && !is_blocked(nx, from_y) {
            return Some((dx, 0));
        }
    }

    // Try vertical
    if dy != 0 {
        let ny = (from_y as i32 + dy) as usize;
        if map.is_walkable(from_x, ny) && !is_blocked(from_x, ny) {
            return Some((0, dy));
        }
    }

    None
}

/// Simple pathfinding to move away from a target
pub fn move_away(
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize,
    map: &Map,
    enemies: &[Enemy],
) -> Option<(i32, i32)> {
    let dx = (from_x as i32 - to_x as i32).signum();
    let dy = (from_y as i32 - to_y as i32).signum();

    if dx == 0 && dy == 0 {
        // Pick a random direction if on top of target
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut rng = thread_rng();
        for (d_x, d_y) in dirs.iter().choose_multiple(&mut rng, 4) {
            let nx = (from_x as i32 + d_x) as usize;
            let ny = (from_y as i32 + d_y) as usize;
            if map.is_walkable(nx, ny) {
                return Some((*d_x, *d_y));
            }
        }
        return None;
    }

    let is_blocked = |x: usize, y: usize| -> bool {
        enemies.iter().any(|e| e.is_alive() && e.x == x && e.y == y)
    };

    // Try moving directly away
    if dx != 0 && dy != 0 {
        let nx = (from_x as i32 + dx) as usize;
        let ny = (from_y as i32 + dy) as usize;
        if map.is_walkable(nx, ny) && !is_blocked(nx, ny) {
            return Some((dx, dy));
        }
    }

    if dx != 0 {
        let nx = (from_x as i32 + dx) as usize;
        if map.is_walkable(nx, from_y) && !is_blocked(nx, from_y) {
            return Some((dx, 0));
        }
    }

    if dy != 0 {
        let ny = (from_y as i32 + dy) as usize;
        if map.is_walkable(from_x, ny) && !is_blocked(from_x, ny) {
            return Some((0, dy));
        }
    }

    None
}

/// Enemy AI decision making
pub fn enemy_decide(
    enemy: &Enemy,
    player_x: usize,
    player_y: usize,
    player_visible: bool,
    map: &Map,
    enemies: &[Enemy],
) -> Option<(i32, i32)> {
    let dist = manhattan_distance(enemy.x, enemy.y, player_x, player_y);

    // If player is adjacent, attack (return direction to player)
    if dist == 1 {
        let dx = player_x as i32 - enemy.x as i32;
        let dy = player_y as i32 - enemy.y as i32;
        return Some((dx, dy));
    }

    // If player is visible, move toward them
    if player_visible {
        return move_toward(enemy.x, enemy.y, player_x, player_y, map, enemies);
    }

    // If we remember seeing the player, move toward last known position
    if let Some((last_x, last_y)) = enemy.last_seen_player {
        if enemy.x != last_x || enemy.y != last_y {
            return move_toward(enemy.x, enemy.y, last_x, last_y, map, enemies);
        }
    }

    // Random movement
    let mut rng = thread_rng();
    let dirs: Vec<(i32, i32)> = vec![
        (1, 0), (-1, 0), (0, 1), (0, -1),
        (1, 1), (-1, 1), (1, -1), (-1, -1),
    ];

    for (dx, dy) in dirs.iter().choose_multiple(&mut rng, 8) {
        let nx = (enemy.x as i32 + dx) as usize;
        let ny = (enemy.y as i32 + dy) as usize;
        if map.is_walkable(nx, ny) {
            // Don't move onto other enemies
            let blocked = enemies.iter().any(|e| e.is_alive() && e.x == nx && e.y == ny);
            if !blocked {
                return Some((*dx, *dy));
            }
        }
    }

    None
}

/// Auto-play AI for demonstration/testing
pub struct AutoPlayAI {
    /// Directions for exploring
    directions: Vec<(i32, i32)>,
}

impl AutoPlayAI {
    /// Creates a new auto-play AI
    pub fn new() -> Self {
        Self {
            directions: vec![
                (1, 0), (-1, 0), (0, 1), (0, -1),
                (1, 1), (-1, 1), (1, -1), (-1, -1),
            ],
        }
    }

    /// Decides the next action for the auto-play AI
    pub fn decide(
        &self,
        player_x: usize,
        player_y: usize,
        player_hp: i32,
        player_max_hp: i32,
        player_mana: i32,
        _player_can_use_skill: bool,
        map: &Map,
        enemies: &[Enemy],
        has_health_potion: Option<usize>,
        has_mana_potion: Option<usize>,
        dungeon_level: u32,
        boss_defeated: bool,
    ) -> AIAction {
        let px = player_x as i32;
        let py = player_y as i32;

        // Priority 1: Use health potion if low HP
        if player_hp < player_max_hp / 4 {
            if let Some(idx) = has_health_potion {
                return AIAction::UseItem(idx);
            }
        }

        // Priority 2: Use mana potion if very low mana
        if player_mana < 10 {
            if let Some(idx) = has_mana_potion {
                return AIAction::UseItem(idx);
            }
        }

        // Priority 3: Attack adjacent enemies
        for enemy in enemies.iter().filter(|e| e.is_alive()) {
            let dx = enemy.x as i32 - px;
            let dy = enemy.y as i32 - py;
            if dx.abs() <= 1 && dy.abs() <= 1 {
                return AIAction::Move(dx, dy);
            }
        }

        // Priority 4: Move toward visible enemies
        if let Some(target) = enemies
            .iter()
            .filter(|e| e.is_alive() && map.visible[e.y][e.x])
            .min_by_key(|e| distance_squared(e.x, e.y, player_x, player_y))
        {
            let dx = (target.x as i32 - px).signum();
            let dy = (target.y as i32 - py).signum();
            let nx = (px + dx) as usize;
            let ny = (py + dy) as usize;
            if map.is_walkable(nx, ny) {
                return AIAction::Move(dx, dy);
            }
            // Try horizontal or vertical if diagonal blocked
            if dx != 0 && map.is_walkable((px + dx) as usize, player_y) {
                return AIAction::Move(dx, 0);
            }
            if dy != 0 && map.is_walkable(player_x, (py + dy) as usize) {
                return AIAction::Move(0, dy);
            }
        }

        // Priority 5: Descend stairs if on them and boss defeated (or no boss)
        let current_tile = map.tiles[player_y][player_x];
        if current_tile == Tile::StairsDown && (boss_defeated || !BOSS_LEVELS.contains(&dungeon_level)) {
            return AIAction::Descend;
        }

        // Priority 6: Move towards stairs if visible and no enemies around
        let no_visible_enemies = !enemies.iter().any(|e| e.is_alive() && map.visible[e.y][e.x]);
        if no_visible_enemies {
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
                            if dx != 0 && map.is_walkable((px + dx) as usize, player_y) {
                                return AIAction::Move(dx, 0);
                            }
                            if dy != 0 && map.is_walkable(player_x, (py + dy) as usize) {
                                return AIAction::Move(0, dy);
                            }
                        }
                    }
                }
            }
        }

        // Priority 7: Explore unexplored areas
        let mut best_unexplored: Option<(usize, usize, i32)> = None;
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                if map.visible[y][x] && !map.explored[y][x] && map.is_walkable(x, y) {
                    let dist = manhattan_distance(x, y, player_x, player_y);
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

        // Priority 8: Random walk to explore
        let mut rng = thread_rng();
        let mut shuffled = self.directions.clone();
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

impl Default for AutoPlayAI {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(manhattan_distance(0, 0, 3, 4), 7);
        assert_eq!(manhattan_distance(5, 5, 5, 5), 0);
    }

    #[test]
    fn test_distance_squared() {
        assert_eq!(distance_squared(0, 0, 3, 4), 25);
        assert_eq!(distance_squared(5, 5, 5, 5), 0);
    }
}
