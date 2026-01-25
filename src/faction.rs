// ============================================================================
// FACTION AND REPUTATION SYSTEM
// ============================================================================

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crossterm::style::Color;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Faction {
    // Major factions
    Kingdom,        // Human kingdom
    ElvenCourt,     // High Elves
    DarkElves,      // Drow
    DwarvenClans,   // Mountain dwarves
    OrcHorde,       // Orcs and goblins
    UndeadLegion,   // Undead forces
    DemonCult,      // Demon worshippers
    DragonFlight,   // Dragon alliance

    // Minor factions
    ThievesGuild,
    MagesCircle,
    HolyChurch,
    MerchantGuild,
    Adventurers,
    Druids,
    Assassins,
    Pirates,
}

impl Faction {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Kingdom => "Kingdom of Men",
            Self::ElvenCourt => "Elven Court",
            Self::DarkElves => "Dark Elves",
            Self::DwarvenClans => "Dwarven Clans",
            Self::OrcHorde => "Orc Horde",
            Self::UndeadLegion => "Undead Legion",
            Self::DemonCult => "Demon Cult",
            Self::DragonFlight => "Dragon Flight",
            Self::ThievesGuild => "Thieves Guild",
            Self::MagesCircle => "Mages Circle",
            Self::HolyChurch => "Holy Church",
            Self::MerchantGuild => "Merchant Guild",
            Self::Adventurers => "Adventurers Guild",
            Self::Druids => "Druid Circle",
            Self::Assassins => "Assassins Brotherhood",
            Self::Pirates => "Pirate Consortium",
        }
    }

    #[allow(dead_code)]
    pub fn color(&self) -> Color {
        match self {
            Self::Kingdom => Color::Blue,
            Self::ElvenCourt => Color::Green,
            Self::DarkElves => Color::DarkMagenta,
            Self::DwarvenClans => Color::Yellow,
            Self::OrcHorde => Color::DarkGreen,
            Self::UndeadLegion => Color::Grey,
            Self::DemonCult => Color::Red,
            Self::DragonFlight => Color::DarkRed,
            Self::ThievesGuild => Color::DarkGrey,
            Self::MagesCircle => Color::Cyan,
            Self::HolyChurch => Color::White,
            Self::MerchantGuild => Color::Yellow,
            Self::Adventurers => Color::DarkYellow,
            Self::Druids => Color::Green,
            Self::Assassins => Color::DarkGrey,
            Self::Pirates => Color::DarkCyan,
        }
    }

    #[allow(dead_code)]
    pub fn all() -> [Faction; 16] {
        [
            Self::Kingdom, Self::ElvenCourt, Self::DarkElves, Self::DwarvenClans,
            Self::OrcHorde, Self::UndeadLegion, Self::DemonCult, Self::DragonFlight,
            Self::ThievesGuild, Self::MagesCircle, Self::HolyChurch, Self::MerchantGuild,
            Self::Adventurers, Self::Druids, Self::Assassins, Self::Pirates,
        ]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ReputationLevel {
    Hated,      // -1000 to -500
    Hostile,    // -500 to -200
    Unfriendly, // -200 to -50
    Neutral,    // -50 to 50
    Friendly,   // 50 to 200
    Honored,    // 200 to 500
    Revered,    // 500 to 1000
    Exalted,    // 1000+
}

impl ReputationLevel {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Hated => "Hated",
            Self::Hostile => "Hostile",
            Self::Unfriendly => "Unfriendly",
            Self::Neutral => "Neutral",
            Self::Friendly => "Friendly",
            Self::Honored => "Honored",
            Self::Revered => "Revered",
            Self::Exalted => "Exalted",
        }
    }

    #[allow(dead_code)]
    pub fn color(&self) -> Color {
        match self {
            Self::Hated => Color::DarkRed,
            Self::Hostile => Color::Red,
            Self::Unfriendly => Color::DarkYellow,
            Self::Neutral => Color::Grey,
            Self::Friendly => Color::Green,
            Self::Honored => Color::Blue,
            Self::Revered => Color::Magenta,
            Self::Exalted => Color::Yellow,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Reputation {
    pub factions: HashMap<Faction, i32>,
}

impl Default for Reputation {
    fn default() -> Self {
        Self::new()
    }
}

impl Reputation {
    pub fn new() -> Self {
        let mut factions = HashMap::new();
        // Start neutral with most factions
        for faction in [
            Faction::Kingdom, Faction::ElvenCourt, Faction::DwarvenClans,
            Faction::ThievesGuild, Faction::MagesCircle, Faction::HolyChurch,
            Faction::MerchantGuild, Faction::Adventurers, Faction::Druids,
            Faction::DragonFlight, Faction::Pirates,
        ] {
            factions.insert(faction, 0);
        }
        // Start hostile with evil factions
        factions.insert(Faction::OrcHorde, -200);
        factions.insert(Faction::UndeadLegion, -300);
        factions.insert(Faction::DemonCult, -500);
        factions.insert(Faction::DarkElves, -100);
        factions.insert(Faction::Assassins, -50);

        Self { factions }
    }

    pub fn get(&self, faction: Faction) -> i32 {
        *self.factions.get(&faction).unwrap_or(&0)
    }

    pub fn modify(&mut self, faction: Faction, amount: i32) {
        let current = self.get(faction);
        self.factions.insert(faction, (current + amount).clamp(-1000, 1500));

        // Reputation with one faction affects others
        self.apply_faction_relationships(faction, amount);
    }

    fn apply_faction_relationships(&mut self, faction: Faction, amount: i32) {
        match faction {
            Faction::Kingdom => {
                self.adjust_related(Faction::OrcHorde, -amount / 2);
                self.adjust_related(Faction::UndeadLegion, -amount / 2);
                self.adjust_related(Faction::HolyChurch, amount / 3);
            }
            Faction::ElvenCourt => {
                self.adjust_related(Faction::DarkElves, -amount);
                self.adjust_related(Faction::OrcHorde, -amount / 2);
                self.adjust_related(Faction::Druids, amount / 2);
            }
            Faction::DarkElves => {
                self.adjust_related(Faction::ElvenCourt, -amount);
                self.adjust_related(Faction::Assassins, amount / 2);
            }
            Faction::DwarvenClans => {
                self.adjust_related(Faction::OrcHorde, -amount / 2);
                self.adjust_related(Faction::Kingdom, amount / 3);
            }
            Faction::OrcHorde => {
                self.adjust_related(Faction::Kingdom, -amount / 2);
                self.adjust_related(Faction::ElvenCourt, -amount / 2);
                self.adjust_related(Faction::DwarvenClans, -amount / 2);
            }
            Faction::UndeadLegion => {
                self.adjust_related(Faction::HolyChurch, -amount);
                self.adjust_related(Faction::Kingdom, -amount / 2);
            }
            Faction::DemonCult => {
                self.adjust_related(Faction::HolyChurch, -amount);
                self.adjust_related(Faction::Kingdom, -amount / 2);
                self.adjust_related(Faction::ElvenCourt, -amount / 2);
            }
            Faction::DragonFlight => {
                // Dragons are independent
            }
            Faction::ThievesGuild => {
                self.adjust_related(Faction::Assassins, amount / 2);
                self.adjust_related(Faction::Kingdom, -amount / 4);
            }
            Faction::MagesCircle => {
                self.adjust_related(Faction::Druids, amount / 3);
            }
            Faction::HolyChurch => {
                self.adjust_related(Faction::UndeadLegion, -amount);
                self.adjust_related(Faction::DemonCult, -amount);
                self.adjust_related(Faction::Kingdom, amount / 3);
            }
            Faction::MerchantGuild => {
                // Merchants are neutral with all
            }
            Faction::Adventurers => {
                // Adventurers are independent
            }
            Faction::Druids => {
                self.adjust_related(Faction::ElvenCourt, amount / 2);
                self.adjust_related(Faction::DemonCult, -amount / 2);
            }
            Faction::Assassins => {
                self.adjust_related(Faction::ThievesGuild, amount / 2);
                self.adjust_related(Faction::HolyChurch, -amount / 3);
            }
            Faction::Pirates => {
                self.adjust_related(Faction::MerchantGuild, -amount / 2);
                self.adjust_related(Faction::Kingdom, -amount / 3);
            }
        }
    }

    fn adjust_related(&mut self, faction: Faction, amount: i32) {
        if amount != 0 {
            let current = self.get(faction);
            self.factions.insert(faction, (current + amount).clamp(-1000, 1500));
        }
    }

    pub fn level(&self, faction: Faction) -> ReputationLevel {
        let rep = self.get(faction);
        match rep {
            r if r <= -500 => ReputationLevel::Hated,
            r if r <= -200 => ReputationLevel::Hostile,
            r if r <= -50 => ReputationLevel::Unfriendly,
            r if r <= 50 => ReputationLevel::Neutral,
            r if r <= 200 => ReputationLevel::Friendly,
            r if r <= 500 => ReputationLevel::Honored,
            r if r <= 1000 => ReputationLevel::Revered,
            _ => ReputationLevel::Exalted,
        }
    }

    #[allow(dead_code)]
    pub fn can_use_services(&self, faction: Faction) -> bool {
        self.get(faction) >= -50
    }

    #[allow(dead_code)]
    pub fn price_modifier(&self, faction: Faction) -> f32 {
        match self.level(faction) {
            ReputationLevel::Hated => 2.0,
            ReputationLevel::Hostile => 1.5,
            ReputationLevel::Unfriendly => 1.2,
            ReputationLevel::Neutral => 1.0,
            ReputationLevel::Friendly => 0.9,
            ReputationLevel::Honored => 0.8,
            ReputationLevel::Revered => 0.7,
            ReputationLevel::Exalted => 0.5,
        }
    }

    #[allow(dead_code)]
    pub fn unlocks(&self, faction: Faction) -> Vec<&'static str> {
        let mut unlocks = vec![];
        let level = self.level(faction);

        match faction {
            Faction::ThievesGuild => {
                if matches!(level, ReputationLevel::Friendly | ReputationLevel::Honored | ReputationLevel::Revered | ReputationLevel::Exalted) {
                    unlocks.push("Access to black market");
                }
                if matches!(level, ReputationLevel::Honored | ReputationLevel::Revered | ReputationLevel::Exalted) {
                    unlocks.push("Fence stolen goods");
                }
                if matches!(level, ReputationLevel::Revered | ReputationLevel::Exalted) {
                    unlocks.push("Assassination contracts");
                }
            }
            Faction::MagesCircle => {
                if matches!(level, ReputationLevel::Friendly | ReputationLevel::Honored | ReputationLevel::Revered | ReputationLevel::Exalted) {
                    unlocks.push("Advanced spell training");
                }
                if matches!(level, ReputationLevel::Honored | ReputationLevel::Revered | ReputationLevel::Exalted) {
                    unlocks.push("Enchanting services");
                }
                if matches!(level, ReputationLevel::Revered | ReputationLevel::Exalted) {
                    unlocks.push("Archmage spells");
                }
            }
            Faction::HolyChurch => {
                if matches!(level, ReputationLevel::Friendly | ReputationLevel::Honored | ReputationLevel::Revered | ReputationLevel::Exalted) {
                    unlocks.push("Blessed healing");
                }
                if matches!(level, ReputationLevel::Honored | ReputationLevel::Revered | ReputationLevel::Exalted) {
                    unlocks.push("Holy water crafting");
                }
                if matches!(level, ReputationLevel::Revered | ReputationLevel::Exalted) {
                    unlocks.push("Divine blessing");
                }
            }
            Faction::DragonFlight => {
                if matches!(level, ReputationLevel::Honored | ReputationLevel::Revered | ReputationLevel::Exalted) {
                    unlocks.push("Dragon scale armor");
                }
                if matches!(level, ReputationLevel::Exalted) {
                    unlocks.push("Dragon mount");
                }
            }
            Faction::Kingdom => {
                if matches!(level, ReputationLevel::Friendly | ReputationLevel::Honored | ReputationLevel::Revered | ReputationLevel::Exalted) {
                    unlocks.push("Royal armory access");
                }
                if matches!(level, ReputationLevel::Honored | ReputationLevel::Revered | ReputationLevel::Exalted) {
                    unlocks.push("Knight title");
                }
                if matches!(level, ReputationLevel::Exalted) {
                    unlocks.push("Noble estate");
                }
            }
            Faction::DwarvenClans => {
                if matches!(level, ReputationLevel::Friendly | ReputationLevel::Honored | ReputationLevel::Revered | ReputationLevel::Exalted) {
                    unlocks.push("Dwarven smithing");
                }
                if matches!(level, ReputationLevel::Honored | ReputationLevel::Revered | ReputationLevel::Exalted) {
                    unlocks.push("Rune crafting");
                }
                if matches!(level, ReputationLevel::Exalted) {
                    unlocks.push("Mithril gear");
                }
            }
            Faction::ElvenCourt => {
                if matches!(level, ReputationLevel::Friendly | ReputationLevel::Honored | ReputationLevel::Revered | ReputationLevel::Exalted) {
                    unlocks.push("Elven archery training");
                }
                if matches!(level, ReputationLevel::Honored | ReputationLevel::Revered | ReputationLevel::Exalted) {
                    unlocks.push("Nature magic");
                }
                if matches!(level, ReputationLevel::Exalted) {
                    unlocks.push("Elven lore secrets");
                }
            }
            _ => {}
        }
        unlocks
    }

    pub fn reputation_gain_for_kill(&self, faction: Faction) -> i32 {
        // Killing enemies from evil factions gives positive reputation with good factions
        match faction {
            Faction::OrcHorde => 5,
            Faction::UndeadLegion => 8,
            Faction::DemonCult => 15,
            Faction::DarkElves => 3,
            _ => 0,
        }
    }
}
