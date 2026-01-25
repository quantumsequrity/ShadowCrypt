//! Guild System
//!
//! 12 guilds players can join with ranks, quests, perks, and guild wars.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The 12 major guilds
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Guild {
    /// Warriors guild
    BladesBrotherhood,
    /// Mages guild
    ArcanumCircle,
    /// Thieves guild
    ShadowSyndicate,
    /// Assassins guild
    SilentBlade,
    /// Merchants guild
    GoldenCoin,
    /// Crafters guild
    MasterArtisans,
    /// Adventurers guild
    PathfindersUnion,
    /// Healers guild
    WhiteRose,
    /// Necromancers guild
    BoneCollectors,
    /// Monster hunters
    BeastSlayers,
    /// Alchemists guild
    PhilosophersStone,
    /// Explorers guild
    CartographersSociety,
}

impl Guild {
    pub fn all() -> &'static [Guild] {
        &[
            Self::BladesBrotherhood, Self::ArcanumCircle, Self::ShadowSyndicate,
            Self::SilentBlade, Self::GoldenCoin, Self::MasterArtisans,
            Self::PathfindersUnion, Self::WhiteRose, Self::BoneCollectors,
            Self::BeastSlayers, Self::PhilosophersStone, Self::CartographersSociety,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::BladesBrotherhood => "The Blades Brotherhood",
            Self::ArcanumCircle => "Arcanum Circle",
            Self::ShadowSyndicate => "Shadow Syndicate",
            Self::SilentBlade => "The Silent Blade",
            Self::GoldenCoin => "Golden Coin Trading Co.",
            Self::MasterArtisans => "Master Artisans Guild",
            Self::PathfindersUnion => "Pathfinders Union",
            Self::WhiteRose => "Order of the White Rose",
            Self::BoneCollectors => "The Bone Collectors",
            Self::BeastSlayers => "Beast Slayers League",
            Self::PhilosophersStone => "Philosopher's Stone Society",
            Self::CartographersSociety => "Cartographers Society",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::BladesBrotherhood => "A brotherhood of warriors dedicated to martial excellence.",
            Self::ArcanumCircle => "Mages studying the arcane arts and magical secrets.",
            Self::ShadowSyndicate => "Thieves and rogues operating in the shadows.",
            Self::SilentBlade => "Assassins who eliminate targets with deadly precision.",
            Self::GoldenCoin => "Merchants and traders seeking profit across realms.",
            Self::MasterArtisans => "Crafters creating the finest weapons and armor.",
            Self::PathfindersUnion => "Adventurers exploring dungeons for glory and treasure.",
            Self::WhiteRose => "Healers and priests dedicated to saving lives.",
            Self::BoneCollectors => "Necromancers studying death and undeath.",
            Self::BeastSlayers => "Hunters specializing in slaying monsters.",
            Self::PhilosophersStone => "Alchemists seeking the secrets of transmutation.",
            Self::CartographersSociety => "Explorers mapping the unknown reaches of the world.",
        }
    }

    pub fn ranks(&self) -> &'static [GuildRank] {
        &[
            GuildRank::Initiate,
            GuildRank::Apprentice,
            GuildRank::Journeyman,
            GuildRank::Adept,
            GuildRank::Expert,
            GuildRank::Master,
            GuildRank::GrandMaster,
            GuildRank::GuildLeader,
        ]
    }

    pub fn perks(&self) -> Vec<GuildPerk> {
        match self {
            Self::BladesBrotherhood => vec![
                GuildPerk::StatBonus { stat: "Attack".into(), amount: 5 },
                GuildPerk::SkillUnlock { skill: "Cleave".into() },
                GuildPerk::ShopDiscount { percent: 10, category: "Weapons".into() },
            ],
            Self::ArcanumCircle => vec![
                GuildPerk::StatBonus { stat: "Mana".into(), amount: 20 },
                GuildPerk::SpellUnlock { spell: "Arcane Missile".into() },
                GuildPerk::ManaRegen { percent: 10 },
            ],
            Self::ShadowSyndicate => vec![
                GuildPerk::StatBonus { stat: "Speed".into(), amount: 5 },
                GuildPerk::SkillUnlock { skill: "Pickpocket".into() },
                GuildPerk::FenceAccess,
            ],
            Self::SilentBlade => vec![
                GuildPerk::StatBonus { stat: "CritChance".into(), amount: 10 },
                GuildPerk::SkillUnlock { skill: "Assassinate".into() },
                GuildPerk::ContractAccess,
            ],
            Self::GoldenCoin => vec![
                GuildPerk::ShopDiscount { percent: 20, category: "All".into() },
                GuildPerk::BetterPrices { sell_bonus: 15 },
                GuildPerk::BankAccess,
            ],
            Self::MasterArtisans => vec![
                GuildPerk::CraftingBonus { percent: 25 },
                GuildPerk::RecipeUnlock { recipe: "Masterwork".into() },
                GuildPerk::WorkshopAccess,
            ],
            Self::PathfindersUnion => vec![
                GuildPerk::XPBonus { percent: 15 },
                GuildPerk::MapReveal { percent: 10 },
                GuildPerk::ContractAccess,
            ],
            Self::WhiteRose => vec![
                GuildPerk::HealingBonus { percent: 25 },
                GuildPerk::SpellUnlock { spell: "Greater Heal".into() },
                GuildPerk::FreeHealing,
            ],
            Self::BoneCollectors => vec![
                GuildPerk::UndeadBonus { percent: 30 },
                GuildPerk::SpellUnlock { spell: "Raise Greater Undead".into() },
                GuildPerk::CorpseHarvest,
            ],
            Self::BeastSlayers => vec![
                GuildPerk::MonsterDamage { percent: 20 },
                GuildPerk::TrophyBonus { percent: 50 },
                GuildPerk::BountyAccess,
            ],
            Self::PhilosophersStone => vec![
                GuildPerk::AlchemyBonus { percent: 30 },
                GuildPerk::RecipeUnlock { recipe: "Elixir".into() },
                GuildPerk::LabAccess,
            ],
            Self::CartographersSociety => vec![
                GuildPerk::MapReveal { percent: 25 },
                GuildPerk::TrapDetection { percent: 20 },
                GuildPerk::FastTravel,
            ],
        }
    }

    pub fn entry_fee(&self) -> u32 {
        match self {
            Self::GoldenCoin => 500,
            Self::SilentBlade => 300,
            Self::ArcanumCircle => 200,
            _ => 100,
        }
    }

    pub fn rival_guilds(&self) -> Vec<Guild> {
        match self {
            Self::BladesBrotherhood => vec![Self::SilentBlade],
            Self::ShadowSyndicate => vec![Self::SilentBlade, Self::GoldenCoin],
            Self::SilentBlade => vec![Self::BladesBrotherhood, Self::WhiteRose],
            Self::WhiteRose => vec![Self::BoneCollectors, Self::SilentBlade],
            Self::BoneCollectors => vec![Self::WhiteRose],
            _ => vec![],
        }
    }
}

/// Guild ranks
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum GuildRank {
    Initiate = 0,
    Apprentice = 1,
    Journeyman = 2,
    Adept = 3,
    Expert = 4,
    Master = 5,
    GrandMaster = 6,
    GuildLeader = 7,
}

impl GuildRank {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Initiate => "Initiate",
            Self::Apprentice => "Apprentice",
            Self::Journeyman => "Journeyman",
            Self::Adept => "Adept",
            Self::Expert => "Expert",
            Self::Master => "Master",
            Self::GrandMaster => "Grand Master",
            Self::GuildLeader => "Guild Leader",
        }
    }

    pub fn xp_required(&self) -> u32 {
        match self {
            Self::Initiate => 0,
            Self::Apprentice => 100,
            Self::Journeyman => 300,
            Self::Adept => 700,
            Self::Expert => 1500,
            Self::Master => 3000,
            Self::GrandMaster => 6000,
            Self::GuildLeader => 10000,
        }
    }

    pub fn next_rank(&self) -> Option<GuildRank> {
        match self {
            Self::Initiate => Some(Self::Apprentice),
            Self::Apprentice => Some(Self::Journeyman),
            Self::Journeyman => Some(Self::Adept),
            Self::Adept => Some(Self::Expert),
            Self::Expert => Some(Self::Master),
            Self::Master => Some(Self::GrandMaster),
            Self::GrandMaster => Some(Self::GuildLeader),
            Self::GuildLeader => None,
        }
    }
}

/// Guild perks
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GuildPerk {
    StatBonus { stat: String, amount: i32 },
    SkillUnlock { skill: String },
    SpellUnlock { spell: String },
    ShopDiscount { percent: u32, category: String },
    BetterPrices { sell_bonus: u32 },
    XPBonus { percent: u32 },
    MapReveal { percent: u32 },
    HealingBonus { percent: u32 },
    ManaRegen { percent: u32 },
    CraftingBonus { percent: u32 },
    AlchemyBonus { percent: u32 },
    UndeadBonus { percent: u32 },
    MonsterDamage { percent: u32 },
    TrophyBonus { percent: u32 },
    TrapDetection { percent: u32 },
    RecipeUnlock { recipe: String },
    FenceAccess,
    ContractAccess,
    BankAccess,
    WorkshopAccess,
    LabAccess,
    FreeHealing,
    CorpseHarvest,
    BountyAccess,
    FastTravel,
}

/// Player's guild membership
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuildMembership {
    pub guild: Guild,
    pub rank: GuildRank,
    pub reputation: u32,
    pub quests_completed: u32,
    pub joined_turn: u32,
    pub active_perks: Vec<GuildPerk>,
}

impl GuildMembership {
    pub fn new(guild: Guild, turn: u32) -> Self {
        Self {
            guild,
            rank: GuildRank::Initiate,
            reputation: 0,
            quests_completed: 0,
            joined_turn: turn,
            active_perks: vec![],
        }
    }

    pub fn gain_reputation(&mut self, amount: u32) -> bool {
        self.reputation += amount;
        if let Some(next) = self.rank.next_rank() {
            if self.reputation >= next.xp_required() {
                self.rank = next;
                self.update_perks();
                return true;
            }
        }
        false
    }

    fn update_perks(&mut self) {
        let all_perks = self.guild.perks();
        let perk_index = self.rank as usize;
        self.active_perks = all_perks.into_iter().take(perk_index + 1).collect();
    }
}

/// Guild quest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuildQuest {
    pub id: u32,
    pub guild: Guild,
    pub name: String,
    pub description: String,
    pub objective: GuildObjective,
    pub reputation_reward: u32,
    pub gold_reward: u32,
    pub item_reward: Option<String>,
    pub min_rank: GuildRank,
    pub completed: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GuildObjective {
    KillMonsters { kind: String, count: u32, current: u32 },
    CollectItems { item: String, count: u32, current: u32 },
    ReachLocation { location: String },
    CraftItems { item: String, count: u32, current: u32 },
    EarnGold { amount: u32, current: u32 },
    ExploreFloors { count: u32, current: u32 },
    DefeatBoss { boss: String },
    Custom { description: String, completed: bool },
}

/// Player's guild system
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GuildSystem {
    pub memberships: HashMap<Guild, GuildMembership>,
    pub active_quests: Vec<GuildQuest>,
    pub completed_quests: u32,
    pub primary_guild: Option<Guild>,
}

impl GuildSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn join_guild(&mut self, guild: Guild, turn: u32) -> bool {
        if self.memberships.contains_key(&guild) {
            return false;
        }
        // Check rival guilds
        for rival in guild.rival_guilds() {
            if self.memberships.contains_key(&rival) {
                return false;
            }
        }
        self.memberships.insert(guild, GuildMembership::new(guild, turn));
        if self.primary_guild.is_none() {
            self.primary_guild = Some(guild);
        }
        true
    }

    pub fn leave_guild(&mut self, guild: Guild) {
        self.memberships.remove(&guild);
        if self.primary_guild == Some(guild) {
            self.primary_guild = self.memberships.keys().next().copied();
        }
    }

    pub fn get_membership(&self, guild: Guild) -> Option<&GuildMembership> {
        self.memberships.get(&guild)
    }

    pub fn gain_reputation(&mut self, guild: Guild, amount: u32) -> bool {
        if let Some(membership) = self.memberships.get_mut(&guild) {
            return membership.gain_reputation(amount);
        }
        false
    }

    pub fn total_perks(&self) -> Vec<&GuildPerk> {
        self.memberships.values()
            .flat_map(|m| m.active_perks.iter())
            .collect()
    }
}
