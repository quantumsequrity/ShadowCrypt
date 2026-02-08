//! Companion agent implementations

use super::{Agent, AgentId, AgentKind, AgentState};
use crate::behavior::{Goal, GoalKind, GoalPriority};
use rand::Rng;
use serde::{Serialize, Deserialize};

/// Companion-specific behaviors
pub struct CompanionBehaviors;

impl CompanionBehaviors {
    /// Creates default goals for a companion
    pub fn default_goals(kind: AgentKind) -> Vec<Goal> {
        match kind {
            // Combat companions attack enemies
            AgentKind::WolfCompanion | AgentKind::BearCompanion => vec![
                Goal::new(GoalKind::Follow, GoalPriority::High),
                Goal::new(GoalKind::ProtectMaster, GoalPriority::Critical),
                Goal::new(GoalKind::AttackEnemy, GoalPriority::High),
            ],
            // Scout companions explore
            AgentKind::HawkCompanion => vec![
                Goal::new(GoalKind::Scout, GoalPriority::High),
                Goal::new(GoalKind::Follow, GoalPriority::Medium),
                Goal::new(GoalKind::AlertMaster, GoalPriority::High),
            ],
            // Minions attack
            AgentKind::SkeletonMinion => vec![
                Goal::new(GoalKind::AttackEnemy, GoalPriority::High),
                Goal::new(GoalKind::Follow, GoalPriority::Medium),
            ],
            // Elementals are aggressive
            AgentKind::FireElemental | AgentKind::IceElemental | AgentKind::EarthElemental => vec![
                Goal::new(GoalKind::AttackEnemy, GoalPriority::High),
                Goal::new(GoalKind::ProtectMaster, GoalPriority::High),
            ],
            // Spirit guides support
            AgentKind::SpiritGuide => vec![
                Goal::new(GoalKind::SupportMaster, GoalPriority::High),
                Goal::new(GoalKind::Heal, GoalPriority::High),
                Goal::new(GoalKind::Follow, GoalPriority::Low),
            ],
            // Mercenaries fight for pay
            AgentKind::Mercenary => vec![
                Goal::new(GoalKind::AttackEnemy, GoalPriority::High),
                Goal::new(GoalKind::Follow, GoalPriority::Medium),
                Goal::new(GoalKind::Survive, GoalPriority::High),
            ],
            // Squires assist
            AgentKind::Squire => vec![
                Goal::new(GoalKind::Follow, GoalPriority::High),
                Goal::new(GoalKind::SupportMaster, GoalPriority::Medium),
                Goal::new(GoalKind::PickUpItems, GoalPriority::Low),
            ],
            // Familiars are magical helpers
            AgentKind::Familiar => vec![
                Goal::new(GoalKind::Follow, GoalPriority::High),
                Goal::new(GoalKind::SupportMaster, GoalPriority::Medium),
                Goal::new(GoalKind::CastSpell, GoalPriority::Low),
            ],
            // Golems are protectors
            AgentKind::Golem => vec![
                Goal::new(GoalKind::ProtectMaster, GoalPriority::Critical),
                Goal::new(GoalKind::AttackEnemy, GoalPriority::High),
                Goal::new(GoalKind::Follow, GoalPriority::Low),
            ],
            // Fairies support and heal
            AgentKind::FairyCompanion => vec![
                Goal::new(GoalKind::Heal, GoalPriority::High),
                Goal::new(GoalKind::SupportMaster, GoalPriority::High),
                Goal::new(GoalKind::Follow, GoalPriority::Medium),
            ],
            // Shadow clones attack
            AgentKind::ShadowClone => vec![
                Goal::new(GoalKind::AttackEnemy, GoalPriority::High),
                Goal::new(GoalKind::Follow, GoalPriority::High),
            ],
            // Guardian angels protect
            AgentKind::GuardianAngel => vec![
                Goal::new(GoalKind::ProtectMaster, GoalPriority::Critical),
                Goal::new(GoalKind::Heal, GoalPriority::High),
                Goal::new(GoalKind::Bless, GoalPriority::Medium),
            ],
            _ => vec![
                Goal::new(GoalKind::Follow, GoalPriority::High),
            ],
        }
    }

    /// Gets random companion dialogue
    pub fn random_dialogue<R: Rng>(kind: AgentKind, rng: &mut R) -> String {
        let dialogues = Self::dialogues_for_kind(kind);
        dialogues[rng.r#gen_range(0..dialogues.len())].to_string()
    }

    /// Gets all dialogues for a companion kind
    fn dialogues_for_kind(kind: AgentKind) -> &'static [&'static str] {
        match kind {
            AgentKind::WolfCompanion => &[
                "*loyal howl*",
                "*wags tail*",
                "*sniffs the air*",
                "*growls protectively*",
            ],
            AgentKind::BearCompanion => &[
                "*protective roar*",
                "*stands tall*",
                "*grumbles contentedly*",
                "*swipes at threat*",
            ],
            AgentKind::HawkCompanion => &[
                "*keen screech*",
                "*circles overhead*",
                "*spots prey*",
                "*lands on shoulder*",
            ],
            AgentKind::SpiritGuide => &[
                "I sense danger ahead...",
                "The spirits guide us...",
                "Trust your instincts...",
                "We are not alone...",
            ],
            AgentKind::Mercenary => &[
                "You're paying for this, right?",
                "Point me at the enemy.",
                "Gold first, questions later.",
                "I've seen worse odds.",
            ],
            AgentKind::Squire => &[
                "At your service, my lord!",
                "I'll carry that for you.",
                "How can I help?",
                "I'm learning so much!",
            ],
            AgentKind::FairyCompanion => &[
                "*sparkles happily*",
                "Ooh, shiny things!",
                "I'll heal your boo-boos!",
                "*giggles magically*",
            ],
            AgentKind::GuardianAngel => &[
                "I shall protect you.",
                "The light guides us.",
                "Fear not, mortal.",
                "Divine grace surrounds you.",
            ],
            _ => &[
                "*loyal presence*",
                "*ready for action*",
                "*awaits command*",
            ],
        }
    }
}

/// Loyalty levels for companions
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LoyaltyLevel {
    /// Just met, may abandon
    Neutral,
    /// Starting to trust
    Friendly,
    /// Reliable companion
    Loyal,
    /// Devoted forever
    Devoted,
    /// Will die for master
    Fanatical,
}

impl LoyaltyLevel {
    /// Returns flee threshold (HP percentage to flee at)
    pub fn flee_threshold(&self) -> f32 {
        match self {
            Self::Neutral => 0.5,
            Self::Friendly => 0.3,
            Self::Loyal => 0.2,
            Self::Devoted => 0.1,
            Self::Fanatical => 0.0,
        }
    }

    /// Returns if companion will sacrifice for master
    pub fn will_sacrifice(&self) -> bool {
        matches!(self, Self::Devoted | Self::Fanatical)
    }

    /// Increases loyalty
    pub fn increase(&self) -> Self {
        match self {
            Self::Neutral => Self::Friendly,
            Self::Friendly => Self::Loyal,
            Self::Loyal => Self::Devoted,
            Self::Devoted => Self::Fanatical,
            Self::Fanatical => Self::Fanatical,
        }
    }

    /// Decreases loyalty
    pub fn decrease(&self) -> Self {
        match self {
            Self::Neutral => Self::Neutral,
            Self::Friendly => Self::Neutral,
            Self::Loyal => Self::Friendly,
            Self::Devoted => Self::Loyal,
            Self::Fanatical => Self::Devoted,
        }
    }
}

/// Companion state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompanionState {
    /// Who owns this companion
    pub master: Option<AgentId>,
    /// Current loyalty level
    pub loyalty: LoyaltyLevel,
    /// Experience with master
    pub experience: u32,
    /// Current command
    pub command: CompanionCommand,
    /// Time remaining for summon (None = permanent)
    pub summon_duration: Option<u32>,
    /// Abilities unlocked
    pub abilities: Vec<CompanionAbility>,
    /// Current formation position
    pub formation_position: (i32, i32),
}

impl Default for CompanionState {
    fn default() -> Self {
        Self {
            master: None,
            loyalty: LoyaltyLevel::Neutral,
            experience: 0,
            command: CompanionCommand::Follow,
            summon_duration: None,
            abilities: Vec::new(),
            formation_position: (0, -1),
        }
    }
}

impl CompanionState {
    /// Creates a new companion state with a master
    pub fn with_master(master: AgentId) -> Self {
        Self {
            master: Some(master),
            ..Default::default()
        }
    }

    /// Gains experience and potentially increases loyalty
    pub fn gain_experience(&mut self, amount: u32) -> bool {
        self.experience += amount;
        let threshold = match self.loyalty {
            LoyaltyLevel::Neutral => 100,
            LoyaltyLevel::Friendly => 300,
            LoyaltyLevel::Loyal => 700,
            LoyaltyLevel::Devoted => 1500,
            LoyaltyLevel::Fanatical => u32::MAX,
        };

        if self.experience >= threshold {
            self.loyalty = self.loyalty.increase();
            return true;
        }
        false
    }

    /// Ticks summon duration
    pub fn tick(&mut self) -> bool {
        if let Some(ref mut duration) = self.summon_duration {
            *duration = duration.saturating_sub(1);
            return *duration == 0;
        }
        false
    }
}

/// Commands that can be given to companions
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompanionCommand {
    /// Follow the master
    Follow,
    /// Stay in place
    Stay,
    /// Attack a specific target
    Attack,
    /// Defend master
    Defend,
    /// Scout ahead
    Scout,
    /// Retreat/flee
    Retreat,
    /// Use special ability
    UseAbility,
    /// Act freely
    FreeWill,
    /// Wait for orders
    Wait,
}

/// Abilities companions can have
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompanionAbility {
    pub name: String,
    pub description: String,
    pub cooldown: u32,
    pub current_cooldown: u32,
    pub effect: CompanionAbilityEffect,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CompanionAbilityEffect {
    /// Deal damage to target
    Attack { damage: i32 },
    /// Heal master or self
    Heal { amount: i32 },
    /// Apply buff to master
    Buff { stat: String, amount: i32, duration: u32 },
    /// Apply debuff to enemy
    Debuff { stat: String, amount: i32, duration: u32 },
    /// Area damage
    AreaAttack { damage: i32, radius: usize },
    /// Shield master from damage
    Shield { amount: i32, duration: u32 },
    /// Taunt enemies
    Taunt { duration: u32 },
    /// Scout area revealing map
    Reveal { radius: usize },
    /// Find items
    FindItems { radius: usize },
}

impl CompanionAbility {
    pub fn new(name: &str, description: &str, cooldown: u32, effect: CompanionAbilityEffect) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            cooldown,
            current_cooldown: 0,
            effect,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.current_cooldown == 0
    }

    pub fn use_ability(&mut self) {
        self.current_cooldown = self.cooldown;
    }

    pub fn tick(&mut self) {
        if self.current_cooldown > 0 {
            self.current_cooldown -= 1;
        }
    }

    /// Gets default abilities for a companion kind
    pub fn for_kind(kind: AgentKind) -> Vec<Self> {
        match kind {
            AgentKind::WolfCompanion => vec![
                Self::new("Bite", "Deals damage and may cause bleeding", 0,
                    CompanionAbilityEffect::Attack { damage: 15 }),
                Self::new("Howl", "Buffs attack of all allies", 8,
                    CompanionAbilityEffect::Buff { stat: "attack".to_string(), amount: 5, duration: 5 }),
            ],
            AgentKind::BearCompanion => vec![
                Self::new("Swipe", "Powerful melee attack", 0,
                    CompanionAbilityEffect::Attack { damage: 25 }),
                Self::new("Roar", "Taunts enemies to attack bear", 10,
                    CompanionAbilityEffect::Taunt { duration: 5 }),
            ],
            AgentKind::HawkCompanion => vec![
                Self::new("Dive", "Quick attack from above", 0,
                    CompanionAbilityEffect::Attack { damage: 10 }),
                Self::new("Eagle Eye", "Reveals nearby area", 5,
                    CompanionAbilityEffect::Reveal { radius: 10 }),
            ],
            AgentKind::FireElemental => vec![
                Self::new("Fireball", "Ranged fire attack", 2,
                    CompanionAbilityEffect::Attack { damage: 20 }),
                Self::new("Immolate", "Area fire damage", 6,
                    CompanionAbilityEffect::AreaAttack { damage: 15, radius: 3 }),
            ],
            AgentKind::FairyCompanion => vec![
                Self::new("Heal", "Heals master", 3,
                    CompanionAbilityEffect::Heal { amount: 20 }),
                Self::new("Magic Shield", "Shields master from damage", 8,
                    CompanionAbilityEffect::Shield { amount: 30, duration: 5 }),
            ],
            AgentKind::GuardianAngel => vec![
                Self::new("Divine Heal", "Powerful healing", 4,
                    CompanionAbilityEffect::Heal { amount: 40 }),
                Self::new("Protection", "Major damage shield", 10,
                    CompanionAbilityEffect::Shield { amount: 50, duration: 8 }),
                Self::new("Smite", "Holy damage to undead/demons", 3,
                    CompanionAbilityEffect::Attack { damage: 35 }),
            ],
            _ => Vec::new(),
        }
    }
}

/// Formation types for multiple companions
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Formation {
    /// Companions follow behind
    Follow,
    /// Companions form line in front
    Line,
    /// Companions surround master
    Circle,
    /// Companions in V shape
    Wedge,
    /// Companions spread out
    Spread,
    /// Companions in column
    Column,
}

impl Formation {
    /// Gets position offsets for formation
    pub fn get_positions(&self, count: usize) -> Vec<(i32, i32)> {
        match self {
            Self::Follow => (0..count).map(|i| (0, -(i as i32 + 1))).collect(),
            Self::Line => (0..count).map(|i| (i as i32 - count as i32 / 2, -1)).collect(),
            Self::Circle => {
                let mut positions = Vec::new();
                for i in 0..count {
                    let angle = (i as f64 / count as f64) * std::f64::consts::TAU;
                    positions.push((
                        (angle.cos() * 2.0).round() as i32,
                        (angle.sin() * 2.0).round() as i32,
                    ));
                }
                positions
            }
            Self::Wedge => {
                let mut positions = Vec::new();
                let mut row = 1;
                let mut pos = 0;
                for _ in 0..count {
                    positions.push((pos - row / 2, -row));
                    pos += 1;
                    if pos >= row {
                        row += 1;
                        pos = 0;
                    }
                }
                positions
            }
            Self::Spread => {
                (0..count).map(|i| {
                    let angle = (i as f64 / count as f64) * std::f64::consts::TAU;
                    (
                        (angle.cos() * 4.0).round() as i32,
                        (angle.sin() * 4.0).round() as i32,
                    )
                }).collect()
            }
            Self::Column => (0..count).map(|i| (0, -(i as i32 + 1))).collect(),
        }
    }
}
