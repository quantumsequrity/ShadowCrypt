//! Particle Effects System for ShadowCrypt GUI
//!
//! This module provides a comprehensive particle effects system for visual feedback
//! including combat effects (blood, sparks), magic effects (fire, ice, lightning),
//! and environmental effects (dust, ambient particles).

use eframe::egui::{self, Color32, Pos2, Vec2};
use shadowcrypt_core::prelude::*;
use shadowcrypt_core::combat::StatusEffect;
use shadowcrypt_core::magic::Skill;
use std::collections::VecDeque;

// ============================================================================
// PARTICLE TYPES
// ============================================================================

/// Types of particles that can be spawned
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParticleType {
    // Combat particles
    BloodSplatter,
    BloodDrop,
    Spark,
    MetalClash,
    CriticalHit,

    // Magic particles - Fire
    Flame,
    Ember,
    FireBurst,

    // Magic particles - Ice
    Snowflake,
    IceShard,
    FrostMist,

    // Magic particles - Lightning
    LightningBolt,
    ElectricSpark,
    StaticCharge,

    // Magic particles - Holy
    HolyLight,
    DivineSpark,
    Blessing,

    // Magic particles - Dark/Necromancy
    ShadowWisp,
    SoulFragment,
    DarkEnergy,
    Corruption,

    // Magic particles - Poison
    PoisonBubble,
    ToxicMist,
    VenomDrop,

    // Environmental particles
    Dust,
    DungeonDust,
    WaterDroplet,
    WaterRipple,
    LavaEmber,
    LavaBubble,
    Steam,
    Smoke,
    Fog,
    Leaf,
    Pollen,
    SnowParticle,
    Ash,

    // Special effects
    Teleport,
    LevelUp,
    ItemPickup,
    GoldSparkle,
    HealingGlow,
    ShieldShimmer,
    StealthFade,
}

// ============================================================================
// PARTICLE STRUCT
// ============================================================================

/// A single particle in the system
#[derive(Clone)]
pub struct Particle {
    pub particle_type: ParticleType,
    pub position: Pos2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub color: Color32,
    pub size: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub rotation: f32,
    pub rotation_speed: f32,
    pub fade_out: bool,
    pub scale_over_time: f32,
    pub gravity_affected: bool,
}

impl Particle {
    pub fn new(particle_type: ParticleType, position: Pos2, rng_seed: u32) -> Self {
        let rand_offset = |base_seed: u32, offset: u32| -> f32 {
            let seed = base_seed.wrapping_add(offset);
            (seed.wrapping_mul(1103515245).wrapping_add(12345) as f32 / u32::MAX as f32).fract()
        };

        let r1 = rand_offset(rng_seed, 1);
        let r2 = rand_offset(rng_seed, 2);
        let r3 = rand_offset(rng_seed, 3);
        let r4 = rand_offset(rng_seed, 4);

        let (color, size, lifetime, velocity, gravity) = match particle_type {
            // Combat - Blood
            ParticleType::BloodSplatter => (
                Color32::from_rgb(180, 20, 20),
                3.0 + r1 * 2.0,
                0.6 + r2 * 0.4,
                Vec2::new((r1 - 0.5) * 100.0, -30.0 - r2 * 50.0),
                true,
            ),
            ParticleType::BloodDrop => (
                Color32::from_rgb(150, 10, 10),
                2.0 + r1 * 1.5,
                0.8 + r2 * 0.6,
                Vec2::new((r1 - 0.5) * 60.0, -20.0 - r2 * 40.0),
                true,
            ),

            // Combat - Sparks
            ParticleType::Spark => (
                Color32::from_rgb(255, 220, (100.0 + r1 * 100.0) as u8),
                1.5 + r1 * 1.0,
                0.2 + r2 * 0.3,
                Vec2::new((r1 - 0.5) * 200.0, (r2 - 0.5) * 200.0),
                false,
            ),
            ParticleType::MetalClash => (
                Color32::from_rgb(255, 255, 200),
                2.0 + r1 * 2.0,
                0.15 + r2 * 0.2,
                Vec2::new((r1 - 0.5) * 240.0, (r2 - 0.5) * 240.0),
                true,
            ),
            ParticleType::CriticalHit => (
                Color32::from_rgb(255, 50, 50),
                5.0 + r1 * 3.0,
                0.4 + r2 * 0.3,
                Vec2::new((r1 - 0.5) * 80.0, -50.0 - r2 * 50.0),
                false,
            ),

            // Fire magic
            ParticleType::Flame => (
                Color32::from_rgb(255, (120.0 + r1 * 80.0) as u8, 30),
                4.0 + r1 * 3.0,
                0.4 + r2 * 0.3,
                Vec2::new((r1 - 0.5) * 40.0, -40.0 - r2 * 60.0),
                false,
            ),
            ParticleType::Ember => (
                Color32::from_rgb(255, (80.0 + r1 * 70.0) as u8, 20),
                2.0 + r1 * 1.5,
                0.8 + r2 * 0.5,
                Vec2::new((r1 - 0.5) * 60.0, -30.0 - r2 * 40.0),
                false,
            ),
            ParticleType::FireBurst => (
                Color32::from_rgb(255, 200, 50),
                6.0 + r1 * 4.0,
                0.3 + r2 * 0.2,
                Vec2::new((r1 - 0.5) * 300.0, (r2 - 0.5) * 300.0),
                false,
            ),

            // Ice magic
            ParticleType::Snowflake => (
                Color32::from_rgb(220, 240, 255),
                2.0 + r1 * 2.0,
                1.2 + r2 * 0.8,
                Vec2::new((r1 - 0.5) * 40.0, 15.0 + r2 * 25.0),
                false,
            ),
            ParticleType::IceShard => (
                Color32::from_rgb(180, 220, 255),
                3.0 + r1 * 2.0,
                0.3 + r2 * 0.3,
                Vec2::new((r1 - 0.5) * 160.0, (r2 - 0.5) * 160.0),
                false,
            ),
            ParticleType::FrostMist => (
                Color32::from_rgba_unmultiplied(200, 230, 255, 100),
                5.0 + r1 * 3.0,
                0.8 + r2 * 0.4,
                Vec2::new((r1 - 0.5) * 30.0, (r2 - 0.5) * 20.0),
                false,
            ),

            // Lightning magic
            ParticleType::LightningBolt => (
                Color32::from_rgb(200, 200, 255),
                1.5 + r1 * 1.0,
                0.1 + r2 * 0.1,
                Vec2::new((r1 - 0.5) * 400.0, (r2 - 0.5) * 400.0),
                false,
            ),
            ParticleType::ElectricSpark => (
                Color32::from_rgb(255, 255, (100.0 + r1 * 100.0) as u8),
                2.0 + r1 * 1.5,
                0.15 + r2 * 0.15,
                Vec2::new((r1 - 0.5) * 300.0, (r2 - 0.5) * 300.0),
                false,
            ),
            ParticleType::StaticCharge => (
                Color32::from_rgba_unmultiplied(180, 180, 255, 150),
                3.0 + r1 * 2.0,
                0.3 + r2 * 0.2,
                Vec2::new((r1 - 0.5) * 20.0, (r2 - 0.5) * 20.0),
                false,
            ),

            // Holy magic
            ParticleType::HolyLight => (
                Color32::from_rgb(255, 255, 200),
                4.0 + r1 * 3.0,
                0.6 + r2 * 0.4,
                Vec2::new((r1 - 0.5) * 60.0, -30.0 - r2 * 40.0),
                false,
            ),
            ParticleType::DivineSpark => (
                Color32::from_rgb(255, 255, 150),
                2.0 + r1 * 2.0,
                0.4 + r2 * 0.3,
                Vec2::new((r1 - 0.5) * 120.0, (r2 - 0.5) * 120.0),
                false,
            ),
            ParticleType::Blessing => (
                Color32::from_rgba_unmultiplied(255, 255, 200, 120),
                6.0 + r1 * 4.0,
                1.0 + r2 * 0.5,
                Vec2::new(0.0, -15.0 - r2 * 10.0),
                false,
            ),

            // Dark/Necromancy magic
            ParticleType::ShadowWisp => (
                Color32::from_rgba_unmultiplied(80, 40, 100, 180),
                3.0 + r1 * 2.0,
                0.8 + r2 * 0.4,
                Vec2::new((r1 - 0.5) * 50.0, (r2 - 0.5) * 40.0),
                false,
            ),
            ParticleType::SoulFragment => (
                Color32::from_rgba_unmultiplied(150, 100, 200, 200),
                2.0 + r1 * 2.0,
                0.6 + r2 * 0.4,
                Vec2::new((r1 - 0.5) * 80.0, -30.0 - r2 * 40.0),
                false,
            ),
            ParticleType::DarkEnergy => (
                Color32::from_rgba_unmultiplied(60, 20, 80, 150),
                5.0 + r1 * 3.0,
                0.5 + r2 * 0.3,
                Vec2::new((r1 - 0.5) * 100.0, (r2 - 0.5) * 100.0),
                false,
            ),
            ParticleType::Corruption => (
                Color32::from_rgb(100, 50, 120),
                2.5 + r1 * 2.0,
                1.2 + r2 * 0.6,
                Vec2::new((r1 - 0.5) * 40.0, (r2 - 0.5) * 30.0),
                false,
            ),

            // Poison
            ParticleType::PoisonBubble => (
                Color32::from_rgb(80, 200, 80),
                2.0 + r1 * 2.0,
                0.6 + r2 * 0.4,
                Vec2::new((r1 - 0.5) * 20.0, -25.0 - r2 * 25.0),
                false,
            ),
            ParticleType::ToxicMist => (
                Color32::from_rgba_unmultiplied(100, 180, 80, 100),
                6.0 + r1 * 3.0,
                1.0 + r2 * 0.4,
                Vec2::new((r1 - 0.5) * 30.0, (r2 - 0.5) * 10.0),
                false,
            ),
            ParticleType::VenomDrop => (
                Color32::from_rgb(60, 180, 60),
                1.5 + r1 * 1.0,
                0.4 + r2 * 0.3,
                Vec2::new((r1 - 0.5) * 40.0, 25.0 + r2 * 30.0),
                true,
            ),

            // Environmental - Dust
            ParticleType::Dust => (
                Color32::from_rgba_unmultiplied(180, 160, 140, 80),
                1.5 + r1 * 1.0,
                2.0 + r2 * 1.5,
                Vec2::new((r1 - 0.5) * 10.0, (r2 - 0.5) * 4.0),
                false,
            ),
            ParticleType::DungeonDust => (
                Color32::from_rgba_unmultiplied(120, 110, 100, 60),
                1.0 + r1 * 0.8,
                3.0 + r2 * 2.0,
                Vec2::new((r1 - 0.5) * 6.0, (r2 - 0.5) * 2.0),
                false,
            ),

            // Environmental - Water
            ParticleType::WaterDroplet => (
                Color32::from_rgba_unmultiplied(100, 150, 255, 180),
                2.0 + r1 * 1.5,
                0.6 + r2 * 0.4,
                Vec2::new((r1 - 0.5) * 40.0, -30.0 - r2 * 30.0),
                true,
            ),
            ParticleType::WaterRipple => (
                Color32::from_rgba_unmultiplied(150, 180, 255, 100),
                3.0 + r1 * 2.0,
                0.8 + r2 * 0.4,
                Vec2::ZERO,
                false,
            ),

            // Environmental - Lava
            ParticleType::LavaEmber => (
                Color32::from_rgb(255, (100.0 + r1 * 80.0) as u8, 30),
                2.0 + r1 * 2.0,
                1.2 + r2 * 0.6,
                Vec2::new((r1 - 0.5) * 30.0, -25.0 - r2 * 25.0),
                false,
            ),
            ParticleType::LavaBubble => (
                Color32::from_rgb(255, 150, 50),
                4.0 + r1 * 2.0,
                0.4 + r2 * 0.3,
                Vec2::new((r1 - 0.5) * 10.0, -15.0 - r2 * 10.0),
                false,
            ),

            // Environmental - Atmosphere
            ParticleType::Steam => (
                Color32::from_rgba_unmultiplied(200, 200, 210, 60),
                4.0 + r1 * 3.0,
                1.2 + r2 * 0.6,
                Vec2::new((r1 - 0.5) * 20.0, -20.0 - r2 * 20.0),
                false,
            ),
            ParticleType::Smoke => (
                Color32::from_rgba_unmultiplied(80, 80, 90, 80),
                5.0 + r1 * 3.0,
                1.8 + r2 * 0.8,
                Vec2::new((r1 - 0.5) * 16.0, -15.0 - r2 * 15.0),
                false,
            ),
            ParticleType::Fog => (
                Color32::from_rgba_unmultiplied(180, 180, 200, 40),
                8.0 + r1 * 5.0,
                3.5 + r2 * 1.5,
                Vec2::new((r1 - 0.5) * 6.0, (r2 - 0.5) * 2.0),
                false,
            ),

            // Environmental - Nature
            ParticleType::Leaf => (
                Color32::from_rgb(80, (120.0 + r1 * 60.0) as u8, 40),
                2.5 + r1 * 1.5,
                2.5 + r2 * 1.5,
                Vec2::new((r1 - 0.5) * 30.0, 10.0 + r2 * 15.0),
                false,
            ),
            ParticleType::Pollen => (
                Color32::from_rgba_unmultiplied(255, 230, 100, 150),
                1.0 + r1 * 1.0,
                3.5 + r2 * 1.5,
                Vec2::new((r1 - 0.5) * 16.0, (r2 - 0.5) * 10.0),
                false,
            ),
            ParticleType::SnowParticle => (
                Color32::from_rgb(240, 245, 255),
                1.5 + r1 * 1.0,
                2.5 + r2 * 1.5,
                Vec2::new((r1 - 0.5) * 20.0, 20.0 + r2 * 20.0),
                false,
            ),
            ParticleType::Ash => (
                Color32::from_rgba_unmultiplied(100, 100, 100, 150),
                1.5 + r1 * 1.0,
                2.0 + r2 * 1.0,
                Vec2::new((r1 - 0.5) * 24.0, (r2 - 0.3) * 25.0),
                false,
            ),

            // Special effects
            ParticleType::Teleport => (
                Color32::from_rgb(150, 100, 255),
                3.0 + r1 * 2.0,
                0.4 + r2 * 0.2,
                Vec2::new((r1 - 0.5) * 200.0, (r2 - 0.5) * 200.0),
                false,
            ),
            ParticleType::LevelUp => (
                Color32::from_rgb(255, 220, 100),
                4.0 + r1 * 3.0,
                1.2 + r2 * 0.6,
                Vec2::new((r1 - 0.5) * 60.0, -50.0 - r2 * 40.0),
                false,
            ),
            ParticleType::ItemPickup => (
                Color32::from_rgb(100, 255, 100),
                2.0 + r1 * 2.0,
                0.6 + r2 * 0.3,
                Vec2::new((r1 - 0.5) * 40.0, -35.0 - r2 * 25.0),
                false,
            ),
            ParticleType::GoldSparkle => (
                Color32::from_rgb(255, 215, 0),
                2.0 + r1 * 1.5,
                0.5 + r2 * 0.3,
                Vec2::new((r1 - 0.5) * 50.0, -30.0 - r2 * 25.0),
                false,
            ),
            ParticleType::HealingGlow => (
                Color32::from_rgba_unmultiplied(100, 255, 150, 150),
                3.0 + r1 * 2.0,
                0.8 + r2 * 0.4,
                Vec2::new((r1 - 0.5) * 20.0, -20.0 - r2 * 20.0),
                false,
            ),
            ParticleType::ShieldShimmer => (
                Color32::from_rgba_unmultiplied(100, 200, 255, 120),
                2.0 + r1 * 2.0,
                0.6 + r2 * 0.4,
                Vec2::new((r1 - 0.5) * 30.0, (r2 - 0.5) * 30.0),
                false,
            ),
            ParticleType::StealthFade => (
                Color32::from_rgba_unmultiplied(100, 100, 120, 100),
                4.0 + r1 * 2.0,
                0.8 + r2 * 0.4,
                Vec2::new((r1 - 0.5) * 40.0, (r2 - 0.5) * 20.0),
                false,
            ),
        };

        Self {
            particle_type,
            position,
            velocity,
            acceleration: Vec2::ZERO,
            color,
            size,
            lifetime,
            max_lifetime: lifetime,
            rotation: r3 * std::f32::consts::TAU,
            rotation_speed: (r4 - 0.5) * 6.0,
            fade_out: true,
            scale_over_time: match particle_type {
                ParticleType::FireBurst | ParticleType::Teleport => 0.5,
                ParticleType::FrostMist | ParticleType::ToxicMist | ParticleType::Fog => 1.3,
                ParticleType::WaterRipple => 1.8,
                ParticleType::Smoke | ParticleType::Steam => 1.2,
                _ => 1.0,
            },
            gravity_affected: gravity,
        }
    }

    /// Update the particle state for a frame
    pub fn update(&mut self, dt: f32) {
        if self.gravity_affected {
            self.velocity.y += 200.0 * dt;
        }
        self.velocity += self.acceleration * dt;
        self.position += self.velocity * dt;
        self.rotation += self.rotation_speed * dt;
        self.lifetime -= dt;

        if self.scale_over_time != 1.0 {
            let scale_factor = 1.0 + (self.scale_over_time - 1.0) * dt;
            self.size *= scale_factor;
        }
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }

    pub fn current_alpha(&self) -> u8 {
        let base_alpha = self.color.a();
        let life_ratio = self.lifetime / self.max_lifetime;

        let alpha = if self.fade_out && life_ratio < 0.3 {
            base_alpha as f32 * (life_ratio / 0.3)
        } else {
            base_alpha as f32
        };

        alpha.clamp(0.0, 255.0) as u8
    }

    pub fn current_color(&self) -> Color32 {
        let alpha = self.current_alpha();
        Color32::from_rgba_unmultiplied(
            self.color.r(),
            self.color.g(),
            self.color.b(),
            alpha,
        )
    }
}

// ============================================================================
// PARTICLE SYSTEM
// ============================================================================

/// The particle system manager
pub struct ParticleSystem {
    particles: Vec<Particle>,
    max_particles: usize,
    ambient_timer: f32,
    ambient_enabled: bool,
    rng_counter: u32,
}

impl ParticleSystem {
    pub fn new(max_particles: usize) -> Self {
        Self {
            particles: Vec::with_capacity(max_particles),
            max_particles,
            ambient_timer: 0.0,
            ambient_enabled: true,
            rng_counter: 0,
        }
    }

    fn next_seed(&mut self) -> u32 {
        self.rng_counter = self.rng_counter.wrapping_add(1);
        let time_component = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0);
        self.rng_counter.wrapping_mul(1103515245).wrapping_add(time_component)
    }

    /// Spawn a single particle
    pub fn spawn(&mut self, particle_type: ParticleType, position: Pos2) {
        if self.particles.len() < self.max_particles {
            let seed = self.next_seed();
            self.particles.push(Particle::new(particle_type, position, seed));
        }
    }

    /// Spawn multiple particles of a type at a position
    pub fn spawn_burst(&mut self, particle_type: ParticleType, position: Pos2, count: usize) {
        for _ in 0..count {
            if self.particles.len() >= self.max_particles {
                break;
            }
            self.spawn(particle_type, position);
        }
    }

    /// Spawn combat blood effect
    pub fn spawn_blood(&mut self, position: Pos2, damage: i32) {
        let count = (damage as usize / 5).clamp(3, 15);
        self.spawn_burst(ParticleType::BloodSplatter, position, count / 2);
        self.spawn_burst(ParticleType::BloodDrop, position, count);
    }

    /// Spawn combat spark effect (for melee hits)
    pub fn spawn_sparks(&mut self, position: Pos2, count: usize) {
        self.spawn_burst(ParticleType::Spark, position, count);
        self.spawn_burst(ParticleType::MetalClash, position, count / 2);
    }

    /// Spawn critical hit effect
    pub fn spawn_critical(&mut self, position: Pos2) {
        self.spawn_burst(ParticleType::CriticalHit, position, 8);
        self.spawn_burst(ParticleType::Spark, position, 12);
    }

    /// Spawn fireball spell effect
    pub fn spawn_fireball(&mut self, position: Pos2) {
        self.spawn_burst(ParticleType::Flame, position, 20);
        self.spawn_burst(ParticleType::Ember, position, 15);
        self.spawn_burst(ParticleType::FireBurst, position, 8);
    }

    /// Spawn ice spell effect
    pub fn spawn_ice_spell(&mut self, position: Pos2) {
        self.spawn_burst(ParticleType::IceShard, position, 15);
        self.spawn_burst(ParticleType::Snowflake, position, 20);
        self.spawn_burst(ParticleType::FrostMist, position, 8);
    }

    /// Spawn lightning spell effect
    pub fn spawn_lightning(&mut self, position: Pos2) {
        self.spawn_burst(ParticleType::LightningBolt, position, 10);
        self.spawn_burst(ParticleType::ElectricSpark, position, 15);
        self.spawn_burst(ParticleType::StaticCharge, position, 5);
    }

    /// Spawn holy spell effect
    pub fn spawn_holy(&mut self, position: Pos2) {
        self.spawn_burst(ParticleType::HolyLight, position, 12);
        self.spawn_burst(ParticleType::DivineSpark, position, 10);
        self.spawn_burst(ParticleType::Blessing, position, 5);
    }

    /// Spawn dark/necromancy spell effect
    pub fn spawn_dark_magic(&mut self, position: Pos2) {
        self.spawn_burst(ParticleType::ShadowWisp, position, 10);
        self.spawn_burst(ParticleType::SoulFragment, position, 8);
        self.spawn_burst(ParticleType::DarkEnergy, position, 6);
    }

    /// Spawn poison effect
    pub fn spawn_poison(&mut self, position: Pos2) {
        self.spawn_burst(ParticleType::PoisonBubble, position, 10);
        self.spawn_burst(ParticleType::ToxicMist, position, 5);
        self.spawn_burst(ParticleType::VenomDrop, position, 8);
    }

    /// Spawn teleport effect
    pub fn spawn_teleport(&mut self, position: Pos2) {
        self.spawn_burst(ParticleType::Teleport, position, 25);
    }

    /// Spawn level up celebration
    pub fn spawn_level_up(&mut self, position: Pos2) {
        self.spawn_burst(ParticleType::LevelUp, position, 30);
        self.spawn_burst(ParticleType::GoldSparkle, position, 15);
    }

    /// Spawn item pickup effect
    pub fn spawn_item_pickup(&mut self, position: Pos2) {
        self.spawn_burst(ParticleType::ItemPickup, position, 8);
    }

    /// Spawn gold pickup effect
    pub fn spawn_gold_pickup(&mut self, position: Pos2) {
        self.spawn_burst(ParticleType::GoldSparkle, position, 12);
    }

    /// Spawn healing effect
    pub fn spawn_healing(&mut self, position: Pos2) {
        self.spawn_burst(ParticleType::HealingGlow, position, 15);
    }

    /// Spawn shield effect
    pub fn spawn_shield(&mut self, position: Pos2) {
        self.spawn_burst(ParticleType::ShieldShimmer, position, 20);
    }

    /// Spawn stealth/invisibility effect
    pub fn spawn_stealth(&mut self, position: Pos2) {
        self.spawn_burst(ParticleType::StealthFade, position, 15);
    }

    /// Spawn ambient particles based on dungeon theme
    pub fn spawn_ambient(&mut self, theme: DungeonTheme, screen_rect: egui::Rect) {
        let seed = self.next_seed();
        let r1 = (seed.wrapping_mul(1103515245).wrapping_add(12345) as f32 / u32::MAX as f32).fract();
        let r2 = (seed.wrapping_mul(1103515247).wrapping_add(12347) as f32 / u32::MAX as f32).fract();
        let r3 = (seed.wrapping_mul(1103515249).wrapping_add(12349) as f32 / u32::MAX as f32).fract();

        let pos = Pos2::new(
            screen_rect.left() + r1 * screen_rect.width(),
            screen_rect.top() + r2 * screen_rect.height(),
        );

        match theme {
            DungeonTheme::Dungeon | DungeonTheme::Cave => {
                self.spawn(ParticleType::DungeonDust, pos);
            }
            DungeonTheme::Crypt => {
                self.spawn(ParticleType::Dust, pos);
                if r3 < 0.3 {
                    self.spawn(ParticleType::ShadowWisp, pos);
                }
            }
            DungeonTheme::Forest => {
                if r3 < 0.5 {
                    self.spawn(ParticleType::Leaf, pos);
                } else {
                    self.spawn(ParticleType::Pollen, pos);
                }
            }
            DungeonTheme::IceCavern => {
                if r3 < 0.6 {
                    self.spawn(ParticleType::SnowParticle, pos);
                } else {
                    self.spawn(ParticleType::FrostMist, pos);
                }
            }
            DungeonTheme::VolcanicLair => {
                if r3 < 0.33 {
                    self.spawn(ParticleType::Ash, pos);
                } else if r3 < 0.66 {
                    self.spawn(ParticleType::Ember, pos);
                } else {
                    self.spawn(ParticleType::Smoke, pos);
                }
            }
            DungeonTheme::AncientRuins => {
                self.spawn(ParticleType::Dust, pos);
                if r3 < 0.2 {
                    self.spawn(ParticleType::DivineSpark, pos);
                }
            }
            DungeonTheme::DemonRealm => {
                if r3 < 0.4 {
                    self.spawn(ParticleType::Corruption, pos);
                } else if r3 < 0.7 {
                    self.spawn(ParticleType::DarkEnergy, pos);
                } else {
                    self.spawn(ParticleType::LavaEmber, pos);
                }
            }
        }
    }

    /// Spawn water tile ambient effect
    pub fn spawn_water_ambient(&mut self, position: Pos2) {
        let seed = self.next_seed();
        let r = (seed as f32 / u32::MAX as f32).fract();
        if r < 0.02 {
            self.spawn(ParticleType::WaterDroplet, position);
        }
        if r > 0.98 {
            self.spawn(ParticleType::WaterRipple, position);
        }
    }

    /// Spawn lava tile ambient effect
    pub fn spawn_lava_ambient(&mut self, position: Pos2) {
        let seed = self.next_seed();
        let r = (seed as f32 / u32::MAX as f32).fract();
        if r < 0.03 {
            self.spawn(ParticleType::LavaEmber, position);
        }
        if r > 0.97 {
            self.spawn(ParticleType::LavaBubble, position);
        }
    }

    /// Update all particles
    pub fn update(&mut self, dt: f32, theme: DungeonTheme, screen_rect: egui::Rect) {
        // Update existing particles
        for particle in &mut self.particles {
            particle.update(dt);
        }

        // Remove dead particles
        self.particles.retain(|p| p.is_alive());

        // Spawn ambient particles
        if self.ambient_enabled {
            self.ambient_timer += dt;
            let spawn_interval = match theme {
                DungeonTheme::VolcanicLair | DungeonTheme::DemonRealm => 0.05,
                DungeonTheme::Forest | DungeonTheme::IceCavern => 0.08,
                _ => 0.15,
            };

            if self.ambient_timer >= spawn_interval {
                self.ambient_timer = 0.0;
                self.spawn_ambient(theme, screen_rect);
            }
        }
    }

    /// Render all particles
    pub fn render(&self, painter: &egui::Painter) {
        for particle in &self.particles {
            let color = particle.current_color();
            let size = particle.size;
            let pos = particle.position;

            match particle.particle_type {
                // Circular particles
                ParticleType::BloodDrop | ParticleType::WaterDroplet | ParticleType::VenomDrop |
                ParticleType::PoisonBubble | ParticleType::LavaBubble => {
                    painter.circle_filled(pos, size, color);
                }

                // Star-shaped particles
                ParticleType::Spark | ParticleType::ElectricSpark | ParticleType::DivineSpark |
                ParticleType::GoldSparkle => {
                    draw_star(painter, pos, size, particle.rotation, color);
                }

                // Diamond-shaped particles
                ParticleType::IceShard | ParticleType::CriticalHit => {
                    draw_diamond(painter, pos, size, particle.rotation, color);
                }

                // Snowflake
                ParticleType::Snowflake => {
                    draw_snowflake(painter, pos, size, particle.rotation, color);
                }

                // Leaf shape
                ParticleType::Leaf => {
                    draw_leaf(painter, pos, size, particle.rotation, color);
                }

                // Lightning bolt
                ParticleType::LightningBolt => {
                    draw_lightning(painter, pos, size, color);
                }

                // Expanding ring (ripples)
                ParticleType::WaterRipple => {
                    let life_ratio = particle.lifetime / particle.max_lifetime;
                    let ring_size = size * (2.0 - life_ratio);
                    painter.circle_stroke(pos, ring_size, egui::Stroke::new(1.5, color));
                }

                // Soft glow particles
                ParticleType::FrostMist | ParticleType::ToxicMist | ParticleType::Fog |
                ParticleType::DarkEnergy | ParticleType::Blessing => {
                    for i in 0..3 {
                        let alpha = (color.a() as f32 * (1.0 - i as f32 * 0.3)) as u8;
                        let glow_color = Color32::from_rgba_unmultiplied(
                            color.r(), color.g(), color.b(), alpha
                        );
                        painter.circle_filled(pos, size * (1.0 + i as f32 * 0.4), glow_color);
                    }
                }

                // Default square/rectangular particles
                _ => {
                    let rect = egui::Rect::from_center_size(pos, egui::vec2(size, size));
                    painter.rect_filled(rect, 0.0, color);
                }
            }
        }
    }

    /// Get current particle count
    pub fn particle_count(&self) -> usize {
        self.particles.len()
    }

    /// Toggle ambient particles
    pub fn toggle_ambient(&mut self) {
        self.ambient_enabled = !self.ambient_enabled;
    }

    /// Check if ambient is enabled
    pub fn is_ambient_enabled(&self) -> bool {
        self.ambient_enabled
    }

    /// Clear all particles
    pub fn clear(&mut self) {
        self.particles.clear();
    }
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self::new(2000)
    }
}

// ============================================================================
// DRAWING HELPERS
// ============================================================================

fn draw_star(painter: &egui::Painter, center: Pos2, size: f32, rotation: f32, color: Color32) {
    let points: Vec<Pos2> = (0..10)
        .map(|i| {
            let angle = rotation + (i as f32 * std::f32::consts::PI / 5.0);
            let r = if i % 2 == 0 { size } else { size * 0.4 };
            Pos2::new(
                center.x + r * angle.cos(),
                center.y + r * angle.sin(),
            )
        })
        .collect();

    painter.add(egui::Shape::convex_polygon(points, color, egui::Stroke::NONE));
}

fn draw_diamond(painter: &egui::Painter, center: Pos2, size: f32, rotation: f32, color: Color32) {
    let points: Vec<Pos2> = (0..4)
        .map(|i| {
            let angle = rotation + (i as f32 * std::f32::consts::PI / 2.0);
            Pos2::new(
                center.x + size * angle.cos(),
                center.y + size * angle.sin(),
            )
        })
        .collect();

    painter.add(egui::Shape::convex_polygon(points, color, egui::Stroke::NONE));
}

fn draw_snowflake(painter: &egui::Painter, center: Pos2, size: f32, rotation: f32, color: Color32) {
    for i in 0..6 {
        let angle = rotation + (i as f32 * std::f32::consts::PI / 3.0);
        let end = Pos2::new(
            center.x + size * angle.cos(),
            center.y + size * angle.sin(),
        );
        painter.line_segment([center, end], egui::Stroke::new(1.0, color));
    }
}

fn draw_leaf(painter: &egui::Painter, center: Pos2, size: f32, rotation: f32, color: Color32) {
    let points = vec![
        Pos2::new(center.x + size * rotation.cos(), center.y + size * rotation.sin()),
        Pos2::new(center.x + size * 0.3 * (rotation + 1.5).cos(), center.y + size * 0.3 * (rotation + 1.5).sin()),
        Pos2::new(center.x + size * (rotation + std::f32::consts::PI).cos(), center.y + size * (rotation + std::f32::consts::PI).sin()),
        Pos2::new(center.x + size * 0.3 * (rotation - 1.5).cos(), center.y + size * 0.3 * (rotation - 1.5).sin()),
    ];
    painter.add(egui::Shape::convex_polygon(points, color, egui::Stroke::NONE));
}

fn draw_lightning(painter: &egui::Painter, start: Pos2, length: f32, color: Color32) {
    let mut current = start;
    let segments = 4;
    let segment_length = length / segments as f32;

    for i in 0..segments {
        let seed = (start.x as u32).wrapping_mul(1103515245).wrapping_add(i);
        let offset = ((seed as f32 / u32::MAX as f32) - 0.5) * segment_length;
        let next = Pos2::new(
            current.x + offset,
            current.y + segment_length,
        );
        painter.line_segment([current, next], egui::Stroke::new(2.0, color));
        current = next;
    }
}

// ============================================================================
// PARTICLE EVENTS
// ============================================================================

/// Events that can trigger particle effects
#[derive(Clone)]
pub enum ParticleEvent {
    MeleeHit { position: Pos2, damage: i32, is_critical: bool },
    EnemyDeath { position: Pos2 },
    PlayerHit { position: Pos2, damage: i32 },
    SpellCast { position: Pos2, skill: Skill },
    StatusApplied { position: Pos2, effect: StatusEffect },
    ItemPickedUp { position: Pos2, is_gold: bool },
    LevelUp { position: Pos2 },
    Teleport { from: Pos2, to: Pos2 },
    HealingReceived { position: Pos2 },
    ShieldActivated { position: Pos2 },
    StealthActivated { position: Pos2 },
}

/// Process game events and spawn appropriate particles
pub fn process_particle_event(system: &mut ParticleSystem, event: ParticleEvent) {
    match event {
        ParticleEvent::MeleeHit { position, damage, is_critical } => {
            system.spawn_sparks(position, 8);
            system.spawn_blood(position, damage);
            if is_critical {
                system.spawn_critical(position);
            }
        }
        ParticleEvent::EnemyDeath { position } => {
            system.spawn_blood(position, 20);
            system.spawn_burst(ParticleType::SoulFragment, position, 5);
        }
        ParticleEvent::PlayerHit { position, damage } => {
            system.spawn_blood(position, damage);
        }
        ParticleEvent::SpellCast { position, skill } => {
            match skill {
                Skill::Fireball => system.spawn_fireball(position),
                Skill::IceSpear => system.spawn_ice_spell(position),
                Skill::Lightning => system.spawn_lightning(position),
                Skill::HolyLight | Skill::DivineShield | Skill::Smite | Skill::Consecrate => {
                    system.spawn_holy(position);
                }
                Skill::RaiseDead | Skill::LifeDrain | Skill::Curse | Skill::DarkPact => {
                    system.spawn_dark_magic(position);
                }
                Skill::PoisonBlade | Skill::PoisonArrow => system.spawn_poison(position),
                Skill::Teleport | Skill::ShadowStep => system.spawn_teleport(position),
                Skill::Vanish => system.spawn_stealth(position),
                _ => system.spawn_sparks(position, 10),
            }
        }
        ParticleEvent::StatusApplied { position, effect } => {
            match effect {
                StatusEffect::Burn => system.spawn_burst(ParticleType::Flame, position, 8),
                StatusEffect::Freeze => system.spawn_burst(ParticleType::IceShard, position, 8),
                StatusEffect::Poison => system.spawn_burst(ParticleType::PoisonBubble, position, 8),
                StatusEffect::Bleed => system.spawn_burst(ParticleType::BloodDrop, position, 8),
                StatusEffect::Shield => system.spawn_shield(position),
                StatusEffect::Regeneration => system.spawn_healing(position),
                StatusEffect::Invisibility => system.spawn_stealth(position),
                StatusEffect::Strength => system.spawn_burst(ParticleType::Flame, position, 5),
                _ => {}
            }
        }
        ParticleEvent::ItemPickedUp { position, is_gold } => {
            if is_gold {
                system.spawn_gold_pickup(position);
            } else {
                system.spawn_item_pickup(position);
            }
        }
        ParticleEvent::LevelUp { position } => {
            system.spawn_level_up(position);
        }
        ParticleEvent::Teleport { from, to } => {
            system.spawn_teleport(from);
            system.spawn_teleport(to);
        }
        ParticleEvent::HealingReceived { position } => {
            system.spawn_healing(position);
        }
        ParticleEvent::ShieldActivated { position } => {
            system.spawn_shield(position);
        }
        ParticleEvent::StealthActivated { position } => {
            system.spawn_stealth(position);
        }
    }
}
