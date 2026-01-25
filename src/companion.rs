// ============================================================================
// COMPANION AND PARTY SYSTEM
// ============================================================================

use crate::{CharacterClass, Color, Enemy, EquipSlot, Item, Skill, StatusEffect};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// AI behavior modes for companions
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum CompanionAI {
    Aggressive,  // Prioritize attacking, charge into battle
    Defensive,   // Stay close to player, protect, use shields
    Support,     // Focus on healing, buffs, stay back
    Balanced,    // Mix of offense and defense based on situation
    Passive,     // Don't attack unless attacked, follow player
}

impl CompanionAI {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Aggressive => "Aggressive",
            Self::Defensive => "Defensive",
            Self::Support => "Support",
            Self::Balanced => "Balanced",
            Self::Passive => "Passive",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Aggressive => "Charges into battle, prioritizes damage",
            Self::Defensive => "Protects allies, blocks attacks, uses shields",
            Self::Support => "Heals and buffs allies, stays at range",
            Self::Balanced => "Adapts tactics based on situation",
            Self::Passive => "Only attacks when threatened, follows player",
        }
    }
}

/// Relationship levels with companions
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub enum Relationship {
    Hostile = -2,    // Will leave or betray you
    Unfriendly = -1, // Reluctant, may refuse orders
    Neutral = 0,     // Basic cooperation
    Friendly = 1,    // Good relations, bonus effectiveness
    Loyal = 2,       // Very strong bond, will sacrifice for you
    Bonded = 3,      // Soulbound, romance available, unique abilities
}

impl Relationship {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Hostile => "Hostile",
            Self::Unfriendly => "Unfriendly",
            Self::Neutral => "Neutral",
            Self::Friendly => "Friendly",
            Self::Loyal => "Loyal",
            Self::Bonded => "Bonded",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::Hostile => Color::DarkRed,
            Self::Unfriendly => Color::Red,
            Self::Neutral => Color::Grey,
            Self::Friendly => Color::Green,
            Self::Loyal => Color::Cyan,
            Self::Bonded => Color::Magenta,
        }
    }

    pub fn stat_bonus(&self) -> f32 {
        match self {
            Self::Hostile => 0.5,
            Self::Unfriendly => 0.75,
            Self::Neutral => 1.0,
            Self::Friendly => 1.15,
            Self::Loyal => 1.3,
            Self::Bonded => 1.5,
        }
    }

    pub fn xp_needed_for_next(&self) -> Option<u32> {
        match self {
            Self::Hostile => Some(100),
            Self::Unfriendly => Some(200),
            Self::Neutral => Some(500),
            Self::Friendly => Some(1000),
            Self::Loyal => Some(2500),
            Self::Bonded => None, // Max level
        }
    }

    pub fn next(&self) -> Option<Self> {
        match self {
            Self::Hostile => Some(Self::Unfriendly),
            Self::Unfriendly => Some(Self::Neutral),
            Self::Neutral => Some(Self::Friendly),
            Self::Friendly => Some(Self::Loyal),
            Self::Loyal => Some(Self::Bonded),
            Self::Bonded => None,
        }
    }

    pub fn previous(&self) -> Option<Self> {
        match self {
            Self::Hostile => None,
            Self::Unfriendly => Some(Self::Hostile),
            Self::Neutral => Some(Self::Unfriendly),
            Self::Friendly => Some(Self::Neutral),
            Self::Loyal => Some(Self::Friendly),
            Self::Bonded => Some(Self::Loyal),
        }
    }
}

/// Combat formation positions
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum FormationPosition {
    FrontLeft, FrontCenter, FrontRight,
    BackLeft, BackCenter, BackRight,
}

impl FormationPosition {
    pub fn is_front_row(&self) -> bool {
        matches!(self, Self::FrontLeft | Self::FrontCenter | Self::FrontRight)
    }

    pub fn defense_modifier(&self) -> f32 { if self.is_front_row() { 1.0 } else { 1.25 } }
    pub fn attack_modifier(&self) -> f32 { if self.is_front_row() { 1.0 } else { 0.85 } }

    pub fn name(&self) -> &'static str {
        match self {
            Self::FrontLeft => "Front Left", Self::FrontCenter => "Front Center",
            Self::FrontRight => "Front Right", Self::BackLeft => "Back Left",
            Self::BackCenter => "Back Center", Self::BackRight => "Back Right",
        }
    }
}

/// Party formations for different combat strategies
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum PartyFormation {
    Standard, Aggressive, Defensive, Flanking, Wedge, Circle,
}

impl PartyFormation {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Standard => "Standard", Self::Aggressive => "Aggressive",
            Self::Defensive => "Defensive", Self::Flanking => "Flanking",
            Self::Wedge => "Wedge", Self::Circle => "Circle",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Standard => "Balanced formation with tanks in front",
            Self::Aggressive => "Most members in front for maximum damage",
            Self::Defensive => "Most members protected in back row",
            Self::Flanking => "Split to attack from multiple angles",
            Self::Wedge => "Arrow formation for breaking through",
            Self::Circle => "All members protect each other",
        }
    }

    pub fn positions(&self) -> Vec<FormationPosition> {
        match self {
            Self::Standard => vec![FormationPosition::FrontLeft, FormationPosition::FrontRight, FormationPosition::BackLeft, FormationPosition::BackRight],
            Self::Aggressive => vec![FormationPosition::FrontLeft, FormationPosition::FrontCenter, FormationPosition::FrontRight, FormationPosition::BackCenter],
            Self::Defensive => vec![FormationPosition::FrontCenter, FormationPosition::BackLeft, FormationPosition::BackCenter, FormationPosition::BackRight],
            Self::Flanking => vec![FormationPosition::FrontLeft, FormationPosition::FrontRight, FormationPosition::BackLeft, FormationPosition::BackRight],
            Self::Wedge => vec![FormationPosition::FrontCenter, FormationPosition::BackLeft, FormationPosition::BackRight, FormationPosition::BackCenter],
            Self::Circle => vec![FormationPosition::FrontLeft, FormationPosition::FrontRight, FormationPosition::BackLeft, FormationPosition::BackRight],
        }
    }

    pub fn attack_bonus(&self) -> i32 { match self { Self::Aggressive => 5, Self::Wedge => 3, Self::Flanking => 4, _ => 0 } }
    pub fn defense_bonus(&self) -> i32 { match self { Self::Defensive => 5, Self::Circle => 4, _ => 0 } }
}

/// Personality traits that affect companion behavior and dialogue
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Personality {
    Brave, Cautious, Loyal, Greedy, Kind, Cruel, Wise, Foolish,
    Romantic, Stoic, Vengeful, Protective, Ambitious, Humble, Mysterious, Cheerful,
}

impl Personality {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Brave => "Brave", Self::Cautious => "Cautious", Self::Loyal => "Loyal",
            Self::Greedy => "Greedy", Self::Kind => "Kind", Self::Cruel => "Cruel",
            Self::Wise => "Wise", Self::Foolish => "Foolish", Self::Romantic => "Romantic",
            Self::Stoic => "Stoic", Self::Vengeful => "Vengeful", Self::Protective => "Protective",
            Self::Ambitious => "Ambitious", Self::Humble => "Humble",
            Self::Mysterious => "Mysterious", Self::Cheerful => "Cheerful",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Brave => "Never retreats, fights harder when wounded",
            Self::Cautious => "Careful and defensive, retreats when hurt",
            Self::Loyal => "Steadfast companion, quickly forms bonds",
            Self::Greedy => "Loves treasure, may pocket some for themselves",
            Self::Kind => "Compassionate healer, loved by all",
            Self::Cruel => "Merciless in battle, no quarter given",
            Self::Wise => "Quick learner, gains experience faster",
            Self::Foolish => "Unpredictable, sometimes brilliantly so",
            Self::Romantic => "Seeks deep connections and love",
            Self::Stoic => "Emotionally strong, resists mental effects",
            Self::Vengeful => "Remembers every slight, seeks payback",
            Self::Protective => "Will sacrifice themselves for allies",
            Self::Ambitious => "Seeks power and recognition",
            Self::Humble => "Quiet and supportive, great team player",
            Self::Mysterious => "Hides secrets and hidden powers",
            Self::Cheerful => "Optimistic, keeps morale high",
        }
    }
}

/// Special abilities unique to companions
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum CompanionAbility {
    // Combat
    ShieldAlly, BattleCry, Taunt, Flurry, Execute, Cleave,
    // Support
    HealingTouch, MassHeal, Resurrect, Bless, Purify, Sanctuary,
    // Magic
    CompFireball, IceStorm, CompLightning, CompTeleport, Summon, TimeStop,
    // Stealth
    CompBackstab, CompVanish, Poison, Steal, Disarm, Ambush,
    // Unique
    DragonBreath, BeastForm, UnholyMight, DivineSmite, NatureWrath, SoulLink,
    // Passive
    CompRegeneration, TreasureHunter, Scout, Diplomat, MartialArts, ArcaneMastery,
}

impl CompanionAbility {
    pub fn name(&self) -> &'static str {
        match self {
            Self::ShieldAlly => "Shield Ally", Self::BattleCry => "Battle Cry",
            Self::Taunt => "Taunt", Self::Flurry => "Flurry",
            Self::Execute => "Execute", Self::Cleave => "Cleave",
            Self::HealingTouch => "Healing Touch", Self::MassHeal => "Mass Heal",
            Self::Resurrect => "Resurrect", Self::Bless => "Bless",
            Self::Purify => "Purify", Self::Sanctuary => "Sanctuary",
            Self::CompFireball => "Fireball", Self::IceStorm => "Ice Storm",
            Self::CompLightning => "Lightning", Self::CompTeleport => "Teleport",
            Self::Summon => "Summon", Self::TimeStop => "Time Stop",
            Self::CompBackstab => "Backstab", Self::CompVanish => "Vanish",
            Self::Poison => "Poison", Self::Steal => "Steal",
            Self::Disarm => "Disarm", Self::Ambush => "Ambush",
            Self::DragonBreath => "Dragon Breath", Self::BeastForm => "Beast Form",
            Self::UnholyMight => "Unholy Might", Self::DivineSmite => "Divine Smite",
            Self::NatureWrath => "Nature's Wrath", Self::SoulLink => "Soul Link",
            Self::CompRegeneration => "Regeneration", Self::TreasureHunter => "Treasure Hunter",
            Self::Scout => "Scout", Self::Diplomat => "Diplomat",
            Self::MartialArts => "Martial Arts", Self::ArcaneMastery => "Arcane Mastery",
        }
    }

    pub fn mana_cost(&self) -> i32 {
        match self {
            Self::ShieldAlly | Self::Taunt | Self::CompBackstab | Self::Poison => 10,
            Self::BattleCry | Self::Flurry | Self::Cleave | Self::Disarm | Self::Ambush => 15,
            Self::HealingTouch | Self::Bless | Self::Purify | Self::CompVanish | Self::Steal => 20,
            Self::Execute | Self::CompFireball | Self::CompLightning => 25,
            Self::MassHeal | Self::Sanctuary | Self::IceStorm | Self::CompTeleport => 35,
            Self::Resurrect | Self::Summon | Self::DragonBreath | Self::BeastForm => 50,
            Self::TimeStop | Self::UnholyMight | Self::DivineSmite | Self::NatureWrath => 60,
            Self::SoulLink => 100,
            Self::CompRegeneration | Self::TreasureHunter | Self::Scout | Self::Diplomat | Self::MartialArts | Self::ArcaneMastery => 0,
        }
    }

    pub fn is_passive(&self) -> bool {
        matches!(self, Self::CompRegeneration | Self::TreasureHunter | Self::Scout | Self::Diplomat | Self::MartialArts | Self::ArcaneMastery)
    }

    pub fn cooldown(&self) -> u32 {
        match self {
            Self::Resurrect | Self::TimeStop | Self::SoulLink => 20,
            Self::MassHeal | Self::Summon | Self::DragonBreath | Self::BeastForm => 10,
            Self::Sanctuary | Self::UnholyMight | Self::DivineSmite | Self::NatureWrath => 8,
            Self::Execute | Self::IceStorm => 5,
            _ => 3,
        }
    }
}

/// Quest types for companion personal quests
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum CompanionQuestType {
    Revenge { target_name: String },
    Redemption { good_deeds_needed: u32 },
    LostFamily { dungeon_level: u32 },
    AncientArtifact { item_name: String },
    ProveWorth { kills_needed: u32 },
    MasterSkill { skill_uses: u32 },
    Confession { relationship_needed: Relationship },
    Homecoming { destination_level: u32 },
}

/// State of a companion's personal quest
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct CompanionQuest {
    pub quest_type: CompanionQuestType,
    pub progress: u32,
    pub completed: bool,
    pub reward_claimed: bool,
    pub description: String,
}

impl CompanionQuest {
    pub fn new(quest_type: CompanionQuestType, description: &str) -> Self {
        Self { quest_type, progress: 0, completed: false, reward_claimed: false, description: description.to_string() }
    }

    pub fn target(&self) -> u32 {
        match &self.quest_type {
            CompanionQuestType::Revenge { .. } => 1,
            CompanionQuestType::Redemption { good_deeds_needed } => *good_deeds_needed,
            CompanionQuestType::LostFamily { .. } => 1,
            CompanionQuestType::AncientArtifact { .. } => 1,
            CompanionQuestType::ProveWorth { kills_needed } => *kills_needed,
            CompanionQuestType::MasterSkill { skill_uses } => *skill_uses,
            CompanionQuestType::Confession { .. } => 1,
            CompanionQuestType::Homecoming { .. } => 1,
        }
    }

    pub fn check_completion(&mut self) -> bool {
        if !self.completed && self.progress >= self.target() { self.completed = true; true } else { false }
    }
}

/// Romance state with a companion
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct RomanceState {
    pub active: bool,
    pub stage: u32,
    pub gifts_given: u32,
    pub dates_completed: u32,
    pub confession_done: bool,
    pub bonded: bool,
}

impl RomanceState {
    pub fn new() -> Self { Self { active: false, stage: 0, gifts_given: 0, dates_completed: 0, confession_done: false, bonded: false } }

    pub fn can_advance(&self, relationship: Relationship) -> bool {
        if self.bonded { return false; }
        match self.stage {
            0 => relationship >= Relationship::Friendly,
            1 => self.gifts_given >= 3 && relationship >= Relationship::Friendly,
            2 => self.dates_completed >= 2 && relationship >= Relationship::Loyal,
            3 => self.confession_done && relationship >= Relationship::Loyal,
            4 => relationship >= Relationship::Bonded,
            _ => false,
        }
    }

    pub fn stage_name(&self) -> &'static str {
        match self.stage { 0 => "Not Started", 1 => "Interested", 2 => "Courting", 3 => "Devoted", 4 => "In Love", 5 => "Soulbound", _ => "Unknown" }
    }
}

impl Default for RomanceState {
    fn default() -> Self { Self::new() }
}

/// Companion death/revival state
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum CompanionState {
    Alive,
    Unconscious { turns_remaining: u32 },
    Dead { can_resurrect: bool },
    Permadead,
}

/// Simplified species enum for companions
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum CompanionSpecies {
    Human, Elf, Dwarf, Orc, Goblin, Demon, Undead, Dragonian,
    Beastkin, Fairy, HalfElf, Tiefling, Golem, Spirit, Vampire,
}

impl CompanionSpecies {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Human => "Human", Self::Elf => "Elf", Self::Dwarf => "Dwarf",
            Self::Orc => "Orc", Self::Goblin => "Goblin", Self::Demon => "Demon",
            Self::Undead => "Undead", Self::Dragonian => "Dragonian", Self::Beastkin => "Beastkin",
            Self::Fairy => "Fairy", Self::HalfElf => "Half-Elf", Self::Tiefling => "Tiefling",
            Self::Golem => "Golem", Self::Spirit => "Spirit", Self::Vampire => "Vampire",
        }
    }

    pub fn stat_bonuses(&self) -> (i32, i32, i32, i32, i32) {
        match self {
            Self::Human => (10, 5, 5, 5, 5), Self::Elf => (0, 5, 0, 10, 15),
            Self::Dwarf => (20, 5, 15, -5, 0), Self::Orc => (15, 15, 5, 0, -5),
            Self::Goblin => (0, 5, 0, 15, 5), Self::Demon => (10, 15, 5, 5, 10),
            Self::Undead => (25, 10, 10, -5, 5), Self::Dragonian => (20, 15, 10, 5, 10),
            Self::Beastkin => (10, 10, 5, 15, 0), Self::Fairy => (-10, 0, -5, 20, 25),
            Self::HalfElf => (5, 5, 5, 7, 10), Self::Tiefling => (5, 10, 5, 5, 15),
            Self::Golem => (40, 10, 25, -15, -10), Self::Spirit => (-20, 5, -5, 15, 30),
            Self::Vampire => (15, 15, 5, 10, 15),
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::Human => Color::White, Self::Elf => Color::Green, Self::Dwarf => Color::DarkYellow,
            Self::Orc => Color::DarkGreen, Self::Goblin => Color::Yellow, Self::Demon => Color::DarkRed,
            Self::Undead => Color::Grey, Self::Dragonian => Color::Red, Self::Beastkin => Color::DarkYellow,
            Self::Fairy => Color::Magenta, Self::HalfElf => Color::Cyan, Self::Tiefling => Color::DarkMagenta,
            Self::Golem => Color::DarkGrey, Self::Spirit => Color::Cyan, Self::Vampire => Color::DarkRed,
        }
    }
}

/// The main Companion struct
#[derive(Clone, Serialize, Deserialize)]
pub struct Companion {
    pub id: u32,
    pub name: String,
    pub title: String,
    pub backstory: String,
    pub x: usize,
    pub y: usize,
    pub formation_position: FormationPosition,
    pub class: CharacterClass,
    pub species: CompanionSpecies,
    pub level: u32,
    pub xp: u32,
    pub xp_to_level: u32,
    pub hp: i32,
    pub max_hp: i32,
    pub mana: i32,
    pub max_mana: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub weapon: Option<Item>,
    pub armor: Option<Item>,
    pub accessory: Option<Item>,
    pub skills: Vec<Skill>,
    pub special_abilities: Vec<CompanionAbility>,
    pub active_ability: usize,
    pub ability_cooldowns: HashMap<CompanionAbility, u32>,
    pub ai_mode: CompanionAI,
    pub personality: Personality,
    pub relationship: Relationship,
    pub relationship_xp: u32,
    pub romance: RomanceState,
    pub personal_quest: Option<CompanionQuest>,
    pub state: CompanionState,
    pub status_effects: HashMap<StatusEffect, u32>,
    pub greeting: String,
    pub battle_cry: String,
    pub victory_quote: String,
    pub death_quote: String,
    pub romance_dialogue: Vec<String>,
    pub is_essential: bool,
    pub is_romanceable: bool,
    pub recruited: bool,
    pub times_revived: u32,
}

/// AI action result for companions
#[derive(Clone, Copy, Debug)]
pub enum CompanionAIAction {
    Move(i32, i32),
    Attack(usize, usize),
    UseAbility(CompanionAbility),
    #[allow(dead_code)]
    UseItem(usize),
    Wait,
}

impl Companion {
    pub fn new(id: u32, name: &str, title: &str, class: CharacterClass, species: CompanionSpecies, personality: Personality, level: u32) -> Self {
        let (base_hp, base_atk, base_def, base_mana, base_spd) = class.base_stats();
        let (hp_bonus, atk_bonus, def_bonus, spd_bonus, mana_bonus) = species.stat_bonuses();
        let hp = base_hp + hp_bonus + (level as i32 * 8);
        let mana = base_mana + mana_bonus + (level as i32 * 5);

        Self {
            id, name: name.to_string(), title: title.to_string(), backstory: String::new(),
            x: 0, y: 0, formation_position: FormationPosition::BackCenter,
            class, species, level, xp: 0, xp_to_level: 100 + (level * 50),
            hp, max_hp: hp, mana, max_mana: mana,
            attack: base_atk + atk_bonus + (level as i32 * 2),
            defense: base_def + def_bonus + (level as i32),
            speed: base_spd + spd_bonus,
            weapon: None, armor: None, accessory: None,
            skills: Skill::for_class(class), special_abilities: Vec::new(),
            active_ability: 0, ability_cooldowns: HashMap::new(),
            ai_mode: CompanionAI::Balanced, personality,
            relationship: Relationship::Neutral, relationship_xp: 0,
            romance: RomanceState::new(), personal_quest: None,
            state: CompanionState::Alive, status_effects: HashMap::new(),
            greeting: format!("Greetings, I am {}.", name),
            battle_cry: "For glory!".to_string(),
            victory_quote: "We are victorious!".to_string(),
            death_quote: "I... cannot go on...".to_string(),
            romance_dialogue: Vec::new(),
            is_essential: false, is_romanceable: true, recruited: false, times_revived: 0,
        }
    }

    pub fn with_backstory(mut self, backstory: &str) -> Self { self.backstory = backstory.to_string(); self }
    pub fn with_dialogue(mut self, greeting: &str, battle_cry: &str, victory: &str, death: &str) -> Self {
        self.greeting = greeting.to_string(); self.battle_cry = battle_cry.to_string();
        self.victory_quote = victory.to_string(); self.death_quote = death.to_string(); self
    }
    pub fn with_abilities(mut self, abilities: Vec<CompanionAbility>) -> Self { self.special_abilities = abilities; self }
    pub fn with_quest(mut self, quest: CompanionQuest) -> Self { self.personal_quest = Some(quest); self }
    pub fn with_romance_dialogue(mut self, dialogue: Vec<&str>) -> Self { self.romance_dialogue = dialogue.iter().map(|s| s.to_string()).collect(); self }

    pub fn is_alive(&self) -> bool { matches!(self.state, CompanionState::Alive) }
    pub fn is_conscious(&self) -> bool { matches!(self.state, CompanionState::Alive) }
    pub fn can_act(&self) -> bool { self.is_conscious() && !self.has_status(StatusEffect::Stun) }

    pub fn total_attack(&self) -> i32 {
        let mut total = self.attack;
        if let Some(ref weapon) = self.weapon { let (atk, _, _, _) = weapon.stats(); total += atk; }
        total = (total as f32 * self.relationship.stat_bonus()) as i32;
        if self.personality == Personality::Brave && self.hp < self.max_hp / 3 { total = (total as f32 * 1.25) as i32; }
        if self.has_status(StatusEffect::Strength) { total = (total as f32 * 1.5) as i32; }
        total
    }

    pub fn total_defense(&self) -> i32 {
        let mut total = self.defense;
        if let Some(ref armor) = self.armor { let (_, def, _, _) = armor.stats(); total += def; }
        total = (total as f32 * self.relationship.stat_bonus()) as i32;
        if self.personality == Personality::Cautious { total = (total as f32 * 1.15) as i32; }
        total
    }

    pub fn take_damage(&mut self, amount: i32) -> i32 {
        let actual = (amount - self.total_defense()).max(1);
        self.hp -= actual;
        if self.hp <= 0 {
            self.hp = 0;
            if self.is_essential || self.times_revived < 3 {
                self.state = CompanionState::Unconscious { turns_remaining: 10 };
            } else {
                self.state = CompanionState::Dead { can_resurrect: true };
            }
        }
        actual
    }

    pub fn heal(&mut self, amount: i32) { self.hp = (self.hp + amount).min(self.max_hp); }

    pub fn revive(&mut self, hp_percent: i32) {
        self.state = CompanionState::Alive;
        self.hp = (self.max_hp * hp_percent / 100).max(1);
        self.times_revived += 1;
    }

    pub fn has_status(&self, effect: StatusEffect) -> bool { self.status_effects.contains_key(&effect) }

    pub fn add_status(&mut self, effect: StatusEffect, duration: u32) {
        if self.personality == Personality::Stoic {
            match effect {
                StatusEffect::Stun | StatusEffect::Confusion | StatusEffect::Blind => {
                    if rand::thread_rng().gen_bool(0.5) { return; }
                }
                _ => {}
            }
        }
        self.status_effects.insert(effect, duration);
    }

    pub fn gain_xp(&mut self, amount: u32) -> bool {
        let bonus = if self.personality == Personality::Wise { 1.25 } else { 1.0 };
        self.xp += (amount as f32 * bonus) as u32;
        if self.xp >= self.xp_to_level {
            self.xp -= self.xp_to_level; self.level += 1;
            self.xp_to_level = (self.xp_to_level as f32 * 1.3) as u32;
            self.max_hp += 8; self.hp = self.max_hp;
            self.max_mana += 5; self.mana = self.max_mana;
            self.attack += 2; self.defense += 1;
            return true;
        }
        false
    }

    pub fn gain_relationship_xp(&mut self, amount: u32) -> Option<Relationship> {
        let bonus = if self.personality == Personality::Loyal { 1.5 } else { 1.0 };
        self.relationship_xp += (amount as f32 * bonus) as u32;
        if let Some(needed) = self.relationship.xp_needed_for_next() {
            if self.relationship_xp >= needed {
                self.relationship_xp -= needed;
                if let Some(next) = self.relationship.next() { self.relationship = next; return Some(next); }
            }
        }
        None
    }

    pub fn tick_cooldowns(&mut self) {
        for cooldown in self.ability_cooldowns.values_mut() { *cooldown = cooldown.saturating_sub(1); }
        self.ability_cooldowns.retain(|_, &mut v| v > 0);
    }

    pub fn can_use_ability(&self, ability: CompanionAbility) -> bool {
        if ability.is_passive() { return false; }
        if self.mana < ability.mana_cost() { return false; }
        if self.ability_cooldowns.get(&ability).copied().unwrap_or(0) > 0 { return false; }
        true
    }

    pub fn use_ability(&mut self, ability: CompanionAbility) -> bool {
        if !self.can_use_ability(ability) { return false; }
        self.mana -= ability.mana_cost();
        self.ability_cooldowns.insert(ability, ability.cooldown());
        true
    }

    pub fn tick_status_effects(&mut self) -> Vec<String> {
        let mut messages = Vec::new();
        let mut to_remove = Vec::new();
        let mut damage = 0;
        let mut heal_amount = 0;

        for (effect, duration) in self.status_effects.iter_mut() {
            match effect {
                StatusEffect::Poison => { damage += 2; messages.push(format!("{} takes poison damage!", self.name)); }
                StatusEffect::Burn => { damage += 3; messages.push(format!("{} is burning!", self.name)); }
                StatusEffect::Bleed => { damage += 1; messages.push(format!("{} is bleeding!", self.name)); }
                StatusEffect::Regeneration => { heal_amount += 3; }
                _ => {}
            }
            *duration = duration.saturating_sub(1);
            if *duration == 0 { to_remove.push(*effect); }
        }

        if self.special_abilities.contains(&CompanionAbility::CompRegeneration) { heal_amount += 2; }
        self.hp = (self.hp - damage + heal_amount).min(self.max_hp);

        for effect in to_remove {
            self.status_effects.remove(&effect);
            messages.push(format!("{}'s {} wore off.", self.name, effect.name()));
        }

        if let CompanionState::Unconscious { turns_remaining } = &mut self.state {
            *turns_remaining = turns_remaining.saturating_sub(1);
            if *turns_remaining == 0 {
                if self.is_essential {
                    self.revive(25);
                    messages.push(format!("{} regains consciousness!", self.name));
                } else {
                    self.state = CompanionState::Dead { can_resurrect: true };
                    messages.push(format!("{} has died!", self.name));
                }
            }
        }
        messages
    }

    pub fn get_ai_action(&self, player_pos: (usize, usize), enemies: &[Enemy], allies: &[&Companion]) -> CompanionAIAction {
        match self.ai_mode {
            CompanionAI::Aggressive => self.ai_aggressive(enemies),
            CompanionAI::Defensive => self.ai_defensive(player_pos, enemies),
            CompanionAI::Support => self.ai_support(player_pos, allies),
            CompanionAI::Balanced => self.ai_balanced(player_pos, enemies, allies),
            CompanionAI::Passive => self.ai_passive(player_pos),
        }
    }

    fn ai_aggressive(&self, enemies: &[Enemy]) -> CompanionAIAction {
        if let Some(target) = self.find_nearest_enemy(enemies) {
            let dx = target.x as i32 - self.x as i32;
            let dy = target.y as i32 - self.y as i32;
            let dist = ((dx * dx + dy * dy) as f32).sqrt();
            if dist <= 1.5 { return CompanionAIAction::Attack(target.x, target.y); }
            return CompanionAIAction::Move(dx.signum(), dy.signum());
        }
        CompanionAIAction::Wait
    }

    fn ai_defensive(&self, player_pos: (usize, usize), enemies: &[Enemy]) -> CompanionAIAction {
        let dx = player_pos.0 as i32 - self.x as i32;
        let dy = player_pos.1 as i32 - self.y as i32;
        let player_dist = ((dx * dx + dy * dy) as f32).sqrt();

        if let Some(enemy) = self.find_nearest_enemy(enemies) {
            let ex = enemy.x as i32 - self.x as i32;
            let ey = enemy.y as i32 - self.y as i32;
            let enemy_dist = ((ex * ex + ey * ey) as f32).sqrt();
            if enemy_dist <= 1.5 { return CompanionAIAction::Attack(enemy.x, enemy.y); }
        }

        if player_dist > 3.0 { return CompanionAIAction::Move(dx.signum(), dy.signum()); }
        if self.can_use_ability(CompanionAbility::ShieldAlly) {
            return CompanionAIAction::UseAbility(CompanionAbility::ShieldAlly);
        }
        CompanionAIAction::Wait
    }

    fn ai_support(&self, player_pos: (usize, usize), allies: &[&Companion]) -> CompanionAIAction {
        let mut lowest_hp_ally: Option<(usize, usize, i32)> = None;
        for ally in allies {
            if ally.id != self.id && ally.is_alive() {
                let hp_percent = (ally.hp * 100) / ally.max_hp.max(1);
                if hp_percent < 50 {
                    if lowest_hp_ally.is_none() || hp_percent < lowest_hp_ally.unwrap().2 {
                        lowest_hp_ally = Some((ally.x, ally.y, hp_percent));
                    }
                }
            }
        }

        if let Some((hx, hy, _)) = lowest_hp_ally {
            if self.can_use_ability(CompanionAbility::HealingTouch) {
                let dx = hx as i32 - self.x as i32;
                let dy = hy as i32 - self.y as i32;
                let dist = ((dx * dx + dy * dy) as f32).sqrt();
                if dist <= 2.0 { return CompanionAIAction::UseAbility(CompanionAbility::HealingTouch); }
                return CompanionAIAction::Move(dx.signum(), dy.signum());
            }
            if self.can_use_ability(CompanionAbility::MassHeal) {
                return CompanionAIAction::UseAbility(CompanionAbility::MassHeal);
            }
        }

        let dx = player_pos.0 as i32 - self.x as i32;
        let dy = player_pos.1 as i32 - self.y as i32;
        let player_dist = ((dx * dx + dy * dy) as f32).sqrt();
        if player_dist > 4.0 || player_dist < 2.0 {
            let move_x = if player_dist > 4.0 { dx.signum() } else { -dx.signum() };
            let move_y = if player_dist > 4.0 { dy.signum() } else { -dy.signum() };
            return CompanionAIAction::Move(move_x, move_y);
        }
        CompanionAIAction::Wait
    }

    fn ai_balanced(&self, player_pos: (usize, usize), enemies: &[Enemy], allies: &[&Companion]) -> CompanionAIAction {
        let hp_percent = (self.hp * 100) / self.max_hp.max(1);
        if hp_percent < 30 { return self.ai_defensive(player_pos, enemies); }
        let allies_hurt: Vec<_> = allies.iter().filter(|a| a.id != self.id && a.is_alive() && a.hp * 100 / a.max_hp.max(1) < 40).collect();
        if !allies_hurt.is_empty() && self.special_abilities.contains(&CompanionAbility::HealingTouch) {
            return self.ai_support(player_pos, allies);
        }
        if !enemies.is_empty() { return self.ai_aggressive(enemies); }
        let dx = player_pos.0 as i32 - self.x as i32;
        let dy = player_pos.1 as i32 - self.y as i32;
        let dist = ((dx * dx + dy * dy) as f32).sqrt();
        if dist > 2.0 { return CompanionAIAction::Move(dx.signum(), dy.signum()); }
        CompanionAIAction::Wait
    }

    fn ai_passive(&self, player_pos: (usize, usize)) -> CompanionAIAction {
        let dx = player_pos.0 as i32 - self.x as i32;
        let dy = player_pos.1 as i32 - self.y as i32;
        let dist = ((dx * dx + dy * dy) as f32).sqrt();
        if dist > 2.0 { return CompanionAIAction::Move(dx.signum(), dy.signum()); }
        CompanionAIAction::Wait
    }

    fn find_nearest_enemy<'a>(&self, enemies: &'a [Enemy]) -> Option<&'a Enemy> {
        enemies.iter().filter(|e| e.is_alive()).min_by_key(|e| {
            let dx = e.x as i32 - self.x as i32;
            let dy = e.y as i32 - self.y as i32;
            dx * dx + dy * dy
        })
    }

    pub fn symbol(&self) -> char {
        if !self.is_alive() { return 'x'; }
        match self.class {
            CharacterClass::Warrior => 'W', CharacterClass::Mage => 'M', CharacterClass::Rogue => 'R',
            CharacterClass::Paladin => 'P', CharacterClass::Ranger => 'A', CharacterClass::Necromancer => 'N',
        }
    }

    pub fn color(&self) -> Color { if !self.is_alive() { Color::DarkGrey } else { self.species.color() } }
}

/// The party system that manages all companions
#[derive(Clone, Serialize, Deserialize)]
pub struct Party {
    pub companions: Vec<Companion>,
    pub formation: PartyFormation,
    pub max_size: usize,
    pub recruited_ids: HashSet<u32>,
    pub morale: i32,
    pub supplies: u32,
}

impl Party {
    pub fn new(base_charisma: i32) -> Self {
        Self {
            companions: Vec::new(), formation: PartyFormation::Standard,
            max_size: Self::max_party_size(base_charisma),
            recruited_ids: HashSet::new(), morale: 100, supplies: 10,
        }
    }

    pub fn max_party_size(charisma: i32) -> usize { (2 + (charisma / 5) as usize).min(6) }
    pub fn update_max_size(&mut self, charisma: i32) { self.max_size = Self::max_party_size(charisma); }
    pub fn can_recruit(&self) -> bool { self.companions.iter().filter(|c| c.is_alive()).count() < self.max_size }

    pub fn recruit(&mut self, mut companion: Companion) -> Result<(), &'static str> {
        if !self.can_recruit() { return Err("Party is full!"); }
        if self.recruited_ids.contains(&companion.id) { return Err("Already recruited this companion!"); }
        companion.recruited = true;
        self.recruited_ids.insert(companion.id);
        self.companions.push(companion);
        self.update_morale(10);
        Ok(())
    }

    pub fn dismiss(&mut self, companion_id: u32) -> Option<Companion> {
        if let Some(pos) = self.companions.iter().position(|c| c.id == companion_id) {
            let companion = self.companions.remove(pos);
            self.update_morale(-15);
            Some(companion)
        } else { None }
    }

    pub fn get_companion(&self, id: u32) -> Option<&Companion> { self.companions.iter().find(|c| c.id == id) }
    pub fn get_companion_mut(&mut self, id: u32) -> Option<&mut Companion> { self.companions.iter_mut().find(|c| c.id == id) }
    pub fn active_companions(&self) -> Vec<&Companion> { self.companions.iter().filter(|c| c.is_conscious()).collect() }

    pub fn set_formation(&mut self, formation: PartyFormation) {
        self.formation = formation;
        let positions = formation.positions();
        for (i, companion) in self.companions.iter_mut().enumerate() {
            if i < positions.len() { companion.formation_position = positions[i]; }
        }
    }

    pub fn update_morale(&mut self, change: i32) { self.morale = (self.morale + change).clamp(0, 150); }

    pub fn morale_modifier(&self) -> f32 {
        match self.morale { 0..=25 => 0.75, 26..=50 => 0.9, 51..=100 => 1.0, 101..=125 => 1.1, _ => 1.2 }
    }

    pub fn tick(&mut self) -> Vec<String> {
        let mut messages = Vec::new();
        for companion in &mut self.companions {
            messages.extend(companion.tick_status_effects());
            companion.tick_cooldowns();
        }
        if self.companions.iter().any(|c| c.personality == Personality::Cheerful && c.is_alive()) && self.morale < 100 {
            self.morale += 1;
        }
        messages
    }

    pub fn total_attack_bonus(&self) -> i32 { (self.formation.attack_bonus() as f32 * self.morale_modifier()) as i32 }
    pub fn total_defense_bonus(&self) -> i32 { (self.formation.defense_bonus() as f32 * self.morale_modifier()) as i32 }

    pub fn has_ability(&self, ability: CompanionAbility) -> bool {
        self.companions.iter().filter(|c| c.is_conscious()).any(|c| c.special_abilities.contains(&ability))
    }

    pub fn distribute_xp(&mut self, total_xp: u32) -> Vec<(String, bool)> {
        let active_count = self.active_companions().len();
        if active_count == 0 { return Vec::new(); }
        let xp_each = total_xp / active_count as u32;
        self.companions.iter_mut().filter(|c| c.is_conscious()).map(|c| {
            let leveled = c.gain_xp(xp_each);
            (c.name.clone(), leveled)
        }).collect()
    }

    pub fn distribute_relationship_xp(&mut self, total_xp: u32) {
        let xp_each = total_xp / self.companions.len().max(1) as u32;
        for companion in &mut self.companions { companion.gain_relationship_xp(xp_each); }
    }
}

impl Default for Party {
    fn default() -> Self { Self::new(10) }
}

// ============================================================================
// PREDEFINED COMPANIONS (30+ unique characters)
// ============================================================================

pub fn create_all_companions() -> Vec<Companion> {
    vec![
        // === WARRIORS ===
        Companion::new(1, "Thorin", "the Unbreakable", CharacterClass::Warrior, CompanionSpecies::Dwarf, Personality::Brave, 5)
            .with_backstory("A legendary dwarven warrior who lost his clan to a dragon. Seeks vengeance and glory.")
            .with_dialogue("Hail, adventurer! My axe thirsts for battle!", "FOR KHAZAD-DUM!", "Another victory for the dwarves!", "The mountain... calls me home...")
            .with_abilities(vec![CompanionAbility::Cleave, CompanionAbility::Taunt, CompanionAbility::ShieldAlly])
            .with_quest(CompanionQuest::new(CompanionQuestType::Revenge { target_name: "Dragon".to_string() }, "Help Thorin slay the dragon that destroyed his clan")),

        Companion::new(2, "Valeria", "the Lioness", CharacterClass::Warrior, CompanionSpecies::Human, Personality::Protective, 6)
            .with_backstory("Former captain of the Royal Guard, disgraced by a false accusation. Fights to restore her honor.")
            .with_dialogue("My sword is yours, if your cause is just.", "By my honor!", "Justice prevails!", "My... honor... is restored...")
            .with_abilities(vec![CompanionAbility::ShieldAlly, CompanionAbility::BattleCry, CompanionAbility::Execute])
            .with_quest(CompanionQuest::new(CompanionQuestType::Redemption { good_deeds_needed: 20 }, "Help Valeria restore her honor through noble deeds")),

        Companion::new(3, "Grimjaw", "the Scarred", CharacterClass::Warrior, CompanionSpecies::Orc, Personality::Vengeful, 7)
            .with_backstory("An orc chieftain's son, scarred by humans. Seeks to prove orcs can be honorable.")
            .with_dialogue("You... not afraid of orc? Good. Grimjaw respect that.", "BLOOD FOR HONOR!", "Grimjaw strong! Grimjaw win!", "Grimjaw... die... standing...")
            .with_abilities(vec![CompanionAbility::Flurry, CompanionAbility::Execute, CompanionAbility::BattleCry])
            .with_quest(CompanionQuest::new(CompanionQuestType::ProveWorth { kills_needed: 50 }, "Help Grimjaw prove his worth through combat")),

        // === MAGES ===
        Companion::new(4, "Seraphina", "the Starweaver", CharacterClass::Mage, CompanionSpecies::Elf, Personality::Wise, 6)
            .with_backstory("An elven archmage who has lived for 500 years. Seeks a forbidden spell that could save her dying homeland.")
            .with_dialogue("The stars have foretold our meeting, young one.", "By the ancient powers!", "As the prophecy foretold.", "My magic... returns to the stars...")
            .with_abilities(vec![CompanionAbility::CompFireball, CompanionAbility::IceStorm, CompanionAbility::TimeStop])
            .with_romance_dialogue(vec!["In five centuries, I have never felt... this.", "Our souls dance like twin stars.", "Time means nothing when I am with you."])
            .with_quest(CompanionQuest::new(CompanionQuestType::AncientArtifact { item_name: "Tome of Infinite Knowledge".to_string() }, "Help Seraphina find the Tome of Infinite Knowledge")),

        Companion::new(5, "Mordecai", "the Flame", CharacterClass::Mage, CompanionSpecies::Tiefling, Personality::Ambitious, 5)
            .with_backstory("A tiefling pyromancer shunned for his demonic heritage. Craves power to show the world his worth.")
            .with_dialogue("Do not fear the flames within me... fear what I do with them.", "BURN IN HELLFIRE!", "See what I can do? Imagine what's next!", "The flames... consume me at last...")
            .with_abilities(vec![CompanionAbility::CompFireball, CompanionAbility::DragonBreath, CompanionAbility::UnholyMight])
            .with_quest(CompanionQuest::new(CompanionQuestType::MasterSkill { skill_uses: 100 }, "Help Mordecai master his fire magic")),

        Companion::new(6, "Whisper", "the Void", CharacterClass::Mage, CompanionSpecies::Spirit, Personality::Mysterious, 8)
            .with_backstory("A spirit trapped between worlds. Seeks the anchor that binds it to this realm.")
            .with_dialogue("I am... here. And not here. Can you... hear me?", "From the void, I strike!", "The living... are so fragile.", "At last... peace...")
            .with_abilities(vec![CompanionAbility::CompTeleport, CompanionAbility::TimeStop, CompanionAbility::CompVanish])
            .with_quest(CompanionQuest::new(CompanionQuestType::LostFamily { dungeon_level: 25 }, "Help Whisper find its anchor in the depths")),

        // === ROGUES ===
        Companion::new(7, "Shadow", "the Silent", CharacterClass::Rogue, CompanionSpecies::HalfElf, Personality::Stoic, 5)
            .with_backstory("An assassin who abandoned her guild after they ordered her to kill a child. Now hunted by her former allies.")
            .with_dialogue("I work in silence. Don't ask questions.", "You won't see me coming.", "Target eliminated.", "The shadows... welcome me...")
            .with_abilities(vec![CompanionAbility::CompBackstab, CompanionAbility::CompVanish, CompanionAbility::Poison])
            .with_romance_dialogue(vec!["I've never let anyone get close. You're... different.", "For you, I would come out of the shadows.", "I was death. You gave me life."])
            .with_quest(CompanionQuest::new(CompanionQuestType::Revenge { target_name: "Guild Master".to_string() }, "Help Shadow defeat her former guild")),

        Companion::new(8, "Felix", "the Lucky", CharacterClass::Rogue, CompanionSpecies::Goblin, Personality::Cheerful, 4)
            .with_backstory("A goblin who was kicked out of his tribe for being 'too nice'. Believes luck will guide him to fortune.")
            .with_dialogue("Hey-hey! Felix is best friend you ever have! Felix promise!", "Shiny things for Felix!", "Felix win! Felix ALWAYS win! Hehehehe!", "Felix... not so lucky after all...")
            .with_abilities(vec![CompanionAbility::Steal, CompanionAbility::TreasureHunter, CompanionAbility::Ambush])
            .with_quest(CompanionQuest::new(CompanionQuestType::AncientArtifact { item_name: "Lucky Coin".to_string() }, "Help Felix find the legendary Lucky Coin")),

        Companion::new(9, "Raven", "the Blade Dancer", CharacterClass::Rogue, CompanionSpecies::Human, Personality::Romantic, 6)
            .with_backstory("A notorious jewel thief who steals only from the corrupt. Leaves a black feather at every heist.")
            .with_dialogue("Care to dance with danger, darling?", "Like poetry in motion!", "Another masterpiece complete!", "One last... dance...")
            .with_abilities(vec![CompanionAbility::Disarm, CompanionAbility::Flurry, CompanionAbility::CompVanish])
            .with_romance_dialogue(vec!["I steal many things, but you've stolen something from me.", "For you, I'd give up all the jewels in the world.", "Let's write our story together, my love."])
            .with_quest(CompanionQuest::new(CompanionQuestType::AncientArtifact { item_name: "Heart of the Ocean".to_string() }, "Help Raven steal the legendary Heart of the Ocean")),

        // === PALADINS ===
        Companion::new(10, "Sir Aldric", "the Dawn", CharacterClass::Paladin, CompanionSpecies::Human, Personality::Kind, 6)
            .with_backstory("A holy knight who lost his faith after his temple was destroyed. Seeks to believe again.")
            .with_dialogue("The light fades, but I still remember its warmth.", "May the dawn come again!", "Perhaps... there is still hope.", "I see... the light... at last...")
            .with_abilities(vec![CompanionAbility::HealingTouch, CompanionAbility::DivineSmite, CompanionAbility::Sanctuary])
            .with_quest(CompanionQuest::new(CompanionQuestType::Homecoming { destination_level: 30 }, "Help Sir Aldric find the source of divine light in the depths")),

        Companion::new(11, "Isolde", "the Redeemer", CharacterClass::Paladin, CompanionSpecies::HalfElf, Personality::Protective, 7)
            .with_backstory("A fallen paladin seeking redemption after accidentally causing innocents' deaths in righteous fury.")
            .with_dialogue("I will protect you, as I failed to protect them.", "Not again! Never again!", "Perhaps I can atone...", "Forgive me... I tried...")
            .with_abilities(vec![CompanionAbility::ShieldAlly, CompanionAbility::MassHeal, CompanionAbility::Resurrect])
            .with_romance_dialogue(vec!["You see past my sins. How?", "With you, I feel worthy of love again.", "My heart beats only for you."])
            .with_quest(CompanionQuest::new(CompanionQuestType::Redemption { good_deeds_needed: 30 }, "Help Isolde atone through saving lives")),

        Companion::new(12, "Brother Marcus", "the Heretic", CharacterClass::Paladin, CompanionSpecies::Human, Personality::Foolish, 5)
            .with_backstory("An eccentric monk who believes the gods speak to him through cheese. Surprisingly effective healer.")
            .with_dialogue("The cheddar has spoken! We must journey forth!", "In the name of the sacred brie!", "The gouda was right all along!", "The... cheese... was silent...")
            .with_abilities(vec![CompanionAbility::HealingTouch, CompanionAbility::Bless, CompanionAbility::Purify])
            .with_quest(CompanionQuest::new(CompanionQuestType::AncientArtifact { item_name: "Divine Cheese Wheel".to_string() }, "Help Brother Marcus find the legendary Divine Cheese")),

        // === RANGERS ===
        Companion::new(13, "Kira", "Wolfheart", CharacterClass::Ranger, CompanionSpecies::Beastkin, Personality::Loyal, 5)
            .with_backstory("A wolf-kin ranger whose pack was killed by hunters. Her wolf companion Ghost is her only family.")
            .with_dialogue("The pack is everything. Will you be pack?", "For the pack!", "The hunt... is good.", "Ghost... stay with them...")
            .with_abilities(vec![CompanionAbility::Summon, CompanionAbility::BeastForm, CompanionAbility::Scout])
            .with_romance_dialogue(vec!["I've never chosen a mate before. My heart chooses you.", "In wolf terms, we are bonded for life now.", "You are my alpha. My everything."])
            .with_quest(CompanionQuest::new(CompanionQuestType::Revenge { target_name: "Hunter".to_string() }, "Help Kira avenge her fallen pack")),

        Companion::new(14, "Ember", "the Wild", CharacterClass::Ranger, CompanionSpecies::Fairy, Personality::Cheerful, 4)
            .with_backstory("A mischievous forest fairy who got bored with her grove. Everything is an adventure!")
            .with_dialogue("Ooooh, you look like FUN! Can I come? Pleeeease?", "Wheeeee! Pew pew pew!", "That was AMAZING! Again! Again!", "Oh no... the light is... pretty...")
            .with_abilities(vec![CompanionAbility::Scout, CompanionAbility::NatureWrath, CompanionAbility::CompTeleport])
            .with_quest(CompanionQuest::new(CompanionQuestType::ProveWorth { kills_needed: 25 }, "Help Ember prove she can be a real adventurer")),

        Companion::new(15, "Hawk", "the Silent Arrow", CharacterClass::Ranger, CompanionSpecies::Elf, Personality::Stoic, 7)
            .with_backstory("An elven master archer who took a vow of silence after failing to save his love. Speaks through actions.")
            .with_dialogue("...", "*Draws bow with grim determination*", "*A rare, subtle nod of approval*", "*Closes eyes peacefully*")
            .with_abilities(vec![CompanionAbility::Ambush, CompanionAbility::Disarm, CompanionAbility::Scout])
            .with_quest(CompanionQuest::new(CompanionQuestType::LostFamily { dungeon_level: 20 }, "Help Hawk find what happened to his lost love")),

        // === NECROMANCERS ===
        Companion::new(16, "Morticia", "the Grave Whisperer", CharacterClass::Necromancer, CompanionSpecies::Vampire, Personality::Kind, 6)
            .with_backstory("A vampire necromancer who only raises the willing dead. She sees death as another form of life.")
            .with_dialogue("The dead are not gone - merely quiet. I give them voice.", "Rise, friends! One more dance!", "See? Death need not be cruel.", "At last... I understand... the silence...")
            .with_abilities(vec![CompanionAbility::Summon, CompanionAbility::UnholyMight, CompanionAbility::CompRegeneration])
            .with_romance_dialogue(vec!["I am cold, yet you warm me.", "An eternity alone... and then I found you.", "I will love you beyond death itself."])
            .with_quest(CompanionQuest::new(CompanionQuestType::LostFamily { dungeon_level: 15 }, "Help Morticia find her husband's spirit")),

        Companion::new(17, "Bones", "the Eternal", CharacterClass::Necromancer, CompanionSpecies::Undead, Personality::Humble, 8)
            .with_backstory("An ancient lich who has forgotten why he became undead. Helps adventurers hoping to remember.")
            .with_dialogue("I have... forgotten much. Perhaps you can help me remember?", "Death is... familiar.", "Yes... this feels... right.", "Finally... I remember... goodbye...")
            .with_abilities(vec![CompanionAbility::Summon, CompanionAbility::Resurrect, CompanionAbility::ArcaneMastery])
            .with_quest(CompanionQuest::new(CompanionQuestType::Homecoming { destination_level: 28 }, "Help Bones find his phylactery and his memories")),

        Companion::new(18, "Lilith", "the Blood Witch", CharacterClass::Necromancer, CompanionSpecies::Demon, Personality::Cruel, 7)
            .with_backstory("A demon who rebelled against the Hells. Uses dark magic for her own twisted sense of justice.")
            .with_dialogue("Don't trust me. I won't trust you. Let's work together.", "Scream for me!", "Delicious suffering!", "Even demons... can feel... pain...")
            .with_abilities(vec![CompanionAbility::UnholyMight, CompanionAbility::Poison, CompanionAbility::SoulLink])
            .with_quest(CompanionQuest::new(CompanionQuestType::Revenge { target_name: "Demon Lord".to_string() }, "Help Lilith destroy her former master")),

        // === MORE UNIQUE COMPANIONS ===
        Companion::new(19, "Granite", "the Living Mountain", CharacterClass::Warrior, CompanionSpecies::Golem, Personality::Protective, 10)
            .with_backstory("An ancient golem awakened to protect. Speaks little, shields much.")
            .with_dialogue("Granite... protect.", "STONE ENDURES!", "All... safe.", "Granite... crumbles...")
            .with_abilities(vec![CompanionAbility::ShieldAlly, CompanionAbility::Taunt, CompanionAbility::Sanctuary]),

        Companion::new(20, "Celeste", "the Oracle", CharacterClass::Mage, CompanionSpecies::Human, Personality::Mysterious, 6)
            .with_backstory("A blind seer who sees the future. Her prophecies are always true, but often misunderstood.")
            .with_dialogue("I have seen your fate... and chosen to walk beside you.", "As I foresaw!", "The future... shifts.", "I see... everything now... beautiful...")
            .with_abilities(vec![CompanionAbility::TimeStop, CompanionAbility::Scout, CompanionAbility::Bless])
            .with_romance_dialogue(vec!["I foresaw loving you. I did not foresee how deeply.", "Every future I see has you in it.", "Our fates are intertwined eternally."]),

        Companion::new(21, "Patches", "the Survivor", CharacterClass::Rogue, CompanionSpecies::Human, Personality::Greedy, 3)
            .with_backstory("A scoundrel who has survived everything. Knows every dirty trick in the book.")
            .with_dialogue("You look like you could use someone with... flexible morals.", "It's not cowardice, it's TACTICS!", "I'll take my cut now, thanks.", "Should've... seen this coming...")
            .with_abilities(vec![CompanionAbility::Steal, CompanionAbility::TreasureHunter, CompanionAbility::CompVanish]),

        Companion::new(22, "Aurora", "the Dragon Princess", CharacterClass::Mage, CompanionSpecies::Dragonian, Personality::Ambitious, 8)
            .with_backstory("A dragonborn princess in exile. Seeks the throne that was stolen from her.")
            .with_dialogue("Bow not to me - but know that I am royalty.", "FEEL MY DRAGON'S WRATH!", "As it should be.", "My kingdom... will rise...")
            .with_abilities(vec![CompanionAbility::DragonBreath, CompanionAbility::CompFireball, CompanionAbility::BattleCry])
            .with_romance_dialogue(vec!["You dare to court a princess? ...I like your courage.", "You would make a worthy consort.", "My heart and my kingdom - both yours."]),

        Companion::new(23, "Pip", "the Brave", CharacterClass::Warrior, CompanionSpecies::Goblin, Personality::Brave, 2)
            .with_backstory("The smallest goblin in his tribe. Dreams of being a hero and proving size doesn't matter.")
            .with_dialogue("P-Pip is brave! Pip will help!", "FOR GLORY! *squeak*", "Pip... Pip did it!?", "Pip... was brave... right?")
            .with_abilities(vec![CompanionAbility::Ambush, CompanionAbility::Flurry])
            .with_quest(CompanionQuest::new(CompanionQuestType::ProveWorth { kills_needed: 30 }, "Help Pip become a true hero")),

        Companion::new(24, "Tempest", "Storm's Daughter", CharacterClass::Mage, CompanionSpecies::HalfElf, Personality::Brave, 6)
            .with_backstory("Child of a mortal and a storm spirit. Lightning courses through her veins.")
            .with_dialogue("The storm is always with me. Can you handle the thunder?", "LIGHTNING STRIKES!", "The storm passes, victorious!", "Return... to the storm...")
            .with_abilities(vec![CompanionAbility::CompLightning, CompanionAbility::CompTeleport, CompanionAbility::IceStorm]),

        Companion::new(25, "Jack", "the Wanderer", CharacterClass::Ranger, CompanionSpecies::Human, Personality::Humble, 4)
            .with_backstory("A simple farmer who lost everything to monsters. Now wanders, helping others avoid his fate.")
            .with_dialogue("I'm just a farmer with a bow. But I won't let them hurt anyone else.", "This is for my family!", "We did it. They're safe now.", "Tell them... I tried...")
            .with_abilities(vec![CompanionAbility::Scout, CompanionAbility::Ambush, CompanionAbility::TreasureHunter]),

        Companion::new(26, "Nyx", "the Nightmare", CharacterClass::Rogue, CompanionSpecies::Demon, Personality::Mysterious, 7)
            .with_backstory("A nightmare demon who feeds on fear. Grew tired of tormenting sleepers and seeks real challenges.")
            .with_dialogue("I've walked your nightmares. Now I'll walk beside you... if you dare.", "Fear me!", "Your fear... it tastes like victory.", "Even nightmares... end...")
            .with_abilities(vec![CompanionAbility::CompVanish, CompanionAbility::Poison, CompanionAbility::UnholyMight]),

        Companion::new(27, "Brother Sun", "the Radiant", CharacterClass::Paladin, CompanionSpecies::Human, Personality::Cheerful, 5)
            .with_backstory("A sun priest who believes joy is the greatest weapon against darkness. Annoyingly optimistic.")
            .with_dialogue("Praise the sun! What a glorious day for adventure!", "LET THE LIGHT SHINE!", "Haha! Magnificent! Simply magnificent!", "The sun... it's so... warm...")
            .with_abilities(vec![CompanionAbility::HealingTouch, CompanionAbility::Bless, CompanionAbility::DivineSmite]),

        Companion::new(28, "Vera", "the Iron Maiden", CharacterClass::Warrior, CompanionSpecies::Human, Personality::Stoic, 7)
            .with_backstory("A legendary gladiator who won her freedom. Now fights for those who cannot fight for themselves.")
            .with_dialogue("Save your words. Show me what you can do.", "No retreat.", "Another victory.", "A good... death...")
            .with_abilities(vec![CompanionAbility::Execute, CompanionAbility::Flurry, CompanionAbility::Taunt]),

        Companion::new(29, "Sage Yuki", "the Frost Flower", CharacterClass::Mage, CompanionSpecies::Elf, Personality::Cautious, 6)
            .with_backstory("A snow elf mage who has never left her frozen homeland. Seeks to understand the world below.")
            .with_dialogue("The world... is much warmer than I expected.", "Winter's embrace!", "Like snow settling peacefully.", "I return... to the eternal... winter...")
            .with_abilities(vec![CompanionAbility::IceStorm, CompanionAbility::Sanctuary, CompanionAbility::CompRegeneration])
            .with_romance_dialogue(vec!["My heart was ice. You melted it.", "I never knew warmth could feel so... right.", "Stay with me. Forever."]),

        Companion::new(30, "Rex", "the Hound", CharacterClass::Ranger, CompanionSpecies::Beastkin, Personality::Loyal, 5)
            .with_backstory("A loyal dog-kin who was abandoned by his master. Seeks a new person to serve.")
            .with_dialogue("Rex will follow! Rex is good boy! ...Rex IS good boy, right?", "Bad people! Rex bite!", "Rex help! Rex GOOD BOY!", "Rex... sorry... Rex tried...")
            .with_abilities(vec![CompanionAbility::Scout, CompanionAbility::Flurry, CompanionAbility::TreasureHunter])
            .with_quest(CompanionQuest::new(CompanionQuestType::Confession { relationship_needed: Relationship::Loyal }, "Show Rex he truly is a good boy")),

        Companion::new(31, "Obsidian", "the Fallen Angel", CharacterClass::Paladin, CompanionSpecies::Spirit, Personality::Stoic, 9)
            .with_backstory("An angel who fell from grace to save mortals. Now serves penance in the mortal realm.")
            .with_dialogue("I gave up heaven. I have no regrets.", "By my fallen grace!", "Perhaps... redemption is possible.", "I return... to the light...")
            .with_abilities(vec![CompanionAbility::DivineSmite, CompanionAbility::HealingTouch, CompanionAbility::Resurrect])
            .with_romance_dialogue(vec!["Angels do not love. But I am no longer an angel.", "For you, I fell. For you, I would fall again.", "My eternity belongs to you."]),

        Companion::new(32, "Zara", "the Chain Breaker", CharacterClass::Rogue, CompanionSpecies::Human, Personality::Vengeful, 5)
            .with_backstory("A former slave who escaped and now frees others. Burns with hatred for slavers.")
            .with_dialogue("No one should be in chains. NO ONE.", "FREEDOM!", "Another cage... broken.", "I die... free...")
            .with_abilities(vec![CompanionAbility::Disarm, CompanionAbility::CompBackstab, CompanionAbility::Ambush])
            .with_quest(CompanionQuest::new(CompanionQuestType::Revenge { target_name: "Slaver".to_string() }, "Help Zara destroy the slave trade")),

        Companion::new(33, "Echo", "the Mirror", CharacterClass::Mage, CompanionSpecies::Spirit, Personality::Foolish, 5)
            .with_backstory("A reflection that gained sentience. Struggles to understand reality and its own existence.")
            .with_dialogue("Am I real? Are you? What is 'real' anyway?", "Reflect this!", "Did I do that? Or did you?", "I was... just a reflection... after all...")
            .with_abilities(vec![CompanionAbility::CompTeleport, CompanionAbility::CompVanish, CompanionAbility::TimeStop]),
    ]
}

/// Get a random selection of companions for a dungeon level
pub fn get_recruitable_companions(level: u32, recruited: &HashSet<u32>, rng: &mut impl Rng) -> Vec<Companion> {
    let all = create_all_companions();
    let available: Vec<_> = all.into_iter()
        .filter(|c| !recruited.contains(&c.id))
        .filter(|c| c.level <= level + 3 && c.level + 5 >= level)
        .collect();
    let count = rng.gen_range(1..=3.min(available.len().max(1)));
    available.into_iter().choose_multiple(rng, count)
}
