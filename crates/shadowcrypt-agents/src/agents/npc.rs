//! NPC agent implementations

use super::{Agent, AgentKind, AgentState};
use crate::behavior::{Goal, GoalKind, GoalPriority};
use rand::Rng;

/// NPC-specific behaviors and dialogue
pub struct NpcBehaviors;

impl NpcBehaviors {
    /// Creates default goals for an NPC based on their kind
    pub fn default_goals(kind: AgentKind) -> Vec<Goal> {
        match kind {
            AgentKind::Blacksmith => vec![
                Goal::new(GoalKind::Work, GoalPriority::Medium),
                Goal::new(GoalKind::Trade, GoalPriority::Low),
            ],
            AgentKind::Merchant => vec![
                Goal::new(GoalKind::Trade, GoalPriority::High),
                Goal::new(GoalKind::Socialize, GoalPriority::Low),
            ],
            AgentKind::Guard => vec![
                Goal::new(GoalKind::Patrol, GoalPriority::High),
                Goal::new(GoalKind::ProtectArea, GoalPriority::Critical),
            ],
            AgentKind::Captain => vec![
                Goal::new(GoalKind::Command, GoalPriority::High),
                Goal::new(GoalKind::Patrol, GoalPriority::Medium),
            ],
            AgentKind::Priest => vec![
                Goal::new(GoalKind::Heal, GoalPriority::High),
                Goal::new(GoalKind::Bless, GoalPriority::Medium),
            ],
            AgentKind::Farmer => vec![
                Goal::new(GoalKind::Work, GoalPriority::High),
                Goal::new(GoalKind::Rest, GoalPriority::Low),
            ],
            AgentKind::Innkeeper => vec![
                Goal::new(GoalKind::Trade, GoalPriority::High),
                Goal::new(GoalKind::Socialize, GoalPriority::Medium),
            ],
            AgentKind::Bard => vec![
                Goal::new(GoalKind::Entertain, GoalPriority::High),
                Goal::new(GoalKind::Socialize, GoalPriority::Medium),
            ],
            AgentKind::Thief => vec![
                Goal::new(GoalKind::Steal, GoalPriority::Medium),
                Goal::new(GoalKind::Hide, GoalPriority::High),
            ],
            AgentKind::Scholar | AgentKind::Librarian => vec![
                Goal::new(GoalKind::Study, GoalPriority::High),
                Goal::new(GoalKind::Teach, GoalPriority::Medium),
            ],
            AgentKind::Wizard => vec![
                Goal::new(GoalKind::Study, GoalPriority::High),
                Goal::new(GoalKind::Practice, GoalPriority::Medium),
            ],
            AgentKind::Hunter => vec![
                Goal::new(GoalKind::Hunt, GoalPriority::High),
                Goal::new(GoalKind::Explore, GoalPriority::Medium),
            ],
            AgentKind::Traveler => vec![
                Goal::new(GoalKind::Explore, GoalPriority::High),
                Goal::new(GoalKind::Trade, GoalPriority::Low),
            ],
            AgentKind::Child => vec![
                Goal::new(GoalKind::Play, GoalPriority::High),
                Goal::new(GoalKind::Follow, GoalPriority::Medium),
            ],
            AgentKind::Beggar => vec![
                Goal::new(GoalKind::Beg, GoalPriority::High),
                Goal::new(GoalKind::Survive, GoalPriority::Critical),
            ],
            AgentKind::Noble => vec![
                Goal::new(GoalKind::Socialize, GoalPriority::High),
                Goal::new(GoalKind::Command, GoalPriority::Medium),
            ],
            _ => vec![
                Goal::new(GoalKind::Idle, GoalPriority::Low),
            ],
        }
    }

    /// Gets random dialogue for an NPC
    pub fn random_dialogue<R: Rng>(kind: AgentKind, rng: &mut R) -> String {
        let dialogues = Self::dialogues_for_kind(kind);
        dialogues[rng.gen_range(0..dialogues.len())].to_string()
    }

    /// Gets all possible dialogues for an NPC kind
    pub fn dialogues_for_kind(kind: AgentKind) -> &'static [&'static str] {
        match kind {
            AgentKind::VillageElder => &[
                "Welcome, traveler. Our village has seen many dark days.",
                "The crypt beneath us holds ancient evils. Be wary.",
                "I sense great power in you. Use it wisely.",
                "The old ways are fading, but hope remains.",
                "Many have entered the dungeon. Few return.",
            ],
            AgentKind::Blacksmith => &[
                "Need something forged? I'm your man.",
                "This blade took me three days to perfect.",
                "Quality steel is hard to come by these days.",
                "I can repair that armor for you.",
                "My father was a blacksmith, and his father before him.",
            ],
            AgentKind::Alchemist => &[
                "Potions, elixirs, and more! What do you need?",
                "Careful with that! It's highly volatile.",
                "I've been experimenting with new formulas.",
                "Healing potions are in high demand lately.",
                "The herbs from the forest make the best mixtures.",
            ],
            AgentKind::Merchant => &[
                "Everything has a price, friend.",
                "I've got goods from all across the realm!",
                "Looking to buy or sell?",
                "These are the finest wares you'll find.",
                "Business has been slow since the darkness came.",
            ],
            AgentKind::Innkeeper => &[
                "Welcome to my humble inn!",
                "A room for the night? Or perhaps a warm meal?",
                "Ale, mead, or something stronger?",
                "You look like you've traveled far.",
                "Plenty of rumors floating around these days.",
            ],
            AgentKind::Guard => &[
                "Move along, citizen.",
                "I've got my eye on you.",
                "Stay out of trouble.",
                "The streets aren't safe after dark.",
                "Report any suspicious activity.",
            ],
            AgentKind::Captain => &[
                "My guards will keep order here.",
                "I answer to the lord of this land.",
                "Discipline is the key to survival.",
                "We've increased patrols since the attacks.",
                "The dungeon threatens us all.",
            ],
            AgentKind::Priest => &[
                "May the light guide your path.",
                "I can heal your wounds, for a small donation.",
                "The darkness grows stronger each day.",
                "Faith is our greatest weapon.",
                "The undead fear the holy light.",
            ],
            AgentKind::Scholar => &[
                "Knowledge is power, young one.",
                "I've studied the ancient texts for decades.",
                "The truth lies buried in these tomes.",
                "History has much to teach us.",
                "The old kingdoms knew secrets we've forgotten.",
            ],
            AgentKind::Bard => &[
                "♪ A hero rises from the depths below... ♪",
                "Care to hear a tale of adventure?",
                "Music soothes the savage beast!",
                "I've traveled far and seen much.",
                "Every hero needs a bard to tell their story!",
            ],
            AgentKind::Thief => &[
                "You didn't see me here, understand?",
                "I've got... special goods, if you're interested.",
                "The shadows are my home.",
                "Everyone has secrets. I know most of them.",
                "Keep your gold close, friend.",
            ],
            AgentKind::Wizard => &[
                "The arcane arts require years of study.",
                "Magic flows through all things.",
                "I sense magical energy on you.",
                "Beware the dark magic in the crypt.",
                "Knowledge of the elements is key.",
            ],
            AgentKind::Farmer => &[
                "The crops aren't doing well this season.",
                "Hard work keeps the village fed.",
                "Have you seen my chickens?",
                "The soil has been strange lately.",
                "We pray for rain and sunshine.",
            ],
            AgentKind::Hunter => &[
                "The forest holds many dangers.",
                "I track all manner of beasts.",
                "Fresh game for sale!",
                "Something has scared the animals away.",
                "My bow never misses.",
            ],
            AgentKind::Healer => &[
                "Let me see those wounds.",
                "Herbs and patience heal most ills.",
                "Rest is the best medicine.",
                "I've treated many adventurers.",
                "Take this poultice for the road.",
            ],
            AgentKind::Child => &[
                "Are you a real adventurer?",
                "Will you play with me?",
                "My mom says the dungeon is scary!",
                "I found a shiny rock!",
                "When I grow up, I want to be a hero!",
            ],
            AgentKind::Beggar => &[
                "Spare a coin for the poor?",
                "I've fallen on hard times...",
                "Anything helps, friend.",
                "I wasn't always like this...",
                "The gods bless those who give.",
            ],
            AgentKind::Traveler => &[
                "I've seen strange things on the road.",
                "The world is vast and full of wonder.",
                "I never stay in one place for long.",
                "Adventure calls to those who listen.",
                "Each journey teaches something new.",
            ],
            _ => &[
                "Hello there.",
                "Nice weather, isn't it?",
                "Be careful out there.",
                "Good luck, adventurer.",
                "Safe travels.",
            ],
        }
    }

    /// Gets greeting dialogue for an NPC
    pub fn greeting(kind: AgentKind) -> &'static str {
        match kind {
            AgentKind::VillageElder => "Greetings, brave adventurer.",
            AgentKind::Blacksmith => "Ho there! Need something forged?",
            AgentKind::Merchant => "Welcome, welcome! Browse my wares!",
            AgentKind::Innkeeper => "Welcome, weary traveler!",
            AgentKind::Guard => "Halt! State your business.",
            AgentKind::Priest => "Blessings upon you, child.",
            AgentKind::Bard => "Ah, a new face! Care for a song?",
            AgentKind::Wizard => "The magic stirs... you have potential.",
            AgentKind::Thief => "Psst! Over here...",
            _ => "Hello there.",
        }
    }

    /// Gets farewell dialogue for an NPC
    pub fn farewell(kind: AgentKind) -> &'static str {
        match kind {
            AgentKind::VillageElder => "May wisdom guide your path.",
            AgentKind::Blacksmith => "Come back when you need repairs!",
            AgentKind::Merchant => "Come again soon!",
            AgentKind::Innkeeper => "Safe travels!",
            AgentKind::Guard => "Move along now.",
            AgentKind::Priest => "Go with the light.",
            AgentKind::Bard => "Until we meet again!",
            AgentKind::Wizard => "The arcane watches over you.",
            AgentKind::Thief => "You never saw me...",
            _ => "Farewell.",
        }
    }
}

/// Represents an NPC's daily schedule
#[derive(Clone, Debug)]
pub struct NpcSchedule {
    /// Activities by hour (0-23)
    pub activities: [(usize, usize, NpcActivity); 24],
}

/// Activities an NPC can perform
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NpcActivity {
    Sleep,
    Work,
    Eat,
    Socialize,
    Patrol,
    Wander,
    Trade,
    Pray,
    Study,
    Train,
    Entertain,
}

impl NpcSchedule {
    /// Creates a default schedule for a given NPC kind
    pub fn default_for_kind(kind: AgentKind) -> Self {
        let activities = match kind {
            AgentKind::Guard => [
                (0, 0, NpcActivity::Sleep), (0, 0, NpcActivity::Sleep),
                (0, 0, NpcActivity::Sleep), (0, 0, NpcActivity::Sleep),
                (0, 0, NpcActivity::Sleep), (0, 0, NpcActivity::Sleep),
                (10, 10, NpcActivity::Patrol), (10, 10, NpcActivity::Patrol),
                (10, 10, NpcActivity::Patrol), (10, 10, NpcActivity::Patrol),
                (10, 10, NpcActivity::Patrol), (10, 10, NpcActivity::Patrol),
                (5, 5, NpcActivity::Eat), (10, 10, NpcActivity::Patrol),
                (10, 10, NpcActivity::Patrol), (10, 10, NpcActivity::Patrol),
                (10, 10, NpcActivity::Patrol), (10, 10, NpcActivity::Patrol),
                (5, 5, NpcActivity::Eat), (10, 10, NpcActivity::Patrol),
                (10, 10, NpcActivity::Patrol), (10, 10, NpcActivity::Patrol),
                (0, 0, NpcActivity::Sleep), (0, 0, NpcActivity::Sleep),
            ],
            AgentKind::Merchant => [
                (0, 0, NpcActivity::Sleep), (0, 0, NpcActivity::Sleep),
                (0, 0, NpcActivity::Sleep), (0, 0, NpcActivity::Sleep),
                (0, 0, NpcActivity::Sleep), (0, 0, NpcActivity::Sleep),
                (0, 0, NpcActivity::Sleep), (5, 5, NpcActivity::Eat),
                (10, 10, NpcActivity::Trade), (10, 10, NpcActivity::Trade),
                (10, 10, NpcActivity::Trade), (10, 10, NpcActivity::Trade),
                (5, 5, NpcActivity::Eat), (10, 10, NpcActivity::Trade),
                (10, 10, NpcActivity::Trade), (10, 10, NpcActivity::Trade),
                (10, 10, NpcActivity::Trade), (10, 10, NpcActivity::Trade),
                (5, 5, NpcActivity::Eat), (15, 15, NpcActivity::Socialize),
                (15, 15, NpcActivity::Socialize), (0, 0, NpcActivity::Sleep),
                (0, 0, NpcActivity::Sleep), (0, 0, NpcActivity::Sleep),
            ],
            _ => [
                (0, 0, NpcActivity::Sleep); 24
            ],
        };
        Self { activities }
    }

    /// Gets the activity for a given hour
    pub fn get_activity(&self, hour: usize) -> NpcActivity {
        self.activities[hour % 24].2
    }

    /// Gets the location for a given hour
    pub fn get_location(&self, hour: usize) -> (usize, usize) {
        let (x, y, _) = self.activities[hour % 24];
        (x, y)
    }
}

/// NPC trade inventory
#[derive(Clone, Debug, Default)]
pub struct NpcShop {
    /// Items for sale with prices
    pub items: Vec<(String, u32)>,
    /// Items the NPC will buy
    pub buy_list: Vec<String>,
    /// Buy price multiplier (e.g., 0.5 = half price)
    pub buy_multiplier: f32,
}

impl NpcShop {
    /// Creates a new shop
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an item to the shop
    pub fn add_item(&mut self, item: String, price: u32) {
        self.items.push((item, price));
    }

    /// Creates a default shop for an NPC kind
    pub fn default_for_kind(kind: AgentKind) -> Self {
        let mut shop = Self::new();
        shop.buy_multiplier = 0.5;

        match kind {
            AgentKind::Blacksmith => {
                shop.add_item("Iron Sword".to_string(), 50);
                shop.add_item("Steel Sword".to_string(), 150);
                shop.add_item("Iron Armor".to_string(), 100);
                shop.add_item("Steel Armor".to_string(), 250);
                shop.add_item("Iron Shield".to_string(), 75);
                shop.buy_list = vec!["Ore".to_string(), "Metal".to_string()];
            }
            AgentKind::Alchemist => {
                shop.add_item("Health Potion".to_string(), 30);
                shop.add_item("Mana Potion".to_string(), 30);
                shop.add_item("Strength Potion".to_string(), 50);
                shop.add_item("Antidote".to_string(), 25);
                shop.add_item("Fire Resistance".to_string(), 75);
                shop.buy_list = vec!["Herb".to_string(), "Mushroom".to_string()];
            }
            AgentKind::Merchant => {
                shop.add_item("Torch".to_string(), 5);
                shop.add_item("Rope".to_string(), 10);
                shop.add_item("Bread".to_string(), 3);
                shop.add_item("Cheese".to_string(), 5);
                shop.add_item("Map".to_string(), 25);
                shop.buy_list = vec!["Gold".to_string(), "Gem".to_string()];
            }
            AgentKind::Innkeeper => {
                shop.add_item("Room (Night)".to_string(), 10);
                shop.add_item("Meal".to_string(), 5);
                shop.add_item("Ale".to_string(), 2);
                shop.add_item("Wine".to_string(), 8);
                shop.buy_list = vec![];
            }
            _ => {}
        }

        shop
    }
}
