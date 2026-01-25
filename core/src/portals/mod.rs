//! Portal and Dimension System
//!
//! Travel between dimensions, discover portal networks, unlock secret realms.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Portal types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PortalType {
    /// Standard dungeon portal
    Dungeon,
    /// Town portal (return home)
    Town,
    /// Waypoint (saved location)
    Waypoint,
    /// Boss portal
    Boss,
    /// Secret realm portal
    SecretRealm,
    /// Dimensional rift
    Rift,
    /// Ancient gateway
    AncientGate,
    /// Demonic portal
    DemonicGate,
    /// Celestial portal
    CelestialGate,
    /// Void portal
    VoidGate,
}

impl PortalType {
    pub fn glyph(&self) -> char {
        match self {
            Self::Dungeon => 'O',
            Self::Town => 'T',
            Self::Waypoint => 'W',
            Self::Boss => 'B',
            Self::SecretRealm => 'S',
            Self::Rift => 'R',
            Self::AncientGate => 'A',
            Self::DemonicGate => 'D',
            Self::CelestialGate => 'C',
            Self::VoidGate => 'V',
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Dungeon => "Dungeon Portal",
            Self::Town => "Town Portal",
            Self::Waypoint => "Waypoint",
            Self::Boss => "Boss Portal",
            Self::SecretRealm => "Secret Realm Gate",
            Self::Rift => "Dimensional Rift",
            Self::AncientGate => "Ancient Gateway",
            Self::DemonicGate => "Demonic Portal",
            Self::CelestialGate => "Celestial Gate",
            Self::VoidGate => "Void Portal",
        }
    }

    pub fn requires_key(&self) -> bool {
        matches!(self, Self::Boss | Self::SecretRealm | Self::AncientGate)
    }
}

/// A portal in the world
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Portal {
    pub id: u32,
    pub portal_type: PortalType,
    pub x: usize,
    pub y: usize,
    pub destination: PortalDestination,
    pub active: bool,
    pub locked: bool,
    pub key_item: Option<String>,
    pub one_way: bool,
    pub uses: Option<u32>,
    pub discovered: bool,
}

impl Portal {
    pub fn new(id: u32, portal_type: PortalType, x: usize, y: usize, destination: PortalDestination) -> Self {
        Self {
            id,
            portal_type,
            x,
            y,
            destination,
            active: true,
            locked: portal_type.requires_key(),
            key_item: if portal_type.requires_key() { Some("Portal Key".to_string()) } else { None },
            one_way: false,
            uses: None,
            discovered: false,
        }
    }

    pub fn can_use(&self) -> bool {
        self.active && !self.locked && self.discovered
    }

    pub fn use_portal(&mut self) -> bool {
        if !self.can_use() {
            return false;
        }
        if let Some(uses) = &mut self.uses {
            *uses -= 1;
            if *uses == 0 {
                self.active = false;
            }
        }
        true
    }

    pub fn unlock(&mut self, key: &str) -> bool {
        if let Some(ref required_key) = self.key_item {
            if key == required_key {
                self.locked = false;
                return true;
            }
        }
        false
    }
}

/// Portal destinations
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PortalDestination {
    /// Go to specific dungeon floor
    Floor(u32),
    /// Go to a specific dimension
    Dimension(Dimension),
    /// Go to saved waypoint
    Waypoint(u32),
    /// Go to town
    Town,
    /// Go to boss arena
    BossArena(String),
    /// Random destination
    Random,
    /// Linked portal pair
    LinkedPortal(u32),
}

/// Alternate dimensions
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Dimension {
    /// Normal world
    Material,
    /// Shadow realm
    Shadow,
    /// Elemental plane of fire
    FirePlane,
    /// Elemental plane of ice
    IcePlane,
    /// Elemental plane of earth
    EarthPlane,
    /// Elemental plane of air
    AirPlane,
    /// The void between worlds
    Void,
    /// Celestial realm
    Celestial,
    /// Demonic realm
    Infernal,
    /// Realm of the dead
    Necrotic,
    /// Dream realm
    Dream,
    /// Mirror dimension
    Mirror,
    /// Pocket dimension
    Pocket,
    /// Ancient ruins dimension
    AncientRealm,
}

impl Dimension {
    pub fn all() -> &'static [Dimension] {
        &[
            Self::Material, Self::Shadow, Self::FirePlane, Self::IcePlane,
            Self::EarthPlane, Self::AirPlane, Self::Void, Self::Celestial,
            Self::Infernal, Self::Necrotic, Self::Dream, Self::Mirror,
            Self::Pocket, Self::AncientRealm,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Material => "Material Plane",
            Self::Shadow => "Shadow Realm",
            Self::FirePlane => "Plane of Fire",
            Self::IcePlane => "Plane of Ice",
            Self::EarthPlane => "Plane of Earth",
            Self::AirPlane => "Plane of Air",
            Self::Void => "The Void",
            Self::Celestial => "Celestial Realm",
            Self::Infernal => "Infernal Depths",
            Self::Necrotic => "Realm of the Dead",
            Self::Dream => "Dream Realm",
            Self::Mirror => "Mirror Dimension",
            Self::Pocket => "Pocket Dimension",
            Self::AncientRealm => "Ancient Realm",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Material => "The normal world where mortals dwell.",
            Self::Shadow => "A dark reflection of reality.",
            Self::FirePlane => "A realm of eternal flame.",
            Self::IcePlane => "A frozen wasteland of ice.",
            Self::EarthPlane => "Endless caves and stone.",
            Self::AirPlane => "Floating islands in endless sky.",
            Self::Void => "The empty space between worlds.",
            Self::Celestial => "Home of the divine beings.",
            Self::Infernal => "The realm of demons.",
            Self::Necrotic => "Where the dead wander eternally.",
            Self::Dream => "A realm shaped by thoughts.",
            Self::Mirror => "Everything is reversed here.",
            Self::Pocket => "A small personal dimension.",
            Self::AncientRealm => "Ruins of a forgotten age.",
        }
    }

    pub fn danger_level(&self) -> u32 {
        match self {
            Self::Material => 1,
            Self::Shadow => 3,
            Self::FirePlane | Self::IcePlane | Self::EarthPlane | Self::AirPlane => 4,
            Self::Celestial => 5,
            Self::Infernal | Self::Necrotic => 6,
            Self::Void => 7,
            Self::Dream | Self::Mirror => 4,
            Self::Pocket => 2,
            Self::AncientRealm => 5,
        }
    }

    pub fn modifiers(&self) -> DimensionModifiers {
        match self {
            Self::Material => DimensionModifiers::default(),
            Self::Shadow => DimensionModifiers {
                visibility: -5, enemy_damage: 1.2, loot_bonus: 1.1, ..Default::default()
            },
            Self::FirePlane => DimensionModifiers {
                fire_damage: 2.0, cold_damage: 0.5, fire_resist: -0.5, ..Default::default()
            },
            Self::IcePlane => DimensionModifiers {
                cold_damage: 2.0, fire_damage: 0.5, cold_resist: -0.5, speed_mod: 0.8, ..Default::default()
            },
            Self::Void => DimensionModifiers {
                visibility: -10, enemy_damage: 1.5, loot_bonus: 2.0, mana_regen: 0.5, ..Default::default()
            },
            Self::Celestial => DimensionModifiers {
                holy_damage: 1.5, dark_damage: 0.5, healing_bonus: 1.5, ..Default::default()
            },
            Self::Infernal => DimensionModifiers {
                dark_damage: 1.5, holy_damage: 0.5, fire_damage: 1.3, ..Default::default()
            },
            _ => DimensionModifiers::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct DimensionModifiers {
    pub visibility: i32,
    pub enemy_damage: f32,
    pub loot_bonus: f32,
    pub xp_bonus: f32,
    pub fire_damage: f32,
    pub cold_damage: f32,
    pub holy_damage: f32,
    pub dark_damage: f32,
    pub fire_resist: f32,
    pub cold_resist: f32,
    pub speed_mod: f32,
    pub mana_regen: f32,
    pub healing_bonus: f32,
}

impl Default for DimensionModifiers {
    fn default() -> Self {
        Self {
            visibility: 0,
            enemy_damage: 1.0,
            loot_bonus: 1.0,
            xp_bonus: 1.0,
            fire_damage: 1.0,
            cold_damage: 1.0,
            holy_damage: 1.0,
            dark_damage: 1.0,
            fire_resist: 0.0,
            cold_resist: 0.0,
            speed_mod: 1.0,
            mana_regen: 1.0,
            healing_bonus: 1.0,
        }
    }
}

/// Waypoint for fast travel
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Waypoint {
    pub id: u32,
    pub name: String,
    pub dimension: Dimension,
    pub floor: u32,
    pub x: usize,
    pub y: usize,
    pub discovered: bool,
}

/// Portal network manager
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PortalSystem {
    pub portals: HashMap<u32, Portal>,
    pub waypoints: HashMap<u32, Waypoint>,
    pub current_dimension: Dimension,
    pub discovered_dimensions: Vec<Dimension>,
    pub next_portal_id: u32,
    pub next_waypoint_id: u32,
    pub town_portal_active: bool,
    pub town_portal_return: Option<(Dimension, u32, usize, usize)>,
}

impl PortalSystem {
    pub fn new() -> Self {
        Self {
            current_dimension: Dimension::Material,
            discovered_dimensions: vec![Dimension::Material],
            ..Default::default()
        }
    }

    pub fn add_portal(&mut self, portal_type: PortalType, x: usize, y: usize, destination: PortalDestination) -> u32 {
        let id = self.next_portal_id;
        self.next_portal_id += 1;
        self.portals.insert(id, Portal::new(id, portal_type, x, y, destination));
        id
    }

    pub fn discover_portal(&mut self, id: u32) {
        if let Some(portal) = self.portals.get_mut(&id) {
            portal.discovered = true;
        }
    }

    pub fn add_waypoint(&mut self, name: &str, dimension: Dimension, floor: u32, x: usize, y: usize) -> u32 {
        let id = self.next_waypoint_id;
        self.next_waypoint_id += 1;
        self.waypoints.insert(id, Waypoint {
            id,
            name: name.to_string(),
            dimension,
            floor,
            x,
            y,
            discovered: true,
        });
        id
    }

    pub fn discover_dimension(&mut self, dimension: Dimension) {
        if !self.discovered_dimensions.contains(&dimension) {
            self.discovered_dimensions.push(dimension);
        }
    }

    pub fn travel_to_dimension(&mut self, dimension: Dimension) -> bool {
        if self.discovered_dimensions.contains(&dimension) {
            self.current_dimension = dimension;
            true
        } else {
            false
        }
    }

    pub fn create_town_portal(&mut self, dimension: Dimension, floor: u32, x: usize, y: usize) {
        self.town_portal_active = true;
        self.town_portal_return = Some((dimension, floor, x, y));
    }

    pub fn use_town_portal(&mut self) -> Option<(Dimension, u32, usize, usize)> {
        if self.town_portal_active {
            self.town_portal_active = false;
            self.town_portal_return.take()
        } else {
            None
        }
    }

    pub fn dimension_modifiers(&self) -> DimensionModifiers {
        self.current_dimension.modifiers()
    }
}
