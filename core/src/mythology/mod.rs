//! Mythology and Divine Beings System
//!
//! Gods from 7 pantheons, divine beasts, demons, and mythological creatures
//! from Hindu, Buddhist, Norse, Greek, Japanese, Chinese, and Egyptian mythology.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rand::prelude::*;

/// The 7 mythological pantheons
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Pantheon {
    Hindu,
    Buddhist,
    Norse,
    Greek,
    Japanese,
    Chinese,
    Egyptian,
}

impl Pantheon {
    pub fn all() -> &'static [Pantheon] {
        &[Self::Hindu, Self::Buddhist, Self::Norse, Self::Greek,
          Self::Japanese, Self::Chinese, Self::Egyptian]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Hindu => "Hindu Pantheon",
            Self::Buddhist => "Buddhist Pantheon",
            Self::Norse => "Norse Pantheon",
            Self::Greek => "Greek Pantheon",
            Self::Japanese => "Shinto Pantheon",
            Self::Chinese => "Chinese Pantheon",
            Self::Egyptian => "Egyptian Pantheon",
        }
    }

    pub fn realm(&self) -> &'static str {
        match self {
            Self::Hindu => "Svarga",
            Self::Buddhist => "Nirvana",
            Self::Norse => "Asgard",
            Self::Greek => "Mount Olympus",
            Self::Japanese => "Takamagahara",
            Self::Chinese => "Heavenly Court",
            Self::Egyptian => "Duat",
        }
    }

    pub fn gods(&self) -> Vec<God> {
        match self {
            Self::Hindu => vec![
                God::new("Brahma", *self, GodDomain::Creation, GodTier::Supreme),
                God::new("Vishnu", *self, GodDomain::Preservation, GodTier::Supreme),
                God::new("Shiva", *self, GodDomain::Destruction, GodTier::Supreme),
                God::new("Indra", *self, GodDomain::Thunder, GodTier::Major),
                God::new("Agni", *self, GodDomain::Fire, GodTier::Major),
                God::new("Vayu", *self, GodDomain::Wind, GodTier::Major),
                God::new("Varuna", *self, GodDomain::Water, GodTier::Major),
                God::new("Yama", *self, GodDomain::Death, GodTier::Major),
                God::new("Surya", *self, GodDomain::Sun, GodTier::Major),
                God::new("Chandra", *self, GodDomain::Moon, GodTier::Major),
                God::new("Ganesha", *self, GodDomain::Wisdom, GodTier::Major),
                God::new("Hanuman", *self, GodDomain::Strength, GodTier::Major),
                God::new("Kali", *self, GodDomain::Destruction, GodTier::Major),
                God::new("Lakshmi", *self, GodDomain::Fortune, GodTier::Major),
                God::new("Saraswati", *self, GodDomain::Knowledge, GodTier::Major),
                God::new("Durga", *self, GodDomain::War, GodTier::Major),
                God::new("Krishna", *self, GodDomain::Love, GodTier::Major),
                God::new("Rama", *self, GodDomain::Virtue, GodTier::Major),
                God::new("Kartikeya", *self, GodDomain::War, GodTier::Minor),
                God::new("Kubera", *self, GodDomain::Wealth, GodTier::Minor),
            ],
            Self::Buddhist => vec![
                God::new("Buddha", *self, GodDomain::Enlightenment, GodTier::Supreme),
                God::new("Avalokiteshvara", *self, GodDomain::Compassion, GodTier::Major),
                God::new("Manjushri", *self, GodDomain::Wisdom, GodTier::Major),
                God::new("Vajrapani", *self, GodDomain::Power, GodTier::Major),
                God::new("Tara", *self, GodDomain::Protection, GodTier::Major),
                God::new("Amitabha", *self, GodDomain::Light, GodTier::Supreme),
                God::new("Vairocana", *self, GodDomain::Truth, GodTier::Supreme),
                God::new("Akshobhya", *self, GodDomain::Mirror, GodTier::Major),
                God::new("Ratnasambhava", *self, GodDomain::Wealth, GodTier::Major),
                God::new("Amoghasiddhi", *self, GodDomain::Action, GodTier::Major),
                God::new("Maitreya", *self, GodDomain::Future, GodTier::Major),
                God::new("Ksitigarbha", *self, GodDomain::Earth, GodTier::Major),
                God::new("Mahakala", *self, GodDomain::Time, GodTier::Major),
                God::new("Yamantaka", *self, GodDomain::Death, GodTier::Major),
                God::new("Hayagriva", *self, GodDomain::Knowledge, GodTier::Minor),
            ],
            Self::Norse => vec![
                God::new("Odin", *self, GodDomain::Wisdom, GodTier::Supreme),
                God::new("Thor", *self, GodDomain::Thunder, GodTier::Major),
                God::new("Loki", *self, GodDomain::Trickery, GodTier::Major),
                God::new("Freya", *self, GodDomain::Love, GodTier::Major),
                God::new("Freyr", *self, GodDomain::Fertility, GodTier::Major),
                God::new("Tyr", *self, GodDomain::War, GodTier::Major),
                God::new("Heimdall", *self, GodDomain::Vigilance, GodTier::Major),
                God::new("Baldur", *self, GodDomain::Light, GodTier::Major),
                God::new("Hel", *self, GodDomain::Death, GodTier::Major),
                God::new("Njord", *self, GodDomain::Sea, GodTier::Major),
                God::new("Skadi", *self, GodDomain::Winter, GodTier::Major),
                God::new("Bragi", *self, GodDomain::Poetry, GodTier::Minor),
                God::new("Idun", *self, GodDomain::Youth, GodTier::Minor),
                God::new("Vidar", *self, GodDomain::Vengeance, GodTier::Minor),
                God::new("Forseti", *self, GodDomain::Justice, GodTier::Minor),
            ],
            Self::Greek => vec![
                God::new("Zeus", *self, GodDomain::Thunder, GodTier::Supreme),
                God::new("Poseidon", *self, GodDomain::Sea, GodTier::Supreme),
                God::new("Hades", *self, GodDomain::Death, GodTier::Supreme),
                God::new("Athena", *self, GodDomain::Wisdom, GodTier::Major),
                God::new("Apollo", *self, GodDomain::Sun, GodTier::Major),
                God::new("Artemis", *self, GodDomain::Hunt, GodTier::Major),
                God::new("Ares", *self, GodDomain::War, GodTier::Major),
                God::new("Aphrodite", *self, GodDomain::Love, GodTier::Major),
                God::new("Hephaestus", *self, GodDomain::Forge, GodTier::Major),
                God::new("Hermes", *self, GodDomain::Trickery, GodTier::Major),
                God::new("Dionysus", *self, GodDomain::Wine, GodTier::Major),
                God::new("Demeter", *self, GodDomain::Harvest, GodTier::Major),
                God::new("Hera", *self, GodDomain::Marriage, GodTier::Major),
                God::new("Hestia", *self, GodDomain::Hearth, GodTier::Minor),
                God::new("Persephone", *self, GodDomain::Spring, GodTier::Minor),
                God::new("Hecate", *self, GodDomain::Magic, GodTier::Minor),
                God::new("Nike", *self, GodDomain::Victory, GodTier::Minor),
                God::new("Nemesis", *self, GodDomain::Vengeance, GodTier::Minor),
                God::new("Thanatos", *self, GodDomain::Death, GodTier::Minor),
                God::new("Hypnos", *self, GodDomain::Sleep, GodTier::Minor),
            ],
            Self::Japanese => vec![
                God::new("Amaterasu", *self, GodDomain::Sun, GodTier::Supreme),
                God::new("Tsukuyomi", *self, GodDomain::Moon, GodTier::Major),
                God::new("Susanoo", *self, GodDomain::Storm, GodTier::Major),
                God::new("Izanagi", *self, GodDomain::Creation, GodTier::Supreme),
                God::new("Izanami", *self, GodDomain::Death, GodTier::Supreme),
                God::new("Raijin", *self, GodDomain::Thunder, GodTier::Major),
                God::new("Fujin", *self, GodDomain::Wind, GodTier::Major),
                God::new("Inari", *self, GodDomain::Fortune, GodTier::Major),
                God::new("Bishamon", *self, GodDomain::War, GodTier::Major),
                God::new("Benzaiten", *self, GodDomain::Arts, GodTier::Major),
                God::new("Daikokuten", *self, GodDomain::Wealth, GodTier::Major),
                God::new("Ebisu", *self, GodDomain::Fishing, GodTier::Minor),
                God::new("Hotei", *self, GodDomain::Happiness, GodTier::Minor),
                God::new("Fukurokuju", *self, GodDomain::Longevity, GodTier::Minor),
                God::new("Jurojin", *self, GodDomain::Wisdom, GodTier::Minor),
                God::new("Ryujin", *self, GodDomain::Sea, GodTier::Major),
                God::new("Hachiman", *self, GodDomain::War, GodTier::Major),
                God::new("Kagutsuchi", *self, GodDomain::Fire, GodTier::Major),
            ],
            Self::Chinese => vec![
                God::new("Jade Emperor", *self, GodDomain::Heaven, GodTier::Supreme),
                God::new("Nuwa", *self, GodDomain::Creation, GodTier::Supreme),
                God::new("Fuxi", *self, GodDomain::Knowledge, GodTier::Supreme),
                God::new("Pangu", *self, GodDomain::Creation, GodTier::Supreme),
                God::new("Guan Yu", *self, GodDomain::War, GodTier::Major),
                God::new("Sun Wukong", *self, GodDomain::Trickery, GodTier::Major),
                God::new("Nezha", *self, GodDomain::Protection, GodTier::Major),
                God::new("Erlang Shen", *self, GodDomain::Truth, GodTier::Major),
                God::new("Lei Gong", *self, GodDomain::Thunder, GodTier::Major),
                God::new("Dian Mu", *self, GodDomain::Lightning, GodTier::Major),
                God::new("Yan Wang", *self, GodDomain::Death, GodTier::Major),
                God::new("Long Wang", *self, GodDomain::Sea, GodTier::Major),
                God::new("Zhu Rong", *self, GodDomain::Fire, GodTier::Major),
                God::new("Gong Gong", *self, GodDomain::Water, GodTier::Major),
                God::new("Xi Wangmu", *self, GodDomain::Immortality, GodTier::Major),
                God::new("Caishen", *self, GodDomain::Wealth, GodTier::Minor),
                God::new("Mazu", *self, GodDomain::Sea, GodTier::Minor),
                God::new("Guanyin", *self, GodDomain::Compassion, GodTier::Major),
            ],
            Self::Egyptian => vec![
                God::new("Ra", *self, GodDomain::Sun, GodTier::Supreme),
                God::new("Osiris", *self, GodDomain::Death, GodTier::Supreme),
                God::new("Isis", *self, GodDomain::Magic, GodTier::Supreme),
                God::new("Horus", *self, GodDomain::Sky, GodTier::Major),
                God::new("Set", *self, GodDomain::Chaos, GodTier::Major),
                God::new("Anubis", *self, GodDomain::Death, GodTier::Major),
                God::new("Thoth", *self, GodDomain::Wisdom, GodTier::Major),
                God::new("Bastet", *self, GodDomain::Protection, GodTier::Major),
                God::new("Sekhmet", *self, GodDomain::War, GodTier::Major),
                God::new("Hathor", *self, GodDomain::Love, GodTier::Major),
                God::new("Ptah", *self, GodDomain::Forge, GodTier::Major),
                God::new("Sobek", *self, GodDomain::Strength, GodTier::Major),
                God::new("Khnum", *self, GodDomain::Creation, GodTier::Major),
                God::new("Ma'at", *self, GodDomain::Justice, GodTier::Major),
                God::new("Nephthys", *self, GodDomain::Night, GodTier::Major),
                God::new("Nut", *self, GodDomain::Sky, GodTier::Major),
                God::new("Geb", *self, GodDomain::Earth, GodTier::Major),
                God::new("Ammit", *self, GodDomain::Judgment, GodTier::Minor),
                God::new("Aten", *self, GodDomain::Sun, GodTier::Major),
                God::new("Khonsu", *self, GodDomain::Moon, GodTier::Minor),
            ],
        }
    }
}

/// God's domain of power
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GodDomain {
    Creation, Destruction, Preservation,
    Sun, Moon, Sky, Earth, Sea, Thunder, Lightning, Storm,
    Fire, Water, Wind, Ice, Nature,
    War, Peace, Victory, Vengeance,
    Death, Life, Immortality, Youth,
    Wisdom, Knowledge, Truth, Enlightenment,
    Love, Marriage, Fertility, Beauty,
    Fortune, Wealth, Harvest, Fishing,
    Magic, Trickery, Chaos, Time,
    Forge, Arts, Poetry, Wine,
    Hunt, Protection, Justice, Judgment,
    Compassion, Virtue, Happiness, Longevity,
    Sleep, Night, Spring, Winter,
    Heaven, Hearth, Light, Mirror,
    Power, Action, Future, Vigilance,
    Strength,
}

impl GodDomain {
    pub fn blessing_effect(&self) -> DivineBlessing {
        match self {
            Self::Sun | Self::Light => DivineBlessing::DamageBoost(15),
            Self::Moon | Self::Night => DivineBlessing::StealthBoost(20),
            Self::War | Self::Victory => DivineBlessing::AttackBoost(20),
            Self::Protection | Self::Hearth => DivineBlessing::DefenseBoost(20),
            Self::Death => DivineBlessing::LifeSteal(10),
            Self::Life | Self::Immortality => DivineBlessing::Regeneration(5),
            Self::Wisdom | Self::Knowledge => DivineBlessing::XPBoost(25),
            Self::Fortune | Self::Wealth => DivineBlessing::GoldBoost(30),
            Self::Magic => DivineBlessing::ManaBoost(30),
            Self::Thunder | Self::Lightning | Self::Storm => DivineBlessing::CritBoost(15),
            Self::Fire => DivineBlessing::BurnDamage(10),
            Self::Ice | Self::Winter => DivineBlessing::SlowEnemies(20),
            Self::Strength | Self::Power => DivineBlessing::StrengthBoost(15),
            Self::Trickery => DivineBlessing::DodgeBoost(15),
            Self::Compassion => DivineBlessing::HealingBoost(25),
            _ => DivineBlessing::AllStatsBoost(5),
        }
    }
}

/// God tier/power level
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum GodTier {
    Minor = 1,
    Major = 2,
    Supreme = 3,
}

impl GodTier {
    pub fn blessing_duration(&self) -> u32 {
        match self {
            Self::Minor => 50,
            Self::Major => 100,
            Self::Supreme => 200,
        }
    }

    pub fn favor_requirement(&self) -> i32 {
        match self {
            Self::Minor => 50,
            Self::Major => 100,
            Self::Supreme => 200,
        }
    }
}

/// A deity
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct God {
    pub name: String,
    pub pantheon: Pantheon,
    pub domain: GodDomain,
    pub tier: GodTier,
    pub description: String,
    pub avatar_glyph: char,
}

impl God {
    pub fn new(name: &str, pantheon: Pantheon, domain: GodDomain, tier: GodTier) -> Self {
        Self {
            name: name.to_string(),
            pantheon,
            domain,
            tier,
            description: format!("{} of {:?}", name, domain),
            avatar_glyph: name.chars().next().unwrap_or('G'),
        }
    }

    pub fn blessing(&self) -> DivineBlessing {
        self.domain.blessing_effect()
    }
}

/// Divine blessings from gods
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum DivineBlessing {
    AttackBoost(i32),
    DefenseBoost(i32),
    StrengthBoost(i32),
    ManaBoost(i32),
    HealingBoost(i32),
    XPBoost(i32),
    GoldBoost(i32),
    CritBoost(i32),
    DodgeBoost(i32),
    StealthBoost(i32),
    DamageBoost(i32),
    LifeSteal(i32),
    Regeneration(i32),
    BurnDamage(i32),
    SlowEnemies(i32),
    AllStatsBoost(i32),
}

/// Divine beasts from mythology
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DivineBeast {
    // Japanese
    NineTailedFox,      // Kitsune
    Kirin,              // Japanese unicorn
    Tengu,              // Bird demon
    Kappa,              // Water creature
    Oni,                // Demon
    Yamata,             // Eight-headed serpent

    // Chinese
    Dragon,             // Long
    Phoenix,            // Fenghuang
    Qilin,              // Chinese unicorn
    Tortoise,           // Xuanwu
    WhiteTiger,         // Baihu
    AzureDragon,        // Qinglong
    VermilionBird,      // Zhuque
    BlackTortoise,      // Xuanwu
    Pixiu,              // Wealth beast
    Yinglong,           // Winged dragon

    // Greek
    Cerberus,           // Three-headed dog
    Hydra,              // Multi-headed serpent
    Chimera,            // Lion-goat-serpent
    Pegasus,            // Winged horse
    Minotaur,           // Bull-man
    Medusa,             // Gorgon
    Typhon,             // Father of monsters
    Griffin,            // Eagle-lion
    Sphinx,             // Human-lion
    Cyclops,            // One-eyed giant

    // Norse
    Fenrir,             // Giant wolf
    Jormungandr,        // World serpent
    Sleipnir,           // Eight-legged horse
    Nidhogg,            // Dragon
    Huginn,             // Raven of thought
    Muninn,             // Raven of memory

    // Egyptian
    Ammut,              // Crocodile-lion-hippo
    Bennu,              // Phoenix
    Apep,               // Chaos serpent
    EgyptianSphinx, // Great sphinx
    Serpopard,          // Serpent-leopard

    // Hindu
    Garuda,             // Eagle deity
    Naga,               // Serpent deity
    Makara,             // Sea creature
    Airavata,           // White elephant
    Kamadhenu,          // Divine cow
    Vasuki,             // King of serpents

    // Universal
    Leviathan,          // Sea monster
    Behemoth,           // Land beast
    Thunderbird,        // Storm bird
    Basilisk,           // King of serpents
    Manticore,          // Lion-scorpion
    Kraken,             // Giant squid
}

impl DivineBeast {
    pub fn all() -> Vec<DivineBeast> {
        vec![
            Self::NineTailedFox, Self::Kirin, Self::Tengu, Self::Kappa, Self::Oni, Self::Yamata,
            Self::Dragon, Self::Phoenix, Self::Qilin, Self::Tortoise, Self::WhiteTiger,
            Self::AzureDragon, Self::VermilionBird, Self::BlackTortoise, Self::Pixiu, Self::Yinglong,
            Self::Cerberus, Self::Hydra, Self::Chimera, Self::Pegasus, Self::Minotaur, Self::Medusa,
            Self::Typhon, Self::Griffin, Self::Sphinx, Self::EgyptianSphinx, Self::Cyclops,
            Self::Fenrir, Self::Jormungandr, Self::Sleipnir, Self::Nidhogg, Self::Huginn, Self::Muninn,
            Self::Ammut, Self::Bennu, Self::Apep, Self::Serpopard,
            Self::Garuda, Self::Naga, Self::Makara, Self::Airavata, Self::Kamadhenu, Self::Vasuki,
            Self::Leviathan, Self::Behemoth, Self::Thunderbird, Self::Basilisk, Self::Manticore, Self::Kraken,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::NineTailedFox => "Nine-Tailed Fox",
            Self::Kirin => "Kirin",
            Self::Tengu => "Tengu",
            Self::Kappa => "Kappa",
            Self::Oni => "Oni",
            Self::Yamata => "Yamata no Orochi",
            Self::Dragon => "Eastern Dragon",
            Self::Phoenix => "Fenghuang",
            Self::Qilin => "Qilin",
            Self::Tortoise => "Black Tortoise",
            Self::WhiteTiger => "White Tiger",
            Self::AzureDragon => "Azure Dragon",
            Self::VermilionBird => "Vermilion Bird",
            Self::BlackTortoise => "Black Tortoise",
            Self::Pixiu => "Pixiu",
            Self::Yinglong => "Yinglong",
            Self::Cerberus => "Cerberus",
            Self::Hydra => "Lernaean Hydra",
            Self::Chimera => "Chimera",
            Self::Pegasus => "Pegasus",
            Self::Minotaur => "Minotaur",
            Self::Medusa => "Medusa",
            Self::Typhon => "Typhon",
            Self::Griffin => "Griffin",
            Self::Sphinx => "Greek Sphinx",
            Self::EgyptianSphinx => "Egyptian Sphinx",
            Self::Cyclops => "Cyclops",
            Self::Fenrir => "Fenrir",
            Self::Jormungandr => "Jormungandr",
            Self::Sleipnir => "Sleipnir",
            Self::Nidhogg => "Nidhogg",
            Self::Huginn => "Huginn",
            Self::Muninn => "Muninn",
            Self::Ammut => "Ammit",
            Self::Bennu => "Bennu",
            Self::Apep => "Apophis",
            Self::Serpopard => "Serpopard",
            Self::Garuda => "Garuda",
            Self::Naga => "Naga Raja",
            Self::Makara => "Makara",
            Self::Airavata => "Airavata",
            Self::Kamadhenu => "Kamadhenu",
            Self::Vasuki => "Vasuki",
            Self::Leviathan => "Leviathan",
            Self::Behemoth => "Behemoth",
            Self::Thunderbird => "Thunderbird",
            Self::Basilisk => "Basilisk",
            Self::Manticore => "Manticore",
            Self::Kraken => "Kraken",
        }
    }

    pub fn glyph(&self) -> char {
        match self {
            Self::NineTailedFox => '狐',
            Self::Dragon | Self::AzureDragon | Self::Yinglong => '龍',
            Self::Phoenix | Self::VermilionBird | Self::Bennu => '鳳',
            Self::Cerberus => 'C',
            Self::Hydra => 'H',
            Self::Fenrir => 'F',
            Self::Kraken => 'K',
            Self::Leviathan => 'L',
            Self::EgyptianSphinx => 'S',
            Self::Garuda => 'G',
            _ => self.name().chars().next().unwrap_or('B'),
        }
    }

    pub fn tier(&self) -> BeastTier {
        match self {
            Self::Typhon | Self::Fenrir | Self::Jormungandr | Self::Leviathan |
            Self::Yamata | Self::Nidhogg | Self::Apep => BeastTier::Legendary,
            Self::Cerberus | Self::Hydra | Self::Dragon | Self::Phoenix |
            Self::NineTailedFox | Self::Garuda | Self::Kraken => BeastTier::Mythical,
            _ => BeastTier::Divine,
        }
    }

    pub fn stats(&self) -> BeastStats {
        let tier = self.tier();
        let base = match tier {
            BeastTier::Divine => BeastStats { hp: 500, attack: 50, defense: 40, speed: 30, xp_value: 1000 },
            BeastTier::Mythical => BeastStats { hp: 1000, attack: 80, defense: 60, speed: 40, xp_value: 3000 },
            BeastTier::Legendary => BeastStats { hp: 2000, attack: 120, defense: 80, speed: 50, xp_value: 10000 },
        };
        base
    }

    pub fn abilities(&self) -> Vec<BeastAbility> {
        match self {
            Self::NineTailedFox => vec![
                BeastAbility::Illusion, BeastAbility::CharmGaze, BeastAbility::FoxFire,
                BeastAbility::Shapeshift, BeastAbility::SoulDrain,
            ],
            Self::Cerberus => vec![
                BeastAbility::TripleHeadBite, BeastAbility::HellFire, BeastAbility::Intimidate,
            ],
            Self::Hydra => vec![
                BeastAbility::MultiHeadAttack, BeastAbility::Regeneration, BeastAbility::PoisonBreath,
            ],
            Self::Dragon => vec![
                BeastAbility::DragonBreath, BeastAbility::Flight, BeastAbility::TailSwipe,
                BeastAbility::DragonFear, BeastAbility::AncientWisdom,
            ],
            Self::Phoenix => vec![
                BeastAbility::Rebirth, BeastAbility::FireAura, BeastAbility::HealingFlame,
            ],
            Self::Fenrir => vec![
                BeastAbility::DevouringBite, BeastAbility::Howl, BeastAbility::Rampage,
            ],
            Self::Leviathan => vec![
                BeastAbility::Tsunami, BeastAbility::Devour, BeastAbility::AbyssalDarkness,
            ],
            Self::Kraken => vec![
                BeastAbility::TentacleGrab, BeastAbility::InkCloud, BeastAbility::Crush,
            ],
            _ => vec![BeastAbility::BasicAttack],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BeastTier {
    Divine,
    Mythical,
    Legendary,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct BeastStats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub xp_value: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BeastAbility {
    BasicAttack,
    // Fox abilities
    Illusion, CharmGaze, FoxFire, Shapeshift, SoulDrain,
    // Cerberus
    TripleHeadBite, HellFire, Intimidate,
    // Hydra
    MultiHeadAttack, Regeneration, PoisonBreath,
    // Dragon
    DragonBreath, Flight, TailSwipe, DragonFear, AncientWisdom,
    // Phoenix
    Rebirth, FireAura, HealingFlame,
    // Wolf
    DevouringBite, Howl, Rampage,
    // Sea monsters
    Tsunami, Devour, AbyssalDarkness,
    TentacleGrab, InkCloud, Crush,
}

/// Player's divine favor with gods
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DivineFavor {
    pub favor: HashMap<String, i32>,
    pub active_blessings: Vec<(String, DivineBlessing, u32)>,
    pub patron_god: Option<String>,
    pub offerings_made: u32,
    pub prayers_answered: u32,
}

impl DivineFavor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn gain_favor(&mut self, god_name: &str, amount: i32) {
        let current = self.favor.entry(god_name.to_string()).or_insert(0);
        *current = (*current + amount).clamp(-100, 100);
    }

    pub fn lose_favor(&mut self, god_name: &str, amount: i32) {
        self.gain_favor(god_name, -amount);
    }

    pub fn get_favor(&self, god_name: &str) -> i32 {
        *self.favor.get(god_name).unwrap_or(&0)
    }

    pub fn pray(&mut self, god: &God) -> Option<DivineBlessing> {
        let favor = self.get_favor(&god.name);
        if favor >= god.tier.favor_requirement() {
            let blessing = god.blessing();
            let duration = god.tier.blessing_duration();
            self.active_blessings.push((god.name.clone(), blessing, duration));
            self.prayers_answered += 1;
            Some(blessing)
        } else {
            None
        }
    }

    pub fn make_offering(&mut self, god_name: &str, value: i32) {
        self.gain_favor(god_name, value / 10);
        self.offerings_made += 1;
    }

    pub fn set_patron(&mut self, god_name: &str) {
        self.patron_god = Some(god_name.to_string());
        self.gain_favor(god_name, 25);
    }

    pub fn tick(&mut self) {
        self.active_blessings.retain_mut(|(_, _, duration)| {
            *duration = duration.saturating_sub(1);
            *duration > 0
        });
    }
}

/// Mythology system manager
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MythologySystem {
    pub pantheons: HashMap<Pantheon, Vec<God>>,
    pub divine_beasts: Vec<DivineBeast>,
    pub player_favor: DivineFavor,
    pub discovered_gods: Vec<String>,
    pub defeated_beasts: Vec<DivineBeast>,
    pub shrines_visited: u32,
}

impl MythologySystem {
    pub fn new() -> Self {
        let mut pantheons = HashMap::new();
        for p in Pantheon::all() {
            pantheons.insert(*p, p.gods());
        }

        Self {
            pantheons,
            divine_beasts: DivineBeast::all(),
            player_favor: DivineFavor::new(),
            discovered_gods: Vec::new(),
            defeated_beasts: Vec::new(),
            shrines_visited: 0,
        }
    }

    pub fn get_god(&self, name: &str) -> Option<&God> {
        for gods in self.pantheons.values() {
            if let Some(god) = gods.iter().find(|g| g.name == name) {
                return Some(god);
            }
        }
        None
    }

    pub fn random_god(&self, rng: &mut impl Rng) -> Option<&God> {
        let all_gods: Vec<&God> = self.pantheons.values().flatten().collect();
        if all_gods.is_empty() {
            None
        } else {
            Some(all_gods[rng.gen_range(0..all_gods.len())])
        }
    }

    pub fn random_beast(&self, rng: &mut impl Rng) -> Option<DivineBeast> {
        if self.divine_beasts.is_empty() {
            None
        } else {
            Some(self.divine_beasts[rng.gen_range(0..self.divine_beasts.len())])
        }
    }

    pub fn discover_god(&mut self, name: &str) {
        if !self.discovered_gods.contains(&name.to_string()) {
            self.discovered_gods.push(name.to_string());
        }
    }

    pub fn defeat_beast(&mut self, beast: DivineBeast) {
        if !self.defeated_beasts.contains(&beast) {
            self.defeated_beasts.push(beast);
        }
    }

    pub fn tick(&mut self) {
        self.player_favor.tick();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pantheons() {
        for pantheon in Pantheon::all() {
            let gods = pantheon.gods();
            assert!(!gods.is_empty(), "{:?} should have gods", pantheon);
        }
    }

    #[test]
    fn test_divine_beasts() {
        let beasts = DivineBeast::all();
        assert!(beasts.len() >= 40, "Should have at least 40 divine beasts");
    }

    #[test]
    fn test_divine_favor() {
        let mut favor = DivineFavor::new();
        favor.gain_favor("Zeus", 50);
        assert_eq!(favor.get_favor("Zeus"), 50);
    }
}
