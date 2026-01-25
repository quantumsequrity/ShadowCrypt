//! Combat system: status effects, critical hits, dodge mechanics, elemental resistances, and combat calculations

use serde::{Serialize, Deserialize};

/// Elemental damage types
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Default)]
pub enum ElementType {
    /// Physical damage (non-elemental)
    #[default]
    Physical,
    /// Fire damage
    Fire,
    /// Ice/Cold damage
    Ice,
    /// Lightning/Electric damage
    Lightning,
    /// Poison/Toxic damage
    Poison,
}

impl ElementType {
    /// Returns the display name of the element
    pub fn name(&self) -> &'static str {
        match self {
            Self::Physical => "Physical",
            Self::Fire => "Fire",
            Self::Ice => "Ice",
            Self::Lightning => "Lightning",
            Self::Poison => "Poison",
        }
    }

    /// Returns a color index for UI rendering
    pub fn color_index(&self) -> u8 {
        match self {
            Self::Physical => 2,   // White
            Self::Fire => 3,       // Red
            Self::Ice => 9,        // Cyan
            Self::Lightning => 11, // Yellow
            Self::Poison => 5,     // Green
        }
    }

    /// Returns the associated status effect for this element
    pub fn status_effect(&self) -> Option<StatusEffect> {
        match self {
            Self::Physical => None,
            Self::Fire => Some(StatusEffect::Burn),
            Self::Ice => Some(StatusEffect::Freeze),
            Self::Lightning => Some(StatusEffect::Stun),
            Self::Poison => Some(StatusEffect::Poison),
        }
    }
}

/// Elemental resistances for damage reduction
/// Values are percentages from 0.0 (no resistance) to 1.0 (immune)
/// Negative values represent vulnerability (takes extra damage)
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub struct ElementalResistances {
    /// Fire resistance (reduces fire damage)
    pub fire: f32,
    /// Ice resistance (reduces ice damage)
    pub ice: f32,
    /// Lightning resistance (reduces lightning damage)
    pub lightning: f32,
    /// Poison resistance (reduces poison damage)
    pub poison: f32,
}

impl Default for ElementalResistances {
    fn default() -> Self {
        Self {
            fire: 0.0,
            ice: 0.0,
            lightning: 0.0,
            poison: 0.0,
        }
    }
}

impl ElementalResistances {
    /// Create new elemental resistances with specified values
    pub fn new(fire: f32, ice: f32, lightning: f32, poison: f32) -> Self {
        Self {
            fire: fire.clamp(-1.0, 1.0),
            ice: ice.clamp(-1.0, 1.0),
            lightning: lightning.clamp(-1.0, 1.0),
            poison: poison.clamp(-1.0, 1.0),
        }
    }

    /// Create resistances with uniform value for all elements
    pub fn uniform(value: f32) -> Self {
        let clamped = value.clamp(-1.0, 1.0);
        Self {
            fire: clamped,
            ice: clamped,
            lightning: clamped,
            poison: clamped,
        }
    }

    /// Get resistance value for a specific element type
    pub fn get(&self, element: ElementType) -> f32 {
        match element {
            ElementType::Physical => 0.0, // Physical damage uses armor, not elemental resistance
            ElementType::Fire => self.fire,
            ElementType::Ice => self.ice,
            ElementType::Lightning => self.lightning,
            ElementType::Poison => self.poison,
        }
    }

    /// Set resistance for a specific element type
    pub fn set(&mut self, element: ElementType, value: f32) {
        let clamped = value.clamp(-1.0, 1.0);
        match element {
            ElementType::Physical => {} // Physical resistance not stored here
            ElementType::Fire => self.fire = clamped,
            ElementType::Ice => self.ice = clamped,
            ElementType::Lightning => self.lightning = clamped,
            ElementType::Poison => self.poison = clamped,
        }
    }

    /// Add resistance values from another source (e.g., equipment)
    pub fn add(&mut self, other: &ElementalResistances) {
        self.fire = (self.fire + other.fire).clamp(-1.0, 1.0);
        self.ice = (self.ice + other.ice).clamp(-1.0, 1.0);
        self.lightning = (self.lightning + other.lightning).clamp(-1.0, 1.0);
        self.poison = (self.poison + other.poison).clamp(-1.0, 1.0);
    }

    /// Combine with another resistance set (takes maximum of each)
    pub fn combine_max(&self, other: &ElementalResistances) -> Self {
        Self {
            fire: self.fire.max(other.fire).clamp(-1.0, 1.0),
            ice: self.ice.max(other.ice).clamp(-1.0, 1.0),
            lightning: self.lightning.max(other.lightning).clamp(-1.0, 1.0),
            poison: self.poison.max(other.poison).clamp(-1.0, 1.0),
        }
    }

    /// Check if entity is immune to an element (100% resistance)
    pub fn is_immune(&self, element: ElementType) -> bool {
        self.get(element) >= 1.0
    }

    /// Check if entity is vulnerable to an element (negative resistance)
    pub fn is_vulnerable(&self, element: ElementType) -> bool {
        self.get(element) < 0.0
    }

    /// Calculate the damage multiplier for an element
    /// Returns value to multiply damage by (1.0 = full damage, 0.0 = immune, >1.0 = vulnerable)
    pub fn damage_multiplier(&self, element: ElementType) -> f32 {
        let resistance = self.get(element);
        (1.0 - resistance).max(0.0)
    }

    /// Get a formatted string showing all resistances
    pub fn display(&self) -> String {
        format!(
            "Fire: {:+.0}% | Ice: {:+.0}% | Lightning: {:+.0}% | Poison: {:+.0}%",
            self.fire * 100.0,
            self.ice * 100.0,
            self.lightning * 100.0,
            self.poison * 100.0
        )
    }
}

/// Calculate damage after applying elemental resistance
/// Returns the reduced damage amount
pub fn calculate_elemental_damage(
    base_damage: i32,
    element: ElementType,
    resistances: &ElementalResistances,
) -> i32 {
    let multiplier = resistances.damage_multiplier(element);
    ((base_damage as f32) * multiplier).round() as i32
}

/// Calculate damage with both armor and elemental resistance
pub fn calculate_full_damage(
    base_damage: i32,
    element: ElementType,
    armor: i32,
    resistances: &ElementalResistances,
) -> i32 {
    // First apply armor reduction for physical component
    let after_armor = if element == ElementType::Physical {
        (base_damage - armor).max(1)
    } else {
        // Elemental damage ignores some armor (25% effective)
        (base_damage - armor / 4).max(1)
    };

    // Then apply elemental resistance
    calculate_elemental_damage(after_armor, element, resistances)
}

/// Status effects that can affect entities
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum StatusEffect {
    Poison,
    Burn,
    Freeze,
    Bleed,
    Stun,
    Blind,
    Haste,
    Shield,
    Regeneration,
    Strength,
    Weakness,
    Invisibility,
    Confusion,
    Focus,
    Evasion,
    ArmorBreak,
    Silence,
    Slow,
    Vulnerable,
    Enrage,
    Curse,
    Blessed,
    Vampiric,
    Thorns,
}

impl StatusEffect {
    /// Returns the display name of the status effect
    pub fn name(&self) -> &'static str {
        match self {
            Self::Poison => "Poisoned",
            Self::Burn => "Burning",
            Self::Freeze => "Frozen",
            Self::Bleed => "Bleeding",
            Self::Stun => "Stunned",
            Self::Blind => "Blind",
            Self::Haste => "Haste",
            Self::Shield => "Shielded",
            Self::Regeneration => "Regenerating",
            Self::Strength => "Strengthened",
            Self::Weakness => "Weakened",
            Self::Invisibility => "Invisible",
            Self::Confusion => "Confused",
            Self::Focus => "Focused",
            Self::Evasion => "Evasive",
            Self::ArmorBreak => "Armor Broken",
            Self::Silence => "Silenced",
            Self::Slow => "Slowed",
            Self::Vulnerable => "Vulnerable",
            Self::Enrage => "Enraged",
            Self::Curse => "Cursed",
            Self::Blessed => "Blessed",
            Self::Vampiric => "Vampiric",
            Self::Thorns => "Thorns",
        }
    }

    /// Returns a color index for the status effect (for UI rendering)
    /// 0=DarkGrey, 1=Grey, 2=White, 3=Red, 4=DarkRed, 5=Green, 6=DarkGreen,
    /// 7=Blue, 8=DarkBlue, 9=Cyan, 10=DarkCyan, 11=Yellow, 12=DarkYellow,
    /// 13=Magenta, 14=DarkMagenta
    pub fn color_index(&self) -> u8 {
        match self {
            Self::Poison => 5,        // Green
            Self::Burn => 3,          // Red
            Self::Freeze => 9,        // Cyan
            Self::Bleed => 4,         // DarkRed
            Self::Stun => 11,         // Yellow
            Self::Blind => 0,         // DarkGrey
            Self::Haste => 7,         // Blue
            Self::Shield => 2,        // White
            Self::Regeneration => 13, // Magenta
            Self::Strength => 11,     // Yellow
            Self::Weakness => 14,     // DarkMagenta
            Self::Invisibility => 1,  // Grey
            Self::Confusion => 12,    // DarkYellow
            Self::Focus => 7,         // Blue
            Self::Evasion => 9,       // Cyan
            Self::ArmorBreak => 4,    // DarkRed
            Self::Silence => 14,      // DarkMagenta
            Self::Slow => 10,         // DarkCyan
            Self::Vulnerable => 3,    // Red
            Self::Enrage => 3,        // Red
            Self::Curse => 14,        // DarkMagenta
            Self::Blessed => 11,      // Yellow
            Self::Vampiric => 4,      // DarkRed
            Self::Thorns => 5,        // Green
        }
    }

    /// Returns whether this is a harmful effect
    pub fn is_harmful(&self) -> bool {
        matches!(
            self,
            Self::Poison | Self::Burn | Self::Freeze | Self::Bleed |
            Self::Stun | Self::Blind | Self::Weakness | Self::Confusion |
            Self::ArmorBreak | Self::Silence | Self::Slow | Self::Vulnerable | Self::Curse
        )
    }

    /// Returns whether this is a beneficial effect
    pub fn is_beneficial(&self) -> bool {
        matches!(
            self,
            Self::Haste | Self::Shield | Self::Regeneration |
            Self::Strength | Self::Invisibility | Self::Focus |
            Self::Evasion | Self::Blessed | Self::Vampiric | Self::Thorns
        )
    }

    /// Returns damage per tick for damage-over-time effects
    pub fn damage_per_tick(&self) -> i32 {
        match self {
            Self::Poison => 2,
            Self::Burn => 3,
            Self::Bleed => 1,
            _ => 0,
        }
    }

    /// Returns the element type for elemental status effects
    pub fn element_type(&self) -> ElementType {
        match self {
            Self::Burn => ElementType::Fire,
            Self::Freeze => ElementType::Ice,
            Self::Poison => ElementType::Poison,
            _ => ElementType::Physical,
        }
    }

    /// Returns healing per tick for healing-over-time effects
    pub fn healing_per_tick(&self) -> i32 {
        match self {
            Self::Regeneration => 3,
            _ => 0,
        }
    }

    /// Returns the attack modifier for this effect (multiplier)
    pub fn attack_modifier(&self) -> f32 {
        match self {
            Self::Strength => 1.5,
            Self::Weakness => 0.6,
            Self::Enrage => 1.4,
            Self::Focus => 1.1,
            _ => 1.0,
        }
    }

    /// Returns the defense modifier for this effect (multiplier)
    pub fn defense_modifier(&self) -> f32 {
        match self {
            Self::Shield => 1.5,
            Self::ArmorBreak => 0.5,
            Self::Enrage => 0.7,
            Self::Vulnerable => 0.6,
            _ => 1.0,
        }
    }

    /// Returns the dodge chance modifier for this effect
    pub fn dodge_modifier(&self) -> f32 {
        match self {
            Self::Evasion => 0.25,
            Self::Haste => 0.15,
            Self::Freeze => -0.20,
            Self::Slow => -0.15,
            Self::Stun => -1.0, // Cannot dodge while stunned
            _ => 0.0,
        }
    }

    /// Returns the crit chance modifier for this effect
    pub fn crit_modifier(&self) -> f32 {
        match self {
            Self::Focus => 0.20,
            Self::Blessed => 0.15,
            Self::Curse => -0.15,
            Self::Blind => -0.20,
            _ => 0.0,
        }
    }

    /// Returns elemental resistance modifiers granted by this status effect
    pub fn elemental_resistance_bonus(&self) -> Option<(ElementType, f32)> {
        match self {
            Self::Burn => Some((ElementType::Ice, -0.25)),  // Burning makes you vulnerable to ice
            Self::Freeze => Some((ElementType::Fire, -0.25)), // Frozen makes you vulnerable to fire
            _ => None,
        }
    }
}

/// Result of a combat roll including crits and dodges
#[derive(Clone, Debug)]
pub struct CombatResult {
    /// Raw damage before defense
    pub raw_damage: i32,
    /// Final damage after all calculations
    pub final_damage: i32,
    /// Element type of the attack
    pub element: ElementType,
    /// Whether the attack was a critical hit
    pub is_critical: bool,
    /// Whether the attack was dodged
    pub is_dodged: bool,
    /// Whether the attack was blocked
    pub is_blocked: bool,
    /// Damage reflected back (thorns)
    pub reflected_damage: i32,
    /// Health stolen (vampiric)
    pub life_stolen: i32,
    /// Resistance applied (for display)
    pub resistance_applied: f32,
}

impl CombatResult {
    pub fn new(final_damage: i32) -> Self {
        Self {
            raw_damage: final_damage,
            final_damage,
            element: ElementType::Physical,
            is_critical: false,
            is_dodged: false,
            is_blocked: false,
            reflected_damage: 0,
            life_stolen: 0,
            resistance_applied: 0.0,
        }
    }

    pub fn new_elemental(final_damage: i32, element: ElementType) -> Self {
        Self {
            raw_damage: final_damage,
            final_damage,
            element,
            is_critical: false,
            is_dodged: false,
            is_blocked: false,
            reflected_damage: 0,
            life_stolen: 0,
            resistance_applied: 0.0,
        }
    }

    pub fn missed() -> Self {
        Self {
            raw_damage: 0,
            final_damage: 0,
            element: ElementType::Physical,
            is_critical: false,
            is_dodged: true,
            is_blocked: false,
            reflected_damage: 0,
            life_stolen: 0,
            resistance_applied: 0.0,
        }
    }

    pub fn critical(damage: i32, multiplier: f32) -> Self {
        let crit_damage = (damage as f32 * multiplier) as i32;
        Self {
            raw_damage: damage,
            final_damage: crit_damage,
            element: ElementType::Physical,
            is_critical: true,
            is_dodged: false,
            is_blocked: false,
            reflected_damage: 0,
            life_stolen: 0,
            resistance_applied: 0.0,
        }
    }

    pub fn with_element(mut self, element: ElementType) -> Self {
        self.element = element;
        self
    }

    pub fn with_resistance(mut self, resistance: f32, final_damage: i32) -> Self {
        self.resistance_applied = resistance;
        self.final_damage = final_damage;
        self
    }
}

/// Combat statistics for tracking combo attacks
#[derive(Clone, Debug, Default)]
pub struct ComboTracker {
    /// Current combo count
    pub combo_count: u32,
    /// Maximum combo achieved
    pub max_combo: u32,
    /// Turns since last hit (combo breaks after 2)
    pub turns_since_hit: u32,
}

impl ComboTracker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a successful hit
    pub fn hit(&mut self) {
        self.combo_count += 1;
        self.turns_since_hit = 0;
        if self.combo_count > self.max_combo {
            self.max_combo = self.combo_count;
        }
    }

    /// Called each turn - breaks combo if no hits
    pub fn tick(&mut self) {
        self.turns_since_hit += 1;
        if self.turns_since_hit >= 2 {
            self.combo_count = 0;
        }
    }

    /// Force break the combo (on miss or getting hit)
    pub fn break_combo(&mut self) {
        self.combo_count = 0;
        self.turns_since_hit = 0;
    }

    /// Get combo damage multiplier
    pub fn multiplier(&self) -> f32 {
        match self.combo_count {
            0..=2 => 1.0,
            3..=5 => 1.1,
            6..=9 => 1.2,
            10..=14 => 1.35,
            15..=19 => 1.5,
            20..=29 => 1.75,
            _ => 2.0,
        }
    }

    /// Get combo tier name for display
    pub fn tier_name(&self) -> &'static str {
        match self.combo_count {
            0..=2 => "",
            3..=5 => "Combo!",
            6..=9 => "Great Combo!",
            10..=14 => "Excellent!",
            15..=19 => "Incredible!",
            20..=29 => "Unstoppable!",
            _ => "LEGENDARY!",
        }
    }
}

/// Calculate damage after applying defense
pub fn calculate_damage(attack: i32, defense: i32) -> i32 {
    (attack - defense).max(1)
}

/// Calculate damage with strength modifier
pub fn calculate_damage_with_strength(base_attack: i32, has_strength: bool, has_weakness: bool) -> i32 {
    let mut attack = base_attack;
    if has_strength {
        attack = (attack as f32 * 1.5) as i32;
    }
    if has_weakness {
        attack = (attack as f32 * 0.5) as i32;
    }
    attack.max(1)
}

/// Roll for critical hit
pub fn roll_critical(crit_chance: f32) -> bool {
    use rand::Rng;
    rand::thread_rng().gen::<f32>() < crit_chance
}

/// Roll for dodge
pub fn roll_dodge(dodge_chance: f32) -> bool {
    use rand::Rng;
    rand::thread_rng().gen::<f32>() < dodge_chance
}

/// Full combat calculation with crits, dodges, combos, and elemental resistances
pub fn calculate_combat(
    base_attack: i32,
    defense: i32,
    crit_chance: f32,
    crit_multiplier: f32,
    dodge_chance: f32,
    combo_multiplier: f32,
    has_strength: bool,
    has_weakness: bool,
    target_has_vulnerable: bool,
    target_has_thorns: bool,
    attacker_has_vampiric: bool,
) -> CombatResult {
    calculate_combat_with_element(
        base_attack,
        defense,
        crit_chance,
        crit_multiplier,
        dodge_chance,
        combo_multiplier,
        has_strength,
        has_weakness,
        target_has_vulnerable,
        target_has_thorns,
        attacker_has_vampiric,
        ElementType::Physical,
        &ElementalResistances::default(),
    )
}

/// Full combat calculation with elemental damage and resistances
pub fn calculate_combat_with_element(
    base_attack: i32,
    defense: i32,
    crit_chance: f32,
    crit_multiplier: f32,
    dodge_chance: f32,
    combo_multiplier: f32,
    has_strength: bool,
    has_weakness: bool,
    target_has_vulnerable: bool,
    target_has_thorns: bool,
    attacker_has_vampiric: bool,
    element: ElementType,
    target_resistances: &ElementalResistances,
) -> CombatResult {
    // Check dodge first
    if roll_dodge(dodge_chance) {
        return CombatResult::missed();
    }

    // Calculate base attack with status modifiers
    let modified_attack = calculate_damage_with_strength(base_attack, has_strength, has_weakness);

    // Apply combo multiplier
    let combo_attack = (modified_attack as f32 * combo_multiplier) as i32;

    // Check for critical hit
    let is_crit = roll_critical(crit_chance);
    let attack_after_crit = if is_crit {
        (combo_attack as f32 * crit_multiplier) as i32
    } else {
        combo_attack
    };

    // Apply defense (elemental damage partially bypasses armor)
    let effective_defense = if element == ElementType::Physical {
        defense
    } else {
        defense / 4 // Elemental damage ignores 75% of armor
    };
    let mut damage = calculate_damage(attack_after_crit, effective_defense);

    // Apply vulnerability
    if target_has_vulnerable {
        damage = (damage as f32 * 1.25) as i32;
    }

    // Apply elemental resistance
    let resistance = target_resistances.get(element);
    let damage_before_resistance = damage;
    damage = calculate_elemental_damage(damage, element, target_resistances);

    // Calculate reflected damage (thorns)
    let reflected = if target_has_thorns {
        (damage as f32 * 0.2) as i32
    } else {
        0
    };

    // Calculate life steal
    let stolen = if attacker_has_vampiric {
        (damage as f32 * 0.2) as i32
    } else {
        0
    };

    CombatResult {
        raw_damage: base_attack,
        final_damage: damage,
        element,
        is_critical: is_crit,
        is_dodged: false,
        is_blocked: false,
        reflected_damage: reflected,
        life_stolen: stolen,
        resistance_applied: resistance,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_damage() {
        assert_eq!(calculate_damage(5, 100), 1);
        assert_eq!(calculate_damage(0, 10), 1);
    }

    #[test]
    fn test_normal_damage() {
        assert_eq!(calculate_damage(10, 3), 7);
        assert_eq!(calculate_damage(20, 5), 15);
    }

    #[test]
    fn test_strength_modifier() {
        let base = 10;
        assert_eq!(calculate_damage_with_strength(base, true, false), 15);
        assert_eq!(calculate_damage_with_strength(base, false, true), 5);
        assert_eq!(calculate_damage_with_strength(base, true, true), 7); // 10 * 1.5 * 0.5 = 7.5 -> 7
    }

    #[test]
    fn test_combo_tracker() {
        let mut combo = ComboTracker::new();
        assert_eq!(combo.multiplier(), 1.0);

        // Build up combo
        for _ in 0..5 {
            combo.hit();
        }
        assert_eq!(combo.combo_count, 5);
        assert_eq!(combo.multiplier(), 1.1);

        // Break combo
        combo.break_combo();
        assert_eq!(combo.combo_count, 0);
        assert_eq!(combo.multiplier(), 1.0);
    }

    #[test]
    fn test_combo_tiers() {
        let mut combo = ComboTracker::new();
        assert_eq!(combo.tier_name(), "");

        for _ in 0..3 {
            combo.hit();
        }
        assert_eq!(combo.tier_name(), "Combo!");

        for _ in 0..7 {
            combo.hit();
        }
        assert_eq!(combo.tier_name(), "Excellent!");
    }

    #[test]
    fn test_status_effect_modifiers() {
        assert_eq!(StatusEffect::Strength.attack_modifier(), 1.5);
        assert_eq!(StatusEffect::Weakness.attack_modifier(), 0.6);
        assert_eq!(StatusEffect::Shield.defense_modifier(), 1.5);
        assert_eq!(StatusEffect::Evasion.dodge_modifier(), 0.25);
        assert_eq!(StatusEffect::Focus.crit_modifier(), 0.20);
    }

    #[test]
    fn test_combat_result() {
        let result = CombatResult::new(10);
        assert_eq!(result.final_damage, 10);
        assert!(!result.is_critical);
        assert!(!result.is_dodged);

        let missed = CombatResult::missed();
        assert!(missed.is_dodged);
        assert_eq!(missed.final_damage, 0);

        let crit = CombatResult::critical(10, 2.0);
        assert!(crit.is_critical);
        assert_eq!(crit.final_damage, 20);
    }

    #[test]
    fn test_new_status_effects() {
        // Test new status effects exist and have proper properties
        assert_eq!(StatusEffect::Focus.name(), "Focused");
        assert_eq!(StatusEffect::Evasion.name(), "Evasive");
        assert_eq!(StatusEffect::ArmorBreak.name(), "Armor Broken");
        assert_eq!(StatusEffect::Vampiric.name(), "Vampiric");
        assert_eq!(StatusEffect::Thorns.name(), "Thorns");

        // Test harmful/beneficial classification
        assert!(StatusEffect::ArmorBreak.is_harmful());
        assert!(StatusEffect::Curse.is_harmful());
        assert!(StatusEffect::Focus.is_beneficial());
        assert!(StatusEffect::Blessed.is_beneficial());
    }

    #[test]
    fn test_elemental_resistances_default() {
        let res = ElementalResistances::default();
        assert_eq!(res.fire, 0.0);
        assert_eq!(res.ice, 0.0);
        assert_eq!(res.lightning, 0.0);
        assert_eq!(res.poison, 0.0);
    }

    #[test]
    fn test_elemental_resistances_new() {
        let res = ElementalResistances::new(0.5, 0.25, -0.25, 1.0);
        assert_eq!(res.fire, 0.5);
        assert_eq!(res.ice, 0.25);
        assert_eq!(res.lightning, -0.25);
        assert_eq!(res.poison, 1.0);
    }

    #[test]
    fn test_elemental_resistance_clamping() {
        let res = ElementalResistances::new(2.0, -2.0, 0.5, 0.5);
        assert_eq!(res.fire, 1.0);  // Clamped from 2.0
        assert_eq!(res.ice, -1.0);   // Clamped from -2.0
    }

    #[test]
    fn test_damage_multiplier() {
        let res = ElementalResistances::new(0.5, 0.0, -0.5, 1.0);

        assert_eq!(res.damage_multiplier(ElementType::Fire), 0.5);      // 50% resistance = 50% damage
        assert_eq!(res.damage_multiplier(ElementType::Ice), 1.0);       // 0% resistance = 100% damage
        assert_eq!(res.damage_multiplier(ElementType::Lightning), 1.5); // -50% resistance = 150% damage
        assert_eq!(res.damage_multiplier(ElementType::Poison), 0.0);    // 100% resistance = immune
    }

    #[test]
    fn test_calculate_elemental_damage() {
        let res = ElementalResistances::new(0.5, 0.0, -0.5, 1.0);

        assert_eq!(calculate_elemental_damage(100, ElementType::Fire, &res), 50);
        assert_eq!(calculate_elemental_damage(100, ElementType::Ice, &res), 100);
        assert_eq!(calculate_elemental_damage(100, ElementType::Lightning, &res), 150);
        assert_eq!(calculate_elemental_damage(100, ElementType::Poison, &res), 0);
        assert_eq!(calculate_elemental_damage(100, ElementType::Physical, &res), 100);
    }

    #[test]
    fn test_is_immune() {
        let res = ElementalResistances::new(1.0, 0.5, 0.0, 0.99);
        assert!(res.is_immune(ElementType::Fire));
        assert!(!res.is_immune(ElementType::Ice));
        assert!(!res.is_immune(ElementType::Poison));
    }

    #[test]
    fn test_is_vulnerable() {
        let res = ElementalResistances::new(-0.5, 0.0, 0.5, -0.1);
        assert!(res.is_vulnerable(ElementType::Fire));
        assert!(!res.is_vulnerable(ElementType::Ice));
        assert!(!res.is_vulnerable(ElementType::Lightning));
        assert!(res.is_vulnerable(ElementType::Poison));
    }

    #[test]
    fn test_element_type_names() {
        assert_eq!(ElementType::Physical.name(), "Physical");
        assert_eq!(ElementType::Fire.name(), "Fire");
        assert_eq!(ElementType::Ice.name(), "Ice");
        assert_eq!(ElementType::Lightning.name(), "Lightning");
        assert_eq!(ElementType::Poison.name(), "Poison");
    }

    #[test]
    fn test_element_status_effects() {
        assert_eq!(ElementType::Fire.status_effect(), Some(StatusEffect::Burn));
        assert_eq!(ElementType::Ice.status_effect(), Some(StatusEffect::Freeze));
        assert_eq!(ElementType::Lightning.status_effect(), Some(StatusEffect::Stun));
        assert_eq!(ElementType::Poison.status_effect(), Some(StatusEffect::Poison));
        assert_eq!(ElementType::Physical.status_effect(), None);
    }

    #[test]
    fn test_status_effect_element_type() {
        assert_eq!(StatusEffect::Burn.element_type(), ElementType::Fire);
        assert_eq!(StatusEffect::Freeze.element_type(), ElementType::Ice);
        assert_eq!(StatusEffect::Poison.element_type(), ElementType::Poison);
        assert_eq!(StatusEffect::Bleed.element_type(), ElementType::Physical);
    }

    #[test]
    fn test_resistances_add() {
        let mut res1 = ElementalResistances::new(0.3, 0.2, 0.1, 0.0);
        let res2 = ElementalResistances::new(0.2, 0.3, 0.4, 0.5);
        res1.add(&res2);

        assert_eq!(res1.fire, 0.5);
        assert_eq!(res1.ice, 0.5);
        assert_eq!(res1.lightning, 0.5);
        assert_eq!(res1.poison, 0.5);
    }

    #[test]
    fn test_resistances_combine_max() {
        let res1 = ElementalResistances::new(0.5, 0.2, 0.8, 0.1);
        let res2 = ElementalResistances::new(0.3, 0.6, 0.4, 0.9);
        let combined = res1.combine_max(&res2);

        assert_eq!(combined.fire, 0.5);
        assert_eq!(combined.ice, 0.6);
        assert_eq!(combined.lightning, 0.8);
        assert_eq!(combined.poison, 0.9);
    }
}
