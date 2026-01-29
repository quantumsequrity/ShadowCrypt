//! Necromancy System
//!
//! Comprehensive undead summoning, soul manipulation, and dark magic system
//! featuring 30+ undead types, 8 schools of necromancy, army management,
//! death domains, and progression from Grave Robber to Death God.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// CONSTANTS
// ============================================================================

pub const BASE_CONTROL_LIMIT: u32 = 3;
pub const CONTROL_PER_RANK: u32 = 5;
pub const MAX_ARMY_SIZE: usize = 100;
pub const SOUL_GEM_CAPACITY_SMALL: u32 = 1;
pub const SOUL_GEM_CAPACITY_MEDIUM: u32 = 5;
pub const SOUL_GEM_CAPACITY_LARGE: u32 = 25;
pub const SOUL_GEM_CAPACITY_GRAND: u32 = 100;
pub const PHYLACTERY_SOUL_COST: u32 = 50;

// ============================================================================
// UNDEAD TYPES
// ============================================================================

/// Categories of undead creatures
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UndeadCategory {
    Skeleton,
    Zombie,
    Ghost,
    Vampire,
    Lich,
    Construct,
    Greater,
    Unique,
}

/// All undead types with their variants (30+ types)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UndeadType {
    // Skeletons (5 variants)
    SkeletonWarrior,
    SkeletonArcher,
    SkeletonMage,
    SkeletonKnight,
    SkeletonGiant,

    // Zombies (4 variants)
    ZombieShambler,
    ZombieRunner,
    ZombieBloated,
    ZombiePlague,

    // Ghosts (4 variants)
    GhostSpectre,
    GhostWraith,
    GhostBanshee,
    GhostPoltergeist,

    // Vampires (4 variants)
    VampireFledgling,
    VampireElder,
    VampireAncient,
    VampireLord,

    // Liches (4 variants)
    LichApprentice,
    LichMaster,
    ArchLich,
    DemiLich,

    // Other undead types
    DeathKnight,
    BoneDragon,
    FleshGolem,
    Revenant,
    Mummy,
    MummyLord,
    Draugr,
    DraugrOverlord,
    Wight,
    WightLord,

    // Additional undead for 30+ total
    BoneHorror,
    CorpseBeast,
    ShadowStalker,
    GraveGuardian,
    PlagueBringer,
    SoulReaper,
    NecroticAbomination,
    SkeletalDrake,
}

impl UndeadType {
    pub fn all() -> &'static [UndeadType] {
        &[
            Self::SkeletonWarrior, Self::SkeletonArcher, Self::SkeletonMage,
            Self::SkeletonKnight, Self::SkeletonGiant,
            Self::ZombieShambler, Self::ZombieRunner, Self::ZombieBloated, Self::ZombiePlague,
            Self::GhostSpectre, Self::GhostWraith, Self::GhostBanshee, Self::GhostPoltergeist,
            Self::VampireFledgling, Self::VampireElder, Self::VampireAncient, Self::VampireLord,
            Self::LichApprentice, Self::LichMaster, Self::ArchLich, Self::DemiLich,
            Self::DeathKnight, Self::BoneDragon, Self::FleshGolem, Self::Revenant,
            Self::Mummy, Self::MummyLord, Self::Draugr, Self::DraugrOverlord,
            Self::Wight, Self::WightLord, Self::BoneHorror, Self::CorpseBeast,
            Self::ShadowStalker, Self::GraveGuardian, Self::PlagueBringer, Self::SoulReaper,
            Self::NecroticAbomination, Self::SkeletalDrake,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::SkeletonWarrior => "Skeleton Warrior",
            Self::SkeletonArcher => "Skeleton Archer",
            Self::SkeletonMage => "Skeleton Mage",
            Self::SkeletonKnight => "Skeleton Knight",
            Self::SkeletonGiant => "Skeleton Giant",
            Self::ZombieShambler => "Shambling Zombie",
            Self::ZombieRunner => "Runner Zombie",
            Self::ZombieBloated => "Bloated Zombie",
            Self::ZombiePlague => "Plague Zombie",
            Self::GhostSpectre => "Spectre",
            Self::GhostWraith => "Wraith",
            Self::GhostBanshee => "Banshee",
            Self::GhostPoltergeist => "Poltergeist",
            Self::VampireFledgling => "Fledgling Vampire",
            Self::VampireElder => "Elder Vampire",
            Self::VampireAncient => "Ancient Vampire",
            Self::VampireLord => "Vampire Lord",
            Self::LichApprentice => "Apprentice Lich",
            Self::LichMaster => "Master Lich",
            Self::ArchLich => "Arch-Lich",
            Self::DemiLich => "Demi-Lich",
            Self::DeathKnight => "Death Knight",
            Self::BoneDragon => "Bone Dragon",
            Self::FleshGolem => "Flesh Golem",
            Self::Revenant => "Revenant",
            Self::Mummy => "Mummy",
            Self::MummyLord => "Mummy Lord",
            Self::Draugr => "Draugr",
            Self::DraugrOverlord => "Draugr Overlord",
            Self::Wight => "Wight",
            Self::WightLord => "Wight Lord",
            Self::BoneHorror => "Bone Horror",
            Self::CorpseBeast => "Corpse Beast",
            Self::ShadowStalker => "Shadow Stalker",
            Self::GraveGuardian => "Grave Guardian",
            Self::PlagueBringer => "Plague Bringer",
            Self::SoulReaper => "Soul Reaper",
            Self::NecroticAbomination => "Necrotic Abomination",
            Self::SkeletalDrake => "Skeletal Drake",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::SkeletonWarrior => "Basic melee undead armed with sword and shield.",
            Self::SkeletonArcher => "Ranged undead with deadly accuracy.",
            Self::SkeletonMage => "Skeletal spellcaster with dark magic.",
            Self::SkeletonKnight => "Elite armored skeleton with powerful attacks.",
            Self::SkeletonGiant => "Massive skeleton towering over the battlefield.",
            Self::ZombieShambler => "Slow but resilient walking corpse.",
            Self::ZombieRunner => "Fast and aggressive infected dead.",
            Self::ZombieBloated => "Explodes into toxic gas upon death.",
            Self::ZombiePlague => "Spreads disease with every attack.",
            Self::GhostSpectre => "Ethereal spirit that drains life force.",
            Self::GhostWraith => "Malevolent spirit of pure hatred.",
            Self::GhostBanshee => "Wailing spirit whose scream kills.",
            Self::GhostPoltergeist => "Mischievous spirit that hurls objects.",
            Self::VampireFledgling => "Newly turned vampire, hungry for blood.",
            Self::VampireElder => "Centuries-old vampire with great power.",
            Self::VampireAncient => "Ancient vampire with legendary abilities.",
            Self::VampireLord => "Master of all vampires, nearly immortal.",
            Self::LichApprentice => "Novice lich still learning undeath.",
            Self::LichMaster => "Accomplished lich of great power.",
            Self::ArchLich => "Supreme lich commanding armies of dead.",
            Self::DemiLich => "Transcendent lich existing as pure soul.",
            Self::DeathKnight => "Fallen knight bound to eternal service.",
            Self::BoneDragon => "Reanimated dragon skeleton of immense power.",
            Self::FleshGolem => "Construct stitched from multiple corpses.",
            Self::Revenant => "Vengeful spirit returned for retribution.",
            Self::Mummy => "Preserved corpse wrapped in enchanted bandages.",
            Self::MummyLord => "Ancient pharaoh risen from the tomb.",
            Self::Draugr => "Norse undead warrior from burial mounds.",
            Self::DraugrOverlord => "Powerful draugr chieftain.",
            Self::Wight => "Malevolent undead that drains life energy.",
            Self::WightLord => "Commander of wights with dark powers.",
            Self::BoneHorror => "Amalgamation of bones from many corpses.",
            Self::CorpseBeast => "Monstrous creature made from fused bodies.",
            Self::ShadowStalker => "Undead assassin that moves through shadows.",
            Self::GraveGuardian => "Eternal protector of burial grounds.",
            Self::PlagueBringer => "Carrier of supernatural diseases.",
            Self::SoulReaper => "Harvester of souls for its master.",
            Self::NecroticAbomination => "Ultimate undead creation of darkest magic.",
            Self::SkeletalDrake => "Lesser dragon skeleton, swift and deadly.",
        }
    }

    pub fn category(&self) -> UndeadCategory {
        match self {
            Self::SkeletonWarrior | Self::SkeletonArcher | Self::SkeletonMage |
            Self::SkeletonKnight | Self::SkeletonGiant => UndeadCategory::Skeleton,

            Self::ZombieShambler | Self::ZombieRunner | Self::ZombieBloated |
            Self::ZombiePlague => UndeadCategory::Zombie,

            Self::GhostSpectre | Self::GhostWraith | Self::GhostBanshee |
            Self::GhostPoltergeist | Self::ShadowStalker => UndeadCategory::Ghost,

            Self::VampireFledgling | Self::VampireElder | Self::VampireAncient |
            Self::VampireLord => UndeadCategory::Vampire,

            Self::LichApprentice | Self::LichMaster | Self::ArchLich |
            Self::DemiLich => UndeadCategory::Lich,

            Self::FleshGolem | Self::BoneHorror | Self::CorpseBeast |
            Self::NecroticAbomination => UndeadCategory::Construct,

            Self::BoneDragon | Self::SkeletalDrake => UndeadCategory::Greater,

            Self::DeathKnight | Self::Revenant | Self::Mummy | Self::MummyLord |
            Self::Draugr | Self::DraugrOverlord | Self::Wight | Self::WightLord |
            Self::GraveGuardian | Self::PlagueBringer | Self::SoulReaper => UndeadCategory::Unique,
        }
    }

    pub fn base_stats(&self) -> UndeadStats {
        match self {
            // Skeletons
            Self::SkeletonWarrior => UndeadStats::new(30, 12, 8, 0, 10, 1),
            Self::SkeletonArcher => UndeadStats::new(25, 15, 5, 0, 12, 1),
            Self::SkeletonMage => UndeadStats::new(20, 8, 4, 30, 8, 2),
            Self::SkeletonKnight => UndeadStats::new(50, 18, 15, 0, 8, 3),
            Self::SkeletonGiant => UndeadStats::new(100, 25, 12, 0, 6, 5),

            // Zombies
            Self::ZombieShambler => UndeadStats::new(50, 10, 8, 0, 4, 1),
            Self::ZombieRunner => UndeadStats::new(35, 14, 5, 0, 14, 2),
            Self::ZombieBloated => UndeadStats::new(80, 8, 10, 0, 3, 3),
            Self::ZombiePlague => UndeadStats::new(45, 12, 6, 10, 6, 4),

            // Ghosts
            Self::GhostSpectre => UndeadStats::new(25, 15, 2, 20, 12, 3),
            Self::GhostWraith => UndeadStats::new(40, 20, 3, 30, 14, 5),
            Self::GhostBanshee => UndeadStats::new(35, 10, 2, 50, 10, 6),
            Self::GhostPoltergeist => UndeadStats::new(30, 18, 2, 15, 16, 4),

            // Vampires
            Self::VampireFledgling => UndeadStats::new(60, 18, 10, 20, 14, 5),
            Self::VampireElder => UndeadStats::new(120, 30, 18, 50, 16, 10),
            Self::VampireAncient => UndeadStats::new(200, 45, 25, 80, 18, 15),
            Self::VampireLord => UndeadStats::new(350, 60, 35, 120, 20, 25),

            // Liches
            Self::LichApprentice => UndeadStats::new(80, 15, 8, 100, 10, 8),
            Self::LichMaster => UndeadStats::new(150, 25, 15, 200, 12, 15),
            Self::ArchLich => UndeadStats::new(300, 40, 25, 400, 14, 30),
            Self::DemiLich => UndeadStats::new(500, 60, 40, 800, 16, 50),

            // Others
            Self::DeathKnight => UndeadStats::new(180, 35, 30, 40, 12, 12),
            Self::BoneDragon => UndeadStats::new(400, 55, 40, 100, 18, 35),
            Self::FleshGolem => UndeadStats::new(150, 30, 25, 0, 6, 8),
            Self::Revenant => UndeadStats::new(100, 25, 15, 10, 12, 7),
            Self::Mummy => UndeadStats::new(70, 18, 18, 30, 8, 5),
            Self::MummyLord => UndeadStats::new(180, 35, 30, 80, 10, 15),
            Self::Draugr => UndeadStats::new(65, 20, 16, 0, 10, 4),
            Self::DraugrOverlord => UndeadStats::new(140, 35, 28, 20, 12, 10),
            Self::Wight => UndeadStats::new(55, 18, 12, 15, 12, 4),
            Self::WightLord => UndeadStats::new(120, 30, 22, 40, 14, 10),
            Self::BoneHorror => UndeadStats::new(200, 40, 20, 0, 8, 12),
            Self::CorpseBeast => UndeadStats::new(250, 45, 25, 0, 10, 15),
            Self::ShadowStalker => UndeadStats::new(45, 30, 5, 25, 20, 8),
            Self::GraveGuardian => UndeadStats::new(120, 22, 35, 20, 8, 8),
            Self::PlagueBringer => UndeadStats::new(90, 20, 15, 60, 10, 10),
            Self::SoulReaper => UndeadStats::new(100, 35, 10, 50, 16, 12),
            Self::NecroticAbomination => UndeadStats::new(500, 70, 45, 150, 8, 40),
            Self::SkeletalDrake => UndeadStats::new(150, 35, 20, 30, 20, 15),
        }
    }

    pub fn control_cost(&self) -> u32 {
        self.base_stats().control_cost
    }

    pub fn required_rank(&self) -> NecromancerRank {
        match self {
            Self::SkeletonWarrior | Self::ZombieShambler => NecromancerRank::GraveRobber,
            Self::SkeletonArcher | Self::ZombieRunner | Self::GhostSpectre => NecromancerRank::CorpseHandler,
            Self::SkeletonMage | Self::ZombieBloated | Self::Wight | Self::Draugr => NecromancerRank::Animator,
            Self::SkeletonKnight | Self::ZombiePlague | Self::GhostWraith |
            Self::GhostPoltergeist | Self::Mummy | Self::FleshGolem => NecromancerRank::Necromancer,
            Self::SkeletonGiant | Self::GhostBanshee | Self::VampireFledgling |
            Self::Revenant | Self::WightLord | Self::DraugrOverlord |
            Self::ShadowStalker | Self::GraveGuardian => NecromancerRank::DeathMage,
            Self::DeathKnight | Self::VampireElder | Self::LichApprentice |
            Self::MummyLord | Self::BoneHorror | Self::CorpseBeast |
            Self::PlagueBringer | Self::SoulReaper => NecromancerRank::Lich,
            Self::VampireAncient | Self::LichMaster | Self::SkeletalDrake => NecromancerRank::DeathLord,
            Self::VampireLord | Self::ArchLich | Self::DemiLich |
            Self::BoneDragon | Self::NecroticAbomination => NecromancerRank::DeathGod,
        }
    }

    pub fn abilities(&self) -> Vec<UndeadAbility> {
        match self {
            Self::SkeletonWarrior => vec![UndeadAbility::ShieldBlock],
            Self::SkeletonArcher => vec![UndeadAbility::PiercingShot, UndeadAbility::Volley],
            Self::SkeletonMage => vec![UndeadAbility::DeathBolt, UndeadAbility::BoneShield],
            Self::SkeletonKnight => vec![UndeadAbility::Charge, UndeadAbility::ShieldBlock, UndeadAbility::Rally],
            Self::SkeletonGiant => vec![UndeadAbility::Stomp, UndeadAbility::Sweep],
            Self::ZombieShambler => vec![UndeadAbility::Resilient],
            Self::ZombieRunner => vec![UndeadAbility::Frenzy, UndeadAbility::Pounce],
            Self::ZombieBloated => vec![UndeadAbility::DeathBurst, UndeadAbility::ToxicAura],
            Self::ZombiePlague => vec![UndeadAbility::PlagueTouch, UndeadAbility::Infection],
            Self::GhostSpectre => vec![UndeadAbility::Incorporeal, UndeadAbility::LifeDrain],
            Self::GhostWraith => vec![UndeadAbility::Incorporeal, UndeadAbility::FearAura, UndeadAbility::LifeDrain],
            Self::GhostBanshee => vec![UndeadAbility::Incorporeal, UndeadAbility::DeathWail, UndeadAbility::Lament],
            Self::GhostPoltergeist => vec![UndeadAbility::Incorporeal, UndeadAbility::Telekinesis, UndeadAbility::Possession],
            Self::VampireFledgling => vec![UndeadAbility::LifeDrain, UndeadAbility::NightVision, UndeadAbility::BloodThirst],
            Self::VampireElder => vec![UndeadAbility::LifeDrain, UndeadAbility::NightVision, UndeadAbility::BloodThirst,
                                       UndeadAbility::MistForm, UndeadAbility::Charm],
            Self::VampireAncient => vec![UndeadAbility::LifeDrain, UndeadAbility::NightVision, UndeadAbility::BloodThirst,
                                         UndeadAbility::MistForm, UndeadAbility::Charm, UndeadAbility::DominatingGaze,
                                         UndeadAbility::BatSwarm],
            Self::VampireLord => vec![UndeadAbility::LifeDrain, UndeadAbility::NightVision, UndeadAbility::BloodThirst,
                                      UndeadAbility::MistForm, UndeadAbility::Charm, UndeadAbility::DominatingGaze,
                                      UndeadAbility::BatSwarm, UndeadAbility::BloodMagic, UndeadAbility::Immortal],
            Self::LichApprentice => vec![UndeadAbility::DeathBolt, UndeadAbility::SoulHarvest],
            Self::LichMaster => vec![UndeadAbility::DeathBolt, UndeadAbility::SoulHarvest,
                                     UndeadAbility::RaiseDead, UndeadAbility::DeathAndDecay],
            Self::ArchLich => vec![UndeadAbility::DeathBolt, UndeadAbility::SoulHarvest, UndeadAbility::RaiseDead,
                                   UndeadAbility::DeathAndDecay, UndeadAbility::ArmyOfTheDead, UndeadAbility::TimeStop],
            Self::DemiLich => vec![UndeadAbility::SoulHarvest, UndeadAbility::DeathAndDecay, UndeadAbility::ArmyOfTheDead,
                                   UndeadAbility::TimeStop, UndeadAbility::DevourSoul, UndeadAbility::Transcendence],
            Self::DeathKnight => vec![UndeadAbility::DeathStrike, UndeadAbility::UnholyAura, UndeadAbility::Rally,
                                      UndeadAbility::DarkResurrection],
            Self::BoneDragon => vec![UndeadAbility::FrostBreath, UndeadAbility::FearAura, UndeadAbility::Flight,
                                     UndeadAbility::TailSwipe, UndeadAbility::BoneStorm],
            Self::FleshGolem => vec![UndeadAbility::Resilient, UndeadAbility::Berserk, UndeadAbility::Regeneration],
            Self::Revenant => vec![UndeadAbility::Vengeance, UndeadAbility::Unstoppable, UndeadAbility::DeathMark],
            Self::Mummy => vec![UndeadAbility::MummyRot, UndeadAbility::Resilient, UndeadAbility::Despair],
            Self::MummyLord => vec![UndeadAbility::MummyRot, UndeadAbility::Despair, UndeadAbility::SandStorm,
                                    UndeadAbility::CurseOfTheAncients, UndeadAbility::RaiseMummies],
            Self::Draugr => vec![UndeadAbility::FrostTouch, UndeadAbility::Resilient],
            Self::DraugrOverlord => vec![UndeadAbility::FrostTouch, UndeadAbility::FrostBreath,
                                         UndeadAbility::UnholyStrength, UndeadAbility::RaiseDraugr],
            Self::Wight => vec![UndeadAbility::LifeDrain, UndeadAbility::NightVision],
            Self::WightLord => vec![UndeadAbility::LifeDrain, UndeadAbility::NightVision, UndeadAbility::CreateWight,
                                    UndeadAbility::UnholyAura],
            Self::BoneHorror => vec![UndeadAbility::MultiAttack, UndeadAbility::Sweep, UndeadAbility::Reassemble],
            Self::CorpseBeast => vec![UndeadAbility::Devour, UndeadAbility::Resilient, UndeadAbility::Berserk],
            Self::ShadowStalker => vec![UndeadAbility::Incorporeal, UndeadAbility::ShadowStep, UndeadAbility::Backstab],
            Self::GraveGuardian => vec![UndeadAbility::ShieldBlock, UndeadAbility::Taunt, UndeadAbility::Sentinel],
            Self::PlagueBringer => vec![UndeadAbility::PlagueTouch, UndeadAbility::PlagueAura, UndeadAbility::Epidemic],
            Self::SoulReaper => vec![UndeadAbility::SoulHarvest, UndeadAbility::ReapingStrike, UndeadAbility::SoulChains],
            Self::NecroticAbomination => vec![UndeadAbility::Devour, UndeadAbility::DeathAndDecay, UndeadAbility::Unstoppable,
                                               UndeadAbility::NecroticBurst, UndeadAbility::Regeneration],
            Self::SkeletalDrake => vec![UndeadAbility::Flight, UndeadAbility::FrostBreath, UndeadAbility::Swoop],
        }
    }

    pub fn corpse_requirements(&self) -> CorpseRequirement {
        match self {
            Self::SkeletonWarrior | Self::SkeletonArcher | Self::SkeletonMage =>
                CorpseRequirement::Single(CorpseType::Humanoid),
            Self::SkeletonKnight => CorpseRequirement::Single(CorpseType::ArmoredHumanoid),
            Self::SkeletonGiant => CorpseRequirement::Single(CorpseType::Giant),
            Self::ZombieShambler | Self::ZombieRunner | Self::ZombiePlague =>
                CorpseRequirement::Single(CorpseType::Fresh),
            Self::ZombieBloated => CorpseRequirement::Single(CorpseType::Bloated),
            Self::GhostSpectre | Self::GhostWraith | Self::GhostBanshee | Self::GhostPoltergeist =>
                CorpseRequirement::Soul(SoulType::Mortal),
            Self::VampireFledgling => CorpseRequirement::Living(LivingType::Humanoid),
            Self::VampireElder | Self::VampireAncient | Self::VampireLord =>
                CorpseRequirement::Upgrade(Box::new(CorpseRequirement::Living(LivingType::Humanoid))),
            Self::LichApprentice => CorpseRequirement::Ritual(RitualType::LichTransformation),
            Self::LichMaster | Self::ArchLich | Self::DemiLich =>
                CorpseRequirement::Upgrade(Box::new(CorpseRequirement::Ritual(RitualType::LichTransformation))),
            Self::DeathKnight => CorpseRequirement::Single(CorpseType::FallenKnight),
            Self::BoneDragon => CorpseRequirement::Single(CorpseType::Dragon),
            Self::FleshGolem => CorpseRequirement::Multiple(CorpseType::Fresh, 5),
            Self::Revenant => CorpseRequirement::Single(CorpseType::VengefulDead),
            Self::Mummy | Self::MummyLord => CorpseRequirement::Single(CorpseType::Preserved),
            Self::Draugr | Self::DraugrOverlord => CorpseRequirement::Single(CorpseType::AncientWarrior),
            Self::Wight | Self::WightLord => CorpseRequirement::Single(CorpseType::Humanoid),
            Self::BoneHorror => CorpseRequirement::Multiple(CorpseType::Bones, 10),
            Self::CorpseBeast => CorpseRequirement::Multiple(CorpseType::Fresh, 8),
            Self::ShadowStalker => CorpseRequirement::Soul(SoulType::Assassin),
            Self::GraveGuardian => CorpseRequirement::Single(CorpseType::ArmoredHumanoid),
            Self::PlagueBringer => CorpseRequirement::Single(CorpseType::Diseased),
            Self::SoulReaper => CorpseRequirement::Soul(SoulType::Reaper),
            Self::NecroticAbomination => CorpseRequirement::Multiple(CorpseType::Fresh, 20),
            Self::SkeletalDrake => CorpseRequirement::Single(CorpseType::Drake),
        }
    }
}

/// Stats for an undead creature
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UndeadStats {
    pub max_hp: i32,
    pub current_hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub mana: i32,
    pub speed: i32,
    pub control_cost: u32,
}

impl UndeadStats {
    pub fn new(hp: i32, attack: i32, defense: i32, mana: i32, speed: i32, control: u32) -> Self {
        Self {
            max_hp: hp,
            current_hp: hp,
            attack,
            defense,
            mana,
            speed,
            control_cost: control,
        }
    }
}

/// Abilities that undead creatures can have
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UndeadAbility {
    // Basic
    ShieldBlock,
    PiercingShot,
    Volley,
    Charge,
    Rally,
    Stomp,
    Sweep,
    Resilient,
    Frenzy,
    Pounce,

    // Death abilities
    DeathBolt,
    DeathBurst,
    DeathStrike,
    DeathWail,
    DeathMark,
    DeathAndDecay,
    DarkResurrection,

    // Poison/Disease
    ToxicAura,
    PlagueTouch,
    Infection,
    PlagueAura,
    Epidemic,
    MummyRot,

    // Ghost abilities
    Incorporeal,
    Possession,
    Telekinesis,
    Lament,
    ShadowStep,
    Backstab,

    // Life manipulation
    LifeDrain,
    BloodThirst,
    BloodMagic,
    Devour,
    DevourSoul,

    // Vampire abilities
    NightVision,
    MistForm,
    Charm,
    DominatingGaze,
    BatSwarm,
    Immortal,

    // Lich abilities
    SoulHarvest,
    RaiseDead,
    ArmyOfTheDead,
    TimeStop,
    Transcendence,

    // Auras
    FearAura,
    UnholyAura,
    BoneShield,

    // Dragon abilities
    FrostBreath,
    Flight,
    TailSwipe,
    BoneStorm,
    Swoop,

    // Combat
    Berserk,
    Regeneration,
    Vengeance,
    Unstoppable,
    MultiAttack,
    Reassemble,
    Taunt,
    Sentinel,
    ReapingStrike,
    SoulChains,
    NecroticBurst,

    // Frost
    FrostTouch,

    // Special
    UnholyStrength,
    RaiseDraugr,
    RaiseMummies,
    CreateWight,
    SandStorm,
    CurseOfTheAncients,
    Despair,
}

impl UndeadAbility {
    pub fn name(&self) -> &'static str {
        match self {
            Self::ShieldBlock => "Shield Block",
            Self::PiercingShot => "Piercing Shot",
            Self::Volley => "Volley",
            Self::Charge => "Charge",
            Self::Rally => "Rally",
            Self::Stomp => "Stomp",
            Self::Sweep => "Sweep",
            Self::Resilient => "Resilient",
            Self::Frenzy => "Frenzy",
            Self::Pounce => "Pounce",
            Self::DeathBolt => "Death Bolt",
            Self::DeathBurst => "Death Burst",
            Self::DeathStrike => "Death Strike",
            Self::DeathWail => "Death Wail",
            Self::DeathMark => "Death Mark",
            Self::DeathAndDecay => "Death and Decay",
            Self::DarkResurrection => "Dark Resurrection",
            Self::ToxicAura => "Toxic Aura",
            Self::PlagueTouch => "Plague Touch",
            Self::Infection => "Infection",
            Self::PlagueAura => "Plague Aura",
            Self::Epidemic => "Epidemic",
            Self::MummyRot => "Mummy Rot",
            Self::Incorporeal => "Incorporeal",
            Self::Possession => "Possession",
            Self::Telekinesis => "Telekinesis",
            Self::Lament => "Lament",
            Self::ShadowStep => "Shadow Step",
            Self::Backstab => "Backstab",
            Self::LifeDrain => "Life Drain",
            Self::BloodThirst => "Blood Thirst",
            Self::BloodMagic => "Blood Magic",
            Self::Devour => "Devour",
            Self::DevourSoul => "Devour Soul",
            Self::NightVision => "Night Vision",
            Self::MistForm => "Mist Form",
            Self::Charm => "Charm",
            Self::DominatingGaze => "Dominating Gaze",
            Self::BatSwarm => "Bat Swarm",
            Self::Immortal => "Immortal",
            Self::SoulHarvest => "Soul Harvest",
            Self::RaiseDead => "Raise Dead",
            Self::ArmyOfTheDead => "Army of the Dead",
            Self::TimeStop => "Time Stop",
            Self::Transcendence => "Transcendence",
            Self::FearAura => "Fear Aura",
            Self::UnholyAura => "Unholy Aura",
            Self::BoneShield => "Bone Shield",
            Self::FrostBreath => "Frost Breath",
            Self::Flight => "Flight",
            Self::TailSwipe => "Tail Swipe",
            Self::BoneStorm => "Bone Storm",
            Self::Swoop => "Swoop",
            Self::Berserk => "Berserk",
            Self::Regeneration => "Regeneration",
            Self::Vengeance => "Vengeance",
            Self::Unstoppable => "Unstoppable",
            Self::MultiAttack => "Multi-Attack",
            Self::Reassemble => "Reassemble",
            Self::Taunt => "Taunt",
            Self::Sentinel => "Sentinel",
            Self::ReapingStrike => "Reaping Strike",
            Self::SoulChains => "Soul Chains",
            Self::NecroticBurst => "Necrotic Burst",
            Self::FrostTouch => "Frost Touch",
            Self::UnholyStrength => "Unholy Strength",
            Self::RaiseDraugr => "Raise Draugr",
            Self::RaiseMummies => "Raise Mummies",
            Self::CreateWight => "Create Wight",
            Self::SandStorm => "Sand Storm",
            Self::CurseOfTheAncients => "Curse of the Ancients",
            Self::Despair => "Despair",
        }
    }

    pub fn cooldown(&self) -> u32 {
        match self {
            Self::ShieldBlock | Self::Resilient | Self::Incorporeal |
            Self::NightVision | Self::BoneShield => 0,
            Self::PiercingShot | Self::Pounce | Self::DeathBolt |
            Self::FrostTouch | Self::Backstab | Self::Swoop => 2,
            Self::Volley | Self::Charge | Self::Frenzy | Self::LifeDrain |
            Self::Charm | Self::Lament => 3,
            Self::Rally | Self::Stomp | Self::Sweep | Self::DeathStrike |
            Self::MistForm | Self::TailSwipe => 4,
            Self::DeathBurst | Self::DeathWail | Self::SoulHarvest |
            Self::FrostBreath | Self::BoneStorm | Self::Taunt => 5,
            Self::ToxicAura | Self::PlagueTouch | Self::Infection |
            Self::Telekinesis | Self::BloodThirst | Self::Devour => 4,
            Self::PlagueAura | Self::MummyRot | Self::Possession |
            Self::DominatingGaze | Self::Vengeance => 6,
            Self::Epidemic | Self::DeathAndDecay | Self::BatSwarm |
            Self::MultiAttack | Self::NecroticBurst => 8,
            Self::ShadowStep | Self::BloodMagic | Self::FearAura |
            Self::UnholyAura | Self::Flight => 5,
            Self::DevourSoul | Self::RaiseDead | Self::Berserk |
            Self::Regeneration | Self::Sentinel => 10,
            Self::DarkResurrection | Self::Immortal | Self::ArmyOfTheDead |
            Self::TimeStop | Self::Unstoppable | Self::Reassemble => 15,
            Self::DeathMark | Self::Transcendence | Self::ReapingStrike |
            Self::SoulChains => 12,
            Self::UnholyStrength | Self::RaiseDraugr | Self::RaiseMummies |
            Self::CreateWight | Self::SandStorm | Self::CurseOfTheAncients |
            Self::Despair => 10,
        }
    }
}

/// Corpse types for reanimation
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CorpseType {
    Humanoid,
    ArmoredHumanoid,
    Giant,
    Fresh,
    Bloated,
    FallenKnight,
    Dragon,
    Drake,
    VengefulDead,
    Preserved,
    AncientWarrior,
    Bones,
    Diseased,
}

/// Soul types for ghost creation
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SoulType {
    Mortal,
    Assassin,
    Reaper,
    Ancient,
    Divine,
}

/// Living types for vampire creation
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LivingType {
    Humanoid,
    Beast,
    Monster,
}

/// Ritual types for special undead
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RitualType {
    LichTransformation,
    DeathKnightBinding,
    DragonReanimation,
    AbominationCreation,
}

/// Requirements to create an undead
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CorpseRequirement {
    Single(CorpseType),
    Multiple(CorpseType, u32),
    Soul(SoulType),
    Living(LivingType),
    Ritual(RitualType),
    Upgrade(Box<CorpseRequirement>),
}

// ============================================================================
// NECROMANCY SCHOOLS
// ============================================================================

/// The 8 schools of necromancy
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NecromancySchool {
    Reanimation,
    SoulMagic,
    DeathMagic,
    BloodMagic,
    BoneCrafting,
    SpiritBinding,
    CurseMagic,
    PlagueMagic,
}

impl NecromancySchool {
    pub fn all() -> &'static [NecromancySchool] {
        &[
            Self::Reanimation, Self::SoulMagic, Self::DeathMagic, Self::BloodMagic,
            Self::BoneCrafting, Self::SpiritBinding, Self::CurseMagic, Self::PlagueMagic,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Reanimation => "Reanimation",
            Self::SoulMagic => "Soul Magic",
            Self::DeathMagic => "Death Magic",
            Self::BloodMagic => "Blood Magic",
            Self::BoneCrafting => "Bone Crafting",
            Self::SpiritBinding => "Spirit Binding",
            Self::CurseMagic => "Curse Magic",
            Self::PlagueMagic => "Plague Magic",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Reanimation => "The art of raising the dead. Animates corpses and skeletons.",
            Self::SoulMagic => "Manipulation of souls. Capture, bind, and consume spirits.",
            Self::DeathMagic => "Direct death spells. Instant kills and life force destruction.",
            Self::BloodMagic => "Life force manipulation. Drain life to fuel dark powers.",
            Self::BoneCrafting => "Creation of bone constructs. Shape bones into weapons and minions.",
            Self::SpiritBinding => "Control of spirits. Command ghosts and ethereal beings.",
            Self::CurseMagic => "Hexes and curses. Weaken and doom your enemies.",
            Self::PlagueMagic => "Disease and decay. Spread pestilence and rot.",
        }
    }

    pub fn spells(&self) -> Vec<NecromancySpell> {
        match self {
            Self::Reanimation => vec![
                NecromancySpell::new("Raise Skeleton", *self, 1, 20, NecromancyEffect::RaiseSkeleton),
                NecromancySpell::new("Animate Corpse", *self, 1, 25, NecromancyEffect::AnimateCorpse),
                NecromancySpell::new("Strengthen Undead", *self, 2, 30, NecromancyEffect::StrengthenUndead { bonus: 20 }),
                NecromancySpell::new("Mass Reanimation", *self, 3, 80, NecromancyEffect::MassRaise { count: 5 }),
                NecromancySpell::new("Create Flesh Golem", *self, 4, 100, NecromancyEffect::CreateConstruct(UndeadType::FleshGolem)),
                NecromancySpell::new("Raise Death Knight", *self, 5, 200, NecromancyEffect::RaiseSpecial(UndeadType::DeathKnight)),
                NecromancySpell::new("Undead Frenzy", *self, 3, 60, NecromancyEffect::UndeadFrenzy),
                NecromancySpell::new("Corpse Explosion", *self, 2, 35, NecromancyEffect::CorpseExplosion { damage: 50 }),
                NecromancySpell::new("Eternal Servitude", *self, 4, 120, NecromancyEffect::PermanentUndead),
                NecromancySpell::new("Army of Darkness", *self, 5, 300, NecromancyEffect::MassRaise { count: 20 }),
            ],
            Self::SoulMagic => vec![
                NecromancySpell::new("Soul Trap", *self, 1, 15, NecromancyEffect::SoulTrap),
                NecromancySpell::new("Soul Drain", *self, 2, 30, NecromancyEffect::SoulDrain { amount: 25 }),
                NecromancySpell::new("Soul Shield", *self, 2, 40, NecromancyEffect::SoulShield { absorb: 50 }),
                NecromancySpell::new("Consume Soul", *self, 3, 50, NecromancyEffect::ConsumeSoul),
                NecromancySpell::new("Soul Rend", *self, 3, 60, NecromancyEffect::SoulRend { damage: 80 }),
                NecromancySpell::new("Create Soul Gem", *self, 2, 25, NecromancyEffect::CreateSoulGem(SoulGemSize::Small)),
                NecromancySpell::new("Soul Storm", *self, 4, 100, NecromancyEffect::SoulStorm { damage: 60, radius: 3 }),
                NecromancySpell::new("Soul Swap", *self, 4, 80, NecromancyEffect::SoulSwap),
                NecromancySpell::new("Create Phylactery", *self, 5, 500, NecromancyEffect::CreatePhylactery),
                NecromancySpell::new("Devour Soul", *self, 5, 150, NecromancyEffect::DevourSoul),
            ],
            Self::DeathMagic => vec![
                NecromancySpell::new("Death Bolt", *self, 1, 15, NecromancyEffect::Damage { amount: 30, damage_type: DeathDamageType::Necrotic }),
                NecromancySpell::new("Life Tap", *self, 1, 10, NecromancyEffect::LifeTap { damage: 20, heal: 10 }),
                NecromancySpell::new("Death Coil", *self, 2, 25, NecromancyEffect::DeathCoil { damage: 40, heal_undead: 40 }),
                NecromancySpell::new("Finger of Death", *self, 4, 100, NecromancyEffect::FingerOfDeath { damage: 150 }),
                NecromancySpell::new("Death Wave", *self, 3, 70, NecromancyEffect::AreaDamage { amount: 50, radius: 3 }),
                NecromancySpell::new("Necrotic Aura", *self, 3, 60, NecromancyEffect::NecroticAura { damage_per_turn: 10, duration: 10 }),
                NecromancySpell::new("Death Pact", *self, 2, 0, NecromancyEffect::DeathPact),
                NecromancySpell::new("Doom", *self, 4, 80, NecromancyEffect::Doom { turns: 5 }),
                NecromancySpell::new("Power Word Kill", *self, 5, 200, NecromancyEffect::InstantKill { hp_threshold: 100 }),
                NecromancySpell::new("Death and Decay", *self, 5, 150, NecromancyEffect::DeathAndDecay { damage_per_turn: 30, duration: 10, radius: 4 }),
            ],
            Self::BloodMagic => vec![
                NecromancySpell::new("Blood Bolt", *self, 1, 0, NecromancyEffect::BloodBolt { hp_cost: 15, damage: 35 }),
                NecromancySpell::new("Blood Drain", *self, 1, 10, NecromancyEffect::BloodDrain { amount: 30, heal_percent: 50 }),
                NecromancySpell::new("Blood Shield", *self, 2, 0, NecromancyEffect::BloodShield { hp_cost: 30, absorb: 60 }),
                NecromancySpell::new("Blood Boil", *self, 3, 0, NecromancyEffect::BloodBoil { hp_cost: 50, damage: 100 }),
                NecromancySpell::new("Sanguine Pact", *self, 2, 0, NecromancyEffect::SanguinePact { hp_sacrifice: 25, buff_percent: 30 }),
                NecromancySpell::new("Blood Ritual", *self, 4, 0, NecromancyEffect::BloodRitual { hp_cost: 100, effect: "MassHeal".into() }),
                NecromancySpell::new("Hemorrhage", *self, 3, 30, NecromancyEffect::Hemorrhage { bleed_per_turn: 15, duration: 8 }),
                NecromancySpell::new("Crimson Tide", *self, 4, 0, NecromancyEffect::CrimsonTide { hp_cost: 80, damage: 70, radius: 3 }),
                NecromancySpell::new("Blood Puppet", *self, 4, 60, NecromancyEffect::BloodPuppet { duration: 10 }),
                NecromancySpell::new("Exsanguinate", *self, 5, 100, NecromancyEffect::Exsanguinate { kill_threshold_percent: 25 }),
            ],
            Self::BoneCrafting => vec![
                NecromancySpell::new("Bone Spike", *self, 1, 15, NecromancyEffect::Damage { amount: 25, damage_type: DeathDamageType::Physical }),
                NecromancySpell::new("Bone Armor", *self, 2, 30, NecromancyEffect::BoneArmor { defense: 30, duration: 20 }),
                NecromancySpell::new("Bone Wall", *self, 2, 40, NecromancyEffect::BoneWall { hp: 100, duration: 15 }),
                NecromancySpell::new("Bone Spear", *self, 2, 25, NecromancyEffect::BoneSpear { damage: 45, pierce: true }),
                NecromancySpell::new("Bone Prison", *self, 3, 50, NecromancyEffect::BonePrison { duration: 5 }),
                NecromancySpell::new("Create Bone Horror", *self, 4, 100, NecromancyEffect::CreateConstruct(UndeadType::BoneHorror)),
                NecromancySpell::new("Bone Storm", *self, 4, 80, NecromancyEffect::BoneStorm { damage_per_turn: 25, duration: 6, radius: 3 }),
                NecromancySpell::new("Skeletal Wings", *self, 3, 60, NecromancyEffect::SkeletalWings { duration: 30 }),
                NecromancySpell::new("Bone Golem", *self, 4, 120, NecromancyEffect::CreateConstruct(UndeadType::BoneHorror)),
                NecromancySpell::new("Bone Dragon", *self, 5, 400, NecromancyEffect::RaiseSpecial(UndeadType::BoneDragon)),
            ],
            Self::SpiritBinding => vec![
                NecromancySpell::new("Summon Spectre", *self, 1, 25, NecromancyEffect::SummonSpirit(UndeadType::GhostSpectre)),
                NecromancySpell::new("Spirit Link", *self, 2, 30, NecromancyEffect::SpiritLink),
                NecromancySpell::new("Banish Spirit", *self, 2, 35, NecromancyEffect::BanishSpirit),
                NecromancySpell::new("Summon Wraith", *self, 3, 50, NecromancyEffect::SummonSpirit(UndeadType::GhostWraith)),
                NecromancySpell::new("Spirit Walk", *self, 3, 45, NecromancyEffect::SpiritWalk { duration: 10 }),
                NecromancySpell::new("Bind Spirit", *self, 3, 60, NecromancyEffect::BindSpirit { duration: 30 }),
                NecromancySpell::new("Summon Banshee", *self, 4, 80, NecromancyEffect::SummonSpirit(UndeadType::GhostBanshee)),
                NecromancySpell::new("Mass Haunt", *self, 4, 100, NecromancyEffect::MassHaunt { count: 5, duration: 15 }),
                NecromancySpell::new("Possession", *self, 4, 90, NecromancyEffect::Possession { duration: 10 }),
                NecromancySpell::new("Spirit Army", *self, 5, 200, NecromancyEffect::SpiritArmy { count: 10, duration: 20 }),
            ],
            Self::CurseMagic => vec![
                NecromancySpell::new("Weakness", *self, 1, 15, NecromancyEffect::Curse { curse_type: CurseType::Weakness, duration: 10 }),
                NecromancySpell::new("Slow", *self, 1, 15, NecromancyEffect::Curse { curse_type: CurseType::Slow, duration: 10 }),
                NecromancySpell::new("Blindness", *self, 2, 25, NecromancyEffect::Curse { curse_type: CurseType::Blindness, duration: 8 }),
                NecromancySpell::new("Hex", *self, 2, 30, NecromancyEffect::Hex { stat_reduction: 20, duration: 15 }),
                NecromancySpell::new("Agony", *self, 3, 40, NecromancyEffect::Curse { curse_type: CurseType::Agony, duration: 10 }),
                NecromancySpell::new("Doom Curse", *self, 4, 70, NecromancyEffect::DoomCurse { damage_per_turn: 20, duration: 10 }),
                NecromancySpell::new("Curse of Exhaustion", *self, 3, 45, NecromancyEffect::Curse { curse_type: CurseType::Exhaustion, duration: 20 }),
                NecromancySpell::new("Bane", *self, 4, 60, NecromancyEffect::Bane { debuff_all: 25, duration: 15 }),
                NecromancySpell::new("Death Curse", *self, 5, 150, NecromancyEffect::DeathCurse),
                NecromancySpell::new("Eternal Torment", *self, 5, 200, NecromancyEffect::EternalTorment),
            ],
            Self::PlagueMagic => vec![
                NecromancySpell::new("Noxious Touch", *self, 1, 10, NecromancyEffect::Disease { disease_type: DiseaseType::MinorRot, duration: 10 }),
                NecromancySpell::new("Poison Cloud", *self, 2, 30, NecromancyEffect::PoisonCloud { damage_per_turn: 10, duration: 8, radius: 2 }),
                NecromancySpell::new("Festering Wounds", *self, 2, 25, NecromancyEffect::Disease { disease_type: DiseaseType::FesteringWounds, duration: 15 }),
                NecromancySpell::new("Plague Strike", *self, 3, 40, NecromancyEffect::Disease { disease_type: DiseaseType::Plague, duration: 20 }),
                NecromancySpell::new("Contagion", *self, 3, 50, NecromancyEffect::Contagion { spread_chance: 50 }),
                NecromancySpell::new("Miasma", *self, 4, 80, NecromancyEffect::Miasma { radius: 4, duration: 15 }),
                NecromancySpell::new("Summon Plague Zombie", *self, 3, 60, NecromancyEffect::SummonPlagueBeast(UndeadType::ZombiePlague)),
                NecromancySpell::new("Epidemic", *self, 4, 100, NecromancyEffect::Epidemic { spread_radius: 5 }),
                NecromancySpell::new("Black Death", *self, 5, 200, NecromancyEffect::Disease { disease_type: DiseaseType::BlackDeath, duration: 30 }),
                NecromancySpell::new("Apocalyptic Plague", *self, 5, 300, NecromancyEffect::ApocalypticPlague),
            ],
        }
    }

    pub fn bonuses(&self) -> SchoolBonuses {
        match self {
            Self::Reanimation => SchoolBonuses {
                undead_hp_bonus: 0.1,
                undead_damage_bonus: 0.0,
                control_limit_bonus: 2,
                spell_cost_reduction: 0.0,
            },
            Self::SoulMagic => SchoolBonuses {
                undead_hp_bonus: 0.0,
                undead_damage_bonus: 0.05,
                control_limit_bonus: 0,
                spell_cost_reduction: 0.1,
            },
            Self::DeathMagic => SchoolBonuses {
                undead_hp_bonus: 0.0,
                undead_damage_bonus: 0.15,
                control_limit_bonus: 0,
                spell_cost_reduction: 0.0,
            },
            Self::BloodMagic => SchoolBonuses {
                undead_hp_bonus: 0.15,
                undead_damage_bonus: 0.1,
                control_limit_bonus: 0,
                spell_cost_reduction: 0.0,
            },
            Self::BoneCrafting => SchoolBonuses {
                undead_hp_bonus: 0.0,
                undead_damage_bonus: 0.0,
                control_limit_bonus: 1,
                spell_cost_reduction: 0.05,
            },
            Self::SpiritBinding => SchoolBonuses {
                undead_hp_bonus: 0.0,
                undead_damage_bonus: 0.05,
                control_limit_bonus: 3,
                spell_cost_reduction: 0.0,
            },
            Self::CurseMagic => SchoolBonuses {
                undead_hp_bonus: 0.0,
                undead_damage_bonus: 0.1,
                control_limit_bonus: 0,
                spell_cost_reduction: 0.15,
            },
            Self::PlagueMagic => SchoolBonuses {
                undead_hp_bonus: 0.05,
                undead_damage_bonus: 0.05,
                control_limit_bonus: 1,
                spell_cost_reduction: 0.05,
            },
        }
    }
}

/// Bonuses from mastering a school
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SchoolBonuses {
    pub undead_hp_bonus: f32,
    pub undead_damage_bonus: f32,
    pub control_limit_bonus: u32,
    pub spell_cost_reduction: f32,
}

/// A necromancy spell
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NecromancySpell {
    pub name: String,
    pub school: NecromancySchool,
    pub tier: u8,
    pub mana_cost: i32,
    pub effect: NecromancyEffect,
    pub cooldown: u32,
    pub current_cooldown: u32,
}

impl NecromancySpell {
    pub fn new(name: &str, school: NecromancySchool, tier: u8, mana: i32, effect: NecromancyEffect) -> Self {
        Self {
            name: name.to_string(),
            school,
            tier,
            mana_cost: mana,
            effect,
            cooldown: tier as u32 * 5,
            current_cooldown: 0,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.current_cooldown == 0
    }

    pub fn cast(&mut self) {
        self.current_cooldown = self.cooldown;
    }

    pub fn tick(&mut self) {
        if self.current_cooldown > 0 {
            self.current_cooldown -= 1;
        }
    }
}

/// Effects of necromancy spells
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NecromancyEffect {
    // Reanimation
    RaiseSkeleton,
    AnimateCorpse,
    StrengthenUndead { bonus: i32 },
    MassRaise { count: u32 },
    CreateConstruct(UndeadType),
    RaiseSpecial(UndeadType),
    UndeadFrenzy,
    CorpseExplosion { damage: i32 },
    PermanentUndead,

    // Soul Magic
    SoulTrap,
    SoulDrain { amount: i32 },
    SoulShield { absorb: i32 },
    ConsumeSoul,
    SoulRend { damage: i32 },
    CreateSoulGem(SoulGemSize),
    SoulStorm { damage: i32, radius: u32 },
    SoulSwap,
    CreatePhylactery,
    DevourSoul,

    // Death Magic
    Damage { amount: i32, damage_type: DeathDamageType },
    LifeTap { damage: i32, heal: i32 },
    DeathCoil { damage: i32, heal_undead: i32 },
    FingerOfDeath { damage: i32 },
    AreaDamage { amount: i32, radius: u32 },
    NecroticAura { damage_per_turn: i32, duration: u32 },
    DeathPact,
    Doom { turns: u32 },
    InstantKill { hp_threshold: i32 },
    DeathAndDecay { damage_per_turn: i32, duration: u32, radius: u32 },

    // Blood Magic
    BloodBolt { hp_cost: i32, damage: i32 },
    BloodDrain { amount: i32, heal_percent: u32 },
    BloodShield { hp_cost: i32, absorb: i32 },
    BloodBoil { hp_cost: i32, damage: i32 },
    SanguinePact { hp_sacrifice: i32, buff_percent: i32 },
    BloodRitual { hp_cost: i32, effect: String },
    Hemorrhage { bleed_per_turn: i32, duration: u32 },
    CrimsonTide { hp_cost: i32, damage: i32, radius: u32 },
    BloodPuppet { duration: u32 },
    Exsanguinate { kill_threshold_percent: u32 },

    // Bone Crafting
    BoneArmor { defense: i32, duration: u32 },
    BoneWall { hp: i32, duration: u32 },
    BoneSpear { damage: i32, pierce: bool },
    BonePrison { duration: u32 },
    BoneStorm { damage_per_turn: i32, duration: u32, radius: u32 },
    SkeletalWings { duration: u32 },

    // Spirit Binding
    SummonSpirit(UndeadType),
    SpiritLink,
    BanishSpirit,
    SpiritWalk { duration: u32 },
    BindSpirit { duration: u32 },
    MassHaunt { count: u32, duration: u32 },
    Possession { duration: u32 },
    SpiritArmy { count: u32, duration: u32 },

    // Curse Magic
    Curse { curse_type: CurseType, duration: u32 },
    Hex { stat_reduction: i32, duration: u32 },
    DoomCurse { damage_per_turn: i32, duration: u32 },
    Bane { debuff_all: i32, duration: u32 },
    DeathCurse,
    EternalTorment,

    // Plague Magic
    Disease { disease_type: DiseaseType, duration: u32 },
    PoisonCloud { damage_per_turn: i32, duration: u32, radius: u32 },
    Contagion { spread_chance: u32 },
    Miasma { radius: u32, duration: u32 },
    SummonPlagueBeast(UndeadType),
    Epidemic { spread_radius: u32 },
    ApocalypticPlague,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeathDamageType {
    Necrotic,
    Physical,
    Shadow,
    Frost,
    Plague,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CurseType {
    Weakness,
    Slow,
    Blindness,
    Agony,
    Exhaustion,
    Doom,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiseaseType {
    MinorRot,
    FesteringWounds,
    Plague,
    BlackDeath,
    NecroticDecay,
}

// ============================================================================
// NECROMANCER RANKS
// ============================================================================

/// Necromancer progression ranks
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum NecromancerRank {
    GraveRobber,
    CorpseHandler,
    Animator,
    Necromancer,
    DeathMage,
    Lich,
    DeathLord,
    DeathGod,
}

impl NecromancerRank {
    pub fn all() -> &'static [NecromancerRank] {
        &[
            Self::GraveRobber, Self::CorpseHandler, Self::Animator,
            Self::Necromancer, Self::DeathMage, Self::Lich,
            Self::DeathLord, Self::DeathGod,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::GraveRobber => "Grave Robber",
            Self::CorpseHandler => "Corpse Handler",
            Self::Animator => "Animator",
            Self::Necromancer => "Necromancer",
            Self::DeathMage => "Death Mage",
            Self::Lich => "Lich",
            Self::DeathLord => "Death Lord",
            Self::DeathGod => "Death God",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::GraveRobber => "A novice who has begun disturbing the dead.",
            Self::CorpseHandler => "One who handles corpses with dark purpose.",
            Self::Animator => "Capable of animating simple undead.",
            Self::Necromancer => "A true master of raising the dead.",
            Self::DeathMage => "Wields death magic with terrible power.",
            Self::Lich => "Has transcended death itself.",
            Self::DeathLord => "Commands legions of the dead.",
            Self::DeathGod => "An avatar of death incarnate.",
        }
    }

    pub fn required_xp(&self) -> u64 {
        match self {
            Self::GraveRobber => 0,
            Self::CorpseHandler => 500,
            Self::Animator => 2000,
            Self::Necromancer => 8000,
            Self::DeathMage => 25000,
            Self::Lich => 80000,
            Self::DeathLord => 250000,
            Self::DeathGod => 1000000,
        }
    }

    pub fn control_limit(&self) -> u32 {
        match self {
            Self::GraveRobber => 3,
            Self::CorpseHandler => 5,
            Self::Animator => 10,
            Self::Necromancer => 20,
            Self::DeathMage => 35,
            Self::Lich => 50,
            Self::DeathLord => 75,
            Self::DeathGod => 100,
        }
    }

    pub fn next_rank(&self) -> Option<NecromancerRank> {
        match self {
            Self::GraveRobber => Some(Self::CorpseHandler),
            Self::CorpseHandler => Some(Self::Animator),
            Self::Animator => Some(Self::Necromancer),
            Self::Necromancer => Some(Self::DeathMage),
            Self::DeathMage => Some(Self::Lich),
            Self::Lich => Some(Self::DeathLord),
            Self::DeathLord => Some(Self::DeathGod),
            Self::DeathGod => None,
        }
    }

    pub fn rank_bonuses(&self) -> RankBonuses {
        match self {
            Self::GraveRobber => RankBonuses::default(),
            Self::CorpseHandler => RankBonuses { undead_hp: 1.05, undead_damage: 1.05, mana_regen: 1.0, spell_power: 1.0 },
            Self::Animator => RankBonuses { undead_hp: 1.10, undead_damage: 1.10, mana_regen: 1.05, spell_power: 1.05 },
            Self::Necromancer => RankBonuses { undead_hp: 1.20, undead_damage: 1.15, mana_regen: 1.10, spell_power: 1.10 },
            Self::DeathMage => RankBonuses { undead_hp: 1.30, undead_damage: 1.25, mana_regen: 1.20, spell_power: 1.20 },
            Self::Lich => RankBonuses { undead_hp: 1.50, undead_damage: 1.40, mana_regen: 1.35, spell_power: 1.35 },
            Self::DeathLord => RankBonuses { undead_hp: 1.75, undead_damage: 1.60, mana_regen: 1.50, spell_power: 1.50 },
            Self::DeathGod => RankBonuses { undead_hp: 2.0, undead_damage: 2.0, mana_regen: 2.0, spell_power: 2.0 },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RankBonuses {
    pub undead_hp: f32,
    pub undead_damage: f32,
    pub mana_regen: f32,
    pub spell_power: f32,
}

impl Default for RankBonuses {
    fn default() -> Self {
        Self {
            undead_hp: 1.0,
            undead_damage: 1.0,
            mana_regen: 1.0,
            spell_power: 1.0,
        }
    }
}

// ============================================================================
// UNDEAD ARMY MANAGEMENT
// ============================================================================

/// An individual undead creature in the army
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UndeadMinion {
    pub id: u64,
    pub name: String,
    pub undead_type: UndeadType,
    pub stats: UndeadStats,
    pub level: u32,
    pub experience: u64,
    pub abilities: Vec<UndeadAbility>,
    pub upgrades: Vec<UndeadUpgrade>,
    pub formation_position: Option<FormationPosition>,
    pub feeding_timer: u32,
    pub loyalty: i32,
    pub is_permanent: bool,
    pub duration: Option<u32>,
}

impl UndeadMinion {
    pub fn new(id: u64, undead_type: UndeadType) -> Self {
        let stats = undead_type.base_stats();
        Self {
            id,
            name: undead_type.name().to_string(),
            undead_type,
            stats,
            level: 1,
            experience: 0,
            abilities: undead_type.abilities(),
            upgrades: vec![],
            formation_position: None,
            feeding_timer: 100,
            loyalty: 100,
            is_permanent: false,
            duration: Some(100),
        }
    }

    pub fn gain_xp(&mut self, amount: u64) -> bool {
        self.experience += amount;
        let xp_needed = self.level as u64 * 100;
        if self.experience >= xp_needed {
            self.experience -= xp_needed;
            self.level += 1;
            self.stats.max_hp = (self.stats.max_hp as f32 * 1.1) as i32;
            self.stats.current_hp = self.stats.max_hp;
            self.stats.attack = (self.stats.attack as f32 * 1.08) as i32;
            self.stats.defense = (self.stats.defense as f32 * 1.05) as i32;
            true
        } else {
            false
        }
    }

    pub fn apply_upgrade(&mut self, upgrade: UndeadUpgrade) {
        match upgrade {
            UndeadUpgrade::EnhancedStrength => self.stats.attack += 10,
            UndeadUpgrade::ReinforcedBones => self.stats.defense += 10,
            UndeadUpgrade::VitalEssence => self.stats.max_hp += 30,
            UndeadUpgrade::SwiftDeath => self.stats.speed += 3,
            UndeadUpgrade::DarkEnchantment => self.stats.mana += 20,
            UndeadUpgrade::EternalBinding => self.is_permanent = true,
            UndeadUpgrade::SoulInfusion => {
                self.stats.attack += 5;
                self.stats.max_hp += 20;
            }
            UndeadUpgrade::NecroticPlating => {
                self.stats.defense += 15;
                self.stats.speed -= 1;
            }
            UndeadUpgrade::BloodFrenzy => {
                self.abilities.push(UndeadAbility::Frenzy);
            }
            UndeadUpgrade::ShadowMeld => {
                self.abilities.push(UndeadAbility::ShadowStep);
            }
        }
        self.upgrades.push(upgrade);
    }

    pub fn needs_feeding(&self) -> bool {
        self.feeding_timer == 0 && self.undead_type.category() != UndeadCategory::Skeleton
    }

    pub fn feed(&mut self, souls: u32) {
        self.feeding_timer = souls * 25;
        self.loyalty = (self.loyalty + 10).min(100);
    }

    pub fn tick(&mut self) -> bool {
        if self.feeding_timer > 0 {
            self.feeding_timer -= 1;
        }

        if let Some(ref mut dur) = self.duration {
            if *dur > 0 {
                *dur -= 1;
            }
            if *dur == 0 && !self.is_permanent {
                return false;
            }
        }

        if self.feeding_timer == 0 && self.undead_type.category() != UndeadCategory::Skeleton {
            self.loyalty -= 1;
            if self.loyalty <= 0 {
                return false;
            }
        }

        true
    }
}

/// Upgrades that can be applied to undead
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UndeadUpgrade {
    EnhancedStrength,
    ReinforcedBones,
    VitalEssence,
    SwiftDeath,
    DarkEnchantment,
    EternalBinding,
    SoulInfusion,
    NecroticPlating,
    BloodFrenzy,
    ShadowMeld,
}

impl UndeadUpgrade {
    pub fn name(&self) -> &'static str {
        match self {
            Self::EnhancedStrength => "Enhanced Strength",
            Self::ReinforcedBones => "Reinforced Bones",
            Self::VitalEssence => "Vital Essence",
            Self::SwiftDeath => "Swift Death",
            Self::DarkEnchantment => "Dark Enchantment",
            Self::EternalBinding => "Eternal Binding",
            Self::SoulInfusion => "Soul Infusion",
            Self::NecroticPlating => "Necrotic Plating",
            Self::BloodFrenzy => "Blood Frenzy",
            Self::ShadowMeld => "Shadow Meld",
        }
    }

    pub fn soul_cost(&self) -> u32 {
        match self {
            Self::EnhancedStrength | Self::ReinforcedBones | Self::VitalEssence | Self::SwiftDeath => 5,
            Self::DarkEnchantment | Self::SoulInfusion | Self::NecroticPlating => 10,
            Self::BloodFrenzy | Self::ShadowMeld => 15,
            Self::EternalBinding => 25,
        }
    }
}

/// Formation positions for army
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FormationPosition {
    FrontLine,
    MidLine,
    BackLine,
    Flanking,
    Reserve,
}

/// Army formations
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArmyFormation {
    Horde,
    Phalanx,
    Wedge,
    Crescent,
    Scattered,
    Defensive,
    Ambush,
}

impl ArmyFormation {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Horde => "Horde",
            Self::Phalanx => "Phalanx",
            Self::Wedge => "Wedge",
            Self::Crescent => "Crescent",
            Self::Scattered => "Scattered",
            Self::Defensive => "Defensive",
            Self::Ambush => "Ambush",
        }
    }

    pub fn bonuses(&self) -> FormationBonuses {
        match self {
            Self::Horde => FormationBonuses { attack: 1.15, defense: 0.90, speed: 1.10, morale: 1.0 },
            Self::Phalanx => FormationBonuses { attack: 0.95, defense: 1.25, speed: 0.85, morale: 1.10 },
            Self::Wedge => FormationBonuses { attack: 1.20, defense: 0.85, speed: 1.15, morale: 1.0 },
            Self::Crescent => FormationBonuses { attack: 1.10, defense: 1.05, speed: 1.0, morale: 1.05 },
            Self::Scattered => FormationBonuses { attack: 1.0, defense: 0.80, speed: 1.25, morale: 0.90 },
            Self::Defensive => FormationBonuses { attack: 0.85, defense: 1.30, speed: 0.80, morale: 1.15 },
            Self::Ambush => FormationBonuses { attack: 1.30, defense: 0.75, speed: 1.20, morale: 0.95 },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FormationBonuses {
    pub attack: f32,
    pub defense: f32,
    pub speed: f32,
    pub morale: f32,
}

/// The undead army manager
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UndeadArmy {
    pub minions: Vec<UndeadMinion>,
    pub formation: ArmyFormation,
    pub control_used: u32,
    pub control_limit: u32,
    next_minion_id: u64,
}

impl UndeadArmy {
    pub fn new(control_limit: u32) -> Self {
        Self {
            minions: Vec::new(),
            formation: ArmyFormation::Horde,
            control_used: 0,
            control_limit,
            next_minion_id: 1,
        }
    }

    pub fn add_minion(&mut self, undead_type: UndeadType) -> Result<u64, &'static str> {
        let cost = undead_type.control_cost();
        if self.control_used + cost > self.control_limit {
            return Err("Not enough control capacity");
        }
        if self.minions.len() >= MAX_ARMY_SIZE {
            return Err("Army at maximum size");
        }

        let id = self.next_minion_id;
        self.next_minion_id += 1;
        let minion = UndeadMinion::new(id, undead_type);
        self.control_used += cost;
        self.minions.push(minion);
        Ok(id)
    }

    pub fn remove_minion(&mut self, id: u64) -> Option<UndeadMinion> {
        if let Some(pos) = self.minions.iter().position(|m| m.id == id) {
            let minion = self.minions.remove(pos);
            self.control_used -= minion.stats.control_cost;
            Some(minion)
        } else {
            None
        }
    }

    pub fn get_minion(&self, id: u64) -> Option<&UndeadMinion> {
        self.minions.iter().find(|m| m.id == id)
    }

    pub fn get_minion_mut(&mut self, id: u64) -> Option<&mut UndeadMinion> {
        self.minions.iter_mut().find(|m| m.id == id)
    }

    pub fn set_formation(&mut self, formation: ArmyFormation) {
        self.formation = formation;
    }

    pub fn tick(&mut self) {
        let mut to_remove = Vec::new();
        for minion in &mut self.minions {
            if !minion.tick() {
                to_remove.push(minion.id);
            }
        }
        for id in to_remove {
            self.remove_minion(id);
        }
    }

    pub fn total_power(&self) -> i32 {
        let bonuses = self.formation.bonuses();
        self.minions.iter()
            .map(|m| {
                let base_power = m.stats.attack + m.stats.defense + (m.stats.max_hp / 10);
                (base_power as f32 * bonuses.attack) as i32
            })
            .sum()
    }

    pub fn by_category(&self, category: UndeadCategory) -> Vec<&UndeadMinion> {
        self.minions.iter()
            .filter(|m| m.undead_type.category() == category)
            .collect()
    }

    pub fn hungry_minions(&self) -> Vec<&UndeadMinion> {
        self.minions.iter()
            .filter(|m| m.needs_feeding())
            .collect()
    }
}

// ============================================================================
// DEATH DOMAINS
// ============================================================================

/// Types of death domains
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeathDomainType {
    Graveyard,
    Crypt,
    Battlefield,
    PlaguePit,
    Catacomb,
    Ossuary,
    Mausoleum,
    MassGrave,
}

impl DeathDomainType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Graveyard => "Graveyard",
            Self::Crypt => "Crypt",
            Self::Battlefield => "Battlefield",
            Self::PlaguePit => "Plague Pit",
            Self::Catacomb => "Catacomb",
            Self::Ossuary => "Ossuary",
            Self::Mausoleum => "Mausoleum",
            Self::MassGrave => "Mass Grave",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Graveyard => "Common burial ground. Good source of basic undead.",
            Self::Crypt => "Ancient burial chamber. Contains powerful ancient dead.",
            Self::Battlefield => "Site of great slaughter. Allows mass raising of warriors.",
            Self::PlaguePit => "Mass grave of plague victims. Source of diseased undead.",
            Self::Catacomb => "Underground tunnel system filled with bones.",
            Self::Ossuary => "Repository of bones. Excellent for bone crafting.",
            Self::Mausoleum => "Ornate tomb of nobles. Contains wealthy dead.",
            Self::MassGrave => "Unmarked mass burial. Many corpses, weak spirits.",
        }
    }

    pub fn spawn_types(&self) -> Vec<UndeadType> {
        match self {
            Self::Graveyard => vec![
                UndeadType::SkeletonWarrior, UndeadType::ZombieShambler,
                UndeadType::GhostSpectre, UndeadType::Wight,
            ],
            Self::Crypt => vec![
                UndeadType::SkeletonKnight, UndeadType::Mummy,
                UndeadType::GhostWraith, UndeadType::Draugr,
            ],
            Self::Battlefield => vec![
                UndeadType::SkeletonWarrior, UndeadType::SkeletonArcher,
                UndeadType::DeathKnight, UndeadType::Revenant,
            ],
            Self::PlaguePit => vec![
                UndeadType::ZombiePlague, UndeadType::ZombieBloated,
                UndeadType::PlagueBringer, UndeadType::CorpseBeast,
            ],
            Self::Catacomb => vec![
                UndeadType::SkeletonWarrior, UndeadType::SkeletonMage,
                UndeadType::GhostSpectre, UndeadType::ShadowStalker,
            ],
            Self::Ossuary => vec![
                UndeadType::SkeletonWarrior, UndeadType::SkeletonGiant,
                UndeadType::BoneHorror, UndeadType::SkeletalDrake,
            ],
            Self::Mausoleum => vec![
                UndeadType::MummyLord, UndeadType::VampireFledgling,
                UndeadType::GhostBanshee, UndeadType::WightLord,
            ],
            Self::MassGrave => vec![
                UndeadType::ZombieShambler, UndeadType::ZombieRunner,
                UndeadType::FleshGolem, UndeadType::NecroticAbomination,
            ],
        }
    }

    pub fn corpse_capacity(&self) -> u32 {
        match self {
            Self::Graveyard => 50,
            Self::Crypt => 20,
            Self::Battlefield => 200,
            Self::PlaguePit => 100,
            Self::Catacomb => 150,
            Self::Ossuary => 300,
            Self::Mausoleum => 15,
            Self::MassGrave => 500,
        }
    }

    pub fn soul_generation(&self) -> u32 {
        match self {
            Self::Graveyard => 2,
            Self::Crypt => 5,
            Self::Battlefield => 10,
            Self::PlaguePit => 3,
            Self::Catacomb => 4,
            Self::Ossuary => 1,
            Self::Mausoleum => 8,
            Self::MassGrave => 15,
        }
    }
}

/// A death domain location
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeathDomain {
    pub id: u64,
    pub name: String,
    pub domain_type: DeathDomainType,
    pub location: (i32, i32),
    pub corpses_available: u32,
    pub souls_available: u32,
    pub is_claimed: bool,
    pub power_level: u32,
    pub corruption: u32,
    pub guardian: Option<UndeadType>,
}

impl DeathDomain {
    pub fn new(id: u64, domain_type: DeathDomainType, location: (i32, i32)) -> Self {
        Self {
            id,
            name: format!("{} #{}", domain_type.name(), id),
            domain_type,
            location,
            corpses_available: domain_type.corpse_capacity(),
            souls_available: domain_type.soul_generation() * 10,
            is_claimed: false,
            power_level: 1,
            corruption: 0,
            guardian: None,
        }
    }

    pub fn claim(&mut self) {
        self.is_claimed = true;
    }

    pub fn set_guardian(&mut self, guardian: UndeadType) {
        self.guardian = Some(guardian);
    }

    pub fn generate_resources(&mut self) {
        if self.is_claimed {
            self.souls_available += self.domain_type.soul_generation() * self.power_level;
            self.corruption += 1;
        }
    }

    pub fn harvest_corpses(&mut self, amount: u32) -> u32 {
        let harvested = amount.min(self.corpses_available);
        self.corpses_available -= harvested;
        harvested
    }

    pub fn harvest_souls(&mut self, amount: u32) -> u32 {
        let harvested = amount.min(self.souls_available);
        self.souls_available -= harvested;
        harvested
    }

    pub fn available_spawns(&self) -> Vec<UndeadType> {
        self.domain_type.spawn_types()
    }
}

// ============================================================================
// SOUL GEMS AND PHYLACTERIES
// ============================================================================

/// Soul gem sizes
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SoulGemSize {
    Petty,
    Lesser,
    Small,
    Common,
    Greater,
    Grand,
    Black,
}

impl SoulGemSize {
    pub fn capacity(&self) -> u32 {
        match self {
            Self::Petty => 1,
            Self::Lesser => 3,
            Self::Small => 5,
            Self::Common => 15,
            Self::Greater => 40,
            Self::Grand => 100,
            Self::Black => 500,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Petty => "Petty Soul Gem",
            Self::Lesser => "Lesser Soul Gem",
            Self::Small => "Small Soul Gem",
            Self::Common => "Common Soul Gem",
            Self::Greater => "Greater Soul Gem",
            Self::Grand => "Grand Soul Gem",
            Self::Black => "Black Soul Gem",
        }
    }
}

/// A soul gem for storing souls
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SoulGem {
    pub id: u64,
    pub size: SoulGemSize,
    pub souls_stored: u32,
    pub capacity: u32,
    pub is_filled: bool,
    pub contained_soul_type: Option<SoulType>,
}

impl SoulGem {
    pub fn new(id: u64, size: SoulGemSize) -> Self {
        Self {
            id,
            size,
            souls_stored: 0,
            capacity: size.capacity(),
            is_filled: false,
            contained_soul_type: None,
        }
    }

    pub fn store_soul(&mut self, amount: u32, soul_type: SoulType) -> u32 {
        let available_space = self.capacity - self.souls_stored;
        let stored = amount.min(available_space);
        self.souls_stored += stored;
        self.contained_soul_type = Some(soul_type);
        if self.souls_stored >= self.capacity {
            self.is_filled = true;
        }
        stored
    }

    pub fn extract_souls(&mut self, amount: u32) -> u32 {
        let extracted = amount.min(self.souls_stored);
        self.souls_stored -= extracted;
        if self.souls_stored == 0 {
            self.is_filled = false;
            self.contained_soul_type = None;
        }
        extracted
    }

    pub fn is_empty(&self) -> bool {
        self.souls_stored == 0
    }
}

/// A phylactery for lich immortality
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Phylactery {
    pub id: u64,
    pub name: String,
    pub souls_invested: u32,
    pub souls_required: u32,
    pub is_complete: bool,
    pub is_destroyed: bool,
    pub location: Option<(i32, i32)>,
    pub protection_level: u32,
    pub regeneration_time: u32,
    pub times_used: u32,
}

impl Phylactery {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            souls_invested: 0,
            souls_required: PHYLACTERY_SOUL_COST,
            is_complete: false,
            is_destroyed: false,
            location: None,
            protection_level: 1,
            regeneration_time: 100,
            times_used: 0,
        }
    }

    pub fn invest_souls(&mut self, amount: u32) -> bool {
        if self.is_complete {
            return false;
        }
        self.souls_invested += amount;
        if self.souls_invested >= self.souls_required {
            self.is_complete = true;
            true
        } else {
            false
        }
    }

    pub fn set_location(&mut self, location: (i32, i32)) {
        self.location = Some(location);
    }

    pub fn use_for_resurrection(&mut self) -> bool {
        if !self.is_complete || self.is_destroyed {
            return false;
        }
        self.times_used += 1;
        self.souls_invested = self.souls_invested.saturating_sub(10);
        if self.souls_invested < self.souls_required / 2 {
            self.is_complete = false;
        }
        true
    }

    pub fn destroy(&mut self) {
        self.is_destroyed = true;
        self.is_complete = false;
    }

    pub fn upgrade_protection(&mut self, souls: u32) -> bool {
        if souls >= 20 {
            self.protection_level += 1;
            self.regeneration_time = (self.regeneration_time as f32 * 0.9) as u32;
            true
        } else {
            false
        }
    }
}

// ============================================================================
// NECROMANCY SYSTEM
// ============================================================================

/// School mastery tracking
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SchoolMastery {
    pub school: Option<NecromancySchool>,
    pub level: u32,
    pub experience: u64,
    pub xp_to_next: u64,
    pub spells_known: Vec<NecromancySpell>,
}

impl SchoolMastery {
    pub fn new(school: NecromancySchool) -> Self {
        Self {
            school: Some(school),
            level: 1,
            experience: 0,
            xp_to_next: 100,
            spells_known: vec![],
        }
    }

    pub fn gain_xp(&mut self, amount: u64) -> bool {
        self.experience += amount;
        if self.experience >= self.xp_to_next {
            self.experience -= self.xp_to_next;
            self.level += 1;
            self.xp_to_next = (self.xp_to_next as f32 * 1.5) as u64;
            true
        } else {
            false
        }
    }

    pub fn can_learn(&self, spell: &NecromancySpell) -> bool {
        spell.tier as u32 <= (self.level + 1) / 2
    }

    pub fn learn_spell(&mut self, spell: NecromancySpell) -> bool {
        if self.can_learn(&spell) && !self.spells_known.iter().any(|s| s.name == spell.name) {
            self.spells_known.push(spell);
            true
        } else {
            false
        }
    }
}

/// The main necromancy system
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NecromancySystem {
    // Rank and progression
    pub rank: NecromancerRank,
    pub total_xp: u64,
    pub souls_collected: u64,
    pub corpses_raised: u64,

    // School masteries
    pub masteries: HashMap<NecromancySchool, SchoolMastery>,
    pub primary_school: Option<NecromancySchool>,
    pub secondary_school: Option<NecromancySchool>,

    // Army management
    pub army: UndeadArmy,

    // Death domains
    pub claimed_domains: Vec<DeathDomain>,

    // Soul storage
    pub soul_gems: Vec<SoulGem>,
    pub loose_souls: u32,

    // Phylactery (for lich transformation)
    pub phylactery: Option<Phylactery>,
    pub is_lich: bool,

    // Statistics
    pub total_damage_dealt: u64,
    pub enemies_killed: u64,
    pub minions_lost: u64,
    pub spells_cast: u64,

    // Internal tracking
    next_gem_id: u64,
    next_domain_id: u64,
}

impl Default for NecromancySystem {
    fn default() -> Self {
        Self::new()
    }
}

impl NecromancySystem {
    pub fn new() -> Self {
        Self {
            rank: NecromancerRank::GraveRobber,
            total_xp: 0,
            souls_collected: 0,
            corpses_raised: 0,
            masteries: HashMap::new(),
            primary_school: None,
            secondary_school: None,
            army: UndeadArmy::new(BASE_CONTROL_LIMIT),
            claimed_domains: Vec::new(),
            soul_gems: Vec::new(),
            loose_souls: 0,
            phylactery: None,
            is_lich: false,
            total_damage_dealt: 0,
            enemies_killed: 0,
            minions_lost: 0,
            spells_cast: 0,
            next_gem_id: 1,
            next_domain_id: 1,
        }
    }

    // === Progression ===

    pub fn gain_xp(&mut self, amount: u64) {
        self.total_xp += amount;
        self.check_rank_up();
    }

    fn check_rank_up(&mut self) {
        if let Some(next) = self.rank.next_rank() {
            if self.total_xp >= next.required_xp() {
                self.rank = next;
                self.army.control_limit = self.calculate_control_limit();
            }
        }
    }

    pub fn calculate_control_limit(&self) -> u32 {
        let base = self.rank.control_limit();
        let school_bonus: u32 = self.masteries.values()
            .filter_map(|m| m.school.map(|s| s.bonuses().control_limit_bonus))
            .sum();
        base + school_bonus
    }

    // === School Management ===

    pub fn specialize(&mut self, school: NecromancySchool) {
        if self.primary_school.is_none() {
            self.primary_school = Some(school);
        } else if self.secondary_school.is_none() && self.primary_school != Some(school) {
            self.secondary_school = Some(school);
        }
        self.masteries.entry(school).or_insert_with(|| SchoolMastery::new(school));
    }

    pub fn get_mastery(&self, school: NecromancySchool) -> Option<&SchoolMastery> {
        self.masteries.get(&school)
    }

    pub fn get_mastery_mut(&mut self, school: NecromancySchool) -> Option<&mut SchoolMastery> {
        self.masteries.get_mut(&school)
    }

    pub fn learn_spell(&mut self, school: NecromancySchool, spell: NecromancySpell) -> bool {
        if let Some(mastery) = self.masteries.get_mut(&school) {
            mastery.learn_spell(spell)
        } else {
            false
        }
    }

    pub fn all_known_spells(&self) -> Vec<&NecromancySpell> {
        self.masteries.values()
            .flat_map(|m| m.spells_known.iter())
            .collect()
    }

    // === Army Management ===

    pub fn raise_undead(&mut self, undead_type: UndeadType) -> Result<u64, &'static str> {
        if undead_type.required_rank() > self.rank {
            return Err("Rank too low to raise this undead");
        }

        let id = self.army.add_minion(undead_type)?;
        self.corpses_raised += 1;
        self.gain_xp(undead_type.control_cost() as u64 * 10);
        Ok(id)
    }

    pub fn dismiss_undead(&mut self, id: u64) -> Option<UndeadMinion> {
        self.army.remove_minion(id)
    }

    pub fn upgrade_minion(&mut self, id: u64, upgrade: UndeadUpgrade) -> bool {
        let cost = upgrade.soul_cost();
        if self.loose_souls < cost {
            return false;
        }

        if let Some(minion) = self.army.get_minion_mut(id) {
            minion.apply_upgrade(upgrade);
            self.loose_souls -= cost;
            true
        } else {
            false
        }
    }

    pub fn set_formation(&mut self, formation: ArmyFormation) {
        self.army.set_formation(formation);
    }

    pub fn feed_minion(&mut self, id: u64, souls: u32) -> bool {
        if self.loose_souls < souls {
            return false;
        }

        if let Some(minion) = self.army.get_minion_mut(id) {
            minion.feed(souls);
            self.loose_souls -= souls;
            true
        } else {
            false
        }
    }

    // === Soul Management ===

    pub fn collect_soul(&mut self, amount: u32, soul_type: SoulType) {
        self.souls_collected += amount as u64;

        // Try to store in gems first
        let mut remaining = amount;
        for gem in &mut self.soul_gems {
            if !gem.is_filled {
                let stored = gem.store_soul(remaining, soul_type);
                remaining -= stored;
                if remaining == 0 {
                    break;
                }
            }
        }

        // Store remainder as loose souls
        self.loose_souls += remaining;
    }

    pub fn create_soul_gem(&mut self, size: SoulGemSize) -> u64 {
        let id = self.next_gem_id;
        self.next_gem_id += 1;
        self.soul_gems.push(SoulGem::new(id, size));
        id
    }

    pub fn extract_from_gem(&mut self, gem_id: u64, amount: u32) -> u32 {
        if let Some(gem) = self.soul_gems.iter_mut().find(|g| g.id == gem_id) {
            let extracted = gem.extract_souls(amount);
            self.loose_souls += extracted;
            extracted
        } else {
            0
        }
    }

    // === Death Domains ===

    pub fn claim_domain(&mut self, domain_type: DeathDomainType, location: (i32, i32)) -> u64 {
        let id = self.next_domain_id;
        self.next_domain_id += 1;
        let mut domain = DeathDomain::new(id, domain_type, location);
        domain.claim();
        self.claimed_domains.push(domain);
        id
    }

    pub fn get_domain(&self, id: u64) -> Option<&DeathDomain> {
        self.claimed_domains.iter().find(|d| d.id == id)
    }

    pub fn get_domain_mut(&mut self, id: u64) -> Option<&mut DeathDomain> {
        self.claimed_domains.iter_mut().find(|d| d.id == id)
    }

    pub fn harvest_from_domain(&mut self, id: u64, corpses: u32, souls: u32) -> (u32, u32) {
        if let Some(domain) = self.get_domain_mut(id) {
            let harvested_corpses = domain.harvest_corpses(corpses);
            let harvested_souls = domain.harvest_souls(souls);
            self.loose_souls += harvested_souls;
            (harvested_corpses, harvested_souls)
        } else {
            (0, 0)
        }
    }

    // === Phylactery and Lich Transformation ===

    pub fn begin_phylactery(&mut self, name: String) -> bool {
        if self.phylactery.is_some() || self.rank < NecromancerRank::DeathMage {
            return false;
        }

        self.phylactery = Some(Phylactery::new(1, name));
        true
    }

    pub fn invest_in_phylactery(&mut self, souls: u32) -> bool {
        if self.loose_souls < souls {
            return false;
        }

        if let Some(ref mut phylactery) = self.phylactery {
            self.loose_souls -= souls;
            if phylactery.invest_souls(souls) {
                // Phylactery complete - can become lich
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn become_lich(&mut self) -> bool {
        if let Some(ref phylactery) = self.phylactery {
            if phylactery.is_complete && !self.is_lich && self.rank >= NecromancerRank::DeathMage {
                self.is_lich = true;
                self.rank = NecromancerRank::Lich;
                self.army.control_limit = self.calculate_control_limit();
                return true;
            }
        }
        false
    }

    pub fn resurrect_via_phylactery(&mut self) -> bool {
        if let Some(ref mut phylactery) = self.phylactery {
            phylactery.use_for_resurrection()
        } else {
            false
        }
    }

    // === Tick/Update ===

    pub fn tick(&mut self) {
        // Update army
        self.army.tick();

        // Generate resources from domains
        for domain in &mut self.claimed_domains {
            domain.generate_resources();
        }

        // Update spell cooldowns
        for mastery in self.masteries.values_mut() {
            for spell in &mut mastery.spells_known {
                spell.tick();
            }
        }
    }

    // === Statistics ===

    pub fn army_power(&self) -> i32 {
        self.army.total_power()
    }

    pub fn total_minions(&self) -> usize {
        self.army.minions.len()
    }

    pub fn control_available(&self) -> u32 {
        self.army.control_limit - self.army.control_used
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undead_types() {
        assert!(UndeadType::all().len() >= 30);
        for undead in UndeadType::all() {
            assert!(!undead.name().is_empty());
        }
    }

    #[test]
    fn test_necromancy_schools() {
        for school in NecromancySchool::all() {
            assert!(!school.spells().is_empty());
            assert!(school.spells().len() >= 10);
        }
    }

    #[test]
    fn test_necromancer_ranks() {
        let ranks = NecromancerRank::all();
        assert_eq!(ranks.len(), 8);
        for i in 0..ranks.len() - 1 {
            assert!(ranks[i].required_xp() < ranks[i + 1].required_xp());
        }
    }

    #[test]
    fn test_necromancy_system() {
        let mut system = NecromancySystem::new();
        assert_eq!(system.rank, NecromancerRank::GraveRobber);

        // Raise a skeleton
        let id = system.raise_undead(UndeadType::SkeletonWarrior).unwrap();
        assert_eq!(system.total_minions(), 1);

        // Collect souls
        system.collect_soul(10, SoulType::Mortal);
        assert_eq!(system.loose_souls, 10);

        // Dismiss undead
        system.dismiss_undead(id);
        assert_eq!(system.total_minions(), 0);
    }

    #[test]
    fn test_undead_army() {
        let mut army = UndeadArmy::new(10);
        let id = army.add_minion(UndeadType::SkeletonWarrior).unwrap();
        assert_eq!(army.minions.len(), 1);
        assert_eq!(army.control_used, 1);

        army.remove_minion(id);
        assert_eq!(army.minions.len(), 0);
        assert_eq!(army.control_used, 0);
    }

    #[test]
    fn test_soul_gems() {
        let mut gem = SoulGem::new(1, SoulGemSize::Lesser);
        assert_eq!(gem.capacity, 3);

        gem.store_soul(2, SoulType::Mortal);
        assert_eq!(gem.souls_stored, 2);
        assert!(!gem.is_filled);

        gem.store_soul(10, SoulType::Mortal);
        assert_eq!(gem.souls_stored, 3);
        assert!(gem.is_filled);
    }

    #[test]
    fn test_phylactery() {
        let mut phylactery = Phylactery::new(1, "Dark Vessel".to_string());
        assert!(!phylactery.is_complete);

        phylactery.invest_souls(PHYLACTERY_SOUL_COST);
        assert!(phylactery.is_complete);

        assert!(phylactery.use_for_resurrection());
    }

    #[test]
    fn test_death_domains() {
        let mut domain = DeathDomain::new(1, DeathDomainType::Graveyard, (0, 0));
        assert!(!domain.is_claimed);

        domain.claim();
        assert!(domain.is_claimed);

        let corpses = domain.harvest_corpses(10);
        assert_eq!(corpses, 10);
    }
}
