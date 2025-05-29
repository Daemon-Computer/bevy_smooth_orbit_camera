use bevy::prelude::{Component, Deref, Entity, Vec2};
use std::f32::consts::PI;

#[derive(Component, Clone, Copy)]
pub struct Orbit {
    // Movement
    pub movement: Vec2,
    pub speed: f32,
    pub friction: f32,
    pub max_speed: f32,
    pub pitch_limits: (f32, f32),

    // Zoom
    pub zoom: f32,
    pub zoom_steps: f32,
    pub zoom_limits: (f32, f32),

    // Interaction
    pub dragging: bool,
}

impl Default for Orbit {
    fn default() -> Self {
        Self {
            movement: Vec2::ZERO,
            speed: 0.2,
            friction: 0.0008,
            max_speed: 0.3,
            pitch_limits: (-PI / 4.0, PI / 4.0),

            zoom: 1.0,
            zoom_limits: (4.0, 8.0),
            zoom_steps: 1.0,

            dragging: false,
        }
    }
}

impl Orbit {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn zoom_limits(mut self, min: f32, max: f32) -> Self {
        self.zoom_limits = (min, max);
        self
    }
}

#[derive(Component, Deref, Debug)]
#[relationship(relationship_target = OrbitedBy)]
pub struct Orbiting(pub Entity);

impl Orbiting {
    pub fn new(entity: Entity) -> Self {
        Self(entity)
    }

    pub fn target(&mut self, entity: Entity) {
        self.0 = entity;
    }
}

#[derive(Component, Debug)]
#[relationship_target(relationship = Orbiting)]
pub struct OrbitedBy(Vec<Entity>);
