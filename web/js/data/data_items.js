'use strict';
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});
SC.DATA = SC.DATA || {};

// Ported from src/main.rs (ItemKind enum, base_stats/glyph/color/name/food_value,
// use_item effects, random_item loot tables) and cross-checked against README.md
// and src/npc.rs shop prices. See SC.DATA.items_notes for invented values.

SC.DATA.items = {
  // ==========================================================================
  // POTIONS (20)
  // ==========================================================================
  health_potion: {
    id: 'health_potion', name: 'Health Potion', kind: 'potion', slot: 'potion',
    effect: 'heal:30+5*level', value: 25, tier: 1, glyph: '!', color: '#e74c3c',
    description: 'A crimson draught that heals 30 HP plus 5 per character level.'
  },
  mana_potion: {
    id: 'mana_potion', name: 'Mana Potion', kind: 'potion', slot: 'potion',
    effect: 'mana:25+3*level', value: 30, tier: 1, glyph: '!', color: '#3498db',
    description: 'A shimmering blue vial that restores 25 mana plus 3 per character level.'
  },
  strength_potion: {
    id: 'strength_potion', name: 'Strength Potion', kind: 'potion', slot: 'potion',
    effect: 'status:strength:20', value: 60, tier: 1, glyph: '!', color: '#f1c40f',
    description: 'Grants the Strength status effect for 20 turns.'
  },
  defense_potion: {
    id: 'defense_potion', name: 'Defense Potion', kind: 'potion', slot: 'potion',
    effect: 'status:shield:20', value: 60, tier: 1, glyph: '!', color: '#1abc9c',
    description: 'Grants the Shield status effect for 20 turns.'
  },
  speed_potion: {
    id: 'speed_potion', name: 'Speed Potion', kind: 'potion', slot: 'potion',
    effect: 'status:haste:20', value: 50, tier: 1, glyph: '!', color: '#d35ded',
    description: 'Grants the Haste status effect for 20 turns.'
  },
  invisibility_potion: {
    id: 'invisibility_potion', name: 'Invisibility Potion', kind: 'potion', slot: 'potion',
    effect: 'status:invisibility:15', value: 80, tier: 1, glyph: '!', color: '#95a5a6',
    description: 'Turns you invisible for 15 turns so enemies cannot see you.'
  },
  fire_resist_potion: {
    id: 'fire_resist_potion', name: 'Fire Resist Potion', kind: 'potion', slot: 'potion',
    value: 45, tier: 1, glyph: '!', color: '#922b21',
    description: 'A smoky red brew said to ward off flames.'
  },
  ice_resist_potion: {
    id: 'ice_resist_potion', name: 'Ice Resist Potion', kind: 'potion', slot: 'potion',
    value: 45, tier: 2, glyph: '!', color: '#148f77',
    description: 'A frigid tonic said to ward off the deepest cold.'
  },
  poison_resist_potion: {
    id: 'poison_resist_potion', name: 'Antidote', kind: 'potion', slot: 'potion',
    effect: 'cure:poison', value: 35, tier: 2, glyph: '!', color: '#2ecc71',
    description: 'A bitter green medicine that cures poison instantly.'
  },
  regeneration_potion: {
    id: 'regeneration_potion', name: 'Regeneration Potion', kind: 'potion', slot: 'potion',
    effect: 'status:regeneration:30', value: 100, tier: 1, glyph: '!', color: '#d35ded',
    description: 'Grants the Regeneration status effect for 30 turns.'
  },
  berserk_potion: {
    id: 'berserk_potion', name: 'Berserk Potion', kind: 'potion', slot: 'potion',
    value: 90, tier: 2, glyph: '!', color: '#f1c40f',
    description: 'A boiling draught that stirs a violent battle fury.'
  },
  giant_potion: {
    id: 'giant_potion', name: "Giant's Strength", kind: 'potion', slot: 'potion',
    value: 110, tier: 2, glyph: '!', color: '#f1c40f',
    description: 'Brewed from giant marrow; the drinker feels titanic power.'
  },
  levitation_potion: {
    id: 'levitation_potion', name: 'Levitation Potion', kind: 'potion', slot: 'potion',
    value: 70, tier: 2, glyph: '!', color: '#ecf0f1',
    description: 'A weightless elixir that lifts the drinker off the ground.'
  },
  xp_potion: {
    id: 'xp_potion', name: 'Potion of Experience', kind: 'potion', slot: 'potion',
    effect: 'xp:100*floor', value: 150, tier: 2, glyph: '!', color: '#1abc9c',
    description: 'Grants 100 XP per dungeon floor when drunk.'
  },
  full_restore_potion: {
    id: 'full_restore_potion', name: 'Full Restore Elixir', kind: 'potion', slot: 'potion',
    effect: 'full_restore', value: 200, tier: 3, glyph: '!', color: '#e74c3c',
    description: 'Fully restores both HP and mana.'
  },
  luck_potion: {
    id: 'luck_potion', name: 'Luck Potion', kind: 'potion', slot: 'potion',
    value: 120, tier: 3, glyph: '!', color: '#ecf0f1',
    description: 'A golden-flecked brew said to bend fortune in your favor.'
  },
  critical_potion: {
    id: 'critical_potion', name: 'Critical Strike Potion', kind: 'potion', slot: 'potion',
    value: 130, tier: 3, glyph: '!', color: '#ecf0f1',
    description: 'Sharpens the senses; strikes seem to find every weak point.'
  },
  vision_potion: {
    id: 'vision_potion', name: 'Potion of True Sight', kind: 'potion', slot: 'potion',
    value: 80, tier: 3, glyph: '!', color: '#ecf0f1',
    description: 'A clear elixir rumored to reveal what is hidden.'
  },
  cure_all_potion: {
    id: 'cure_all_potion', name: 'Cure All Elixir', kind: 'potion', slot: 'potion',
    value: 180, tier: 3, glyph: '!', color: '#ecf0f1',
    description: 'A panacea reputed to cleanse every ailment at once.'
  },
  ultimate_power_potion: {
    id: 'ultimate_power_potion', name: 'Ultimate Power Elixir', kind: 'potion', slot: 'potion',
    value: 300, tier: 3, glyph: '!', color: '#ecf0f1',
    description: 'A legendary elixir crackling with raw, untamed power.'
  },

  // ==========================================================================
  // SCROLLS (18)
  // ==========================================================================
  scroll_teleport: {
    id: 'scroll_teleport', name: 'Scroll of Teleport', kind: 'scroll', slot: 'scroll',
    effect: 'teleport_random', value: 100, tier: 1, glyph: '?', color: '#3498db',
    description: 'Instantly teleports you to a random location on the floor.'
  },
  scroll_fireball: {
    id: 'scroll_fireball', name: 'Scroll of Fireball', kind: 'scroll', slot: 'scroll',
    effect: 'aoe:burn:30:r4', value: 75, tier: 1, glyph: '?', color: '#e74c3c',
    description: 'Deals 30 fire damage and burns enemies within a radius of 4.'
  },
  scroll_ice_storm: {
    id: 'scroll_ice_storm', name: 'Scroll of Ice Storm', kind: 'scroll', slot: 'scroll',
    effect: 'aoe:freeze:25:r5', value: 75, tier: 1, glyph: '?', color: '#1abc9c',
    description: 'Deals 25 ice damage and freezes enemies within a radius of 5.'
  },
  scroll_lightning: {
    id: 'scroll_lightning', name: 'Scroll of Lightning', kind: 'scroll', slot: 'scroll',
    effect: 'damage_all_visible:40', value: 80, tier: 1, glyph: '?', color: '#f1c40f',
    description: 'Lightning strikes every visible enemy for 40 damage.'
  },
  scroll_mapping: {
    id: 'scroll_mapping', name: 'Scroll of Mapping', kind: 'scroll', slot: 'scroll',
    effect: 'reveal_map', value: 120, tier: 1, glyph: '?', color: '#ecf0f1',
    description: 'Reveals the entire map of the current floor.'
  },
  scroll_identify: {
    id: 'scroll_identify', name: 'Scroll of Identify', kind: 'scroll', slot: 'scroll',
    value: 40, tier: 1, glyph: '?', color: '#95a5a6',
    description: 'Arcane script that lays bare the nature of an unknown item.'
  },
  scroll_enchant: {
    id: 'scroll_enchant', name: 'Scroll of Enchant', kind: 'scroll', slot: 'scroll',
    value: 150, tier: 1, glyph: '?', color: '#d35ded',
    description: 'Imbues a piece of equipment with magical power.'
  },
  scroll_summon: {
    id: 'scroll_summon', name: 'Scroll of Summoning', kind: 'scroll', slot: 'scroll',
    value: 130, tier: 1, glyph: '?', color: '#2ecc71',
    description: 'Calls forth a creature from beyond to serve the reader.'
  },
  scroll_banish: {
    id: 'scroll_banish', name: 'Scroll of Banishment', kind: 'scroll', slot: 'scroll',
    value: 160, tier: 2, glyph: '?', color: '#76448a',
    description: 'Words of exile that cast a foe out of this plane.'
  },
  scroll_time_stop: {
    id: 'scroll_time_stop', name: 'Scroll of Time Stop', kind: 'scroll', slot: 'scroll',
    value: 250, tier: 2, glyph: '?', color: '#148f77',
    description: 'Freezes the flow of time for everyone but the reader.'
  },
  scroll_mass_heal: {
    id: 'scroll_mass_heal', name: 'Scroll of Mass Heal', kind: 'scroll', slot: 'scroll',
    effect: 'full_heal', value: 200, tier: 2, glyph: '?', color: '#e74c3c',
    description: 'A wave of healing light fully restores your HP.'
  },
  scroll_death: {
    id: 'scroll_death', name: 'Scroll of Death', kind: 'scroll', slot: 'scroll',
    value: 300, tier: 2, glyph: '?', color: '#922b21',
    description: 'Forbidden words that snuff out life itself.'
  },
  scroll_earthquake: {
    id: 'scroll_earthquake', name: 'Scroll of Earthquake', kind: 'scroll', slot: 'scroll',
    value: 220, tier: 2, glyph: '?', color: '#ecf0f1',
    description: 'The ground heaves and splits at the reader’s command.'
  },
  scroll_meteor: {
    id: 'scroll_meteor', name: 'Scroll of Meteor', kind: 'scroll', slot: 'scroll',
    value: 280, tier: 2, glyph: '?', color: '#ecf0f1',
    description: 'Calls a burning star down upon your enemies.'
  },
  scroll_blizzard: {
    id: 'scroll_blizzard', name: 'Scroll of Blizzard', kind: 'scroll', slot: 'scroll',
    value: 260, tier: 3, glyph: '?', color: '#ecf0f1',
    description: 'Unleashes a howling storm of ice and snow.'
  },
  scroll_chain_lightning: {
    id: 'scroll_chain_lightning', name: 'Scroll of Chain Lightning', kind: 'scroll', slot: 'scroll',
    value: 240, tier: 3, glyph: '?', color: '#ecf0f1',
    description: 'A bolt that leaps from foe to foe in a crackling arc.'
  },
  scroll_divine_wrath: {
    id: 'scroll_divine_wrath', name: 'Scroll of Divine Wrath', kind: 'scroll', slot: 'scroll',
    value: 350, tier: 3, glyph: '?', color: '#ecf0f1',
    description: 'Invokes the fury of the gods upon the wicked.'
  },
  scroll_darkness: {
    id: 'scroll_darkness', name: 'Scroll of Darkness', kind: 'scroll', slot: 'scroll',
    value: 180, tier: 3, glyph: '?', color: '#ecf0f1',
    description: 'Smothers all light, cloaking the reader in shadow.'
  },

  // ==========================================================================
  // WEAPONS (25)
  // ==========================================================================
  dagger: {
    id: 'dagger', name: 'Dagger', kind: 'weapon', slot: 'weapon',
    atk: 3, value: 25, tier: 1, glyph: '|', color: '#ecf0f1',
    description: 'A short, quick blade favored by those who strike first.'
  },
  short_sword: {
    id: 'short_sword', name: 'Short Sword', kind: 'weapon', slot: 'weapon',
    atk: 5, value: 50, tier: 1, glyph: '/', color: '#ecf0f1',
    description: 'A reliable one-handed sword, standard issue for adventurers.'
  },
  long_sword: {
    id: 'long_sword', name: 'Long Sword', kind: 'weapon', slot: 'weapon',
    atk: 8, value: 120, tier: 1, glyph: '/', color: '#ecf0f1',
    description: 'A knightly blade with reach and balance.'
  },
  greatsword: {
    id: 'greatsword', name: 'Greatsword', kind: 'weapon', slot: 'weapon',
    atk: 12, value: 250, tier: 2, glyph: '/', color: '#ecf0f1', twoHanded: true,
    description: 'A massive two-handed sword that cleaves through armor.'
  },
  axe: {
    id: 'axe', name: 'Battle Axe', kind: 'weapon', slot: 'weapon',
    atk: 7, value: 100, tier: 1, glyph: 'P', color: '#ecf0f1',
    description: 'A heavy-bladed axe built for war, not woodcutting.'
  },
  battle_axe: {
    id: 'battle_axe', name: 'Great Axe', kind: 'weapon', slot: 'weapon',
    atk: 14, value: 300, tier: 2, glyph: 'P', color: '#ecf0f1',
    description: 'An enormous axe that trades finesse for devastating blows.'
  },
  mace: {
    id: 'mace', name: 'Mace', kind: 'weapon', slot: 'weapon',
    atk: 6, def: 1, value: 90, tier: 1, glyph: 'T', color: '#ecf0f1',
    description: 'A flanged club that crushes bone and dents plate.'
  },
  war_hammer: {
    id: 'war_hammer', name: 'War Hammer', kind: 'weapon', slot: 'weapon',
    atk: 10, def: 2, value: 220, tier: 2, glyph: 'T', color: '#ecf0f1',
    description: 'A brutal hammer whose weight doubles as a guard.'
  },
  spear: {
    id: 'spear', name: 'Spear', kind: 'weapon', slot: 'weapon',
    atk: 6, value: 80, tier: 1, glyph: '|', color: '#ecf0f1',
    description: 'A simple polearm that keeps enemies at a respectful distance.'
  },
  halberd: {
    id: 'halberd', name: 'Halberd', kind: 'weapon', slot: 'weapon',
    atk: 11, def: 1, value: 260, tier: 2, glyph: '|', color: '#ecf0f1', twoHanded: true,
    description: 'Axe, hook, and spike on a long haft — a soldier’s answer to everything.'
  },
  staff: {
    id: 'staff', name: 'Staff', kind: 'weapon', slot: 'weapon',
    atk: 4, bonuses: { hp: 0, atk: 0, def: 0, mana: 20, spd: 0 },
    value: 110, tier: 1, glyph: '/', color: '#ecf0f1', twoHanded: true,
    description: 'A carved focus that channels mana as readily as it cracks skulls.'
  },
  bow: {
    id: 'bow', name: 'Bow', kind: 'weapon', slot: 'weapon',
    atk: 7, value: 120, tier: 2, glyph: '}', color: '#ecf0f1', twoHanded: true, ranged: true,
    description: 'A hunter’s bow for striking foes before they close in.'
  },
  crossbow: {
    id: 'crossbow', name: 'Crossbow', kind: 'weapon', slot: 'weapon',
    atk: 10, value: 200, tier: 2, glyph: '}', color: '#ecf0f1', ranged: true,
    description: 'A mechanical bow that punches bolts through mail.'
  },
  wand: {
    id: 'wand', name: 'Wand', kind: 'weapon', slot: 'weapon',
    atk: 3, bonuses: { hp: 0, atk: 0, def: 0, mana: 30, spd: 0 },
    value: 130, tier: 1, glyph: '/', color: '#ecf0f1',
    description: 'A slender rod humming with stored arcane energy.'
  },
  scythe: {
    id: 'scythe', name: 'Scythe', kind: 'weapon', slot: 'weapon',
    atk: 15, value: 320, tier: 2, glyph: '7', color: '#ecf0f1', twoHanded: true,
    description: 'A reaper’s blade repurposed for a grimmer harvest.'
  },
  katana: {
    id: 'katana', name: 'Katana', kind: 'weapon', slot: 'weapon',
    atk: 11, value: 280, tier: 3, glyph: '/', color: '#ecf0f1',
    description: 'A folded-steel blade of exceptional sharpness.'
  },
  rapier: {
    id: 'rapier', name: 'Rapier', kind: 'weapon', slot: 'weapon',
    atk: 9, def: 1, value: 200, tier: 3, glyph: '|', color: '#ecf0f1',
    description: 'A duelist’s needle that darts past shields and parries.'
  },
  flail: {
    id: 'flail', name: 'Flail', kind: 'weapon', slot: 'weapon',
    atk: 10, value: 210, tier: 3, glyph: 'T', color: '#ecf0f1',
    description: 'A spiked ball on a chain that swings around any guard.'
  },
  morningstar: {
    id: 'morningstar', name: 'Morningstar', kind: 'weapon', slot: 'weapon',
    atk: 12, def: 1, value: 290, tier: 3, glyph: 'T', color: '#ecf0f1',
    description: 'A spiked mace that greets the dawn with broken helms.'
  },
  trident: {
    id: 'trident', name: 'Trident', kind: 'weapon', slot: 'weapon',
    atk: 10, def: 2, value: 260, tier: 3, glyph: '|', color: '#ecf0f1',
    description: 'A three-pronged spear that pins foes and turns blades.'
  },
  flame_sword: {
    id: 'flame_sword', name: 'Flame Sword', kind: 'weapon', slot: 'weapon',
    atk: 14, bonuses: { hp: 0, atk: 0, def: 0, mana: 5, spd: 0 },
    value: 500, tier: 4, glyph: '/', color: '#ecf0f1',
    description: 'A blade wreathed in ever-burning fire.'
  },
  frost_blade: {
    id: 'frost_blade', name: 'Frost Blade', kind: 'weapon', slot: 'weapon',
    atk: 13, bonuses: { hp: 0, atk: 0, def: 0, mana: 5, spd: 0 },
    value: 480, tier: 4, glyph: '/', color: '#ecf0f1',
    description: 'A sword of rimed steel that chills to the bone.'
  },
  thunder_axe: {
    id: 'thunder_axe', name: 'Thunder Axe', kind: 'weapon', slot: 'weapon',
    atk: 16, bonuses: { hp: 0, atk: 0, def: 0, mana: 10, spd: 0 },
    value: 600, tier: 4, glyph: 'P', color: '#ecf0f1',
    description: 'Each swing lands with a crack of rolling thunder.'
  },
  void_staff: {
    id: 'void_staff', name: 'Void Staff', kind: 'weapon', slot: 'weapon',
    atk: 8, bonuses: { hp: 0, atk: 0, def: 0, mana: 50, spd: 0 },
    value: 650, tier: 4, glyph: '/', color: '#ecf0f1',
    description: 'A staff carved from nothing, brimming with impossible mana.'
  },
  demon_slayer: {
    id: 'demon_slayer', name: 'Demon Slayer', kind: 'weapon', slot: 'weapon',
    atk: 20, value: 800, tier: 4, glyph: '7', color: '#ecf0f1',
    description: 'A legendary blade forged to end the Demon King himself.'
  },

  // ==========================================================================
  // SHIELDS (10)
  // ==========================================================================
  buckler: {
    id: 'buckler', name: 'Buckler', kind: 'shield', slot: 'shield',
    def: 2, value: 30, tier: 1, glyph: ')', color: '#ecf0f1',
    description: 'A small fist-shield for deflecting quick strikes.'
  },
  wooden_shield: {
    id: 'wooden_shield', name: 'Wooden Shield', kind: 'shield', slot: 'shield',
    def: 3, value: 45, tier: 1, glyph: ')', color: '#ecf0f1',
    description: 'Planks and iron banding — humble but dependable.'
  },
  iron_shield: {
    id: 'iron_shield', name: 'Iron Shield', kind: 'shield', slot: 'shield',
    def: 5, value: 80, tier: 2, glyph: ')', color: '#ecf0f1',
    description: 'A solid iron shield that shrugs off most blows.'
  },
  tower_shield: {
    id: 'tower_shield', name: 'Tower Shield', kind: 'shield', slot: 'shield',
    def: 8, value: 180, tier: 3, glyph: ')', color: '#ecf0f1',
    description: 'A full-body shield that turns its bearer into a wall.'
  },
  magic_shield: {
    id: 'magic_shield', name: 'Magic Shield', kind: 'shield', slot: 'shield',
    def: 6, bonuses: { hp: 0, atk: 0, def: 0, mana: 10, spd: 0 },
    value: 200, tier: 3, glyph: ')', color: '#ecf0f1',
    description: 'A rune-etched shield that also stores a reserve of mana.'
  },
  dragon_shield: {
    id: 'dragon_shield', name: 'Dragon Shield', kind: 'shield', slot: 'shield',
    def: 10, bonuses: { hp: 10, atk: 0, def: 0, mana: 0, spd: 0 },
    value: 450, tier: 3, glyph: ')', color: '#ecf0f1',
    description: 'Forged from dragon scales; its warmth hardens the bearer.'
  },
  spiked_shield: {
    id: 'spiked_shield', name: 'Spiked Shield', kind: 'shield', slot: 'shield',
    atk: 3, def: 6, value: 220, tier: 4, glyph: ')', color: '#ecf0f1',
    description: 'A shield studded with spikes — blocking it hurts too.'
  },
  mirror_shield: {
    id: 'mirror_shield', name: 'Mirror Shield', kind: 'shield', slot: 'shield',
    def: 7, bonuses: { hp: 0, atk: 0, def: 0, mana: 15, spd: 0 },
    value: 320, tier: 4, glyph: ')', color: '#ecf0f1',
    description: 'A polished shield said to turn spells back on their casters.'
  },
  phoenix_shield: {
    id: 'phoenix_shield', name: 'Phoenix Shield', kind: 'shield', slot: 'shield',
    def: 9, bonuses: { hp: 15, atk: 0, def: 0, mana: 5, spd: 0 },
    value: 500, tier: 4, glyph: ')', color: '#ecf0f1',
    description: 'Feather-light and warm, blessed by the undying firebird.'
  },
  abyssal_shield: {
    id: 'abyssal_shield', name: 'Abyssal Shield', kind: 'shield', slot: 'shield',
    atk: 2, def: 12, bonuses: { hp: 0, atk: 0, def: 0, mana: 10, spd: 0 },
    value: 600, tier: 4, glyph: ')', color: '#ecf0f1',
    description: 'A shield of black depths that swallows incoming blows.'
  },

  // ==========================================================================
  // ARMOR (12)
  // ==========================================================================
  leather_armor: {
    id: 'leather_armor', name: 'Leather Armor', kind: 'armor', slot: 'armor',
    def: 3, value: 60, tier: 1, glyph: '[', color: '#ecf0f1',
    description: 'Boiled leather that stops teeth and glancing blades.'
  },
  chain_mail: {
    id: 'chain_mail', name: 'Chain Mail', kind: 'armor', slot: 'armor',
    def: 5, value: 150, tier: 1, glyph: '[', color: '#ecf0f1',
    description: 'Interlocked rings that spread the force of every hit.'
  },
  scale_mail: {
    id: 'scale_mail', name: 'Scale Mail', kind: 'armor', slot: 'armor',
    def: 7, value: 220, tier: 2, glyph: '[', color: '#ecf0f1',
    description: 'Overlapping metal scales, flexible yet stout.'
  },
  plate_mail: {
    id: 'plate_mail', name: 'Plate Mail', kind: 'armor', slot: 'armor',
    def: 10, value: 350, tier: 2, glyph: '[', color: '#ecf0f1',
    description: 'Full plate armor — heavy, loud, and nearly impenetrable.'
  },
  dragon_armor: {
    id: 'dragon_armor', name: 'Dragon Armor', kind: 'armor', slot: 'armor',
    def: 15, bonuses: { hp: 20, atk: 0, def: 0, mana: 0, spd: 0 },
    value: 700, tier: 3, glyph: '[', color: '#ecf0f1',
    description: 'Armor wrought from dragon hide, pulsing with vitality.'
  },
  mage_robes: {
    id: 'mage_robes', name: 'Mage Robes', kind: 'armor', slot: 'armor',
    def: 2, bonuses: { hp: 0, atk: 0, def: 0, mana: 30, spd: 0 },
    value: 120, tier: 1, glyph: '[', color: '#ecf0f1',
    description: 'Enchanted robes woven to hold and amplify mana.'
  },
  assassin_garb: {
    id: 'assassin_garb', name: "Assassin's Garb", kind: 'armor', slot: 'armor',
    atk: 3, def: 4, value: 250, tier: 3, glyph: '[', color: '#ecf0f1',
    description: 'Dark fitted cloth with hidden blades sewn into the seams.'
  },
  holy_armor: {
    id: 'holy_armor', name: 'Holy Armor', kind: 'armor', slot: 'armor',
    def: 12, bonuses: { hp: 10, atk: 0, def: 0, mana: 10, spd: 0 },
    value: 550, tier: 4, glyph: '[', color: '#ecf0f1',
    description: 'Consecrated plate that steadies both body and spirit.'
  },
  demon_armor: {
    id: 'demon_armor', name: 'Demon Armor', kind: 'armor', slot: 'armor',
    atk: 5, def: 14, value: 650, tier: 4, glyph: '[', color: '#ecf0f1',
    description: 'Infernal armor that hungers for battle as much as its wearer.'
  },
  crystal_armor: {
    id: 'crystal_armor', name: 'Crystal Armor', kind: 'armor', slot: 'armor',
    def: 11, bonuses: { hp: 0, atk: 0, def: 0, mana: 25, spd: 0 },
    value: 500, tier: 4, glyph: '[', color: '#ecf0f1',
    description: 'Faceted crystal plates that refract blows and store mana.'
  },
  shadow_cloak: {
    id: 'shadow_cloak', name: 'Shadow Cloak', kind: 'armor', slot: 'armor',
    atk: 3, def: 6, bonuses: { hp: 0, atk: 0, def: 0, mana: 15, spd: 0 },
    value: 400, tier: 4, glyph: '[', color: '#ecf0f1',
    description: 'A cloak of living shadow that sharpens the knife in the dark.'
  },
  titan_plate: {
    id: 'titan_plate', name: 'Titan Plate', kind: 'armor', slot: 'armor',
    def: 18, bonuses: { hp: 30, atk: 0, def: 0, mana: 0, spd: 0 },
    value: 900, tier: 4, glyph: '[', color: '#ecf0f1',
    description: 'Armor scaled for titans, granting colossal endurance.'
  },

  // ==========================================================================
  // HELMETS (10)
  // ==========================================================================
  leather_cap: {
    id: 'leather_cap', name: 'Leather Cap', kind: 'helmet', slot: 'helmet',
    def: 1, value: 25, tier: 1, glyph: '^', color: '#ecf0f1',
    description: 'A simple cap that is better than nothing.'
  },
  iron_helm: {
    id: 'iron_helm', name: 'Iron Helm', kind: 'helmet', slot: 'helmet',
    def: 3, value: 60, tier: 1, glyph: '^', color: '#ecf0f1',
    description: 'A sturdy iron helmet with a nose guard.'
  },
  steel_helm: {
    id: 'steel_helm', name: 'Steel Helm', kind: 'helmet', slot: 'helmet',
    def: 5, value: 110, tier: 1, glyph: '^', color: '#ecf0f1',
    description: 'Fine steel, well-tempered and battle-proven.'
  },
  crown_of_kings: {
    id: 'crown_of_kings', name: 'Crown of Kings', kind: 'helmet', slot: 'helmet',
    atk: 2, def: 3, bonuses: { hp: 20, atk: 0, def: 0, mana: 20, spd: 0 },
    value: 600, tier: 2, glyph: '^', color: '#ecf0f1',
    description: 'A golden crown that carries the authority of ancient rulers.'
  },
  wizard_hat: {
    id: 'wizard_hat', name: "Wizard's Hat", kind: 'helmet', slot: 'helmet',
    def: 1, bonuses: { hp: 0, atk: 0, def: 0, mana: 20, spd: 0 },
    value: 90, tier: 2, glyph: '^', color: '#ecf0f1',
    description: 'A pointed hat stitched with mana-conducting thread.'
  },
  demon_skull: {
    id: 'demon_skull', name: 'Demon Skull', kind: 'helmet', slot: 'helmet',
    atk: 5, def: 2, value: 300, tier: 2, glyph: '^', color: '#ecf0f1',
    description: 'A horned skull worn as a helm; it whispers of violence.'
  },
  dragon_helm: {
    id: 'dragon_helm', name: 'Dragon Helm', kind: 'helmet', slot: 'helmet',
    atk: 2, def: 6, bonuses: { hp: 10, atk: 0, def: 0, mana: 0, spd: 0 },
    value: 400, tier: 2, glyph: '^', color: '#ecf0f1',
    description: 'A helm crested with dragon horn, fierce and enduring.'
  },
  crystal_crown: {
    id: 'crystal_crown', name: 'Crystal Crown', kind: 'helmet', slot: 'helmet',
    def: 4, bonuses: { hp: 10, atk: 0, def: 0, mana: 25, spd: 0 },
    value: 450, tier: 2, glyph: '^', color: '#ecf0f1',
    description: 'A circlet of pure crystal that hums with arcane resonance.'
  },
  hood_of_shadows: {
    id: 'hood_of_shadows', name: 'Hood of Shadows', kind: 'helmet', slot: 'helmet',
    atk: 2, def: 2, bonuses: { hp: 0, atk: 0, def: 0, mana: 15, spd: 0 },
    value: 250, tier: 2, glyph: '^', color: '#ecf0f1',
    description: 'A deep hood that keeps the wearer’s face — and intent — hidden.'
  },
  helm_of_valor: {
    id: 'helm_of_valor', name: 'Helm of Valor', kind: 'helmet', slot: 'helmet',
    atk: 3, def: 5, bonuses: { hp: 15, atk: 0, def: 0, mana: 0, spd: 0 },
    value: 380, tier: 2, glyph: '^', color: '#ecf0f1',
    description: 'A hero’s helm that steels the heart against fear.'
  },

  // ==========================================================================
  // GLOVES (8)
  // ==========================================================================
  leather_gloves: {
    id: 'leather_gloves', name: 'Leather Gloves', kind: 'gloves', slot: 'gloves',
    atk: 1, value: 20, tier: 1, glyph: '{', color: '#ecf0f1',
    description: 'Supple gloves that improve your grip on any weapon.'
  },
  iron_gauntlets: {
    id: 'iron_gauntlets', name: 'Iron Gauntlets', kind: 'gloves', slot: 'gloves',
    atk: 2, def: 1, value: 45, tier: 1, glyph: '{', color: '#ecf0f1',
    description: 'Articulated iron gauntlets that add weight to every punch.'
  },
  gloves_of_power: {
    id: 'gloves_of_power', name: 'Gloves of Power', kind: 'gloves', slot: 'gloves',
    atk: 5, value: 200, tier: 2, glyph: '{', color: '#ecf0f1',
    description: 'Enchanted gloves that lend inhuman strength to each blow.'
  },
  thieves_gloves: {
    id: 'thieves_gloves', name: "Thief's Gloves", kind: 'gloves', slot: 'gloves',
    atk: 3, value: 120, tier: 3, glyph: '{', color: '#ecf0f1',
    description: 'Silent, fingerless gloves made for quick and dirty work.'
  },
  dragon_gauntlets: {
    id: 'dragon_gauntlets', name: 'Dragon Gauntlets', kind: 'gloves', slot: 'gloves',
    atk: 4, def: 3, value: 320, tier: 3, glyph: '{', color: '#ecf0f1',
    description: 'Clawed gauntlets of dragon scale, hot to the touch.'
  },
  frost_gauntlets: {
    id: 'frost_gauntlets', name: 'Frost Gauntlets', kind: 'gloves', slot: 'gloves',
    atk: 3, def: 2, bonuses: { hp: 0, atk: 0, def: 0, mana: 10, spd: 0 },
    value: 220, tier: 3, glyph: '{', color: '#ecf0f1',
    description: 'Rime-crusted gauntlets that numb whatever they strike.'
  },
  flame_gauntlets: {
    id: 'flame_gauntlets', name: 'Flame Gauntlets', kind: 'gloves', slot: 'gloves',
    atk: 5, def: 1, bonuses: { hp: 0, atk: 0, def: 0, mana: 5, spd: 0 },
    value: 260, tier: 3, glyph: '{', color: '#ecf0f1',
    description: 'Smoldering gauntlets that sear with every touch.'
  },
  gauntlets_of_might: {
    id: 'gauntlets_of_might', name: 'Gauntlets of Might', kind: 'gloves', slot: 'gloves',
    atk: 7, def: 2, bonuses: { hp: 5, atk: 0, def: 0, mana: 0, spd: 0 },
    value: 420, tier: 3, glyph: '{', color: '#ecf0f1',
    description: 'Legendary gauntlets said to let a man arm-wrestle a giant.'
  },

  // ==========================================================================
  // BOOTS (8)
  // ==========================================================================
  leather_boots: {
    id: 'leather_boots', name: 'Leather Boots', kind: 'boots', slot: 'boots',
    def: 1, value: 20, tier: 1, glyph: '}', color: '#ecf0f1',
    description: 'Worn but comfortable boots for long dungeon marches.'
  },
  iron_boots: {
    id: 'iron_boots', name: 'Iron Boots', kind: 'boots', slot: 'boots',
    def: 2, value: 45, tier: 1, glyph: '}', color: '#ecf0f1',
    description: 'Heavy iron boots that protect shins and toes alike.'
  },
  boots_of_speed: {
    id: 'boots_of_speed', name: 'Boots of Speed', kind: 'boots', slot: 'boots',
    def: 1, value: 150, tier: 1, glyph: '}', color: '#ecf0f1',
    description: 'Feather-stitched boots that make every step feel lighter.'
  },
  boots_of_leaping: {
    id: 'boots_of_leaping', name: 'Boots of Leaping', kind: 'boots', slot: 'boots',
    def: 1, value: 130, tier: 3, glyph: '}', color: '#ecf0f1',
    description: 'Springy boots that carry their wearer over gaps and foes.'
  },
  winged_boots: {
    id: 'winged_boots', name: 'Winged Boots', kind: 'boots', slot: 'boots',
    def: 2, bonuses: { hp: 0, atk: 0, def: 0, mana: 10, spd: 0 },
    value: 250, tier: 3, glyph: '}', color: '#ecf0f1',
    description: 'Small wings at the ankles lift the wearer just off the ground.'
  },
  shadow_boots: {
    id: 'shadow_boots', name: 'Shadow Boots', kind: 'boots', slot: 'boots',
    atk: 2, def: 1, bonuses: { hp: 0, atk: 0, def: 0, mana: 10, spd: 0 },
    value: 220, tier: 3, glyph: '}', color: '#ecf0f1',
    description: 'Boots that make no sound, even on gravel and glass.'
  },
  lava_walkers: {
    id: 'lava_walkers', name: 'Lava Walkers', kind: 'boots', slot: 'boots',
    def: 3, value: 240, tier: 3, glyph: '}', color: '#ecf0f1',
    description: 'Obsidian-soled boots said to tread safely across molten rock.'
  },
  boots_of_the_wind: {
    id: 'boots_of_the_wind', name: 'Boots of the Wind', kind: 'boots', slot: 'boots',
    def: 2, bonuses: { hp: 0, atk: 0, def: 0, mana: 15, spd: 0 },
    value: 300, tier: 3, glyph: '}', color: '#ecf0f1',
    description: 'A tailwind follows whoever laces these boots.'
  },

  // ==========================================================================
  // RINGS (15)
  // ==========================================================================
  ring_of_strength: {
    id: 'ring_of_strength', name: 'Ring of Strength', kind: 'ring', slot: 'ring',
    atk: 5, value: 200, tier: 1, glyph: 'o', color: '#ecf0f1',
    description: 'A heavy band that swells the wearer’s muscles with power.'
  },
  ring_of_protection: {
    id: 'ring_of_protection', name: 'Ring of Protection', kind: 'ring', slot: 'ring',
    def: 5, value: 200, tier: 1, glyph: 'o', color: '#ecf0f1',
    description: 'A warded ring that turns aside blades and claws.'
  },
  ring_of_speed: {
    id: 'ring_of_speed', name: 'Ring of Speed', kind: 'ring', slot: 'ring',
    value: 180, tier: 1, glyph: 'o', color: '#ecf0f1',
    description: 'A quicksilver band said to hasten its wearer’s step.'
  },
  ring_of_regeneration: {
    id: 'ring_of_regeneration', name: 'Ring of Regeneration', kind: 'ring', slot: 'ring',
    bonuses: { hp: 10, atk: 0, def: 0, mana: 0, spd: 0 },
    value: 220, tier: 1, glyph: 'o', color: '#ecf0f1',
    description: 'A living ring that knits flesh and bolsters vitality.'
  },
  ring_of_fireball: {
    id: 'ring_of_fireball', name: 'Ring of Fireball', kind: 'ring', slot: 'ring',
    atk: 3, bonuses: { hp: 0, atk: 0, def: 0, mana: 10, spd: 0 },
    value: 250, tier: 2, glyph: 'o', color: '#ecf0f1',
    description: 'A ruby ring warm to the touch, crackling with fire magic.'
  },
  ring_of_invisibility: {
    id: 'ring_of_invisibility', name: 'Ring of Invisibility', kind: 'ring', slot: 'ring',
    value: 300, tier: 2, glyph: 'o', color: '#ecf0f1',
    description: 'A plain gold band rumored to hide its wearer from sight.'
  },
  ring_of_the_vampire: {
    id: 'ring_of_the_vampire', name: 'Vampire Ring', kind: 'ring', slot: 'ring',
    atk: 3, value: 280, tier: 2, glyph: 'o', color: '#ecf0f1',
    description: 'A blood-red ring that thirsts alongside its wearer.'
  },
  ring_of_mana: {
    id: 'ring_of_mana', name: 'Ring of Mana', kind: 'ring', slot: 'ring',
    bonuses: { hp: 0, atk: 0, def: 0, mana: 30, spd: 0 },
    value: 180, tier: 1, glyph: 'o', color: '#ecf0f1',
    description: 'A sapphire ring that deepens the wearer’s well of mana.'
  },
  ring_of_luck: {
    id: 'ring_of_luck', name: 'Ring of Luck', kind: 'ring', slot: 'ring',
    atk: 1, def: 1, bonuses: { hp: 5, atk: 0, def: 0, mana: 5, spd: 0 },
    value: 260, tier: 2, glyph: 'o', color: '#ecf0f1',
    description: 'A charm-laden ring that nudges fate ever so slightly.'
  },
  ring_of_death: {
    id: 'ring_of_death', name: 'Ring of Death', kind: 'ring', slot: 'ring',
    atk: 10, bonuses: { hp: -20, atk: 0, def: 0, mana: 0, spd: 0 },
    value: 400, tier: 2, glyph: 'o', color: '#ecf0f1',
    description: 'Terrible power at a terrible price: it feeds on your life force.'
  },
  ring_of_frost: {
    id: 'ring_of_frost', name: 'Ring of Frost', kind: 'ring', slot: 'ring',
    atk: 2, bonuses: { hp: 0, atk: 0, def: 0, mana: 15, spd: 0 },
    value: 240, tier: 2, glyph: 'o', color: '#ecf0f1',
    description: 'An ice-blue ring that rimes the wearer’s knuckles with frost.'
  },
  ring_of_flame: {
    id: 'ring_of_flame', name: 'Ring of Flame', kind: 'ring', slot: 'ring',
    atk: 4, bonuses: { hp: 0, atk: 0, def: 0, mana: 10, spd: 0 },
    value: 260, tier: 2, glyph: 'o', color: '#ecf0f1',
    description: 'A band of embered gold that never quite cools.'
  },
  ring_of_thunder: {
    id: 'ring_of_thunder', name: 'Ring of Thunder', kind: 'ring', slot: 'ring',
    atk: 5, bonuses: { hp: 0, atk: 0, def: 0, mana: 20, spd: 0 },
    value: 320, tier: 2, glyph: 'o', color: '#ecf0f1',
    description: 'A storm-forged ring that hums with static charge.'
  },
  ring_of_shadows: {
    id: 'ring_of_shadows', name: 'Ring of Shadows', kind: 'ring', slot: 'ring',
    def: 2, bonuses: { hp: 0, atk: 0, def: 0, mana: 20, spd: 0 },
    value: 280, tier: 2, glyph: 'o', color: '#ecf0f1',
    description: 'A ring of dark metal that drinks in the surrounding light.'
  },
  ring_of_the_ancients: {
    id: 'ring_of_the_ancients', name: 'Ring of the Ancients', kind: 'ring', slot: 'ring',
    atk: 6, def: 3, bonuses: { hp: 15, atk: 0, def: 0, mana: 15, spd: 0 },
    value: 500, tier: 3, glyph: 'o', color: '#ecf0f1',
    description: 'A relic of a forgotten age, strong in every respect.'
  },

  // ==========================================================================
  // AMULETS (12)
  // ==========================================================================
  amulet_of_health: {
    id: 'amulet_of_health', name: 'Amulet of Health', kind: 'amulet', slot: 'amulet',
    bonuses: { hp: 30, atk: 0, def: 0, mana: 0, spd: 0 },
    value: 250, tier: 1, glyph: '"', color: '#ecf0f1',
    description: 'A warm pendant that fortifies the body against harm.'
  },
  amulet_of_mana: {
    id: 'amulet_of_mana', name: 'Amulet of Mana', kind: 'amulet', slot: 'amulet',
    bonuses: { hp: 0, atk: 0, def: 0, mana: 40, spd: 0 },
    value: 260, tier: 1, glyph: '"', color: '#ecf0f1',
    description: 'A crystalline pendant that greatly expands the wearer’s mana.'
  },
  amulet_of_protection: {
    id: 'amulet_of_protection', name: 'Amulet of Protection', kind: 'amulet', slot: 'amulet',
    def: 8, value: 320, tier: 1, glyph: '"', color: '#ecf0f1',
    description: 'A warded talisman that blunts incoming blows.'
  },
  amulet_of_power: {
    id: 'amulet_of_power', name: 'Amulet of Power', kind: 'amulet', slot: 'amulet',
    atk: 8, value: 320, tier: 1, glyph: '"', color: '#ecf0f1',
    description: 'A heavy talisman that lends brutal force to every strike.'
  },
  amulet_of_wisdom: {
    id: 'amulet_of_wisdom', name: 'Amulet of Wisdom', kind: 'amulet', slot: 'amulet',
    bonuses: { hp: 0, atk: 0, def: 0, mana: 50, spd: 0 },
    value: 300, tier: 1, glyph: '"', color: '#ecf0f1',
    description: 'An ancient scholar’s pendant brimming with arcane insight.'
  },
  amulet_of_life: {
    id: 'amulet_of_life', name: 'Amulet of Life', kind: 'amulet', slot: 'amulet',
    bonuses: { hp: 50, atk: 0, def: 0, mana: 0, spd: 0 },
    value: 400, tier: 2, glyph: '"', color: '#ecf0f1',
    description: 'A verdant charm pulsing with pure life energy.'
  },
  amulet_of_death: {
    id: 'amulet_of_death', name: 'Amulet of Death', kind: 'amulet', slot: 'amulet',
    atk: 15, bonuses: { hp: -30, atk: 0, def: 0, mana: 0, spd: 0 },
    value: 500, tier: 3, glyph: '"', color: '#ecf0f1',
    description: 'A skull-carved amulet trading life essence for killing power.'
  },
  amulet_of_the_gods: {
    id: 'amulet_of_the_gods', name: 'Amulet of the Gods', kind: 'amulet', slot: 'amulet',
    atk: 5, def: 5, bonuses: { hp: 25, atk: 0, def: 0, mana: 25, spd: 0 },
    value: 800, tier: 3, glyph: '"', color: '#ecf0f1',
    description: 'A divine relic that blesses every aspect of its bearer.'
  },
  amulet_of_dragons: {
    id: 'amulet_of_dragons', name: 'Amulet of Dragons', kind: 'amulet', slot: 'amulet',
    atk: 8, def: 4, bonuses: { hp: 20, atk: 0, def: 0, mana: 10, spd: 0 },
    value: 700, tier: 3, glyph: '"', color: '#ecf0f1',
    description: 'A fang-and-scale talisman carrying draconic might.'
  },
  amulet_of_chaos: {
    id: 'amulet_of_chaos', name: 'Amulet of Chaos', kind: 'amulet', slot: 'amulet',
    atk: 12, bonuses: { hp: -10, atk: 0, def: 0, mana: 30, spd: 0 },
    value: 600, tier: 3, glyph: '"', color: '#ecf0f1',
    description: 'An ever-shifting stone of wild power — potent but unstable.'
  },
  amulet_of_order: {
    id: 'amulet_of_order', name: 'Amulet of Order', kind: 'amulet', slot: 'amulet',
    def: 10, bonuses: { hp: 20, atk: 0, def: 0, mana: 20, spd: 0 },
    value: 600, tier: 3, glyph: '"', color: '#ecf0f1',
    description: 'A perfectly symmetrical talisman radiating calm resilience.'
  },
  amulet_of_balance: {
    id: 'amulet_of_balance', name: 'Amulet of Balance', kind: 'amulet', slot: 'amulet',
    atk: 6, def: 6, bonuses: { hp: 20, atk: 0, def: 0, mana: 20, spd: 0 },
    value: 650, tier: 3, glyph: '"', color: '#ecf0f1',
    description: 'Twin stones in equilibrium, strengthening body and mind alike.'
  },

  // ==========================================================================
  // FOOD (8)
  // ==========================================================================
  bread: {
    id: 'bread', name: 'Bread', kind: 'food', slot: 'food',
    hungerRestore: 25, value: 5, tier: 1, glyph: '%', color: '#ecf0f1',
    description: 'A dense loaf of dungeon bread. Restores 25 hunger.'
  },
  meat: {
    id: 'meat', name: 'Meat', kind: 'food', slot: 'food',
    hungerRestore: 40, value: 8, tier: 1, glyph: '%', color: '#ecf0f1',
    description: 'A hearty cut of cooked meat. Restores 40 hunger.'
  },
  apple: {
    id: 'apple', name: 'Apple', kind: 'food', slot: 'food',
    hungerRestore: 10, value: 3, tier: 1, glyph: '%', color: '#ecf0f1',
    description: 'A crisp apple. Restores 10 hunger.'
  },
  cheese: {
    id: 'cheese', name: 'Cheese', kind: 'food', slot: 'food',
    hungerRestore: 20, value: 4, tier: 1, glyph: '%', color: '#ecf0f1',
    description: 'A wedge of sharp cheese. Restores 20 hunger.'
  },
  feast: {
    id: 'feast', name: 'Royal Feast', kind: 'food', slot: 'food',
    hungerRestore: 100, value: 50, tier: 2, glyph: '%', color: '#ecf0f1',
    description: 'A banquet fit for a king. Restores 100 hunger.'
  },
  dragon_fruit: {
    id: 'dragon_fruit', name: 'Dragon Fruit', kind: 'food', slot: 'food',
    hungerRestore: 30, value: 25, tier: 2, glyph: '%', color: '#ecf0f1',
    description: 'An exotic scaled fruit, faintly warm. Restores 30 hunger.'
  },
  ancient_wine: {
    id: 'ancient_wine', name: 'Ancient Wine', kind: 'food', slot: 'food',
    hungerRestore: 35, value: 30, tier: 2, glyph: '%', color: '#ecf0f1',
    description: 'A dusty vintage from a fallen kingdom. Restores 35 hunger.'
  },
  golden_apple: {
    id: 'golden_apple', name: 'Golden Apple', kind: 'food', slot: 'food',
    hungerRestore: 50, value: 40, tier: 2, glyph: '%', color: '#ecf0f1',
    description: 'A gleaming apple of solid gold hue. Restores 50 hunger.'
  },

  // ==========================================================================
  // SPECIAL (10)
  // ==========================================================================
  gold: {
    id: 'gold', name: 'Gold', kind: 'special', slot: 'special',
    effect: 'gold:5-25*(1+floor/3)', value: 1, tier: 1, glyph: '$', color: '#f1c40f',
    description: 'A pile of coins. Picked up automatically, worth 5-25 gold scaled by floor.'
  },
  key: {
    id: 'key', name: 'Key', kind: 'special', slot: 'special',
    effect: 'key:+1', value: 25, tier: 1, glyph: 'k', color: '#f1c40f',
    description: 'A dungeon key. Picked up automatically and added to your key count.'
  },
  bomb: {
    id: 'bomb', name: 'Bomb', kind: 'special', slot: 'special',
    effect: 'aoe:burn:50:r3', value: 50, tier: 1, glyph: '*', color: '#e74c3c',
    description: 'Explodes for 50 fire damage, burning enemies within a radius of 3.'
  },
  torch: {
    id: 'torch', name: 'Torch', kind: 'special', slot: 'special',
    value: 10, tier: 1, glyph: '(', color: '#f1c40f',
    description: 'A burning brand used to temporarily increase visibility in the dark.'
  },
  compass: {
    id: 'compass', name: 'Compass', kind: 'special', slot: 'special',
    value: 40, tier: 3, glyph: 'c', color: '#1abc9c',
    description: 'A brass compass whose needle points toward the unknown.'
  },
  teleport_crystal: {
    id: 'teleport_crystal', name: 'Teleport Crystal', kind: 'special', slot: 'special',
    value: 120, tier: 3, glyph: '+', color: '#ecf0f1',
    description: 'A crystal humming with spatial magic.'
  },
  soul_gem: {
    id: 'soul_gem', name: 'Soul Gem', kind: 'special', slot: 'special',
    value: 200, tier: 3, glyph: 'o', color: '#ecf0f1',
    description: 'A gem that flickers with a trapped, restless light.'
  },
  ancient_relic: {
    id: 'ancient_relic', name: 'Ancient Relic', kind: 'special', slot: 'special',
    value: 300, tier: 3, glyph: '*', color: '#ecf0f1',
    description: 'A priceless artifact from a civilization long buried.'
  },
  dragon_scale: {
    id: 'dragon_scale', name: 'Dragon Scale', kind: 'special', slot: 'special',
    value: 150, tier: 3, glyph: 's', color: '#ecf0f1',
    description: 'An iridescent scale shed by a true dragon.'
  },
  demon_heart: {
    id: 'demon_heart', name: 'Demon Heart', kind: 'special', slot: 'special',
    value: 250, tier: 3, glyph: 'h', color: '#ecf0f1',
    description: 'A still-beating heart torn from a demon. It radiates malice.'
  }
};

// Rarities ported verbatim from Rarity enum in src/main.rs
// (color(), prefix(), stat_bonus()) and README rarity table.
SC.DATA.rarities = [
  { id: 'common',    name: 'Common',    prefix: '',           multiplier: 1.0,  color: '#95a5a6' },
  { id: 'uncommon',  name: 'Uncommon',  prefix: 'Fine ',      multiplier: 1.25, color: '#2ecc71' },
  { id: 'rare',      name: 'Rare',      prefix: 'Superior ',  multiplier: 1.5,  color: '#3498db' },
  { id: 'epic',      name: 'Epic',      prefix: 'Epic ',      multiplier: 2.0,  color: '#d35ded' },
  { id: 'legendary', name: 'Legendary', prefix: 'Legendary ', multiplier: 3.0,  color: '#f1c40f' },
  { id: 'mythic',    name: 'Mythic',    prefix: 'Mythic ',    multiplier: 5.0,  color: '#e74c3c' }
];

SC.DATA.items_notes = [
  'Gold values: the Rust source has no per-item value function. Values are taken verbatim from src/npc.rs shop stock where available (health_potion 25 / mana_potion 30 at the merchant, torch 10, bomb 50, scroll_teleport 100, short_sword 50, long_sword 120, iron_shield 80, chain_mail 150, iron_helm 60, iron_gauntlets 45, iron_boots 45, strength_potion 60, defense_potion 60, speed_potion 50, poison_resist_potion 35, regeneration_potion 100, scroll_fireball 75, scroll_ice_storm 75, scroll_lightning 80, scroll_mapping 120, ring_of_protection 200, ring_of_mana 180); the alchemist sells health/mana potions cheaper (20/25). All other values were invented on a scale anchored to those shop prices.',
  'Descriptions are original flavor text: the Rust source has no per-item description strings. Where an item has an implemented mechanic (use_item effects, food_value, gold/key pickup), the description states those exact numbers.',
  'tier (1-4) is derived from the rarity-gated index ranges of the loot tables in random_item() (src/main.rs): tier 1 items can drop at Common rarity rolls, higher tiers require Rare/Epic+ rolls. Items appearing in multiple tables use their earliest (lowest) tier. The source has no explicit per-item tier or minFloor field.',
  'bonuses.spd is always 0: ItemKind::base_stats() in src/main.rs only defines (attack, defense, hp_bonus, mana_bonus); there is no numeric speed stat, so speed-themed items (Boots of Speed, Ring of Speed, Ring of Invisibility) carry no speed number in the source.',
  'effect strings exist only for consumables actually implemented in use_item() (src/main.rs ~7756). The following consumables fall through to "Can\'t use that item." in the source and therefore have no effect field: fire/ice resist, berserk, giant, levitation, luck, critical, vision, cure-all and ultimate power potions; scrolls of identify, enchant, summoning, banishment, time stop, death, earthquake, meteor, blizzard, chain lightning, divine wrath and darkness; torch, compass, teleport crystal, soul gem, ancient relic, dragon scale and demon heart.',
  'torch description ("temporarily increase visibility") comes from the README Field of View section; the effect is not implemented in use_item().',
  'twoHanded is derived from the ItemKind->WeaponType mapping in Item::new_advanced_weapon plus WeaponType::grip_type(): greatsword, halberd, staff, bow (LongBow) and scythe (Glaive) are TwoHanded. Versatile weapons (long_sword, katana, battle_axe, war_hammer, spear, trident) are treated as one-handed. Elemental/unique weapons (flame_sword, frost_blade, thunder_axe, void_staff, demon_slayer) hit the default LongSword arm of that mapping, so no grip data exists; they are treated as one-handed.',
  'Colors come verbatim from ItemKind::color() in src/main.rs; every item not explicitly listed there (all equipment, food, later-tier potions/scrolls, and most specials) uses that function\'s catch-all Color::White (#ecf0f1). In the terminal game items are actually rendered in their rarity color.',
  'Rarity prefixes keep their trailing space verbatim from Rarity::prefix() (display code concatenates prefix + name directly).',
  'hungerRestore is verbatim from ItemKind::food_value(). ranged is derived from WeaponCategory::Ranged (bow, crossbow).'
];
