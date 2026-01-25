//! ShadowCrypt GUI Library
//!
//! This library provides the graphical frontend for ShadowCrypt,
//! including particle effects, lighting, and rendering systems.

pub mod particles;

pub use particles::{
    Particle, ParticleSystem, ParticleType, ParticleEvent,
    process_particle_event,
};
