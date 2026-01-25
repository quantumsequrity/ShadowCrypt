// NPC System for ShadowCrypt Roguelike
use crossterm::style::Color;

// Forward declare types from main.rs
use crate::{ItemKind, Rarity};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NPCType {
    Merchant,
    QuestGiver,
    Trainer,
    Blacksmith,
    Alchemist,
    Enchanter,
    Healer,
    Sage,
    Guard,
    Prisoner,
    Companion,
}

#[derive(Clone, Debug)]
pub enum DialogueAction {
    GiveQuest(u32),
    CompleteQuest(u32),
    OpenShop,
    OpenTraining,
    Heal,
    GiveItem(ItemKind),
    TakeGold(u32),
    JoinParty,
    Fight,
}

#[derive(Clone, Debug)]
pub enum DialogueCondition {
    HasQuest(u32),
    QuestComplete(u32),
    HasItem(ItemKind),
    HasGold(u32),
    LevelAtLeast(u32),
    SpeciesIs(String),
    ClassIs(String),
}

#[derive(Clone, Debug)]
pub struct DialogueResponse {
    pub text: String,
    pub next_node: Option<usize>,
    pub action: Option<DialogueAction>,
    pub condition: Option<DialogueCondition>,
}

#[derive(Clone, Debug)]
pub struct DialogueNode {
    #[allow(dead_code)]
    pub id: usize,
    pub speaker: String,
    pub text: String,
    pub responses: Vec<DialogueResponse>,
}

#[derive(Clone, Debug)]
pub struct DialogueTree {
    pub nodes: Vec<DialogueNode>,
    pub current_node: usize,
}

impl DialogueTree {
    pub fn current(&self) -> Option<&DialogueNode> {
        self.nodes.get(self.current_node)
    }

    pub fn reset(&mut self) {
        self.current_node = 0;
    }

    pub fn advance(&mut self, response_idx: usize) -> Option<DialogueResponse> {
        if let Some(node) = self.nodes.get(self.current_node) {
            if let Some(response) = node.responses.get(response_idx) {
                if let Some(next) = response.next_node {
                    self.current_node = next;
                }
                return Some(response.clone());
            }
        }
        None
    }
}

#[derive(Clone, Debug)]
pub struct NPC {
    pub x: usize,
    pub y: usize,
    pub name: String,
    pub npc_type: NPCType,
    pub dialogue_tree: DialogueTree,
    pub shop_inventory: Option<Vec<(ItemKind, Rarity, u32)>>,
    #[allow(dead_code)]
    pub quests: Vec<u32>,
    pub friendly: bool,
}

impl NPC {
    pub fn merchant(x: usize, y: usize) -> Self {
        Self {
            x, y,
            name: "Traveling Merchant".into(),
            npc_type: NPCType::Merchant,
            dialogue_tree: DialogueTree {
                nodes: vec![
                    DialogueNode {
                        id: 0,
                        speaker: "Merchant".into(),
                        text: "Welcome, traveler! Care to browse my wares?".into(),
                        responses: vec![
                            DialogueResponse {
                                text: "Show me what you have.".into(),
                                next_node: None,
                                action: Some(DialogueAction::OpenShop),
                                condition: None,
                            },
                            DialogueResponse {
                                text: "Not right now.".into(),
                                next_node: None,
                                action: None,
                                condition: None,
                            },
                        ],
                    },
                ],
                current_node: 0,
            },
            shop_inventory: Some(vec![
                (ItemKind::HealthPotion, Rarity::Common, 25),
                (ItemKind::ManaPotion, Rarity::Common, 30),
                (ItemKind::Torch, Rarity::Common, 10),
                (ItemKind::Bomb, Rarity::Uncommon, 50),
                (ItemKind::ScrollTeleport, Rarity::Rare, 100),
            ]),
            quests: vec![],
            friendly: true,
        }
    }

    pub fn healer(x: usize, y: usize) -> Self {
        Self {
            x, y,
            name: "Temple Healer".into(),
            npc_type: NPCType::Healer,
            dialogue_tree: DialogueTree {
                nodes: vec![
                    DialogueNode {
                        id: 0,
                        speaker: "Healer".into(),
                        text: "Blessings upon you. Do you need healing? (50 gold)".into(),
                        responses: vec![
                            DialogueResponse {
                                text: "Yes, please heal me.".into(),
                                next_node: Some(1),
                                action: Some(DialogueAction::Heal),
                                condition: Some(DialogueCondition::HasGold(50)),
                            },
                            DialogueResponse {
                                text: "No thank you.".into(),
                                next_node: None,
                                action: None,
                                condition: None,
                            },
                        ],
                    },
                    DialogueNode {
                        id: 1,
                        speaker: "Healer".into(),
                        text: "May the light guide your path.".into(),
                        responses: vec![
                            DialogueResponse {
                                text: "Thank you.".into(),
                                next_node: None,
                                action: None,
                                condition: None,
                            },
                        ],
                    },
                ],
                current_node: 0,
            },
            shop_inventory: None,
            quests: vec![],
            friendly: true,
        }
    }

    pub fn blacksmith(x: usize, y: usize) -> Self {
        Self {
            x, y,
            name: "Master Blacksmith".into(),
            npc_type: NPCType::Blacksmith,
            dialogue_tree: DialogueTree {
                nodes: vec![
                    DialogueNode {
                        id: 0,
                        speaker: "Blacksmith".into(),
                        text: "Need some proper steel? I forge the finest weapons and armor!".into(),
                        responses: vec![
                            DialogueResponse {
                                text: "Show me your weapons.".into(),
                                next_node: None,
                                action: Some(DialogueAction::OpenShop),
                                condition: None,
                            },
                            DialogueResponse {
                                text: "Maybe later.".into(),
                                next_node: None,
                                action: None,
                                condition: None,
                            },
                        ],
                    },
                ],
                current_node: 0,
            },
            shop_inventory: Some(vec![
                (ItemKind::ShortSword, Rarity::Common, 50),
                (ItemKind::LongSword, Rarity::Uncommon, 120),
                (ItemKind::IronShield, Rarity::Common, 80),
                (ItemKind::ChainMail, Rarity::Common, 150),
                (ItemKind::IronHelm, Rarity::Common, 60),
                (ItemKind::IronGauntlets, Rarity::Common, 45),
                (ItemKind::IronBoots, Rarity::Common, 45),
            ]),
            quests: vec![],
            friendly: true,
        }
    }

    pub fn alchemist(x: usize, y: usize) -> Self {
        Self {
            x, y,
            name: "Eccentric Alchemist".into(),
            npc_type: NPCType::Alchemist,
            dialogue_tree: DialogueTree {
                nodes: vec![
                    DialogueNode {
                        id: 0,
                        speaker: "Alchemist".into(),
                        text: "Bubbles and brews! I have potions for every occasion!".into(),
                        responses: vec![
                            DialogueResponse {
                                text: "What potions do you have?".into(),
                                next_node: None,
                                action: Some(DialogueAction::OpenShop),
                                condition: None,
                            },
                            DialogueResponse {
                                text: "I'll pass for now.".into(),
                                next_node: None,
                                action: None,
                                condition: None,
                            },
                        ],
                    },
                ],
                current_node: 0,
            },
            shop_inventory: Some(vec![
                (ItemKind::HealthPotion, Rarity::Common, 20),
                (ItemKind::ManaPotion, Rarity::Common, 25),
                (ItemKind::StrengthPotion, Rarity::Uncommon, 60),
                (ItemKind::DefensePotion, Rarity::Uncommon, 60),
                (ItemKind::SpeedPotion, Rarity::Uncommon, 50),
                (ItemKind::PoisonResistPotion, Rarity::Common, 35),
                (ItemKind::RegenerationPotion, Rarity::Rare, 100),
            ]),
            quests: vec![],
            friendly: true,
        }
    }

    pub fn sage(x: usize, y: usize, level: u32) -> Self {
        let lore = match level {
            1..=4 => "These dungeons were once the cellars of an ancient castle. Beware the Goblin King on level 5!",
            5..=8 => "The caves twist deep into the earth. Something ancient stirs below...",
            9..=12 => "This crypt holds the remains of a forgotten civilization. The dead do not rest easy here.",
            13..=16 => "The cursed forest grows even underground. Nature itself has turned hostile.",
            17..=20 => "The Frost Giant's domain. Fire magic will serve you well here.",
            21..=24 => "Volcanic depths... demons draw power from these flames.",
            25..=28 => "Ancient ruins of a lost empire. Their guardians still protect these halls.",
            _ => "The Demon Realm awaits. Only the strongest survive here.",
        };

        Self {
            x, y,
            name: "Wandering Sage".into(),
            npc_type: NPCType::Sage,
            dialogue_tree: DialogueTree {
                nodes: vec![
                    DialogueNode {
                        id: 0,
                        speaker: "Sage".into(),
                        text: format!("Greetings, adventurer. {}", lore),
                        responses: vec![
                            DialogueResponse {
                                text: "Thank you for the wisdom.".into(),
                                next_node: None,
                                action: None,
                                condition: None,
                            },
                            DialogueResponse {
                                text: "Any other advice?".into(),
                                next_node: Some(1),
                                action: None,
                                condition: None,
                            },
                        ],
                    },
                    DialogueNode {
                        id: 1,
                        speaker: "Sage".into(),
                        text: "Remember: press TAB to cycle skills, SPACE to use them. Explore carefully and conserve resources.".into(),
                        responses: vec![
                            DialogueResponse {
                                text: "I'll remember that.".into(),
                                next_node: None,
                                action: None,
                                condition: None,
                            },
                        ],
                    },
                ],
                current_node: 0,
            },
            shop_inventory: None,
            quests: vec![],
            friendly: true,
        }
    }

    pub fn guard(x: usize, y: usize) -> Self {
        Self {
            x, y,
            name: "Dungeon Guard".into(),
            npc_type: NPCType::Guard,
            dialogue_tree: DialogueTree {
                nodes: vec![
                    DialogueNode {
                        id: 0,
                        speaker: "Guard".into(),
                        text: "Halt! This area is dangerous. Are you sure you want to proceed?".into(),
                        responses: vec![
                            DialogueResponse {
                                text: "I can handle myself.".into(),
                                next_node: Some(1),
                                action: None,
                                condition: None,
                            },
                            DialogueResponse {
                                text: "What lurks ahead?".into(),
                                next_node: Some(2),
                                action: None,
                                condition: None,
                            },
                        ],
                    },
                    DialogueNode {
                        id: 1,
                        speaker: "Guard".into(),
                        text: "Very well. May fortune favor you, warrior.".into(),
                        responses: vec![
                            DialogueResponse {
                                text: "Thanks.".into(),
                                next_node: None,
                                action: None,
                                condition: None,
                            },
                        ],
                    },
                    DialogueNode {
                        id: 2,
                        speaker: "Guard".into(),
                        text: "Monsters grow stronger the deeper you go. Stock up on supplies before venturing forth.".into(),
                        responses: vec![
                            DialogueResponse {
                                text: "I understand.".into(),
                                next_node: None,
                                action: None,
                                condition: None,
                            },
                        ],
                    },
                ],
                current_node: 0,
            },
            shop_inventory: None,
            quests: vec![],
            friendly: true,
        }
    }

    pub fn enchanter(x: usize, y: usize) -> Self {
        Self {
            x, y,
            name: "Mystic Enchanter".into(),
            npc_type: NPCType::Enchanter,
            dialogue_tree: DialogueTree {
                nodes: vec![
                    DialogueNode {
                        id: 0,
                        speaker: "Enchanter".into(),
                        text: "I deal in magical artifacts and scrolls of power...".into(),
                        responses: vec![
                            DialogueResponse {
                                text: "Show me your magical wares.".into(),
                                next_node: None,
                                action: Some(DialogueAction::OpenShop),
                                condition: None,
                            },
                            DialogueResponse {
                                text: "Not interested.".into(),
                                next_node: None,
                                action: None,
                                condition: None,
                            },
                        ],
                    },
                ],
                current_node: 0,
            },
            shop_inventory: Some(vec![
                (ItemKind::ScrollFireball, Rarity::Uncommon, 75),
                (ItemKind::ScrollIceStorm, Rarity::Uncommon, 75),
                (ItemKind::ScrollLightning, Rarity::Uncommon, 80),
                (ItemKind::ScrollTeleport, Rarity::Rare, 100),
                (ItemKind::ScrollMapping, Rarity::Rare, 120),
                (ItemKind::RingOfProtection, Rarity::Uncommon, 200),
                (ItemKind::RingOfMana, Rarity::Uncommon, 180),
            ]),
            quests: vec![],
            friendly: true,
        }
    }

    pub fn glyph(&self) -> char {
        match self.npc_type {
            NPCType::Merchant => '$',
            NPCType::QuestGiver => '!',
            NPCType::Trainer => 'T',
            NPCType::Blacksmith => 'B',
            NPCType::Alchemist => 'A',
            NPCType::Enchanter => 'E',
            NPCType::Healer => '+',
            NPCType::Sage => 'S',
            NPCType::Guard => 'G',
            NPCType::Prisoner => 'P',
            NPCType::Companion => '@',
        }
    }

    pub fn color(&self) -> Color {
        if self.friendly { Color::Cyan } else { Color::Red }
    }
}
