//! Spawn the main level.

use bevy::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(_app: &mut App) {

}



/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,

) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),

    ));
}
