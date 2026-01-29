//! Skill trees system - character progression paths
//! Stub module for compilation

use serde::{Deserialize, Serialize};

/// Skill tree categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillTreeType {
    Combat,
    Magic,
    Survival,
    Crafting,
    Social,
}

impl SkillTreeType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Combat => "Combat",
            Self::Magic => "Magic",
            Self::Survival => "Survival",
            Self::Crafting => "Crafting",
            Self::Social => "Social",
        }
    }
}

/// A skill node in the tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillNode {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub tree: SkillTreeType,
    pub tier: u32,
    pub cost: u32,
    pub prerequisites: Vec<u32>,
    pub unlocked: bool,
}

impl SkillNode {
    pub fn new(id: u32, name: &str, tree: SkillTreeType, tier: u32) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: String::new(),
            tree,
            tier,
            cost: tier,
            prerequisites: Vec::new(),
            unlocked: false,
        }
    }
}

/// Player skill tree state
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SkillTreeState {
    pub skill_points: u32,
    pub unlocked_skills: Vec<u32>,
}

impl SkillTreeState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn has_skill(&self, skill_id: u32) -> bool {
        self.unlocked_skills.contains(&skill_id)
    }

    pub fn can_unlock(&self, node: &SkillNode) -> bool {
        if self.has_skill(node.id) {
            return false;
        }
        if self.skill_points < node.cost {
            return false;
        }
        node.prerequisites.iter().all(|prereq| self.has_skill(*prereq))
    }

    pub fn unlock(&mut self, node: &mut SkillNode) -> bool {
        if self.can_unlock(node) {
            self.skill_points -= node.cost;
            self.unlocked_skills.push(node.id);
            node.unlocked = true;
            true
        } else {
            false
        }
    }

    pub fn add_points(&mut self, points: u32) {
        self.skill_points += points;
    }
}
