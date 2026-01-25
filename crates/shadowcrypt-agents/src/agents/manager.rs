//! Agent manager - handles all agents in the game

use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use rand::prelude::*;

use super::{Agent, AgentId, AgentKind, AgentCategory, AgentState, AgentAction};
use crate::communication::{MessageBus, Message, Event, EventKind};
use crate::factions::{FactionId, FactionManager};
use crate::parallel::AgentScheduler;

/// Manages all agents in the game
pub struct AgentManager {
    /// All agents by ID
    agents: HashMap<AgentId, Agent>,
    /// Agents by category for quick lookup
    by_category: HashMap<AgentCategory, Vec<AgentId>>,
    /// Agents by position for spatial queries
    by_position: HashMap<(usize, usize), Vec<AgentId>>,
    /// Message bus for inter-agent communication
    pub message_bus: MessageBus,
    /// Faction manager
    pub factions: FactionManager,
    /// Scheduler for parallel execution
    pub scheduler: AgentScheduler,
    /// Random number generator
    rng: StdRng,
    /// Next unique ID counter
    next_id: u128,
    /// Turn counter
    turn: u32,
}

impl AgentManager {
    /// Creates a new agent manager
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
            by_category: HashMap::new(),
            by_position: HashMap::new(),
            message_bus: MessageBus::new(),
            factions: FactionManager::new(),
            scheduler: AgentScheduler::new(),
            rng: StdRng::from_entropy(),
            next_id: 1,
            turn: 0,
        }
    }

    /// Creates a new agent manager with a seed
    pub fn with_seed(seed: u64) -> Self {
        Self {
            agents: HashMap::new(),
            by_category: HashMap::new(),
            by_position: HashMap::new(),
            message_bus: MessageBus::new(),
            factions: FactionManager::new(),
            scheduler: AgentScheduler::new(),
            rng: StdRng::seed_from_u64(seed),
            next_id: 1,
            turn: 0,
        }
    }

    /// Spawns a new agent
    pub fn spawn(&mut self, kind: AgentKind, name: String, x: usize, y: usize) -> AgentId {
        let id = AgentId::from_u128(self.next_id);
        self.next_id += 1;

        let mut agent = Agent::new(kind, name, x, y).with_id(id);

        // Assign to default faction
        if let Some(faction) = self.factions.default_faction_for(kind) {
            agent.faction = Some(faction);
        }

        // Register in indices
        self.by_category
            .entry(kind.category())
            .or_insert_with(Vec::new)
            .push(id);
        self.by_position
            .entry((x, y))
            .or_insert_with(Vec::new)
            .push(id);

        self.agents.insert(id, agent);

        // Broadcast spawn event
        self.message_bus.broadcast(Event::new(
            EventKind::AgentSpawned { agent_id: id, kind },
        ));

        id
    }

    /// Spawns multiple agents of mixed types
    pub fn spawn_population(&mut self, count: usize, area: (usize, usize, usize, usize)) {
        let (min_x, min_y, max_x, max_y) = area;
        let kinds = AgentKind::all();

        for i in 0..count {
            let kind = kinds[i % kinds.len()];
            let x = self.rng.gen_range(min_x..max_x);
            let y = self.rng.gen_range(min_y..max_y);
            let name = format!("{} #{}", kind.name(), i + 1);
            self.spawn(kind, name, x, y);
        }
    }

    /// Spawns the default set of 75 agents
    pub fn spawn_default_agents(&mut self, width: usize, height: usize) {
        // System agents (5)
        self.spawn(AgentKind::QuestGiver, "Quest Master".to_string(), width / 2, height / 2);
        self.spawn(AgentKind::Narrator, "The Narrator".to_string(), 0, 0);
        self.spawn(AgentKind::TutorialGuide, "Tutorial Guide".to_string(), 1, 1);
        self.spawn(AgentKind::AchievementTracker, "Achievement Keeper".to_string(), 0, 0);
        self.spawn(AgentKind::DifficultyAdjuster, "Balance Keeper".to_string(), 0, 0);

        // Environmental agents (10)
        self.spawn(AgentKind::WeatherController, "Weather Spirit".to_string(), 0, 0);
        self.spawn(AgentKind::DayNightCycle, "Time Keeper".to_string(), 0, 0);
        self.spawn(AgentKind::SeasonManager, "Season Spirit".to_string(), 0, 0);
        self.spawn(AgentKind::WildlifeSpawner, "Nature Spirit".to_string(), 0, 0);
        self.spawn(AgentKind::VegetationManager, "Plant Spirit".to_string(), 0, 0);
        self.spawn(AgentKind::EventSpawner, "Fate Weaver".to_string(), 0, 0);
        self.spawn(AgentKind::AmbientSound, "Echo Spirit".to_string(), 0, 0);
        self.spawn(AgentKind::FogController, "Mist Spirit".to_string(), 0, 0);
        self.spawn(AgentKind::Earthquake, "Earth Tremor".to_string(), 0, 0);
        self.spawn(AgentKind::Storm, "Storm Spirit".to_string(), 0, 0);

        // NPCs (25) - spawn in safe areas
        let npc_kinds = vec![
            AgentKind::VillageElder, AgentKind::Blacksmith, AgentKind::Alchemist,
            AgentKind::Merchant, AgentKind::Innkeeper, AgentKind::Farmer,
            AgentKind::Guard, AgentKind::Captain, AgentKind::Priest,
            AgentKind::Scholar, AgentKind::Librarian, AgentKind::Bard,
            AgentKind::Thief, AgentKind::Beggar, AgentKind::Noble,
            AgentKind::Child, AgentKind::Hermit, AgentKind::Miner,
            AgentKind::Hunter, AgentKind::Fisher, AgentKind::Baker,
            AgentKind::Healer, AgentKind::Wizard, AgentKind::Apprentice,
            AgentKind::Traveler,
        ];

        for (i, kind) in npc_kinds.iter().enumerate() {
            let x = 5 + (i % 10) * 3;
            let y = 5 + (i / 10) * 3;
            let name = format!("{}", kind.name());
            self.spawn(*kind, name, x, y);
        }

        // Enemies (20) - spawn deeper in dungeon
        let enemy_kinds = vec![
            AgentKind::GoblinScout, AgentKind::GoblinShaman, AgentKind::OrcWarrior,
            AgentKind::OrcChieftain, AgentKind::SkeletonSoldier, AgentKind::SkeletonMage,
            AgentKind::ZombieHorde, AgentKind::VampireLord, AgentKind::Werewolf,
            AgentKind::BanditLeader, AgentKind::Assassin, AgentKind::DarkKnight,
            AgentKind::EnemyNecromancer, AgentKind::DemonSoldier, AgentKind::DemonLord,
            AgentKind::DragonWhelp, AgentKind::ElderDragon, AgentKind::LichKing,
            AgentKind::ChaosBeast, AgentKind::ShadowAssassin,
        ];

        for (i, kind) in enemy_kinds.iter().enumerate() {
            let x = width / 2 + self.rng.gen_range(0..width / 3);
            let y = height / 2 + self.rng.gen_range(0..height / 3);
            let name = format!("{}", kind.name());
            self.spawn(*kind, name, x, y);
        }

        // Companions (15) - scattered
        let companion_kinds = vec![
            AgentKind::WolfCompanion, AgentKind::BearCompanion, AgentKind::HawkCompanion,
            AgentKind::SkeletonMinion, AgentKind::FireElemental, AgentKind::IceElemental,
            AgentKind::EarthElemental, AgentKind::SpiritGuide, AgentKind::Mercenary,
            AgentKind::Squire, AgentKind::Familiar, AgentKind::Golem,
            AgentKind::FairyCompanion, AgentKind::ShadowClone, AgentKind::GuardianAngel,
        ];

        for kind in companion_kinds {
            let x = self.rng.gen_range(10..width - 10);
            let y = self.rng.gen_range(10..height - 10);
            let name = format!("{}", kind.name());
            self.spawn(kind, name, x, y);
        }
    }

    /// Gets an agent by ID
    pub fn get(&self, id: AgentId) -> Option<&Agent> {
        self.agents.get(&id)
    }

    /// Gets a mutable agent by ID
    pub fn get_mut(&mut self, id: AgentId) -> Option<&mut Agent> {
        self.agents.get_mut(&id)
    }

    /// Gets all agents
    pub fn all(&self) -> impl Iterator<Item = &Agent> {
        self.agents.values()
    }

    /// Gets all agents mutably
    pub fn all_mut(&mut self) -> impl Iterator<Item = &mut Agent> {
        self.agents.values_mut()
    }

    /// Gets agents by category
    pub fn by_category(&self, category: AgentCategory) -> Vec<&Agent> {
        self.by_category
            .get(&category)
            .map(|ids| ids.iter().filter_map(|id| self.agents.get(id)).collect())
            .unwrap_or_default()
    }

    /// Gets agents at a position
    pub fn at_position(&self, x: usize, y: usize) -> Vec<&Agent> {
        self.by_position
            .get(&(x, y))
            .map(|ids| ids.iter().filter_map(|id| self.agents.get(id)).collect())
            .unwrap_or_default()
    }

    /// Gets agents within a radius
    pub fn in_radius(&self, x: usize, y: usize, radius: usize) -> Vec<&Agent> {
        self.agents
            .values()
            .filter(|a| a.manhattan_distance_to(x, y) <= radius)
            .collect()
    }

    /// Gets visible agents for a given position and range
    pub fn visible_from(&self, x: usize, y: usize, range: usize) -> Vec<&Agent> {
        self.agents
            .values()
            .filter(|a| a.visible && a.manhattan_distance_to(x, y) <= range)
            .collect()
    }

    /// Gets all living agents
    pub fn living(&self) -> impl Iterator<Item = &Agent> {
        self.agents.values().filter(|a| a.is_alive())
    }

    /// Gets all dead agents
    pub fn dead(&self) -> impl Iterator<Item = &Agent> {
        self.agents.values().filter(|a| !a.is_alive())
    }

    /// Removes dead agents
    pub fn cleanup_dead(&mut self) {
        let dead_ids: Vec<_> = self.agents
            .iter()
            .filter(|(_, a)| !a.is_alive())
            .map(|(id, _)| *id)
            .collect();

        for id in dead_ids {
            self.remove(id);
        }
    }

    /// Removes an agent
    pub fn remove(&mut self, id: AgentId) {
        if let Some(agent) = self.agents.remove(&id) {
            // Remove from category index
            if let Some(ids) = self.by_category.get_mut(&agent.kind.category()) {
                ids.retain(|&i| i != id);
            }

            // Remove from position index
            if let Some(ids) = self.by_position.get_mut(&(agent.x, agent.y)) {
                ids.retain(|&i| i != id);
            }

            // Broadcast death event
            self.message_bus.broadcast(Event::new(
                EventKind::AgentDied { agent_id: id },
            ));
        }
    }

    /// Updates an agent's position in the index
    pub fn update_position(&mut self, id: AgentId, old_x: usize, old_y: usize, new_x: usize, new_y: usize) {
        // Remove from old position
        if let Some(ids) = self.by_position.get_mut(&(old_x, old_y)) {
            ids.retain(|&i| i != id);
        }

        // Add to new position
        self.by_position
            .entry((new_x, new_y))
            .or_insert_with(Vec::new)
            .push(id);
    }

    /// Gets agent count
    pub fn count(&self) -> usize {
        self.agents.len()
    }

    /// Gets living agent count
    pub fn living_count(&self) -> usize {
        self.agents.values().filter(|a| a.is_alive()).count()
    }

    /// Processes a single game turn for all agents
    pub fn process_turn(&mut self) {
        self.turn += 1;

        // Collect agent IDs for processing
        let agent_ids: Vec<_> = self.agents.keys().copied().collect();

        // Process each agent
        for id in agent_ids {
            if let Some(action) = self.decide_action(id) {
                self.execute_action(id, action);
            }
        }

        // Process messages
        self.message_bus.process_all();

        // Clean up dead agents periodically
        if self.turn % 10 == 0 {
            self.cleanup_dead();
        }
    }

    /// Decides an action for an agent based on their goals and state
    fn decide_action(&self, id: AgentId) -> Option<AgentAction> {
        let agent = self.agents.get(&id)?;

        if !agent.is_alive() || !agent.active {
            return None;
        }

        // Use behavior tree to decide action
        agent.behavior.decide(agent, self)
    }

    /// Executes an action for an agent
    fn execute_action(&mut self, id: AgentId, action: AgentAction) {
        let (old_x, old_y) = {
            let agent = match self.agents.get(&id) {
                Some(a) => a,
                None => return,
            };
            (agent.x, agent.y)
        };

        match action {
            AgentAction::Move { dx, dy } => {
                if let Some(agent) = self.agents.get_mut(&id) {
                    let new_x = (agent.x as i32 + dx).max(0) as usize;
                    let new_y = (agent.y as i32 + dy).max(0) as usize;
                    agent.move_to(new_x, new_y);
                    self.update_position(id, old_x, old_y, new_x, new_y);
                }
            }
            AgentAction::Attack { target_id } => {
                if let (Some(attacker), Some(target)) = (self.agents.get(&id), self.agents.get(&target_id)) {
                    let damage = attacker.stats.attack;
                    drop(attacker);
                    if let Some(target) = self.agents.get_mut(&target_id) {
                        let actual_damage = target.take_damage(damage);
                        self.message_bus.broadcast(Event::new(
                            EventKind::AgentAttacked {
                                attacker: id,
                                target: target_id,
                                damage: actual_damage,
                            },
                        ));
                    }
                }
            }
            AgentAction::Speak { message, target } => {
                if let Some(agent) = self.agents.get_mut(&id) {
                    agent.say(&message);
                    self.message_bus.broadcast(Event::new(
                        EventKind::AgentSpoke {
                            agent_id: id,
                            message: message.clone(),
                            target,
                        },
                    ));
                }
            }
            AgentAction::Wait => {
                if let Some(agent) = self.agents.get_mut(&id) {
                    agent.state = AgentState::Idle;
                }
            }
            AgentAction::Rest => {
                if let Some(agent) = self.agents.get_mut(&id) {
                    agent.state = AgentState::Resting;
                    agent.heal(agent.stats.max_hp / 20);
                }
            }
            AgentAction::Flee { from_x, from_y } => {
                if let Some(agent) = self.agents.get_mut(&id) {
                    agent.state = AgentState::Fleeing;
                    let dx = if agent.x > from_x { 1 } else if agent.x < from_x { -1 } else { 0 };
                    let dy = if agent.y > from_y { 1 } else if agent.y < from_y { -1 } else { 0 };
                    let new_x = (agent.x as i32 + dx).max(0) as usize;
                    let new_y = (agent.y as i32 + dy).max(0) as usize;
                    agent.move_to(new_x, new_y);
                    self.update_position(id, old_x, old_y, new_x, new_y);
                }
            }
            AgentAction::Follow { target_id } => {
                if let (Some(follower), Some(target)) = (self.agents.get(&id).cloned(), self.agents.get(&target_id)) {
                    let dx = if target.x > follower.x { 1 } else if target.x < follower.x { -1 } else { 0 };
                    let dy = if target.y > follower.y { 1 } else if target.y < follower.y { -1 } else { 0 };
                    if let Some(agent) = self.agents.get_mut(&id) {
                        agent.state = AgentState::Following;
                        let new_x = (agent.x as i32 + dx).max(0) as usize;
                        let new_y = (agent.y as i32 + dy).max(0) as usize;
                        agent.move_to(new_x, new_y);
                        self.update_position(id, old_x, old_y, new_x, new_y);
                    }
                }
            }
            AgentAction::Hide => {
                if let Some(agent) = self.agents.get_mut(&id) {
                    agent.state = AgentState::Hiding;
                    agent.visible = false;
                }
            }
            _ => {}
        }

        // Record last action
        if let Some(agent) = self.agents.get_mut(&id) {
            agent.last_action = Some(action);
            agent.turn = self.turn;
        }
    }

    /// Sends a message from one agent to another
    pub fn send_message(&mut self, from: AgentId, to: AgentId, content: String) {
        self.message_bus.send(Message::new(from, to, content));
    }

    /// Broadcasts a message from one agent to all nearby agents
    pub fn broadcast_local(&mut self, from: AgentId, content: String, radius: usize) {
        if let Some(sender) = self.agents.get(&from) {
            let x = sender.x;
            let y = sender.y;
            let nearby: Vec<_> = self.agents
                .iter()
                .filter(|(id, a)| **id != from && a.manhattan_distance_to(x, y) <= radius)
                .map(|(id, _)| *id)
                .collect();

            for to in nearby {
                self.message_bus.send(Message::new(from, to, content.clone()));
            }
        }
    }

    /// Gets statistics about agents
    pub fn stats(&self) -> AgentStats {
        let mut stats = AgentStats::default();
        stats.total = self.agents.len();
        stats.alive = self.agents.values().filter(|a| a.is_alive()).count();
        stats.dead = stats.total - stats.alive;

        for agent in self.agents.values() {
            match agent.kind.category() {
                AgentCategory::Npc => stats.npcs += 1,
                AgentCategory::Enemy => stats.enemies += 1,
                AgentCategory::Companion => stats.companions += 1,
                AgentCategory::Environmental => stats.environmental += 1,
                AgentCategory::System => stats.system += 1,
            }
        }

        stats
    }
}

impl Default for AgentManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about agents
#[derive(Clone, Debug, Default)]
pub struct AgentStats {
    pub total: usize,
    pub alive: usize,
    pub dead: usize,
    pub npcs: usize,
    pub enemies: usize,
    pub companions: usize,
    pub environmental: usize,
    pub system: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_agents() {
        let mut manager = AgentManager::new();
        let id = manager.spawn(AgentKind::Guard, "Test Guard".to_string(), 10, 10);
        assert!(manager.get(id).is_some());
        assert_eq!(manager.count(), 1);
    }

    #[test]
    fn test_agent_categories() {
        let mut manager = AgentManager::new();
        manager.spawn(AgentKind::Guard, "Guard".to_string(), 0, 0);
        manager.spawn(AgentKind::GoblinScout, "Goblin".to_string(), 5, 5);

        assert_eq!(manager.by_category(AgentCategory::Npc).len(), 1);
        assert_eq!(manager.by_category(AgentCategory::Enemy).len(), 1);
    }

    #[test]
    fn test_spawn_population() {
        let mut manager = AgentManager::with_seed(12345);
        manager.spawn_default_agents(100, 100);
        assert!(manager.count() >= 75);
    }
}
