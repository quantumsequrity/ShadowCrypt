//! Enemy agent implementations

use super::{Agent, AgentId, AgentKind, AgentState};
use crate::behavior::{Goal, GoalKind, GoalPriority};
use rand::Rng;
use serde::{Serialize, Deserialize};

/// Enemy-specific behaviors and combat AI
pub struct EnemyBehaviors;

impl EnemyBehaviors {
    /// Creates default goals for an enemy
    pub fn default_goals(kind: AgentKind) -> Vec<Goal> {
        match kind {
            // Scouts patrol and alert others
            AgentKind::GoblinScout => vec![
                Goal::new(GoalKind::Patrol, GoalPriority::Medium),
                Goal::new(GoalKind::AlertAllies, GoalPriority::High),
                Goal::new(GoalKind::AttackEnemy, GoalPriority::Medium),
            ],
            // Shamans support allies
            AgentKind::GoblinShaman | AgentKind::SkeletonMage => vec![
                Goal::new(GoalKind::SupportAllies, GoalPriority::High),
                Goal::new(GoalKind::CastSpell, GoalPriority::Medium),
                Goal::new(GoalKind::Flee, GoalPriority::Critical),
            ],
            // Warriors are aggressive
            AgentKind::OrcWarrior | AgentKind::SkeletonSoldier | AgentKind::DemonSoldier => vec![
                Goal::new(GoalKind::AttackEnemy, GoalPriority::High),
                Goal::new(GoalKind::Charge, GoalPriority::Medium),
            ],
            // Leaders command and fight
            AgentKind::OrcChieftain | AgentKind::BanditLeader | AgentKind::DemonLord => vec![
                Goal::new(GoalKind::Command, GoalPriority::High),
                Goal::new(GoalKind::AttackEnemy, GoalPriority::Medium),
                Goal::new(GoalKind::Rally, GoalPriority::High),
            ],
            // Undead shamble forward
            AgentKind::ZombieHorde => vec![
                Goal::new(GoalKind::Pursue, GoalPriority::High),
                Goal::new(GoalKind::AttackEnemy, GoalPriority::High),
            ],
            // Vampires drain and flee
            AgentKind::VampireLord => vec![
                Goal::new(GoalKind::DrainLife, GoalPriority::High),
                Goal::new(GoalKind::AttackEnemy, GoalPriority::Medium),
                Goal::new(GoalKind::Flee, GoalPriority::Medium),
            ],
            // Assassins ambush
            AgentKind::Assassin | AgentKind::ShadowAssassin => vec![
                Goal::new(GoalKind::Ambush, GoalPriority::High),
                Goal::new(GoalKind::Hide, GoalPriority::Medium),
                Goal::new(GoalKind::AttackEnemy, GoalPriority::High),
            ],
            // Werewolves hunt
            AgentKind::Werewolf => vec![
                Goal::new(GoalKind::Hunt, GoalPriority::High),
                Goal::new(GoalKind::Pursue, GoalPriority::High),
            ],
            // Dragons are territorial
            AgentKind::DragonWhelp | AgentKind::ElderDragon => vec![
                Goal::new(GoalKind::DefendTerritory, GoalPriority::Critical),
                Goal::new(GoalKind::AttackEnemy, GoalPriority::High),
                Goal::new(GoalKind::CastSpell, GoalPriority::Medium),
            ],
            // Necromancers raise dead
            AgentKind::EnemyNecromancer | AgentKind::LichKing => vec![
                Goal::new(GoalKind::RaiseUndead, GoalPriority::High),
                Goal::new(GoalKind::CastSpell, GoalPriority::Medium),
                Goal::new(GoalKind::Flee, GoalPriority::High),
            ],
            // Dark knights are relentless
            AgentKind::DarkKnight => vec![
                Goal::new(GoalKind::Charge, GoalPriority::High),
                Goal::new(GoalKind::AttackEnemy, GoalPriority::High),
            ],
            // Chaos beasts are unpredictable
            AgentKind::ChaosBeast => vec![
                Goal::new(GoalKind::Rampage, GoalPriority::High),
                Goal::new(GoalKind::AttackEnemy, GoalPriority::Medium),
            ],
            _ => vec![
                Goal::new(GoalKind::AttackEnemy, GoalPriority::Medium),
            ],
        }
    }

    /// Gets taunt dialogue for an enemy
    pub fn taunt<R: Rng>(kind: AgentKind, rng: &mut R) -> String {
        let taunts = Self::taunts_for_kind(kind);
        taunts[rng.gen_range(0..taunts.len())].to_string()
    }

    /// Gets all taunts for an enemy kind
    fn taunts_for_kind(kind: AgentKind) -> &'static [&'static str] {
        match kind {
            AgentKind::GoblinScout | AgentKind::GoblinShaman => &[
                "Nasty human! We kills it!",
                "More shinies for us!",
                "Goblins everywhere! Hehehe!",
                "Die die die!",
            ],
            AgentKind::OrcWarrior | AgentKind::OrcChieftain => &[
                "WAAAGH!",
                "Puny human dies today!",
                "For the horde!",
                "Crush! Kill! Destroy!",
            ],
            AgentKind::SkeletonSoldier | AgentKind::SkeletonMage => &[
                "...",
                "*rattling bones*",
                "Join us in death...",
                "Eternal service awaits...",
            ],
            AgentKind::VampireLord => &[
                "Your blood smells... exquisite.",
                "Immortality has its privileges.",
                "Join my eternal court.",
                "I have lived for centuries. You are nothing.",
            ],
            AgentKind::Werewolf => &[
                "*HOWL*",
                "The hunt is on!",
                "Fresh meat!",
                "I can smell your fear!",
            ],
            AgentKind::DemonSoldier | AgentKind::DemonLord => &[
                "Your soul will burn!",
                "Hell awaits you!",
                "Embrace the darkness!",
                "Mortal fool!",
            ],
            AgentKind::ElderDragon => &[
                "Insects dare challenge me?",
                "I am older than your entire civilization!",
                "BURN!",
                "My treasure stays mine!",
            ],
            AgentKind::LichKing => &[
                "Death is only the beginning.",
                "My phylactery is beyond your reach.",
                "Bow before the master of death!",
                "Your ancestors serve me now.",
            ],
            _ => &[
                "Die!",
                "You shouldn't have come here!",
                "This is your end!",
                "Attack!",
            ],
        }
    }

    /// Gets death cry for an enemy
    pub fn death_cry(kind: AgentKind) -> &'static str {
        match kind {
            AgentKind::GoblinScout | AgentKind::GoblinShaman => "Eeek! *splat*",
            AgentKind::OrcWarrior | AgentKind::OrcChieftain => "No... the horde... *thud*",
            AgentKind::SkeletonSoldier | AgentKind::SkeletonMage => "*bones clatter*",
            AgentKind::VampireLord => "No! The eternal night... fades...",
            AgentKind::Werewolf => "*whimper* *thud*",
            AgentKind::DemonSoldier | AgentKind::DemonLord => "I return to the abyss...",
            AgentKind::ElderDragon => "Impossible... I am... eternal...",
            AgentKind::LichKing => "My phylactery... protects... me...",
            _ => "*death cry*",
        }
    }
}

/// Combat tactics for enemies
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CombatTactic {
    /// Rush in and attack
    Aggressive,
    /// Wait for opportunity
    Defensive,
    /// Keep distance, use ranged
    Ranged,
    /// Support allies
    Support,
    /// Hit and run
    Guerrilla,
    /// Surround target
    Flanking,
    /// Focus on weak targets
    Opportunistic,
    /// Unpredictable attacks
    Chaotic,
}

impl CombatTactic {
    /// Gets the default tactic for an enemy kind
    pub fn for_kind(kind: AgentKind) -> Self {
        match kind {
            AgentKind::GoblinScout => Self::Guerrilla,
            AgentKind::GoblinShaman | AgentKind::SkeletonMage => Self::Support,
            AgentKind::OrcWarrior | AgentKind::DarkKnight => Self::Aggressive,
            AgentKind::OrcChieftain | AgentKind::BanditLeader => Self::Flanking,
            AgentKind::SkeletonSoldier | AgentKind::ZombieHorde => Self::Aggressive,
            AgentKind::VampireLord => Self::Opportunistic,
            AgentKind::Werewolf => Self::Aggressive,
            AgentKind::Assassin | AgentKind::ShadowAssassin => Self::Opportunistic,
            AgentKind::EnemyNecromancer | AgentKind::LichKing => Self::Support,
            AgentKind::DemonSoldier => Self::Aggressive,
            AgentKind::DemonLord => Self::Flanking,
            AgentKind::DragonWhelp => Self::Ranged,
            AgentKind::ElderDragon => Self::Aggressive,
            AgentKind::ChaosBeast => Self::Chaotic,
            _ => Self::Aggressive,
        }
    }
}

/// Abilities that enemies can use
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnemyAbility {
    pub name: String,
    pub damage: i32,
    pub range: usize,
    pub cooldown: u32,
    pub current_cooldown: u32,
    pub effect: AbilityEffect,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AbilityEffect {
    None,
    Poison { duration: u32, damage_per_turn: i32 },
    Burn { duration: u32, damage_per_turn: i32 },
    Freeze { duration: u32 },
    Stun { duration: u32 },
    LifeDrain { percent: f32 },
    Summon { kind: String, count: u32 },
    AreaDamage { radius: usize },
    Buff { stat: String, amount: i32, duration: u32 },
    Debuff { stat: String, amount: i32, duration: u32 },
}

impl EnemyAbility {
    /// Creates a new ability
    pub fn new(name: &str, damage: i32, range: usize, cooldown: u32, effect: AbilityEffect) -> Self {
        Self {
            name: name.to_string(),
            damage,
            range,
            cooldown,
            current_cooldown: 0,
            effect,
        }
    }

    /// Checks if the ability can be used
    pub fn is_ready(&self) -> bool {
        self.current_cooldown == 0
    }

    /// Uses the ability
    pub fn use_ability(&mut self) {
        self.current_cooldown = self.cooldown;
    }

    /// Ticks the cooldown
    pub fn tick(&mut self) {
        if self.current_cooldown > 0 {
            self.current_cooldown -= 1;
        }
    }

    /// Gets default abilities for an enemy kind
    pub fn for_kind(kind: AgentKind) -> Vec<Self> {
        match kind {
            AgentKind::GoblinShaman => vec![
                Self::new("Curse", 5, 5, 3, AbilityEffect::Debuff {
                    stat: "defense".to_string(), amount: 5, duration: 3
                }),
                Self::new("Heal Ally", 0, 4, 4, AbilityEffect::Buff {
                    stat: "hp".to_string(), amount: 10, duration: 1
                }),
            ],
            AgentKind::VampireLord => vec![
                Self::new("Life Drain", 15, 1, 2, AbilityEffect::LifeDrain { percent: 0.5 }),
                Self::new("Bat Swarm", 10, 3, 5, AbilityEffect::AreaDamage { radius: 2 }),
            ],
            AgentKind::EnemyNecromancer | AgentKind::LichKing => vec![
                Self::new("Raise Dead", 0, 5, 6, AbilityEffect::Summon {
                    kind: "Skeleton".to_string(), count: 2
                }),
                Self::new("Dark Bolt", 20, 6, 2, AbilityEffect::None),
                Self::new("Wither", 5, 4, 4, AbilityEffect::Debuff {
                    stat: "attack".to_string(), amount: 10, duration: 5
                }),
            ],
            AgentKind::ElderDragon => vec![
                Self::new("Fire Breath", 40, 8, 3, AbilityEffect::Burn {
                    duration: 3, damage_per_turn: 10
                }),
                Self::new("Tail Sweep", 25, 2, 2, AbilityEffect::Stun { duration: 1 }),
                Self::new("Wing Buffet", 15, 3, 4, AbilityEffect::AreaDamage { radius: 3 }),
            ],
            AgentKind::DemonLord => vec![
                Self::new("Hellfire", 35, 6, 3, AbilityEffect::Burn {
                    duration: 4, damage_per_turn: 8
                }),
                Self::new("Summon Imps", 0, 0, 8, AbilityEffect::Summon {
                    kind: "Imp".to_string(), count: 3
                }),
                Self::new("Soul Crush", 50, 1, 5, AbilityEffect::None),
            ],
            AgentKind::Assassin | AgentKind::ShadowAssassin => vec![
                Self::new("Backstab", 30, 1, 0, AbilityEffect::None),
                Self::new("Poison Blade", 10, 1, 3, AbilityEffect::Poison {
                    duration: 5, damage_per_turn: 5
                }),
                Self::new("Vanish", 0, 0, 6, AbilityEffect::None),
            ],
            _ => Vec::new(),
        }
    }
}

/// Enemy group/pack behavior
#[derive(Clone, Debug)]
pub struct EnemyPack {
    /// Pack leader
    pub leader: AgentId,
    /// Pack members
    pub members: Vec<AgentId>,
    /// Pack behavior
    pub behavior: PackBehavior,
    /// Current target
    pub target: Option<AgentId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PackBehavior {
    /// Follow the leader
    FollowLeader,
    /// Spread out and surround
    Surround,
    /// Attack in waves
    Waves,
    /// Scatter and ambush
    Scatter,
    /// Protect leader
    ProtectLeader,
}

impl EnemyPack {
    /// Creates a new pack
    pub fn new(leader: AgentId, members: Vec<AgentId>, behavior: PackBehavior) -> Self {
        Self {
            leader,
            members,
            behavior,
            target: None,
        }
    }

    /// Sets the pack's target
    pub fn set_target(&mut self, target: AgentId) {
        self.target = Some(target);
    }

    /// Returns all pack agents
    pub fn all_members(&self) -> Vec<AgentId> {
        let mut all = vec![self.leader];
        all.extend(self.members.iter().copied());
        all
    }
}

/// Aggro/threat table for enemies
#[derive(Clone, Debug, Default)]
pub struct ThreatTable {
    /// Threat levels by agent ID
    pub threats: Vec<(AgentId, i32)>,
}

impl ThreatTable {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds threat from an agent
    pub fn add_threat(&mut self, agent: AgentId, amount: i32) {
        if let Some(entry) = self.threats.iter_mut().find(|(id, _)| *id == agent) {
            entry.1 += amount;
        } else {
            self.threats.push((agent, amount));
        }
        self.threats.sort_by(|a, b| b.1.cmp(&a.1));
    }

    /// Gets the highest threat target
    pub fn top_threat(&self) -> Option<AgentId> {
        self.threats.first().map(|(id, _)| *id)
    }

    /// Decays all threat over time
    pub fn decay(&mut self, amount: i32) {
        for (_, threat) in &mut self.threats {
            *threat = (*threat - amount).max(0);
        }
        self.threats.retain(|(_, threat)| *threat > 0);
    }

    /// Clears all threat
    pub fn clear(&mut self) {
        self.threats.clear();
    }
}
