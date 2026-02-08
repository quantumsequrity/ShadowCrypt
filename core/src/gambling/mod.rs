//! Gambling System: NPCs, mystery items, and risk/reward mechanics
//!
//! This module provides a complete gambling system for the ShadowCrypt roguelike.
//! Players can interact with various gambling NPCs to play games of chance,
//! purchase mystery items, and engage in high-risk/high-reward activities.
//!
//! # Features
//!
//! - **Gambling NPCs**: Various NPCs offering different gambling activities
//! - **Mystery Items**: Unidentified items sold at discounted prices
//! - **Mini-Games**: Dice games, card games, wheel of fortune, etc.
//! - **Risk Tiers**: Different risk levels with corresponding rewards
//! - **Streak System**: Bonuses for consecutive wins/losses

use std::collections::HashMap;
use rand::prelude::*;
use serde::{Serialize, Deserialize};

use crate::items::{Item, ItemKind, Rarity};

/// Maximum gold that can be wagered in a single bet
pub const MAX_WAGER: u32 = 10000;

/// Minimum gold required to participate in gambling
pub const MIN_WAGER: u32 = 10;

/// Streak threshold for bonus rewards
pub const STREAK_BONUS_THRESHOLD: u32 = 3;

/// All gambling NPC types available in the game
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum GamblerKind {
    // Common Gamblers (found in taverns, early levels)
    ShadyDealer,       // Sells mystery items, offers rigged games
    DiceMaster,        // Specializes in dice games
    CardShark,         // Card game specialist

    // Uncommon Gamblers (mid-game)
    FortuneWheel,      // Wheel of fortune operator
    CoinFlipper,       // Simple high/low coin games
    BetBroker,         // Takes bets on combat outcomes

    // Rare Gamblers (late game, higher stakes)
    HighRoller,        // Luxury gambling, high minimums
    MysteryMerchant,   // Sells rare mystery boxes
    SoulTrader,        // Gambles with HP/stats instead of gold

    // Legendary Gamblers (special encounters)
    LuckDemon,         // Demonic gambler, extreme stakes
    FateWeaver,        // Can alter luck temporarily
    GoldenDragon,      // Dragon hoarder, massive jackpots
}

impl GamblerKind {
    /// Returns the display name of this gambler type
    pub fn name(&self) -> &'static str {
        match self {
            Self::ShadyDealer => "Shady Dealer",
            Self::DiceMaster => "Dice Master",
            Self::CardShark => "Card Shark",
            Self::FortuneWheel => "Fortune Wheel Operator",
            Self::CoinFlipper => "Coin Flipper",
            Self::BetBroker => "Bet Broker",
            Self::HighRoller => "High Roller",
            Self::MysteryMerchant => "Mystery Merchant",
            Self::SoulTrader => "Soul Trader",
            Self::LuckDemon => "Luck Demon",
            Self::FateWeaver => "Fate Weaver",
            Self::GoldenDragon => "Golden Dragon",
        }
    }

    /// Returns the glyph character for this gambler
    pub fn glyph(&self) -> char {
        match self {
            Self::ShadyDealer => '$',
            Self::DiceMaster => 'd',
            Self::CardShark => 'c',
            Self::FortuneWheel => 'W',
            Self::CoinFlipper => 'o',
            Self::BetBroker => 'B',
            Self::HighRoller => 'H',
            Self::MysteryMerchant => 'M',
            Self::SoulTrader => 'S',
            Self::LuckDemon => '&',
            Self::FateWeaver => 'F',
            Self::GoldenDragon => 'D',
        }
    }

    /// Returns a color index for UI rendering
    pub fn color_index(&self) -> u8 {
        match self {
            Self::ShadyDealer => 0,       // Dark grey
            Self::DiceMaster => 2,        // White
            Self::CardShark => 3,         // Red
            Self::FortuneWheel => 11,     // Yellow
            Self::CoinFlipper => 11,      // Yellow
            Self::BetBroker => 5,         // Green
            Self::HighRoller => 13,       // Magenta
            Self::MysteryMerchant => 7,   // Blue
            Self::SoulTrader => 4,        // Dark red
            Self::LuckDemon => 3,         // Red
            Self::FateWeaver => 9,        // Cyan
            Self::GoldenDragon => 11,     // Yellow/Gold
        }
    }

    /// Returns the rarity tier of this gambler
    pub fn rarity(&self) -> GamblerRarity {
        match self {
            Self::ShadyDealer | Self::DiceMaster | Self::CardShark => GamblerRarity::Common,
            Self::FortuneWheel | Self::CoinFlipper | Self::BetBroker => GamblerRarity::Uncommon,
            Self::HighRoller | Self::MysteryMerchant | Self::SoulTrader => GamblerRarity::Rare,
            Self::LuckDemon | Self::FateWeaver | Self::GoldenDragon => GamblerRarity::Legendary,
        }
    }

    /// Returns the minimum gold required to interact with this gambler
    pub fn min_gold(&self) -> u32 {
        match self {
            Self::ShadyDealer => 10,
            Self::DiceMaster => 25,
            Self::CardShark => 50,
            Self::FortuneWheel => 100,
            Self::CoinFlipper => 20,
            Self::BetBroker => 75,
            Self::HighRoller => 500,
            Self::MysteryMerchant => 200,
            Self::SoulTrader => 0,  // Trades HP, not gold
            Self::LuckDemon => 1000,
            Self::FateWeaver => 250,
            Self::GoldenDragon => 2000,
        }
    }

    /// Returns the games this gambler offers
    pub fn available_games(&self) -> Vec<GamblingGame> {
        match self {
            Self::ShadyDealer => vec![
                GamblingGame::MysteryBox,
                GamblingGame::ThreeCardMonte,
                GamblingGame::RiggedDice,
            ],
            Self::DiceMaster => vec![
                GamblingGame::HighLowDice,
                GamblingGame::CrapsDice,
                GamblingGame::LuckyNumber,
            ],
            Self::CardShark => vec![
                GamblingGame::Blackjack,
                GamblingGame::HighCard,
                GamblingGame::ThreeCardMonte,
            ],
            Self::FortuneWheel => vec![
                GamblingGame::WheelOfFortune,
                GamblingGame::BigWheel,
            ],
            Self::CoinFlipper => vec![
                GamblingGame::CoinFlip,
                GamblingGame::DoubleTrouble,
            ],
            Self::BetBroker => vec![
                GamblingGame::CombatBet,
                GamblingGame::BossPool,
            ],
            Self::HighRoller => vec![
                GamblingGame::Blackjack,
                GamblingGame::WheelOfFortune,
                GamblingGame::HighStakesDice,
            ],
            Self::MysteryMerchant => vec![
                GamblingGame::MysteryBox,
                GamblingGame::LegendaryLottery,
                GamblingGame::CursedChest,
            ],
            Self::SoulTrader => vec![
                GamblingGame::SoulWager,
                GamblingGame::LifeOrDeath,
                GamblingGame::StatGamble,
            ],
            Self::LuckDemon => vec![
                GamblingGame::DemonDeal,
                GamblingGame::SoulWager,
                GamblingGame::AllOrNothing,
            ],
            Self::FateWeaver => vec![
                GamblingGame::FateSpin,
                GamblingGame::DestinyDraw,
                GamblingGame::LuckBlessing,
            ],
            Self::GoldenDragon => vec![
                GamblingGame::DragonHoard,
                GamblingGame::LegendaryLottery,
                GamblingGame::AllOrNothing,
            ],
        }
    }

    /// Returns the dialogue greeting for this gambler
    pub fn greeting(&self) -> &'static str {
        match self {
            Self::ShadyDealer => "Psst... hey friend, wanna see what I got in my coat?",
            Self::DiceMaster => "Step right up! The dice never lie... most of the time.",
            Self::CardShark => "Care for a game of cards? I promise to go easy on you.",
            Self::FortuneWheel => "Spin the wheel of destiny! Everyone's a winner... eventually.",
            Self::CoinFlipper => "Heads or tails? Simple as that. What could go wrong?",
            Self::BetBroker => "Place your bets! Who will survive the next encounter?",
            Self::HighRoller => "Ah, a player of distinguished taste. Shall we raise the stakes?",
            Self::MysteryMerchant => "Mysteries await within these boxes. Dare to open one?",
            Self::SoulTrader => "Gold is worthless to me. But your life force... that interests me.",
            Self::LuckDemon => "Mortal! Your soul smells of ambition. Let's make a wager...",
            Self::FateWeaver => "I see the threads of your destiny. Let me... rearrange them.",
            Self::GoldenDragon => "You seek my hoard? Prove your worth through fortune's test.",
        }
    }

    /// Returns whether this gambler can be found at the given dungeon level
    pub fn available_at_level(&self, level: u32) -> bool {
        match self.rarity() {
            GamblerRarity::Common => level >= 1,
            GamblerRarity::Uncommon => level >= 5,
            GamblerRarity::Rare => level >= 12,
            GamblerRarity::Legendary => level >= 20,
        }
    }

    /// Returns a random gambler appropriate for the dungeon level
    pub fn random_for_level(level: u32, rng: &mut impl Rng) -> Self {
        let available: Vec<Self> = [
            Self::ShadyDealer, Self::DiceMaster, Self::CardShark,
            Self::FortuneWheel, Self::CoinFlipper, Self::BetBroker,
            Self::HighRoller, Self::MysteryMerchant, Self::SoulTrader,
            Self::LuckDemon, Self::FateWeaver, Self::GoldenDragon,
        ].into_iter()
            .filter(|k| k.available_at_level(level))
            .collect();

        available[rng.gen_range(0..available.len())]
    }
}

/// Rarity tiers for gambling NPCs
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub enum GamblerRarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

impl GamblerRarity {
    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Common => "Common",
            Self::Uncommon => "Uncommon",
            Self::Rare => "Rare",
            Self::Legendary => "Legendary",
        }
    }

    /// Returns the spawn chance per floor (per 1000)
    pub fn spawn_chance(&self) -> u32 {
        match self {
            Self::Common => 150,      // 15%
            Self::Uncommon => 80,     // 8%
            Self::Rare => 30,         // 3%
            Self::Legendary => 10,    // 1%
        }
    }
}

/// All gambling games available
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum GamblingGame {
    // Dice Games
    HighLowDice,      // Guess if next roll is higher or lower
    CrapsDice,        // Classic craps rules
    LuckyNumber,      // Pick a number 1-6, roll matches
    RiggedDice,       // Looks fair but house always wins slightly
    HighStakesDice,   // Same as HighLow but 10x stakes

    // Card Games
    Blackjack,        // Get closest to 21
    HighCard,         // Draw highest card wins
    ThreeCardMonte,   // Find the queen

    // Wheel Games
    WheelOfFortune,   // Spin for multipliers
    BigWheel,         // Large prizes, low odds

    // Coin Games
    CoinFlip,         // 50/50 double or nothing
    DoubleTrouble,    // Must win 2 flips in a row

    // Mystery Items
    MysteryBox,       // Random item, usually bad
    CursedChest,      // High risk, could be cursed or legendary
    LegendaryLottery, // Tiny chance for legendary item

    // Combat/Meta Games
    CombatBet,        // Bet on surviving next fight
    BossPool,         // Bet on boss kill time

    // Soul/Stat Games (no gold)
    SoulWager,        // Bet HP for rewards
    LifeOrDeath,      // All or nothing HP gamble
    StatGamble,       // Risk stats for bonuses

    // Special Games
    DemonDeal,        // Make a deal with a demon
    FateSpin,         // Fate wheel with varied outcomes
    DestinyDraw,      // Draw cards that affect your fate
    LuckBlessing,     // Pay for temporary luck boost
    DragonHoard,      // Access dragon's treasure
    AllOrNothing,     // Bet everything for massive reward
}

impl GamblingGame {
    /// Returns the display name of this game
    pub fn name(&self) -> &'static str {
        match self {
            Self::HighLowDice => "High-Low Dice",
            Self::CrapsDice => "Craps",
            Self::LuckyNumber => "Lucky Number",
            Self::RiggedDice => "Fair Dice",  // Ironic name
            Self::HighStakesDice => "High Stakes Dice",
            Self::Blackjack => "Blackjack",
            Self::HighCard => "High Card Draw",
            Self::ThreeCardMonte => "Three Card Monte",
            Self::WheelOfFortune => "Wheel of Fortune",
            Self::BigWheel => "Big Wheel",
            Self::CoinFlip => "Coin Flip",
            Self::DoubleTrouble => "Double Trouble",
            Self::MysteryBox => "Mystery Box",
            Self::CursedChest => "Cursed Chest",
            Self::LegendaryLottery => "Legendary Lottery",
            Self::CombatBet => "Combat Bet",
            Self::BossPool => "Boss Pool",
            Self::SoulWager => "Soul Wager",
            Self::LifeOrDeath => "Life or Death",
            Self::StatGamble => "Stat Gamble",
            Self::DemonDeal => "Demon's Deal",
            Self::FateSpin => "Fate Spin",
            Self::DestinyDraw => "Destiny Draw",
            Self::LuckBlessing => "Luck Blessing",
            Self::DragonHoard => "Dragon's Hoard",
            Self::AllOrNothing => "All or Nothing",
        }
    }

    /// Returns a description of the game rules
    pub fn description(&self) -> &'static str {
        match self {
            Self::HighLowDice => "Guess if the next dice roll will be higher or lower than 3.5",
            Self::CrapsDice => "Roll 7 or 11 on first roll to win, 2/3/12 loses, otherwise set point",
            Self::LuckyNumber => "Pick a number 1-6. Roll the dice. Match to win 5x your bet!",
            Self::RiggedDice => "A totally fair dice game. Trust me.",
            Self::HighStakesDice => "High-Low but with 10x minimum bet",
            Self::Blackjack => "Get closest to 21 without going over. Beat the dealer!",
            Self::HighCard => "Draw a card. Higher card wins. Simple as that.",
            Self::ThreeCardMonte => "Find the queen among three cards. Easy... right?",
            Self::WheelOfFortune => "Spin the wheel for multipliers from 0x to 10x!",
            Self::BigWheel => "Massive jackpots, tiny odds. Dream big!",
            Self::CoinFlip => "Heads or tails. Win to double your money!",
            Self::DoubleTrouble => "Win two coin flips in a row to quadruple your bet!",
            Self::MysteryBox => "Buy a mystery box. Contents unknown. Results vary.",
            Self::CursedChest => "Open a cursed chest. Could be trash, could be treasure, could be... worse.",
            Self::LegendaryLottery => "1 in 100 chance for a legendary item. Feeling lucky?",
            Self::CombatBet => "Bet gold on surviving your next combat encounter",
            Self::BossPool => "Contribute to the boss pool. Kill the boss fast for bonus rewards!",
            Self::SoulWager => "Wager your life force instead of gold. Risk HP for rewards.",
            Self::LifeOrDeath => "Bet half your HP. Win to restore double, lose to take the hit.",
            Self::StatGamble => "Risk a permanent stat point for a chance at greater power.",
            Self::DemonDeal => "Make a deal with a demon. Great rewards, terrible consequences.",
            Self::FateSpin => "Spin the wheel of fate. Outcomes affect your entire run.",
            Self::DestinyDraw => "Draw a destiny card. Your future hangs in the balance.",
            Self::LuckBlessing => "Pay gold for a temporary luck boost on all games.",
            Self::DragonHoard => "Access the dragon's hoard. High entry, high rewards.",
            Self::AllOrNothing => "Bet EVERYTHING. Win big or lose it all.",
        }
    }

    /// Returns the risk tier of this game
    pub fn risk_tier(&self) -> RiskTier {
        match self {
            Self::CoinFlip | Self::HighCard | Self::HighLowDice => RiskTier::Low,
            Self::Blackjack | Self::WheelOfFortune | Self::LuckyNumber |
            Self::MysteryBox | Self::DoubleTrouble => RiskTier::Medium,
            Self::CrapsDice | Self::ThreeCardMonte | Self::BigWheel |
            Self::HighStakesDice | Self::CombatBet | Self::CursedChest => RiskTier::High,
            Self::LegendaryLottery | Self::SoulWager | Self::BossPool |
            Self::FateSpin | Self::DestinyDraw | Self::StatGamble => RiskTier::VeryHigh,
            Self::AllOrNothing | Self::DemonDeal | Self::LifeOrDeath |
            Self::DragonHoard | Self::RiggedDice | Self::LuckBlessing => RiskTier::Extreme,
        }
    }

    /// Returns the base odds of winning (percentage)
    pub fn base_win_chance(&self) -> u32 {
        match self {
            Self::CoinFlip => 50,
            Self::HighLowDice => 50,
            Self::HighCard => 45,
            Self::Blackjack => 42,
            Self::WheelOfFortune => 65,  // But varied payouts
            Self::LuckyNumber => 16,     // 1 in 6
            Self::CrapsDice => 49,
            Self::ThreeCardMonte => 25,  // Appears 33% but rigged
            Self::BigWheel => 15,
            Self::DoubleTrouble => 25,   // 50% * 50%
            Self::MysteryBox => 40,      // Chance of "good" item
            Self::CursedChest => 30,
            Self::LegendaryLottery => 1,
            Self::CombatBet => 60,       // Depends on player strength
            Self::BossPool => 50,
            Self::SoulWager => 45,
            Self::LifeOrDeath => 50,
            Self::StatGamble => 35,
            Self::DemonDeal => 40,
            Self::FateSpin => 50,        // Varied outcomes
            Self::DestinyDraw => 45,
            Self::LuckBlessing => 100,   // Always "works", but is it worth it?
            Self::DragonHoard => 33,
            Self::AllOrNothing => 20,
            Self::RiggedDice => 35,      // Advertised as 50%
            Self::HighStakesDice => 48,
        }
    }

    /// Returns the default wager multiplier on win
    pub fn win_multiplier(&self) -> f32 {
        match self {
            Self::CoinFlip => 2.0,
            Self::HighLowDice => 1.9,
            Self::HighCard => 2.0,
            Self::Blackjack => 2.0,
            Self::WheelOfFortune => 1.5,  // Average, actual varies
            Self::LuckyNumber => 5.0,
            Self::CrapsDice => 2.0,
            Self::ThreeCardMonte => 3.0,
            Self::BigWheel => 10.0,
            Self::DoubleTrouble => 4.0,
            Self::MysteryBox => 1.0,      // Item based
            Self::CursedChest => 1.0,     // Item based
            Self::LegendaryLottery => 100.0,
            Self::CombatBet => 1.5,
            Self::BossPool => 3.0,
            Self::SoulWager => 2.5,
            Self::LifeOrDeath => 2.0,
            Self::StatGamble => 1.0,      // Stat based
            Self::DemonDeal => 5.0,
            Self::FateSpin => 1.0,        // Effect based
            Self::DestinyDraw => 1.0,     // Effect based
            Self::LuckBlessing => 1.0,    // Buff based
            Self::DragonHoard => 8.0,
            Self::AllOrNothing => 20.0,
            Self::RiggedDice => 2.0,
            Self::HighStakesDice => 2.0,
        }
    }

    /// Returns true if this game uses gold as wager
    pub fn uses_gold(&self) -> bool {
        !matches!(self,
            Self::SoulWager | Self::LifeOrDeath | Self::StatGamble
        )
    }

    /// Returns the minimum bet for this game
    pub fn min_bet(&self) -> u32 {
        match self {
            Self::CoinFlip => 10,
            Self::HighLowDice => 20,
            Self::HighCard => 25,
            Self::Blackjack => 50,
            Self::WheelOfFortune => 30,
            Self::LuckyNumber => 15,
            Self::CrapsDice => 40,
            Self::ThreeCardMonte => 25,
            Self::BigWheel => 100,
            Self::DoubleTrouble => 20,
            Self::MysteryBox => 50,
            Self::CursedChest => 150,
            Self::LegendaryLottery => 500,
            Self::CombatBet => 100,
            Self::BossPool => 200,
            Self::SoulWager => 10,  // HP
            Self::LifeOrDeath => 0, // Uses half current HP
            Self::StatGamble => 1,  // Stat points
            Self::DemonDeal => 500,
            Self::FateSpin => 100,
            Self::DestinyDraw => 75,
            Self::LuckBlessing => 200,
            Self::DragonHoard => 1000,
            Self::AllOrNothing => 0,  // All gold
            Self::RiggedDice => 30,
            Self::HighStakesDice => 200,
        }
    }
}

/// Risk tiers for gambling activities
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub enum RiskTier {
    Low,
    Medium,
    High,
    VeryHigh,
    Extreme,
}

impl RiskTier {
    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Low => "Low Risk",
            Self::Medium => "Medium Risk",
            Self::High => "High Risk",
            Self::VeryHigh => "Very High Risk",
            Self::Extreme => "EXTREME Risk",
        }
    }

    /// Returns the color index for UI
    pub fn color_index(&self) -> u8 {
        match self {
            Self::Low => 5,        // Green
            Self::Medium => 11,    // Yellow
            Self::High => 6,       // Orange
            Self::VeryHigh => 3,   // Red
            Self::Extreme => 4,    // Dark red
        }
    }
}

/// Mystery box tiers for item purchases
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum MysteryBoxTier {
    Rusty,      // Cheap, mostly junk
    Bronze,     // Common items
    Silver,     // Uncommon with rare chance
    Gold,       // Rare with epic chance
    Platinum,   // Epic with legendary chance
    Void,       // Could be anything, including cursed
}

impl MysteryBoxTier {
    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Rusty => "Rusty Box",
            Self::Bronze => "Bronze Box",
            Self::Silver => "Silver Box",
            Self::Gold => "Gold Box",
            Self::Platinum => "Platinum Box",
            Self::Void => "Void Box",
        }
    }

    /// Returns the cost of this box
    pub fn cost(&self) -> u32 {
        match self {
            Self::Rusty => 25,
            Self::Bronze => 75,
            Self::Silver => 200,
            Self::Gold => 500,
            Self::Platinum => 1500,
            Self::Void => 1000,
        }
    }

    /// Returns the rarity distribution (common, uncommon, rare, epic, legendary, cursed)
    pub fn rarity_chances(&self) -> (u32, u32, u32, u32, u32, u32) {
        match self {
            Self::Rusty =>    (70, 20, 8, 2, 0, 0),
            Self::Bronze =>   (50, 35, 12, 3, 0, 0),
            Self::Silver =>   (25, 40, 25, 8, 2, 0),
            Self::Gold =>     (10, 25, 35, 22, 8, 0),
            Self::Platinum => (5, 10, 25, 35, 25, 0),
            Self::Void =>     (15, 15, 20, 20, 15, 15),  // Can be cursed!
        }
    }

    /// Generate a random item from this box
    pub fn generate_item(&self, x: usize, y: usize, rng: &mut impl Rng) -> (Item, bool) {
        let (common, uncommon, rare, epic, legendary, cursed) = self.rarity_chances();
        let roll = rng.gen_range(0..100);

        let mut acc = 0;
        let (rarity, is_cursed) = if roll < { acc += cursed; acc } {
            (Rarity::Rare, true)  // Cursed items appear rare but are bad
        } else if roll < { acc += common; acc } {
            (Rarity::Common, false)
        } else if roll < { acc += uncommon; acc } {
            (Rarity::Uncommon, false)
        } else if roll < { acc += rare; acc } {
            (Rarity::Rare, false)
        } else if roll < { acc += epic; acc } {
            (Rarity::Epic, false)
        } else if roll < { acc + legendary; acc } {
            (Rarity::Legendary, false)
        } else {
            (Rarity::Mythic, false)
        };

        let item_kind = Self::random_item_kind(rarity, rng);
        (Item::new(x, y, item_kind, rarity), is_cursed)
    }

    /// Get a random item kind appropriate for the rarity
    fn random_item_kind(rarity: Rarity, rng: &mut impl Rng) -> ItemKind {
        let items: Vec<ItemKind> = match rarity {
            Rarity::Common => vec![
                ItemKind::HealthPotion, ItemKind::ManaPotion, ItemKind::Bread,
                ItemKind::Dagger, ItemKind::LeatherArmor, ItemKind::LeatherBoots,
                ItemKind::Torch, ItemKind::Buckler,
            ],
            Rarity::Uncommon => vec![
                ItemKind::StrengthPotion, ItemKind::DefensePotion, ItemKind::ShortSword,
                ItemKind::ChainMail, ItemKind::IronHelm, ItemKind::IronGauntlets,
                ItemKind::ScrollTeleport, ItemKind::WoodenShield,
            ],
            Rarity::Rare => vec![
                ItemKind::RegenerationPotion, ItemKind::LongSword, ItemKind::ScaleMail,
                ItemKind::RingOfStrength, ItemKind::ScrollFireball, ItemKind::BootsOfSpeed,
                ItemKind::AmuletOfHealth, ItemKind::IronShield,
            ],
            Rarity::Epic => vec![
                ItemKind::FullRestorePotion, ItemKind::Greatsword, ItemKind::PlateMail,
                ItemKind::RingOfRegeneration, ItemKind::ScrollTimeStop, ItemKind::WingedBoots,
                ItemKind::AmuletOfPower, ItemKind::DragonShield,
            ],
            Rarity::Legendary => vec![
                ItemKind::UltimatePowerPotion, ItemKind::FlameSword, ItemKind::DragonArmor,
                ItemKind::RingOfTheAncients, ItemKind::ScrollDivineWrath, ItemKind::CrownOfKings,
                ItemKind::AmuletOfTheGods, ItemKind::PhoenixShield,
            ],
            Rarity::Mythic => vec![
                ItemKind::DemonSlayer, ItemKind::TitanPlate, ItemKind::AmuletOfBalance,
                ItemKind::VoidStaff, ItemKind::GauntletsOfMight, ItemKind::BootsOfTheWind,
            ],
        };
        items[rng.gen_range(0..items.len())]
    }
}

/// Possible outcomes from gambling
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GambleOutcome {
    /// Won gold
    WonGold(u32),
    /// Lost gold
    LostGold(u32),
    /// Won an item
    WonItem(Item),
    /// Lost an item (rare)
    LostItem,
    /// Gained HP
    GainedHP(i32),
    /// Lost HP
    LostHP(i32),
    /// Stat change (stat_name, delta)
    StatChange(String, i32),
    /// Gained a buff effect (effect_name, duration)
    GainedBuff(String, u32),
    /// Gained a debuff effect (effect_name, duration)
    GainedDebuff(String, u32),
    /// Special jackpot
    Jackpot(u32, Option<Item>),
    /// Cursed - bad effect
    Cursed(String),
    /// Nothing happened
    Push,
    /// Unlocked something special
    Unlocked(String),
}

impl GambleOutcome {
    /// Returns a description of the outcome
    pub fn description(&self) -> String {
        match self {
            Self::WonGold(amount) => format!("Won {} gold!", amount),
            Self::LostGold(amount) => format!("Lost {} gold.", amount),
            Self::WonItem(item) => format!("Won a {}!", item.display_name()),
            Self::LostItem => "Lost an item!".to_string(),
            Self::GainedHP(amount) => format!("Gained {} HP!", amount),
            Self::LostHP(amount) => format!("Lost {} HP!", amount),
            Self::StatChange(stat, delta) => {
                if *delta > 0 {
                    format!("Gained {} {}!", delta, stat)
                } else {
                    format!("Lost {} {}!", -delta, stat)
                }
            },
            Self::GainedBuff(name, duration) => {
                format!("Gained {} for {} turns!", name, duration)
            },
            Self::GainedDebuff(name, duration) => {
                format!("Afflicted with {} for {} turns!", name, duration)
            },
            Self::Jackpot(gold, item) => {
                if let Some(i) = item {
                    format!("JACKPOT! Won {} gold and a {}!", gold, i.display_name())
                } else {
                    format!("JACKPOT! Won {} gold!", gold)
                }
            },
            Self::Cursed(curse) => format!("CURSED! {}", curse),
            Self::Push => "Push - no winner.".to_string(),
            Self::Unlocked(thing) => format!("Unlocked: {}!", thing),
        }
    }

    /// Returns whether this is a positive outcome
    pub fn is_positive(&self) -> bool {
        match self {
            Self::WonGold(_) | Self::WonItem(_) | Self::GainedHP(_) |
            Self::GainedBuff(_, _) | Self::Jackpot(_, _) | Self::Unlocked(_) => true,
            Self::StatChange(_, delta) => *delta > 0,
            _ => false,
        }
    }
}

/// Player's gambling statistics
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct GamblingStats {
    /// Total gold wagered
    pub total_wagered: u64,
    /// Total gold won
    pub total_won: u64,
    /// Total gold lost
    pub total_lost: u64,
    /// Number of games played
    pub games_played: u32,
    /// Number of games won
    pub games_won: u32,
    /// Current win streak
    pub win_streak: u32,
    /// Current loss streak
    pub loss_streak: u32,
    /// Best win streak ever
    pub best_win_streak: u32,
    /// Worst loss streak ever
    pub worst_loss_streak: u32,
    /// Jackpots hit
    pub jackpots: u32,
    /// Times cursed
    pub times_cursed: u32,
    /// Mystery boxes opened
    pub boxes_opened: u32,
    /// Luck modifier (affected by items/buffs)
    pub luck_modifier: i32,
    /// Per-game statistics
    pub game_stats: HashMap<String, (u32, u32)>,  // (wins, losses)
}

impl GamblingStats {
    /// Create new gambling stats
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a game result
    pub fn record_game(&mut self, game: GamblingGame, won: bool, wagered: u32, result: i64) {
        self.games_played += 1;
        self.total_wagered += wagered as u64;

        if won {
            self.games_won += 1;
            self.total_won += result.max(0) as u64;
            self.win_streak += 1;
            self.loss_streak = 0;
            if self.win_streak > self.best_win_streak {
                self.best_win_streak = self.win_streak;
            }
        } else {
            self.total_lost += (-result).max(0) as u64;
            self.loss_streak += 1;
            self.win_streak = 0;
            if self.loss_streak > self.worst_loss_streak {
                self.worst_loss_streak = self.loss_streak;
            }
        }

        // Update per-game stats
        let game_name = game.name().to_string();
        let entry = self.game_stats.entry(game_name).or_insert((0, 0));
        if won {
            entry.0 += 1;
        } else {
            entry.1 += 1;
        }
    }

    /// Calculate effective win chance with luck modifier
    pub fn effective_win_chance(&self, base_chance: u32) -> u32 {
        let modified = base_chance as i32 + self.luck_modifier;
        modified.clamp(1, 95) as u32
    }

    /// Get the win rate as a percentage
    pub fn win_rate(&self) -> f32 {
        if self.games_played == 0 {
            return 0.0;
        }
        (self.games_won as f32 / self.games_played as f32) * 100.0
    }

    /// Get net profit/loss
    pub fn net_profit(&self) -> i64 {
        self.total_won as i64 - self.total_lost as i64
    }

    /// Check if player qualifies for streak bonus
    pub fn has_streak_bonus(&self) -> bool {
        self.win_streak >= STREAK_BONUS_THRESHOLD
    }

    /// Get streak bonus multiplier
    pub fn streak_bonus(&self) -> f32 {
        if self.win_streak >= STREAK_BONUS_THRESHOLD {
            1.0 + (self.win_streak - STREAK_BONUS_THRESHOLD + 1) as f32 * 0.1
        } else {
            1.0
        }
    }
}

/// A gambling NPC instance
#[derive(Clone, Serialize, Deserialize)]
pub struct Gambler {
    pub kind: GamblerKind,
    pub x: usize,
    pub y: usize,
    /// Custom name (optional)
    pub name: Option<String>,
    /// Reputation with this gambler (-100 to 100)
    pub reputation: i32,
    /// Whether this gambler has been cheated
    pub knows_cheater: bool,
    /// Special modifier for this instance
    pub house_edge_modifier: i32,
    /// Number of interactions
    pub interactions: u32,
    /// Gold this gambler has (for jackpot purposes)
    pub gold_pool: u32,
}

impl Gambler {
    /// Create a new gambler NPC
    pub fn new(kind: GamblerKind, x: usize, y: usize) -> Self {
        let base_pool = match kind.rarity() {
            GamblerRarity::Common => 500,
            GamblerRarity::Uncommon => 2000,
            GamblerRarity::Rare => 10000,
            GamblerRarity::Legendary => 50000,
        };

        Self {
            kind,
            x,
            y,
            name: None,
            reputation: 0,
            knows_cheater: false,
            house_edge_modifier: 0,
            interactions: 0,
            gold_pool: base_pool,
        }
    }

    /// Create a gambler with a custom name
    pub fn new_named(kind: GamblerKind, name: String, x: usize, y: usize) -> Self {
        let mut gambler = Self::new(kind, x, y);
        gambler.name = Some(name);
        gambler
    }

    /// Get the display name
    pub fn display_name(&self) -> String {
        self.name.clone().unwrap_or_else(|| self.kind.name().to_string())
    }

    /// Get the effective win chance for a game (with house edge)
    pub fn effective_win_chance(&self, game: GamblingGame, player_stats: &GamblingStats) -> u32 {
        let base = game.base_win_chance();
        let mut modified = base as i32;

        // Apply house edge
        modified -= self.house_edge_modifier;

        // Apply player luck
        modified += player_stats.luck_modifier;

        // Reputation affects odds slightly
        modified += self.reputation / 20;

        // If gambler knows player is a cheater, reduce odds
        if self.knows_cheater {
            modified -= 10;
        }

        modified.clamp(1, 95) as u32
    }

    /// Play a gambling game
    pub fn play_game(
        &mut self,
        game: GamblingGame,
        wager: u32,
        player_stats: &mut GamblingStats,
        player_gold: u32,
        player_hp: i32,
        rng: &mut impl Rng,
    ) -> Result<Vec<GambleOutcome>, String> {
        // Validate wager
        if game.uses_gold() {
            if wager < game.min_bet() {
                return Err(format!("Minimum bet is {} gold", game.min_bet()));
            }
            if wager > player_gold {
                return Err("Not enough gold!".to_string());
            }
            if wager > MAX_WAGER {
                return Err(format!("Maximum wager is {} gold", MAX_WAGER));
            }
        }

        self.interactions += 1;
        let mut outcomes = Vec::new();

        // Get effective win chance
        let win_chance = self.effective_win_chance(game, player_stats);
        let roll = rng.gen_range(0..100);
        let won = roll < win_chance;

        match game {
            GamblingGame::CoinFlip | GamblingGame::HighLowDice | GamblingGame::HighCard => {
                if won {
                    let winnings = (wager as f32 * game.win_multiplier()) as u32;
                    outcomes.push(GambleOutcome::WonGold(winnings));
                    self.gold_pool = self.gold_pool.saturating_sub(winnings);
                } else {
                    outcomes.push(GambleOutcome::LostGold(wager));
                    self.gold_pool += wager;
                }
            },

            GamblingGame::DoubleTrouble => {
                // Must win two flips
                let flip1 = rng.gen_range(0..100) < win_chance;
                let flip2 = rng.gen_range(0..100) < win_chance;
                if flip1 && flip2 {
                    let winnings = wager * 4;
                    outcomes.push(GambleOutcome::WonGold(winnings));
                } else {
                    outcomes.push(GambleOutcome::LostGold(wager));
                }
            },

            GamblingGame::LuckyNumber => {
                let picked = rng.gen_range(1..=6);
                let rolled = rng.gen_range(1..=6);
                if picked == rolled {
                    let winnings = wager * 5;
                    outcomes.push(GambleOutcome::WonGold(winnings));
                } else {
                    outcomes.push(GambleOutcome::LostGold(wager));
                }
            },

            GamblingGame::WheelOfFortune => {
                let spin = rng.gen_range(0..100);
                let multiplier = match spin {
                    0..=5 => 0.0,      // Bankrupt
                    6..=25 => 0.5,     // Half back
                    26..=50 => 1.0,    // Break even
                    51..=70 => 1.5,    // Small win
                    71..=85 => 2.0,    // Double
                    86..=94 => 3.0,    // Triple
                    95..=98 => 5.0,    // Big win
                    _ => 10.0,         // Jackpot!
                };

                if multiplier >= 10.0 {
                    outcomes.push(GambleOutcome::Jackpot(wager * 10, None));
                    player_stats.jackpots += 1;
                } else if multiplier > 1.0 {
                    outcomes.push(GambleOutcome::WonGold((wager as f32 * multiplier) as u32));
                } else if multiplier < 1.0 {
                    let loss = wager - (wager as f32 * multiplier) as u32;
                    outcomes.push(GambleOutcome::LostGold(loss));
                } else {
                    outcomes.push(GambleOutcome::Push);
                }
            },

            GamblingGame::MysteryBox => {
                let tier = match wager {
                    0..=50 => MysteryBoxTier::Rusty,
                    51..=100 => MysteryBoxTier::Bronze,
                    101..=300 => MysteryBoxTier::Silver,
                    301..=750 => MysteryBoxTier::Gold,
                    751..=2000 => MysteryBoxTier::Platinum,
                    _ => MysteryBoxTier::Void,
                };

                let (item, is_cursed) = tier.generate_item(0, 0, rng);
                player_stats.boxes_opened += 1;

                if is_cursed {
                    outcomes.push(GambleOutcome::Cursed("The item crumbles to dust in your hands!".to_string()));
                    player_stats.times_cursed += 1;
                } else {
                    outcomes.push(GambleOutcome::WonItem(item));
                }
            },

            GamblingGame::CursedChest => {
                let curse_roll = rng.gen_range(0..100);
                if curse_roll < 20 {
                    // Cursed!
                    let curses = [
                        "You are weakened! -2 Attack for 50 turns.",
                        "Your gold turns to lead! Lost 25% of your gold.",
                        "A dark presence drains your life! -20 HP.",
                        "Bad luck follows you! -10% win chance for 20 games.",
                    ];
                    let curse = curses[rng.gen_range(0..curses.len())];
                    outcomes.push(GambleOutcome::Cursed(curse.to_string()));
                    player_stats.times_cursed += 1;
                } else if curse_roll < 40 {
                    // Junk
                    outcomes.push(GambleOutcome::LostGold(wager));
                } else {
                    // Good item!
                    let tier = if curse_roll > 95 {
                        MysteryBoxTier::Platinum
                    } else if curse_roll > 80 {
                        MysteryBoxTier::Gold
                    } else {
                        MysteryBoxTier::Silver
                    };
                    let (item, _) = tier.generate_item(0, 0, rng);
                    outcomes.push(GambleOutcome::WonItem(item));
                }
            },

            GamblingGame::SoulWager => {
                let hp_wager = wager.min(player_hp as u32 / 2);
                if won {
                    let heal = (hp_wager as f32 * 2.5) as i32;
                    outcomes.push(GambleOutcome::GainedHP(heal));
                    // Also win some gold
                    outcomes.push(GambleOutcome::WonGold(hp_wager * 5));
                } else {
                    outcomes.push(GambleOutcome::LostHP(hp_wager as i32));
                }
            },

            GamblingGame::LifeOrDeath => {
                let hp_at_stake = player_hp / 2;
                if won {
                    outcomes.push(GambleOutcome::GainedHP(hp_at_stake));
                    outcomes.push(GambleOutcome::GainedBuff("Death's Reprieve".to_string(), 30));
                } else {
                    outcomes.push(GambleOutcome::LostHP(hp_at_stake));
                    outcomes.push(GambleOutcome::GainedDebuff("Near Death".to_string(), 20));
                }
            },

            GamblingGame::StatGamble => {
                if won {
                    let stats = ["Attack", "Defense", "Max HP", "Max Mana"];
                    let stat = stats[rng.gen_range(0..stats.len())];
                    let gain = rng.gen_range(1..=3);
                    outcomes.push(GambleOutcome::StatChange(stat.to_string(), gain));
                } else {
                    let stats = ["Attack", "Defense"];
                    let stat = stats[rng.gen_range(0..stats.len())];
                    outcomes.push(GambleOutcome::StatChange(stat.to_string(), -1));
                }
            },

            GamblingGame::DemonDeal => {
                if won {
                    // Great rewards
                    outcomes.push(GambleOutcome::WonGold(wager * 5));
                    outcomes.push(GambleOutcome::GainedBuff("Demon's Favor".to_string(), 100));
                    if rng.gen_range(0..100) < 25 {
                        let (item, _) = MysteryBoxTier::Gold.generate_item(0, 0, rng);
                        outcomes.push(GambleOutcome::WonItem(item));
                    }
                } else {
                    // Terrible consequences
                    outcomes.push(GambleOutcome::LostGold(wager));
                    outcomes.push(GambleOutcome::LostHP(20));
                    outcomes.push(GambleOutcome::Cursed("The demon marks your soul!".to_string()));
                    player_stats.times_cursed += 1;
                }
            },

            GamblingGame::AllOrNothing => {
                if won {
                    let jackpot = player_gold * 20;
                    let (item, _) = MysteryBoxTier::Platinum.generate_item(0, 0, rng);
                    outcomes.push(GambleOutcome::Jackpot(jackpot, Some(item)));
                    player_stats.jackpots += 1;
                } else {
                    outcomes.push(GambleOutcome::LostGold(player_gold));
                    outcomes.push(GambleOutcome::Cursed("Fortune has abandoned you...".to_string()));
                }
            },

            GamblingGame::LuckBlessing => {
                // Always "succeeds" but costs gold
                player_stats.luck_modifier += 5;
                outcomes.push(GambleOutcome::GainedBuff("Lucky".to_string(), 50));
            },

            GamblingGame::DragonHoard => {
                if won {
                    let treasure = wager * 8;
                    outcomes.push(GambleOutcome::WonGold(treasure));
                    if rng.gen_range(0..100) < 10 {
                        let (item, _) = MysteryBoxTier::Platinum.generate_item(0, 0, rng);
                        outcomes.push(GambleOutcome::WonItem(item));
                    }
                } else {
                    outcomes.push(GambleOutcome::LostGold(wager));
                    outcomes.push(GambleOutcome::LostHP(10));  // Dragon's displeasure
                }
            },

            // Default handling for other games
            _ => {
                if won {
                    let winnings = (wager as f32 * game.win_multiplier()) as u32;
                    outcomes.push(GambleOutcome::WonGold(winnings));
                } else {
                    outcomes.push(GambleOutcome::LostGold(wager));
                }
            }
        }

        // Record game in stats
        let net_result: i64 = outcomes.iter().map(|o| match o {
            GambleOutcome::WonGold(g) => *g as i64,
            GambleOutcome::LostGold(g) => -(*g as i64),
            GambleOutcome::Jackpot(g, _) => *g as i64,
            _ => 0,
        }).sum();

        player_stats.record_game(game, won, wager, net_result);

        // Update reputation based on outcome
        if won {
            self.reputation = (self.reputation - 1).max(-100);
        } else {
            self.reputation = (self.reputation + 1).min(100);
        }

        Ok(outcomes)
    }

    /// Get the greeting message
    pub fn greeting(&self) -> String {
        let base = self.kind.greeting();
        if self.reputation < -50 {
            format!("{} (They eye you suspiciously)", base)
        } else if self.reputation > 50 {
            format!("{} (They seem pleased to see you)", base)
        } else {
            base.to_string()
        }
    }
}

/// Represents a gambling encounter in the dungeon
#[derive(Clone, Serialize, Deserialize)]
pub struct GamblingDen {
    pub x: usize,
    pub y: usize,
    pub gamblers: Vec<Gambler>,
    pub name: String,
    pub entry_fee: u32,
    pub is_discovered: bool,
}

impl GamblingDen {
    /// Create a new gambling den
    pub fn new(x: usize, y: usize, dungeon_level: u32, rng: &mut impl Rng) -> Self {
        let num_gamblers = rng.gen_range(1..=3);
        let mut gamblers = Vec::new();

        for _ in 0..num_gamblers {
            let kind = GamblerKind::random_for_level(dungeon_level, rng);
            let gx = x + rng.gen_range(0..3);
            let gy = y + rng.gen_range(0..3);
            gamblers.push(Gambler::new(kind, gx, gy));
        }

        let names = [
            "The Lucky Coin", "Shadow's Dice", "Fortune's Folly",
            "The Golden Gambit", "Risky Business", "Dragon's Hoard Casino",
            "The Rusty Wheel", "Midnight Stakes", "The Cursed Table",
        ];

        Self {
            x,
            y,
            gamblers,
            name: names[rng.gen_range(0..names.len())].to_string(),
            entry_fee: dungeon_level * 10,
            is_discovered: false,
        }
    }

    /// Discover this gambling den
    pub fn discover(&mut self) {
        self.is_discovered = true;
    }
}

/// Generate gambling encounters for a dungeon level
pub fn generate_gambling_encounters(
    rooms: &[crate::world::Room],
    dungeon_level: u32,
    rng: &mut impl Rng,
) -> Vec<GamblingDen> {
    let mut dens = Vec::new();

    // Chance of gambling den per floor
    let den_chance = 10 + dungeon_level.min(20) as i32;  // 10-30%

    for (i, room) in rooms.iter().enumerate() {
        // Skip first room (player spawn) and boss rooms
        if i == 0 || i == rooms.len() - 1 {
            continue;
        }

        if rng.gen_range(0..100) < den_chance {
            let (x, y) = room.random_point(rng);
            dens.push(GamblingDen::new(x, y, dungeon_level, rng));
        }
    }

    dens
}

/// Special gambling events that can occur
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum GamblingEvent {
    /// Lucky streak - bonus to all gambling
    LuckyStreak,
    /// Unlucky day - penalties to gambling
    UnluckyDay,
    /// High roller challenge - exclusive high-stakes games
    HighRollerChallenge,
    /// Mystery merchant visit - rare items available
    MysteryMerchantVisit,
    /// Demon's invitation - extreme risk/reward
    DemonInvitation,
    /// Fortune's blessing - free spins/plays
    FortuneBlessing,
}

impl GamblingEvent {
    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::LuckyStreak => "Lucky Streak",
            Self::UnluckyDay => "Unlucky Day",
            Self::HighRollerChallenge => "High Roller Challenge",
            Self::MysteryMerchantVisit => "Mystery Merchant Visit",
            Self::DemonInvitation => "Demon's Invitation",
            Self::FortuneBlessing => "Fortune's Blessing",
        }
    }

    /// Returns a description
    pub fn description(&self) -> &'static str {
        match self {
            Self::LuckyStreak => "Fortune smiles upon you! +15% win chance for all games.",
            Self::UnluckyDay => "Dark clouds gather... -10% win chance for all games.",
            Self::HighRollerChallenge => "A high roller challenges you to an exclusive game!",
            Self::MysteryMerchantVisit => "A mysterious merchant offers rare gambling boxes.",
            Self::DemonInvitation => "A luck demon offers you a deal you can't refuse...",
            Self::FortuneBlessing => "You found a lucky coin! One free game of your choice.",
        }
    }

    /// Get the luck modifier for this event
    pub fn luck_modifier(&self) -> i32 {
        match self {
            Self::LuckyStreak => 15,
            Self::UnluckyDay => -10,
            Self::FortuneBlessing => 20,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gambler_creation() {
        let gambler = Gambler::new(GamblerKind::DiceMaster, 5, 5);
        assert_eq!(gambler.kind, GamblerKind::DiceMaster);
        assert_eq!(gambler.reputation, 0);
        assert!(gambler.gold_pool > 0);
    }

    #[test]
    fn test_gambling_stats() {
        let mut stats = GamblingStats::new();
        assert_eq!(stats.games_played, 0);
        assert_eq!(stats.win_rate(), 0.0);

        stats.record_game(GamblingGame::CoinFlip, true, 100, 100);
        assert_eq!(stats.games_played, 1);
        assert_eq!(stats.games_won, 1);
        assert_eq!(stats.win_streak, 1);

        stats.record_game(GamblingGame::CoinFlip, false, 100, -100);
        assert_eq!(stats.games_played, 2);
        assert_eq!(stats.games_won, 1);
        assert_eq!(stats.win_streak, 0);
        assert_eq!(stats.loss_streak, 1);
    }

    #[test]
    fn test_streak_bonus() {
        let mut stats = GamblingStats::new();

        // No bonus initially
        assert!(!stats.has_streak_bonus());
        assert_eq!(stats.streak_bonus(), 1.0);

        // Build a streak
        for _ in 0..STREAK_BONUS_THRESHOLD {
            stats.record_game(GamblingGame::CoinFlip, true, 10, 10);
        }

        assert!(stats.has_streak_bonus());
        assert!(stats.streak_bonus() > 1.0);
    }

    #[test]
    fn test_mystery_box_generation() {
        let mut rng = rand::thread_rng();

        for tier in [
            MysteryBoxTier::Rusty,
            MysteryBoxTier::Bronze,
            MysteryBoxTier::Silver,
            MysteryBoxTier::Gold,
            MysteryBoxTier::Platinum,
            MysteryBoxTier::Void,
        ] {
            let (item, _is_cursed) = tier.generate_item(0, 0, &mut rng);
            assert!(item.kind.name().len() > 0);
        }
    }

    #[test]
    fn test_risk_tier_ordering() {
        assert!(RiskTier::Low < RiskTier::Medium);
        assert!(RiskTier::Medium < RiskTier::High);
        assert!(RiskTier::High < RiskTier::VeryHigh);
        assert!(RiskTier::VeryHigh < RiskTier::Extreme);
    }

    #[test]
    fn test_game_win_chances() {
        // All games should have reasonable win chances
        for game in [
            GamblingGame::CoinFlip,
            GamblingGame::HighLowDice,
            GamblingGame::Blackjack,
            GamblingGame::WheelOfFortune,
        ] {
            let chance = game.base_win_chance();
            assert!(chance > 0 && chance <= 100);
        }
    }

    #[test]
    fn test_gambler_level_availability() {
        // Common gamblers available at level 1
        assert!(GamblerKind::ShadyDealer.available_at_level(1));
        assert!(GamblerKind::DiceMaster.available_at_level(1));

        // Legendary gamblers not available at level 1
        assert!(!GamblerKind::LuckDemon.available_at_level(1));
        assert!(!GamblerKind::GoldenDragon.available_at_level(1));

        // But available at high levels
        assert!(GamblerKind::LuckDemon.available_at_level(25));
        assert!(GamblerKind::GoldenDragon.available_at_level(25));
    }

    #[test]
    fn test_gambling_den_creation() {
        let mut rng = rand::thread_rng();
        let den = GamblingDen::new(10, 10, 5, &mut rng);

        assert!(!den.gamblers.is_empty());
        assert!(!den.name.is_empty());
        assert!(!den.is_discovered);
    }

    #[test]
    fn test_outcome_description() {
        let outcomes = vec![
            GambleOutcome::WonGold(100),
            GambleOutcome::LostGold(50),
            GambleOutcome::Push,
            GambleOutcome::Cursed("Test curse".to_string()),
        ];

        for outcome in outcomes {
            assert!(!outcome.description().is_empty());
        }
    }

    #[test]
    fn test_effective_win_chance() {
        let gambler = Gambler::new(GamblerKind::DiceMaster, 0, 0);
        let mut stats = GamblingStats::new();

        let base = GamblingGame::CoinFlip.base_win_chance();
        let effective = gambler.effective_win_chance(GamblingGame::CoinFlip, &stats);

        // Should be similar to base without modifiers
        assert!((effective as i32 - base as i32).abs() <= 5);

        // With luck modifier
        stats.luck_modifier = 10;
        let effective_lucky = gambler.effective_win_chance(GamblingGame::CoinFlip, &stats);
        assert!(effective_lucky > effective);
    }
}
