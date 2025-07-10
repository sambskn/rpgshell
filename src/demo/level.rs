//! Spawn the main level.

use bevy::prelude::*;

use crate::{screens::Screen, text_boxes::text_box};

pub(super) fn plugin(_app: &mut App) {}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        text_box("yo".to_string(), meshes, materials),
    ));
}
