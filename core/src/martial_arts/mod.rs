//! Martial Arts System
//!
//! A comprehensive martial arts system featuring 30+ styles, chi/ki mechanics,
//! combo systems, training methods, masters, and dojos.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// CONSTANTS
// ============================================================================

/// Maximum chi points a character can have
pub const MAX_CHI_POINTS: u32 = 1000;

/// Base chi regeneration per turn
pub const BASE_CHI_REGEN: u32 = 5;

/// Maximum mastery level for a style
pub const MAX_MASTERY_LEVEL: u32 = 100;

/// Experience needed per mastery level
pub const XP_PER_MASTERY_LEVEL: u32 = 500;

/// Maximum combo length
pub const MAX_COMBO_LENGTH: usize = 10;

/// Tournament rounds to win
pub const TOURNAMENT_ROUNDS: u32 = 5;

// ============================================================================
// MARTIAL ARTS STYLE CATEGORIES
// ============================================================================

/// Category of martial arts style
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StyleCategory {
    /// Northern styles - long range, kicks
    Northern,
    /// Southern styles - close range, hands
    Southern,
    /// Internal styles - chi focused
    Internal,
    /// External styles - physical power
    External,
    /// Weapon-based styles
    Weapon,
    /// Legendary mythical styles
    Legendary,
}

impl StyleCategory {
    pub fn all() -> &'static [StyleCategory] {
        &[
            Self::Northern,
            Self::Southern,
            Self::Internal,
            Self::External,
            Self::Weapon,
            Self::Legendary,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Northern => "Northern Style",
            Self::Southern => "Southern Style",
            Self::Internal => "Internal Style",
            Self::External => "External Style",
            Self::Weapon => "Weapon Style",
            Self::Legendary => "Legendary Style",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Northern => "Emphasizes long-range attacks, powerful kicks, and fluid movement.",
            Self::Southern => "Focuses on close-range combat, strong hand techniques, and stable stances.",
            Self::Internal => "Cultivates internal energy (chi), emphasizing breathing and meditation.",
            Self::External => "Develops external strength, speed, and conditioning through rigorous training.",
            Self::Weapon => "Masters traditional weapons through precise forms and combat techniques.",
            Self::Legendary => "Ancient mythical styles passed down by legendary masters.",
        }
    }

    pub fn primary_stat(&self) -> &'static str {
        match self {
            Self::Northern => "Agility",
            Self::Southern => "Strength",
            Self::Internal => "Chi",
            Self::External => "Stamina",
            Self::Weapon => "Precision",
            Self::Legendary => "Spirit",
        }
    }
}

// ============================================================================
// MARTIAL ARTS STYLES (36 total)
// ============================================================================

/// All martial arts styles available in the game
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MartialStyle {
    // Northern Styles (6)
    NorthernPrayingMantis,
    EagleClaw,
    LongFist,
    NorthernShaolin,
    FanziQuan,
    ChaqQuan,

    // Southern Styles (6)
    WingChun,
    HungGar,
    WhiteCrane,
    ChoyLiFut,
    SouthernPrayingMantis,
    DragonStyle,

    // Internal Styles (6)
    TaiChi,
    BaguaZhang,
    XingYiQuan,
    LiuheiBafaQuan,
    YiQuan,
    WudangQuan,

    // External Styles (6)
    ShaolinKungFu,
    Sanda,
    ChangQuan,
    NanQuan,
    TongBeiQuan,
    PiguaQuan,

    // Weapon Styles (6)
    JianShu,
    DaoShu,
    GunShu,
    QiangShu,
    ShuangDao,
    SanJieGun,

    // Legendary Styles (6)
    DragonFist,
    TigerClaw,
    CraneWing,
    SnakeStrike,
    LeopardPaw,
    PhoenixTalon,
}

impl MartialStyle {
    pub fn all() -> &'static [MartialStyle] {
        &[
            // Northern
            Self::NorthernPrayingMantis, Self::EagleClaw, Self::LongFist,
            Self::NorthernShaolin, Self::FanziQuan, Self::ChaqQuan,
            // Southern
            Self::WingChun, Self::HungGar, Self::WhiteCrane,
            Self::ChoyLiFut, Self::SouthernPrayingMantis, Self::DragonStyle,
            // Internal
            Self::TaiChi, Self::BaguaZhang, Self::XingYiQuan,
            Self::LiuheiBafaQuan, Self::YiQuan, Self::WudangQuan,
            // External
            Self::ShaolinKungFu, Self::Sanda, Self::ChangQuan,
            Self::NanQuan, Self::TongBeiQuan, Self::PiguaQuan,
            // Weapon
            Self::JianShu, Self::DaoShu, Self::GunShu,
            Self::QiangShu, Self::ShuangDao, Self::SanJieGun,
            // Legendary
            Self::DragonFist, Self::TigerClaw, Self::CraneWing,
            Self::SnakeStrike, Self::LeopardPaw, Self::PhoenixTalon,
        ]
    }

    pub fn by_category(category: StyleCategory) -> Vec<MartialStyle> {
        Self::all()
            .iter()
            .filter(|s| s.category() == category)
            .copied()
            .collect()
    }

    pub fn category(&self) -> StyleCategory {
        match self {
            Self::NorthernPrayingMantis | Self::EagleClaw | Self::LongFist |
            Self::NorthernShaolin | Self::FanziQuan | Self::ChaqQuan => StyleCategory::Northern,

            Self::WingChun | Self::HungGar | Self::WhiteCrane |
            Self::ChoyLiFut | Self::SouthernPrayingMantis | Self::DragonStyle => StyleCategory::Southern,

            Self::TaiChi | Self::BaguaZhang | Self::XingYiQuan |
            Self::LiuheiBafaQuan | Self::YiQuan | Self::WudangQuan => StyleCategory::Internal,

            Self::ShaolinKungFu | Self::Sanda | Self::ChangQuan |
            Self::NanQuan | Self::TongBeiQuan | Self::PiguaQuan => StyleCategory::External,

            Self::JianShu | Self::DaoShu | Self::GunShu |
            Self::QiangShu | Self::ShuangDao | Self::SanJieGun => StyleCategory::Weapon,

            Self::DragonFist | Self::TigerClaw | Self::CraneWing |
            Self::SnakeStrike | Self::LeopardPaw | Self::PhoenixTalon => StyleCategory::Legendary,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            // Northern
            Self::NorthernPrayingMantis => "Northern Praying Mantis",
            Self::EagleClaw => "Eagle Claw",
            Self::LongFist => "Long Fist",
            Self::NorthernShaolin => "Northern Shaolin",
            Self::FanziQuan => "Fanzi Quan (Tumbling Fist)",
            Self::ChaqQuan => "Cha Quan",
            // Southern
            Self::WingChun => "Wing Chun",
            Self::HungGar => "Hung Gar",
            Self::WhiteCrane => "White Crane",
            Self::ChoyLiFut => "Choy Li Fut",
            Self::SouthernPrayingMantis => "Southern Praying Mantis",
            Self::DragonStyle => "Dragon Style",
            // Internal
            Self::TaiChi => "Tai Chi Chuan",
            Self::BaguaZhang => "Bagua Zhang",
            Self::XingYiQuan => "Xing Yi Quan",
            Self::LiuheiBafaQuan => "Liuhei Bafa Quan",
            Self::YiQuan => "Yi Quan",
            Self::WudangQuan => "Wudang Quan",
            // External
            Self::ShaolinKungFu => "Shaolin Kung Fu",
            Self::Sanda => "Sanda (Chinese Kickboxing)",
            Self::ChangQuan => "Chang Quan",
            Self::NanQuan => "Nan Quan",
            Self::TongBeiQuan => "Tong Bei Quan",
            Self::PiguaQuan => "Pigua Quan",
            // Weapon
            Self::JianShu => "Jian Shu (Straight Sword)",
            Self::DaoShu => "Dao Shu (Broadsword)",
            Self::GunShu => "Gun Shu (Staff)",
            Self::QiangShu => "Qiang Shu (Spear)",
            Self::ShuangDao => "Shuang Dao (Dual Blades)",
            Self::SanJieGun => "San Jie Gun (Three-Section Staff)",
            // Legendary
            Self::DragonFist => "Dragon Fist",
            Self::TigerClaw => "Tiger Claw",
            Self::CraneWing => "Crane Wing",
            Self::SnakeStrike => "Snake Strike",
            Self::LeopardPaw => "Leopard Paw",
            Self::PhoenixTalon => "Phoenix Talon",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::NorthernPrayingMantis => "Mimics the movements of the praying mantis with hooking hands and quick strikes.",
            Self::EagleClaw => "Focuses on gripping techniques and pressure point attacks like an eagle's talons.",
            Self::LongFist => "Emphasizes extended, long-range strikes with fully extended limbs.",
            Self::NorthernShaolin => "Traditional Shaolin techniques adapted for the northern terrain.",
            Self::FanziQuan => "Known for rapid, continuous strikes resembling tumbling movements.",
            Self::ChaqQuan => "Combines long-range techniques with quick footwork and spinning attacks.",
            Self::WingChun => "Close-range combat emphasizing simultaneous defense and attack.",
            Self::HungGar => "Powerful low stances and strong hand techniques inspired by the tiger and crane.",
            Self::WhiteCrane => "Graceful evasive movements combined with precise pecking strikes.",
            Self::ChoyLiFut => "Combines northern and southern techniques with circular movements.",
            Self::SouthernPrayingMantis => "Close-range mantis techniques with emphasis on sticky hands.",
            Self::DragonStyle => "Fluid, undulating movements that flow like a mythical dragon.",
            Self::TaiChi => "Slow, meditative movements that cultivate internal energy and balance.",
            Self::BaguaZhang => "Circle walking and palm techniques based on the eight trigrams.",
            Self::XingYiQuan => "Direct, linear movements based on the five elements.",
            Self::LiuheiBafaQuan => "Combines internal principles of multiple styles into one.",
            Self::YiQuan => "Standing meditation and spontaneous movement cultivation.",
            Self::WudangQuan => "Taoist martial arts emphasizing softness and internal cultivation.",
            Self::ShaolinKungFu => "The legendary temple style combining Buddhist philosophy with combat.",
            Self::Sanda => "Modern combat sport combining traditional techniques with kickboxing.",
            Self::ChangQuan => "Long-range style known for its acrobatic kicks and stances.",
            Self::NanQuan => "Southern fist style with low stances and powerful arm techniques.",
            Self::TongBeiQuan => "Whipping arm techniques that generate power through the back.",
            Self::PiguaQuan => "Chopping and swinging palm techniques with explosive power.",
            Self::JianShu => "The art of the straight double-edged sword.",
            Self::DaoShu => "Techniques of the single-edged broadsword.",
            Self::GunShu => "Staff techniques ranging from sweeps to thrusts.",
            Self::QiangShu => "The king of weapons - spear techniques for offense and defense.",
            Self::ShuangDao => "Dual wielding blade techniques for overwhelming attacks.",
            Self::SanJieGun => "Complex techniques with the segmented three-section staff.",
            Self::DragonFist => "Legendary style channeling the power of ancient dragons.",
            Self::TigerClaw => "Ferocious ripping and tearing techniques of the tiger.",
            Self::CraneWing => "Elegant blocking and counter-striking like a crane's wings.",
            Self::SnakeStrike => "Lightning-fast precision strikes targeting vital points.",
            Self::LeopardPaw => "Speed and power combined in rapid successive attacks.",
            Self::PhoenixTalon => "Mythical style of rebirth - stronger after each defeat.",
        }
    }

    pub fn base_damage(&self) -> i32 {
        match self.category() {
            StyleCategory::Northern => 12,
            StyleCategory::Southern => 15,
            StyleCategory::Internal => 8,
            StyleCategory::External => 18,
            StyleCategory::Weapon => 20,
            StyleCategory::Legendary => 25,
        }
    }

    pub fn chi_cost(&self) -> u32 {
        match self.category() {
            StyleCategory::Northern => 10,
            StyleCategory::Southern => 8,
            StyleCategory::Internal => 5,
            StyleCategory::External => 15,
            StyleCategory::Weapon => 12,
            StyleCategory::Legendary => 20,
        }
    }

    pub fn range(&self) -> AttackRange {
        match self.category() {
            StyleCategory::Northern => AttackRange::Long,
            StyleCategory::Southern => AttackRange::Close,
            StyleCategory::Internal => AttackRange::Medium,
            StyleCategory::External => AttackRange::Medium,
            StyleCategory::Weapon => AttackRange::Extended,
            StyleCategory::Legendary => AttackRange::Variable,
        }
    }

    pub fn learning_difficulty(&self) -> Difficulty {
        match self {
            Self::LongFist | Self::Sanda | Self::GunShu => Difficulty::Beginner,
            Self::WingChun | Self::HungGar | Self::ShaolinKungFu |
            Self::ChangQuan | Self::DaoShu | Self::QiangShu => Difficulty::Intermediate,
            Self::NorthernPrayingMantis | Self::EagleClaw | Self::BaguaZhang |
            Self::XingYiQuan | Self::JianShu | Self::ShuangDao => Difficulty::Advanced,
            Self::TaiChi | Self::WhiteCrane | Self::DragonStyle |
            Self::WudangQuan | Self::SanJieGun => Difficulty::Expert,
            Self::DragonFist | Self::TigerClaw | Self::CraneWing |
            Self::SnakeStrike | Self::LeopardPaw | Self::PhoenixTalon => Difficulty::Legendary,
            _ => Difficulty::Intermediate,
        }
    }

    pub fn signature_technique(&self) -> &'static str {
        match self {
            Self::NorthernPrayingMantis => "Mantis Hook",
            Self::EagleClaw => "Iron Talon Grip",
            Self::LongFist => "Cannon Punch",
            Self::NorthernShaolin => "Buddha Palm",
            Self::FanziQuan => "Rolling Thunder",
            Self::ChaqQuan => "Whirlwind Kick",
            Self::WingChun => "Chain Punch",
            Self::HungGar => "Tiger-Crane Double Form",
            Self::WhiteCrane => "Crane Spreads Wings",
            Self::ChoyLiFut => "Circular Bridge",
            Self::SouthernPrayingMantis => "Sticky Bridge",
            Self::DragonStyle => "Dragon Coils",
            Self::TaiChi => "Push Hands",
            Self::BaguaZhang => "Circle Walking Palm",
            Self::XingYiQuan => "Five Element Fist",
            Self::LiuheiBafaQuan => "Six Harmonies Strike",
            Self::YiQuan => "Standing Post",
            Self::WudangQuan => "Taoist Palm",
            Self::ShaolinKungFu => "Iron Shirt",
            Self::Sanda => "Takedown Sweep",
            Self::ChangQuan => "Butterfly Kick",
            Self::NanQuan => "Bridge Breaking Punch",
            Self::TongBeiQuan => "Back Whip Strike",
            Self::PiguaQuan => "Splitting Chop",
            Self::JianShu => "Point Thrust",
            Self::DaoShu => "Tiger Head Slash",
            Self::GunShu => "Monkey Steals Peach",
            Self::QiangShu => "Dragon Spear Dance",
            Self::ShuangDao => "Cross Slash",
            Self::SanJieGun => "Triple Coiling Strike",
            Self::DragonFist => "Azure Dragon Ascends",
            Self::TigerClaw => "Descending Tiger",
            Self::CraneWing => "White Crane Spreads Wings",
            Self::SnakeStrike => "Viper's Kiss",
            Self::LeopardPaw => "Leopard's Rush",
            Self::PhoenixTalon => "Phoenix Rising",
        }
    }

    pub fn required_weapon(&self) -> Option<WeaponType> {
        match self {
            Self::JianShu => Some(WeaponType::StraightSword),
            Self::DaoShu => Some(WeaponType::Broadsword),
            Self::GunShu => Some(WeaponType::Staff),
            Self::QiangShu => Some(WeaponType::Spear),
            Self::ShuangDao => Some(WeaponType::DualBlades),
            Self::SanJieGun => Some(WeaponType::ThreeSectionStaff),
            _ => None,
        }
    }
}

// ============================================================================
// ATTACK RANGE AND DIFFICULTY
// ============================================================================

/// Attack range types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttackRange {
    Close,
    Medium,
    Long,
    Extended,
    Variable,
}

impl AttackRange {
    pub fn tiles(&self) -> u32 {
        match self {
            Self::Close => 1,
            Self::Medium => 2,
            Self::Long => 3,
            Self::Extended => 4,
            Self::Variable => 3,
        }
    }
}

/// Difficulty levels
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Difficulty {
    Beginner = 0,
    Intermediate = 1,
    Advanced = 2,
    Expert = 3,
    Legendary = 4,
}

impl Difficulty {
    pub fn xp_multiplier(&self) -> f32 {
        match self {
            Self::Beginner => 1.0,
            Self::Intermediate => 1.5,
            Self::Advanced => 2.0,
            Self::Expert => 3.0,
            Self::Legendary => 5.0,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Beginner => "Beginner",
            Self::Intermediate => "Intermediate",
            Self::Advanced => "Advanced",
            Self::Expert => "Expert",
            Self::Legendary => "Legendary",
        }
    }
}

// ============================================================================
// WEAPON TYPES
// ============================================================================

/// Weapon types for martial arts
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponType {
    StraightSword,
    Broadsword,
    Staff,
    Spear,
    DualBlades,
    ThreeSectionStaff,
    Nunchaku,
    HookSwords,
    ChainWhip,
    Halberd,
    FlyingGuillotine,
    IronFan,
}

impl WeaponType {
    pub fn all() -> &'static [WeaponType] {
        &[
            Self::StraightSword, Self::Broadsword, Self::Staff,
            Self::Spear, Self::DualBlades, Self::ThreeSectionStaff,
            Self::Nunchaku, Self::HookSwords, Self::ChainWhip,
            Self::Halberd, Self::FlyingGuillotine, Self::IronFan,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::StraightSword => "Jian (Straight Sword)",
            Self::Broadsword => "Dao (Broadsword)",
            Self::Staff => "Gun (Staff)",
            Self::Spear => "Qiang (Spear)",
            Self::DualBlades => "Shuang Dao (Dual Blades)",
            Self::ThreeSectionStaff => "San Jie Gun (Three-Section Staff)",
            Self::Nunchaku => "Shuang Jie Gun (Nunchaku)",
            Self::HookSwords => "Gou (Hook Swords)",
            Self::ChainWhip => "Jiu Jie Bian (Chain Whip)",
            Self::Halberd => "Ji (Halberd)",
            Self::FlyingGuillotine => "Xue Di Zi (Flying Guillotine)",
            Self::IronFan => "Tie Shan (Iron Fan)",
        }
    }

    pub fn damage_bonus(&self) -> i32 {
        match self {
            Self::StraightSword => 8,
            Self::Broadsword => 12,
            Self::Staff => 6,
            Self::Spear => 15,
            Self::DualBlades => 10,
            Self::ThreeSectionStaff => 9,
            Self::Nunchaku => 7,
            Self::HookSwords => 11,
            Self::ChainWhip => 8,
            Self::Halberd => 18,
            Self::FlyingGuillotine => 20,
            Self::IronFan => 5,
        }
    }

    pub fn reach(&self) -> u32 {
        match self {
            Self::StraightSword => 2,
            Self::Broadsword => 2,
            Self::Staff => 3,
            Self::Spear => 4,
            Self::DualBlades => 1,
            Self::ThreeSectionStaff => 3,
            Self::Nunchaku => 2,
            Self::HookSwords => 2,
            Self::ChainWhip => 4,
            Self::Halberd => 3,
            Self::FlyingGuillotine => 5,
            Self::IronFan => 1,
        }
    }
}

// ============================================================================
// CHI/KI SYSTEM - MERIDIANS
// ============================================================================

/// Meridian pathways for chi flow
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Meridian {
    /// Lung meridian - breathing and defense
    Lung,
    /// Heart meridian - spirit and courage
    Heart,
    /// Liver meridian - flexibility and anger
    Liver,
    /// Spleen meridian - stability and worry
    Spleen,
    /// Kidney meridian - vitality and fear
    Kidney,
    /// Pericardium meridian - protection
    Pericardium,
    /// Triple Burner - metabolism
    TripleBurner,
    /// Gallbladder meridian - decision and action
    Gallbladder,
    /// Stomach meridian - nourishment
    Stomach,
    /// Small Intestine meridian - absorption
    SmallIntestine,
    /// Large Intestine meridian - release
    LargeIntestine,
    /// Bladder meridian - purification
    Bladder,
    /// Governing Vessel - yang energy
    GoverningVessel,
    /// Conception Vessel - yin energy
    ConceptionVessel,
}

impl Meridian {
    pub fn all() -> &'static [Meridian] {
        &[
            Self::Lung, Self::Heart, Self::Liver, Self::Spleen,
            Self::Kidney, Self::Pericardium, Self::TripleBurner,
            Self::Gallbladder, Self::Stomach, Self::SmallIntestine,
            Self::LargeIntestine, Self::Bladder, Self::GoverningVessel,
            Self::ConceptionVessel,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Lung => "Lung Meridian (Shou Taiyin)",
            Self::Heart => "Heart Meridian (Shou Shaoyin)",
            Self::Liver => "Liver Meridian (Zu Jueyin)",
            Self::Spleen => "Spleen Meridian (Zu Taiyin)",
            Self::Kidney => "Kidney Meridian (Zu Shaoyin)",
            Self::Pericardium => "Pericardium Meridian (Shou Jueyin)",
            Self::TripleBurner => "Triple Burner (Shou Shaoyang)",
            Self::Gallbladder => "Gallbladder Meridian (Zu Shaoyang)",
            Self::Stomach => "Stomach Meridian (Zu Yangming)",
            Self::SmallIntestine => "Small Intestine (Shou Taiyang)",
            Self::LargeIntestine => "Large Intestine (Shou Yangming)",
            Self::Bladder => "Bladder Meridian (Zu Taiyang)",
            Self::GoverningVessel => "Governing Vessel (Du Mai)",
            Self::ConceptionVessel => "Conception Vessel (Ren Mai)",
        }
    }

    pub fn associated_element(&self) -> Element {
        match self {
            Self::Lung | Self::LargeIntestine => Element::Metal,
            Self::Heart | Self::SmallIntestine | Self::Pericardium | Self::TripleBurner => Element::Fire,
            Self::Liver | Self::Gallbladder => Element::Wood,
            Self::Spleen | Self::Stomach => Element::Earth,
            Self::Kidney | Self::Bladder => Element::Water,
            Self::GoverningVessel | Self::ConceptionVessel => Element::Void,
        }
    }

    pub fn chi_capacity_bonus(&self) -> u32 {
        match self {
            Self::Kidney | Self::Heart => 50,
            Self::Liver | Self::Spleen | Self::Lung => 30,
            Self::GoverningVessel | Self::ConceptionVessel => 100,
            _ => 20,
        }
    }
}

/// Five elements plus void
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Element {
    Wood,
    Fire,
    Earth,
    Metal,
    Water,
    Void,
}

impl Element {
    pub fn all() -> &'static [Element] {
        &[Self::Wood, Self::Fire, Self::Earth, Self::Metal, Self::Water, Self::Void]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Wood => "Wood (Mu)",
            Self::Fire => "Fire (Huo)",
            Self::Earth => "Earth (Tu)",
            Self::Metal => "Metal (Jin)",
            Self::Water => "Water (Shui)",
            Self::Void => "Void (Kong)",
        }
    }

    /// Returns the element this one overcomes
    pub fn overcomes(&self) -> Element {
        match self {
            Self::Wood => Self::Earth,
            Self::Fire => Self::Metal,
            Self::Earth => Self::Water,
            Self::Metal => Self::Wood,
            Self::Water => Self::Fire,
            Self::Void => Self::Void,
        }
    }

    /// Returns the element that generates this one
    pub fn generated_by(&self) -> Element {
        match self {
            Self::Wood => Self::Water,
            Self::Fire => Self::Wood,
            Self::Earth => Self::Fire,
            Self::Metal => Self::Earth,
            Self::Water => Self::Metal,
            Self::Void => Self::Void,
        }
    }

    pub fn damage_multiplier(&self, target_element: Element) -> f32 {
        if self.overcomes() == target_element {
            1.5
        } else if target_element.overcomes() == *self {
            0.5
        } else {
            1.0
        }
    }
}

// ============================================================================
// CHI/KI SYSTEM - PRESSURE POINTS
// ============================================================================

/// Pressure points on the body
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PressurePoint {
    /// Baihui - top of head
    Baihui,
    /// Yintang - third eye
    Yintang,
    /// Tiantu - throat
    Tiantu,
    /// Tanzhong - chest center
    Tanzhong,
    /// Zhongwan - solar plexus
    Zhongwan,
    /// Qihai - lower abdomen (dantian)
    Qihai,
    /// Jianjing - shoulder well
    Jianjing,
    /// Quchi - elbow
    Quchi,
    /// Hegu - hand
    Hegu,
    /// Huantiao - hip
    Huantiao,
    /// Zusanli - below knee
    Zusanli,
    /// Yongquan - foot sole
    Yongquan,
}

impl PressurePoint {
    pub fn all() -> &'static [PressurePoint] {
        &[
            Self::Baihui, Self::Yintang, Self::Tiantu, Self::Tanzhong,
            Self::Zhongwan, Self::Qihai, Self::Jianjing, Self::Quchi,
            Self::Hegu, Self::Huantiao, Self::Zusanli, Self::Yongquan,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Baihui => "Baihui (Hundred Meetings)",
            Self::Yintang => "Yintang (Third Eye)",
            Self::Tiantu => "Tiantu (Celestial Projection)",
            Self::Tanzhong => "Tanzhong (Chest Center)",
            Self::Zhongwan => "Zhongwan (Central Cavity)",
            Self::Qihai => "Qihai (Sea of Chi)",
            Self::Jianjing => "Jianjing (Shoulder Well)",
            Self::Quchi => "Quchi (Pool at the Bend)",
            Self::Hegu => "Hegu (Joining Valley)",
            Self::Huantiao => "Huantiao (Jumping Round)",
            Self::Zusanli => "Zusanli (Leg Three Miles)",
            Self::Yongquan => "Yongquan (Bubbling Spring)",
        }
    }

    pub fn effect(&self) -> PressurePointEffect {
        match self {
            Self::Baihui => PressurePointEffect::Stun { duration: 3 },
            Self::Yintang => PressurePointEffect::Confusion { duration: 5 },
            Self::Tiantu => PressurePointEffect::Silence { duration: 4 },
            Self::Tanzhong => PressurePointEffect::ChiBlock { duration: 6 },
            Self::Zhongwan => PressurePointEffect::Weaken { amount: 30 },
            Self::Qihai => PressurePointEffect::ChiDrain { amount: 50 },
            Self::Jianjing => PressurePointEffect::ArmDisable { duration: 4 },
            Self::Quchi => PressurePointEffect::ArmNumb { duration: 3 },
            Self::Hegu => PressurePointEffect::Pain { damage: 20 },
            Self::Huantiao => PressurePointEffect::LegDisable { duration: 4 },
            Self::Zusanli => PressurePointEffect::Slow { duration: 5 },
            Self::Yongquan => PressurePointEffect::Knockdown,
        }
    }

    pub fn hit_difficulty(&self) -> u32 {
        match self {
            Self::Baihui | Self::Yintang => 90,
            Self::Tiantu | Self::Tanzhong | Self::Qihai => 70,
            Self::Zhongwan | Self::Jianjing => 60,
            Self::Quchi | Self::Hegu | Self::Huantiao => 50,
            Self::Zusanli | Self::Yongquan => 40,
        }
    }
}

/// Effects from striking pressure points
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PressurePointEffect {
    Stun { duration: u32 },
    Confusion { duration: u32 },
    Silence { duration: u32 },
    ChiBlock { duration: u32 },
    ChiDrain { amount: u32 },
    Weaken { amount: u32 },
    ArmDisable { duration: u32 },
    ArmNumb { duration: u32 },
    LegDisable { duration: u32 },
    Slow { duration: u32 },
    Pain { damage: i32 },
    Knockdown,
}

// ============================================================================
// CHI/KI SYSTEM - CHI STATE
// ============================================================================

/// Chi state for a character
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChiState {
    pub current_chi: u32,
    pub max_chi: u32,
    pub chi_regen: u32,
    pub meridian_levels: HashMap<Meridian, u32>,
    pub blocked_meridians: Vec<Meridian>,
    pub active_element: Option<Element>,
    pub chi_cultivation_level: u32,
}

impl Default for ChiState {
    fn default() -> Self {
        Self {
            current_chi: 100,
            max_chi: 100,
            chi_regen: BASE_CHI_REGEN,
            meridian_levels: HashMap::new(),
            blocked_meridians: Vec::new(),
            active_element: None,
            chi_cultivation_level: 1,
        }
    }
}

impl ChiState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spend_chi(&mut self, amount: u32) -> bool {
        if self.current_chi >= amount {
            self.current_chi -= amount;
            true
        } else {
            false
        }
    }

    pub fn restore_chi(&mut self, amount: u32) {
        self.current_chi = (self.current_chi + amount).min(self.max_chi);
    }

    pub fn regenerate(&mut self) {
        let regen = if self.blocked_meridians.is_empty() {
            self.chi_regen
        } else {
            self.chi_regen / 2
        };
        self.restore_chi(regen);
    }

    pub fn cultivate_meridian(&mut self, meridian: Meridian) {
        let level = self.meridian_levels.entry(meridian).or_insert(0);
        *level += 1;
        self.max_chi += meridian.chi_capacity_bonus() / 10;
    }

    pub fn block_meridian(&mut self, meridian: Meridian) {
        if !self.blocked_meridians.contains(&meridian) {
            self.blocked_meridians.push(meridian);
        }
    }

    pub fn unblock_meridian(&mut self, meridian: Meridian) {
        self.blocked_meridians.retain(|m| *m != meridian);
    }

    pub fn set_element(&mut self, element: Element) {
        self.active_element = Some(element);
    }

    pub fn total_meridian_level(&self) -> u32 {
        self.meridian_levels.values().sum()
    }
}

// ============================================================================
// CHI ATTACKS AND DEFENSES
// ============================================================================

/// Chi-based attack types
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChiAttack {
    ChiPalm,
    ChiBolt,
    ChiExplosion,
    ChiStrike,
    DimMak,
    ChiVampire,
    ElementalBurst { element: Element },
    ChiOverload,
}

impl ChiAttack {
    pub fn name(&self) -> &'static str {
        match self {
            Self::ChiPalm => "Chi Palm",
            Self::ChiBolt => "Chi Bolt",
            Self::ChiExplosion => "Chi Explosion",
            Self::ChiStrike => "Chi Strike",
            Self::DimMak => "Dim Mak (Death Touch)",
            Self::ChiVampire => "Chi Vampire",
            Self::ElementalBurst { .. } => "Elemental Burst",
            Self::ChiOverload => "Chi Overload",
        }
    }

    pub fn chi_cost(&self) -> u32 {
        match self {
            Self::ChiPalm => 10,
            Self::ChiBolt => 15,
            Self::ChiExplosion => 40,
            Self::ChiStrike => 20,
            Self::DimMak => 50,
            Self::ChiVampire => 30,
            Self::ElementalBurst { .. } => 35,
            Self::ChiOverload => 80,
        }
    }

    pub fn base_damage(&self) -> i32 {
        match self {
            Self::ChiPalm => 15,
            Self::ChiBolt => 20,
            Self::ChiExplosion => 35,
            Self::ChiStrike => 25,
            Self::DimMak => 50,
            Self::ChiVampire => 15,
            Self::ElementalBurst { .. } => 30,
            Self::ChiOverload => 100,
        }
    }

    pub fn range(&self) -> u32 {
        match self {
            Self::ChiPalm => 1,
            Self::ChiBolt => 5,
            Self::ChiExplosion => 3,
            Self::ChiStrike => 1,
            Self::DimMak => 1,
            Self::ChiVampire => 2,
            Self::ElementalBurst { .. } => 4,
            Self::ChiOverload => 2,
        }
    }
}

/// Chi-based defense types
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChiDefense {
    ChiShield,
    IronShirt,
    ChiDeflect,
    ChiCounter,
    ChiAbsorb,
    ChiDodge,
    ElementalWard { element: Element },
    DiamondBody,
}

impl ChiDefense {
    pub fn name(&self) -> &'static str {
        match self {
            Self::ChiShield => "Chi Shield",
            Self::IronShirt => "Iron Shirt",
            Self::ChiDeflect => "Chi Deflection",
            Self::ChiCounter => "Chi Counter",
            Self::ChiAbsorb => "Chi Absorption",
            Self::ChiDodge => "Chi Dodge",
            Self::ElementalWard { .. } => "Elemental Ward",
            Self::DiamondBody => "Diamond Body",
        }
    }

    pub fn chi_cost(&self) -> u32 {
        match self {
            Self::ChiShield => 15,
            Self::IronShirt => 25,
            Self::ChiDeflect => 10,
            Self::ChiCounter => 20,
            Self::ChiAbsorb => 30,
            Self::ChiDodge => 8,
            Self::ElementalWard { .. } => 20,
            Self::DiamondBody => 60,
        }
    }

    pub fn damage_reduction(&self) -> f32 {
        match self {
            Self::ChiShield => 0.3,
            Self::IronShirt => 0.5,
            Self::ChiDeflect => 0.2,
            Self::ChiCounter => 0.1,
            Self::ChiAbsorb => 0.4,
            Self::ChiDodge => 0.0,
            Self::ElementalWard { .. } => 0.6,
            Self::DiamondBody => 0.8,
        }
    }

    pub fn duration(&self) -> u32 {
        match self {
            Self::ChiShield => 3,
            Self::IronShirt => 5,
            Self::ChiDeflect => 1,
            Self::ChiCounter => 1,
            Self::ChiAbsorb => 2,
            Self::ChiDodge => 1,
            Self::ElementalWard { .. } => 4,
            Self::DiamondBody => 2,
        }
    }
}

// ============================================================================
// COMBO SYSTEM
// ============================================================================

/// Individual move in a combo
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComboMove {
    pub name: String,
    pub move_type: MoveType,
    pub damage: i32,
    pub chi_cost: u32,
    pub stun_chance: u32,
}

/// Types of moves
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MoveType {
    Punch,
    Kick,
    Palm,
    Elbow,
    Knee,
    Throw,
    Sweep,
    Block,
    Grab,
    Special,
}

impl MoveType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Punch => "Punch",
            Self::Kick => "Kick",
            Self::Palm => "Palm Strike",
            Self::Elbow => "Elbow Strike",
            Self::Knee => "Knee Strike",
            Self::Throw => "Throw",
            Self::Sweep => "Sweep",
            Self::Block => "Block",
            Self::Grab => "Grab",
            Self::Special => "Special",
        }
    }
}

/// Combo definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Combo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub moves: Vec<ComboMove>,
    pub combo_type: ComboType,
    pub required_style: Option<MartialStyle>,
    pub total_damage: i32,
    pub total_chi_cost: u32,
    pub finisher_bonus: f32,
}

/// Types of combos
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComboType {
    Basic,
    Advanced,
    Ultimate,
    Signature,
    Aerial,
    Ground,
}

impl ComboType {
    pub fn max_moves(&self) -> usize {
        match self {
            Self::Basic => 3,
            Self::Advanced => 5,
            Self::Ultimate => 7,
            Self::Signature => 5,
            Self::Aerial => 4,
            Self::Ground => 4,
        }
    }

    pub fn damage_multiplier(&self) -> f32 {
        match self {
            Self::Basic => 1.0,
            Self::Advanced => 1.3,
            Self::Ultimate => 2.0,
            Self::Signature => 1.5,
            Self::Aerial => 1.2,
            Self::Ground => 1.4,
        }
    }
}

impl Combo {
    pub fn new_basic(name: &str, description: &str, moves: Vec<ComboMove>) -> Self {
        let total_damage: i32 = moves.iter().map(|m| m.damage).sum();
        let total_chi_cost: u32 = moves.iter().map(|m| m.chi_cost).sum();
        Self {
            id: name.to_lowercase().replace(' ', "_"),
            name: name.to_string(),
            description: description.to_string(),
            moves,
            combo_type: ComboType::Basic,
            required_style: None,
            total_damage,
            total_chi_cost,
            finisher_bonus: 1.0,
        }
    }

    pub fn new_advanced(name: &str, description: &str, moves: Vec<ComboMove>, style: MartialStyle) -> Self {
        let total_damage: i32 = moves.iter().map(|m| m.damage).sum();
        let total_chi_cost: u32 = moves.iter().map(|m| m.chi_cost).sum();
        Self {
            id: name.to_lowercase().replace(' ', "_"),
            name: name.to_string(),
            description: description.to_string(),
            moves,
            combo_type: ComboType::Advanced,
            required_style: Some(style),
            total_damage,
            total_chi_cost,
            finisher_bonus: 1.3,
        }
    }

    pub fn new_ultimate(name: &str, description: &str, moves: Vec<ComboMove>, style: MartialStyle) -> Self {
        let total_damage: i32 = moves.iter().map(|m| m.damage).sum();
        let total_chi_cost: u32 = moves.iter().map(|m| m.chi_cost).sum();
        Self {
            id: name.to_lowercase().replace(' ', "_"),
            name: name.to_string(),
            description: description.to_string(),
            moves,
            combo_type: ComboType::Ultimate,
            required_style: Some(style),
            total_damage,
            total_chi_cost,
            finisher_bonus: 2.0,
        }
    }

    pub fn calculate_damage(&self, mastery_level: u32) -> i32 {
        let base = (self.total_damage as f32 * self.combo_type.damage_multiplier()) as i32;
        let mastery_bonus = (base as f32 * (mastery_level as f32 / 100.0)) as i32;
        ((base + mastery_bonus) as f32 * self.finisher_bonus) as i32
    }
}

/// Get basic combos (3-hit)
pub fn get_basic_combos() -> Vec<Combo> {
    vec![
        Combo::new_basic(
            "Triple Strike",
            "A quick three-hit combination of punches",
            vec![
                ComboMove { name: "Jab".into(), move_type: MoveType::Punch, damage: 5, chi_cost: 2, stun_chance: 5 },
                ComboMove { name: "Cross".into(), move_type: MoveType::Punch, damage: 8, chi_cost: 3, stun_chance: 10 },
                ComboMove { name: "Hook".into(), move_type: MoveType::Punch, damage: 10, chi_cost: 5, stun_chance: 15 },
            ]
        ),
        Combo::new_basic(
            "Kick Flurry",
            "Three rapid kicks targeting different heights",
            vec![
                ComboMove { name: "Low Kick".into(), move_type: MoveType::Kick, damage: 6, chi_cost: 3, stun_chance: 5 },
                ComboMove { name: "Body Kick".into(), move_type: MoveType::Kick, damage: 8, chi_cost: 4, stun_chance: 10 },
                ComboMove { name: "High Kick".into(), move_type: MoveType::Kick, damage: 12, chi_cost: 6, stun_chance: 20 },
            ]
        ),
        Combo::new_basic(
            "Palm Rush",
            "Three palm strikes flowing like water",
            vec![
                ComboMove { name: "Push Palm".into(), move_type: MoveType::Palm, damage: 7, chi_cost: 4, stun_chance: 8 },
                ComboMove { name: "Tiger Palm".into(), move_type: MoveType::Palm, damage: 9, chi_cost: 5, stun_chance: 12 },
                ComboMove { name: "Iron Palm".into(), move_type: MoveType::Palm, damage: 11, chi_cost: 6, stun_chance: 18 },
            ]
        ),
        Combo::new_basic(
            "Knee Destroyer",
            "Close-range knee strikes",
            vec![
                ComboMove { name: "Rising Knee".into(), move_type: MoveType::Knee, damage: 8, chi_cost: 4, stun_chance: 10 },
                ComboMove { name: "Driving Knee".into(), move_type: MoveType::Knee, damage: 10, chi_cost: 5, stun_chance: 15 },
                ComboMove { name: "Crushing Knee".into(), move_type: MoveType::Knee, damage: 12, chi_cost: 6, stun_chance: 25 },
            ]
        ),
    ]
}

/// Get advanced combos (5-hit)
pub fn get_advanced_combos() -> Vec<Combo> {
    vec![
        Combo::new_advanced(
            "Dragon Chain",
            "Five flowing strikes mimicking a dragon's movement",
            vec![
                ComboMove { name: "Dragon Tail".into(), move_type: MoveType::Sweep, damage: 8, chi_cost: 5, stun_chance: 15 },
                ComboMove { name: "Dragon Claw".into(), move_type: MoveType::Grab, damage: 10, chi_cost: 6, stun_chance: 10 },
                ComboMove { name: "Dragon Palm".into(), move_type: MoveType::Palm, damage: 12, chi_cost: 7, stun_chance: 12 },
                ComboMove { name: "Dragon Fang".into(), move_type: MoveType::Punch, damage: 14, chi_cost: 8, stun_chance: 18 },
                ComboMove { name: "Dragon Breath".into(), move_type: MoveType::Special, damage: 20, chi_cost: 15, stun_chance: 30 },
            ],
            MartialStyle::DragonFist
        ),
        Combo::new_advanced(
            "Tiger Rampage",
            "Ferocious five-strike tiger combination",
            vec![
                ComboMove { name: "Tiger Pounce".into(), move_type: MoveType::Special, damage: 10, chi_cost: 6, stun_chance: 12 },
                ComboMove { name: "Tiger Claw Left".into(), move_type: MoveType::Punch, damage: 12, chi_cost: 7, stun_chance: 15 },
                ComboMove { name: "Tiger Claw Right".into(), move_type: MoveType::Punch, damage: 12, chi_cost: 7, stun_chance: 15 },
                ComboMove { name: "Tiger Bite".into(), move_type: MoveType::Grab, damage: 15, chi_cost: 8, stun_chance: 20 },
                ComboMove { name: "Tiger Roar".into(), move_type: MoveType::Special, damage: 18, chi_cost: 12, stun_chance: 35 },
            ],
            MartialStyle::TigerClaw
        ),
        Combo::new_advanced(
            "Mantis Assault",
            "Quick mantis-style hooking attacks",
            vec![
                ComboMove { name: "Mantis Hook".into(), move_type: MoveType::Grab, damage: 8, chi_cost: 4, stun_chance: 10 },
                ComboMove { name: "Mantis Strike".into(), move_type: MoveType::Punch, damage: 10, chi_cost: 5, stun_chance: 12 },
                ComboMove { name: "Mantis Trap".into(), move_type: MoveType::Grab, damage: 9, chi_cost: 5, stun_chance: 15 },
                ComboMove { name: "Mantis Fury".into(), move_type: MoveType::Punch, damage: 12, chi_cost: 6, stun_chance: 18 },
                ComboMove { name: "Mantis Finisher".into(), move_type: MoveType::Special, damage: 16, chi_cost: 10, stun_chance: 25 },
            ],
            MartialStyle::NorthernPrayingMantis
        ),
        Combo::new_advanced(
            "Wing Chun Barrage",
            "Rapid centerline chain punches",
            vec![
                ComboMove { name: "Pak Sao".into(), move_type: MoveType::Block, damage: 3, chi_cost: 2, stun_chance: 5 },
                ComboMove { name: "Chain Punch 1".into(), move_type: MoveType::Punch, damage: 8, chi_cost: 4, stun_chance: 8 },
                ComboMove { name: "Chain Punch 2".into(), move_type: MoveType::Punch, damage: 8, chi_cost: 4, stun_chance: 8 },
                ComboMove { name: "Chain Punch 3".into(), move_type: MoveType::Punch, damage: 8, chi_cost: 4, stun_chance: 8 },
                ComboMove { name: "Biu Jee".into(), move_type: MoveType::Special, damage: 15, chi_cost: 8, stun_chance: 20 },
            ],
            MartialStyle::WingChun
        ),
    ]
}

/// Get ultimate combos (finishers)
pub fn get_ultimate_combos() -> Vec<Combo> {
    vec![
        Combo::new_ultimate(
            "Hundred Crack Fist",
            "An overwhelming barrage of punches too fast to see",
            vec![
                ComboMove { name: "Opening Strike".into(), move_type: MoveType::Punch, damage: 10, chi_cost: 5, stun_chance: 10 },
                ComboMove { name: "Rapid Fist x10".into(), move_type: MoveType::Punch, damage: 30, chi_cost: 15, stun_chance: 20 },
                ComboMove { name: "Rapid Fist x20".into(), move_type: MoveType::Punch, damage: 40, chi_cost: 20, stun_chance: 25 },
                ComboMove { name: "Rapid Fist x30".into(), move_type: MoveType::Punch, damage: 50, chi_cost: 25, stun_chance: 30 },
                ComboMove { name: "Rapid Fist x40".into(), move_type: MoveType::Punch, damage: 60, chi_cost: 30, stun_chance: 35 },
                ComboMove { name: "Final Flash".into(), move_type: MoveType::Special, damage: 80, chi_cost: 40, stun_chance: 50 },
                ComboMove { name: "You are already defeated".into(), move_type: MoveType::Special, damage: 100, chi_cost: 50, stun_chance: 100 },
            ],
            MartialStyle::ShaolinKungFu
        ),
        Combo::new_ultimate(
            "Phoenix Rebirth Technique",
            "Channel the phoenix to unleash devastating fire",
            vec![
                ComboMove { name: "Phoenix Stance".into(), move_type: MoveType::Special, damage: 5, chi_cost: 10, stun_chance: 0 },
                ComboMove { name: "Wing Sweep".into(), move_type: MoveType::Kick, damage: 15, chi_cost: 8, stun_chance: 15 },
                ComboMove { name: "Talon Strike".into(), move_type: MoveType::Punch, damage: 20, chi_cost: 10, stun_chance: 20 },
                ComboMove { name: "Feather Dance".into(), move_type: MoveType::Special, damage: 30, chi_cost: 15, stun_chance: 25 },
                ComboMove { name: "Rising Phoenix".into(), move_type: MoveType::Special, damage: 40, chi_cost: 20, stun_chance: 30 },
                ComboMove { name: "Phoenix Fire".into(), move_type: MoveType::Special, damage: 60, chi_cost: 30, stun_chance: 40 },
                ComboMove { name: "Eternal Rebirth".into(), move_type: MoveType::Special, damage: 80, chi_cost: 40, stun_chance: 60 },
            ],
            MartialStyle::PhoenixTalon
        ),
        Combo::new_ultimate(
            "Five Element Destruction",
            "Channel all five elements in devastating sequence",
            vec![
                ComboMove { name: "Wood - Growing Strike".into(), move_type: MoveType::Palm, damage: 20, chi_cost: 12, stun_chance: 15 },
                ComboMove { name: "Fire - Blazing Fist".into(), move_type: MoveType::Punch, damage: 25, chi_cost: 15, stun_chance: 20 },
                ComboMove { name: "Earth - Mountain Crush".into(), move_type: MoveType::Elbow, damage: 30, chi_cost: 18, stun_chance: 30 },
                ComboMove { name: "Metal - Piercing Strike".into(), move_type: MoveType::Punch, damage: 35, chi_cost: 20, stun_chance: 25 },
                ComboMove { name: "Water - Flowing Finish".into(), move_type: MoveType::Palm, damage: 40, chi_cost: 22, stun_chance: 35 },
                ComboMove { name: "Void - Absolute Zero".into(), move_type: MoveType::Special, damage: 60, chi_cost: 35, stun_chance: 50 },
                ComboMove { name: "Elemental Annihilation".into(), move_type: MoveType::Special, damage: 100, chi_cost: 50, stun_chance: 75 },
            ],
            MartialStyle::XingYiQuan
        ),
    ]
}

// ============================================================================
// TRAINING SYSTEM
// ============================================================================

/// Types of training activities
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrainingType {
    WoodenDummy,
    Sparring,
    Meditation,
    FormPractice,
    Conditioning,
    WeaponDrill,
    PressurePointPractice,
    ChiCirculation,
}

impl TrainingType {
    pub fn all() -> &'static [TrainingType] {
        &[
            Self::WoodenDummy, Self::Sparring, Self::Meditation,
            Self::FormPractice, Self::Conditioning, Self::WeaponDrill,
            Self::PressurePointPractice, Self::ChiCirculation,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::WoodenDummy => "Wooden Dummy Training",
            Self::Sparring => "Sparring",
            Self::Meditation => "Meditation",
            Self::FormPractice => "Form Practice",
            Self::Conditioning => "Conditioning",
            Self::WeaponDrill => "Weapon Drill",
            Self::PressurePointPractice => "Pressure Point Practice",
            Self::ChiCirculation => "Chi Circulation",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::WoodenDummy => "Practice techniques on the wooden dummy to improve timing and power.",
            Self::Sparring => "Spar with training partners to test your skills in combat.",
            Self::Meditation => "Meditate to cultivate chi and improve mental focus.",
            Self::FormPractice => "Practice traditional forms to internalize techniques.",
            Self::Conditioning => "Strengthen body through rigorous physical exercises.",
            Self::WeaponDrill => "Train with weapons to improve handling and technique.",
            Self::PressurePointPractice => "Study and practice pressure point locations.",
            Self::ChiCirculation => "Practice circulating chi through meridians.",
        }
    }

    pub fn xp_gain(&self) -> u32 {
        match self {
            Self::WoodenDummy => 20,
            Self::Sparring => 35,
            Self::Meditation => 15,
            Self::FormPractice => 25,
            Self::Conditioning => 20,
            Self::WeaponDrill => 30,
            Self::PressurePointPractice => 25,
            Self::ChiCirculation => 20,
        }
    }

    pub fn chi_gain(&self) -> u32 {
        match self {
            Self::WoodenDummy => 5,
            Self::Sparring => 10,
            Self::Meditation => 30,
            Self::FormPractice => 15,
            Self::Conditioning => 5,
            Self::WeaponDrill => 8,
            Self::PressurePointPractice => 12,
            Self::ChiCirculation => 25,
        }
    }

    pub fn stat_improved(&self) -> &'static str {
        match self {
            Self::WoodenDummy => "Attack",
            Self::Sparring => "Combat",
            Self::Meditation => "Chi",
            Self::FormPractice => "Technique",
            Self::Conditioning => "Stamina",
            Self::WeaponDrill => "Weapon Skill",
            Self::PressurePointPractice => "Precision",
            Self::ChiCirculation => "Chi Regen",
        }
    }
}

/// Practice dummy types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DummyType {
    WoodenMan,
    MookJongDummy,
    MakiwaraPost,
    IronDummy,
    CombatDummy,
    AcupunctureDummy,
}

impl DummyType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::WoodenMan => "Wooden Man",
            Self::MookJongDummy => "Mook Jong (Wing Chun Dummy)",
            Self::MakiwaraPost => "Makiwara Post",
            Self::IronDummy => "Iron Dummy",
            Self::CombatDummy => "Combat Dummy",
            Self::AcupunctureDummy => "Acupuncture Dummy",
        }
    }

    pub fn training_bonus(&self) -> f32 {
        match self {
            Self::WoodenMan => 1.0,
            Self::MookJongDummy => 1.3,
            Self::MakiwaraPost => 1.2,
            Self::IronDummy => 1.5,
            Self::CombatDummy => 1.4,
            Self::AcupunctureDummy => 1.6,
        }
    }
}

/// Training session result
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrainingResult {
    pub training_type: TrainingType,
    pub xp_gained: u32,
    pub chi_gained: u32,
    pub mastery_progress: u32,
    pub stat_improvements: HashMap<String, i32>,
    pub breakthroughs: Vec<String>,
}

// ============================================================================
// STYLE MASTERY
// ============================================================================

/// Mastery progress for a martial style
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StyleMastery {
    pub style: MartialStyle,
    pub level: u32,
    pub experience: u32,
    pub techniques_learned: Vec<String>,
    pub forms_mastered: Vec<String>,
    pub combos_unlocked: Vec<String>,
    pub time_trained: u32,
}

impl StyleMastery {
    pub fn new(style: MartialStyle) -> Self {
        Self {
            style,
            level: 1,
            experience: 0,
            techniques_learned: Vec::new(),
            forms_mastered: Vec::new(),
            combos_unlocked: Vec::new(),
            time_trained: 0,
        }
    }

    pub fn add_experience(&mut self, xp: u32) -> bool {
        self.experience += xp;
        let xp_needed = self.level * XP_PER_MASTERY_LEVEL;
        if self.experience >= xp_needed && self.level < MAX_MASTERY_LEVEL {
            self.experience -= xp_needed;
            self.level += 1;
            true
        } else {
            false
        }
    }

    pub fn mastery_title(&self) -> &'static str {
        match self.level {
            1..=10 => "Novice",
            11..=25 => "Student",
            26..=40 => "Practitioner",
            41..=55 => "Adept",
            56..=70 => "Expert",
            71..=85 => "Master",
            86..=95 => "Grandmaster",
            96..=100 => "Legendary Master",
            _ => "Unknown",
        }
    }

    pub fn damage_bonus(&self) -> f32 {
        1.0 + (self.level as f32 * 0.02)
    }

    pub fn chi_efficiency(&self) -> f32 {
        1.0 - (self.level as f32 * 0.005).min(0.5)
    }
}

// ============================================================================
// MASTERS AND DOJOS
// ============================================================================

/// Legendary martial arts masters
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Master {
    pub id: String,
    pub name: String,
    pub title: String,
    pub description: String,
    pub styles_taught: Vec<MartialStyle>,
    pub location: DojoLocation,
    pub reputation_required: u32,
    pub teaching_quality: f32,
    pub secret_techniques: Vec<String>,
    pub is_legendary: bool,
}

impl Master {
    pub fn teaching_xp_bonus(&self) -> f32 {
        self.teaching_quality * if self.is_legendary { 2.0 } else { 1.0 }
    }
}

/// Get all legendary masters
pub fn get_legendary_masters() -> Vec<Master> {
    vec![
        Master {
            id: "master_wong".into(),
            name: "Wong Fei-Hung".into(),
            title: "The Tiger of Canton".into(),
            description: "Legendary master of Hung Gar and traditional medicine.".into(),
            styles_taught: vec![MartialStyle::HungGar, MartialStyle::TigerClaw],
            location: DojoLocation::SouthernTemple,
            reputation_required: 500,
            teaching_quality: 1.8,
            secret_techniques: vec!["Tiger-Crane Double Form".into(), "Shadowless Kick".into()],
            is_legendary: true,
        },
        Master {
            id: "master_ip".into(),
            name: "Ip Man".into(),
            title: "Grandmaster of Wing Chun".into(),
            description: "The legendary Wing Chun master who taught many famous students.".into(),
            styles_taught: vec![MartialStyle::WingChun],
            location: DojoLocation::CityDojo,
            reputation_required: 400,
            teaching_quality: 1.9,
            secret_techniques: vec!["Wooden Dummy 108 Moves".into(), "Chi Sao Mastery".into()],
            is_legendary: true,
        },
        Master {
            id: "master_zhang".into(),
            name: "Zhang Sanfeng".into(),
            title: "Founder of Tai Chi".into(),
            description: "The immortal Taoist who created Tai Chi Chuan.".into(),
            styles_taught: vec![MartialStyle::TaiChi, MartialStyle::WudangQuan],
            location: DojoLocation::WudangMountain,
            reputation_required: 800,
            teaching_quality: 2.0,
            secret_techniques: vec!["Primordial Chaos Palm".into(), "Yin-Yang Integration".into()],
            is_legendary: true,
        },
        Master {
            id: "master_dong".into(),
            name: "Dong Haichuan".into(),
            title: "Creator of Bagua Zhang".into(),
            description: "The legendary eunuch who developed the circle walking art.".into(),
            styles_taught: vec![MartialStyle::BaguaZhang],
            location: DojoLocation::ImperialPalace,
            reputation_required: 700,
            teaching_quality: 1.85,
            secret_techniques: vec!["64 Palms of Heaven".into(), "Dragon Body Palm".into()],
            is_legendary: true,
        },
        Master {
            id: "master_bodhidharma".into(),
            name: "Bodhidharma".into(),
            title: "First Patriarch of Shaolin".into(),
            description: "The Indian monk who brought martial arts to Shaolin Temple.".into(),
            styles_taught: vec![MartialStyle::ShaolinKungFu, MartialStyle::DragonFist],
            location: DojoLocation::ShaolinTemple,
            reputation_required: 1000,
            teaching_quality: 2.5,
            secret_techniques: vec!["18 Luohan Hands".into(), "Marrow Washing Classic".into()],
            is_legendary: true,
        },
        Master {
            id: "master_wang".into(),
            name: "Wang Xiangzhai".into(),
            title: "Founder of Yi Quan".into(),
            description: "Revolutionary master who stripped kung fu to its essence.".into(),
            styles_taught: vec![MartialStyle::YiQuan, MartialStyle::XingYiQuan],
            location: DojoLocation::NorthernAcademy,
            reputation_required: 600,
            teaching_quality: 1.75,
            secret_techniques: vec!["Standing Stake Method".into(), "Explosive Power Fajin".into()],
            is_legendary: true,
        },
        Master {
            id: "master_yang".into(),
            name: "Yang Luchan".into(),
            title: "Yang the Invincible".into(),
            description: "Never defeated master who spread Tai Chi to the world.".into(),
            styles_taught: vec![MartialStyle::TaiChi],
            location: DojoLocation::YangFamilyVillage,
            reputation_required: 750,
            teaching_quality: 1.9,
            secret_techniques: vec!["Yang Family Push Hands".into(), "Neutralizing Force".into()],
            is_legendary: true,
        },
        Master {
            id: "master_huo".into(),
            name: "Huo Yuanjia".into(),
            title: "Hero of China".into(),
            description: "Patriotic master who defended Chinese honor against foreigners.".into(),
            styles_taught: vec![MartialStyle::Sanda, MartialStyle::NorthernPrayingMantis],
            location: DojoLocation::JingwuSchool,
            reputation_required: 550,
            teaching_quality: 1.7,
            secret_techniques: vec!["Lost Track Fist".into(), "Jingwu Spirit".into()],
            is_legendary: true,
        },
    ]
}

/// Get regular masters
pub fn get_regular_masters() -> Vec<Master> {
    vec![
        Master {
            id: "sifu_chen".into(),
            name: "Sifu Chen".into(),
            title: "Eagle Claw Instructor".into(),
            description: "A skilled instructor specializing in gripping techniques.".into(),
            styles_taught: vec![MartialStyle::EagleClaw],
            location: DojoLocation::CityDojo,
            reputation_required: 100,
            teaching_quality: 1.2,
            secret_techniques: vec!["108 Locking Hands".into()],
            is_legendary: false,
        },
        Master {
            id: "sifu_lin".into(),
            name: "Sifu Lin".into(),
            title: "White Crane Master".into(),
            description: "Graceful master of the crane style from Fujian.".into(),
            styles_taught: vec![MartialStyle::WhiteCrane, MartialStyle::CraneWing],
            location: DojoLocation::CoastalDojo,
            reputation_required: 200,
            teaching_quality: 1.4,
            secret_techniques: vec!["Crying Crane Form".into()],
            is_legendary: false,
        },
        Master {
            id: "sifu_wu".into(),
            name: "Sifu Wu".into(),
            title: "Weapon Master".into(),
            description: "Expert in traditional Chinese weapons.".into(),
            styles_taught: vec![MartialStyle::JianShu, MartialStyle::DaoShu, MartialStyle::GunShu],
            location: DojoLocation::WeaponSchool,
            reputation_required: 300,
            teaching_quality: 1.5,
            secret_techniques: vec!["Seven Star Sword".into()],
            is_legendary: false,
        },
        Master {
            id: "sifu_zhao".into(),
            name: "Sifu Zhao".into(),
            title: "Northern Fist Expert".into(),
            description: "Master of long-range northern techniques.".into(),
            styles_taught: vec![MartialStyle::LongFist, MartialStyle::ChangQuan, MartialStyle::FanziQuan],
            location: DojoLocation::NorthernAcademy,
            reputation_required: 150,
            teaching_quality: 1.3,
            secret_techniques: vec!["Cannon Through Sky".into()],
            is_legendary: false,
        },
    ]
}

/// Dojo/training locations
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DojoLocation {
    ShaolinTemple,
    WudangMountain,
    SouthernTemple,
    NorthernAcademy,
    CityDojo,
    CoastalDojo,
    MountainRetreat,
    ForestHermitage,
    ImperialPalace,
    UndergroundArena,
    WeaponSchool,
    JingwuSchool,
    YangFamilyVillage,
    HiddenValley,
}

impl DojoLocation {
    pub fn all() -> &'static [DojoLocation] {
        &[
            Self::ShaolinTemple, Self::WudangMountain, Self::SouthernTemple,
            Self::NorthernAcademy, Self::CityDojo, Self::CoastalDojo,
            Self::MountainRetreat, Self::ForestHermitage, Self::ImperialPalace,
            Self::UndergroundArena, Self::WeaponSchool, Self::JingwuSchool,
            Self::YangFamilyVillage, Self::HiddenValley,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::ShaolinTemple => "Shaolin Temple",
            Self::WudangMountain => "Wudang Mountain",
            Self::SouthernTemple => "Southern Temple",
            Self::NorthernAcademy => "Northern Academy",
            Self::CityDojo => "City Martial Arts School",
            Self::CoastalDojo => "Coastal Training Hall",
            Self::MountainRetreat => "Mountain Retreat",
            Self::ForestHermitage => "Forest Hermitage",
            Self::ImperialPalace => "Imperial Palace Guard School",
            Self::UndergroundArena => "Underground Fighting Arena",
            Self::WeaponSchool => "Traditional Weapon School",
            Self::JingwuSchool => "Jingwu Athletic Association",
            Self::YangFamilyVillage => "Yang Family Village",
            Self::HiddenValley => "Hidden Valley Sanctuary",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::ShaolinTemple => "The legendary Buddhist temple where kung fu was born.",
            Self::WudangMountain => "Sacred Taoist mountain known for internal martial arts.",
            Self::SouthernTemple => "Temple in the south specializing in close-range combat.",
            Self::NorthernAcademy => "Academy teaching northern long-range techniques.",
            Self::CityDojo => "A modern martial arts school in the city.",
            Self::CoastalDojo => "Training hall by the sea, known for crane styles.",
            Self::MountainRetreat => "Secluded retreat for intensive training.",
            Self::ForestHermitage => "Hidden hermitage where masters go to meditate.",
            Self::ImperialPalace => "Elite training grounds for palace guards.",
            Self::UndergroundArena => "Secret fighting pit for testing real combat skills.",
            Self::WeaponSchool => "School dedicated to traditional weapon arts.",
            Self::JingwuSchool => "Famous school promoting martial arts as national heritage.",
            Self::YangFamilyVillage => "Ancestral home of Yang style Tai Chi.",
            Self::HiddenValley => "Mysterious valley where legendary masters train in secret.",
        }
    }

    pub fn training_bonus(&self) -> f32 {
        match self {
            Self::ShaolinTemple => 1.5,
            Self::WudangMountain => 1.6,
            Self::SouthernTemple => 1.3,
            Self::NorthernAcademy => 1.3,
            Self::CityDojo => 1.0,
            Self::CoastalDojo => 1.1,
            Self::MountainRetreat => 1.4,
            Self::ForestHermitage => 1.5,
            Self::ImperialPalace => 1.4,
            Self::UndergroundArena => 1.2,
            Self::WeaponSchool => 1.3,
            Self::JingwuSchool => 1.2,
            Self::YangFamilyVillage => 1.4,
            Self::HiddenValley => 2.0,
        }
    }

    pub fn available_training(&self) -> Vec<TrainingType> {
        match self {
            Self::ShaolinTemple => vec![
                TrainingType::FormPractice, TrainingType::Meditation,
                TrainingType::Conditioning, TrainingType::WoodenDummy,
            ],
            Self::WudangMountain => vec![
                TrainingType::Meditation, TrainingType::ChiCirculation,
                TrainingType::FormPractice,
            ],
            Self::UndergroundArena => vec![
                TrainingType::Sparring, TrainingType::Conditioning,
            ],
            Self::WeaponSchool => vec![
                TrainingType::WeaponDrill, TrainingType::FormPractice,
            ],
            _ => TrainingType::all().to_vec(),
        }
    }
}

/// Dojo information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dojo {
    pub location: DojoLocation,
    pub masters: Vec<String>,
    pub students: u32,
    pub reputation: u32,
    pub facilities: Vec<DummyType>,
    pub entry_fee: u32,
    pub monthly_dues: u32,
}

impl Dojo {
    pub fn new(location: DojoLocation) -> Self {
        Self {
            location,
            masters: Vec::new(),
            students: 0,
            reputation: 100,
            facilities: vec![DummyType::WoodenMan],
            entry_fee: 100,
            monthly_dues: 50,
        }
    }
}

// ============================================================================
// TOURNAMENTS
// ============================================================================

/// Tournament types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TournamentType {
    LocalAmateur,
    Regional,
    National,
    International,
    Underground,
    GrandMasterTournament,
}

impl TournamentType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::LocalAmateur => "Local Amateur Tournament",
            Self::Regional => "Regional Championship",
            Self::National => "National Championship",
            Self::International => "International Martial Arts Tournament",
            Self::Underground => "Underground Fighting Circuit",
            Self::GrandMasterTournament => "Grand Master Tournament",
        }
    }

    pub fn entry_requirement(&self) -> u32 {
        match self {
            Self::LocalAmateur => 10,
            Self::Regional => 30,
            Self::National => 50,
            Self::International => 70,
            Self::Underground => 40,
            Self::GrandMasterTournament => 90,
        }
    }

    pub fn prize_pool(&self) -> u32 {
        match self {
            Self::LocalAmateur => 500,
            Self::Regional => 2000,
            Self::National => 10000,
            Self::International => 50000,
            Self::Underground => 5000,
            Self::GrandMasterTournament => 100000,
        }
    }

    pub fn reputation_reward(&self) -> u32 {
        match self {
            Self::LocalAmateur => 10,
            Self::Regional => 30,
            Self::National => 100,
            Self::International => 300,
            Self::Underground => 50,
            Self::GrandMasterTournament => 500,
        }
    }
}

/// Tournament match
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TournamentMatch {
    pub round: u32,
    pub opponent_name: String,
    pub opponent_style: MartialStyle,
    pub opponent_level: u32,
    pub is_boss: bool,
}

/// Tournament state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tournament {
    pub tournament_type: TournamentType,
    pub current_round: u32,
    pub max_rounds: u32,
    pub matches: Vec<TournamentMatch>,
    pub wins: u32,
    pub losses: u32,
    pub is_active: bool,
    pub prize_won: u32,
}

impl Tournament {
    pub fn new(tournament_type: TournamentType) -> Self {
        let max_rounds = match tournament_type {
            TournamentType::LocalAmateur => 3,
            TournamentType::Regional => 4,
            TournamentType::National => 5,
            TournamentType::International => 6,
            TournamentType::Underground => 5,
            TournamentType::GrandMasterTournament => 7,
        };

        Self {
            tournament_type,
            current_round: 0,
            max_rounds,
            matches: Vec::new(),
            wins: 0,
            losses: 0,
            is_active: true,
            prize_won: 0,
        }
    }

    pub fn advance_round(&mut self) -> bool {
        if self.current_round < self.max_rounds {
            self.current_round += 1;
            true
        } else {
            self.is_active = false;
            false
        }
    }

    pub fn record_win(&mut self) {
        self.wins += 1;
        if self.wins >= self.max_rounds {
            self.prize_won = self.tournament_type.prize_pool();
            self.is_active = false;
        }
    }

    pub fn record_loss(&mut self) {
        self.losses += 1;
        if self.losses >= 2 {
            self.is_active = false;
        }
    }
}

// ============================================================================
// MAIN MARTIAL ARTS SYSTEM
// ============================================================================

/// Tournament record tracking
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TournamentRecord {
    pub total_fights: u32,
    pub wins: u32,
    pub losses: u32,
    pub tournaments_won: u32,
    pub tournaments_entered: u32,
    pub highest_tournament_reached: Option<TournamentType>,
}

/// Main martial arts system managing all martial arts mechanics
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MartialArtsSystem {
    pub learned_styles: HashMap<MartialStyle, StyleMastery>,
    pub active_style: Option<MartialStyle>,
    pub chi_state: ChiState,
    pub known_combos: Vec<Combo>,
    pub active_combo: Option<String>,
    pub combo_counter: u32,
    pub training_history: Vec<TrainingResult>,
    pub masters_trained_under: Vec<String>,
    pub current_dojo: Option<DojoLocation>,
    pub tournament_record: TournamentRecord,
    pub active_tournament: Option<Tournament>,
    pub reputation: u32,
    pub titles: Vec<String>,
    pub total_training_time: u32,
    pub weapon_proficiency: HashMap<WeaponType, u32>,
    pub known_pressure_points: Vec<PressurePoint>,
    pub active_defense: Option<ChiDefense>,
    pub defense_duration: u32,
}

impl Default for MartialArtsSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl MartialArtsSystem {
    pub fn new() -> Self {
        Self {
            learned_styles: HashMap::new(),
            active_style: None,
            chi_state: ChiState::new(),
            known_combos: get_basic_combos(),
            active_combo: None,
            combo_counter: 0,
            training_history: Vec::new(),
            masters_trained_under: Vec::new(),
            current_dojo: None,
            tournament_record: TournamentRecord::default(),
            active_tournament: None,
            reputation: 0,
            titles: Vec::new(),
            total_training_time: 0,
            weapon_proficiency: HashMap::new(),
            known_pressure_points: Vec::new(),
            active_defense: None,
            defense_duration: 0,
        }
    }

    /// Learn a new martial arts style
    pub fn learn_style(&mut self, style: MartialStyle) -> bool {
        if self.learned_styles.contains_key(&style) {
            return false;
        }

        if style.category() == StyleCategory::Legendary {
            let required_mastery = self.learned_styles.values()
                .filter(|m| m.level >= 50)
                .count();
            if required_mastery < 3 {
                return false;
            }
        }

        self.learned_styles.insert(style, StyleMastery::new(style));

        if self.active_style.is_none() {
            self.active_style = Some(style);
        }

        true
    }

    /// Set the active fighting style
    pub fn set_active_style(&mut self, style: MartialStyle) -> bool {
        if self.learned_styles.contains_key(&style) {
            self.active_style = Some(style);
            true
        } else {
            false
        }
    }

    /// Get mastery level for a style
    pub fn get_mastery(&self, style: MartialStyle) -> Option<&StyleMastery> {
        self.learned_styles.get(&style)
    }

    /// Add experience to the active style
    pub fn add_style_experience(&mut self, xp: u32) -> Option<bool> {
        if let Some(style) = self.active_style {
            if let Some(mastery) = self.learned_styles.get_mut(&style) {
                return Some(mastery.add_experience(xp));
            }
        }
        None
    }

    /// Perform training
    pub fn train(&mut self, training_type: TrainingType) -> TrainingResult {
        let location_bonus = self.current_dojo
            .map(|d| d.training_bonus())
            .unwrap_or(1.0);

        let base_xp = training_type.xp_gain();
        let base_chi = training_type.chi_gain();

        let xp_gained = (base_xp as f32 * location_bonus) as u32;
        let chi_gained = (base_chi as f32 * location_bonus) as u32;

        self.add_style_experience(xp_gained);

        if matches!(training_type, TrainingType::Meditation | TrainingType::ChiCirculation) {
            self.chi_state.restore_chi(chi_gained);
        }

        self.total_training_time += 1;

        let result = TrainingResult {
            training_type,
            xp_gained,
            chi_gained,
            mastery_progress: xp_gained,
            stat_improvements: HashMap::new(),
            breakthroughs: Vec::new(),
        };

        self.training_history.push(result.clone());
        result
    }

    /// Execute a chi attack
    pub fn execute_chi_attack(&mut self, attack: &ChiAttack) -> Option<i32> {
        let cost = attack.chi_cost();
        if self.chi_state.spend_chi(cost) {
            let base_damage = attack.base_damage();

            let element_bonus = if let ChiAttack::ElementalBurst { element } = attack {
                if self.chi_state.active_element == Some(*element) { 1.5 } else { 1.0 }
            } else {
                1.0
            };

            let mastery_bonus = self.active_style
                .and_then(|s| self.learned_styles.get(&s))
                .map(|m| m.damage_bonus())
                .unwrap_or(1.0);

            Some((base_damage as f32 * element_bonus * mastery_bonus) as i32)
        } else {
            None
        }
    }

    /// Activate a chi defense
    pub fn activate_defense(&mut self, defense: ChiDefense) -> bool {
        let cost = defense.chi_cost();
        if self.chi_state.spend_chi(cost) {
            self.defense_duration = defense.duration();
            self.active_defense = Some(defense);
            true
        } else {
            false
        }
    }

    /// Process defense (call each turn)
    pub fn process_defense(&mut self) {
        if self.defense_duration > 0 {
            self.defense_duration -= 1;
            if self.defense_duration == 0 {
                self.active_defense = None;
            }
        }
    }

    /// Get current damage reduction from active defense
    pub fn get_damage_reduction(&self) -> f32 {
        self.active_defense
            .as_ref()
            .map(|d| d.damage_reduction())
            .unwrap_or(0.0)
    }

    /// Start a combo
    pub fn start_combo(&mut self, combo_id: &str) -> bool {
        if let Some(combo) = self.known_combos.iter().find(|c| c.id == combo_id) {
            if let Some(required_style) = combo.required_style {
                if self.active_style != Some(required_style) {
                    return false;
                }
            }

            if self.chi_state.current_chi >= combo.total_chi_cost {
                self.active_combo = Some(combo_id.to_string());
                self.combo_counter = 0;
                return true;
            }
        }
        false
    }

    /// Execute next move in combo
    pub fn execute_combo_move(&mut self) -> Option<i32> {
        let combo_id = self.active_combo.clone()?;
        let combo = self.known_combos.iter().find(|c| c.id == combo_id)?;

        if self.combo_counter as usize >= combo.moves.len() {
            self.finish_combo();
            return None;
        }

        let move_data = &combo.moves[self.combo_counter as usize];

        if !self.chi_state.spend_chi(move_data.chi_cost) {
            self.finish_combo();
            return None;
        }

        let mastery_level = self.active_style
            .and_then(|s| self.learned_styles.get(&s))
            .map(|m| m.level)
            .unwrap_or(1);

        let damage = (move_data.damage as f32 * (1.0 + mastery_level as f32 * 0.01)) as i32;

        self.combo_counter += 1;

        if self.combo_counter as usize >= combo.moves.len() {
            let bonus_damage = (damage as f32 * combo.finisher_bonus) as i32;
            self.finish_combo();
            return Some(bonus_damage);
        }

        Some(damage)
    }

    /// Finish/cancel combo
    pub fn finish_combo(&mut self) {
        self.active_combo = None;
        self.combo_counter = 0;
    }

    /// Learn a combo
    pub fn learn_combo(&mut self, combo: Combo) {
        if !self.known_combos.iter().any(|c| c.id == combo.id) {
            self.known_combos.push(combo);
        }
    }

    /// Learn a pressure point
    pub fn learn_pressure_point(&mut self, point: PressurePoint) {
        if !self.known_pressure_points.contains(&point) {
            self.known_pressure_points.push(point);
        }
    }

    /// Attack a pressure point
    pub fn attack_pressure_point(&mut self, point: PressurePoint) -> Option<PressurePointEffect> {
        if !self.known_pressure_points.contains(&point) {
            return None;
        }

        let chi_cost = point.hit_difficulty() / 5;
        if !self.chi_state.spend_chi(chi_cost) {
            return None;
        }

        Some(point.effect())
    }

    /// Join a dojo
    pub fn join_dojo(&mut self, location: DojoLocation) -> bool {
        self.current_dojo = Some(location);
        true
    }

    /// Leave current dojo
    pub fn leave_dojo(&mut self) {
        self.current_dojo = None;
    }

    /// Train under a master
    pub fn train_under_master(&mut self, master: &Master) -> bool {
        if self.reputation < master.reputation_required {
            return false;
        }

        if !self.masters_trained_under.contains(&master.id) {
            self.masters_trained_under.push(master.id.clone());
        }

        for style in &master.styles_taught {
            self.learn_style(*style);
        }

        let xp = (50.0 * master.teaching_xp_bonus()) as u32;
        self.add_style_experience(xp);

        true
    }

    /// Enter a tournament
    pub fn enter_tournament(&mut self, tournament_type: TournamentType) -> bool {
        if self.active_tournament.is_some() {
            return false;
        }

        let avg_mastery: u32 = if self.learned_styles.is_empty() {
            0
        } else {
            self.learned_styles.values().map(|m| m.level).sum::<u32>()
                / self.learned_styles.len() as u32
        };

        if avg_mastery < tournament_type.entry_requirement() {
            return false;
        }

        self.active_tournament = Some(Tournament::new(tournament_type));
        self.tournament_record.tournaments_entered += 1;
        true
    }

    /// Record tournament win
    pub fn tournament_win(&mut self) {
        if let Some(ref mut tournament) = self.active_tournament {
            tournament.record_win();
            tournament.advance_round();
            self.tournament_record.wins += 1;
            self.tournament_record.total_fights += 1;

            if !tournament.is_active && tournament.wins >= tournament.max_rounds {
                self.tournament_record.tournaments_won += 1;
                self.reputation += tournament.tournament_type.reputation_reward();

                if self.tournament_record.highest_tournament_reached.is_none()
                    || tournament.tournament_type as u8 > self.tournament_record.highest_tournament_reached.unwrap() as u8 {
                    self.tournament_record.highest_tournament_reached = Some(tournament.tournament_type);
                }
            }
        }
    }

    /// Record tournament loss
    pub fn tournament_loss(&mut self) {
        if let Some(ref mut tournament) = self.active_tournament {
            tournament.record_loss();
            self.tournament_record.losses += 1;
            self.tournament_record.total_fights += 1;
        }
    }

    /// Finish tournament
    pub fn finish_tournament(&mut self) -> Option<u32> {
        let prize = self.active_tournament.as_ref().map(|t| t.prize_won);
        self.active_tournament = None;
        prize
    }

    /// Add weapon proficiency
    pub fn train_weapon(&mut self, weapon: WeaponType, amount: u32) {
        let level = self.weapon_proficiency.entry(weapon).or_insert(0);
        *level = (*level + amount).min(100);
    }

    /// Get weapon proficiency
    pub fn get_weapon_proficiency(&self, weapon: WeaponType) -> u32 {
        *self.weapon_proficiency.get(&weapon).unwrap_or(&0)
    }

    /// Add reputation
    pub fn add_reputation(&mut self, amount: u32) {
        self.reputation += amount;

        if self.reputation >= 100 && !self.titles.contains(&"Martial Artist".to_string()) {
            self.titles.push("Martial Artist".to_string());
        }
        if self.reputation >= 500 && !self.titles.contains(&"Skilled Fighter".to_string()) {
            self.titles.push("Skilled Fighter".to_string());
        }
        if self.reputation >= 1000 && !self.titles.contains(&"Master".to_string()) {
            self.titles.push("Master".to_string());
        }
        if self.reputation >= 5000 && !self.titles.contains(&"Grandmaster".to_string()) {
            self.titles.push("Grandmaster".to_string());
        }
        if self.reputation >= 10000 && !self.titles.contains(&"Living Legend".to_string()) {
            self.titles.push("Living Legend".to_string());
        }
    }

    /// Regenerate chi (call each turn)
    pub fn tick(&mut self) {
        self.chi_state.regenerate();
        self.process_defense();
    }

    /// Calculate total attack power
    pub fn calculate_attack_power(&self) -> i32 {
        let base = self.active_style
            .map(|s| s.base_damage())
            .unwrap_or(5);

        let mastery_bonus = self.active_style
            .and_then(|s| self.learned_styles.get(&s))
            .map(|m| (base as f32 * (m.damage_bonus() - 1.0)) as i32)
            .unwrap_or(0);

        base + mastery_bonus
    }

    /// Get all available combos for current style
    pub fn get_available_combos(&self) -> Vec<&Combo> {
        self.known_combos.iter()
            .filter(|c| {
                c.required_style.is_none() || c.required_style == self.active_style
            })
            .collect()
    }

    /// Get summary of martial arts progress
    pub fn get_summary(&self) -> MartialArtsSummary {
        MartialArtsSummary {
            styles_learned: self.learned_styles.len() as u32,
            active_style: self.active_style,
            highest_mastery: self.learned_styles.values()
                .map(|m| m.level)
                .max()
                .unwrap_or(0),
            total_chi: self.chi_state.max_chi,
            combos_known: self.known_combos.len() as u32,
            reputation: self.reputation,
            tournaments_won: self.tournament_record.tournaments_won,
            titles: self.titles.clone(),
        }
    }
}

/// Summary of martial arts progress
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MartialArtsSummary {
    pub styles_learned: u32,
    pub active_style: Option<MartialStyle>,
    pub highest_mastery: u32,
    pub total_chi: u32,
    pub combos_known: u32,
    pub reputation: u32,
    pub tournaments_won: u32,
    pub titles: Vec<String>,
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_categories() {
        assert_eq!(MartialStyle::WingChun.category(), StyleCategory::Southern);
        assert_eq!(MartialStyle::TaiChi.category(), StyleCategory::Internal);
        assert_eq!(MartialStyle::DragonFist.category(), StyleCategory::Legendary);
    }

    #[test]
    fn test_chi_state() {
        let mut chi = ChiState::new();
        assert!(chi.spend_chi(50));
        assert_eq!(chi.current_chi, 50);
        chi.restore_chi(30);
        assert_eq!(chi.current_chi, 80);
    }

    #[test]
    fn test_martial_arts_system() {
        let mut system = MartialArtsSystem::new();
        assert!(system.learn_style(MartialStyle::WingChun));
        assert!(!system.learn_style(MartialStyle::WingChun));
        assert_eq!(system.active_style, Some(MartialStyle::WingChun));
    }

    #[test]
    fn test_element_relationships() {
        assert_eq!(Element::Fire.overcomes(), Element::Metal);
        assert_eq!(Element::Water.overcomes(), Element::Fire);
        assert_eq!(Element::Fire.damage_multiplier(Element::Metal), 1.5);
    }

    #[test]
    fn test_combo_creation() {
        let combos = get_basic_combos();
        assert!(!combos.is_empty());
        assert_eq!(combos[0].moves.len(), 3);
    }

    #[test]
    fn test_mastery_level_up() {
        let mut mastery = StyleMastery::new(MartialStyle::ShaolinKungFu);
        assert_eq!(mastery.level, 1);
        mastery.add_experience(500);
        assert_eq!(mastery.level, 2);
    }

    #[test]
    fn test_pressure_points() {
        let point = PressurePoint::Baihui;
        assert_eq!(point.hit_difficulty(), 90);
    }
}
