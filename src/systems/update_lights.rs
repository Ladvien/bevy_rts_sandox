// use bevy::prelude::*;

// use crate::{
//     constants::{constants::*, player::ROTATION_SPEED},
//     Game,
// };

// pub fn update_lights(
//     time: Res<Time>,
//     mut game: ResMut<Game>,
//     mut transforms: Query<&mut Transform>,
// ) {
//     if game.main_light.move_timer.tick(time.delta()).finished() {
//         game.main_light.i = game.mechanics.i;
//         game.main_light.j = game.mechanics.j;
//     }
// }
