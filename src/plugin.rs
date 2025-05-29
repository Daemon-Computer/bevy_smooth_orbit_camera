use crate::systems::update_orbit;
use bevy::prelude::{App, Plugin, Update};

pub struct OrbitPlugin;

impl Plugin for OrbitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_orbit);
    }
}
