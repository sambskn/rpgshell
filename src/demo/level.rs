//! Spawn the main level.

use bevy::prelude::*;

use crate::{screens::Screen, text_boxes::text_box};

pub(super) fn plugin(_app: &mut App) {}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        text_box(
            vec!["yo".to_string()],
            time.elapsed_secs(),
            &mut meshes,
            &mut materials,
        ),
    ));
}
