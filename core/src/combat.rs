//! Combat system: status effects, critical hits, dodge mechanics, and combat calculations

use serde::{Serialize, Deserialize};

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
}

/// Result of a combat roll including crits and dodges
#[derive(Clone, Debug)]
pub struct CombatResult {
    /// Raw damage before defense
    pub raw_damage: i32,
    /// Final damage after all calculations
    pub final_damage: i32,
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
}

impl CombatResult {
    pub fn new(final_damage: i32) -> Self {
        Self {
            raw_damage: final_damage,
            final_damage,
            is_critical: false,
            is_dodged: false,
            is_blocked: false,
            reflected_damage: 0,
            life_stolen: 0,
        }
    }

    pub fn missed() -> Self {
        Self {
            raw_damage: 0,
            final_damage: 0,
            is_critical: false,
            is_dodged: true,
            is_blocked: false,
            reflected_damage: 0,
            life_stolen: 0,
        }
    }

    pub fn critical(damage: i32, multiplier: f32) -> Self {
        let crit_damage = (damage as f32 * multiplier) as i32;
        Self {
            raw_damage: damage,
            final_damage: crit_damage,
            is_critical: true,
            is_dodged: false,
            is_blocked: false,
            reflected_damage: 0,
            life_stolen: 0,
        }
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

/// Full combat calculation with crits, dodges, and combos
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

    // Apply defense
    let mut damage = calculate_damage(attack_after_crit, defense);

    // Apply vulnerability
    if target_has_vulnerable {
        damage = (damage as f32 * 1.25) as i32;
    }

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
        is_critical: is_crit,
        is_dodged: false,
        is_blocked: false,
        reflected_damage: reflected,
        life_stolen: stolen,
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
}
