//! Personality system for agents
//!
//! Defines personality traits, emotions, and moods that affect behavior.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use rand::prelude::*;

use crate::agents::AgentKind;

/// Complete personality profile for an agent
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Personality {
    /// Big Five personality traits
    pub traits: PersonalityTraits,
    /// Current emotional state
    pub emotion: Emotion,
    /// Current mood
    pub mood: Mood,
    /// Emotional stability (how quickly emotions change)
    pub stability: f32,
    /// Values and beliefs
    pub values: Vec<Value>,
    /// Quirks and unique behaviors
    pub quirks: Vec<Quirk>,
    /// Fears
    pub fears: Vec<Fear>,
    /// Desires
    pub desires: Vec<Desire>,
}

impl Personality {
    /// Creates a new random personality
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            traits: PersonalityTraits::random(&mut rng),
            emotion: Emotion::Neutral,
            mood: Mood::Neutral,
            stability: rng.gen_range(0.3..0.9),
            values: Vec::new(),
            quirks: Vec::new(),
            fears: Vec::new(),
            desires: Vec::new(),
        }
    }

    /// Creates a personality suitable for an agent kind
    pub fn random_for_kind(kind: AgentKind) -> Self {
        let mut personality = Self::random();

        // Adjust traits based on kind
        match kind {
            // Guards are conscientious and less neurotic
            AgentKind::Guard | AgentKind::Captain => {
                personality.traits.conscientiousness = 0.8;
                personality.traits.neuroticism = 0.3;
                personality.values.push(Value::Duty);
                personality.values.push(Value::Order);
            }
            // Merchants are extraverted and agreeable
            AgentKind::Merchant | AgentKind::Innkeeper => {
                personality.traits.extraversion = 0.8;
                personality.traits.agreeableness = 0.7;
                personality.values.push(Value::Wealth);
                personality.desires.push(Desire::Profit);
            }
            // Wizards are open and introverted
            AgentKind::Wizard | AgentKind::Scholar | AgentKind::Librarian => {
                personality.traits.openness = 0.9;
                personality.traits.extraversion = 0.3;
                personality.values.push(Value::Knowledge);
                personality.desires.push(Desire::Understanding);
            }
            // Thieves are low agreeableness
            AgentKind::Thief | AgentKind::Assassin => {
                personality.traits.agreeableness = 0.2;
                personality.traits.conscientiousness = 0.6;
                personality.values.push(Value::Freedom);
                personality.desires.push(Desire::Wealth);
            }
            // Priests are agreeable and conscientious
            AgentKind::Priest | AgentKind::Healer => {
                personality.traits.agreeableness = 0.9;
                personality.traits.conscientiousness = 0.7;
                personality.values.push(Value::Faith);
                personality.values.push(Value::Compassion);
            }
            // Children are high extraversion, low conscientiousness
            AgentKind::Child => {
                personality.traits.extraversion = 0.8;
                personality.traits.conscientiousness = 0.3;
                personality.traits.neuroticism = 0.5;
                personality.desires.push(Desire::Play);
            }
            // Enemies tend toward aggression
            AgentKind::OrcWarrior | AgentKind::OrcChieftain => {
                personality.traits.agreeableness = 0.1;
                personality.traits.neuroticism = 0.7;
                personality.values.push(Value::Strength);
                personality.fears.push(Fear::Weakness);
            }
            // Undead are emotionless
            AgentKind::SkeletonSoldier | AgentKind::SkeletonMage | AgentKind::ZombieHorde => {
                personality.traits = PersonalityTraits::neutral();
                personality.stability = 1.0;
                personality.emotion = Emotion::Neutral;
            }
            // Demons are chaotic
            AgentKind::DemonSoldier | AgentKind::DemonLord => {
                personality.traits.agreeableness = 0.0;
                personality.traits.neuroticism = 0.9;
                personality.values.push(Value::Power);
                personality.values.push(Value::Chaos);
            }
            _ => {}
        }

        personality
    }

    /// Triggers an emotion
    pub fn feel(&mut self, emotion: Emotion, intensity: f32) {
        let change = intensity * (1.0 - self.stability);
        if change > 0.3 {
            self.emotion = emotion;
        }
    }

    /// Updates mood based on recent emotions
    pub fn update_mood(&mut self, recent_emotions: &[Emotion]) {
        if recent_emotions.is_empty() {
            return;
        }

        // Count positive vs negative emotions
        let mut positive = 0;
        let mut negative = 0;
        for emotion in recent_emotions {
            match emotion {
                Emotion::Happy | Emotion::Excited | Emotion::Curious |
                Emotion::Grateful | Emotion::Proud => positive += 1,
                Emotion::Sad | Emotion::Angry | Emotion::Fearful |
                Emotion::Disgusted | Emotion::Anxious => negative += 1,
                _ => {}
            }
        }

        if positive > negative * 2 {
            self.mood = Mood::Happy;
        } else if negative > positive * 2 {
            self.mood = Mood::Depressed;
        } else if negative > positive {
            self.mood = Mood::Irritable;
        } else {
            self.mood = Mood::Neutral;
        }
    }

    /// Gets a behavior modifier based on current emotional state
    pub fn behavior_modifier(&self) -> BehaviorModifier {
        let mut modifier = BehaviorModifier::default();

        // Trait effects
        modifier.aggression += (1.0 - self.traits.agreeableness) * 0.3;
        modifier.caution += self.traits.neuroticism * 0.3;
        modifier.sociability += self.traits.extraversion * 0.3;
        modifier.creativity += self.traits.openness * 0.3;
        modifier.reliability += self.traits.conscientiousness * 0.3;

        // Emotion effects
        match self.emotion {
            Emotion::Angry => modifier.aggression += 0.5,
            Emotion::Fearful => modifier.caution += 0.5,
            Emotion::Happy => modifier.sociability += 0.3,
            Emotion::Sad => modifier.sociability -= 0.3,
            Emotion::Excited => modifier.creativity += 0.3,
            _ => {}
        }

        // Mood effects
        match self.mood {
            Mood::Happy => {
                modifier.sociability += 0.2;
                modifier.aggression -= 0.2;
            }
            Mood::Depressed => {
                modifier.sociability -= 0.3;
                modifier.caution += 0.2;
            }
            Mood::Irritable => {
                modifier.aggression += 0.3;
                modifier.sociability -= 0.2;
            }
            Mood::Anxious => {
                modifier.caution += 0.4;
            }
            _ => {}
        }

        modifier
    }

    /// Checks if this personality values something
    pub fn values(&self, value: Value) -> bool {
        self.values.contains(&value)
    }

    /// Checks if this personality fears something
    pub fn fears(&self, fear: Fear) -> bool {
        self.fears.contains(&fear)
    }

    /// Checks if this personality desires something
    pub fn desires(&self, desire: Desire) -> bool {
        self.desires.contains(&desire)
    }
}

impl Default for Personality {
    fn default() -> Self {
        Self::random()
    }
}

/// Big Five personality traits (OCEAN model)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PersonalityTraits {
    /// Openness to experience (0.0 to 1.0)
    pub openness: f32,
    /// Conscientiousness (0.0 to 1.0)
    pub conscientiousness: f32,
    /// Extraversion (0.0 to 1.0)
    pub extraversion: f32,
    /// Agreeableness (0.0 to 1.0)
    pub agreeableness: f32,
    /// Neuroticism (0.0 to 1.0)
    pub neuroticism: f32,
}

impl PersonalityTraits {
    /// Creates random traits
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        Self {
            openness: rng.gen(),
            conscientiousness: rng.gen(),
            extraversion: rng.gen(),
            agreeableness: rng.gen(),
            neuroticism: rng.gen(),
        }
    }

    /// Creates neutral traits (all 0.5)
    pub fn neutral() -> Self {
        Self {
            openness: 0.5,
            conscientiousness: 0.5,
            extraversion: 0.5,
            agreeableness: 0.5,
            neuroticism: 0.5,
        }
    }
}

impl Default for PersonalityTraits {
    fn default() -> Self {
        Self::neutral()
    }
}

/// Current emotional state
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Emotion {
    Neutral,
    Happy,
    Sad,
    Angry,
    Fearful,
    Surprised,
    Disgusted,
    Curious,
    Excited,
    Bored,
    Grateful,
    Resentful,
    Proud,
    Ashamed,
    Anxious,
    Calm,
    Hopeful,
    Desperate,
}

impl Emotion {
    /// Returns whether this is a positive emotion
    pub fn is_positive(&self) -> bool {
        matches!(
            self,
            Self::Happy | Self::Excited | Self::Curious |
            Self::Grateful | Self::Proud | Self::Calm | Self::Hopeful
        )
    }

    /// Returns whether this is a negative emotion
    pub fn is_negative(&self) -> bool {
        matches!(
            self,
            Self::Sad | Self::Angry | Self::Fearful |
            Self::Disgusted | Self::Resentful | Self::Ashamed |
            Self::Anxious | Self::Desperate
        )
    }

    /// Returns the opposite emotion
    pub fn opposite(&self) -> Self {
        match self {
            Self::Happy => Self::Sad,
            Self::Sad => Self::Happy,
            Self::Angry => Self::Calm,
            Self::Fearful => Self::Calm,
            Self::Excited => Self::Bored,
            Self::Bored => Self::Excited,
            Self::Hopeful => Self::Desperate,
            Self::Desperate => Self::Hopeful,
            Self::Proud => Self::Ashamed,
            Self::Ashamed => Self::Proud,
            Self::Grateful => Self::Resentful,
            Self::Resentful => Self::Grateful,
            _ => Self::Neutral,
        }
    }
}

/// Current mood (longer-lasting than emotions)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mood {
    Neutral,
    Happy,
    Depressed,
    Irritable,
    Anxious,
    Excited,
    Calm,
    Aggressive,
}

impl Mood {
    /// Returns whether this mood makes the agent more hostile
    pub fn is_hostile(&self) -> bool {
        matches!(self, Self::Irritable | Self::Aggressive)
    }

    /// Returns whether this mood makes the agent more friendly
    pub fn is_friendly(&self) -> bool {
        matches!(self, Self::Happy | Self::Calm)
    }
}

/// Core values that guide behavior
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Value {
    Honor,
    Duty,
    Family,
    Freedom,
    Knowledge,
    Power,
    Wealth,
    Faith,
    Compassion,
    Justice,
    Order,
    Chaos,
    Nature,
    Tradition,
    Progress,
    Strength,
    Wisdom,
    Beauty,
    Truth,
    Loyalty,
}

/// Personality quirks
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Quirk {
    TalksToSelf,
    HoardsShinyThings,
    NeverLies,
    AlwaysLies,
    Superstitious,
    OCD,
    Narcoleptic,
    Perfectionist,
    Glutton,
    Teetotaler,
    GamblingAddict,
    Workaholic,
    Lazy,
    Paranoid,
    Optimistic,
    Pessimistic,
    Forgetful,
    PhotonicMemory,
}

/// Things agents can fear
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Fear {
    Death,
    Darkness,
    Heights,
    Fire,
    Water,
    Undead,
    Demons,
    Magic,
    Crowds,
    Solitude,
    Failure,
    Rejection,
    Poverty,
    Weakness,
    Betrayal,
    Unknown,
}

/// Things agents can desire
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Desire {
    Wealth,
    Power,
    Knowledge,
    Love,
    Revenge,
    Peace,
    Adventure,
    Fame,
    Understanding,
    Safety,
    Freedom,
    Control,
    Pleasure,
    Purpose,
    Connection,
    Profit,
    Play,
}

/// Modifiers to behavior based on personality
#[derive(Clone, Debug, Default)]
pub struct BehaviorModifier {
    /// Tendency toward aggressive actions
    pub aggression: f32,
    /// Tendency toward cautious actions
    pub caution: f32,
    /// Tendency toward social actions
    pub sociability: f32,
    /// Tendency toward creative solutions
    pub creativity: f32,
    /// Tendency toward reliable behavior
    pub reliability: f32,
}

/// Trait for defining personality archetypes
pub struct PersonalityArchetype {
    pub name: String,
    pub traits: PersonalityTraits,
    pub values: Vec<Value>,
    pub quirks: Vec<Quirk>,
    pub fears: Vec<Fear>,
    pub desires: Vec<Desire>,
}

impl PersonalityArchetype {
    /// The Hero archetype
    pub fn hero() -> Self {
        Self {
            name: "Hero".to_string(),
            traits: PersonalityTraits {
                openness: 0.7,
                conscientiousness: 0.8,
                extraversion: 0.6,
                agreeableness: 0.7,
                neuroticism: 0.3,
            },
            values: vec![Value::Honor, Value::Justice, Value::Compassion],
            quirks: vec![],
            fears: vec![Fear::Failure],
            desires: vec![Desire::Purpose, Desire::Fame],
        }
    }

    /// The Villain archetype
    pub fn villain() -> Self {
        Self {
            name: "Villain".to_string(),
            traits: PersonalityTraits {
                openness: 0.5,
                conscientiousness: 0.7,
                extraversion: 0.5,
                agreeableness: 0.1,
                neuroticism: 0.6,
            },
            values: vec![Value::Power, Value::Chaos],
            quirks: vec![Quirk::Paranoid],
            fears: vec![Fear::Weakness, Fear::Betrayal],
            desires: vec![Desire::Power, Desire::Control],
        }
    }

    /// The Sage archetype
    pub fn sage() -> Self {
        Self {
            name: "Sage".to_string(),
            traits: PersonalityTraits {
                openness: 0.9,
                conscientiousness: 0.7,
                extraversion: 0.3,
                agreeableness: 0.6,
                neuroticism: 0.2,
            },
            values: vec![Value::Knowledge, Value::Truth, Value::Wisdom],
            quirks: vec![Quirk::TalksToSelf],
            fears: vec![Fear::Unknown],
            desires: vec![Desire::Knowledge, Desire::Understanding],
        }
    }

    /// The Trickster archetype
    pub fn trickster() -> Self {
        Self {
            name: "Trickster".to_string(),
            traits: PersonalityTraits {
                openness: 0.8,
                conscientiousness: 0.3,
                extraversion: 0.7,
                agreeableness: 0.4,
                neuroticism: 0.4,
            },
            values: vec![Value::Freedom, Value::Chaos],
            quirks: vec![Quirk::AlwaysLies],
            fears: vec![Fear::Solitude],
            desires: vec![Desire::Freedom, Desire::Pleasure],
        }
    }

    /// Applies archetype to a personality
    pub fn apply_to(&self, personality: &mut Personality) {
        personality.traits = self.traits.clone();
        personality.values = self.values.clone();
        personality.quirks = self.quirks.clone();
        personality.fears = self.fears.clone();
        personality.desires = self.desires.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_personality_creation() {
        let personality = Personality::random();
        assert!(personality.traits.openness >= 0.0 && personality.traits.openness <= 1.0);
    }

    #[test]
    fn test_emotion_polarity() {
        assert!(Emotion::Happy.is_positive());
        assert!(Emotion::Angry.is_negative());
        assert!(!Emotion::Neutral.is_positive());
        assert!(!Emotion::Neutral.is_negative());
    }

    #[test]
    fn test_archetype() {
        let mut personality = Personality::random();
        PersonalityArchetype::hero().apply_to(&mut personality);
        assert!(personality.values.contains(&Value::Honor));
    }
}
