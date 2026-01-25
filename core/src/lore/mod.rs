//! Lore and Codex System for ShadowCrypt
//!
//! This module provides a comprehensive lore system including:
//! - Discoverable lore entries
//! - Monster bestiary with descriptions
//! - Item histories and legends
//! - World backstory and mythology

use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use crate::entities::EnemyKind;
use crate::items::ItemKind;

/// Categories of lore entries
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum LoreCategory {
    /// World history and mythology
    WorldHistory,
    /// Bestiary - monster descriptions
    Bestiary,
    /// Item legends and histories
    ItemLore,
    /// Location descriptions
    Locations,
    /// Character and faction lore
    Factions,
    /// Ancient prophecies and legends
    Prophecies,
}

impl LoreCategory {
    /// Returns the display name of this category
    pub fn name(&self) -> &'static str {
        match self {
            Self::WorldHistory => "World History",
            Self::Bestiary => "Bestiary",
            Self::ItemLore => "Artifacts & Items",
            Self::Locations => "Locations",
            Self::Factions => "Factions & Orders",
            Self::Prophecies => "Prophecies",
        }
    }

    /// Returns a description of this category
    pub fn description(&self) -> &'static str {
        match self {
            Self::WorldHistory => "Chronicles of the world's creation and major historical events.",
            Self::Bestiary => "Knowledge of the creatures that dwell within the Shadow Crypts.",
            Self::ItemLore => "Legends and histories of powerful artifacts and magical items.",
            Self::Locations => "Descriptions of the various regions within the Shadow Crypts.",
            Self::Factions => "Information about the factions, orders, and notable figures.",
            Self::Prophecies => "Ancient prophecies and mystical predictions.",
        }
    }
}

/// A unique identifier for a lore entry
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum LoreEntryId {
    // World History entries (1-20)
    WorldCreation,
    TheFirstAge,
    TheShadowfall,
    TheSealing,
    TheProphecy,
    AgeOfHeroes,
    TheCataclysm,
    TheAwakening,
    ShadowCryptOrigins,
    TheForgottenKings,

    // Location entries (21-40)
    LocationDungeon,
    LocationCaves,
    LocationCrypt,
    LocationForest,
    LocationIceCavern,
    LocationVolcanic,
    LocationAncientRuins,
    LocationDemonRealm,
    LocationThrone,
    LocationHiddenSanctuary,

    // Faction entries (41-60)
    FactionShadowCult,
    FactionKnightsOfDawn,
    FactionAncientMages,
    FactionDemonLords,
    FactionForestGuardians,
    FactionFrostGiants,
    FactionUndeadLegion,
    FactionGoblinTribes,
    FactionDragonkin,
    FactionLostSouls,

    // Prophecy entries (61-70)
    ProphecyChosenOne,
    ProphecyEternalDarkness,
    ProphecyDemonReturn,
    ProphecyFinalBattle,
    ProphecyBalance,

    // Monster entries (71-150) - linked to EnemyKind
    MonsterEntry(EnemyKind),

    // Item entries (151-250) - linked to ItemKind
    ItemEntry(ItemKind),
}

/// A single lore entry containing text and metadata
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LoreEntry {
    pub id: LoreEntryId,
    pub category: LoreCategory,
    pub title: String,
    pub content: String,
    pub discovery_hint: Option<String>,
}

impl LoreEntry {
    /// Create a new lore entry
    pub fn new(
        id: LoreEntryId,
        category: LoreCategory,
        title: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            id,
            category,
            title: title.into(),
            content: content.into(),
            discovery_hint: None,
        }
    }

    /// Create a new lore entry with a discovery hint
    pub fn with_hint(
        id: LoreEntryId,
        category: LoreCategory,
        title: impl Into<String>,
        content: impl Into<String>,
        hint: impl Into<String>,
    ) -> Self {
        Self {
            id,
            category,
            title: title.into(),
            content: content.into(),
            discovery_hint: Some(hint.into()),
        }
    }
}

/// The player's codex tracking discovered lore
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Codex {
    /// Set of discovered lore entry IDs
    discovered: HashSet<LoreEntryId>,
    /// Monsters the player has encountered
    encountered_monsters: HashSet<EnemyKind>,
    /// Items the player has found
    found_items: HashSet<ItemKind>,
    /// Total discoveries made
    pub total_discoveries: u32,
}

impl Codex {
    /// Create a new empty codex
    pub fn new() -> Self {
        Self::default()
    }

    /// Discover a lore entry, returns true if newly discovered
    pub fn discover(&mut self, id: LoreEntryId) -> bool {
        if self.discovered.insert(id) {
            self.total_discoveries += 1;
            true
        } else {
            false
        }
    }

    /// Check if an entry has been discovered
    pub fn is_discovered(&self, id: &LoreEntryId) -> bool {
        self.discovered.contains(id)
    }

    /// Record encountering a monster, auto-discovers bestiary entry
    pub fn encounter_monster(&mut self, kind: EnemyKind) -> bool {
        if self.encountered_monsters.insert(kind) {
            self.discover(LoreEntryId::MonsterEntry(kind));
            true
        } else {
            false
        }
    }

    /// Record finding an item, auto-discovers item lore entry
    pub fn find_item(&mut self, kind: ItemKind) -> bool {
        if self.found_items.insert(kind) {
            self.discover(LoreEntryId::ItemEntry(kind));
            true
        } else {
            false
        }
    }

    /// Get the number of discovered entries in a category
    pub fn count_in_category(&self, category: LoreCategory) -> usize {
        self.discovered
            .iter()
            .filter(|id| get_lore_entry(id).map_or(false, |e| e.category == category))
            .count()
    }

    /// Get total entries in a category
    pub fn total_in_category(category: LoreCategory) -> usize {
        ALL_LORE_ENTRIES
            .iter()
            .filter(|e| e.category == category)
            .count()
    }

    /// Get all discovered entries
    pub fn get_discovered_entries(&self) -> Vec<&'static LoreEntry> {
        self.discovered
            .iter()
            .filter_map(get_lore_entry)
            .collect()
    }

    /// Get discovered entries in a category
    pub fn get_discovered_in_category(&self, category: LoreCategory) -> Vec<&'static LoreEntry> {
        self.discovered
            .iter()
            .filter_map(get_lore_entry)
            .filter(|e| e.category == category)
            .collect()
    }

    /// Get completion percentage
    pub fn completion_percentage(&self) -> f32 {
        let total = ALL_LORE_ENTRIES.len();
        if total == 0 {
            return 100.0;
        }
        (self.discovered.len() as f32 / total as f32) * 100.0
    }

    /// Get number of monsters encountered
    pub fn monsters_encountered(&self) -> usize {
        self.encountered_monsters.len()
    }

    /// Get number of items found
    pub fn items_found(&self) -> usize {
        self.found_items.len()
    }
}

/// Get a lore entry by ID
pub fn get_lore_entry(id: &LoreEntryId) -> Option<&'static LoreEntry> {
    ALL_LORE_ENTRIES.iter().find(|e| &e.id == id)
}

/// Get monster lore for a specific enemy kind
pub fn get_monster_lore(kind: EnemyKind) -> &'static MonsterLore {
    MONSTER_LORE.iter()
        .find(|m| m.kind == kind)
        .unwrap_or(&DEFAULT_MONSTER_LORE)
}

/// Get item lore for a specific item kind
pub fn get_item_lore(kind: ItemKind) -> &'static ItemLoreEntry {
    ITEM_LORE.iter()
        .find(|i| i.kind == kind)
        .unwrap_or(&DEFAULT_ITEM_LORE)
}

// ============================================================================
// MONSTER BESTIARY LORE
// ============================================================================

/// Detailed monster lore entry
#[derive(Clone, Debug)]
pub struct MonsterLore {
    pub kind: EnemyKind,
    pub description: &'static str,
    pub origin: &'static str,
    pub behavior: &'static str,
    pub weakness: Option<&'static str>,
    pub legend: Option<&'static str>,
}

static DEFAULT_MONSTER_LORE: MonsterLore = MonsterLore {
    kind: EnemyKind::Rat,
    description: "A mysterious creature of unknown origin.",
    origin: "Little is known about this entity.",
    behavior: "Its behavior patterns remain unstudied.",
    weakness: None,
    legend: None,
};

/// All monster lore entries
pub static MONSTER_LORE: &[MonsterLore] = &[
    // ========== TIER 1: DUNGEON CREATURES ==========
    MonsterLore {
        kind: EnemyKind::Rat,
        description: "Common vermin that infest the upper dungeon levels. Though individually weak, they carry diseases and often attack in swarms.",
        origin: "These rats have lived in the Shadow Crypts for generations, feeding on the refuse of darker creatures and absorbing trace amounts of shadow magic.",
        behavior: "Cowardly alone but emboldened in groups. They flee from fire and bright light.",
        weakness: Some("Fire and area attacks"),
        legend: Some("Old adventurers speak of the Rat King, a massive rodent said to command all vermin in the crypts."),
    },
    MonsterLore {
        kind: EnemyKind::Bat,
        description: "Cave-dwelling creatures whose high-pitched shrieks echo through the dungeon corridors. Their bite can transmit the Shadow Fever.",
        origin: "Descended from ordinary bats that nested in the crypts before the Shadowfall. Centuries of exposure have made them larger and more aggressive.",
        behavior: "Hunts using echolocation. Attracted to movement and warmth.",
        weakness: Some("Sensitive to loud sounds and bright flashes"),
        legend: None,
    },
    MonsterLore {
        kind: EnemyKind::Spider,
        description: "Web-spinning arachnids that trap unwary adventurers in their sticky silk. Their venom causes paralysis.",
        origin: "Mutated by shadow magic, these spiders have grown far beyond their natural size. Some scholars believe they serve a greater Spider Queen deep within the crypts.",
        behavior: "Patient ambush predators. They weave elaborate trap networks and wait for prey.",
        weakness: Some("Fire destroys their webs and they fear it greatly"),
        legend: Some("The Weaver's Children, as they're called, are said to remember every creature that escapes their webs."),
    },
    MonsterLore {
        kind: EnemyKind::Goblin,
        description: "Small, cunning humanoids with green skin and sharp teeth. They are the most numerous intelligent species in the upper crypts.",
        origin: "Goblins were driven underground during the Age of Heroes and have since formed a complex tribal society within the Shadow Crypts.",
        behavior: "Cowardly individually but dangerous in groups. They use crude traps and ambush tactics.",
        weakness: Some("Weak to organized opposition and break formation when their leaders fall"),
        legend: Some("The goblins speak of a prophecy where their king will one day conquer the surface world."),
    },
    MonsterLore {
        kind: EnemyKind::Skeleton,
        description: "Animated bones of fallen adventurers, held together by necromantic energy. They feel no pain and never tire.",
        origin: "Created when the death energy of the crypts reanimates corpses. The longer a body lies in the shadow, the more likely it rises.",
        behavior: "Follows simple commands from their creator or patrols endlessly if masterless.",
        weakness: Some("Holy magic and blunt weapons that shatter their bones"),
        legend: Some("Some skeletons retain fragments of memory from their past lives, occasionally speaking names or phrases."),
    },
    MonsterLore {
        kind: EnemyKind::Kobold,
        description: "Small reptilian creatures distantly related to dragons. They are cunning trap-makers and surprisingly skilled miners.",
        origin: "Kobolds claim descent from the great dragons and serve them fanatically. They came to the crypts seeking dragon artifacts.",
        behavior: "Extremely territorial. They set elaborate traps and attack intruders with overwhelming numbers.",
        weakness: Some("Bright light hurts their sensitive eyes"),
        legend: Some("Kobolds believe that serving dragons will one day grant them wings and breath weapons."),
    },
    MonsterLore {
        kind: EnemyKind::GiantRat,
        description: "Enormous rodents the size of large dogs. Their teeth can gnaw through armor and their bites cause festering wounds.",
        origin: "The largest and most aggressive rats grow to enormous size by consuming shadow-tainted flesh.",
        behavior: "More aggressive than their smaller kin. They lead rat swarms and defend territory fiercely.",
        weakness: Some("Fire and poisons"),
        legend: None,
    },
    MonsterLore {
        kind: EnemyKind::CaveCrawler,
        description: "Multi-legged insectoid creatures that cling to walls and ceilings, dropping on unsuspecting prey.",
        origin: "Native to the deepest caves, they've spread throughout the crypts following the trails of other creatures.",
        behavior: "Ambush predators that prefer to attack from above. They're attracted to the scent of blood.",
        weakness: Some("Vulnerable underbelly when flipped"),
        legend: None,
    },

    // ========== TIER 2: CAVE DWELLERS ==========
    MonsterLore {
        kind: EnemyKind::GiantSpider,
        description: "Massive arachnids whose webs can span entire caverns. A single bite delivers enough venom to kill a horse.",
        origin: "The matriarchs of spider colonies, grown enormous on a diet of larger prey and shadow essence.",
        behavior: "Creates vast web networks and waits at the center. Can sense vibrations through their webs from great distances.",
        weakness: Some("Fire and severing their connection to their web"),
        legend: Some("The Spider Queen Arachnia is said to be the mother of all giant spiders, dwelling in a realm of endless webs."),
    },
    MonsterLore {
        kind: EnemyKind::Orc,
        description: "Brutal, tusked warriors who value strength above all else. They forge crude but effective weapons.",
        origin: "Orcish raiders were among the first to explore the Shadow Crypts, seeking treasure. Many tribes now call it home.",
        behavior: "Aggressive and territorial. They respect only strength and will follow whoever defeats their leader.",
        weakness: Some("Disorganized without strong leadership"),
        legend: Some("The Orc Warlord Grommash once united all the tribes. His descendants still fight over his legacy."),
    },
    MonsterLore {
        kind: EnemyKind::Troll,
        description: "Towering creatures with rubbery green skin that regenerates from almost any wound. Only fire stops their healing.",
        origin: "Trolls are ancient beings that predate human civilization. They retreated underground when humans spread across the land.",
        behavior: "Extremely territorial and always hungry. They'll eat anything that moves and much that doesn't.",
        weakness: Some("Fire and acid prevent regeneration"),
        legend: Some("It's said that cutting a troll in half will eventually result in two trolls if both pieces aren't burned."),
    },
    MonsterLore {
        kind: EnemyKind::CaveOgre,
        description: "Massive humanoids with thick skin and tremendous strength. They're not intelligent but are extremely dangerous.",
        origin: "Ogres were bred as war beasts by ancient sorcerers. When their masters fell, they went feral.",
        behavior: "Simple-minded but cunning hunters. They use clubs and thrown rocks as weapons.",
        weakness: Some("Easily outsmarted and lured into traps"),
        legend: None,
    },
    MonsterLore {
        kind: EnemyKind::Slime,
        description: "Amorphous masses of corrosive gel that dissolve organic matter. They split when struck with physical weapons.",
        origin: "Slimes spontaneously generate in places saturated with magical waste. The crypts produce them endlessly.",
        behavior: "Mindlessly seeks organic matter to consume. Flows through tiny gaps and reforms on the other side.",
        weakness: Some("Magic and elemental damage; physical attacks may cause them to split"),
        legend: Some("Alchemists prize slime essence for its dissolving properties."),
    },
    MonsterLore {
        kind: EnemyKind::Hobgoblin,
        description: "Larger, more disciplined cousins of goblins. They form organized military units and use sophisticated tactics.",
        origin: "Hobgoblins emerged when goblin tribes were exposed to greater concentrations of shadow magic over generations.",
        behavior: "Military discipline and tactical thinking. They coordinate attacks and protect their weaker kin.",
        weakness: Some("Breaking their formation disrupts their effectiveness"),
        legend: None,
    },
    MonsterLore {
        kind: EnemyKind::CaveBear,
        description: "Enormous bears adapted to life in the darkness. Their claws can tear through stone, and they're always hungry.",
        origin: "Bears that wandered into caves and adapted over countless generations. Shadow magic made them larger and more aggressive.",
        behavior: "Hibernates in hidden dens and awakens ravenously hungry. Fiercely protects its territory.",
        weakness: Some("Loud noises can startle them temporarily"),
        legend: None,
    },
    MonsterLore {
        kind: EnemyKind::Mushroom,
        description: "Ambulatory fungi that release clouds of toxic spores. They're drawn to decaying matter.",
        origin: "Fungi that developed mobility and primitive sentience from exposure to concentrated death magic.",
        behavior: "Slowly wanders seeking organic matter. Releases spores when threatened or damaged.",
        weakness: Some("Fire destroys them quickly"),
        legend: Some("Some adventurers claim the mushrooms communicate through their spore networks."),
    },
    MonsterLore {
        kind: EnemyKind::RockElemental,
        description: "Living stone animated by earth magic. They're nearly impervious to physical damage but slow-moving.",
        origin: "Formed when powerful earth magic saturates stone. The crypts' magical nature creates them spontaneously.",
        behavior: "Guards specific areas or mineral deposits. Can remain motionless for centuries until disturbed.",
        weakness: Some("Vulnerable to water magic and precision strikes at their core"),
        legend: Some("The Stone Heart that animates them can be used to create powerful artifacts."),
    },

    // ========== TIER 3: CRYPT UNDEAD ==========
    MonsterLore {
        kind: EnemyKind::Zombie,
        description: "Shambling corpses driven by an insatiable hunger for living flesh. They spread corruption wherever they walk.",
        origin: "Created when corpses are saturated with death energy. The crypts produce them in endless numbers.",
        behavior: "Relentlessly pursues the living. Feels no pain and continues until destroyed completely.",
        weakness: Some("Holy magic and destroying the brain"),
        legend: Some("Ancient texts speak of a first zombie, the Patient Zero who started the undead plague."),
    },
    MonsterLore {
        kind: EnemyKind::Ghost,
        description: "Spectral remnants of those who died with unfinished business. Their touch drains life force.",
        origin: "Souls trapped between worlds by violent death or powerful regret. The crypts are filled with such tragedies.",
        behavior: "Haunts the site of their death. Some seek to complete their final task; others simply rage at the living.",
        weakness: Some("Holy magic and salt can repel them; resolving their unfinished business releases them"),
        legend: Some("The Ghost of Lady Morrigan still searches for her lost children."),
    },
    MonsterLore {
        kind: EnemyKind::Wraith,
        description: "Malevolent spirits of pure hatred. Their very presence saps the will to live from nearby creatures.",
        origin: "Formed when ghosts are consumed by rage and despair, losing all connection to their former selves.",
        behavior: "Hunts the living with cold intelligence. Can phase through solid matter.",
        weakness: Some("Holy magic and blessed weapons; sunlight weakens them"),
        legend: Some("Wraiths are said to be drawn to those who share their anger."),
    },
    MonsterLore {
        kind: EnemyKind::Vampire,
        description: "Undead nobles who feed on blood to sustain their immortal existence. They possess supernatural strength, speed, and charm.",
        origin: "Created through a dark ritual or the bite of another vampire. They've built a hidden society within the crypts.",
        behavior: "Cunning predators who prefer to manipulate victims. They maintain a facade of nobility and sophistication.",
        weakness: Some("Sunlight, holy symbols, garlic, and stakes through the heart"),
        legend: Some("The First Vampire, Lord Sanguinus, is said to sleep in the deepest crypt, waiting for the eternal night."),
    },
    MonsterLore {
        kind: EnemyKind::Mummy,
        description: "Preserved corpses wrapped in enchanted bandages. They curse those who disturb their tombs.",
        origin: "Ancient kings and priests preserved through ritual. They were placed in the crypts as eternal guardians.",
        behavior: "Protects their tomb and treasures. Their curses can follow tomb robbers across the world.",
        weakness: Some("Fire and holy magic"),
        legend: Some("The Mummy Lords remember the world before the Shadowfall and guard secrets of that lost age."),
    },
    MonsterLore {
        kind: EnemyKind::Ghoul,
        description: "Undead creatures that feast on corpses. Their paralytic touch immobilizes prey for later consumption.",
        origin: "Created when cannibals or grave robbers die and rise again, cursed to eternally hunger for dead flesh.",
        behavior: "Lurks in graveyards and crypts. Hoards corpses in hidden larders.",
        weakness: Some("Holy magic and they flee from strong light"),
        legend: None,
    },
    MonsterLore {
        kind: EnemyKind::Banshee,
        description: "Wailing spirits of women who died in grief. Their scream can stop hearts and shatter minds.",
        origin: "Born from the deaths of those who experienced profound loss. Their eternal mourning echoes through the crypts.",
        behavior: "Warns of impending death with her wail. Some say her scream is actually an attempt to save the living.",
        weakness: Some("Music and sounds of joy weaken them"),
        legend: Some("A banshee's tears, if collected, can heal any wound but are nearly impossible to obtain."),
    },
    MonsterLore {
        kind: EnemyKind::DeathKnight,
        description: "Fallen paladins raised as undead warriors. They retain their combat skills and are encased in cursed armor.",
        origin: "Knights who broke their sacred oaths and were condemned to eternal service in undeath.",
        behavior: "Commands lesser undead. Seeks to corrupt other knights and spread their curse.",
        weakness: Some("Holy magic and weapons blessed by the order they betrayed"),
        legend: Some("Sir Aldric the Betrayer was the first Death Knight, and his armor is said to be indestructible."),
    },
    MonsterLore {
        kind: EnemyKind::BoneGolem,
        description: "A towering construct assembled from countless bones. It feels no pain and never tires.",
        origin: "Created by necromancers as tireless guardians. Some have outlived their creators by centuries.",
        behavior: "Follows the last orders given by its creator. Without orders, it guards the area where it was created.",
        weakness: Some("Holy magic and destroying the skull that serves as its core"),
        legend: None,
    },

    // ========== TIER 4: FOREST CREATURES ==========
    MonsterLore {
        kind: EnemyKind::Wolf,
        description: "Pack hunters with keen senses and coordinated tactics. They're more aggressive than surface wolves.",
        origin: "Wolves that followed prey into the underground forest realm and adapted to the darkness.",
        behavior: "Hunts in coordinated packs. One wolf distracts while others flank.",
        weakness: Some("Fire frightens them; killing the alpha scatters the pack"),
        legend: None,
    },
    MonsterLore {
        kind: EnemyKind::DireWolf,
        description: "Massive wolves the size of horses. Their howl can be heard for miles and strikes fear into prey.",
        origin: "Alpha wolves that consumed shadow essence and grew to enormous size. They lead great packs.",
        behavior: "Commands lesser wolves. More intelligent than normal wolves and can plan complex hunts.",
        weakness: Some("Fire and challenging their dominance"),
        legend: Some("The Great Wolf Fenris is said to be the ancestor of all dire wolves."),
    },
    MonsterLore {
        kind: EnemyKind::TreeEnt,
        description: "Ancient trees awakened to sentience. They protect the underground forest with terrible fury.",
        origin: "Trees that absorbed enough nature magic over millennia to achieve awareness. They remember the time before darkness.",
        behavior: "Slow to anger but terrible when provoked. They protect all plant life in their domain.",
        weakness: Some("Fire, though using it earns the enmity of all forest creatures"),
        legend: Some("The Elder Ent Yggdros is said to be older than the crypts themselves."),
    },
    MonsterLore {
        kind: EnemyKind::ForestTroll,
        description: "Trolls adapted to forest life. Their green skin provides camouflage and they're even more vicious than cave trolls.",
        origin: "Trolls that migrated to the underground forest and evolved to hunt its unique creatures.",
        behavior: "Ambush predator that uses the forest for cover. Extremely territorial.",
        weakness: Some("Fire prevents regeneration"),
        legend: None,
    },
    MonsterLore {
        kind: EnemyKind::Druid,
        description: "Once noble protectors of nature, corrupted by shadow magic. They command plants and animals.",
        origin: "Druids who ventured too deep seeking to heal the corruption and were instead consumed by it.",
        behavior: "Uses nature magic offensively. Commands forest creatures and can shape plants into weapons.",
        weakness: Some("Breaking their connection to their corrupted grove weakens them"),
        legend: Some("Some say the corrupted druids can be saved if the source of corruption is destroyed."),
    },
    MonsterLore {
        kind: EnemyKind::WildBoar,
        description: "Aggressive boars with razor-sharp tusks. They charge anything that enters their territory.",
        origin: "Boars that thrived in the underground forest, growing larger and more aggressive on shadow-tainted roots.",
        behavior: "Charges intruders on sight. Males are extremely territorial during mating season.",
        weakness: Some("Sidestepping their charge leaves them vulnerable"),
        legend: None,
    },
    MonsterLore {
        kind: EnemyKind::GiantWasp,
        description: "Horse-sized insects with venomous stingers. Their nests contain hundreds of workers.",
        origin: "Wasps mutated by magical pollution in the underground forest. Their hives are sources of magical honey.",
        behavior: "Fiercely protects their hive. Worker wasps will sacrifice themselves for the queen.",
        weakness: Some("Smoke disorients them; destroying the queen throws the hive into chaos"),
        legend: Some("Giant wasp honey has powerful healing properties but harvesting it is extremely dangerous."),
    },
    MonsterLore {
        kind: EnemyKind::VenomousVine,
        description: "Animated plants that constrict and poison their prey. They're nearly invisible until they strike.",
        origin: "Created when regular vines absorbed too much death essence from corpses left in the forest.",
        behavior: "Waits motionless until prey is within reach. Constricts while injecting paralyzing venom.",
        weakness: Some("Fire and cold; they can't move if their roots are damaged"),
        legend: None,
    },
    MonsterLore {
        kind: EnemyKind::ForestSpirit,
        description: "Ethereal beings that embody the forest's consciousness. They judge all who enter their domain.",
        origin: "Manifestations of the forest's collective will. They've grown angry as corruption spreads.",
        behavior: "Tests intruders with illusions and trials. Those who fail are led astray forever.",
        weakness: Some("Offerings to the forest may appease them"),
        legend: Some("The spirits remember when the forest covered the entire world."),
    },

    // ========== TIER 5: ICE CAVERN ==========
    MonsterLore {
        kind: EnemyKind::IceElemental,
        description: "Beings of pure frozen water and cold magic. Their touch causes frostbite and their presence drops temperatures dramatically.",
        origin: "Formed in places where cold magic concentrates. The ice caverns spawn them regularly.",
        behavior: "Patrols frozen areas and attacks sources of heat. Can merge with ice to ambush prey.",
        weakness: Some("Fire magic melts their form"),
        legend: Some("Ice elementals are said to be fragments of an ancient frost god."),
    },
    MonsterLore {
        kind: EnemyKind::FrostGiant,
        description: "Towering humanoids adapted to extreme cold. They wield massive ice weapons and can summon blizzards.",
        origin: "An ancient race that ruled during the Ice Age. They retreated to the frozen caverns when the world warmed.",
        behavior: "Proud warriors who consider smaller races beneath them. They hunt for sport.",
        weakness: Some("Fire and attacking their slow reaction time"),
        legend: Some("The Frost Giant King Thrym seeks a way to plunge the world into eternal winter."),
    },
    MonsterLore {
        kind: EnemyKind::YetiWarrior,
        description: "Massive ape-like creatures covered in white fur. They're intelligent and wield weapons made of ice and bone.",
        origin: "Native to the coldest mountain peaks, some migrated to the ice caverns following prey.",
        behavior: "Tribal warriors who protect their clan fiercely. They respect strength in combat.",
        weakness: Some("Fire and their protective instincts can be used against them"),
        legend: Some("Yetis supposedly know the location of the Frozen Throne, a seat of incredible power."),
    },
    MonsterLore {
        kind: EnemyKind::IceWraith,
        description: "Spirits of those who froze to death, now composed of bitter cold and malice.",
        origin: "Created when someone dies of cold while feeling intense hatred. The ice caverns are full of such tragedies.",
        behavior: "Drawn to warmth and life, which they seek to extinguish. Their touch freezes flesh instantly.",
        weakness: Some("Fire magic and sources of intense heat"),
        legend: None,
    },
    MonsterLore {
        kind: EnemyKind::FrostWolf,
        description: "Wolves with fur of pure white and breath of freezing mist. They hunt in the endless snow.",
        origin: "Wolves blessed (or cursed) by frost spirits. Their bite can freeze blood in veins.",
        behavior: "Pack hunters adapted to arctic conditions. They can track prey through blizzards.",
        weakness: Some("Fire frightens them"),
        legend: None,
    },
    MonsterLore {
        kind: EnemyKind::IceSpider,
        description: "Spiders that spin webs of frozen silk. Their venom causes both paralysis and hypothermia.",
        origin: "Spiders that adapted to the ice caverns, developing the ability to survive extreme cold.",
        behavior: "Weaves webs across frozen passages. Prey stuck in the webs slowly freeze to death.",
        weakness: Some("Fire melts their webs and they're vulnerable to heat"),
        legend: None,
    },
    MonsterLore {
        kind: EnemyKind::FrozenKnight,
        description: "Warriors entombed in enchanted ice. They're awakened by intruders and fight with relentless precision.",
        origin: "Knights who swore to guard the ice caverns forever. Their oath bound them even in death.",
        behavior: "Follows ancient patrol routes. Attacks any who enter the areas they protect.",
        weakness: Some("Fire can melt their frozen armor"),
        legend: Some("The Frozen Knights were once the honor guard of the Ice Dragon."),
    },
    MonsterLore {
        kind: EnemyKind::Wendigo,
        description: "Former humanoids transformed by consuming human flesh in the frozen wastes. They're eternally hungry.",
        origin: "Created when desperate travelers resort to cannibalism. The curse transforms them into monsters.",
        behavior: "Hunts intelligent prey exclusively. They can mimic voices to lure victims.",
        weakness: Some("Fire and silver weapons"),
        legend: Some("It's said that wendigos remember their former lives and seek out their families."),
    },

    // ========== TIER 6: VOLCANIC ==========
    MonsterLore {
        kind: EnemyKind::FireElemental,
        description: "Living flames that burn with incredible intensity. They leave trails of fire wherever they go.",
        origin: "Born in the hearts of volcanoes and places of intense fire magic. They serve the primal force of flame.",
        behavior: "Consumes anything flammable. Drawn to sources of fuel and heat.",
        weakness: Some("Water and ice magic can extinguish them"),
        legend: Some("Fire elementals are said to carry messages from the Flame Lords."),
    },
    MonsterLore {
        kind: EnemyKind::LavaGolem,
        description: "Massive constructs of molten rock. Their bodies radiate lethal heat and their strikes leave burning wounds.",
        origin: "Formed when earth magic and fire magic combine in volcanic chambers. Extremely rare and dangerous.",
        behavior: "Slow but relentless. Guards volcanic treasures and attacks anything that approaches.",
        weakness: Some("Water-based attacks cause them to cool and crack"),
        legend: Some("A lava golem's core contains solidified fire essence of immense magical value."),
    },
    MonsterLore {
        kind: EnemyKind::Hellhound,
        description: "Demonic dogs wreathed in flames. Their bites burn with infernal fire and they can track prey across any terrain.",
        origin: "Bred in the demon realm as hunting beasts. Some escaped or were summoned to guard volcanic passages.",
        behavior: "Loyal pack hunters. Once they have a scent, they never lose it.",
        weakness: Some("Holy water and ice magic"),
        legend: Some("Hellhound pups are sometimes raised as guardians, though the risk is immense."),
    },
    MonsterLore {
        kind: EnemyKind::FireDrake,
        description: "Lesser dragons that breathe gouts of flame. Though smaller than true dragons, they're still incredibly dangerous.",
        origin: "Descendants of true dragons, diminished through generations but still formidable.",
        behavior: "Territorial and greedy. They hoard treasures and attack any who approach.",
        weakness: Some("Their underbelly has weaker scales"),
        legend: Some("Fire drake blood can be used to forge weapons that never lose their edge."),
    },
    MonsterLore {
        kind: EnemyKind::MagmaSlime,
        description: "Slimes composed of molten rock. They leave burning trails and can engulf prey in their scorching mass.",
        origin: "Formed when regular slimes absorb too much heat and fire magic. They're nearly indestructible.",
        behavior: "Seeks organic matter to consume. Immune to most physical attacks.",
        weakness: Some("Water magic and cold-based attacks"),
        legend: None,
    },
    MonsterLore {
        kind: EnemyKind::Salamander,
        description: "Reptilian humanoids that thrive in extreme heat. They forge weapons in volcanic vents and serve the fire lords.",
        origin: "An ancient race born from primordial fire. They built civilizations in the volcanic depths.",
        behavior: "Skilled warriors and craftsmen. They consider lesser races as resources to be exploited.",
        weakness: Some("Cold magic and attacks on their thin limbs"),
        legend: Some("Salamander-forged weapons carry the heat of their creation and never cool."),
    },
    MonsterLore {
        kind: EnemyKind::CinderWraith,
        description: "The burning spirits of those who died in fire. They spread flames and seek to share their eternal burning.",
        origin: "Created when someone dies by fire while feeling intense anger. They can never escape the flames.",
        behavior: "Sets fires wherever it goes. Drawn to the living, whom it envies.",
        weakness: Some("Water and ice magic"),
        legend: Some("A cinder wraith can be put to rest if their remains are properly buried."),
    },
    MonsterLore {
        kind: EnemyKind::InfernalImp,
        description: "Small demons that delight in causing chaos. They throw fireballs and can teleport short distances.",
        origin: "The lowest rank of demons, often sent ahead as scouts or summoned as minor servants.",
        behavior: "Mischievous and cruel. They enjoy tormenting victims before killing them.",
        weakness: Some("Holy magic and trapping them prevents teleportation"),
        legend: Some("Imps sometimes bargain for their lives, but their deals always have hidden costs."),
    },

    // ========== TIER 7: ANCIENT RUINS ==========
    MonsterLore {
        kind: EnemyKind::Golem,
        description: "Massive stone constructs animated by ancient magic. They were built to guard the ruins forever.",
        origin: "Created by the ancient civilization that built the ruins. Their magic has kept them functioning for millennia.",
        behavior: "Follows ancient protocols. May recognize certain symbols or passwords.",
        weakness: Some("Destroying the magical core in their chest"),
        legend: Some("Golem cores can be reprogrammed by those who know the ancient command words."),
    },
    MonsterLore {
        kind: EnemyKind::AncientGuardian,
        description: "Elite constructs built to protect the most sacred areas. They wield weapons of pure magical force.",
        origin: "The finest works of ancient artificers, built to guard treasures and secrets for eternity.",
        behavior: "Intelligent defenders that adapt to intruders' tactics. They never leave their posts.",
        weakness: Some("Ancient command words can disable them temporarily"),
        legend: Some("The Guardians are said to remember their creators and mourn their passing."),
    },
    MonsterLore {
        kind: EnemyKind::Sphinx,
        description: "Enigmatic creatures with the body of a lion and a humanoid head. They guard knowledge with deadly riddles.",
        origin: "Created as guardians of wisdom by ancient mages. They test all who seek forbidden knowledge.",
        behavior: "Poses riddles to all who approach. Those who answer correctly may pass; those who fail are devoured.",
        weakness: Some("Bound by their nature to honor correct answers"),
        legend: Some("The Grand Sphinx knows the answer to any question but will only share it if asked correctly."),
    },
    MonsterLore {
        kind: EnemyKind::Lich,
        description: "Undead sorcerers who achieved immortality through dark rituals. They command vast magical power.",
        origin: "Mages who feared death so greatly they bound their souls to phylacteries, becoming undead.",
        behavior: "Schemes eternally in their lairs. Seeks magical power and knowledge above all else.",
        weakness: Some("Destroying their phylactery makes them vulnerable"),
        legend: Some("The Lich King Acererak was the first to discover the ritual of undeath."),
    },
    MonsterLore {
        kind: EnemyKind::Gargoyle,
        description: "Stone creatures that appear as statues until they strike. They can remain motionless for centuries.",
        origin: "Created as guardians for ancient buildings. They've outlived their creators many times over.",
        behavior: "Ambush predators that pretend to be statues. They attack when prey turns its back.",
        weakness: Some("Unable to move in direct sunlight; slow to react when first awakening"),
        legend: Some("Some gargoyles developed true intelligence and secretly watch mortal affairs."),
    },
    MonsterLore {
        kind: EnemyKind::MummyLord,
        description: "Ancient kings preserved through powerful rituals. They command armies of lesser undead and cast divine curses.",
        origin: "Pharaohs and high priests mummified with the most powerful preservation magic. They retain their royal power.",
        behavior: "Rules their tomb as they ruled in life. Demands worship and punishes desecration.",
        weakness: Some("Holy magic and destroying their canopic jars"),
        legend: Some("The Mummy Lords guard the secrets of the gods they served in life."),
    },
    MonsterLore {
        kind: EnemyKind::CursedStatue,
        description: "Animated statues inhabited by trapped souls. They attack with supernatural strength and feel no pain.",
        origin: "Created when criminals were sentenced to eternal imprisonment within stone forms.",
        behavior: "Attacks all living things it encounters. The trapped souls within scream silently.",
        weakness: Some("Breaking the statue releases and destroys the soul within"),
        legend: Some("Some say the trapped souls can be freed through specific rituals."),
    },
    MonsterLore {
        kind: EnemyKind::ShadowAssassin,
        description: "Killers who have merged with shadow itself. They can become invisible in darkness and strike without warning.",
        origin: "Members of an ancient assassin cult who underwent dark rituals to become living shadows.",
        behavior: "Patient killers who study their targets. They prefer to strike when victory is certain.",
        weakness: Some("Bright light reveals them and prevents their shadow-merge"),
        legend: Some("The Shadow Assassins serve a mysterious master who has never been seen."),
    },

    // ========== TIER 8: DEMON REALM ==========
    MonsterLore {
        kind: EnemyKind::Demon,
        description: "Creatures of pure malevolence from the infernal planes. They delight in corruption and destruction.",
        origin: "Born in the demon realm from concentrated evil. They seek to spread their influence to other worlds.",
        behavior: "Cruel and cunning. They prefer to corrupt mortals rather than kill them outright.",
        weakness: Some("Holy magic and blessed weapons"),
        legend: Some("Every demon was once something else before being transformed by the abyss."),
    },
    MonsterLore {
        kind: EnemyKind::DemonLord,
        description: "Powerful demon nobles who command legions. They possess vast magical power and political cunning.",
        origin: "Demons who accumulated enough power and followers to claim noble titles in the infernal hierarchy.",
        behavior: "Schemes constantly for more power. Views mortals as tools or toys.",
        weakness: Some("Their true name gives power over them"),
        legend: Some("Each Demon Lord covets the throne of the Demon King."),
    },
    MonsterLore {
        kind: EnemyKind::Succubus,
        description: "Demons who feed on life force through seduction. They appear beautiful but their true form is monstrous.",
        origin: "Created to corrupt mortals through their desires. They've destroyed kingdoms with whispered words.",
        behavior: "Seduces targets and drains their life force. Prefers manipulation to direct combat.",
        weakness: Some("Holy magic and true love can break their enchantments"),
        legend: Some("A succubus who truly falls in love becomes mortal."),
    },
    MonsterLore {
        kind: EnemyKind::Balrog,
        description: "Massive demons of shadow and flame. They wield weapons of fire and their wings darken the sky.",
        origin: "Ancient demons who served in the first wars between the planes. They are destruction incarnate.",
        behavior: "Destroys everything in its path. Only the most powerful demon lords can command them.",
        weakness: Some("Ancient words of power and weapons blessed by gods"),
        legend: Some("A Balrog's whip is forged from concentrated malice and can bind the soul."),
    },
    MonsterLore {
        kind: EnemyKind::PitFiend,
        description: "Generals of the demon armies. They combine tremendous physical power with devastating magic.",
        origin: "Promoted through the demon ranks by proving their cunning and strength over millennia.",
        behavior: "Strategic commanders who view battles as games. They toy with powerful opponents.",
        weakness: Some("Holy magic and exploiting their arrogance"),
        legend: Some("Pit Fiends collect the souls of those they defeat as trophies."),
    },
    MonsterLore {
        kind: EnemyKind::ShadowDemon,
        description: "Demons composed of living darkness. They can possess the shadows of mortals and control their actions.",
        origin: "Formed from the shadows cast by evil deeds. They grow stronger in places of great wickedness.",
        behavior: "Prefers to possess and corrupt rather than fight directly. Can hide in any shadow.",
        weakness: Some("Bright light forces them out of shadows; they cannot exist in areas without darkness"),
        legend: Some("Shadow demons can hear every secret whispered in darkness."),
    },
    MonsterLore {
        kind: EnemyKind::AbyssalHorror,
        description: "Nameless things from the deepest abyss. Their very presence causes madness and reality warps around them.",
        origin: "Not truly demons but something older and stranger. They existed before the planes were separated.",
        behavior: "Incomprehensible motives. Their actions follow a logic alien to mortal minds.",
        weakness: Some("Unknown - few have survived to report"),
        legend: Some("Looking directly at an Abyssal Horror reveals truths that shatter sanity."),
    },
    MonsterLore {
        kind: EnemyKind::DoomGuard,
        description: "Elite demon warriors encased in cursed armor. They serve as the personal guard of demon royalty.",
        origin: "Chosen demons granted powerful armor forged in the deepest pits. The armor bonds permanently.",
        behavior: "Utterly loyal to their masters. Will fight until completely destroyed.",
        weakness: Some("Removing their helmet temporarily blinds them"),
        legend: Some("Doom Guard armor contains the essence of a hundred conquered souls."),
    },

    // ========== BOSSES ==========
    MonsterLore {
        kind: EnemyKind::BossGoblinKing,
        description: "The supreme ruler of all goblin tribes within the crypts. He wears a crown of stolen gold and commands absolute loyalty.",
        origin: "Rose to power by defeating all rival chieftains in combat. His throne is built from the weapons of his enemies.",
        behavior: "Cunning strategist who lets his minions weaken foes before engaging. Never fights fair.",
        weakness: Some("His crown is the source of his authority; damaging it causes his followers to waver"),
        legend: Some("The Goblin King dreams of conquering the surface world and making all races bow to goblin-kind."),
    },
    MonsterLore {
        kind: EnemyKind::BossOrcWarlord,
        description: "A legendary orc warrior who united the fractured tribes through unmatched strength and brutality.",
        origin: "Descended from Grommash himself. He proved his lineage by single-handedly slaying a giant.",
        behavior: "Leads from the front. Considers strategic retreat a form of cowardice.",
        weakness: Some("His pride can be exploited through challenges to single combat"),
        legend: Some("The Orc Warlord seeks Grommash's legendary axe, said to be hidden somewhere in the crypts."),
    },
    MonsterLore {
        kind: EnemyKind::BossVampireLord,
        description: "The ancient master of the crypt's vampire court. He has existed for millennia and his power is immense.",
        origin: "One of the first vampires created by Lord Sanguinus himself. He has drunk the blood of kings and heroes.",
        behavior: "Sophisticated and patient. He views combat as beneath him but is devastating when provoked.",
        weakness: Some("Ancient vampire-slaying weapons and the blood of the pure-hearted"),
        legend: Some("The Vampire Lord knows the location of Sanguinus's tomb and guards this secret jealously."),
    },
    MonsterLore {
        kind: EnemyKind::BossForestGuardian,
        description: "The primordial spirit of the underground forest given physical form. It is nature's wrath incarnate.",
        origin: "Manifested when the forest's corruption reached critical levels. It seeks to purge all intruders.",
        behavior: "Commands all forest creatures. The trees themselves move at its will.",
        weakness: Some("Cannot be truly killed while any of the forest remains; pacifying the corruption weakens it"),
        legend: Some("The Guardian was once a gentle spirit that helped lost travelers. Corruption twisted it into a destroyer."),
    },
    MonsterLore {
        kind: EnemyKind::BossIceDragon,
        description: "An ancient wyrm of terrible power. Its breath can freeze armies solid and its scales are harder than steel.",
        origin: "One of the last true dragons, it retreated to the ice caverns after the Age of Heroes.",
        behavior: "Proud and ancient. It considers itself superior to all other beings.",
        weakness: Some("Fire magic and attacks on the unscaled spot over its heart"),
        legend: Some("The Ice Dragon guards a hoard containing artifacts from before the Shadowfall."),
    },
    MonsterLore {
        kind: EnemyKind::BossDemonKing,
        description: "The supreme ruler of the demon incursion. His power rivals the gods themselves.",
        origin: "Led the demon invasion that nearly conquered the world. He was sealed away but the seal weakens.",
        behavior: "Absolute confidence in his superiority. He toys with challengers before destroying them.",
        weakness: Some("The artifacts used to seal him originally; weapons blessed by all the gods"),
        legend: Some("The Prophecy speaks of one who will either slay the Demon King or free him to conquer all."),
    },

    // ========== MINI-BOSSES ==========
    MonsterLore {
        kind: EnemyKind::GoblinChampion,
        description: "The greatest warrior among the goblin tribes. He earned his title through countless victories.",
        origin: "A goblin who trained obsessively to overcome his race's physical limitations.",
        behavior: "Seeks worthy opponents to test himself against. Protects the Goblin King fiercely.",
        weakness: Some("Overconfident in his abilities"),
        legend: None,
    },
    MonsterLore {
        kind: EnemyKind::OrcBerserker,
        description: "An orc who has mastered the blood rage. In battle, he feels no pain and fears nothing.",
        origin: "Trained by shamans in the ancient ways of the berserker. He has never been defeated in single combat.",
        behavior: "Charges the strongest enemy without hesitation. Cannot be reasoned with during rage.",
        weakness: Some("Exhaustion after the rage ends leaves him vulnerable"),
        legend: None,
    },
    MonsterLore {
        kind: EnemyKind::VampireElite,
        description: "A vampire of the inner court, personally chosen by the Vampire Lord for exceptional ability.",
        origin: "Former nobles or warriors who impressed the Vampire Lord enough to be granted the dark gift.",
        behavior: "Sophisticated predators who combine combat skill with supernatural powers.",
        weakness: Some("Standard vampire weaknesses but requires stronger applications"),
        legend: None,
    },
    MonsterLore {
        kind: EnemyKind::AncientWyrm,
        description: "A serpentine dragon of immense age. Though not as powerful as true dragons, it is still devastating.",
        origin: "Wyrms are dragons that never developed wings but gained length and cunning instead.",
        behavior: "Ambush predator that strikes from concealment. Can swallow humans whole.",
        weakness: Some("Its length makes it slow to turn"),
        legend: Some("Wyrm scales can be crafted into armor that grants resistance to magic."),
    },
    MonsterLore {
        kind: EnemyKind::FrostLord,
        description: "A frost giant who has claimed dominion over the ice caverns through sheer power.",
        origin: "The strongest warrior among the frost giants, he challenges the Ice Dragon's rule.",
        behavior: "Commands frost giant raiding parties. Seeks treasures to build his power.",
        weakness: Some("Fire magic and his rivalry with the Ice Dragon can be exploited"),
        legend: None,
    },
    MonsterLore {
        kind: EnemyKind::InfernalLord,
        description: "A demon of significant power who commands the volcanic region's infernal forces.",
        origin: "Sent by the Demon King to prepare for the coming invasion. He revels in his authority.",
        behavior: "Cruel commander who punishes failure with destruction. Seeks to impress the Demon King.",
        weakness: Some("Holy magic and his fear of failure before the Demon King"),
        legend: None,
    },
];

// ============================================================================
// ITEM LORE
// ============================================================================

/// Detailed item lore entry
#[derive(Clone, Debug)]
pub struct ItemLoreEntry {
    pub kind: ItemKind,
    pub description: &'static str,
    pub history: &'static str,
    pub magical_properties: Option<&'static str>,
    pub legend: Option<&'static str>,
}

static DEFAULT_ITEM_LORE: ItemLoreEntry = ItemLoreEntry {
    kind: ItemKind::Gold,
    description: "A mysterious item of unknown origin.",
    history: "Its history has been lost to time.",
    magical_properties: None,
    legend: None,
};

/// All item lore entries
pub static ITEM_LORE: &[ItemLoreEntry] = &[
    // ========== LEGENDARY WEAPONS ==========
    ItemLoreEntry {
        kind: ItemKind::DemonSlayer,
        description: "A massive blade forged specifically to combat the forces of darkness. It glows with holy light when demons are near.",
        history: "Forged by the legendary smith Wayland during the Demon War. It was wielded by the hero Aldric who pushed the demon armies back to their realm.",
        magical_properties: Some("Deals additional damage to demons and undead. Glows in the presence of evil."),
        legend: Some("It is said that the Demon Slayer contains a shard of divine essence and can never be wielded by one with evil in their heart."),
    },
    ItemLoreEntry {
        kind: ItemKind::VoidStaff,
        description: "A staff carved from wood that grew at the boundary between worlds. It channels magic from the void between planes.",
        history: "Created by the Archmage Nethys who sought to tap into the raw magic that exists between realities.",
        magical_properties: Some("Greatly amplifies magical power. Allows casting of forbidden void magic."),
        legend: Some("Those who use the Void Staff too often begin to fade from reality, eventually vanishing entirely."),
    },
    ItemLoreEntry {
        kind: ItemKind::FlameSword,
        description: "A blade permanently wreathed in magical fire. It was quenched in dragon's blood during its forging.",
        history: "One of the Elemental Blades, forged by the Salamander smiths in the heart of a volcano.",
        magical_properties: Some("Burns enemies on contact. Never needs sharpening as the flames reshape the blade."),
        legend: Some("The Flame Sword is said to be jealous and will burn those who wield other weapons."),
    },
    ItemLoreEntry {
        kind: ItemKind::FrostBlade,
        description: "A sword of eternal ice that freezes anything it touches. Mist constantly flows from its edge.",
        history: "Carved from ice that never melts, taken from the heart of an ancient glacier during the Ice Age.",
        magical_properties: Some("Freezes enemies on contact. Keeps its wielder cool in any environment."),
        legend: Some("The Frost Blade was the weapon of the Winter Queen before her fall."),
    },
    ItemLoreEntry {
        kind: ItemKind::ThunderAxe,
        description: "A massive axe that crackles with lightning. Thunder sounds with each swing.",
        history: "Said to be forged from a bolt of lightning captured by the storm giants.",
        magical_properties: Some("Strikes with the force of thunder. Can call down lightning in storms."),
        legend: Some("The Thunder Axe was used to slay the Storm Titan in ages past."),
    },

    // ========== LEGENDARY ARMOR ==========
    ItemLoreEntry {
        kind: ItemKind::DragonArmor,
        description: "Armor forged from the scales of an ancient dragon. It is nearly impervious to damage.",
        history: "Created from the scales of Drakonius the Elder, who willingly gave them to protect his human friend.",
        magical_properties: Some("Grants resistance to elemental damage. Scales regenerate over time."),
        legend: Some("The armor retains a fragment of dragon consciousness and advises its wearer."),
    },
    ItemLoreEntry {
        kind: ItemKind::HolyArmor,
        description: "Blessed armor that radiates divine light. Undead and demons recoil from its presence.",
        history: "Worn by the paladins of the Dawn Order during the Demon War. Few sets remain.",
        magical_properties: Some("Provides protection against evil. Heals the wearer slowly over time."),
        legend: Some("Only those pure of heart can wear Holy Armor without burning."),
    },
    ItemLoreEntry {
        kind: ItemKind::DemonArmor,
        description: "Armor forged in the demon realm from souls and shadow. It grants terrible power at a cost.",
        history: "Created by the Demon King as a gift for his greatest champions. It corrupts all who wear it.",
        magical_properties: Some("Grants immense strength and durability. Slowly corrupts the wearer's soul."),
        legend: Some("Those who die wearing Demon Armor become demons themselves."),
    },
    ItemLoreEntry {
        kind: ItemKind::TitanPlate,
        description: "Massive armor made from the remains of a fallen titan. It grants the strength of giants.",
        history: "Crafted from the bones and flesh of Kronos, the last titan, after his defeat.",
        magical_properties: Some("Grants tremendous strength and size. The wearer never tires."),
        legend: Some("The Titan Plate seeks to reunite with the other pieces of Kronos."),
    },

    // ========== LEGENDARY ACCESSORIES ==========
    ItemLoreEntry {
        kind: ItemKind::CrownOfKings,
        description: "A crown worn by the greatest rulers of the ancient world. It grants wisdom and authority.",
        history: "Passed down through a hundred kings before being lost in the Shadow Crypts.",
        magical_properties: Some("Enhances all abilities. Commands respect from all who see it."),
        legend: Some("The Crown chooses its wearer and has refused many who sought to claim it."),
    },
    ItemLoreEntry {
        kind: ItemKind::AmuletOfTheGods,
        description: "An amulet containing divine essence from multiple gods. Its power is beyond mortal understanding.",
        history: "Created during the God War when deities combined their power to defeat a threat to all reality.",
        magical_properties: Some("Grants a fraction of divine power. Protects against divine and demonic forces."),
        legend: Some("The gods themselves watch those who possess this amulet."),
    },
    ItemLoreEntry {
        kind: ItemKind::RingOfTheAncients,
        description: "A ring from before recorded history. Its origins are unknown but its power is undeniable.",
        history: "Found in ruins that predate all known civilizations. Scholars debate its purpose endlessly.",
        magical_properties: Some("Enhances all aspects of the wearer. Grants visions of the ancient past."),
        legend: Some("The Ring is said to be one of several, and reuniting them would grant godlike power."),
    },

    // ========== SPECIAL ITEMS ==========
    ItemLoreEntry {
        kind: ItemKind::SoulGem,
        description: "A gem that can capture and store souls. It pulses with an inner light that might be consciousness.",
        history: "Created by necromancers to power their greatest works. Each gem contains countless trapped souls.",
        magical_properties: Some("Can capture the souls of dying enemies. Souls can be released for various effects."),
        legend: Some("Freeing the souls trapped in a Soul Gem grants their eternal gratitude - or vengeance."),
    },
    ItemLoreEntry {
        kind: ItemKind::AncientRelic,
        description: "An artifact from the age before the Shadowfall. Its purpose has been forgotten but its power remains.",
        history: "One of many relics scattered when the ancient civilization fell. Each is unique and irreplaceable.",
        magical_properties: Some("Effects vary by specific relic. All are powerful in unpredictable ways."),
        legend: Some("Collecting all the Ancient Relics would restore the power of the lost civilization."),
    },
    ItemLoreEntry {
        kind: ItemKind::DragonScale,
        description: "A scale from a true dragon. It is harder than steel and retains magical properties.",
        history: "Dragons shed scales rarely, and each scale can be used to create powerful items.",
        magical_properties: Some("Grants elemental resistance based on the dragon's type."),
        legend: Some("A dragon can sense when their scales are near and may seek to reclaim them."),
    },
    ItemLoreEntry {
        kind: ItemKind::DemonHeart,
        description: "The crystallized heart of a powerful demon. It pulses with infernal energy.",
        history: "When powerful demons die, their hearts crystallize rather than dissolve. They retain demonic power.",
        magical_properties: Some("Can be used to enhance weapons with demonic power or cast powerful dark magic."),
        legend: Some("Consuming a Demon Heart grants demonic power but risks losing one's soul."),
    },

    // ========== POTIONS ==========
    ItemLoreEntry {
        kind: ItemKind::HealthPotion,
        description: "A red liquid that accelerates the body's natural healing. Essential for any adventurer.",
        history: "The recipe was developed by the Alchemist's Guild and has saved countless lives.",
        magical_properties: Some("Instantly heals wounds and restores vitality."),
        legend: None,
    },
    ItemLoreEntry {
        kind: ItemKind::ManaPotion,
        description: "A blue liquid that restores magical energy. It tastes of starlight and possibility.",
        history: "Created by mages who needed to replenish their power during long rituals.",
        magical_properties: Some("Restores magical energy to the drinker."),
        legend: None,
    },
    ItemLoreEntry {
        kind: ItemKind::UltimatePowerPotion,
        description: "A swirling mixture of impossible colors. Drinking it temporarily elevates one beyond mortal limits.",
        history: "The formula is known only to the greatest alchemists and requires ingredients from across the world.",
        magical_properties: Some("Temporarily grants immense power in all aspects."),
        legend: Some("Some who drink this potion never want to return to their normal state."),
    },

    // ========== SCROLLS ==========
    ItemLoreEntry {
        kind: ItemKind::ScrollTeleport,
        description: "A scroll containing a teleportation spell. Reading it transports the user to a random safe location.",
        history: "Developed as an emergency escape method for mages in dangerous situations.",
        magical_properties: Some("Instantly transports the reader to safety."),
        legend: None,
    },
    ItemLoreEntry {
        kind: ItemKind::ScrollDeath,
        description: "A forbidden scroll that channels the power of death itself. Reading it can slay even powerful foes.",
        history: "Written by the Lich King Acererak as a weapon against his enemies.",
        magical_properties: Some("Has a chance to instantly kill any single target."),
        legend: Some("Each use of the scroll brings the reader closer to death themselves."),
    },
    ItemLoreEntry {
        kind: ItemKind::ScrollDivineWrath,
        description: "A scroll that calls down divine punishment on enemies. It channels the fury of the gods.",
        history: "Granted to the most faithful priests for use against great evils.",
        magical_properties: Some("Calls down holy fire that devastates demons and undead."),
        legend: Some("The gods take note of those who use their power."),
    },

    // ========== SHIELDS ==========
    ItemLoreEntry {
        kind: ItemKind::MirrorShield,
        description: "A shield with a perfectly reflective surface. It can reflect magical attacks back at their source.",
        history: "Created to combat magical threats. The reflection is so perfect it can show truth through illusions.",
        magical_properties: Some("Can reflect magical attacks. Shows the true form of things in its reflection."),
        legend: Some("Looking into the Mirror Shield shows your true self - some cannot bear what they see."),
    },
    ItemLoreEntry {
        kind: ItemKind::PhoenixShield,
        description: "A shield blessed by a phoenix. It protects against fire and can bring its wielder back from the brink of death.",
        history: "Granted to a hero who saved a phoenix's nest from hunters.",
        magical_properties: Some("Grants fire immunity. Once per day, can save the wielder from a killing blow."),
        legend: Some("The phoenix's blessing must be renewed through acts of protection and sacrifice."),
    },
    ItemLoreEntry {
        kind: ItemKind::AbyssalShield,
        description: "A shield forged in the abyss from concentrated darkness. It absorbs attacks into an inner void.",
        history: "Created by a smith who made a bargain with abyssal powers.",
        magical_properties: Some("Absorbs damage and can release it as a devastating counterattack."),
        legend: Some("The shield hungers and may consume more than its wielder intends."),
    },

    // ========== RINGS ==========
    ItemLoreEntry {
        kind: ItemKind::RingOfTheVampire,
        description: "A ring that grants vampiric powers. Attacks drain life from enemies to heal the wearer.",
        history: "Given by the Vampire Lord to his favored servants. Possessing one marks the wearer as under his protection.",
        magical_properties: Some("Attacks drain life force from enemies."),
        legend: Some("The Vampire Lord can see through any of his rings and knows their wearers' locations."),
    },
    ItemLoreEntry {
        kind: ItemKind::RingOfDeath,
        description: "A ring that channels death energy. It increases power greatly but slowly kills its wearer.",
        history: "Created by a necromancer who sought power at any cost. He was its first victim.",
        magical_properties: Some("Greatly increases attack power. Slowly drains the wearer's life."),
        legend: Some("The ring cannot be removed without powerful magic once put on."),
    },

    // ========== FOOD ==========
    ItemLoreEntry {
        kind: ItemKind::GoldenApple,
        description: "An apple of pure gold that is somehow edible. It grants vitality and a taste of immortality.",
        history: "Grown in the gardens of the gods. Mortals who eat them are changed forever.",
        magical_properties: Some("Fully restores health and hunger. Grants temporary invulnerability."),
        legend: Some("A tree that grows golden apples exists somewhere in the Shadow Crypts."),
    },
    ItemLoreEntry {
        kind: ItemKind::DragonFruit,
        description: "A fruit that grows only near dragon lairs. It contains a spark of draconic power.",
        history: "Dragons cultivate these fruits, and they are precious treasures in their hoards.",
        magical_properties: Some("Restores health and temporarily grants dragon-like abilities."),
        legend: Some("Eating enough Dragon Fruit will begin to transform a mortal into something more."),
    },
];

// ============================================================================
// WORLD LORE ENTRIES
// ============================================================================

/// All discoverable lore entries
pub static ALL_LORE_ENTRIES: &[LoreEntry] = &[
    // ========== WORLD HISTORY ==========
    LoreEntry {
        id: LoreEntryId::WorldCreation,
        category: LoreCategory::WorldHistory,
        title: String::new(), // Will be replaced at compile time
        content: String::new(),
        discovery_hint: None,
    },
];

// Since we can't use String::new() in static context, we'll use a function to get entries
/// Get a world history entry
pub fn get_world_history_entry(id: &LoreEntryId) -> Option<LoreEntry> {
    match id {
        LoreEntryId::WorldCreation => Some(LoreEntry::new(
            *id,
            LoreCategory::WorldHistory,
            "The Creation of the World",
            "In the beginning, there was only the Void - an infinite expanse of nothing and everything. \
            From the Void emerged the Primordials: beings of immense power who shaped reality from chaos. \
            They created the world as a garden, a place of beauty and balance. The sun and moon were set \
            in the sky, the mountains raised, the seas filled. Life flourished under their watchful gaze.\n\n\
            But the Primordials grew weary, and they retreated to the spaces between worlds to sleep. \
            In their absence, their children - the gods - took stewardship of creation. For ages, there \
            was peace and prosperity. But darkness stirred in the depths, waiting for its moment."
        )),
        LoreEntryId::TheFirstAge => Some(LoreEntry::new(
            *id,
            LoreCategory::WorldHistory,
            "The First Age: Age of Wonder",
            "The First Age was a time of marvels. The mortal races - humans, elves, dwarves, and others - \
            built great civilizations under the guidance of the gods. Magic flowed freely, and wonders \
            that modern scholars can barely imagine were commonplace.\n\n\
            The greatest achievement of this age was the Grand Library, said to contain all knowledge \
            that ever was or ever would be. Mages could teleport across continents, and diseases were \
            unknown. It was a golden age that many believed would last forever.\n\n\
            But pride grew with power. The mortal races began to believe they no longer needed the gods. \
            Some sought to become gods themselves. This hubris would have terrible consequences."
        )),
        LoreEntryId::TheShadowfall => Some(LoreEntry::new(
            *id,
            LoreCategory::WorldHistory,
            "The Shadowfall: End of the First Age",
            "The Shadowfall began when the Archmage Malachar attempted the Ascension Ritual - a spell \
            to elevate himself to godhood. The ritual required a tear in the fabric of reality, a door \
            to the realm of pure power beyond the gods.\n\n\
            What Malachar found was not power but the Demon Realm - an infinite expanse of malevolent \
            entities that had been sealed away since before time. The tear became a flood, and demons \
            poured into the world.\n\n\
            The war that followed devastated creation. Cities burned, nations fell, and billions died. \
            The gods themselves descended to fight, and many were slain or diminished. The world was \
            changed forever, scarred by shadow magic that would never fully fade."
        )),
        LoreEntryId::TheSealing => Some(LoreEntry::new(
            *id,
            LoreCategory::WorldHistory,
            "The Sealing: Hope in Darkness",
            "When all seemed lost, seven heroes arose - one from each of the mortal races, plus one \
            who was something more. They gathered the remaining power of the dying gods and the broken \
            artifacts of the First Age.\n\n\
            In a ritual that cost five of them their lives, they created the Great Seal - a barrier \
            between worlds that pushed the demon armies back to their realm. The Demon King himself \
            was bound, his power contained but not destroyed.\n\n\
            The two survivors, a human mage and an elven warrior, became the first Shadow Wardens - \
            guardians of the seal who watch for signs of its weakening. The place where the seal was \
            made became the Shadow Crypts, forever marked by the ritual's power."
        )),
        LoreEntryId::TheProphecy => Some(LoreEntry::new(
            *id,
            LoreCategory::WorldHistory,
            "The Prophecy of Shadow and Light",
            "After the Sealing, the last oracle spoke a prophecy before her power burned her away:\n\n\
            'When shadows deepen and the seal grows thin,\n\
            One shall descend where none dare begin.\n\
            Through crypt and cavern, through fire and ice,\n\
            One soul shall pay the ultimate price.\n\n\
            The crown of kings and the demon's heart,\n\
            The dragon's flame and the void's dark art,\n\
            These four united in worthy hands,\n\
            Shall break the chains or free the bands.\n\n\
            In darkness deep where the demon king waits,\n\
            A hero shall come to seal their fates.\n\
            Victory or doom, the choice is made,\n\
            In the shadow's heart where light will fade.'\n\n\
            The prophecy has guided the Shadow Wardens for millennia. They believe the time approaches."
        )),
        LoreEntryId::AgeOfHeroes => Some(LoreEntry::new(
            *id,
            LoreCategory::WorldHistory,
            "The Age of Heroes",
            "In the centuries following the Sealing, heroes rose to push back the darkness that remained. \
            These were the great champions whose names are still remembered: Aldric the Demon Slayer, \
            Elindra of the Silver Bow, Thorgrim Ironhand, and many others.\n\n\
            They cleared the lands of demonic remnants, founded new kingdoms, and established the orders \
            that would protect civilization. The Knights of Dawn, the Shadow Wardens, the Mage Councils - \
            all trace their origins to this age.\n\n\
            But even heroes die, and with each generation, a little more knowledge was lost. The great \
            artifacts were scattered or hidden, and the old magics faded. The Age of Heroes ended not \
            with a battle but with a slow forgetting."
        )),
        LoreEntryId::TheCataclysm => Some(LoreEntry::new(
            *id,
            LoreCategory::WorldHistory,
            "The Cataclysm: Second Breaking",
            "Five hundred years ago, the Cataclysm shook the world. Earthquakes shattered mountains, \
            the seas rose and fell, and magic itself went wild for days. Many believed it was the end.\n\n\
            The cause was the first major weakening of the Great Seal. The Demon King, patient in his \
            prison, had gathered enough strength to test his bonds. He failed to break free, but the \
            attempt nearly destroyed the world.\n\n\
            The Shadow Wardens reinforced the seal, but they warned that it would weaken again. Each \
            attempt by the Demon King damages it further. Some believe the seal has perhaps a few more \
            centuries; others think it could fail any day."
        )),
        LoreEntryId::TheAwakening => Some(LoreEntry::new(
            *id,
            LoreCategory::WorldHistory,
            "The Awakening: Present Day",
            "In recent years, the signs have multiplied. The Shadow Crypts have grown more active, \
            vomiting forth monsters that hadn't been seen in centuries. The dead rise more easily, \
            and dark cults multiply in the shadows of civilization.\n\n\
            The Shadow Wardens believe another assault on the seal is imminent. They have sent out \
            a call for heroes, seeking those who might fulfill the ancient prophecy. Many have \
            answered; few have returned.\n\n\
            The Shadow Crypts await. Somewhere in their depths, the Demon King stirs. The prophecy \
            speaks of one who will descend into darkness and emerge to decide the world's fate. \
            Perhaps that one is you."
        )),
        LoreEntryId::ShadowCryptOrigins => Some(LoreEntry::new(
            *id,
            LoreCategory::WorldHistory,
            "Origins of the Shadow Crypts",
            "The Shadow Crypts were not always a place of darkness. Before the Shadowfall, this \
            location was the site of a great temple complex dedicated to all the gods. Pilgrims \
            came from across the world to seek wisdom and healing.\n\n\
            When the Sealing ritual was performed here, the sacred site was transformed. The temples \
            collapsed into the earth, forming vast underground chambers. The power of the seal \
            saturated everything, creating a place where the barrier between worlds is thin.\n\n\
            This thinness is both blessing and curse. The seal is strongest here, but so is the \
            influence of the Demon Realm. Creatures of shadow are drawn to the crypts, and the \
            dead rise easily in its shadowed halls."
        )),
        LoreEntryId::TheForgottenKings => Some(LoreEntry::new(
            *id,
            LoreCategory::WorldHistory,
            "The Forgotten Kings",
            "Before the great nations of today, there were the Forgotten Kings - rulers of an \
            empire that spanned the known world. They built wonders that have never been equaled \
            and wielded magic that modern mages can only dream of.\n\n\
            The Forgotten Kings were not human, elf, or dwarf, but something older. Some scholars \
            believe they were the children of the Primordials, or perhaps a race from another world \
            entirely. Their language cannot be deciphered, and their motives remain mysterious.\n\n\
            They vanished suddenly, leaving only ruins and artifacts. Many of these ruins lie within \
            the Shadow Crypts, preserved by the magical saturation. Adventurers who find them \
            discover wonders and horrors in equal measure."
        )),

        // ========== LOCATIONS ==========
        LoreEntryId::LocationDungeon => Some(LoreEntry::new(
            *id,
            LoreCategory::Locations,
            "The Upper Dungeons",
            "The upper levels of the Shadow Crypts were once the basements and crypts of the temple \
            complex. Now they are infested with the lesser creatures drawn to shadow magic - rats, \
            goblins, and reanimated dead.\n\n\
            Despite the dangers, these levels are relatively safe compared to what lies below. The \
            Shadow Wardens maintain some presence here, and the monsters are manageable for prepared \
            adventurers. Many expeditions end here, their members deciding that the treasures below \
            aren't worth the risk.\n\n\
            The walls still bear traces of holy symbols, worn by time and corrupted by shadow. Some \
            say prayers spoken here are still heard by the old gods."
        )),
        LoreEntryId::LocationCaves => Some(LoreEntry::new(
            *id,
            LoreCategory::Locations,
            "The Deep Caves",
            "Below the dungeons lie natural cave systems that were expanded by the Forgotten Kings \
            for purposes unknown. These caves extend for miles in all directions, a labyrinth where \
            many have become lost forever.\n\n\
            The creatures here are larger and more dangerous - trolls, ogres, and things that have \
            never seen the sun. Strange fungi provide dim illumination, and underground rivers create \
            unexpected hazards.\n\n\
            Rich mineral deposits attract miners foolish or desperate enough to work here. Few last \
            long, but those who do sometimes find gems of unusual properties."
        )),
        LoreEntryId::LocationCrypt => Some(LoreEntry::new(
            *id,
            LoreCategory::Locations,
            "The Ancient Crypt",
            "The true crypts begin below the caves - burial chambers of increasing antiquity and \
            darkness. Here lie the dead of ages, from recent fallen adventurers to mummies of the \
            Forgotten Kings.\n\n\
            The concentration of death energy is intense. The dead rise easily here, and vampires \
            have established a hidden court among the tombs. Whispers speak of the Vampire Lord \
            who rules from a throne of bones.\n\n\
            Great treasures lie buried with the dead, but disturbing them carries terrible risks. \
            The curses of mummy lords have followed tomb robbers across continents."
        )),
        LoreEntryId::LocationForest => Some(LoreEntry::new(
            *id,
            LoreCategory::Locations,
            "The Sunless Forest",
            "Perhaps the strangest region of the crypts is the underground forest - a vast cavern \
            where trees grow without sunlight, sustained by pure magical energy. Bioluminescent \
            plants provide an eerie glow.\n\n\
            The forest was created by druids fleeing the Shadowfall, who used the last of their \
            power to create a sanctuary underground. But the shadow magic corrupted their work, \
            and now the forest is as dangerous as anywhere in the crypts.\n\n\
            Corrupted druids and feral beasts rule here. The Forest Guardian, once a protective \
            spirit, has become a destroyer of all intruders."
        )),
        LoreEntryId::LocationIceCavern => Some(LoreEntry::new(
            *id,
            LoreCategory::Locations,
            "The Frozen Depths",
            "Deeper still, the temperature drops dramatically. The ice caverns are a realm of \
            eternal winter, where frost giants and ice dragons have made their home since fleeing \
            the warming world above.\n\n\
            The cold here is supernatural - no amount of mundane protection is sufficient. Only \
            magical warmth can keep an explorer alive. But the treasures frozen in the ice are \
            legendary: artifacts from the Ice Age preserved perfectly.\n\n\
            The Ice Dragon who rules these depths guards a hoard accumulated over millennia. Many \
            seek it; none have claimed it."
        )),
        LoreEntryId::LocationVolcanic => Some(LoreEntry::new(
            *id,
            LoreCategory::Locations,
            "The Volcanic Chambers",
            "Below the ice lies fire - volcanic chambers where magma flows freely and the heat \
            would instantly kill an unprotected mortal. This is the realm of fire elementals, \
            salamanders, and creatures born of flame.\n\n\
            The volcanic chambers are closest to the Demon Realm, separated only by the thinnest \
            barrier. Demons slip through regularly, and the Infernal Lord commands them in \
            preparation for their king's eventual freedom.\n\n\
            The salamanders forge weapons here that are prized above all others. Those brave \
            enough to trade with them can acquire items of incredible power."
        )),
        LoreEntryId::LocationAncientRuins => Some(LoreEntry::new(
            *id,
            LoreCategory::Locations,
            "The Ancient Ruins",
            "The lowest mortal-accessible levels contain the ruins of the Forgotten Kings - \
            structures so old they predate the gods' arrival in the world. The architecture \
            follows no human logic, and the proportions suggest their builders were not our size.\n\n\
            Golems and guardians still patrol these halls, following orders given millennia ago. \
            The magic here is strange - spells work differently, and impossible things become \
            possible.\n\n\
            The greatest treasures and the greatest dangers await here. Many of the artifacts \
            needed to confront the Demon King are said to rest in these ruins."
        )),
        LoreEntryId::LocationDemonRealm => Some(LoreEntry::new(
            *id,
            LoreCategory::Locations,
            "The Demon Threshold",
            "At the deepest point of the Shadow Crypts lies the threshold between worlds - the \
            place where the Great Seal was created and where it is weakest. Here, the Demon King \
            waits in his prison of light and shadow.\n\n\
            No mortal has entered this realm and returned to tell of it. The few scraps of \
            knowledge come from visions and the ravings of those driven mad by proximity. It is \
            said to be a place of impossible geometry and eternal torment.\n\n\
            The prophecy says one hero will enter this place and emerge victorious. Whether that \
            means killing the Demon King or something else entirely remains to be seen."
        )),
        LoreEntryId::LocationThrone => Some(LoreEntry::new(
            *id,
            LoreCategory::Locations,
            "The Shadow Throne",
            "At the heart of the Demon Realm sits the Shadow Throne - the seat of the Demon King's \
            power. From here, he once commanded armies that nearly conquered all of creation.\n\n\
            The Throne is not merely a seat but a focus of power. It channels the energy of the \
            Demon Realm, granting its occupant unimaginable might. Even sealed, the Demon King \
            draws power from it.\n\n\
            Destroying the Throne might weaken the Demon King enough to destroy him permanently. \
            But it might also release him from his bonds. The prophecy is unclear."
        )),
        LoreEntryId::LocationHiddenSanctuary => Some(LoreEntry::new(
            *id,
            LoreCategory::Locations,
            "The Hidden Sanctuary",
            "Somewhere in the Shadow Crypts is a place untouched by darkness - a sanctuary \
            maintained by the Shadow Wardens as a place of rest and resupply for worthy heroes. \
            Its location is known only to those who have proven themselves.\n\n\
            The Sanctuary contains stores of healing supplies, weapons blessed against demons, \
            and knowledge passed down from the original Wardens. It is perhaps the only safe \
            place in all the crypts.\n\n\
            Finding the Sanctuary is itself a test. The Wardens leave clues for those with the \
            wisdom to recognize them."
        )),

        // ========== FACTIONS ==========
        LoreEntryId::FactionShadowCult => Some(LoreEntry::new(
            *id,
            LoreCategory::Factions,
            "The Shadow Cult",
            "Not all mortals oppose the Demon King. The Shadow Cult believes that his victory is \
            inevitable and that those who serve him will be rewarded in the new order. They work \
            to weaken the seal and prepare the world for his return.\n\n\
            The Cult operates in secret, with cells in every major city. Their members include \
            nobles, merchants, and even some mages. They provide resources and information to \
            the forces within the crypts.\n\n\
            The Cult's high priests can channel demonic power, making them dangerous opponents. \
            But their greatest weapon is their secrecy - anyone could be a cultist."
        )),
        LoreEntryId::FactionKnightsOfDawn => Some(LoreEntry::new(
            *id,
            LoreCategory::Factions,
            "The Knights of Dawn",
            "Founded in the Age of Heroes, the Knights of Dawn are dedicated to fighting demons \
            and undead wherever they appear. They are the mortal world's first line of defense \
            against supernatural evil.\n\n\
            Knights undergo rigorous training in combat and holy magic. Their weapons are blessed, \
            and their faith provides protection against dark powers. They work closely with the \
            Shadow Wardens.\n\n\
            In recent years, the Knights have been stretched thin by increasing demonic activity. \
            They desperately seek new recruits worthy of the order's legacy."
        )),
        LoreEntryId::FactionAncientMages => Some(LoreEntry::new(
            *id,
            LoreCategory::Factions,
            "The Mage Councils",
            "The Mage Councils are the successors of the great magical academies of the First Age. \
            Though much knowledge was lost in the Shadowfall, they still command significant power \
            and influence.\n\n\
            Council mages study the Shadow Crypts from a safe distance, sending expeditions to \
            recover artifacts and knowledge. They provide magical support to those who venture \
            inside and analyze what is brought out.\n\n\
            Some Council members have more personal agendas - seeking forbidden knowledge or \
            power that should remain buried. Not all mages can be trusted."
        )),
        LoreEntryId::FactionDemonLords => Some(LoreEntry::new(
            *id,
            LoreCategory::Factions,
            "The Demon Lords",
            "The Demon Realm is not unified. Below the Demon King, various Demon Lords compete \
            for power and favor. Each commands legions of lesser demons and schemes against \
            their rivals.\n\n\
            This competition is both blessing and curse for the mortal world. The Lords are too \
            busy fighting each other to coordinate effectively, but their schemes often spill \
            over into mortal realms.\n\n\
            Some have tried to bargain with Demon Lords against the Demon King. None have \
            succeeded without losing far more than they gained."
        )),
        LoreEntryId::FactionForestGuardians => Some(LoreEntry::new(
            *id,
            LoreCategory::Factions,
            "The Corrupted Circle",
            "The druids who created the Sunless Forest are now its greatest threat. Calling \
            themselves the Corrupted Circle, they believe the forest's twisted form is its \
            true nature and seek to spread it to the surface.\n\n\
            They command the forest's creatures and can shape plants into weapons and servants. \
            Their leader, the Archdruid, has been corrupted so thoroughly that she barely \
            resembles anything human.\n\n\
            Some say the original druids' spirits still exist, trapped within the corrupted \
            forms. Freeing them might restore the forest - or destroy it entirely."
        )),
        LoreEntryId::FactionFrostGiants => Some(LoreEntry::new(
            *id,
            LoreCategory::Factions,
            "The Frost Giant Clans",
            "The frost giants are refugees from a dying age. When the world warmed after the \
            Ice Age, they retreated to the coldest places - the mountain peaks and the frozen \
            depths of the earth.\n\n\
            They remember a time when ice covered the world and they were its masters. They \
            dream of returning the world to eternal winter and ruling it once again. The \
            Ice Dragon's plans align with theirs.\n\n\
            The giants are not united. Multiple clans compete for territory and resources. \
            An outsider might exploit these divisions, but the giants despise all smaller \
            races as 'the warm-bloods.'"
        )),
        LoreEntryId::FactionUndeadLegion => Some(LoreEntry::new(
            *id,
            LoreCategory::Factions,
            "The Undead Courts",
            "The intelligent undead of the crypts have formed a complex society with the \
            Vampire Lord at its apex. Below him are the vampire nobility, then the liches \
            and death knights, then the mindless masses.\n\n\
            The courts follow elaborate rules of conduct and precedence. Vampires scheme \
            against each other endlessly, but all serve the Vampire Lord's greater vision \
            of a world where the living are merely cattle.\n\n\
            The Vampire Lord has his own plans regarding the Demon King. He does not wish \
            to serve - he wishes to rule."
        )),
        LoreEntryId::FactionGoblinTribes => Some(LoreEntry::new(
            *id,
            LoreCategory::Factions,
            "The Goblin Tribes",
            "Goblins are the most numerous intelligent species in the crypts. Dozens of tribes \
            compete for territory and resources, united only by their loyalty to the Goblin King.\n\n\
            Despite their individual weakness, goblins are cunning and numerous. They use traps, \
            ambushes, and sheer weight of numbers to bring down stronger foes. Their crafting \
            skills, while crude, are effective.\n\n\
            The Goblin King dreams of leading his people to conquer the surface world. He sees \
            the chaos of the Demon King's potential escape as an opportunity."
        )),
        LoreEntryId::FactionDragonkin => Some(LoreEntry::new(
            *id,
            LoreCategory::Factions,
            "The Dragonkin",
            "True dragons are rare, but their descendants and servants are numerous. Kobolds \
            worship them as gods, drakes serve as mounts and guards, and the dragons themselves \
            rule their domains with absolute authority.\n\n\
            The Ice Dragon is the most powerful dragon in the crypts, but others lurk in \
            hidden lairs. They collect hoards of treasure and knowledge, and they rarely \
            interfere in the affairs of 'lesser' beings.\n\n\
            A dragon's favor is a powerful thing, but earning it requires extraordinary deeds \
            or offerings of extraordinary treasure."
        )),
        LoreEntryId::FactionLostSouls => Some(LoreEntry::new(
            *id,
            LoreCategory::Factions,
            "The Lost Souls",
            "Not all who enter the crypts serve darkness or die. Some become lost in ways that \
            transcend simple navigation - they become unmoored from time and reality, wandering \
            the halls in a perpetual daze.\n\n\
            The Lost Souls are not hostile, but they are not helpful either. They mutter cryptic \
            phrases and sometimes provide useful information, but they cannot be relied upon. \
            Some have been wandering for centuries.\n\n\
            Helping a Lost Soul find peace is said to grant their blessings. Some carry items \
            or knowledge from ages past."
        )),

        // ========== PROPHECIES ==========
        LoreEntryId::ProphecyChosenOne => Some(LoreEntry::new(
            *id,
            LoreCategory::Prophecies,
            "The Chosen One",
            "The prophecy speaks of one who will descend into darkness and face the Demon King. \
            This Chosen One is not marked by birth or destiny but by their actions - by choosing \
            to enter the crypts and face what lies within.\n\n\
            Many believe they are the Chosen One. The crypts are littered with their remains. \
            The true Chosen One will not proclaim themselves but will be known by their deeds.\n\n\
            Whether the Chosen One will save the world or doom it is not certain. The prophecy \
            speaks of a choice, and choices can go either way."
        )),
        LoreEntryId::ProphecyEternalDarkness => Some(LoreEntry::new(
            *id,
            LoreCategory::Prophecies,
            "The Dark Future",
            "Seers who look too deeply into the future see a world of eternal darkness - skies \
            black with demon wings, cities burning, and mortals reduced to slaves and cattle. \
            This is the future if the Demon King escapes.\n\n\
            But seers also see this: the darkness is not inevitable. There are paths that lead \
            elsewhere, moments where the future branches. The Chosen One stands at the most \
            important branch.\n\n\
            The Dark Future is a warning, not a certainty. It exists to be prevented."
        )),
        LoreEntryId::ProphecyDemonReturn => Some(LoreEntry::new(
            *id,
            LoreCategory::Prophecies,
            "Signs of the Return",
            "The ancient texts describe signs that will herald the Demon King's return:\n\n\
            - The dead will rise unbidden in places of power\n\
            - Children will dream of fire and shadow\n\
            - Holy symbols will crack and crumble\n\
            - The faithful will doubt and the doubters will believe\n\
            - Stars will fall and the sun will dim\n\n\
            All of these signs have been observed in recent years. The time grows short."
        )),
        LoreEntryId::ProphecyFinalBattle => Some(LoreEntry::new(
            *id,
            LoreCategory::Prophecies,
            "The Final Battle",
            "The prophecy describes a final confrontation between the Chosen One and the Demon \
            King. It will take place in the heart of darkness, at the threshold between worlds.\n\n\
            The Chosen One will need four artifacts: the Crown of Kings (representing authority), \
            the Demon's Heart (representing darkness), the Dragon's Flame (representing power), \
            and the Void's Secret (representing knowledge).\n\n\
            With these four artifacts, the Chosen One can either permanently seal the Demon King \
            or absorb his power - becoming either the world's savior or its new tyrant."
        )),
        LoreEntryId::ProphecyBalance => Some(LoreEntry::new(
            *id,
            LoreCategory::Prophecies,
            "The Balance of All Things",
            "An older prophecy, nearly forgotten, speaks not of victory or defeat but of balance. \
            It suggests that the conflict between light and darkness is eternal and necessary - \
            that neither can truly defeat the other without destroying creation itself.\n\n\
            According to this prophecy, the Chosen One's true purpose is not to destroy the \
            Demon King but to restore the balance disrupted by the Shadowfall. This might mean \
            something other than combat.\n\n\
            The Shadow Wardens dismiss this prophecy as heresy. But some believe it holds a \
            deeper truth."
        )),

        // Monster entries are generated dynamically
        // Item entries are generated dynamically
    };
    None
}

/// Get entries for a specific category
pub fn get_category_entries(category: LoreCategory) -> Vec<LoreEntry> {
    let mut entries = Vec::new();

    // Add fixed entries
    let ids: Vec<LoreEntryId> = match category {
        LoreCategory::WorldHistory => vec![
            LoreEntryId::WorldCreation,
            LoreEntryId::TheFirstAge,
            LoreEntryId::TheShadowfall,
            LoreEntryId::TheSealing,
            LoreEntryId::TheProphecy,
            LoreEntryId::AgeOfHeroes,
            LoreEntryId::TheCataclysm,
            LoreEntryId::TheAwakening,
            LoreEntryId::ShadowCryptOrigins,
            LoreEntryId::TheForgottenKings,
        ],
        LoreCategory::Locations => vec![
            LoreEntryId::LocationDungeon,
            LoreEntryId::LocationCaves,
            LoreEntryId::LocationCrypt,
            LoreEntryId::LocationForest,
            LoreEntryId::LocationIceCavern,
            LoreEntryId::LocationVolcanic,
            LoreEntryId::LocationAncientRuins,
            LoreEntryId::LocationDemonRealm,
            LoreEntryId::LocationThrone,
            LoreEntryId::LocationHiddenSanctuary,
        ],
        LoreCategory::Factions => vec![
            LoreEntryId::FactionShadowCult,
            LoreEntryId::FactionKnightsOfDawn,
            LoreEntryId::FactionAncientMages,
            LoreEntryId::FactionDemonLords,
            LoreEntryId::FactionForestGuardians,
            LoreEntryId::FactionFrostGiants,
            LoreEntryId::FactionUndeadLegion,
            LoreEntryId::FactionGoblinTribes,
            LoreEntryId::FactionDragonkin,
            LoreEntryId::FactionLostSouls,
        ],
        LoreCategory::Prophecies => vec![
            LoreEntryId::ProphecyChosenOne,
            LoreEntryId::ProphecyEternalDarkness,
            LoreEntryId::ProphecyDemonReturn,
            LoreEntryId::ProphecyFinalBattle,
            LoreEntryId::ProphecyBalance,
        ],
        LoreCategory::Bestiary => {
            // Generate from monster lore
            return MONSTER_LORE.iter().map(|m| {
                LoreEntry::new(
                    LoreEntryId::MonsterEntry(m.kind),
                    LoreCategory::Bestiary,
                    m.kind.name(),
                    format!(
                        "{}\n\n**Origin:** {}\n\n**Behavior:** {}{}{}",
                        m.description,
                        m.origin,
                        m.behavior,
                        m.weakness.map_or(String::new(), |w| format!("\n\n**Weakness:** {}", w)),
                        m.legend.map_or(String::new(), |l| format!("\n\n**Legend:** {}", l)),
                    )
                )
            }).collect();
        },
        LoreCategory::ItemLore => {
            // Generate from item lore
            return ITEM_LORE.iter().map(|i| {
                LoreEntry::new(
                    LoreEntryId::ItemEntry(i.kind),
                    LoreCategory::ItemLore,
                    i.kind.name(),
                    format!(
                        "{}\n\n**History:** {}{}{}",
                        i.description,
                        i.history,
                        i.magical_properties.map_or(String::new(), |p| format!("\n\n**Magical Properties:** {}", p)),
                        i.legend.map_or(String::new(), |l| format!("\n\n**Legend:** {}", l)),
                    )
                )
            }).collect();
        },
    };

    for id in ids {
        if let Some(entry) = get_world_history_entry(&id) {
            entries.push(entry);
        }
    }

    entries
}

/// Get all lore categories
pub fn get_all_categories() -> Vec<LoreCategory> {
    vec![
        LoreCategory::WorldHistory,
        LoreCategory::Bestiary,
        LoreCategory::ItemLore,
        LoreCategory::Locations,
        LoreCategory::Factions,
        LoreCategory::Prophecies,
    ]
}

/// Random lore discovery events that can occur during gameplay
#[derive(Clone, Debug)]
pub struct LoreDiscoveryEvent {
    pub entry_id: LoreEntryId,
    pub trigger_message: &'static str,
    pub discovery_text: &'static str,
}

/// Get possible lore discoveries for a dungeon level
pub fn get_level_discoveries(level: u32) -> Vec<LoreDiscoveryEvent> {
    match level {
        1..=4 => vec![
            LoreDiscoveryEvent {
                entry_id: LoreEntryId::LocationDungeon,
                trigger_message: "You find ancient writing on the wall...",
                discovery_text: "The faded text describes the temple that once stood here.",
            },
            LoreDiscoveryEvent {
                entry_id: LoreEntryId::FactionGoblinTribes,
                trigger_message: "You discover goblin markings...",
                discovery_text: "The crude symbols tell of goblin tribal territories.",
            },
        ],
        5..=8 => vec![
            LoreDiscoveryEvent {
                entry_id: LoreEntryId::LocationCaves,
                trigger_message: "You find a miner's journal...",
                discovery_text: "The journal describes the cave systems and their dangers.",
            },
            LoreDiscoveryEvent {
                entry_id: LoreEntryId::TheFirstAge,
                trigger_message: "An ancient tablet lies half-buried...",
                discovery_text: "The tablet speaks of the wonders of the First Age.",
            },
        ],
        9..=12 => vec![
            LoreDiscoveryEvent {
                entry_id: LoreEntryId::LocationCrypt,
                trigger_message: "Tomb inscriptions catch your eye...",
                discovery_text: "The inscriptions warn of the dead that rest here.",
            },
            LoreDiscoveryEvent {
                entry_id: LoreEntryId::FactionUndeadLegion,
                trigger_message: "You find a vampire's diary...",
                discovery_text: "The diary reveals the structure of the undead courts.",
            },
        ],
        13..=16 => vec![
            LoreDiscoveryEvent {
                entry_id: LoreEntryId::LocationForest,
                trigger_message: "Carved into a tree, you find words...",
                discovery_text: "The words tell of the druids who created this place.",
            },
            LoreDiscoveryEvent {
                entry_id: LoreEntryId::FactionForestGuardians,
                trigger_message: "A corrupted shrine holds a scroll...",
                discovery_text: "The scroll describes the Corrupted Circle.",
            },
        ],
        17..=20 => vec![
            LoreDiscoveryEvent {
                entry_id: LoreEntryId::LocationIceCavern,
                trigger_message: "Frozen in the ice, a message waits...",
                discovery_text: "The preserved text tells of the frozen depths.",
            },
            LoreDiscoveryEvent {
                entry_id: LoreEntryId::FactionFrostGiants,
                trigger_message: "Giant runes adorn the walls...",
                discovery_text: "The runes speak of the frost giant clans.",
            },
        ],
        21..=24 => vec![
            LoreDiscoveryEvent {
                entry_id: LoreEntryId::LocationVolcanic,
                trigger_message: "A heat-resistant scroll survives...",
                discovery_text: "The scroll describes the volcanic chambers.",
            },
            LoreDiscoveryEvent {
                entry_id: LoreEntryId::TheShadowfall,
                trigger_message: "An ancient mural depicts disaster...",
                discovery_text: "The mural shows the Shadowfall and its horrors.",
            },
        ],
        25..=28 => vec![
            LoreDiscoveryEvent {
                entry_id: LoreEntryId::LocationAncientRuins,
                trigger_message: "Strange symbols glow on a pedestal...",
                discovery_text: "The symbols are from the Forgotten Kings' language.",
            },
            LoreDiscoveryEvent {
                entry_id: LoreEntryId::TheForgottenKings,
                trigger_message: "A holographic projection activates...",
                discovery_text: "The projection shows beings unlike any known race.",
            },
        ],
        _ => vec![
            LoreDiscoveryEvent {
                entry_id: LoreEntryId::LocationDemonRealm,
                trigger_message: "Reality warps around a tear in space...",
                discovery_text: "Through the tear, you glimpse the Demon Realm.",
            },
            LoreDiscoveryEvent {
                entry_id: LoreEntryId::ProphecyFinalBattle,
                trigger_message: "A dying angel speaks prophecy...",
                discovery_text: "The angel's words describe the final battle.",
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codex_discovery() {
        let mut codex = Codex::new();

        assert!(codex.discover(LoreEntryId::WorldCreation));
        assert!(!codex.discover(LoreEntryId::WorldCreation)); // Already discovered
        assert!(codex.is_discovered(&LoreEntryId::WorldCreation));
        assert_eq!(codex.total_discoveries, 1);
    }

    #[test]
    fn test_monster_encounter() {
        let mut codex = Codex::new();

        assert!(codex.encounter_monster(EnemyKind::Rat));
        assert!(!codex.encounter_monster(EnemyKind::Rat)); // Already encountered
        assert!(codex.is_discovered(&LoreEntryId::MonsterEntry(EnemyKind::Rat)));
        assert_eq!(codex.monsters_encountered(), 1);
    }

    #[test]
    fn test_item_discovery() {
        let mut codex = Codex::new();

        assert!(codex.find_item(ItemKind::DemonSlayer));
        assert!(!codex.find_item(ItemKind::DemonSlayer)); // Already found
        assert!(codex.is_discovered(&LoreEntryId::ItemEntry(ItemKind::DemonSlayer)));
        assert_eq!(codex.items_found(), 1);
    }

    #[test]
    fn test_monster_lore_retrieval() {
        let lore = get_monster_lore(EnemyKind::Rat);
        assert!(!lore.description.is_empty());
        assert!(!lore.origin.is_empty());
    }

    #[test]
    fn test_item_lore_retrieval() {
        let lore = get_item_lore(ItemKind::DemonSlayer);
        assert!(!lore.description.is_empty());
        assert!(!lore.history.is_empty());
    }

    #[test]
    fn test_category_entries() {
        let entries = get_category_entries(LoreCategory::WorldHistory);
        assert!(!entries.is_empty());

        let bestiary = get_category_entries(LoreCategory::Bestiary);
        assert!(!bestiary.is_empty());
    }

    #[test]
    fn test_level_discoveries() {
        let discoveries = get_level_discoveries(1);
        assert!(!discoveries.is_empty());

        let deep_discoveries = get_level_discoveries(30);
        assert!(!deep_discoveries.is_empty());
    }
}
