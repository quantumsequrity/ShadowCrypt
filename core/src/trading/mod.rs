//! Trading and Auction System
//!
//! Comprehensive economic system including:
//! - Direct NPC trading and bartering
//! - Multiple currency types (Gold, Spirit Stones, Points)
//! - Trade routes with regional price differences
//! - Auction house with bidding and buyouts
//! - Various shop types (General, Weapon, Alchemy, Black Market, etc.)
//! - Trading NPCs (Traveling Merchants, Rare Item Dealers, Collectors)
//! - Dynamic economy with supply/demand and price history

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// CURRENCY SYSTEM
// ============================================================================

/// All currency types in the game
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CurrencyType {
    /// Common currency used everywhere
    Gold,
    /// Low-grade cultivation resource
    LowSpiritStone,
    /// Mid-grade cultivation resource (worth 100 low-grade)
    MidSpiritStone,
    /// High-grade cultivation resource (worth 100 mid-grade)
    HighSpiritStone,
    /// Supreme-grade cultivation resource (worth 100 high-grade)
    SupremeSpiritStone,
    /// Guild/Sect contribution currency
    ContributionPoints,
    /// Earned from arena battles
    ArenaPoints,
    /// Earned from good deeds and quests
    MeritPoints,
    /// Earned from moral choices (can be positive or negative)
    KarmaPoints,
    /// Dungeon exploration currency
    DungeonTokens,
    /// Rare trade currency from ancient civilizations
    AncientCoins,
    /// Premium currency for special items
    CelestialJade,
}

impl CurrencyType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Gold => "Gold",
            Self::LowSpiritStone => "Low-Grade Spirit Stone",
            Self::MidSpiritStone => "Mid-Grade Spirit Stone",
            Self::HighSpiritStone => "High-Grade Spirit Stone",
            Self::SupremeSpiritStone => "Supreme Spirit Stone",
            Self::ContributionPoints => "Contribution Points",
            Self::ArenaPoints => "Arena Points",
            Self::MeritPoints => "Merit Points",
            Self::KarmaPoints => "Karma Points",
            Self::DungeonTokens => "Dungeon Tokens",
            Self::AncientCoins => "Ancient Coins",
            Self::CelestialJade => "Celestial Jade",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            Self::Gold => "G",
            Self::LowSpiritStone => "LSS",
            Self::MidSpiritStone => "MSS",
            Self::HighSpiritStone => "HSS",
            Self::SupremeSpiritStone => "SSS",
            Self::ContributionPoints => "CP",
            Self::ArenaPoints => "AP",
            Self::MeritPoints => "MP",
            Self::KarmaPoints => "KP",
            Self::DungeonTokens => "DT",
            Self::AncientCoins => "AC",
            Self::CelestialJade => "CJ",
        }
    }

    pub fn glyph(&self) -> char {
        match self {
            Self::Gold => '$',
            Self::LowSpiritStone | Self::MidSpiritStone |
            Self::HighSpiritStone | Self::SupremeSpiritStone => '*',
            Self::ContributionPoints => 'C',
            Self::ArenaPoints => 'A',
            Self::MeritPoints => 'M',
            Self::KarmaPoints => 'K',
            Self::DungeonTokens => 'D',
            Self::AncientCoins => 'O',
            Self::CelestialJade => 'J',
        }
    }

    /// Returns the conversion rate to gold (for relative pricing)
    pub fn gold_value(&self) -> u64 {
        match self {
            Self::Gold => 1,
            Self::LowSpiritStone => 10,
            Self::MidSpiritStone => 1_000,
            Self::HighSpiritStone => 100_000,
            Self::SupremeSpiritStone => 10_000_000,
            Self::ContributionPoints => 5,
            Self::ArenaPoints => 8,
            Self::MeritPoints => 3,
            Self::KarmaPoints => 2,
            Self::DungeonTokens => 15,
            Self::AncientCoins => 500,
            Self::CelestialJade => 50_000,
        }
    }

    /// Whether this currency can be traded with others
    pub fn is_tradeable(&self) -> bool {
        !matches!(self, Self::KarmaPoints | Self::ContributionPoints)
    }

    /// All spirit stone grades for iteration
    pub fn spirit_stones() -> &'static [CurrencyType] {
        &[
            Self::LowSpiritStone,
            Self::MidSpiritStone,
            Self::HighSpiritStone,
            Self::SupremeSpiritStone,
        ]
    }

    /// All point-based currencies
    pub fn point_currencies() -> &'static [CurrencyType] {
        &[
            Self::ContributionPoints,
            Self::ArenaPoints,
            Self::MeritPoints,
            Self::KarmaPoints,
        ]
    }
}

/// Player's wallet containing all currencies
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Wallet {
    currencies: HashMap<CurrencyType, i64>,
    /// Lifetime earnings per currency type
    lifetime_earned: HashMap<CurrencyType, u64>,
    /// Lifetime spent per currency type
    lifetime_spent: HashMap<CurrencyType, u64>,
}

impl Wallet {
    pub fn new() -> Self {
        let mut wallet = Self::default();
        // Start with some gold
        wallet.currencies.insert(CurrencyType::Gold, 100);
        wallet
    }

    pub fn new_with_gold(gold: i64) -> Self {
        let mut wallet = Self::default();
        wallet.currencies.insert(CurrencyType::Gold, gold);
        wallet
    }

    pub fn balance(&self, currency: CurrencyType) -> i64 {
        *self.currencies.get(&currency).unwrap_or(&0)
    }

    pub fn add(&mut self, currency: CurrencyType, amount: u64) {
        let entry = self.currencies.entry(currency).or_insert(0);
        *entry += amount as i64;
        *self.lifetime_earned.entry(currency).or_insert(0) += amount;
    }

    pub fn subtract(&mut self, currency: CurrencyType, amount: u64) -> bool {
        let current = self.balance(currency);
        if current >= amount as i64 {
            *self.currencies.entry(currency).or_insert(0) -= amount as i64;
            *self.lifetime_spent.entry(currency).or_insert(0) += amount;
            true
        } else {
            false
        }
    }

    pub fn can_afford(&self, currency: CurrencyType, amount: u64) -> bool {
        self.balance(currency) >= amount as i64
    }

    pub fn can_afford_price(&self, price: &Price) -> bool {
        price.costs.iter().all(|(curr, amt)| self.can_afford(*curr, *amt))
    }

    pub fn pay_price(&mut self, price: &Price) -> bool {
        if !self.can_afford_price(price) {
            return false;
        }
        for (curr, amt) in &price.costs {
            self.subtract(*curr, *amt);
        }
        true
    }

    /// Convert between spirit stone grades
    pub fn convert_spirit_stones(&mut self, from: CurrencyType, to: CurrencyType, amount: u64) -> bool {
        let stones = CurrencyType::spirit_stones();
        let from_idx = stones.iter().position(|s| *s == from);
        let to_idx = stones.iter().position(|s| *s == to);

        match (from_idx, to_idx) {
            (Some(f), Some(t)) if f < t => {
                // Converting to higher grade (100:1 ratio per grade difference)
                let ratio = 100u64.pow((t - f) as u32);
                if amount >= ratio && self.can_afford(from, amount) {
                    let result = amount / ratio;
                    self.subtract(from, result * ratio);
                    self.add(to, result);
                    true
                } else {
                    false
                }
            }
            (Some(f), Some(t)) if f > t => {
                // Converting to lower grade (1:100 ratio per grade difference)
                let ratio = 100u64.pow((f - t) as u32);
                if self.can_afford(from, amount) {
                    self.subtract(from, amount);
                    self.add(to, amount * ratio);
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// Get total wealth in gold equivalent
    pub fn total_wealth(&self) -> u64 {
        self.currencies.iter()
            .map(|(curr, amt)| (*amt as u64) * curr.gold_value())
            .sum()
    }

    pub fn lifetime_earned(&self, currency: CurrencyType) -> u64 {
        *self.lifetime_earned.get(&currency).unwrap_or(&0)
    }

    pub fn lifetime_spent(&self, currency: CurrencyType) -> u64 {
        *self.lifetime_spent.get(&currency).unwrap_or(&0)
    }
}

/// A price that may require multiple currencies
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Price {
    pub costs: Vec<(CurrencyType, u64)>,
}

impl Price {
    pub fn gold(amount: u64) -> Self {
        Self { costs: vec![(CurrencyType::Gold, amount)] }
    }

    pub fn new(currency: CurrencyType, amount: u64) -> Self {
        Self { costs: vec![(currency, amount)] }
    }

    pub fn multi(costs: Vec<(CurrencyType, u64)>) -> Self {
        Self { costs }
    }

    pub fn add_cost(&mut self, currency: CurrencyType, amount: u64) {
        if let Some(entry) = self.costs.iter_mut().find(|(c, _)| *c == currency) {
            entry.1 += amount;
        } else {
            self.costs.push((currency, amount));
        }
    }

    pub fn total_gold_value(&self) -> u64 {
        self.costs.iter()
            .map(|(curr, amt)| amt * curr.gold_value())
            .sum()
    }

    pub fn display(&self) -> String {
        self.costs.iter()
            .map(|(curr, amt)| format!("{} {}", amt, curr.short_name()))
            .collect::<Vec<_>>()
            .join(" + ")
    }
}

// ============================================================================
// CURRENCY EXCHANGE
// ============================================================================

/// Currency exchange rates and services
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CurrencyExchange {
    /// Current exchange rates (from_currency -> to_currency -> rate)
    rates: HashMap<CurrencyType, HashMap<CurrencyType, f64>>,
    /// Exchange fee percentage (0-100)
    pub fee_percent: u32,
    /// Minimum exchange amount
    pub min_exchange: u64,
    /// Daily exchange limit per currency
    pub daily_limits: HashMap<CurrencyType, u64>,
    /// Amount exchanged today
    exchanged_today: HashMap<CurrencyType, u64>,
}

impl Default for CurrencyExchange {
    fn default() -> Self {
        Self::new()
    }
}

impl CurrencyExchange {
    pub fn new() -> Self {
        let mut exchange = Self {
            rates: HashMap::new(),
            fee_percent: 5,
            min_exchange: 10,
            daily_limits: HashMap::new(),
            exchanged_today: HashMap::new(),
        };
        exchange.initialize_rates();
        exchange
    }

    fn initialize_rates(&mut self) {
        // Set up base exchange rates based on gold values
        let currencies = [
            CurrencyType::Gold,
            CurrencyType::LowSpiritStone,
            CurrencyType::MidSpiritStone,
            CurrencyType::ArenaPoints,
            CurrencyType::MeritPoints,
            CurrencyType::DungeonTokens,
            CurrencyType::AncientCoins,
        ];

        for from in &currencies {
            let mut to_rates = HashMap::new();
            for to in &currencies {
                if from != to && from.is_tradeable() && to.is_tradeable() {
                    let rate = from.gold_value() as f64 / to.gold_value() as f64;
                    to_rates.insert(*to, rate);
                }
            }
            self.rates.insert(*from, to_rates);
        }

        // Set daily limits
        self.daily_limits.insert(CurrencyType::Gold, 100_000);
        self.daily_limits.insert(CurrencyType::LowSpiritStone, 10_000);
        self.daily_limits.insert(CurrencyType::MidSpiritStone, 100);
    }

    pub fn get_rate(&self, from: CurrencyType, to: CurrencyType) -> Option<f64> {
        self.rates.get(&from)?.get(&to).copied()
    }

    pub fn calculate_exchange(&self, from: CurrencyType, to: CurrencyType, amount: u64) -> Option<u64> {
        let rate = self.get_rate(from, to)?;
        let gross = (amount as f64 * rate) as u64;
        let fee = gross * self.fee_percent as u64 / 100;
        Some(gross.saturating_sub(fee))
    }

    pub fn can_exchange(&self, from: CurrencyType, amount: u64) -> bool {
        if amount < self.min_exchange {
            return false;
        }
        if let Some(limit) = self.daily_limits.get(&from) {
            let exchanged = self.exchanged_today.get(&from).unwrap_or(&0);
            if exchanged + amount > *limit {
                return false;
            }
        }
        true
    }

    pub fn exchange(&mut self, wallet: &mut Wallet, from: CurrencyType, to: CurrencyType, amount: u64) -> Result<u64, ExchangeError> {
        if !from.is_tradeable() || !to.is_tradeable() {
            return Err(ExchangeError::NotTradeable);
        }
        if amount < self.min_exchange {
            return Err(ExchangeError::BelowMinimum);
        }
        if !wallet.can_afford(from, amount) {
            return Err(ExchangeError::InsufficientFunds);
        }
        if !self.can_exchange(from, amount) {
            return Err(ExchangeError::DailyLimitReached);
        }

        let result = self.calculate_exchange(from, to, amount)
            .ok_or(ExchangeError::NoExchangeRate)?;

        wallet.subtract(from, amount);
        wallet.add(to, result);
        *self.exchanged_today.entry(from).or_insert(0) += amount;

        Ok(result)
    }

    pub fn reset_daily_limits(&mut self) {
        self.exchanged_today.clear();
    }

    /// Fluctuate rates slightly (called periodically)
    pub fn fluctuate_rates(&mut self, variance: f64) {
        for rates in self.rates.values_mut() {
            for rate in rates.values_mut() {
                let change = 1.0 + (rand_float() * 2.0 - 1.0) * variance;
                *rate *= change;
                *rate = rate.clamp(0.1, 10.0);
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExchangeError {
    NotTradeable,
    BelowMinimum,
    InsufficientFunds,
    DailyLimitReached,
    NoExchangeRate,
}

// Simple random float for rate fluctuation
fn rand_float() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    (nanos as f64 % 1000.0) / 1000.0
}

// ============================================================================
// MERCHANT REPUTATION
// ============================================================================

/// Reputation levels with merchants
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ReputationLevel {
    Hated = 0,
    Hostile = 1,
    Unfriendly = 2,
    Neutral = 3,
    Friendly = 4,
    Honored = 5,
    Revered = 6,
    Exalted = 7,
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

    pub fn from_points(points: i32) -> Self {
        match points {
            p if p < -3000 => Self::Hated,
            p if p < -1000 => Self::Hostile,
            p if p < 0 => Self::Unfriendly,
            p if p < 1000 => Self::Neutral,
            p if p < 3000 => Self::Friendly,
            p if p < 6000 => Self::Honored,
            p if p < 10000 => Self::Revered,
            _ => Self::Exalted,
        }
    }

    /// Price modifier based on reputation (negative = discount)
    pub fn price_modifier(&self) -> i32 {
        match self {
            Self::Hated => 50,
            Self::Hostile => 30,
            Self::Unfriendly => 15,
            Self::Neutral => 0,
            Self::Friendly => -5,
            Self::Honored => -10,
            Self::Revered => -15,
            Self::Exalted => -25,
        }
    }

    /// Whether merchant will trade at all
    pub fn will_trade(&self) -> bool {
        *self >= Self::Unfriendly
    }

    /// Whether merchant offers special items
    pub fn offers_special_items(&self) -> bool {
        *self >= Self::Honored
    }
}

/// Tracks reputation with different merchant factions
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MerchantReputation {
    factions: HashMap<MerchantFaction, i32>,
    /// Individual NPC reputation modifiers
    individual: HashMap<u64, i32>,
}

impl MerchantReputation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_faction_rep(&self, faction: MerchantFaction) -> i32 {
        *self.factions.get(&faction).unwrap_or(&0)
    }

    pub fn get_faction_level(&self, faction: MerchantFaction) -> ReputationLevel {
        ReputationLevel::from_points(self.get_faction_rep(faction))
    }

    pub fn modify_faction_rep(&mut self, faction: MerchantFaction, amount: i32) {
        let entry = self.factions.entry(faction).or_insert(0);
        *entry = (*entry + amount).clamp(-10000, 15000);
    }

    pub fn get_individual_rep(&self, npc_id: u64) -> i32 {
        *self.individual.get(&npc_id).unwrap_or(&0)
    }

    pub fn modify_individual_rep(&mut self, npc_id: u64, amount: i32) {
        let entry = self.individual.entry(npc_id).or_insert(0);
        *entry = (*entry + amount).clamp(-5000, 5000);
    }

    /// Get effective reputation with a specific merchant
    pub fn effective_rep(&self, faction: MerchantFaction, npc_id: Option<u64>) -> i32 {
        let faction_rep = self.get_faction_rep(faction);
        let individual_rep = npc_id.map(|id| self.get_individual_rep(id)).unwrap_or(0);
        faction_rep + individual_rep
    }

    pub fn effective_level(&self, faction: MerchantFaction, npc_id: Option<u64>) -> ReputationLevel {
        ReputationLevel::from_points(self.effective_rep(faction, npc_id))
    }
}

/// Merchant factions
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MerchantFaction {
    GeneralMerchants,
    WeaponSmiths,
    Armorers,
    Alchemists,
    FormationMasters,
    TreasurePavilion,
    BlackMarket,
    TravelingMerchants,
    RareCollectors,
    AncientTraders,
}

impl MerchantFaction {
    pub fn name(&self) -> &'static str {
        match self {
            Self::GeneralMerchants => "General Merchants Guild",
            Self::WeaponSmiths => "Weapon Smiths Guild",
            Self::Armorers => "Armorers Guild",
            Self::Alchemists => "Alchemist Association",
            Self::FormationMasters => "Formation Masters Circle",
            Self::TreasurePavilion => "Treasure Pavilion",
            Self::BlackMarket => "Shadow Traders",
            Self::TravelingMerchants => "Wandering Traders",
            Self::RareCollectors => "Collectors Society",
            Self::AncientTraders => "Ancient Traders",
        }
    }
}

// ============================================================================
// TRADE ROUTES
// ============================================================================

/// Regions for trade route pricing
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TradeRegion {
    CentralKingdom,
    NorthernWastes,
    EasternForests,
    SouthernDeserts,
    WesternMountains,
    CoastalPorts,
    UndergroundCities,
    FloatingIsles,
    DemonLands,
    CelestialRealm,
}

impl TradeRegion {
    pub fn name(&self) -> &'static str {
        match self {
            Self::CentralKingdom => "Central Kingdom",
            Self::NorthernWastes => "Northern Wastes",
            Self::EasternForests => "Eastern Forests",
            Self::SouthernDeserts => "Southern Deserts",
            Self::WesternMountains => "Western Mountains",
            Self::CoastalPorts => "Coastal Ports",
            Self::UndergroundCities => "Underground Cities",
            Self::FloatingIsles => "Floating Isles",
            Self::DemonLands => "Demon Lands",
            Self::CelestialRealm => "Celestial Realm",
        }
    }

    /// Base price modifier for this region (percentage)
    pub fn base_modifier(&self) -> i32 {
        match self {
            Self::CentralKingdom => 0,
            Self::NorthernWastes => 15,
            Self::EasternForests => -5,
            Self::SouthernDeserts => 10,
            Self::WesternMountains => 20,
            Self::CoastalPorts => -10,
            Self::UndergroundCities => 25,
            Self::FloatingIsles => 40,
            Self::DemonLands => 50,
            Self::CelestialRealm => 100,
        }
    }

    /// Items that are cheaper in this region
    pub fn abundant_goods(&self) -> Vec<TradeGoodCategory> {
        match self {
            Self::CentralKingdom => vec![TradeGoodCategory::General, TradeGoodCategory::Food],
            Self::NorthernWastes => vec![TradeGoodCategory::Furs, TradeGoodCategory::IceMaterials],
            Self::EasternForests => vec![TradeGoodCategory::Herbs, TradeGoodCategory::Wood],
            Self::SouthernDeserts => vec![TradeGoodCategory::Gems, TradeGoodCategory::Spices],
            Self::WesternMountains => vec![TradeGoodCategory::Ores, TradeGoodCategory::Weapons],
            Self::CoastalPorts => vec![TradeGoodCategory::Fish, TradeGoodCategory::Imports],
            Self::UndergroundCities => vec![TradeGoodCategory::DarkMaterials, TradeGoodCategory::Ores],
            Self::FloatingIsles => vec![TradeGoodCategory::SkyMaterials, TradeGoodCategory::Enchantments],
            Self::DemonLands => vec![TradeGoodCategory::DemonParts, TradeGoodCategory::CursedItems],
            Self::CelestialRealm => vec![TradeGoodCategory::HolyItems, TradeGoodCategory::CelestialMaterials],
        }
    }

    /// Items that are more expensive in this region
    pub fn scarce_goods(&self) -> Vec<TradeGoodCategory> {
        match self {
            Self::CentralKingdom => vec![TradeGoodCategory::DarkMaterials, TradeGoodCategory::DemonParts],
            Self::NorthernWastes => vec![TradeGoodCategory::Spices, TradeGoodCategory::Food],
            Self::EasternForests => vec![TradeGoodCategory::Ores, TradeGoodCategory::DarkMaterials],
            Self::SouthernDeserts => vec![TradeGoodCategory::IceMaterials, TradeGoodCategory::Wood],
            Self::WesternMountains => vec![TradeGoodCategory::Fish, TradeGoodCategory::Herbs],
            Self::CoastalPorts => vec![TradeGoodCategory::Ores, TradeGoodCategory::Furs],
            Self::UndergroundCities => vec![TradeGoodCategory::Food, TradeGoodCategory::HolyItems],
            Self::FloatingIsles => vec![TradeGoodCategory::Ores, TradeGoodCategory::Fish],
            Self::DemonLands => vec![TradeGoodCategory::HolyItems, TradeGoodCategory::CelestialMaterials],
            Self::CelestialRealm => vec![TradeGoodCategory::DemonParts, TradeGoodCategory::CursedItems],
        }
    }
}

/// Categories of trade goods for regional pricing
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TradeGoodCategory {
    General,
    Weapons,
    Armor,
    Food,
    Herbs,
    Ores,
    Gems,
    Wood,
    Furs,
    Fish,
    Spices,
    Imports,
    Enchantments,
    IceMaterials,
    DarkMaterials,
    SkyMaterials,
    DemonParts,
    HolyItems,
    CursedItems,
    CelestialMaterials,
    Artifacts,
}

impl TradeGoodCategory {
    pub fn name(&self) -> &'static str {
        match self {
            Self::General => "General Goods",
            Self::Weapons => "Weapons",
            Self::Armor => "Armor",
            Self::Food => "Food",
            Self::Herbs => "Herbs",
            Self::Ores => "Ores",
            Self::Gems => "Gems",
            Self::Wood => "Wood",
            Self::Furs => "Furs",
            Self::Fish => "Fish",
            Self::Spices => "Spices",
            Self::Imports => "Imports",
            Self::Enchantments => "Enchantments",
            Self::IceMaterials => "Ice Materials",
            Self::DarkMaterials => "Dark Materials",
            Self::SkyMaterials => "Sky Materials",
            Self::DemonParts => "Demon Parts",
            Self::HolyItems => "Holy Items",
            Self::CursedItems => "Cursed Items",
            Self::CelestialMaterials => "Celestial Materials",
            Self::Artifacts => "Artifacts",
        }
    }
}

/// Trade route system for regional price differences
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeRouteSystem {
    current_region: TradeRegion,
    /// Price modifiers per category per region
    regional_prices: HashMap<TradeRegion, HashMap<TradeGoodCategory, i32>>,
    /// Known trade routes
    discovered_routes: Vec<TradeRoute>,
    /// Active trade caravans
    active_caravans: Vec<Caravan>,
}

impl Default for TradeRouteSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl TradeRouteSystem {
    pub fn new() -> Self {
        let mut system = Self {
            current_region: TradeRegion::CentralKingdom,
            regional_prices: HashMap::new(),
            discovered_routes: Vec::new(),
            active_caravans: Vec::new(),
        };
        system.initialize_prices();
        system
    }

    fn initialize_prices(&mut self) {
        // Set up regional prices based on abundance and scarcity
        for region in [
            TradeRegion::CentralKingdom, TradeRegion::NorthernWastes,
            TradeRegion::EasternForests, TradeRegion::SouthernDeserts,
            TradeRegion::WesternMountains, TradeRegion::CoastalPorts,
            TradeRegion::UndergroundCities, TradeRegion::FloatingIsles,
            TradeRegion::DemonLands, TradeRegion::CelestialRealm,
        ] {
            let mut prices = HashMap::new();

            // Base modifier applies to all
            let base = region.base_modifier();

            // Abundant goods are cheaper
            for category in region.abundant_goods() {
                prices.insert(category, base - 30);
            }

            // Scarce goods are more expensive
            for category in region.scarce_goods() {
                prices.insert(category, base + 40);
            }

            self.regional_prices.insert(region, prices);
        }
    }

    pub fn set_current_region(&mut self, region: TradeRegion) {
        self.current_region = region;
    }

    pub fn current_region(&self) -> TradeRegion {
        self.current_region
    }

    /// Get price modifier for a category in the current region
    pub fn get_price_modifier(&self, category: TradeGoodCategory) -> i32 {
        self.get_price_modifier_for_region(self.current_region, category)
    }

    pub fn get_price_modifier_for_region(&self, region: TradeRegion, category: TradeGoodCategory) -> i32 {
        self.regional_prices
            .get(&region)
            .and_then(|prices| prices.get(&category))
            .copied()
            .unwrap_or(region.base_modifier())
    }

    /// Calculate arbitrage profit between two regions
    pub fn calculate_arbitrage(&self, category: TradeGoodCategory, from: TradeRegion, to: TradeRegion) -> i32 {
        let buy_modifier = self.get_price_modifier_for_region(from, category);
        let sell_modifier = self.get_price_modifier_for_region(to, category);
        sell_modifier - buy_modifier
    }

    pub fn discover_route(&mut self, route: TradeRoute) {
        if !self.discovered_routes.iter().any(|r| r.from == route.from && r.to == route.to) {
            self.discovered_routes.push(route);
        }
    }

    pub fn start_caravan(&mut self, caravan: Caravan) {
        self.active_caravans.push(caravan);
    }

    pub fn update_caravans(&mut self) -> Vec<CaravanResult> {
        let mut results = Vec::new();
        let mut completed = Vec::new();

        for (idx, caravan) in self.active_caravans.iter_mut().enumerate() {
            caravan.progress += 1;
            if caravan.progress >= caravan.travel_time {
                results.push(CaravanResult {
                    caravan_id: caravan.id,
                    goods_delivered: caravan.goods.clone(),
                    destination: caravan.destination,
                    profit_modifier: self.get_price_modifier_for_region(
                        caravan.destination,
                        caravan.goods_category,
                    ),
                });
                completed.push(idx);
            }
        }

        // Remove completed caravans in reverse order
        for idx in completed.into_iter().rev() {
            self.active_caravans.remove(idx);
        }

        results
    }
}

/// A trade route between regions
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeRoute {
    pub from: TradeRegion,
    pub to: TradeRegion,
    pub travel_time: u32,
    pub danger_level: u32,
    pub toll_cost: u64,
}

/// A trade caravan
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Caravan {
    pub id: u64,
    pub origin: TradeRegion,
    pub destination: TradeRegion,
    pub goods: Vec<TradeGood>,
    pub goods_category: TradeGoodCategory,
    pub travel_time: u32,
    pub progress: u32,
}

/// Trade goods being transported
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeGood {
    pub name: String,
    pub category: TradeGoodCategory,
    pub quantity: u32,
    pub base_value: u64,
}

/// Result of a completed caravan journey
#[derive(Clone, Debug)]
pub struct CaravanResult {
    pub caravan_id: u64,
    pub goods_delivered: Vec<TradeGood>,
    pub destination: TradeRegion,
    pub profit_modifier: i32,
}

// ============================================================================
// SHOP SYSTEM
// ============================================================================

/// Types of shops
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShopType {
    GeneralStore,
    WeaponShop,
    ArmorShop,
    AlchemyShop,
    FormationShop,
    TreasurePavilion,
    BlackMarket,
    FoodMarket,
    MaterialShop,
    ScrollShop,
    JewelryShop,
    PetShop,
    MountShop,
    RepairShop,
}

impl ShopType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::GeneralStore => "General Store",
            Self::WeaponShop => "Weapon Shop",
            Self::ArmorShop => "Armor Shop",
            Self::AlchemyShop => "Alchemy Shop",
            Self::FormationShop => "Formation Shop",
            Self::TreasurePavilion => "Treasure Pavilion",
            Self::BlackMarket => "Black Market",
            Self::FoodMarket => "Food Market",
            Self::MaterialShop => "Material Shop",
            Self::ScrollShop => "Scroll Shop",
            Self::JewelryShop => "Jewelry Shop",
            Self::PetShop => "Pet Shop",
            Self::MountShop => "Mount Shop",
            Self::RepairShop => "Repair Shop",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::GeneralStore => "Basic supplies and common items",
            Self::WeaponShop => "Weapons for all classes and fighting styles",
            Self::ArmorShop => "Protective gear from leather to plate",
            Self::AlchemyShop => "Potions, elixirs, and alchemical ingredients",
            Self::FormationShop => "Formation flags, arrays, and components",
            Self::TreasurePavilion => "Rare treasures and unique artifacts",
            Self::BlackMarket => "Forbidden items and illegal goods",
            Self::FoodMarket => "Food, ingredients, and cooking supplies",
            Self::MaterialShop => "Crafting materials and resources",
            Self::ScrollShop => "Skill scrolls and technique manuals",
            Self::JewelryShop => "Rings, amulets, and magical accessories",
            Self::PetShop => "Beast companions and pet supplies",
            Self::MountShop => "Mounts and riding equipment",
            Self::RepairShop => "Item repair and maintenance services",
        }
    }

    pub fn faction(&self) -> MerchantFaction {
        match self {
            Self::GeneralStore | Self::FoodMarket => MerchantFaction::GeneralMerchants,
            Self::WeaponShop => MerchantFaction::WeaponSmiths,
            Self::ArmorShop => MerchantFaction::Armorers,
            Self::AlchemyShop => MerchantFaction::Alchemists,
            Self::FormationShop => MerchantFaction::FormationMasters,
            Self::TreasurePavilion => MerchantFaction::TreasurePavilion,
            Self::BlackMarket => MerchantFaction::BlackMarket,
            _ => MerchantFaction::GeneralMerchants,
        }
    }

    pub fn primary_currency(&self) -> CurrencyType {
        match self {
            Self::TreasurePavilion => CurrencyType::HighSpiritStone,
            Self::BlackMarket => CurrencyType::AncientCoins,
            Self::FormationShop => CurrencyType::ContributionPoints,
            _ => CurrencyType::Gold,
        }
    }

    pub fn accepts_barter(&self) -> bool {
        matches!(self, Self::GeneralStore | Self::BlackMarket | Self::MaterialShop)
    }

    /// Minimum reputation level to access this shop
    pub fn required_reputation(&self) -> ReputationLevel {
        match self {
            Self::BlackMarket => ReputationLevel::Unfriendly,
            Self::TreasurePavilion => ReputationLevel::Friendly,
            _ => ReputationLevel::Neutral,
        }
    }
}

/// An item listed in a shop
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShopItem {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub category: TradeGoodCategory,
    pub base_price: Price,
    pub quantity: Option<u32>, // None = unlimited
    pub required_reputation: ReputationLevel,
    pub is_special: bool,
    /// Item data reference (could be item ID, kind, etc.)
    pub item_data: String,
}

impl ShopItem {
    pub fn new(id: u64, name: &str, description: &str, price: Price) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            category: TradeGoodCategory::General,
            base_price: price,
            quantity: None,
            required_reputation: ReputationLevel::Neutral,
            is_special: false,
            item_data: String::new(),
        }
    }

    pub fn with_category(mut self, category: TradeGoodCategory) -> Self {
        self.category = category;
        self
    }

    pub fn with_quantity(mut self, quantity: u32) -> Self {
        self.quantity = Some(quantity);
        self
    }

    pub fn with_reputation(mut self, rep: ReputationLevel) -> Self {
        self.required_reputation = rep;
        self
    }

    pub fn as_special(mut self) -> Self {
        self.is_special = true;
        self
    }
}

/// A shop instance
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shop {
    pub id: u64,
    pub name: String,
    pub shop_type: ShopType,
    pub region: TradeRegion,
    pub inventory: Vec<ShopItem>,
    pub owner_id: Option<u64>,
    pub restock_timer: u32,
    pub restock_interval: u32,
    /// Bonus/penalty to prices (shop-specific)
    pub price_modifier: i32,
    pub accepts_haggling: bool,
    pub max_haggle_discount: u32,
}

impl Shop {
    pub fn new(id: u64, name: &str, shop_type: ShopType, region: TradeRegion) -> Self {
        Self {
            id,
            name: name.to_string(),
            shop_type,
            region,
            inventory: Vec::new(),
            owner_id: None,
            restock_timer: 0,
            restock_interval: 100, // turns
            price_modifier: 0,
            accepts_haggling: true,
            max_haggle_discount: 15,
        }
    }

    pub fn add_item(&mut self, item: ShopItem) {
        self.inventory.push(item);
    }

    pub fn remove_item(&mut self, item_id: u64) -> Option<ShopItem> {
        if let Some(pos) = self.inventory.iter().position(|i| i.id == item_id) {
            Some(self.inventory.remove(pos))
        } else {
            None
        }
    }

    pub fn get_item(&self, item_id: u64) -> Option<&ShopItem> {
        self.inventory.iter().find(|i| i.id == item_id)
    }

    pub fn get_item_mut(&mut self, item_id: u64) -> Option<&mut ShopItem> {
        self.inventory.iter_mut().find(|i| i.id == item_id)
    }

    /// Calculate final price for an item considering all modifiers
    pub fn calculate_price(
        &self,
        item: &ShopItem,
        trade_routes: &TradeRouteSystem,
        reputation: &MerchantReputation,
    ) -> Price {
        let base = item.base_price.total_gold_value();

        // Apply regional modifier
        let regional_mod = trade_routes.get_price_modifier(item.category);

        // Apply shop modifier
        let shop_mod = self.price_modifier;

        // Apply reputation modifier
        let rep_level = reputation.effective_level(self.shop_type.faction(), self.owner_id);
        let rep_mod = rep_level.price_modifier();

        // Calculate total modifier
        let total_mod = regional_mod + shop_mod + rep_mod;
        let modifier = 1.0 + (total_mod as f64 / 100.0);

        let final_price = (base as f64 * modifier) as u64;

        Price::new(self.shop_type.primary_currency(), final_price.max(1))
    }

    /// Try to haggle for a better price
    pub fn haggle(&self, item: &ShopItem, skill_level: u32, current_price: &Price) -> Option<Price> {
        if !self.accepts_haggling {
            return None;
        }

        // Success chance based on skill
        let success_chance = (skill_level * 5).min(80);
        let roll = (rand_float() * 100.0) as u32;

        if roll < success_chance {
            let discount = ((skill_level as f64 / 10.0) * rand_float() * self.max_haggle_discount as f64) as u64;
            let discount = discount.min(self.max_haggle_discount as u64);

            let new_total = current_price.total_gold_value() * (100 - discount) / 100;
            Some(Price::new(self.shop_type.primary_currency(), new_total.max(1)))
        } else {
            None
        }
    }

    pub fn restock(&mut self) {
        self.restock_timer = 0;
        // Restore quantities for limited items
        for item in &mut self.inventory {
            if item.quantity == Some(0) {
                item.quantity = Some(1); // Restock 1 of each
            }
        }
    }

    pub fn tick(&mut self) -> bool {
        self.restock_timer += 1;
        if self.restock_timer >= self.restock_interval {
            self.restock();
            true
        } else {
            false
        }
    }
}

// ============================================================================
// BARTER SYSTEM
// ============================================================================

/// A barter offer
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BarterOffer {
    pub id: u64,
    /// Items being offered by the player
    pub offered_items: Vec<BarterItem>,
    /// Items being requested
    pub requested_items: Vec<BarterItem>,
    /// Additional currency in either direction
    pub currency_balance: i64, // Positive = player pays, negative = player receives
    pub currency_type: CurrencyType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BarterItem {
    pub item_data: String,
    pub quantity: u32,
    pub estimated_value: u64,
}

impl BarterOffer {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            offered_items: Vec::new(),
            requested_items: Vec::new(),
            currency_balance: 0,
            currency_type: CurrencyType::Gold,
        }
    }

    pub fn add_offered_item(&mut self, item: BarterItem) {
        self.offered_items.push(item);
    }

    pub fn add_requested_item(&mut self, item: BarterItem) {
        self.requested_items.push(item);
    }

    pub fn offered_value(&self) -> u64 {
        self.offered_items.iter().map(|i| i.estimated_value * i.quantity as u64).sum()
    }

    pub fn requested_value(&self) -> u64 {
        self.requested_items.iter().map(|i| i.estimated_value * i.quantity as u64).sum()
    }

    /// Check if the barter is fair (within 10% margin)
    pub fn is_fair(&self) -> bool {
        let offered = self.offered_value() as i64 + if self.currency_balance > 0 { self.currency_balance } else { 0 };
        let requested = self.requested_value() as i64 + if self.currency_balance < 0 { -self.currency_balance } else { 0 };

        let difference = (offered - requested).abs();
        let average = (offered + requested) / 2;

        if average == 0 {
            return difference == 0;
        }

        (difference as f64 / average as f64) <= 0.1
    }

    /// Calculate how much currency would make this trade fair
    pub fn balance_needed(&self) -> i64 {
        let offered = self.offered_value() as i64;
        let requested = self.requested_value() as i64;
        requested - offered - self.currency_balance
    }
}

// ============================================================================
// AUCTION HOUSE
// ============================================================================

/// Auction types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuctionType {
    /// Standard auction visible to everyone
    Standard,
    /// Anonymous auction (bidders hidden)
    Hidden,
    /// VIP only auction
    VIP,
    /// Quick auction (shorter duration, no buyout)
    Quick,
    /// Blind auction (bids hidden until end)
    Blind,
}

impl AuctionType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Standard => "Standard Auction",
            Self::Hidden => "Hidden Auction",
            Self::VIP => "VIP Auction",
            Self::Quick => "Quick Auction",
            Self::Blind => "Blind Auction",
        }
    }

    pub fn listing_fee_multiplier(&self) -> f64 {
        match self {
            Self::Standard => 1.0,
            Self::Hidden => 1.5,
            Self::VIP => 2.0,
            Self::Quick => 0.5,
            Self::Blind => 1.25,
        }
    }

    pub fn default_duration(&self) -> u32 {
        match self {
            Self::Standard => 1440, // 24 hours (in turns, assuming 1 turn = 1 minute)
            Self::Hidden => 1440,
            Self::VIP => 2880, // 48 hours
            Self::Quick => 360, // 6 hours
            Self::Blind => 720, // 12 hours
        }
    }
}

/// Status of an auction
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuctionStatus {
    Active,
    Sold,
    Expired,
    Cancelled,
}

/// A bid on an auction
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuctionBid {
    pub bidder_id: u64,
    pub bidder_name: String,
    pub amount: u64,
    pub currency: CurrencyType,
    pub timestamp: u64,
    pub is_auto_bid: bool,
    pub max_auto_bid: Option<u64>,
}

/// An auction listing
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuctionListing {
    pub id: u64,
    pub seller_id: u64,
    pub seller_name: String,
    pub item_name: String,
    pub item_description: String,
    pub item_data: String,
    pub category: TradeGoodCategory,
    pub auction_type: AuctionType,
    pub starting_price: u64,
    pub current_price: u64,
    pub buyout_price: Option<u64>,
    pub currency: CurrencyType,
    pub duration: u32,
    pub time_remaining: u32,
    pub bids: Vec<AuctionBid>,
    pub status: AuctionStatus,
    pub reserve_price: Option<u64>,
    pub created_at: u64,
}

impl AuctionListing {
    pub fn new(
        id: u64,
        seller_id: u64,
        seller_name: &str,
        item_name: &str,
        item_description: &str,
        starting_price: u64,
        currency: CurrencyType,
        auction_type: AuctionType,
    ) -> Self {
        let duration = auction_type.default_duration();
        Self {
            id,
            seller_id,
            seller_name: seller_name.to_string(),
            item_name: item_name.to_string(),
            item_description: item_description.to_string(),
            item_data: String::new(),
            category: TradeGoodCategory::General,
            auction_type,
            starting_price,
            current_price: starting_price,
            buyout_price: None,
            currency,
            duration,
            time_remaining: duration,
            bids: Vec::new(),
            status: AuctionStatus::Active,
            reserve_price: None,
            created_at: 0,
        }
    }

    pub fn with_buyout(mut self, price: u64) -> Self {
        self.buyout_price = Some(price);
        self
    }

    pub fn with_reserve(mut self, price: u64) -> Self {
        self.reserve_price = Some(price);
        self
    }

    pub fn with_duration(mut self, duration: u32) -> Self {
        self.duration = duration;
        self.time_remaining = duration;
        self
    }

    pub fn with_category(mut self, category: TradeGoodCategory) -> Self {
        self.category = category;
        self
    }

    pub fn highest_bidder(&self) -> Option<&AuctionBid> {
        self.bids.last()
    }

    pub fn bid_count(&self) -> usize {
        self.bids.len()
    }

    pub fn minimum_bid(&self) -> u64 {
        // Minimum increment is 5% or 1, whichever is higher
        let increment = (self.current_price / 20).max(1);
        self.current_price + increment
    }

    pub fn place_bid(&mut self, bid: AuctionBid) -> Result<(), AuctionError> {
        if self.status != AuctionStatus::Active {
            return Err(AuctionError::AuctionEnded);
        }
        if bid.bidder_id == self.seller_id {
            return Err(AuctionError::CannotBidOwnAuction);
        }
        if bid.amount < self.minimum_bid() {
            return Err(AuctionError::BidTooLow);
        }

        self.current_price = bid.amount;
        self.bids.push(bid);

        // Extend auction if bid in last 5 minutes
        if self.time_remaining < 5 {
            self.time_remaining = 5;
        }

        Ok(())
    }

    pub fn buyout(&mut self, buyer_id: u64, buyer_name: &str) -> Result<(), AuctionError> {
        if self.status != AuctionStatus::Active {
            return Err(AuctionError::AuctionEnded);
        }
        if buyer_id == self.seller_id {
            return Err(AuctionError::CannotBidOwnAuction);
        }

        let buyout = self.buyout_price.ok_or(AuctionError::NoBuyout)?;

        let bid = AuctionBid {
            bidder_id: buyer_id,
            bidder_name: buyer_name.to_string(),
            amount: buyout,
            currency: self.currency,
            timestamp: 0,
            is_auto_bid: false,
            max_auto_bid: None,
        };

        self.current_price = buyout;
        self.bids.push(bid);
        self.status = AuctionStatus::Sold;

        Ok(())
    }

    pub fn tick(&mut self) -> bool {
        if self.status != AuctionStatus::Active {
            return false;
        }

        self.time_remaining = self.time_remaining.saturating_sub(1);

        if self.time_remaining == 0 {
            if self.bids.is_empty() {
                self.status = AuctionStatus::Expired;
            } else if let Some(reserve) = self.reserve_price {
                if self.current_price >= reserve {
                    self.status = AuctionStatus::Sold;
                } else {
                    self.status = AuctionStatus::Expired;
                }
            } else {
                self.status = AuctionStatus::Sold;
            }
            true
        } else {
            false
        }
    }

    pub fn cancel(&mut self) -> Result<(), AuctionError> {
        if !self.bids.is_empty() {
            return Err(AuctionError::HasBids);
        }
        if self.status != AuctionStatus::Active {
            return Err(AuctionError::AuctionEnded);
        }

        self.status = AuctionStatus::Cancelled;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuctionError {
    AuctionEnded,
    BidTooLow,
    CannotBidOwnAuction,
    NoBuyout,
    HasBids,
    InsufficientFunds,
    NotFound,
    NotAuthorized,
}

/// The auction house system
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AuctionHouse {
    listings: Vec<AuctionListing>,
    next_id: u64,
    /// Fee percentage for successful sales
    pub sale_fee_percent: u32,
    /// Base listing fee
    pub listing_fee: u64,
    /// VIP member IDs
    vip_members: Vec<u64>,
    /// Transaction history
    history: Vec<AuctionTransaction>,
}

impl AuctionHouse {
    pub fn new() -> Self {
        Self {
            listings: Vec::new(),
            next_id: 1,
            sale_fee_percent: 5,
            listing_fee: 10,
            vip_members: Vec::new(),
            history: Vec::new(),
        }
    }

    pub fn list_item(&mut self, mut listing: AuctionListing) -> Result<u64, AuctionError> {
        // Check VIP requirements
        if listing.auction_type == AuctionType::VIP && !self.vip_members.contains(&listing.seller_id) {
            return Err(AuctionError::NotAuthorized);
        }

        listing.id = self.next_id;
        self.next_id += 1;

        let id = listing.id;
        self.listings.push(listing);
        Ok(id)
    }

    pub fn calculate_listing_fee(&self, auction_type: AuctionType, starting_price: u64) -> u64 {
        let base = self.listing_fee;
        let price_fee = starting_price / 100; // 1% of starting price
        let multiplier = auction_type.listing_fee_multiplier();

        ((base + price_fee) as f64 * multiplier) as u64
    }

    pub fn get_listing(&self, id: u64) -> Option<&AuctionListing> {
        self.listings.iter().find(|l| l.id == id)
    }

    pub fn get_listing_mut(&mut self, id: u64) -> Option<&mut AuctionListing> {
        self.listings.iter_mut().find(|l| l.id == id)
    }

    pub fn place_bid(&mut self, listing_id: u64, bid: AuctionBid) -> Result<(), AuctionError> {
        let listing = self.get_listing_mut(listing_id).ok_or(AuctionError::NotFound)?;

        // Check VIP access
        if listing.auction_type == AuctionType::VIP && !self.vip_members.contains(&bid.bidder_id) {
            return Err(AuctionError::NotAuthorized);
        }

        listing.place_bid(bid)
    }

    pub fn buyout(&mut self, listing_id: u64, buyer_id: u64, buyer_name: &str) -> Result<u64, AuctionError> {
        let listing = self.get_listing_mut(listing_id).ok_or(AuctionError::NotFound)?;

        // Check VIP access
        if listing.auction_type == AuctionType::VIP && !self.vip_members.contains(&buyer_id) {
            return Err(AuctionError::NotAuthorized);
        }

        listing.buyout(buyer_id, buyer_name)?;
        Ok(listing.current_price)
    }

    pub fn cancel_listing(&mut self, listing_id: u64, user_id: u64) -> Result<(), AuctionError> {
        let listing = self.get_listing_mut(listing_id).ok_or(AuctionError::NotFound)?;

        if listing.seller_id != user_id {
            return Err(AuctionError::NotAuthorized);
        }

        listing.cancel()
    }

    pub fn search(&self, query: &AuctionQuery) -> Vec<&AuctionListing> {
        self.listings.iter().filter(|l| {
            if l.status != AuctionStatus::Active {
                return false;
            }

            if let Some(ref name) = query.name_contains {
                if !l.item_name.to_lowercase().contains(&name.to_lowercase()) {
                    return false;
                }
            }

            if let Some(category) = query.category {
                if l.category != category {
                    return false;
                }
            }

            if let Some(min) = query.min_price {
                if l.current_price < min {
                    return false;
                }
            }

            if let Some(max) = query.max_price {
                if l.current_price > max {
                    return false;
                }
            }

            if let Some(auction_type) = query.auction_type {
                if l.auction_type != auction_type {
                    return false;
                }
            }

            if query.has_buyout && l.buyout_price.is_none() {
                return false;
            }

            if let Some(seller_id) = query.seller_id {
                if l.seller_id != seller_id {
                    return false;
                }
            }

            true
        }).collect()
    }

    pub fn get_user_listings(&self, user_id: u64) -> Vec<&AuctionListing> {
        self.listings.iter().filter(|l| l.seller_id == user_id).collect()
    }

    pub fn get_user_bids(&self, user_id: u64) -> Vec<&AuctionListing> {
        self.listings.iter().filter(|l| {
            l.status == AuctionStatus::Active &&
            l.bids.iter().any(|b| b.bidder_id == user_id)
        }).collect()
    }

    pub fn tick(&mut self) -> Vec<AuctionResult> {
        let mut results = Vec::new();

        for listing in &mut self.listings {
            if listing.tick() {
                let result = AuctionResult {
                    listing_id: listing.id,
                    item_name: listing.item_name.clone(),
                    seller_id: listing.seller_id,
                    winner: listing.highest_bidder().map(|b| (b.bidder_id, b.bidder_name.clone())),
                    final_price: listing.current_price,
                    currency: listing.currency,
                    status: listing.status,
                };
                results.push(result);

                // Record transaction
                if listing.status == AuctionStatus::Sold {
                    if let Some(winner) = listing.highest_bidder() {
                        self.history.push(AuctionTransaction {
                            listing_id: listing.id,
                            item_name: listing.item_name.clone(),
                            seller_id: listing.seller_id,
                            buyer_id: winner.bidder_id,
                            price: listing.current_price,
                            currency: listing.currency,
                            timestamp: 0,
                        });
                    }
                }
            }
        }

        results
    }

    pub fn cleanup_old_listings(&mut self) {
        self.listings.retain(|l| {
            l.status == AuctionStatus::Active ||
            l.time_remaining > 0
        });
    }

    pub fn add_vip_member(&mut self, user_id: u64) {
        if !self.vip_members.contains(&user_id) {
            self.vip_members.push(user_id);
        }
    }

    pub fn remove_vip_member(&mut self, user_id: u64) {
        self.vip_members.retain(|id| *id != user_id);
    }

    pub fn is_vip(&self, user_id: u64) -> bool {
        self.vip_members.contains(&user_id)
    }

    pub fn get_history(&self, limit: usize) -> &[AuctionTransaction] {
        let start = self.history.len().saturating_sub(limit);
        &self.history[start..]
    }

    /// Calculate fees for a successful sale
    pub fn calculate_sale_fee(&self, price: u64) -> u64 {
        price * self.sale_fee_percent as u64 / 100
    }
}

/// Query parameters for auction search
#[derive(Clone, Debug, Default)]
pub struct AuctionQuery {
    pub name_contains: Option<String>,
    pub category: Option<TradeGoodCategory>,
    pub min_price: Option<u64>,
    pub max_price: Option<u64>,
    pub auction_type: Option<AuctionType>,
    pub has_buyout: bool,
    pub seller_id: Option<u64>,
}

/// Result of an ended auction
#[derive(Clone, Debug)]
pub struct AuctionResult {
    pub listing_id: u64,
    pub item_name: String,
    pub seller_id: u64,
    pub winner: Option<(u64, String)>,
    pub final_price: u64,
    pub currency: CurrencyType,
    pub status: AuctionStatus,
}

/// Historical auction transaction
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuctionTransaction {
    pub listing_id: u64,
    pub item_name: String,
    pub seller_id: u64,
    pub buyer_id: u64,
    pub price: u64,
    pub currency: CurrencyType,
    pub timestamp: u64,
}

// ============================================================================
// TRADING NPCS
// ============================================================================

/// Types of trading NPCs
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TradingNpcType {
    /// Roaming merchant with random goods
    TravelingMerchant,
    /// Sells rare and exotic items
    RareItemDealer,
    /// Buys specific items at premium prices
    Collector,
    /// Exchanges currency and items
    Broker,
    /// Fence for stolen goods
    Fence,
    /// Sells black market goods
    ShadowDealer,
    /// Buys monster parts
    MonsterPartsBuyer,
    /// Sells treasure maps and tips
    InformationBroker,
    /// Ancient entity with rare trades
    MysteriousTrader,
}

impl TradingNpcType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::TravelingMerchant => "Traveling Merchant",
            Self::RareItemDealer => "Rare Item Dealer",
            Self::Collector => "Collector",
            Self::Broker => "Broker",
            Self::Fence => "Fence",
            Self::ShadowDealer => "Shadow Dealer",
            Self::MonsterPartsBuyer => "Monster Parts Buyer",
            Self::InformationBroker => "Information Broker",
            Self::MysteriousTrader => "Mysterious Trader",
        }
    }

    pub fn faction(&self) -> MerchantFaction {
        match self {
            Self::TravelingMerchant => MerchantFaction::TravelingMerchants,
            Self::RareItemDealer | Self::Collector => MerchantFaction::RareCollectors,
            Self::Broker => MerchantFaction::GeneralMerchants,
            Self::Fence | Self::ShadowDealer => MerchantFaction::BlackMarket,
            Self::MonsterPartsBuyer => MerchantFaction::GeneralMerchants,
            Self::InformationBroker => MerchantFaction::TravelingMerchants,
            Self::MysteriousTrader => MerchantFaction::AncientTraders,
        }
    }

    pub fn spawn_chance(&self) -> f64 {
        match self {
            Self::TravelingMerchant => 0.05,
            Self::RareItemDealer => 0.02,
            Self::Collector => 0.03,
            Self::Broker => 0.04,
            Self::Fence => 0.02,
            Self::ShadowDealer => 0.01,
            Self::MonsterPartsBuyer => 0.03,
            Self::InformationBroker => 0.02,
            Self::MysteriousTrader => 0.005,
        }
    }
}

/// A trading NPC instance
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradingNpc {
    pub id: u64,
    pub name: String,
    pub npc_type: TradingNpcType,
    pub x: usize,
    pub y: usize,
    pub inventory: Vec<ShopItem>,
    /// Items this NPC wants to buy
    pub wanted_items: Vec<WantedItem>,
    /// Special trades offered
    pub special_trades: Vec<SpecialTrade>,
    /// Turns until NPC leaves
    pub departure_timer: u32,
    /// Whether NPC has been discovered
    pub discovered: bool,
    /// Personal relationship with player
    pub relationship: i32,
}

impl TradingNpc {
    pub fn new(id: u64, name: &str, npc_type: TradingNpcType, x: usize, y: usize) -> Self {
        let departure_timer = match npc_type {
            TradingNpcType::TravelingMerchant => 50,
            TradingNpcType::MysteriousTrader => 20,
            _ => 100,
        };

        Self {
            id,
            name: name.to_string(),
            npc_type,
            x,
            y,
            inventory: Vec::new(),
            wanted_items: Vec::new(),
            special_trades: Vec::new(),
            departure_timer,
            discovered: false,
            relationship: 0,
        }
    }

    pub fn tick(&mut self) -> bool {
        self.departure_timer = self.departure_timer.saturating_sub(1);
        self.departure_timer == 0
    }

    pub fn improve_relationship(&mut self, amount: i32) {
        self.relationship = (self.relationship + amount).clamp(-100, 100);
    }

    pub fn relationship_discount(&self) -> i32 {
        self.relationship / 10 // -10% to +10%
    }
}

/// An item a collector wants
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WantedItem {
    pub item_name: String,
    pub item_data: String,
    pub quantity_wanted: u32,
    pub quantity_bought: u32,
    pub price_per_unit: Price,
    /// Premium percentage over base value
    pub premium_percent: u32,
}

/// A special trade offer
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpecialTrade {
    pub id: u64,
    pub description: String,
    pub required_items: Vec<BarterItem>,
    pub required_currency: Option<(CurrencyType, u64)>,
    pub reward_item: Option<String>,
    pub reward_currency: Option<(CurrencyType, u64)>,
    pub one_time_only: bool,
    pub completed: bool,
}

// ============================================================================
// ECONOMY SYSTEM
// ============================================================================

/// Market trends
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketTrend {
    Crashing,
    Declining,
    Stable,
    Rising,
    Booming,
}

impl MarketTrend {
    pub fn price_modifier(&self) -> f64 {
        match self {
            Self::Crashing => 0.5,
            Self::Declining => 0.8,
            Self::Stable => 1.0,
            Self::Rising => 1.2,
            Self::Booming => 1.5,
        }
    }
}

/// Price history entry
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PriceHistoryEntry {
    pub timestamp: u64,
    pub price: u64,
    pub volume: u32,
}

/// Tracks a single item's market data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemMarketData {
    pub item_id: String,
    pub item_name: String,
    pub current_price: u64,
    pub base_price: u64,
    pub supply: u32,
    pub demand: u32,
    pub price_history: Vec<PriceHistoryEntry>,
    pub trend: MarketTrend,
    pub is_rare: bool,
    pub last_seen: u64,
}

impl ItemMarketData {
    pub fn new(item_id: &str, item_name: &str, base_price: u64) -> Self {
        Self {
            item_id: item_id.to_string(),
            item_name: item_name.to_string(),
            current_price: base_price,
            base_price,
            supply: 10,
            demand: 10,
            price_history: Vec::new(),
            trend: MarketTrend::Stable,
            is_rare: false,
            last_seen: 0,
        }
    }

    pub fn update_price(&mut self) {
        // Price based on supply/demand ratio
        let ratio = if self.supply > 0 {
            self.demand as f64 / self.supply as f64
        } else {
            2.0
        };

        let trend_mod = self.trend.price_modifier();
        let new_price = (self.base_price as f64 * ratio * trend_mod) as u64;

        self.current_price = new_price.clamp(self.base_price / 4, self.base_price * 4);
    }

    pub fn record_sale(&mut self, price: u64, quantity: u32, timestamp: u64) {
        self.price_history.push(PriceHistoryEntry {
            timestamp,
            price,
            volume: quantity,
        });

        // Keep last 100 entries
        if self.price_history.len() > 100 {
            self.price_history.remove(0);
        }

        // Increase demand, decrease supply
        self.demand = (self.demand + quantity / 2).min(100);
        self.supply = self.supply.saturating_sub(quantity);
        self.last_seen = timestamp;
    }

    pub fn record_listing(&mut self, quantity: u32) {
        self.supply = (self.supply + quantity).min(100);
    }

    pub fn update_trend(&mut self) {
        if self.price_history.len() < 5 {
            self.trend = MarketTrend::Stable;
            return;
        }

        let recent: Vec<_> = self.price_history.iter().rev().take(5).collect();
        let avg_recent: u64 = recent.iter().map(|e| e.price).sum::<u64>() / 5;
        let avg_old: u64 = self.price_history.iter().take(5).map(|e| e.price).sum::<u64>() / 5;

        let change = if avg_old > 0 {
            (avg_recent as f64 - avg_old as f64) / avg_old as f64
        } else {
            0.0
        };

        self.trend = if change < -0.3 {
            MarketTrend::Crashing
        } else if change < -0.1 {
            MarketTrend::Declining
        } else if change > 0.3 {
            MarketTrend::Booming
        } else if change > 0.1 {
            MarketTrend::Rising
        } else {
            MarketTrend::Stable
        };
    }

    pub fn decay_demand(&mut self) {
        // Demand slowly returns to baseline
        if self.demand > 10 {
            self.demand = self.demand.saturating_sub(1);
        } else if self.demand < 10 {
            self.demand += 1;
        }
    }

    pub fn average_price(&self, periods: usize) -> u64 {
        if self.price_history.is_empty() {
            return self.current_price;
        }

        let entries: Vec<_> = self.price_history.iter().rev().take(periods).collect();
        if entries.is_empty() {
            return self.current_price;
        }

        entries.iter().map(|e| e.price).sum::<u64>() / entries.len() as u64
    }
}

/// The economy system
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EconomySystem {
    /// Market data for tracked items
    market_data: HashMap<String, ItemMarketData>,
    /// Rare items that have been seen
    rare_items_seen: Vec<RareItemSighting>,
    /// Global economic modifiers
    pub inflation_rate: f64,
    pub economic_health: f64,
    /// Current game time for timestamps
    current_time: u64,
}

impl EconomySystem {
    pub fn new() -> Self {
        Self {
            market_data: HashMap::new(),
            rare_items_seen: Vec::new(),
            inflation_rate: 0.0,
            economic_health: 1.0,
            current_time: 0,
        }
    }

    pub fn track_item(&mut self, item_id: &str, item_name: &str, base_price: u64) {
        if !self.market_data.contains_key(item_id) {
            self.market_data.insert(
                item_id.to_string(),
                ItemMarketData::new(item_id, item_name, base_price),
            );
        }
    }

    pub fn get_market_data(&self, item_id: &str) -> Option<&ItemMarketData> {
        self.market_data.get(item_id)
    }

    pub fn get_market_data_mut(&mut self, item_id: &str) -> Option<&mut ItemMarketData> {
        self.market_data.get_mut(item_id)
    }

    pub fn record_sale(&mut self, item_id: &str, price: u64, quantity: u32) {
        if let Some(data) = self.market_data.get_mut(item_id) {
            data.record_sale(price, quantity, self.current_time);
        }
    }

    pub fn record_listing(&mut self, item_id: &str, quantity: u32) {
        if let Some(data) = self.market_data.get_mut(item_id) {
            data.record_listing(quantity);
        }
    }

    pub fn get_current_price(&self, item_id: &str) -> Option<u64> {
        self.market_data.get(item_id).map(|d| {
            let price = d.current_price as f64 * (1.0 + self.inflation_rate) * self.economic_health;
            price as u64
        })
    }

    pub fn record_rare_item(&mut self, item_name: &str, location: &str, price: Option<u64>) {
        self.rare_items_seen.push(RareItemSighting {
            item_name: item_name.to_string(),
            location: location.to_string(),
            price,
            timestamp: self.current_time,
        });
    }

    pub fn get_rare_sightings(&self, limit: usize) -> &[RareItemSighting] {
        let start = self.rare_items_seen.len().saturating_sub(limit);
        &self.rare_items_seen[start..]
    }

    pub fn tick(&mut self) {
        self.current_time += 1;

        // Update all tracked items
        for data in self.market_data.values_mut() {
            data.decay_demand();
            data.update_price();

            // Update trend periodically
            if self.current_time % 100 == 0 {
                data.update_trend();
            }
        }

        // Slowly normalize economic health
        if self.economic_health < 1.0 {
            self.economic_health = (self.economic_health + 0.001).min(1.0);
        } else if self.economic_health > 1.0 {
            self.economic_health = (self.economic_health - 0.001).max(1.0);
        }
    }

    /// Trigger an economic event
    pub fn trigger_event(&mut self, event: EconomicEvent) {
        match event {
            EconomicEvent::MarketCrash => {
                self.economic_health *= 0.7;
                for data in self.market_data.values_mut() {
                    data.trend = MarketTrend::Crashing;
                }
            }
            EconomicEvent::GoldRush => {
                self.inflation_rate += 0.1;
            }
            EconomicEvent::TradeWar { category } => {
                for data in self.market_data.values_mut() {
                    // This would need item->category mapping
                    data.supply = data.supply.saturating_sub(5);
                }
                let _ = category; // Suppress unused warning
            }
            EconomicEvent::Surplus { category } => {
                for data in self.market_data.values_mut() {
                    data.supply = (data.supply + 10).min(100);
                }
                let _ = category;
            }
            EconomicEvent::Shortage { category } => {
                for data in self.market_data.values_mut() {
                    data.supply = data.supply.saturating_sub(10);
                    data.trend = MarketTrend::Rising;
                }
                let _ = category;
            }
        }
    }

    pub fn get_trending_items(&self, rising: bool, limit: usize) -> Vec<&ItemMarketData> {
        let mut items: Vec<_> = self.market_data.values()
            .filter(|d| {
                if rising {
                    matches!(d.trend, MarketTrend::Rising | MarketTrend::Booming)
                } else {
                    matches!(d.trend, MarketTrend::Declining | MarketTrend::Crashing)
                }
            })
            .collect();

        items.sort_by(|a, b| {
            let a_change = a.current_price as f64 / a.base_price as f64;
            let b_change = b.current_price as f64 / b.base_price as f64;
            if rising {
                b_change.partial_cmp(&a_change).unwrap_or(std::cmp::Ordering::Equal)
            } else {
                a_change.partial_cmp(&b_change).unwrap_or(std::cmp::Ordering::Equal)
            }
        });

        items.into_iter().take(limit).collect()
    }
}

/// Rare item sighting record
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RareItemSighting {
    pub item_name: String,
    pub location: String,
    pub price: Option<u64>,
    pub timestamp: u64,
}

/// Economic events that affect the market
#[derive(Clone, Debug)]
pub enum EconomicEvent {
    MarketCrash,
    GoldRush,
    TradeWar { category: TradeGoodCategory },
    Surplus { category: TradeGoodCategory },
    Shortage { category: TradeGoodCategory },
}

// ============================================================================
// DIRECT TRADING (Player-NPC Trade)
// ============================================================================

/// A direct trade session with an NPC
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeSession {
    pub id: u64,
    pub npc_id: u64,
    pub npc_name: String,
    pub player_offer: Vec<TradeItem>,
    pub npc_offer: Vec<TradeItem>,
    pub player_gold: i64,
    pub npc_gold: i64,
    pub status: TradeStatus,
    pub npc_satisfaction: i32, // -100 to 100
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeItem {
    pub item_id: String,
    pub item_name: String,
    pub quantity: u32,
    pub value: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeStatus {
    Negotiating,
    PlayerAccepted,
    NpcAccepted,
    Completed,
    Cancelled,
    Rejected,
}

impl TradeSession {
    pub fn new(id: u64, npc_id: u64, npc_name: &str) -> Self {
        Self {
            id,
            npc_id,
            npc_name: npc_name.to_string(),
            player_offer: Vec::new(),
            npc_offer: Vec::new(),
            player_gold: 0,
            npc_gold: 0,
            status: TradeStatus::Negotiating,
            npc_satisfaction: 0,
        }
    }

    pub fn player_total_value(&self) -> u64 {
        self.player_offer.iter().map(|i| i.value * i.quantity as u64).sum::<u64>()
            + if self.player_gold > 0 { self.player_gold as u64 } else { 0 }
    }

    pub fn npc_total_value(&self) -> u64 {
        self.npc_offer.iter().map(|i| i.value * i.quantity as u64).sum::<u64>()
            + if self.npc_gold > 0 { self.npc_gold as u64 } else { 0 }
    }

    pub fn value_difference(&self) -> i64 {
        self.player_total_value() as i64 - self.npc_total_value() as i64
    }

    pub fn is_fair_for_player(&self) -> bool {
        self.value_difference() >= 0
    }

    pub fn update_npc_satisfaction(&mut self) {
        let diff = self.value_difference();
        let npc_value = self.npc_total_value() as i64;

        if npc_value == 0 {
            self.npc_satisfaction = if diff > 0 { 100 } else { -100 };
            return;
        }

        // NPC satisfaction based on how good the deal is for them
        let ratio = diff as f64 / npc_value as f64;
        self.npc_satisfaction = (ratio * 100.0).clamp(-100.0, 100.0) as i32;
    }

    /// Check if NPC would accept this trade
    pub fn npc_would_accept(&self, npc_greed: i32) -> bool {
        // npc_greed: -50 (generous) to 50 (greedy)
        self.npc_satisfaction >= -10 + npc_greed
    }

    pub fn add_player_item(&mut self, item: TradeItem) {
        self.player_offer.push(item);
        self.update_npc_satisfaction();
    }

    pub fn add_npc_item(&mut self, item: TradeItem) {
        self.npc_offer.push(item);
        self.update_npc_satisfaction();
    }

    pub fn set_player_gold(&mut self, amount: i64) {
        self.player_gold = amount;
        self.npc_gold = -amount;
        self.update_npc_satisfaction();
    }

    pub fn complete(&mut self) -> bool {
        if self.status == TradeStatus::Negotiating {
            self.status = TradeStatus::Completed;
            true
        } else {
            false
        }
    }

    pub fn cancel(&mut self) {
        self.status = TradeStatus::Cancelled;
    }

    pub fn reject(&mut self) {
        self.status = TradeStatus::Rejected;
    }
}

// ============================================================================
// MAIN TRADING SYSTEM
// ============================================================================

/// The main trading system that coordinates all trading functionality
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradingSystem {
    /// Player's wallet
    pub wallet: Wallet,
    /// Currency exchange service
    pub exchange: CurrencyExchange,
    /// Merchant reputation
    pub reputation: MerchantReputation,
    /// Trade routes and regional pricing
    pub trade_routes: TradeRouteSystem,
    /// All shops in the world
    pub shops: HashMap<u64, Shop>,
    /// Auction house
    pub auction_house: AuctionHouse,
    /// Trading NPCs currently in the world
    pub trading_npcs: HashMap<u64, TradingNpc>,
    /// Economy system
    pub economy: EconomySystem,
    /// Active trade sessions
    pub trade_sessions: HashMap<u64, TradeSession>,
    /// Next IDs for various entities
    next_shop_id: u64,
    next_npc_id: u64,
    next_session_id: u64,
    /// Statistics
    pub stats: TradingStats,
}

impl Default for TradingSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl TradingSystem {
    pub fn new() -> Self {
        Self {
            wallet: Wallet::new(),
            exchange: CurrencyExchange::new(),
            reputation: MerchantReputation::new(),
            trade_routes: TradeRouteSystem::new(),
            shops: HashMap::new(),
            auction_house: AuctionHouse::new(),
            trading_npcs: HashMap::new(),
            economy: EconomySystem::new(),
            trade_sessions: HashMap::new(),
            next_shop_id: 1,
            next_npc_id: 1,
            next_session_id: 1,
            stats: TradingStats::default(),
        }
    }

    // ========== Shop Management ==========

    pub fn create_shop(&mut self, name: &str, shop_type: ShopType, region: TradeRegion) -> u64 {
        let id = self.next_shop_id;
        self.next_shop_id += 1;

        let shop = Shop::new(id, name, shop_type, region);
        self.shops.insert(id, shop);
        id
    }

    pub fn get_shop(&self, shop_id: u64) -> Option<&Shop> {
        self.shops.get(&shop_id)
    }

    pub fn get_shop_mut(&mut self, shop_id: u64) -> Option<&mut Shop> {
        self.shops.get_mut(&shop_id)
    }

    pub fn buy_from_shop(&mut self, shop_id: u64, item_id: u64) -> Result<String, TradeError> {
        let shop = self.shops.get(&shop_id).ok_or(TradeError::ShopNotFound)?;

        // Check reputation
        let rep_level = self.reputation.effective_level(shop.shop_type.faction(), shop.owner_id);
        if !rep_level.will_trade() {
            return Err(TradeError::InsufficientReputation);
        }

        let item = shop.get_item(item_id).ok_or(TradeError::ItemNotFound)?;

        // Check item reputation requirement
        if rep_level < item.required_reputation {
            return Err(TradeError::InsufficientReputation);
        }

        // Check quantity
        if let Some(qty) = item.quantity {
            if qty == 0 {
                return Err(TradeError::OutOfStock);
            }
        }

        // Calculate price
        let price = shop.calculate_price(item, &self.trade_routes, &self.reputation);

        // Check if player can afford
        if !self.wallet.can_afford_price(&price) {
            return Err(TradeError::InsufficientFunds);
        }

        let item_name = item.name.clone();
        let item_data = item.item_data.clone();

        // Complete purchase
        self.wallet.pay_price(&price);
        self.stats.total_spent += price.total_gold_value();
        self.stats.items_bought += 1;

        // Decrease quantity if limited
        let shop = self.shops.get_mut(&shop_id).unwrap();
        if let Some(item) = shop.get_item_mut(item_id) {
            if let Some(ref mut qty) = item.quantity {
                *qty = qty.saturating_sub(1);
            }
        }

        // Record in economy
        self.economy.record_sale(&item_data, price.total_gold_value(), 1);

        // Improve reputation
        self.reputation.modify_faction_rep(shop.shop_type.faction(), 1);

        Ok(item_name)
    }

    pub fn sell_to_shop(&mut self, shop_id: u64, item_data: &str, item_name: &str, base_value: u64) -> Result<u64, TradeError> {
        let shop = self.shops.get(&shop_id).ok_or(TradeError::ShopNotFound)?;

        let rep_level = self.reputation.effective_level(shop.shop_type.faction(), shop.owner_id);
        if !rep_level.will_trade() {
            return Err(TradeError::InsufficientReputation);
        }

        // Calculate sell price (typically 50% of buy price, modified by reputation)
        let sell_modifier = 50 + rep_level.price_modifier().abs() as u64;
        let regional_mod = self.trade_routes.get_price_modifier(TradeGoodCategory::General);
        let total_mod = sell_modifier as i32 - regional_mod;

        let sell_price = (base_value * total_mod as u64 / 100).max(1);

        // Add to player wallet
        self.wallet.add(shop.shop_type.primary_currency(), sell_price);
        self.stats.total_earned += sell_price;
        self.stats.items_sold += 1;

        // Record in economy
        self.economy.record_listing(item_data, 1);

        // Improve reputation
        let faction = shop.shop_type.faction();
        self.reputation.modify_faction_rep(faction, 1);

        Ok(sell_price)
    }

    // ========== NPC Trading ==========

    pub fn spawn_trading_npc(&mut self, npc_type: TradingNpcType, name: &str, x: usize, y: usize) -> u64 {
        let id = self.next_npc_id;
        self.next_npc_id += 1;

        let npc = TradingNpc::new(id, name, npc_type, x, y);
        self.trading_npcs.insert(id, npc);
        id
    }

    pub fn get_trading_npc(&self, npc_id: u64) -> Option<&TradingNpc> {
        self.trading_npcs.get(&npc_id)
    }

    pub fn get_trading_npc_mut(&mut self, npc_id: u64) -> Option<&mut TradingNpc> {
        self.trading_npcs.get_mut(&npc_id)
    }

    pub fn start_trade_session(&mut self, npc_id: u64) -> Result<u64, TradeError> {
        let npc = self.trading_npcs.get(&npc_id).ok_or(TradeError::NpcNotFound)?;

        let session_id = self.next_session_id;
        self.next_session_id += 1;

        let session = TradeSession::new(session_id, npc_id, &npc.name);
        self.trade_sessions.insert(session_id, session);

        Ok(session_id)
    }

    pub fn get_trade_session(&self, session_id: u64) -> Option<&TradeSession> {
        self.trade_sessions.get(&session_id)
    }

    pub fn get_trade_session_mut(&mut self, session_id: u64) -> Option<&mut TradeSession> {
        self.trade_sessions.get_mut(&session_id)
    }

    pub fn complete_trade_session(&mut self, session_id: u64) -> Result<(), TradeError> {
        let session = self.trade_sessions.get_mut(&session_id).ok_or(TradeError::SessionNotFound)?;

        if session.status != TradeStatus::Negotiating {
            return Err(TradeError::InvalidTradeState);
        }

        // Check if NPC accepts
        if !session.npc_would_accept(0) {
            session.reject();
            return Err(TradeError::TradeRejected);
        }

        // Process gold exchange
        if session.player_gold > 0 {
            if !self.wallet.can_afford(CurrencyType::Gold, session.player_gold as u64) {
                return Err(TradeError::InsufficientFunds);
            }
            self.wallet.subtract(CurrencyType::Gold, session.player_gold as u64);
        } else if session.player_gold < 0 {
            self.wallet.add(CurrencyType::Gold, (-session.player_gold) as u64);
        }

        session.complete();
        self.stats.trades_completed += 1;

        // Improve NPC relationship
        if let Some(npc) = self.trading_npcs.get_mut(&session.npc_id) {
            npc.improve_relationship(5);
        }

        Ok(())
    }

    // ========== Auction House ==========

    pub fn list_auction(&mut self, listing: AuctionListing) -> Result<u64, TradeError> {
        let fee = self.auction_house.calculate_listing_fee(listing.auction_type, listing.starting_price);

        if !self.wallet.can_afford(CurrencyType::Gold, fee) {
            return Err(TradeError::InsufficientFunds);
        }

        self.wallet.subtract(CurrencyType::Gold, fee);

        let id = self.auction_house.list_item(listing)
            .map_err(|_| TradeError::AuctionFailed)?;

        self.stats.auctions_created += 1;
        Ok(id)
    }

    pub fn bid_on_auction(&mut self, listing_id: u64, amount: u64, bidder_name: &str) -> Result<(), TradeError> {
        let listing = self.auction_house.get_listing(listing_id)
            .ok_or(TradeError::AuctionNotFound)?;

        if !self.wallet.can_afford(listing.currency, amount) {
            return Err(TradeError::InsufficientFunds);
        }

        let bid = AuctionBid {
            bidder_id: 0, // Would be player ID
            bidder_name: bidder_name.to_string(),
            amount,
            currency: listing.currency,
            timestamp: 0,
            is_auto_bid: false,
            max_auto_bid: None,
        };

        self.auction_house.place_bid(listing_id, bid)
            .map_err(|e| match e {
                AuctionError::BidTooLow => TradeError::BidTooLow,
                AuctionError::AuctionEnded => TradeError::AuctionEnded,
                _ => TradeError::AuctionFailed,
            })?;

        self.stats.bids_placed += 1;
        Ok(())
    }

    // ========== System Updates ==========

    pub fn tick(&mut self) -> Vec<TradingEvent> {
        let mut events = Vec::new();

        // Update shops
        for shop in self.shops.values_mut() {
            if shop.tick() {
                events.push(TradingEvent::ShopRestocked { shop_id: shop.id });
            }
        }

        // Update trading NPCs
        let mut departed_npcs = Vec::new();
        for (id, npc) in &mut self.trading_npcs {
            if npc.tick() {
                departed_npcs.push(*id);
                events.push(TradingEvent::NpcDeparted {
                    npc_id: *id,
                    npc_name: npc.name.clone(),
                });
            }
        }
        for id in departed_npcs {
            self.trading_npcs.remove(&id);
        }

        // Update auctions
        let auction_results = self.auction_house.tick();
        for result in auction_results {
            if result.status == AuctionStatus::Sold {
                if let Some((winner_id, winner_name)) = result.winner {
                    events.push(TradingEvent::AuctionEnded {
                        listing_id: result.listing_id,
                        item_name: result.item_name,
                        winner_id: Some(winner_id),
                        winner_name: Some(winner_name),
                        final_price: result.final_price,
                    });
                }
            } else {
                events.push(TradingEvent::AuctionExpired {
                    listing_id: result.listing_id,
                    item_name: result.item_name,
                });
            }
        }

        // Update caravans
        let caravan_results = self.trade_routes.update_caravans();
        for result in caravan_results {
            events.push(TradingEvent::CaravanArrived {
                caravan_id: result.caravan_id,
                destination: result.destination,
            });
        }

        // Update economy
        self.economy.tick();

        events
    }

    pub fn reset_daily(&mut self) {
        self.exchange.reset_daily_limits();
    }

    // ========== Utility Methods ==========

    pub fn set_region(&mut self, region: TradeRegion) {
        self.trade_routes.set_current_region(region);
    }

    pub fn get_shops_in_region(&self, region: TradeRegion) -> Vec<&Shop> {
        self.shops.values().filter(|s| s.region == region).collect()
    }

    pub fn get_nearby_npcs(&self, x: usize, y: usize, range: usize) -> Vec<&TradingNpc> {
        self.trading_npcs.values().filter(|npc| {
            let dx = (npc.x as i32 - x as i32).abs() as usize;
            let dy = (npc.y as i32 - y as i32).abs() as usize;
            dx <= range && dy <= range
        }).collect()
    }
}

/// Trading-related errors
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TradeError {
    InsufficientFunds,
    InsufficientReputation,
    ItemNotFound,
    ShopNotFound,
    NpcNotFound,
    SessionNotFound,
    AuctionNotFound,
    OutOfStock,
    TradeRejected,
    InvalidTradeState,
    BidTooLow,
    AuctionEnded,
    AuctionFailed,
}

/// Events generated by the trading system
#[derive(Clone, Debug)]
pub enum TradingEvent {
    ShopRestocked { shop_id: u64 },
    NpcDeparted { npc_id: u64, npc_name: String },
    AuctionEnded {
        listing_id: u64,
        item_name: String,
        winner_id: Option<u64>,
        winner_name: Option<String>,
        final_price: u64,
    },
    AuctionExpired { listing_id: u64, item_name: String },
    CaravanArrived { caravan_id: u64, destination: TradeRegion },
    PriceChange { item_id: String, old_price: u64, new_price: u64 },
    RareItemSpotted { item_name: String, location: String },
}

/// Trading statistics
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TradingStats {
    pub items_bought: u64,
    pub items_sold: u64,
    pub total_spent: u64,
    pub total_earned: u64,
    pub trades_completed: u64,
    pub auctions_created: u64,
    pub auctions_won: u64,
    pub bids_placed: u64,
    pub best_profit: u64,
    pub worst_loss: u64,
}

// ============================================================================
// GENERATOR FUNCTIONS
// ============================================================================

/// Generate default shops for a region
pub fn generate_region_shops(region: TradeRegion) -> Vec<Shop> {
    let region_name = region.name();
    let mut shops = Vec::new();
    let mut id = 1;

    // General Store
    let mut general = Shop::new(id, &format!("{} General Store", region_name), ShopType::GeneralStore, region);
    general.add_item(ShopItem::new(1, "Torch", "Illuminates dark areas", Price::gold(5)).with_category(TradeGoodCategory::General));
    general.add_item(ShopItem::new(2, "Rope", "50 feet of sturdy rope", Price::gold(10)).with_category(TradeGoodCategory::General));
    general.add_item(ShopItem::new(3, "Rations", "A day's worth of food", Price::gold(3)).with_category(TradeGoodCategory::Food));
    shops.push(general);
    id += 1;

    // Weapon Shop
    let mut weapons = Shop::new(id, &format!("{} Armory", region_name), ShopType::WeaponShop, region);
    weapons.add_item(ShopItem::new(1, "Iron Sword", "A reliable blade", Price::gold(50)).with_category(TradeGoodCategory::Weapons));
    weapons.add_item(ShopItem::new(2, "Steel Axe", "Heavy chopping weapon", Price::gold(75)).with_category(TradeGoodCategory::Weapons));
    weapons.add_item(ShopItem::new(3, "Longbow", "Ranged weapon with good range", Price::gold(60)).with_category(TradeGoodCategory::Weapons));
    shops.push(weapons);
    id += 1;

    // Armor Shop
    let mut armor = Shop::new(id, &format!("{} Armor Smith", region_name), ShopType::ArmorShop, region);
    armor.add_item(ShopItem::new(1, "Leather Armor", "Light protection", Price::gold(40)).with_category(TradeGoodCategory::Armor));
    armor.add_item(ShopItem::new(2, "Chain Mail", "Medium protection", Price::gold(100)).with_category(TradeGoodCategory::Armor));
    armor.add_item(ShopItem::new(3, "Iron Shield", "Blocks attacks", Price::gold(35)).with_category(TradeGoodCategory::Armor));
    shops.push(armor);
    id += 1;

    // Alchemy Shop
    let mut alchemy = Shop::new(id, &format!("{} Alchemist", region_name), ShopType::AlchemyShop, region);
    alchemy.add_item(ShopItem::new(1, "Health Potion", "Restores 50 HP", Price::gold(25)).with_category(TradeGoodCategory::General));
    alchemy.add_item(ShopItem::new(2, "Mana Potion", "Restores 30 MP", Price::gold(30)).with_category(TradeGoodCategory::General));
    alchemy.add_item(ShopItem::new(3, "Antidote", "Cures poison", Price::gold(15)).with_category(TradeGoodCategory::General));
    shops.push(alchemy);

    shops
}

/// Generate a traveling merchant with random inventory
pub fn generate_traveling_merchant(id: u64, x: usize, y: usize) -> TradingNpc {
    let names = [
        "Wandering Zephyr", "Marco the Trader", "Silk Road Sam",
        "Mysterious Merchant", "Fortune's Favor", "The Collector",
    ];

    let name = names[(id as usize) % names.len()];
    let mut npc = TradingNpc::new(id, name, TradingNpcType::TravelingMerchant, x, y);

    // Add some random inventory
    npc.inventory.push(ShopItem::new(
        1, "Exotic Spice", "Rare cooking ingredient",
        Price::gold(100)
    ).with_category(TradeGoodCategory::Spices));

    npc.inventory.push(ShopItem::new(
        2, "Ancient Map Fragment", "Part of a treasure map",
        Price::gold(250)
    ).with_category(TradeGoodCategory::Artifacts));

    npc
}

/// Generate a rare item dealer
pub fn generate_rare_dealer(id: u64, x: usize, y: usize) -> TradingNpc {
    let mut npc = TradingNpc::new(id, "The Antiquarian", TradingNpcType::RareItemDealer, x, y);

    npc.inventory.push(ShopItem::new(
        1, "Dragon Scale", "A scale from an ancient dragon",
        Price::new(CurrencyType::HighSpiritStone, 5)
    ).with_category(TradeGoodCategory::DemonParts).as_special());

    npc.inventory.push(ShopItem::new(
        2, "Phoenix Feather", "A feather that radiates warmth",
        Price::new(CurrencyType::HighSpiritStone, 3)
    ).with_category(TradeGoodCategory::CelestialMaterials).as_special());

    npc
}

/// Generate a collector looking for specific items
pub fn generate_collector(id: u64, x: usize, y: usize, specialty: &str) -> TradingNpc {
    let name = format!("{} Collector", specialty);
    let mut npc = TradingNpc::new(id, &name, TradingNpcType::Collector, x, y);

    npc.wanted_items.push(WantedItem {
        item_name: format!("Rare {}", specialty),
        item_data: specialty.to_lowercase(),
        quantity_wanted: 5,
        quantity_bought: 0,
        price_per_unit: Price::gold(500),
        premium_percent: 50,
    });

    npc
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_operations() {
        let mut wallet = Wallet::new();
        assert_eq!(wallet.balance(CurrencyType::Gold), 100);

        wallet.add(CurrencyType::Gold, 50);
        assert_eq!(wallet.balance(CurrencyType::Gold), 150);

        assert!(wallet.subtract(CurrencyType::Gold, 100));
        assert_eq!(wallet.balance(CurrencyType::Gold), 50);

        assert!(!wallet.subtract(CurrencyType::Gold, 100));
        assert_eq!(wallet.balance(CurrencyType::Gold), 50);
    }

    #[test]
    fn test_spirit_stone_conversion() {
        let mut wallet = Wallet::new();
        wallet.add(CurrencyType::LowSpiritStone, 200);

        assert!(wallet.convert_spirit_stones(CurrencyType::LowSpiritStone, CurrencyType::MidSpiritStone, 100));
        assert_eq!(wallet.balance(CurrencyType::LowSpiritStone), 100);
        assert_eq!(wallet.balance(CurrencyType::MidSpiritStone), 1);
    }

    #[test]
    fn test_price_creation() {
        let price = Price::gold(100);
        assert_eq!(price.total_gold_value(), 100);

        let multi_price = Price::multi(vec![
            (CurrencyType::Gold, 100),
            (CurrencyType::LowSpiritStone, 10),
        ]);
        assert_eq!(multi_price.total_gold_value(), 200);
    }

    #[test]
    fn test_reputation_levels() {
        assert_eq!(ReputationLevel::from_points(-5000), ReputationLevel::Hated);
        assert_eq!(ReputationLevel::from_points(0), ReputationLevel::Neutral);
        assert_eq!(ReputationLevel::from_points(5000), ReputationLevel::Honored);
        assert_eq!(ReputationLevel::from_points(15000), ReputationLevel::Exalted);
    }

    #[test]
    fn test_auction_bidding() {
        let mut listing = AuctionListing::new(
            1, 100, "Seller", "Test Item", "A test item",
            100, CurrencyType::Gold, AuctionType::Standard
        );

        let bid = AuctionBid {
            bidder_id: 200,
            bidder_name: "Bidder".to_string(),
            amount: 150,
            currency: CurrencyType::Gold,
            timestamp: 0,
            is_auto_bid: false,
            max_auto_bid: None,
        };

        assert!(listing.place_bid(bid).is_ok());
        assert_eq!(listing.current_price, 150);
        assert_eq!(listing.bid_count(), 1);
    }

    #[test]
    fn test_auction_buyout() {
        let mut listing = AuctionListing::new(
            1, 100, "Seller", "Test Item", "A test item",
            100, CurrencyType::Gold, AuctionType::Standard
        ).with_buyout(500);

        assert!(listing.buyout(200, "Buyer").is_ok());
        assert_eq!(listing.status, AuctionStatus::Sold);
        assert_eq!(listing.current_price, 500);
    }

    #[test]
    fn test_barter_fairness() {
        let mut offer = BarterOffer::new(1);

        offer.add_offered_item(BarterItem {
            item_data: "sword".to_string(),
            quantity: 1,
            estimated_value: 100,
        });

        offer.add_requested_item(BarterItem {
            item_data: "shield".to_string(),
            quantity: 1,
            estimated_value: 100,
        });

        assert!(offer.is_fair());
        assert_eq!(offer.balance_needed(), 0);
    }

    #[test]
    fn test_trade_session() {
        let mut session = TradeSession::new(1, 100, "Test NPC");

        session.add_player_item(TradeItem {
            item_id: "1".to_string(),
            item_name: "Gold Bar".to_string(),
            quantity: 1,
            value: 100,
        });

        assert_eq!(session.player_total_value(), 100);
        assert!(session.npc_would_accept(0)); // NPC gets free stuff
    }

    #[test]
    fn test_trading_system_creation() {
        let system = TradingSystem::new();
        assert_eq!(system.wallet.balance(CurrencyType::Gold), 100);
        assert!(system.shops.is_empty());
        assert!(system.trading_npcs.is_empty());
    }

    #[test]
    fn test_shop_creation() {
        let mut system = TradingSystem::new();
        let shop_id = system.create_shop("Test Shop", ShopType::GeneralStore, TradeRegion::CentralKingdom);

        assert!(system.get_shop(shop_id).is_some());

        let shop = system.get_shop_mut(shop_id).unwrap();
        shop.add_item(ShopItem::new(1, "Test Item", "A test", Price::gold(10)));

        assert_eq!(shop.inventory.len(), 1);
    }

    #[test]
    fn test_economy_system() {
        let mut economy = EconomySystem::new();
        economy.track_item("sword_1", "Iron Sword", 50);

        economy.record_sale("sword_1", 55, 1);

        let data = economy.get_market_data("sword_1").unwrap();
        assert_eq!(data.price_history.len(), 1);
    }

    #[test]
    fn test_trade_regions() {
        let system = TradeRouteSystem::new();

        // Items should be cheaper where abundant
        let eastern_herbs = system.get_price_modifier_for_region(
            TradeRegion::EasternForests,
            TradeGoodCategory::Herbs
        );
        let desert_herbs = system.get_price_modifier_for_region(
            TradeRegion::SouthernDeserts,
            TradeGoodCategory::Herbs
        );

        assert!(eastern_herbs < desert_herbs);
    }

    #[test]
    fn test_generate_region_shops() {
        let shops = generate_region_shops(TradeRegion::CentralKingdom);
        assert!(!shops.is_empty());
        assert!(shops.iter().any(|s| s.shop_type == ShopType::GeneralStore));
        assert!(shops.iter().any(|s| s.shop_type == ShopType::WeaponShop));
    }
}
